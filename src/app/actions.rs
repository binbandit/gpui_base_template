//! Logical application actions and their default shortcuts.

use gpui::{App, KeyBinding, actions};

actions!(
    starter,
    [
        NavigateOverview,
        NavigateComponents,
        NavigateSettings,
        ToggleTheme,
        IncrementCounter,
        RunSync,
        ToggleCompactSidebar,
        DismissNotice,
        FocusNext,
        FocusPrevious,
        Quit,
    ]
);

pub(crate) fn bind_keys(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("tab", FocusNext, Some("Starter")),
        KeyBinding::new("shift-tab", FocusPrevious, Some("Starter")),
        KeyBinding::new("secondary-1", NavigateOverview, Some("Starter")),
        KeyBinding::new("secondary-2", NavigateComponents, Some("Starter")),
        KeyBinding::new("secondary-,", NavigateSettings, Some("Starter")),
        KeyBinding::new("secondary-shift-t", ToggleTheme, Some("Starter")),
        KeyBinding::new("secondary-r", RunSync, Some("Starter")),
        KeyBinding::new("secondary-q", Quit, Some("Starter")),
        KeyBinding::new("escape", DismissNotice, Some("Starter")),
    ]);
}
