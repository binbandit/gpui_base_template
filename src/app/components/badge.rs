use crate::app::theme::Theme;
use gpui::{App, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px};

/// Semantic color treatment for a [`Badge`].
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum BadgeTone {
    Neutral,
    Accent,
    Success,
    Warning,
}

/// Compact, non-interactive status or category label.
#[derive(IntoElement)]
pub struct Badge {
    label: SharedString,
    tone: BadgeTone,
}

impl Badge {
    /// Creates a neutral badge.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            tone: BadgeTone::Neutral,
        }
    }

    /// Applies a semantic tone.
    pub fn tone(mut self, tone: BadgeTone) -> Self {
        self.tone = tone;
        self
    }
}

impl RenderOnce for Badge {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::current(cx);
        let color = match self.tone {
            BadgeTone::Neutral => theme.text_muted,
            BadgeTone::Accent => theme.accent,
            BadgeTone::Success => theme.success,
            BadgeTone::Warning => theme.warning,
        };

        div()
            .flex()
            .items_center()
            .h(px(24.0))
            .px_2()
            .rounded_full()
            .border_1()
            .border_color(color.opacity(0.42))
            .bg(color.opacity(0.10))
            .text_xs()
            .font_weight(gpui::FontWeight::MEDIUM)
            .text_color(color)
            .child(self.label)
    }
}
