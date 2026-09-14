use super::PageContext;
use crate::app::actions::{IncrementCounter, NavigateComponents, RunSync};
use crate::app::components::{Badge, BadgeTone, Button, ButtonSize, ButtonVariant};
use crate::app::state::SyncState;
use crate::app::theme::{Theme, ThemeMode};
use gpui::{AnyElement, IntoElement, SharedString, div, prelude::*, px};

pub(super) fn render(context: &PageContext<'_>) -> AnyElement {
    let theme = context.theme;
    let (status, detail, tone) = match context.sync {
        SyncState::Idle => (
            "Ready",
            "Run a sample job to see background work in action.",
            BadgeTone::Neutral,
        ),
        SyncState::Running => (
            "Running",
            "Working in the background. You can keep exploring.",
            BadgeTone::Accent,
        ),
        SyncState::Complete { .. } => (
            "Complete",
            "The sample job finished. Run it again any time.",
            BadgeTone::Success,
        ),
    };

    div()
        .flex()
        .flex_col()
        .gap_8()
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .pt_3()
                .child(
                    div()
                        .text_3xl()
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .child("Make it yours."),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(theme.text_muted)
                        .child("A small native workspace, ready for your next idea."),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .rounded_lg()
                .border_1()
                .border_color(theme.border)
                .bg(theme.surface)
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .gap_4()
                        .p_5()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_3()
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(gpui::FontWeight::SEMIBOLD)
                                        .child("Background job"),
                                )
                                .child(Badge::new("Demo")),
                        )
                        .child(Badge::new(status).tone(tone)),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .gap_6()
                        .px_5()
                        .pb_6()
                        .when(context.narrow, |row| row.flex_col().items_start())
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_2()
                                .child(
                                    div()
                                        .text_xl()
                                        .font_weight(gpui::FontWeight::MEDIUM)
                                        .child("Sync your workspace"),
                                )
                                .child(div().text_sm().text_color(theme.text_muted).child(detail)),
                        )
                        .child(
                            Button::new(
                                "run-sync",
                                if context.sync.is_running() {
                                    "Syncing…"
                                } else {
                                    "Run demo sync"
                                },
                                RunSync,
                            )
                            .variant(ButtonVariant::Primary)
                            .disabled(context.sync.is_running()),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .border_t_1()
                        .border_color(theme.border)
                        .when(context.narrow, |row| row.flex_col())
                        .child(stat(
                            "Records processed",
                            match context.sync {
                                SyncState::Complete { records } => records.to_string(),
                                _ => "—".into(),
                            },
                            theme,
                        ))
                        .child(stat("Data source", "Sample data", theme))
                        .child(stat("Connection", "Local simulation", theme)),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_4()
                .child(
                    div()
                        .text_sm()
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .child("Explore the workspace"),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .border_t_1()
                        .border_color(theme.border)
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .gap_4()
                                .py_5()
                                .border_b_1()
                                .border_color(theme.border)
                                .child(row_description(
                                    "Shared state",
                                    "Changes stay with you as you move between pages.",
                                    theme,
                                ))
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_4()
                                        .child(
                                            div()
                                                .text_lg()
                                                .font_weight(gpui::FontWeight::SEMIBOLD)
                                                .child(context.counter.to_string()),
                                        )
                                        .child(
                                            Button::new(
                                                "overview-increment",
                                                "Add one",
                                                IncrementCounter,
                                            )
                                            .size(ButtonSize::Small),
                                        ),
                                ),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .gap_4()
                                .py_5()
                                .border_b_1()
                                .border_color(theme.border)
                                .child(row_description(
                                    "Interface kit",
                                    "Buttons, status labels, and controls you can reuse.",
                                    theme,
                                ))
                                .child(
                                    Button::new(
                                        "browse-components",
                                        "Explore controls",
                                        NavigateComponents,
                                    )
                                    .size(ButtonSize::Small),
                                ),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .gap_4()
                                .py_5()
                                .border_b_1()
                                .border_color(theme.border)
                                .child(row_description(
                                    "Your preferences",
                                    "Appearance and navigation are saved on this device.",
                                    theme,
                                ))
                                .child(Badge::new(if context.settings.theme == ThemeMode::Dark {
                                    "Dark theme"
                                } else {
                                    "Light theme"
                                })),
                        ),
                ),
        )
        .child(
            div()
                .text_xs()
                .text_color(theme.text_faint)
                .child("Start small. Replace this workspace with something useful."),
        )
        .into_any_element()
}

fn stat(label: &'static str, value: impl Into<SharedString>, theme: &Theme) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .flex_1()
        .min_w(px(0.0))
        .gap_2()
        .p_5()
        .child(div().text_xs().text_color(theme.text_faint).child(label))
        .child(
            div()
                .text_sm()
                .font_weight(gpui::FontWeight::MEDIUM)
                .child(value.into()),
        )
}

fn row_description(title: &'static str, detail: &'static str, theme: &Theme) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .flex_1()
        .min_w(px(0.0))
        .gap_1()
        .child(
            div()
                .text_sm()
                .font_weight(gpui::FontWeight::MEDIUM)
                .child(title),
        )
        .child(div().text_xs().text_color(theme.text_muted).child(detail))
}
