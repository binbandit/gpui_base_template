use crate::app::theme::Theme;
use gpui::{
    Action, App, ElementId, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px,
};

/// Labeled boolean control that dispatches a typed action when activated.
///
/// The checked value is controlled by the caller, so persistent state remains
/// in the owning GPUI entity rather than inside this stateless component.
#[derive(IntoElement)]
pub struct Toggle<A: Action + Clone> {
    id: ElementId,
    label: SharedString,
    description: SharedString,
    checked: bool,
    action: A,
}

impl<A: Action + Clone> Toggle<A> {
    /// Creates a controlled toggle with a label and supporting description.
    pub fn new(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        description: impl Into<SharedString>,
        checked: bool,
        action: A,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: description.into(),
            checked,
            action,
        }
    }
}

impl<A: Action + Clone> RenderOnce for Toggle<A> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::current(cx);
        let checked = self.checked;
        let click_action = self.action.clone();
        let key_action = self.action;

        div()
            .id(self.id)
            .tab_index(0)
            .flex()
            .items_center()
            .justify_between()
            .gap_4()
            .w_full()
            .px_2()
            .py_3()
            .rounded_lg()
            .border_1()
            .border_color(gpui::transparent_black())
            .focus(|style| style.border_2().border_color(theme.focus_ring))
            .cursor_pointer()
            .on_click(move |_, window, cx| {
                window.dispatch_action(click_action.boxed_clone(), cx);
            })
            .on_key_down(move |event, window, cx| {
                if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                    window.dispatch_action(key_action.boxed_clone(), cx);
                }
            })
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::MEDIUM)
                            .text_color(theme.text)
                            .child(self.label),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child(self.description),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .w(px(44.0))
                    .h(px(24.0))
                    .p(px(3.0))
                    .rounded_full()
                    .border_1()
                    .border_color(if checked {
                        theme.accent
                    } else {
                        theme.border_strong
                    })
                    .bg(if checked {
                        theme.accent
                    } else {
                        theme.surface_muted
                    })
                    .child(
                        div()
                            .size(px(16.0))
                            .rounded_full()
                            .bg(if checked {
                                theme.accent_ink
                            } else {
                                theme.text_faint
                            })
                            .when(checked, |thumb| thumb.ml(px(18.0))),
                    ),
            )
    }
}
