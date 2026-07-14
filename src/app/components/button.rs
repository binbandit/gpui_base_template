use crate::app::theme::Theme;
use gpui::{
    Action, App, ElementId, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px,
};

/// Visual emphasis for a [`Button`].
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Ghost,
    Danger,
}

/// Supported button density presets.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum ButtonSize {
    Small,
    Medium,
}

/// Focusable action button with unified pointer and keyboard behavior.
///
/// The generic action is dispatched for click, Enter, and Space activation.
#[derive(IntoElement)]
pub struct Button<A: Action + Clone> {
    id: ElementId,
    label: SharedString,
    action: A,
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
}

impl<A: Action + Clone> Button<A> {
    /// Creates an enabled, medium, secondary button.
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>, action: A) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            action,
            variant: ButtonVariant::Secondary,
            size: ButtonSize::Medium,
            disabled: false,
        }
    }

    /// Sets visual emphasis.
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets the control size.
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Disables activation and removes the control from keyboard traversal.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl<A: Action + Clone> RenderOnce for Button<A> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::current(cx);
        let disabled = self.disabled;
        let click_action = self.action.clone();
        let key_action = self.action;
        let (background, foreground, border, hover) = match self.variant {
            ButtonVariant::Primary => (
                theme.accent,
                theme.accent_ink,
                theme.accent,
                theme.accent_hover,
            ),
            ButtonVariant::Secondary => (
                theme.surface_muted,
                theme.text,
                theme.border_strong,
                theme.surface_hover,
            ),
            ButtonVariant::Ghost => (
                gpui::transparent_black(),
                theme.text_muted,
                gpui::transparent_black(),
                theme.surface_hover,
            ),
            ButtonVariant::Danger => (
                theme.danger.opacity(0.12),
                theme.danger,
                theme.danger.opacity(0.35),
                theme.danger.opacity(0.20),
            ),
        };
        let (height, horizontal_padding, text_size) = match self.size {
            ButtonSize::Small => (px(32.0), px(12.0), px(12.0)),
            ButtonSize::Medium => (px(40.0), px(16.0), px(14.0)),
        };

        div()
            .id(self.id)
            .when(!disabled, |button| button.tab_index(0))
            .flex()
            .items_center()
            .justify_center()
            .h(height)
            .px(horizontal_padding)
            .rounded_lg()
            .border_1()
            .border_color(border)
            .focus(|style| style.border_2().border_color(theme.focus_ring))
            .bg(background)
            .text_size(text_size)
            .font_weight(gpui::FontWeight::SEMIBOLD)
            .text_color(foreground)
            .cursor_pointer()
            .when(disabled, |button| button.opacity(0.45).cursor_default())
            .when(!disabled, |button| {
                button
                    .hover(move |style| style.bg(hover))
                    .on_click(move |_, window, cx| {
                        window.dispatch_action(click_action.boxed_clone(), cx);
                    })
                    .on_key_down(move |event, window, cx| {
                        if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                            window.dispatch_action(key_action.boxed_clone(), cx);
                        }
                    })
            })
            .child(self.label)
    }
}
