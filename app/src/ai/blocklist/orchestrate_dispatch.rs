//! Per-agent dispatch helper for the `orchestrate` tool call's Accept path.
//!
//! On Accept (Stage 1) or auto-launch (Stage 2), the client iterates over
//! `Orchestrate.agent_run_configs` in input order, dispatches a parallel
//! `CreateAgentTask` GraphQL mutation per child via
//! `futures::future::join_all`, then emits `OrchestrateResult.Launched` with
//! per-agent outcomes **in input order regardless of completion order**.
//! Tasks already created are not rolled back — they are simply reported as
//! `launched` outcomes alongside any `failed` siblings.
//!
//! This module is the testable seam for that flow. The view's Accept handler
//! constructs a [`DispatchInputs`] from the (possibly user-edited) tool call
//! state plus the in-memory `AgentConfigSnapshot` builder used by
//! `start_agent_v2`, then awaits [`dispatch_orchestrate`] against the live
//! `AIClient`. Tests construct a [`MockAIClient`](server_api::ai::MockAIClient)
//! and assert on the resulting [`OrchestrateResult`].
//!
//! Spec references: TECH.md §9 ("Client: per-agent task creation"), PRODUCT.md
//! "Confirmation card → card actions → Accept", "Invariants" (per-agent
//! `CreateAgentTask` calls issued in parallel; outcomes reported in input
//! order regardless of completion order).
//
// Dead-code warnings on this module's exported entry points are silenced
// while the live Accept-button caller is the structural-stub described in
// `view_impl::orchestrate`'s file-level TODO. Once that wiring lands, the
// `#[allow(dead_code)]` annotations below can be removed.
#![allow(dead_code)]

use std::sync::Arc;

use ai::agent::action::{OrchestrateAgentRunConfig, OrchestrateExecutionMode, OrchestrateRequest};
use ai::agent::action_result::{
    OrchestrateAgentOutcome, OrchestrateAgentOutcomeKind, OrchestrateLaunchedExecutionMode,
    OrchestrateResult,
};
use futures::future::join_all;

use crate::ai::ambient_agents::task::{harness_from_name, HarnessConfig};
use crate::server::server_api::ai::{AIClient, AgentConfigSnapshot};

/// Inputs to a single orchestrate dispatch pass.
///
/// `request` is the orchestrate tool call payload (after any user edits in
/// the inline editor). `client` is the GraphQL mutation surface; injected as
/// a trait object so tests can supply a `MockAIClient`. `parent_run_id` is
/// passed straight through to `CreateAgentTask` so child runs link back to
/// their orchestrator.
pub struct DispatchInputs {
    pub request: OrchestrateRequest,
    pub client: Arc<dyn AIClient>,
    pub parent_run_id: Option<String>,
}

/// Execute the per-agent dispatch loop and produce the terminal
/// [`OrchestrateResult`].
///
/// Behavior:
/// - Issues N parallel `CreateAgentTask` calls, one per
///   `agent_run_configs[i]`.
/// - Per-child prompt is `base_prompt + "\n\n" + agent_run_configs[i].prompt`
///   when both are non-empty, else just `base_prompt`. Falls back to the
///   per-agent `prompt` alone when `base_prompt` is empty.
/// - Returns `OrchestrateResult::Launched` with one `AgentOutcome` per child
///   in input order regardless of GraphQL response order.
/// - If `agent_run_configs` is empty (defensive — server validation should
///   reject this), returns `OrchestrateResult::Failure` with a sentinel
///   message rather than `Launched` with an empty agents vec.
///
/// Note: this helper does NOT itself surface a pre-dispatch failure path
/// (e.g. transport-level "could not begin the launch sequence") because the
/// client is a trait object; if the call site needs to short-circuit before
/// dispatch (e.g. after detecting a malformed config), it constructs
/// [`failure_result`] directly. The per-child error path is the normal
/// `Result::Err` from `create_agent_task`, which produces a `failed` agent
/// outcome inside the `Launched` result, matching the spec's invariant that
/// partial-batch failures are reported inside `Launched` rather than via the
/// top-level `Failure` variant.
pub async fn dispatch_orchestrate(inputs: DispatchInputs) -> OrchestrateResult {
    let DispatchInputs {
        request,
        client,
        parent_run_id,
    } = inputs;

    if request.agent_run_configs.is_empty() {
        // Defensive: server-side validation rejects empty agent_run_configs
        // (1 ≤ N ≤ 32). If we somehow received a defaulted request here,
        // surface a `Failure` rather than Launched-with-zero-agents.
        return failure_result("orchestrate: empty agent_run_configs");
    }

    let environment_uid = match &request.execution_mode {
        OrchestrateExecutionMode::Local => None,
        OrchestrateExecutionMode::Remote { environment_id, .. } => {
            if environment_id.trim().is_empty() {
                None
            } else {
                Some(environment_id.clone())
            }
        }
    };

    // Build a base AgentConfigSnapshot once, then clone-and-override per
    // child so each task carries the run-wide config.
    let base_snapshot = build_base_snapshot(&request);

    // Spawn each call in input order; `join_all` preserves that order in its
    // output regardless of which future resolves first.
    let futures = request
        .agent_run_configs
        .iter()
        .map(|cfg| {
            let prompt = compose_prompt(&request.base_prompt, &cfg.prompt);
            let mut snapshot = base_snapshot.clone();
            // Per-agent name is searchability-only metadata; the title is set
            // independently below.
            snapshot.name = Some(cfg.name.clone());
            let environment_uid = environment_uid.clone();
            let parent_run_id = parent_run_id.clone();
            let client = Arc::clone(&client);
            async move {
                client
                    .create_agent_task(prompt, environment_uid, parent_run_id, Some(snapshot))
                    .await
            }
        })
        .collect::<Vec<_>>();

    let raw_outcomes = join_all(futures).await;

    // Zip with the original configs to preserve input-order semantics.
    let agents = request
        .agent_run_configs
        .iter()
        .zip(raw_outcomes.into_iter())
        .map(|(cfg, result)| OrchestrateAgentOutcome {
            name: cfg.name.clone(),
            title: cfg.title.clone(),
            kind: match result {
                Ok(task_id) => OrchestrateAgentOutcomeKind::Launched {
                    agent_id: task_id.to_string(),
                },
                Err(err) => OrchestrateAgentOutcomeKind::Failed {
                    error: format!("{err:#}"),
                },
            },
        })
        .collect();

    OrchestrateResult::Launched {
        model_id: request.model_id.clone(),
        harness_type: request.harness_type.clone(),
        execution_mode: launched_execution_mode(&request.execution_mode),
        agents,
    }
}

/// Helper to construct the pre-dispatch failure result. Used by the view's
/// Accept handler when a transport-level issue prevents *any* per-agent
/// `CreateAgentTask` call from being issued (the `failure` variant per
/// PRODUCT.md "Invariants").
pub fn failure_result(error: impl Into<String>) -> OrchestrateResult {
    OrchestrateResult::Failure {
        error: error.into(),
    }
}

fn launched_execution_mode(mode: &OrchestrateExecutionMode) -> OrchestrateLaunchedExecutionMode {
    match mode {
        OrchestrateExecutionMode::Local => OrchestrateLaunchedExecutionMode::Local,
        OrchestrateExecutionMode::Remote {
            environment_id,
            worker_host,
            computer_use_enabled,
        } => OrchestrateLaunchedExecutionMode::Remote {
            environment_id: environment_id.clone(),
            worker_host: worker_host.clone(),
            computer_use_enabled: *computer_use_enabled,
        },
    }
}

/// Compute per-child prompt per the spec invariant:
/// `base_prompt + "\n\n" + agent_run_configs[i].prompt` when both are
/// non-empty, just `base_prompt` when the per-agent `prompt` is empty, just
/// the per-agent `prompt` when `base_prompt` is empty (defensive).
fn compose_prompt(base_prompt: &str, per_agent_prompt: &str) -> String {
    let base_trimmed = base_prompt.trim();
    let per_agent_trimmed = per_agent_prompt.trim();
    match (base_trimmed.is_empty(), per_agent_trimmed.is_empty()) {
        (false, false) => format!("{base_prompt}\n\n{per_agent_prompt}"),
        (false, true) => base_prompt.to_string(),
        (true, false) => per_agent_prompt.to_string(),
        (true, true) => String::new(),
    }
}

/// Build the run-wide [`AgentConfigSnapshot`] once. Per-child name is
/// patched on a clone before each `create_agent_task` call.
fn build_base_snapshot(request: &OrchestrateRequest) -> AgentConfigSnapshot {
    let (environment_id, worker_host, computer_use_enabled) = match &request.execution_mode {
        OrchestrateExecutionMode::Local => (None, None, None),
        OrchestrateExecutionMode::Remote {
            environment_id,
            worker_host,
            computer_use_enabled,
        } => (
            (!environment_id.is_empty()).then(|| environment_id.clone()),
            (!worker_host.is_empty()).then(|| worker_host.clone()),
            Some(*computer_use_enabled),
        ),
    };

    let harness = (!request.harness_type.is_empty()).then(|| {
        HarnessConfig::from_harness_type(harness_from_name(request.harness_type.as_str()))
    });

    AgentConfigSnapshot {
        name: None,
        environment_id,
        model_id: (!request.model_id.is_empty()).then(|| request.model_id.clone()),
        base_prompt: None,
        mcp_servers: None,
        profile_id: None,
        worker_host,
        skill_spec: None,
        computer_use_enabled,
        harness,
        harness_auth_secrets: None,
    }
}

/// Public no-op alias to silence dead-code warnings on the helper struct
/// in builds where the view module isn't yet wired. Removed once the view
/// calls into this directly.
#[allow(dead_code)]
fn _make_used(_cfg: OrchestrateAgentRunConfig) {}

#[cfg(test)]
#[path = "orchestrate_dispatch_tests.rs"]
mod tests;
