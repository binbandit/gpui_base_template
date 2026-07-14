use crate::app::actions::{NavigateComponents, NavigateOverview, NavigateSettings};
use crate::app::assets::embedded_image;
use crate::app::components::{Badge, BadgeTone};
use crate::app::state::Route;
use crate::app::theme::Theme;
use gpui::{Action, IntoElement, div, img, prelude::*, px};

pub(crate) fn render(active_route: Route, compact: bool, theme: &Theme) -> impl IntoElement {
    let width = if compact { px(76.0) } else { px(244.0) };

    div()
        .flex()
        .flex_col()
        .flex_none()
        .w(width)
        .h_full()
        .p_4()
        .gap_5()
        .border_r_1()
        .border_color(theme.border)
        .bg(theme.sidebar)
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .h(px(48.0))
                .child(
                    img(embedded_image("mark.png"))
                        .flex_none()
                        .size(px(34.0))
                        .rounded_lg(),
                )
                .when(!compact, |brand| {
                    brand.child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .text_color(theme.text)
                                    .child("GPUI BASE"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_faint)
                                    .child("SHIP NATIVE"),
                            ),
                    )
                }),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .child(nav_item(
                    active_route,
                    Route::Overview,
                    NavigateOverview,
                    compact,
                    theme,
                ))
                .child(nav_item(
                    active_route,
                    Route::Components,
                    NavigateComponents,
                    compact,
                    theme,
                ))
                .child(nav_item(
                    active_route,
                    Route::Settings,
                    NavigateSettings,
                    compact,
                    theme,
                )),
        )
        .child(div().flex_1())
        .child(div().flex().flex_col().gap_3().when(!compact, |footer| {
            footer
                .child(Badge::new("GPUI 0.2.2").tone(BadgeTone::Success))
                .child(
                    div()
                        .text_xs()
                        .line_height(px(18.0))
                        .text_color(theme.text_faint)
                        .child(
                            "Pinned APIs. Reproducible builds. Your app, not another framework.",
                        ),
                )
        }))
}

fn nav_item<A: Action + Clone>(
    active_route: Route,
    route: Route,
    action: A,
    compact: bool,
    theme: &Theme,
) -> impl IntoElement {
    let active = active_route == route;
    let click_action = action.clone();
    let key_action = action;

    div()
        .id(route.nav_id())
        .tab_index(0)
        .flex()
        .items_center()
        .justify_between()
        .h(px(42.0))
        .px_3()
        .rounded_lg()
        .border_1()
        .border_color(if active {
            theme.accent.opacity(0.42)
        } else {
            gpui::transparent_black()
        })
        .focus(|style| style.border_2().border_color(theme.focus_ring))
        .bg(if active {
            theme.accent.opacity(0.12)
        } else {
            gpui::transparent_black()
        })
        .text_sm()
        .font_weight(if active {
            gpui::FontWeight::SEMIBOLD
        } else {
            gpui::FontWeight::NORMAL
        })
        .text_color(if active {
            theme.accent
        } else {
            theme.text_muted
        })
        .cursor_pointer()
        .hover(|style| style.bg(theme.surface_hover))
        .on_click(move |_, window, cx| {
            window.dispatch_action(click_action.boxed_clone(), cx);
        })
        .on_key_down(move |event, window, cx| {
            if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                window.dispatch_action(key_action.boxed_clone(), cx);
            }
        })
        .child(if compact {
            route.compact_label()
        } else {
            route.title()
        })
        .when(!compact, |item| {
            item.child(
                div()
                    .text_xs()
                    .text_color(theme.text_faint)
                    .child(route.shortcut()),
            )
        })
}
