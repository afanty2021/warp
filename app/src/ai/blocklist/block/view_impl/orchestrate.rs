//! Renders the inline confirmation card for an `orchestrate` tool call.
//!
//! Visual structure (Figma-driven):
//!  - A code-block-style outer shell: 1px border + rounded 8px corners. The
//!    border switches to the theme accent when the card is blocked on user
//!    confirmation, mirroring `requested_command.rs`.
//!  - A header bar (rendered via [`HeaderConfig`]) containing the static
//!    "Can I add additional agents to this task?" title, a leading
//!    `stop-filled` accent icon, and the Reject / Edit / Accept action
//!    cluster on the trailing edge.
//!  - A body region (`theme.background()` fill) holding, in order: an
//!    optional validation-error line (theme error color), the LLM-supplied
//!    `summary` text followed inline by a `[⌘][E]` keyboard chip, an
//!    `Agents (N)` label, and a horizontal row of static agent pills.
//!  - When the inline editor is open, an inset surface_1 panel is appended
//!    below the body containing the Local/Cloud toggle and the
//!    Model / Harness / Environment dropdown pickers.
//!
//! Spec references: TECH.md §8, §9; PRODUCT.md "Confirmation card",
//! "Post-action card states", "Invariants".

use ai::agent::action::{OrchestrateExecutionMode, OrchestrateRequest};
use ai::agent::action_result::{OrchestrateAgentOutcomeKind, OrchestrateResult};
use std::rc::Rc;
use warpui::elements::{
    Border, ChildView, Container, CornerRadius, CrossAxisAlignment, Empty, Flex, Hoverable,
    MainAxisSize, MouseStateHandle, ParentElement, Radius, Text,
};
use warpui::keymap::Keystroke;
use warpui::platform::Cursor;
use warpui::ui_components::components::UiComponent;
use warpui::{AppContext, Element, SingletonEntity};

use crate::ai::agent::icons;
use crate::ai::agent::{AIAgentActionId, AIAgentActionResultType};
use crate::ai::blocklist::action_model::AIActionStatus;
use crate::ai::blocklist::agent_view::orchestration_pill_bar::render_static_agent_pill;
use crate::ai::blocklist::block::{AIBlockAction, OrchestrateCardHandles, OrchestrateEditState};
use crate::ai::blocklist::inline_action::inline_action_header::{HeaderConfig, InteractionMode};
use crate::ai::blocklist::inline_action::inline_action_icons;
use crate::ai::blocklist::inline_action::requested_action::render_requested_action_row_for_text;
use crate::appearance::Appearance;
use crate::ui_components::blended_colors;
use crate::view_components::compactible_action_button::{
    RenderCompactibleActionButton, MEDIUM_SIZE_SWITCH_THRESHOLD,
};

use super::output::Props;
use super::WithContentItemSpacing;

/// Static title rendered in the orchestrate confirmation card header. Per
/// spec §8 this is invariant client copy; the LLM-supplied `summary` field
/// is repurposed as the body description.
const ORCHESTRATE_CARD_TITLE: &str = "Can I add additional agents to this task?";

/// Renders the full orchestrate confirmation card.
///
/// Dispatched from the tool-call view dispatcher in `output.rs`. The card
/// is gated on `FeatureFlag::OrchestrateTool` at the dispatcher level; when
/// the flag is off this function is never reached.
pub(super) fn render_orchestrate(
    props: Props,
    action_id: &AIAgentActionId,
    req: &OrchestrateRequest,
    app: &AppContext,
) -> Box<dyn Element> {
    let appearance = Appearance::as_ref(app);
    let status = props.action_model.as_ref(app).get_action_status(action_id);

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

    // Pre-dispatch confirmation layout. Pulls per-action edit state +
    // button handles from the AIBlock; the LLM-supplied request is the
    // source of truth until the user clicks Edit. Per the polish round
    // (P2.4) we no longer render a separate "Preparing orchestration..."
    // placeholder during streaming — the confirmation card stands in for
    // that intermediate state, mirroring how the edit/apply-diff
    // tool-call card behaves before the user accepts.
    let display_state = props
        .orchestrate_edit_states
        .get(action_id)
        .cloned()
        .unwrap_or_else(|| OrchestrateEditState::from_request(req));
    let handles = props
        .orchestrate_card_handles
        .get(action_id)
        .cloned()
        .unwrap_or_default();

    let is_blocked = matches!(status, Some(AIActionStatus::Blocked));
    render_confirmation_card(action_id, &display_state, &handles, is_blocked, app)
}

fn render_confirmation_card(
    action_id: &AIAgentActionId,
    state: &OrchestrateEditState,
    handles: &OrchestrateCardHandles,
    is_blocked: bool,
    app: &AppContext,
) -> Box<dyn Element> {
    let appearance = Appearance::as_ref(app);
    let theme = appearance.theme();

    let header = render_header(handles, app);
    let body = render_body(state, app);

    let mut content = Flex::column()
        .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
        .with_child(header)
        .with_child(body);

    if state.is_editor_open {
        content.add_child(render_editor(action_id, state, handles, appearance));
    }

    let border_color = if is_blocked {
        theme.accent()
    } else {
        theme.surface_2()
    };

    Container::new(content.finish())
        .with_corner_radius(CornerRadius::with_all(Radius::Pixels(8.)))
        .with_border(Border::all(1.).with_border_fill(border_color))
        .finish()
        .with_agent_output_item_spacing(app)
        .finish()
}

fn render_header(handles: &OrchestrateCardHandles, app: &AppContext) -> Box<dyn Element> {
    let appearance = Appearance::as_ref(app);
    let mut config = HeaderConfig::new(ORCHESTRATE_CARD_TITLE, app)
        .with_icon(icons::yellow_stop_icon(appearance))
        .with_corner_radius_override(CornerRadius::with_top(Radius::Pixels(8.)));

    if let (Some(reject), Some(edit), Some(accept)) = (
        handles.reject_button.as_ref(),
        handles.edit_button.as_ref(),
        handles.accept_button.as_ref(),
    ) {
        let action_buttons: Vec<Rc<dyn RenderCompactibleActionButton>> = vec![
            Rc::new(reject.clone()),
            Rc::new(edit.clone()),
            Rc::new(accept.clone()),
        ];
        config = config.with_interaction_mode(InteractionMode::ActionButtons {
            action_buttons,
            size_switch_threshold: MEDIUM_SIZE_SWITCH_THRESHOLD,
        });
    }

    config.render(app)
}

fn render_body(state: &OrchestrateEditState, app: &AppContext) -> Box<dyn Element> {
    let appearance = Appearance::as_ref(app);
    let theme = appearance.theme();
    let mut column = Flex::column().with_cross_axis_alignment(CrossAxisAlignment::Stretch);

    // The validation error appears above the summary so it sits between the
    // header (where the buttons live) and the rest of the body, satisfying
    // spec §8's "above the buttons" directive.
    if let Some(reason) = state.accept_disabled_reason() {
        column.add_child(render_validation_error(reason, appearance));
    }

    column.add_child(render_summary_with_edit_chip(state, appearance));
    column.add_child(render_agents_section(state, app));

    Container::new(column.finish())
        .with_horizontal_padding(16.)
        .with_vertical_padding(12.)
        .with_background_color(theme.background().into_solid())
        .with_corner_radius(CornerRadius::with_bottom(Radius::Pixels(8.)))
        .finish()
}

fn render_summary_with_edit_chip(
    state: &OrchestrateEditState,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let summary = if state.summary.trim().is_empty() {
        format!(
            "Spawn {} agent(s) to address this task.",
            state.agent_run_configs.len()
        )
    } else {
        state.summary.clone()
    };
    let summary_text = Text::new(
        summary,
        appearance.ui_font_family(),
        appearance.monospace_font_size(),
    )
    .with_color(blended_colors::text_main(theme, theme.background()))
    .with_selectable(true)
    .finish();

    let row = Flex::row()
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_main_axis_size(MainAxisSize::Min)
        .with_spacing(8.)
        .with_child(summary_text)
        .with_child(render_edit_keyboard_chip(appearance))
        .finish();

    Container::new(row).with_margin_bottom(8.).finish()
}

fn render_edit_keyboard_chip(appearance: &Appearance) -> Box<dyn Element> {
    let keystroke = Keystroke::parse("cmdorctrl-e")
        .expect("orchestrate card edit keystroke literal must parse");
    appearance
        .ui_builder()
        .keyboard_shortcut(&keystroke)
        .build()
        .finish()
}

fn render_agents_section(state: &OrchestrateEditState, app: &AppContext) -> Box<dyn Element> {
    let appearance = Appearance::as_ref(app);
    let theme = appearance.theme();
    let label = Text::new(
        format!("Agents ({})", state.agent_run_configs.len()),
        appearance.ui_font_family(),
        appearance.monospace_font_size() - 1.,
    )
    .with_color(blended_colors::text_disabled(theme, theme.background()))
    .finish();

    let mut pills_row = Flex::row()
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_main_axis_size(MainAxisSize::Min)
        .with_spacing(4.);
    for cfg in &state.agent_run_configs {
        pills_row.add_child(render_static_agent_pill(&cfg.name, app));
    }

    Flex::column()
        .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
        .with_child(Container::new(label).with_margin_bottom(6.).finish())
        .with_child(pills_row.finish())
        .finish()
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
            // Per P2.3, all-success uses "Spawned N agent(s)" with proper
            // pluralization; mixed uses "Spawned X of Y agents".
            let label = if launched == total {
                if total == 1 {
                    "Spawned 1 agent".to_string()
                } else {
                    format!("Spawned {total} agents")
                }
            } else {
                format!("Spawned {launched} of {total} agents")
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
    Success,
    Mixed,
    Failure,
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
        StatusKind::Mixed => icons::yellow_running_icon(appearance).finish(),
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
    // Match the confirmation card's outer spacing (P2.6) so the post-action
    // card sits in the same indentation lane and has consistent bottom
    // breathing room before the next output item.
    Container::new(row)
        .with_background_color(blended_colors::neutral_2(theme))
        .with_corner_radius(CornerRadius::with_all(Radius::Pixels(8.)))
        .finish()
        .with_agent_output_item_spacing(app)
        .finish()
}

fn render_editor(
    action_id: &AIAgentActionId,
    state: &OrchestrateEditState,
    handles: &OrchestrateCardHandles,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let mut column = Flex::column().with_cross_axis_alignment(CrossAxisAlignment::Stretch);

    column.add_child(render_mode_toggle(action_id, state, handles, appearance));
    if let Some(model_picker) = &handles.model_picker {
        column.add_child(render_picker_row(
            "Model",
            ChildView::new(model_picker).finish(),
            appearance,
        ));
    }
    if let Some(harness_picker) = &handles.harness_picker {
        column.add_child(render_picker_row(
            "Harness",
            ChildView::new(harness_picker).finish(),
            appearance,
        ));
    }
    if let OrchestrateExecutionMode::Remote { worker_host, .. } = &state.execution_mode {
        if let Some(env_picker) = &handles.environment_picker {
            column.add_child(render_picker_row(
                "Environment",
                ChildView::new(env_picker).finish(),
                appearance,
            ));
        }
        column.add_child(
            Container::new(
                Text::new(
                    format!(
                        "Worker host: {} (TODO: editable picker)",
                        if worker_host.is_empty() {
                            "warp"
                        } else {
                            worker_host.as_str()
                        }
                    ),
                    appearance.ui_font_family(),
                    appearance.monospace_font_size(),
                )
                .with_color(blended_colors::text_disabled(theme, theme.surface_1()))
                .finish(),
            )
            .with_margin_top(4.)
            .finish(),
        );
    }

    Container::new(column.finish())
        .with_horizontal_padding(8.)
        .with_vertical_padding(8.)
        .with_margin_left(16.)
        .with_margin_right(16.)
        .with_margin_bottom(12.)
        .with_background_color(theme.surface_1().into())
        .with_corner_radius(CornerRadius::with_all(Radius::Pixels(6.)))
        .finish()
}

fn render_mode_toggle(
    action_id: &AIAgentActionId,
    state: &OrchestrateEditState,
    handles: &OrchestrateCardHandles,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let is_remote = state.execution_mode.is_remote();
    let label = Text::new(
        "Run on:".to_string(),
        appearance.ui_font_family(),
        appearance.monospace_font_size(),
    )
    .with_color(blended_colors::text_disabled(theme, theme.surface_1()))
    .finish();
    let local_button = render_segment_button(
        "Local",
        !is_remote,
        AIBlockAction::OrchestrateExecutionModeToggled {
            action_id: action_id.clone(),
            is_remote: false,
        },
        handles.local_toggle.clone(),
        appearance,
    );
    let cloud_button = render_segment_button(
        "Cloud",
        is_remote,
        AIBlockAction::OrchestrateExecutionModeToggled {
            action_id: action_id.clone(),
            is_remote: true,
        },
        handles.cloud_toggle.clone(),
        appearance,
    );
    let row = Flex::row()
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_main_axis_size(MainAxisSize::Min)
        .with_child(Container::new(label).with_margin_right(8.).finish())
        .with_child(local_button)
        .with_child(cloud_button)
        .finish();
    Container::new(row).with_margin_bottom(6.).finish()
}

fn render_picker_row(
    label: &str,
    picker: Box<dyn Element>,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let label_el = Text::new(
        format!("{label}:"),
        appearance.ui_font_family(),
        appearance.monospace_font_size(),
    )
    .with_color(blended_colors::text_disabled(theme, theme.surface_1()))
    .finish();
    let row = Flex::row()
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_main_axis_size(MainAxisSize::Min)
        .with_child(Container::new(label_el).with_margin_right(8.).finish())
        .with_child(picker)
        .finish();
    Container::new(row).with_margin_bottom(4.).finish()
}

fn render_segment_button(
    label: &str,
    is_active: bool,
    on_click: AIBlockAction,
    mouse_state: MouseStateHandle,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let bg = if is_active {
        theme.surface_3().into()
    } else {
        theme.surface_2().into()
    };
    let label_owned = label.to_string();
    let font_family = appearance.ui_font_family();
    let font_size = appearance.monospace_font_size();
    let hoverable = Hoverable::new(mouse_state, move |_| {
        Container::new(Text::new(label_owned.clone(), font_family, font_size).finish())
            .with_horizontal_padding(8.)
            .with_vertical_padding(4.)
            .with_corner_radius(CornerRadius::with_all(Radius::Pixels(4.)))
            .with_background_color(bg)
            .finish()
    })
    .on_click(move |ctx, _, _| {
        ctx.dispatch_typed_action(on_click.clone());
    })
    .with_cursor(Cursor::PointingHand)
    .finish();
    Container::new(hoverable).with_margin_right(4.).finish()
}

fn render_validation_error(reason: &str, appearance: &Appearance) -> Box<dyn Element> {
    let theme = appearance.theme();
    Container::new(
        Text::new(
            reason.to_string(),
            appearance.ui_font_family(),
            appearance.monospace_font_size(),
        )
        .with_color(theme.ui_error_color())
        .finish(),
    )
    .with_margin_bottom(8.)
    .finish()
}

#[cfg(test)]
#[path = "orchestrate_tests.rs"]
mod tests;
