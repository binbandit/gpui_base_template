use super::PageContext;
use crate::app::actions::{ToggleAnimations, ToggleCompactSidebar, ToggleTheme};
use crate::app::components::{Card, Toggle};
use crate::app::theme::{Theme, ThemeMode};
use gpui::{AnyElement, IntoElement, div, prelude::*};

pub(super) fn render(context: &PageContext<'_>) -> AnyElement {
    let theme = context.theme;

    div()
        .flex()
        .flex_col()
        .gap_5()
        .child(
            Card::new()
                .title("Appearance")
                .child(Toggle::new(
                    "theme-setting",
                    "Use light theme",
                    "Theme changes are semantic and persisted to the platform config directory.",
                    context.settings.theme == ThemeMode::Light,
                    ToggleTheme,
                ))
                .child(Toggle::new(
                    "animation-setting",
                    "Enable animation",
                    "A single preference future features can consistently honor.",
                    context.settings.animations,
                    ToggleAnimations,
                )),
        )
        .child(Card::new().title("Workspace").child(Toggle::new(
            "compact-setting",
            "Compact sidebar",
            "Collapse labels while preserving keyboard navigation and tool structure.",
            context.settings.compact_sidebar,
            ToggleCompactSidebar,
        )))
        .child(
            Card::new()
                .title("Keyboard map")
                .child(shortcut_row(
                    "Navigate",
                    command_shortcut("⌘1 / ⌘2 / ⌘,", "Ctrl+1 / Ctrl+2 / Ctrl+,"),
                    theme,
                ))
                .child(shortcut_row(
                    "Toggle theme",
                    command_shortcut("⌘⇧T", "Ctrl+Shift+T"),
                    theme,
                ))
                .child(shortcut_row(
                    "Run async demo",
                    command_shortcut("⌘R", "Ctrl+R"),
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
        .py_2()
        .border_b_1()
        .border_color(theme.border)
        .text_sm()
        .text_color(theme.text_muted)
        .child(label)
        .child(
            div()
                .px_2()
                .py_1()
                .rounded_md()
                .bg(theme.surface_muted)
                .font_family("monospace")
                .text_xs()
                .text_color(theme.text)
                .child(shortcut),
        )
}
