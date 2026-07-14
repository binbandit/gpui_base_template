use crate::app::theme::Theme;
use gpui::{AnyElement, App, IntoElement, RenderOnce, SharedString, Window, div, prelude::*};

/// Composable surface container for related content.
#[derive(IntoElement)]
pub struct Card {
    title: Option<SharedString>,
    children: Vec<AnyElement>,
    elevated: bool,
}

impl Card {
    /// Creates an untitled, flat card.
    pub fn new() -> Self {
        Self {
            title: None,
            children: Vec::new(),
            elevated: false,
        }
    }

    /// Adds a heading above the card contents.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Uses the stronger border and elevation treatment.
    pub fn elevated(mut self) -> Self {
        self.elevated = true;
        self
    }

    /// Appends child content in vertical order.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for Card {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::current(cx);
        let title = self.title;

        div()
            .flex()
            .flex_col()
            .gap_4()
            .p_5()
            .rounded_xl()
            .border_1()
            .border_color(if self.elevated {
                theme.border_strong
            } else {
                theme.border
            })
            .bg(theme.surface)
            .when(self.elevated, |card| card.shadow_lg())
            .children(title.map(|title| {
                div()
                    .text_sm()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .text_color(theme.text)
                    .child(title)
            }))
            .children(self.children)
    }
}
