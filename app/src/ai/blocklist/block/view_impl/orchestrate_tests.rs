//! Tests for the orchestrate view module.
//!
//! Note: full rendering tests for the confirmation card require a fully-set-up
//! `AIBlock` view (with `AIBlockStateHandles`, `BlocklistAIActionModel`,
//! `BlocklistAIHistoryModel`, etc.) and are best authored as integration tests
//! under `crates/integration` once the live Reject/Edit/Accept buttons are
//! wired (see the file-level TODO in `orchestrate.rs`). The pre-dispatch /
//! per-agent dispatch paths are exhaustively covered by the unit tests in
//! `crate::ai::blocklist::orchestrate_dispatch_tests`.
//!
//! Stage 1 ordering / mid-batch / M=0 / pre-dispatch coverage lives in:
//! - `crate::ai::blocklist::orchestrate_dispatch_tests::n_children_all_succeed_returns_launched_with_input_order`
//! - `crate::ai::blocklist::orchestrate_dispatch_tests::mid_batch_failure_reports_mixed_outcomes_in_input_order`
//! - `crate::ai::blocklist::orchestrate_dispatch_tests::m_zero_all_failed_returns_launched_with_all_failed_outcomes`
//! - `crate::ai::blocklist::orchestrate_dispatch_tests::pre_dispatch_failure_returns_failure_variant`
//! - `crate::ai::blocklist::orchestrate_dispatch_tests::empty_agent_run_configs_returns_failure_defensively`
//! - `crate::ai::blocklist::orchestrate_dispatch_tests::remote_execution_mode_propagates_environment_id_and_worker_host`
