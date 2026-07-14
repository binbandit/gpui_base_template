use crate::app::actions::ToggleTheme;
use crate::app::components::{Badge, BadgeTone, Button, ButtonSize, ButtonVariant};
use crate::app::services::AppSettings;
use crate::app::state::{Route, SyncState};
use crate::app::theme::{Theme, ThemeMode};
use gpui::{IntoElement, div, prelude::*, px};

pub(crate) fn render(
    route: Route,
    sync: SyncState,
    settings: &AppSettings,
    compact: bool,
    theme: &Theme,
) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .justify_between()
        .h(px(76.0))
        .px_6()
        .border_b_1()
        .border_color(theme.border)
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_xs()
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .text_color(theme.accent)
                        .child(route.eyebrow()),
                )
                .child(
                    div()
                        .text_xl()
                        .font_weight(gpui::FontWeight::BOLD)
                        .text_color(theme.text)
                        .child(route.title()),
                ),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .when(!compact, |actions| {
                    actions.child(Badge::new(sync.label()).tone(if sync.is_running() {
                        BadgeTone::Warning
                    } else {
                        BadgeTone::Neutral
                    }))
                })
                .child(
                    Button::new(
                        "theme-button",
                        if settings.theme == ThemeMode::Dark {
                            "Light mode"
                        } else {
                            "Dark mode"
                        },
                        ToggleTheme,
                    )
                    .variant(ButtonVariant::Ghost)
                    .size(ButtonSize::Small),
                ),
        )
}
