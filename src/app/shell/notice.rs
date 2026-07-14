use crate::app::actions::DismissNotice;
use crate::app::components::{Button, ButtonSize, ButtonVariant};
use crate::app::theme::Theme;
use gpui::{AnyElement, IntoElement, SharedString, div, prelude::*};

#[derive(Clone, Copy)]
pub(super) enum NoticeTone {
    Info,
    Success,
    Warning,
}

/// User-visible feedback owned by the root and rendered by the shell.
pub(crate) struct Notice {
    message: SharedString,
    tone: NoticeTone,
}

impl Notice {
    pub(crate) fn info(message: impl Into<SharedString>) -> Self {
        Self {
            message: message.into(),
            tone: NoticeTone::Info,
        }
    }

    pub(crate) fn success(message: impl Into<SharedString>) -> Self {
        Self {
            message: message.into(),
            tone: NoticeTone::Success,
        }
    }

    pub(crate) fn warning(message: impl Into<SharedString>) -> Self {
        Self {
            message: message.into(),
            tone: NoticeTone::Warning,
        }
    }
}

pub(crate) fn render(notice: Option<&Notice>, theme: &Theme) -> Option<AnyElement> {
    notice.map(|notice| {
        let color = match notice.tone {
            NoticeTone::Info => theme.accent,
            NoticeTone::Success => theme.success,
            NoticeTone::Warning => theme.warning,
        };

        div()
            .flex()
            .items_center()
            .justify_between()
            .gap_4()
            .mx_6()
            .mb_4()
            .px_4()
            .py_3()
            .rounded_lg()
            .border_1()
            .border_color(color.opacity(0.42))
            .bg(color.opacity(0.10))
            .text_sm()
            .text_color(theme.text)
            .child(notice.message.clone())
            .child(
                Button::new("dismiss-notice", "Dismiss", DismissNotice)
                    .variant(ButtonVariant::Ghost)
                    .size(ButtonSize::Small),
            )
            .into_any_element()
    })
}
