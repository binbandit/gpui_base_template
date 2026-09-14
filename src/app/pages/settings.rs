use super::PageContext;
use crate::app::actions::{ToggleCompactSidebar, ToggleTheme};
use crate::app::components::{Card, Toggle};
use crate::app::theme::{Theme, ThemeMode};
use gpui::{AnyElement, IntoElement, div, prelude::*};

pub(super) fn render(context: &PageContext<'_>) -> AnyElement {
    let theme = context.theme;

    div()
        .flex()
        .flex_col()
        .gap_6()
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
                        .child("A space that feels like yours."),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(theme.text_muted)
                        .child("Changes are saved automatically on this device."),
                ),
        )
        .child(
            Card::new()
                .title("Appearance")
                .child(Toggle::new(
                    "theme-setting",
                    "Light theme",
                    "Use a bright canvas and light surfaces.",
                    context.settings.theme == ThemeMode::Light,
                    ToggleTheme,
                ))
                .child(Toggle::new(
                    "compact-setting",
                    "Compact navigation",
                    "Show icons in the sidebar. Small windows use this automatically.",
                    context.settings.compact_sidebar,
                    ToggleCompactSidebar,
                )),
        )
        .child(
            Card::new()
                .title("Keyboard shortcuts")
                .child(shortcut_row(
                    "Overview",
                    command_shortcut("⌘1", "Ctrl+1"),
                    theme,
                ))
                .child(shortcut_row(
                    "Components",
                    command_shortcut("⌘2", "Ctrl+2"),
                    theme,
                ))
                .child(shortcut_row(
                    "Settings",
                    command_shortcut("⌘,", "Ctrl+,"),
                    theme,
                ))
                .child(shortcut_row(
                    "Switch theme",
                    command_shortcut("⌘⇧T", "Ctrl+Shift+T"),
                    theme,
                ))
                .child(shortcut_row(
                    "Run demo sync",
                    command_shortcut("⌘R", "Ctrl+R"),
                    theme,
                ))
                .child(shortcut_row(
                    "Move between controls",
                    "Tab / Shift+Tab",
                    theme,
                ))
                .child(shortcut_row(
                    "Quit",
                    command_shortcut("⌘Q", "Ctrl+Q"),
                    theme,
                )),
        )
        .into_any_element()
}

fn command_shortcut(macos: &'static str, other: &'static str) -> &'static str {
    if cfg!(target_os = "macos") {
        macos
    } else {
        other
    }
}

fn shortcut_row(label: &'static str, shortcut: &'static str, theme: &Theme) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .justify_between()
        .gap_4()
        .py_1()
        .text_sm()
        .child(div().text_color(theme.text_muted).child(label))
        .child(
            div()
                .px_2()
                .py_1()
                .rounded_md()
                .border_1()
                .border_color(theme.border)
                .bg(theme.surface_muted)
                .text_xs()
                .text_color(theme.text)
                .child(shortcut),
        )
}
