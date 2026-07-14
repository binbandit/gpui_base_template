use super::PageContext;
use crate::app::actions::{CopyInstallCommand, RunSync};
use crate::app::assets::embedded_image;
use crate::app::components::{Badge, BadgeTone, Button, ButtonVariant, Card};
use crate::app::theme::Theme;
use gpui::{AnyElement, IntoElement, SharedString, div, img, prelude::*, px};

pub(super) fn render(context: &PageContext<'_>) -> AnyElement {
    let theme = context.theme;
    let narrow = context.narrow;

    div()
        .flex()
        .flex_col()
        .gap_5()
        .child(
            Card::new().elevated().child(
                div()
                    .flex()
                    .items_start()
                    .justify_between()
                    .gap_6()
                    .when(narrow, |hero| hero.flex_col())
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .min_w(px(0.0))
                            .gap_3()
                            .max_w(px(680.0))
                            .child(
                                div().flex().child(
                                    Badge::new("READY TO BUILD").tone(BadgeTone::Accent),
                                ),
                            )
                            .child(
                                div()
                                    .text_3xl()
                                    .line_height(px(42.0))
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .text_color(theme.text)
                                    .child("A polished foundation for native Rust apps."),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .line_height(px(22.0))
                                    .text_color(theme.text_muted)
                                    .child("The architectural edges are wired. Product-specific accessibility, signing, and operations stay in your hands."),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_wrap()
                                    .items_center()
                                    .gap_3()
                                    .pt_2()
                                    .child(
                                        Button::new(
                                            "copy-setup",
                                            "Copy setup command",
                                            CopyInstallCommand,
                                        )
                                        .variant(ButtonVariant::Primary),
                                    )
                                    .child(
                                        Button::new("run-sync", "Run async demo", RunSync)
                                            .variant(ButtonVariant::Secondary)
                                            .disabled(context.sync.is_running()),
                                    ),
                            ),
                    )
                    .when(!narrow, |hero| {
                        hero.child(
                            img(embedded_image("mark.png"))
                                .flex_none()
                                .size(px(92.0))
                                .rounded_xl(),
                        )
                    }),
            ),
        )
        .child(
            div()
                .flex()
                .gap_4()
                .when(narrow, |metrics| metrics.flex_col())
                .children([
                    metric(
                        "INTERACTIONS",
                        context.counter.to_string(),
                        "Entity-local state",
                        theme,
                    )
                    .into_any_element(),
                    metric("BACKGROUND", context.sync.label(), "GPUI executor", theme)
                        .into_any_element(),
                    metric(
                        "THEME",
                        format!("{:?}", context.settings.theme),
                        "Persisted preference",
                        theme,
                    )
                    .into_any_element(),
                ]),
        )
        .child(
            Card::new()
                .title("Architecture, without ceremony")
                .child(
                    div()
                        .flex()
                        .gap_4()
                        .when(narrow, |features| features.flex_col())
                        .children([
                            feature(
                                "01",
                                "Entities",
                                "State lives in GPUI's ownership model, with typed updates and notifications.",
                                theme,
                            )
                            .into_any_element(),
                            feature(
                                "02",
                                "Actions",
                                "Clicks and shortcuts dispatch the same logical commands.",
                                theme,
                            )
                            .into_any_element(),
                            feature(
                                "03",
                                "Services",
                                "Persistence, assets, logging, and async work have explicit boundaries.",
                                theme,
                            )
                            .into_any_element(),
                        ]),
                ),
        )
        .into_any_element()
}

fn metric(
    label: &'static str,
    value: impl Into<SharedString>,
    detail: impl Into<SharedString>,
    theme: &Theme,
) -> impl IntoElement {
    div().flex_1().min_w(px(0.0)).child(
        Card::new()
            .child(
                div()
                    .text_xs()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .text_color(theme.text_faint)
                    .child(label),
            )
            .child(
                div()
                    .text_2xl()
                    .font_weight(gpui::FontWeight::BOLD)
                    .text_color(theme.text)
                    .child(value.into()),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(theme.text_muted)
                    .child(detail.into()),
            ),
    )
}

fn feature(
    number: &'static str,
    title: &'static str,
    body: &'static str,
    theme: &Theme,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .flex_1()
        .min_w(px(0.0))
        .gap_2()
        .p_4()
        .rounded_lg()
        .bg(theme.surface_muted)
        .child(
            div()
                .text_xs()
                .font_weight(gpui::FontWeight::BOLD)
                .text_color(theme.accent)
                .child(number),
        )
        .child(
            div()
                .text_sm()
                .font_weight(gpui::FontWeight::SEMIBOLD)
                .text_color(theme.text)
                .child(title),
        )
        .child(
            div()
                .text_xs()
                .line_height(px(18.0))
                .text_color(theme.text_muted)
                .child(body),
        )
}
