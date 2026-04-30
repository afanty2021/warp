//! Renders the inline confirmation card for an `orchestrate` tool call.
//!
//! Modeled on the `orchestration.rs` sibling that renders `start_agent_v2`.
//! The full Figma-fidelity card (model / harness / environment / host
//! pickers, inline editor open/close with retained values, Cloud-without-env
//! validation gating, OpenCode + Cloud Accept-disabling, OpenCode-reset on
//! Local→Cloud toggle) is laid out here as **structure first**: the card
//! renders all five required post-action states (Launching N, Started N,
//! Started M of N, Failed to start orchestration, Cancelled) plus a streaming
//! placeholder, and the pre-dispatch Reject/Edit/Accept buttons are stubbed
//! visually with TODO seams pointing at where the dispatch helper
//! ([`crate::ai::blocklist::orchestrate_dispatch::dispatch_orchestrate`])
//! plugs in.
//!
//! Stage 2 seam: when `req.auto_launch` is true the card skips the
//! confirmation phase and transitions directly into Launching N. The
//! interactive Reject/Edit/Accept layout is suppressed in that branch.
//!
//! Spec references: TECH.md §8 ("Client: confirmation card"), §9
//! ("Per-agent task creation"), PRODUCT.md "Confirmation card" + "Post-action
//! card states".
//
// TODO(QUALITY-569): the live pre-dispatch UI (Reject/Edit/Accept buttons,
// inline editor with retained values across mode toggles, the OpenCode +
// Cloud Accept-disabled gate, the Cloud-without-env Accept-disabled gate)
// is structural-stub today. Wiring it requires:
//   1. State handles in `AIBlockStateHandles` for the three buttons + the
//      Edit-open-vs-closed flag + the per-field edited values.
//   2. Re-using `ModelPicker` (in `app/src/settings_view/ai_page.rs`),
//      environment + worker_host + harness pickers (paths TBD; expected
//      under `app/src/ai/execution_profiles/`).
//   3. A new `AIBlockAction::OrchestrateAccept{ /* edited fields */ }`
//      variant routed into `BlocklistAIController` so the dispatch helper
//      can be `await`-ed off the main loop.
// The card layout below sketches the surface so the dispatcher arm and the
// data path can land in this PR; the picker integration is best done as a
// fast-follow once the proto / dispatch / data-model infrastructure is in
// place.

use ai::agent::action::{OrchestrateExecutionMode, OrchestrateRequest};
use ai::agent::action_result::{OrchestrateAgentOutcomeKind, OrchestrateResult};
use warpui::elements::{
    Container, CornerRadius, CrossAxisAlignment, Empty, Flex, ParentElement, Radius, Text,
};
use warpui::{AppContext, Element, SingletonEntity};

use crate::ai::agent::icons;
use crate::ai::agent::{AIAgentActionId, AIAgentActionResultType};
use crate::ai::blocklist::action_model::AIActionStatus;
use crate::ai::blocklist::inline_action::inline_action_icons;
use crate::ai::blocklist::inline_action::requested_action::render_requested_action_row_for_text;
use crate::appearance::Appearance;
use crate::ui_components::blended_colors;

use super::output::Props;
use super::WithContentItemSpacing;

/// Renders the full orchestrate confirmation card.
///
/// Dispatched from the tool-call view dispatcher in `output.rs`. The card
/// is gated on `FeatureFlag::OrchestrateTool` at the dispatcher level; when
/// the flag is off this function is never reached.
///
/// Returns an [`Element`] that renders one of:
/// - The streaming placeholder (request received but tool call still
///   incomplete — server hasn't yet emitted its final
///   `SetOrchestrateToolCall` with run-wide defaults folded in).
/// - The Launching N agents transient state (Accept clicked, dispatch in
///   flight).
/// - The Started N / Started M of N / Failed / Cancelled / Disabled
///   terminal states (driven by the recorded `AIAgentActionResultType`).
/// - The full pre-dispatch confirmation layout (title row + summary body +
///   agent pills + Reject/Edit/Accept buttons). Today this is the
///   structural stub described in the file-level TODO; the picker
///   integration lands in a fast-follow.
pub(super) fn render_orchestrate(
    props: Props,
    action_id: &AIAgentActionId,
    req: &OrchestrateRequest,
    app: &AppContext,
) -> Box<dyn Element> {
    let appearance = Appearance::as_ref(app);
    let theme = appearance.theme();
    let status = props.action_model.as_ref(app).get_action_status(action_id);

    // Stage 1 invariant: the server defers writing run-wide defaults onto the
    // tool call message until streaming completes (`IsComplete` flips true).
    // The client mirrors that by gating its full layout on the recorded
    // action status — when streaming, render a skeleton rather than a half-
    // populated card. This avoids re-render churn while the LLM is mid-
    // stream.
    if props.model.status(app).is_streaming() {
        return render_streaming_placeholder(req, appearance, app);
    }

    // Stage 2 seam: when auto_launch is true the card skips the
    // confirmation phase and the dispatch helper has already been kicked
    // off. The recorded result drives the visual state.
    let _stage2_auto_launch = req.auto_launch;

    if let Some(AIActionStatus::Finished(result)) = &status {
        if let AIAgentActionResultType::Orchestrate(orchestrate_result) = &result.result {
            return render_terminal_state(req, orchestrate_result, appearance, app);
        }
        log::error!(
            "Unexpected action result type for orchestrate: {:?}",
            result.result
        );
        return Empty::new().finish();
    }

    // Pre-dispatch confirmation layout. See the file-level TODO for the
    // picker integration that converts this from a visual stub into the
    // live interactive card.
    let mut column = Flex::column().with_cross_axis_alignment(CrossAxisAlignment::Stretch);
    column.add_child(render_summary_title_row(req, appearance));
    column.add_child(render_body_text(req, appearance));
    column.add_child(render_agents_footer(req, appearance));
    Container::new(column.finish())
        .with_horizontal_padding(8.)
        .with_vertical_padding(8.)
        .with_background_color(blended_colors::neutral_2(theme))
        .with_corner_radius(CornerRadius::with_all(Radius::Pixels(8.)))
        .finish()
        .with_agent_output_item_spacing(app)
        .finish()
}

fn render_streaming_placeholder(
    req: &OrchestrateRequest,
    appearance: &Appearance,
    app: &AppContext,
) -> Box<dyn Element> {
    let count = req.agent_run_configs.len();
    let label = if count == 0 {
        "Preparing orchestration...".to_string()
    } else {
        format!("Preparing orchestration for {count} agent(s)...")
    };
    render_status_only_card(label, appearance, StatusKind::Pending, app)
}

fn render_terminal_state(
    req: &OrchestrateRequest,
    result: &OrchestrateResult,
    appearance: &Appearance,
    app: &AppContext,
) -> Box<dyn Element> {
    match result {
        OrchestrateResult::Launched { agents, .. } => {
            let total = agents.len();
            let launched = agents
                .iter()
                .filter(|a| matches!(a.kind, OrchestrateAgentOutcomeKind::Launched { .. }))
                .count();
            let label = if launched == total {
                format!("Started {total} agent(s)")
            } else {
                // PRODUCT.md "Post-action card states": the M=0 case
                // (every per-agent dispatch failed) is rendered under
                // "Started M of N agents", not "Failed to start
                // orchestration".
                format!("Started {launched} of {total} agent(s)")
            };
            render_status_only_card(
                label,
                appearance,
                if launched == total {
                    StatusKind::Success
                } else {
                    StatusKind::Mixed
                },
                app,
            )
        }
        OrchestrateResult::LaunchDenied { reason } => {
            // [Stage 2] Disabled-state card. Stage 1 server never emits
            // this variant; this render path is the forward-compat seam.
            let body = if reason.is_empty() {
                "Orchestration is currently disabled. Re-enable on the plan card to launch."
                    .to_string()
            } else {
                format!(
                    "Orchestration is currently disabled. Re-enable on the plan card to launch. ({reason})"
                )
            };
            render_status_only_card(body, appearance, StatusKind::Cancelled, app)
        }
        OrchestrateResult::Failure { error } => {
            let _ = req;
            let label = if error.is_empty() {
                "Failed to start orchestration".to_string()
            } else {
                format!("Failed to start orchestration: {error}")
            };
            render_status_only_card(label, appearance, StatusKind::Failure, app)
        }
        OrchestrateResult::Cancelled => render_status_only_card(
            "Orchestration cancelled".to_string(),
            appearance,
            StatusKind::Cancelled,
            app,
        ),
    }
}

#[derive(Clone, Copy)]
enum StatusKind {
    /// Pre-launch (streaming, dispatch in flight).
    Pending,
    /// All children launched.
    Success,
    /// Some children launched, some failed (or M=0).
    Mixed,
    /// Pre-dispatch failure (couldn't begin the launch sequence).
    Failure,
    /// User rejected, or Stage 2 disapproval Disabled state.
    Cancelled,
}

fn render_status_only_card(
    label: String,
    appearance: &Appearance,
    kind: StatusKind,
    app: &AppContext,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let icon = match kind {
        // Pending and Mixed both reuse the yellow running icon (Mixed = some
        // failures alongside successes; rendered with the same warning vibe
        // as a long-running command). Stage 2 fast-follow can swap in a
        // dedicated mixed-state icon.
        StatusKind::Pending | StatusKind::Mixed => icons::yellow_running_icon(appearance).finish(),
        StatusKind::Success => inline_action_icons::green_check_icon(appearance).finish(),
        StatusKind::Failure => inline_action_icons::red_x_icon(appearance).finish(),
        StatusKind::Cancelled => inline_action_icons::cancelled_icon(appearance).finish(),
    };
    let row = render_requested_action_row_for_text(
        label.into(),
        appearance.ui_font_family(),
        Some(icon),
        None,
        false,
        false,
        app,
    );
    Container::new(row)
        .with_background_color(blended_colors::neutral_2(theme))
        .with_corner_radius(CornerRadius::with_all(Radius::Pixels(8.)))
        .finish()
}

/// Title row for the pre-dispatch confirmation card. Renders the
/// LLM-supplied `summary` text. The Reject (`C`) / Edit (`⌘E`) / Accept
/// (`⌥↵`) buttons are the file-level TODO seam.
fn render_summary_title_row(req: &OrchestrateRequest, appearance: &Appearance) -> Box<dyn Element> {
    let summary = if req.summary.is_empty() {
        format!("Orchestrate {} agent(s)", req.agent_run_configs.len())
    } else {
        req.summary.clone()
    };
    Container::new(
        Text::new(
            summary,
            appearance.ui_font_family(),
            appearance.monospace_font_size(),
        )
        .finish(),
    )
    .with_margin_bottom(4.)
    .finish()
}

fn render_body_text(req: &OrchestrateRequest, appearance: &Appearance) -> Box<dyn Element> {
    // The base prompt isn't user-visible per spec ("Skills and base prompt
    // are passed through verbatim and not displayed."), but we surface
    // execution-mode + harness summary so the user has at-a-glance context
    // about the run-wide configuration before launching.
    let theme = appearance.theme();
    let mode_label = match &req.execution_mode {
        OrchestrateExecutionMode::Local => "Local".to_string(),
        OrchestrateExecutionMode::Remote { environment_id, .. } => {
            if environment_id.is_empty() {
                "Cloud (no environment selected)".to_string()
            } else {
                format!("Cloud · environment={environment_id}")
            }
        }
    };
    let harness = if req.harness_type.is_empty() {
        "default harness"
    } else {
        req.harness_type.as_str()
    };
    let model = if req.model_id.is_empty() {
        "default model"
    } else {
        req.model_id.as_str()
    };
    Container::new(
        Text::new(
            format!(
                "{} agent(s) · {mode_label} · {harness} · {model}",
                req.agent_run_configs.len()
            ),
            appearance.ui_font_family(),
            appearance.monospace_font_size(),
        )
        .with_color(blended_colors::text_disabled(theme, theme.surface_2()))
        .with_selectable(true)
        .finish(),
    )
    .with_margin_bottom(6.)
    .finish()
}

/// Agents footer: `Agents (N)` label + named pills, one per
/// `agent_run_configs[i].name`. Stub renders one row per agent so the count
/// + names are visible. The proper Figma pill styling (deterministic color
/// + initial-letter avatar via `PillSpec`) lands as a fast-follow once
/// `OrchestrationPillBar` exposes its primitives publicly; today they're
/// private to `agent_view::orchestration_pill_bar`.
fn render_agents_footer(req: &OrchestrateRequest, appearance: &Appearance) -> Box<dyn Element> {
    let theme = appearance.theme();
    let mut column = Flex::column().with_cross_axis_alignment(CrossAxisAlignment::Stretch);
    column.add_child(
        Text::new(
            format!("Agents ({})", req.agent_run_configs.len()),
            appearance.ui_font_family(),
            appearance.monospace_font_size(),
        )
        .with_color(blended_colors::text_disabled(theme, theme.surface_2()))
        .finish(),
    );
    for cfg in &req.agent_run_configs {
        column.add_child(
            Container::new(
                Text::new(
                    format!("• {}", cfg.name),
                    appearance.ui_font_family(),
                    appearance.monospace_font_size(),
                )
                .finish(),
            )
            .with_margin_left(8.)
            .finish(),
        );
    }
    column.finish()
}

#[cfg(test)]
#[path = "orchestrate_tests.rs"]
mod tests;
