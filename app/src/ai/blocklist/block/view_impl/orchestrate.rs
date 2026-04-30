//! Renders the inline confirmation card for an `orchestrate` tool call.
//!
//! Modeled on the `orchestration.rs` sibling that renders `start_agent_v2`.
//! Stage 1 covers the user-facing card states + interactive
//! Reject / Edit / Accept buttons, the inline editor (Local/Cloud toggle,
//! model/harness/environment dropdown pickers, worker_host text-only with
//! TODO), and the Cloud-without-env / OpenCode+Cloud Accept-disabled
//! validation gating.
//!
//! Spec references: TECH.md §8 ("Client: confirmation card"), §9
//! ("Per-agent task creation"), PRODUCT.md "Confirmation card" + "Post-action
//! card states" + "Invariants".

use ai::agent::action::{OrchestrateExecutionMode, OrchestrateRequest};
use ai::agent::action_result::{OrchestrateAgentOutcomeKind, OrchestrateResult};
use pathfinder_color::ColorU;
use warpui::elements::{
    ChildView, Container, CornerRadius, CrossAxisAlignment, Empty, Flex, Hoverable,
    MainAxisAlignment, MainAxisSize, MouseStateHandle, ParentElement, Radius, Text,
};
use warpui::platform::Cursor;
use warpui::{AppContext, Element, SingletonEntity};

use crate::ai::agent::icons;
use crate::ai::agent::{AIAgentActionId, AIAgentActionResultType};
use crate::ai::blocklist::action_model::AIActionStatus;
use crate::ai::blocklist::block::{AIBlockAction, OrchestrateCardHandles, OrchestrateEditState};
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
pub(super) fn render_orchestrate(
    props: Props,
    action_id: &AIAgentActionId,
    req: &OrchestrateRequest,
    app: &AppContext,
) -> Box<dyn Element> {
    let appearance = Appearance::as_ref(app);
    let theme = appearance.theme();
    let status = props.action_model.as_ref(app).get_action_status(action_id);

    if props.model.status(app).is_streaming() {
        return render_streaming_placeholder(req, appearance, app);
    }

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
    // source of truth until the user clicks Edit.
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

    let mut column = Flex::column().with_cross_axis_alignment(CrossAxisAlignment::Stretch);
    column.add_child(render_summary_title_row(
        action_id,
        &display_state,
        &handles,
        appearance,
    ));
    column.add_child(render_body_text(&display_state, appearance));
    column.add_child(render_agents_footer(&display_state, appearance));
    if display_state.is_editor_open {
        column.add_child(render_editor(
            action_id,
            &display_state,
            &handles,
            appearance,
        ));
    }
    if let Some(reason) = display_state.accept_disabled_reason() {
        column.add_child(render_validation_error(reason, appearance));
    }
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
    Pending,
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

fn render_summary_title_row(
    action_id: &AIAgentActionId,
    state: &OrchestrateEditState,
    handles: &OrchestrateCardHandles,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let summary = if state.summary.is_empty() {
        format!("Orchestrate {} agent(s)", state.agent_run_configs.len())
    } else {
        state.summary.clone()
    };
    let title = Text::new(
        summary,
        appearance.ui_font_family(),
        appearance.monospace_font_size(),
    )
    .finish();

    let edit_label = if state.is_editor_open {
        "Done editing"
    } else {
        "Edit"
    };
    let accept_disabled = state.accept_disabled_reason().is_some();

    let reject_button = render_card_button(
        "Reject",
        AIBlockAction::OrchestrateReject {
            action_id: action_id.clone(),
        },
        handles.reject_button.clone(),
        false,
        appearance,
    );
    let edit_button = render_card_button(
        edit_label,
        AIBlockAction::OrchestrateToggleEdit {
            action_id: action_id.clone(),
        },
        handles.edit_button.clone(),
        false,
        appearance,
    );
    let accept_button = render_card_button(
        "Accept",
        AIBlockAction::OrchestrateAccept {
            action_id: action_id.clone(),
        },
        handles.accept_button.clone(),
        accept_disabled,
        appearance,
    );

    let buttons = Flex::row()
        .with_main_axis_alignment(MainAxisAlignment::End)
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_main_axis_size(MainAxisSize::Min)
        .with_child(reject_button)
        .with_child(edit_button)
        .with_child(accept_button)
        .finish();

    let row = Flex::row()
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_main_axis_size(MainAxisSize::Max)
        .with_main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .with_child(title)
        .with_child(buttons)
        .finish();
    Container::new(row).with_margin_bottom(4.).finish()
}

fn render_body_text(state: &OrchestrateEditState, appearance: &Appearance) -> Box<dyn Element> {
    let theme = appearance.theme();
    let mode_label = match &state.execution_mode {
        OrchestrateExecutionMode::Local => "Local".to_string(),
        OrchestrateExecutionMode::Remote { environment_id, .. } => {
            if environment_id.is_empty() {
                "Cloud (no environment selected)".to_string()
            } else {
                format!("Cloud · environment={environment_id}")
            }
        }
    };
    let harness = if state.harness_type.is_empty() {
        "default harness"
    } else {
        state.harness_type.as_str()
    };
    let model = if state.model_id.is_empty() {
        "default model"
    } else {
        state.model_id.as_str()
    };
    Container::new(
        Text::new(
            format!(
                "{} agent(s) · {mode_label} · {harness} · {model}",
                state.agent_run_configs.len()
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

fn render_agents_footer(state: &OrchestrateEditState, appearance: &Appearance) -> Box<dyn Element> {
    let theme = appearance.theme();
    let mut column = Flex::column().with_cross_axis_alignment(CrossAxisAlignment::Stretch);
    column.add_child(
        Text::new(
            format!("Agents ({})", state.agent_run_configs.len()),
            appearance.ui_font_family(),
            appearance.monospace_font_size(),
        )
        .with_color(blended_colors::text_disabled(theme, theme.surface_2()))
        .finish(),
    );
    for cfg in &state.agent_run_configs {
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
                .with_color(blended_colors::text_disabled(theme, theme.surface_2()))
                .finish(),
            )
            .with_margin_top(4.)
            .finish(),
        );
    }

    Container::new(column.finish())
        .with_horizontal_padding(8.)
        .with_vertical_padding(8.)
        .with_margin_top(6.)
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
    .with_color(blended_colors::text_disabled(theme, theme.surface_2()))
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
    .with_color(blended_colors::text_disabled(theme, theme.surface_2()))
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

fn render_card_button(
    label: &str,
    on_click: AIBlockAction,
    mouse_state: MouseStateHandle,
    disabled: bool,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let bg: ColorU = if disabled {
        theme.surface_1().into()
    } else {
        theme.surface_2().into()
    };
    let text_color = if disabled {
        blended_colors::text_disabled(theme, theme.surface_2())
    } else {
        theme.main_text_color(theme.background()).into()
    };
    let label_owned = label.to_string();
    let font_family = appearance.ui_font_family();
    let font_size = appearance.monospace_font_size();
    let inner_factory = move || {
        Container::new(
            Text::new(label_owned.clone(), font_family, font_size)
                .with_color(text_color)
                .finish(),
        )
        .with_horizontal_padding(8.)
        .with_vertical_padding(4.)
        .with_corner_radius(CornerRadius::with_all(Radius::Pixels(4.)))
        .with_background_color(bg)
        .finish()
    };
    if disabled {
        return Container::new(inner_factory())
            .with_margin_left(4.)
            .finish();
    }
    let hoverable = Hoverable::new(mouse_state, move |_| inner_factory())
        .on_click(move |ctx, _, _| {
            ctx.dispatch_typed_action(on_click.clone());
        })
        .with_cursor(Cursor::PointingHand)
        .finish();
    Container::new(hoverable).with_margin_left(4.).finish()
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
    .with_margin_top(6.)
    .finish()
}

#[cfg(test)]
#[path = "orchestrate_tests.rs"]
mod tests;
