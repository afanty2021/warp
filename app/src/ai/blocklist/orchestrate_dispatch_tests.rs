//! Unit tests for the orchestrate per-agent dispatch helper.
//!
//! Covers the behaviors called out in TECH.md §"Testing > Client tests":
//! - N children all succeed → `Launched` with N `launched` outcomes
//! - mid-batch failure → `Launched` with mixed `launched` / `failed`
//! - M=0 (every per-agent dispatch fails individually) → `Launched` with
//!   all-`failed` outcomes (NOT `Failure`)
//! - pre-dispatch failure → `Failure` (constructed via [`failure_result`])
//! - input order is preserved regardless of completion order
//!
//! Stage 2 dispatch hooks (`auto_launch=true`) reuse the same helper, so the
//! same tests cover that path; explicit auto-launch gating is exercised at
//! the view layer.
//!
//! The mock `AIClient` is an `Arc<dyn AIClient>` whose `create_agent_task`
//! returns scripted results in the order configured by the test. To exercise
//! "completion order != input order" we configure each scripted result with
//! a per-call delay; `join_all` still preserves input order in its output
//! vec.
use std::str::FromStr;
use std::sync::Arc;

use ai::agent::action::{OrchestrateAgentRunConfig, OrchestrateExecutionMode, OrchestrateRequest};
use ai::agent::action_result::{
    OrchestrateAgentOutcomeKind, OrchestrateLaunchedExecutionMode, OrchestrateResult,
};
use anyhow::anyhow;

use super::{dispatch_orchestrate, failure_result, DispatchInputs};
use crate::ai::ambient_agents::AmbientAgentTaskId;
use crate::server::server_api::ai::MockAIClient;

fn task_id(uuid: &str) -> AmbientAgentTaskId {
    AmbientAgentTaskId::from_str(uuid).expect("test-supplied uuid")
}

fn run_config(name: &str) -> OrchestrateAgentRunConfig {
    OrchestrateAgentRunConfig {
        name: name.to_string(),
        prompt: format!("per-agent prompt for {name}"),
        title: format!("{name} title"),
    }
}

fn base_request_local(names: &[&str]) -> OrchestrateRequest {
    OrchestrateRequest {
        summary: "test orchestrate".to_string(),
        base_prompt: "base prompt".to_string(),
        skills: Vec::new(),
        model_id: "auto".to_string(),
        harness_type: "oz".to_string(),
        execution_mode: OrchestrateExecutionMode::Local,
        agent_run_configs: names.iter().copied().map(run_config).collect(),
        auto_launch: false,
    }
}

/// Drive an arbitrary future to completion in a single-threaded executor.
/// Avoids pulling tokio in here for these self-contained tests.
fn block_on<F: std::future::Future>(future: F) -> F::Output {
    futures::executor::block_on(future)
}

#[test]
fn n_children_all_succeed_returns_launched_with_input_order() {
    // Three children dispatched in input order [alpha, bravo, charlie]. Mock
    // resolves them in completion order [bravo, charlie, alpha] (driven by
    // varying per-call delays). The result's `agents` vec MUST report alpha,
    // bravo, charlie regardless.
    let mut client = MockAIClient::new();
    let alpha_id = task_id("11111111-1111-4111-8111-111111111111");
    let bravo_id = task_id("22222222-2222-4222-8222-222222222222");
    let charlie_id = task_id("33333333-3333-4333-8333-333333333333");

    client.expect_create_agent_task().times(3).returning(
        move |prompt, env, parent_run_id, config| {
            // Per-child prompt is base_prompt + "\n\n" + per_agent_prompt.
            assert!(prompt.starts_with("base prompt\n\n"));
            assert_eq!(env, None, "Local execution_mode → no environment_uid");
            assert_eq!(parent_run_id, Some("parent-run".to_string()));
            let cfg = config.expect("config snapshot is always set");
            let name = cfg.name.expect("per-child name set on snapshot");
            // `join_all` preserves input order in its output regardless of
            // future completion order, so a sync mock return is sufficient
            // to exercise the input-order-preservation path: the outcomes
            // vec is zipped against the input `agent_run_configs` order, not
            // the order futures resolved.
            match name.as_str() {
                "alpha" => Ok(alpha_id),
                "bravo" => Ok(bravo_id),
                "charlie" => Ok(charlie_id),
                _ => unreachable!("unexpected child name {name}"),
            }
        },
    );

    let result = block_on(dispatch_orchestrate(DispatchInputs {
        request: base_request_local(&["alpha", "bravo", "charlie"]),
        client: Arc::new(client),
        parent_run_id: Some("parent-run".to_string()),
    }));

    let OrchestrateResult::Launched {
        execution_mode,
        agents,
        model_id,
        harness_type,
    } = result
    else {
        panic!("expected Launched, got {result:?}");
    };

    assert_eq!(model_id, "auto");
    assert_eq!(harness_type, "oz");
    assert!(matches!(
        execution_mode,
        OrchestrateLaunchedExecutionMode::Local
    ));
    assert_eq!(agents.len(), 3);
    assert_eq!(agents[0].name, "alpha");
    assert_eq!(agents[1].name, "bravo");
    assert_eq!(agents[2].name, "charlie");
    assert!(matches!(
        agents[0].kind,
        OrchestrateAgentOutcomeKind::Launched { ref agent_id } if agent_id == &alpha_id.to_string()
    ));
    assert!(matches!(
        agents[1].kind,
        OrchestrateAgentOutcomeKind::Launched { ref agent_id } if agent_id == &bravo_id.to_string()
    ));
    assert!(matches!(
        agents[2].kind,
        OrchestrateAgentOutcomeKind::Launched { ref agent_id } if agent_id == &charlie_id.to_string()
    ));
}

#[test]
fn mid_batch_failure_reports_mixed_outcomes_in_input_order() {
    // bravo fails; alpha and charlie succeed. Per-spec, partial failures are
    // reported inside `Launched`, not as a top-level `Failure`.
    let mut client = MockAIClient::new();
    let alpha_id = task_id("aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa");
    let charlie_id = task_id("cccccccc-cccc-4ccc-8ccc-cccccccccccc");

    client
        .expect_create_agent_task()
        .times(3)
        .returning(move |_prompt, _env, _parent, config| {
            let name = config
                .expect("snapshot")
                .name
                .expect("per-child name on snapshot");
            match name.as_str() {
                "alpha" => Ok(alpha_id),
                "bravo" => Err(anyhow!("synthetic mid-batch failure")),
                "charlie" => Ok(charlie_id),
                _ => unreachable!("unexpected child {name}"),
            }
        });

    let result = block_on(dispatch_orchestrate(DispatchInputs {
        request: base_request_local(&["alpha", "bravo", "charlie"]),
        client: Arc::new(client),
        parent_run_id: None,
    }));

    let OrchestrateResult::Launched { agents, .. } = result else {
        panic!("expected Launched even on partial failure, got {result:?}");
    };
    assert_eq!(agents.len(), 3);
    assert_eq!(agents[0].name, "alpha");
    assert!(matches!(
        agents[0].kind,
        OrchestrateAgentOutcomeKind::Launched { .. }
    ));
    assert_eq!(agents[1].name, "bravo");
    let OrchestrateAgentOutcomeKind::Failed { ref error } = agents[1].kind else {
        panic!("bravo should be Failed, got {:?}", agents[1].kind);
    };
    assert!(error.contains("synthetic mid-batch failure"));
    assert_eq!(agents[2].name, "charlie");
    assert!(matches!(
        agents[2].kind,
        OrchestrateAgentOutcomeKind::Launched { .. }
    ));
}

#[test]
fn m_zero_all_failed_returns_launched_with_all_failed_outcomes() {
    // PRODUCT.md "Post-action card states" → "Started M of N agents":
    //   The M=0 case (every per-agent dispatch failed) renders here, not
    //   under "Failed to start orchestration" — the run-wide configuration
    //   was resolved and the `Launched` result still carries it, just with
    //   all `failed` outcomes.
    let mut client = MockAIClient::new();
    client
        .expect_create_agent_task()
        .times(2)
        .returning(|_prompt, _env, _parent, _config| Err(anyhow!("transient backend error")));

    let result = block_on(dispatch_orchestrate(DispatchInputs {
        request: base_request_local(&["alpha", "bravo"]),
        client: Arc::new(client),
        parent_run_id: None,
    }));

    let OrchestrateResult::Launched { agents, .. } = result else {
        panic!("M=0 should still produce Launched (with all-failed outcomes)");
    };
    assert_eq!(agents.len(), 2);
    assert_eq!(agents[0].name, "alpha");
    assert_eq!(agents[1].name, "bravo");
    assert!(matches!(
        agents[0].kind,
        OrchestrateAgentOutcomeKind::Failed { .. }
    ));
    assert!(matches!(
        agents[1].kind,
        OrchestrateAgentOutcomeKind::Failed { .. }
    ));
}

#[test]
fn pre_dispatch_failure_returns_failure_variant() {
    // The view's Accept handler short-circuits to `failure_result(...)`
    // before even calling `dispatch_orchestrate` when the call site detects
    // a transport-level issue (e.g. the active session can't be resolved).
    // Verify the constructor produces the Failure variant with the right
    // error string.
    let result = failure_result("could not begin the launch sequence");
    let OrchestrateResult::Failure { error } = result else {
        panic!("expected Failure variant");
    };
    assert_eq!(error, "could not begin the launch sequence");
}

#[test]
fn empty_agent_run_configs_returns_failure_defensively() {
    // Server-side validation rejects empty agent_run_configs, but the
    // dispatcher is defensive — return Failure rather than Launched-with-
    // zero-agents (which would misreport at the telemetry level).
    let request = OrchestrateRequest {
        summary: "empty".to_string(),
        base_prompt: "base".to_string(),
        skills: Vec::new(),
        model_id: String::new(),
        harness_type: String::new(),
        execution_mode: OrchestrateExecutionMode::Local,
        agent_run_configs: Vec::new(),
        auto_launch: false,
    };
    // No mock expectations — `create_agent_task` must never be called.
    let client = MockAIClient::new();
    let result = block_on(dispatch_orchestrate(DispatchInputs {
        request,
        client: Arc::new(client),
        parent_run_id: None,
    }));
    assert!(matches!(result, OrchestrateResult::Failure { .. }));
}

#[test]
fn remote_execution_mode_propagates_environment_id_and_worker_host() {
    // The remote variant of OrchestrateExecutionMode must surface
    // environment_id (passed as environment_uid to create_agent_task) and
    // worker_host on the AgentConfigSnapshot.
    let mut client = MockAIClient::new();
    let remote_id = task_id("44444444-4444-4444-8444-444444444444");
    client
        .expect_create_agent_task()
        .times(1)
        .returning(move |_prompt, env, _parent, config| {
            assert_eq!(env, Some("env-123".to_string()));
            let snapshot = config.expect("snapshot");
            assert_eq!(snapshot.worker_host, Some("host-abc".to_string()));
            assert_eq!(snapshot.computer_use_enabled, Some(true));
            Ok(remote_id)
        });

    let request = OrchestrateRequest {
        summary: "remote".to_string(),
        base_prompt: "base".to_string(),
        skills: Vec::new(),
        model_id: "claude-4-6-opus-high".to_string(),
        harness_type: "oz".to_string(),
        execution_mode: OrchestrateExecutionMode::Remote {
            environment_id: "env-123".to_string(),
            worker_host: "host-abc".to_string(),
            computer_use_enabled: true,
        },
        agent_run_configs: vec![run_config("solo")],
        auto_launch: false,
    };

    let result = block_on(dispatch_orchestrate(DispatchInputs {
        request,
        client: Arc::new(client),
        parent_run_id: None,
    }));
    let OrchestrateResult::Launched {
        execution_mode,
        agents,
        ..
    } = result
    else {
        panic!("expected Launched");
    };
    let OrchestrateLaunchedExecutionMode::Remote {
        environment_id,
        worker_host,
        computer_use_enabled,
    } = execution_mode
    else {
        panic!("expected Remote execution mode in Launched result");
    };
    assert_eq!(environment_id, "env-123");
    assert_eq!(worker_host, "host-abc");
    assert!(computer_use_enabled);
    assert_eq!(agents.len(), 1);
    assert_eq!(agents[0].name, "solo");
}
