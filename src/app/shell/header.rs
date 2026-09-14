use crate::app::actions::ToggleTheme;
use crate::app::components::{Button, ButtonSize, ButtonVariant};
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
        .flex_none()
        .items_center()
        .justify_between()
        .h(px(56.0))
        .px_6()
        .border_b_1()
        .border_color(theme.border)
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .text_sm()
                .when(!compact, |crumb| {
                    crumb
                        .child(div().text_color(theme.text_faint).child("Workspace"))
                        .child(div().text_color(theme.text_faint).child("/"))
                })
                .child(div().text_color(theme.text).child(route.title())),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .when(sync.is_running(), |actions| {
                    actions.child(
                        div()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child("Syncing…"),
                    )
                })
                .child(
                    Button::new(
                        "theme-button",
                        if settings.theme == ThemeMode::Dark {
                            "Light theme"
                        } else {
                            "Dark theme"
                        },
                        ToggleTheme,
                    )
                    .variant(ButtonVariant::Ghost)
                    .size(ButtonSize::Small),
                ),
        )
}
