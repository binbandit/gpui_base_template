use crate::app::actions::{NavigateComponents, NavigateOverview, NavigateSettings};
use crate::app::state::Route;
use crate::app::theme::Theme;
use gpui::{Action, AppContext, Context, IntoElement, Render, Window, div, prelude::*, px, svg};

pub(crate) fn render(active_route: Route, compact: bool, theme: &Theme) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .flex_none()
        .w(px(if compact { 64.0 } else { 216.0 }))
        .h_full()
        .border_r_1()
        .border_color(theme.border)
        .bg(theme.sidebar)
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .h(px(72.0))
                .px_5()
                .flex_none()
                .child(
                    svg()
                        .path("mark.svg")
                        .size(px(24.0))
                        .text_color(theme.accent),
                )
                .when(!compact, |brand| {
                    brand.child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(theme.text)
                            .child("GPUI Starter"),
                    )
                }),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .px_3()
                .gap_1()
                .when(!compact, |nav| {
                    nav.child(
                        div()
                            .px_2()
                            .pt_4()
                            .pb_3()
                            .text_xs()
                            .text_color(theme.text_faint)
                            .child("WORKSPACE"),
                    )
                })
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
        .when(!compact, |sidebar| {
            sidebar.child(
                div()
                    .m_4()
                    .p_3()
                    .border_t_1()
                    .border_color(theme.border)
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child("Your next native app."),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_faint)
                            .child("Rust + GPUI 0.2.2"),
                    ),
            )
        })
}

fn nav_item<A: Action + Clone>(
    active_route: Route,
    route: Route,
    action: A,
    compact: bool,
    theme: &Theme,
) -> impl IntoElement {
    let active = active_route == route;
    let icon = match route {
        Route::Overview => "icons/overview.svg",
        Route::Components => "icons/components.svg",
        Route::Settings => "icons/settings.svg",
    };

    div()
        .id(route.nav_id())
        .tab_index(0)
        .tooltip(move |_, cx| cx.new(|_| NavigationTooltip(route)).into())
        .flex()
        .items_center()
        .gap_3()
        .h(px(38.0))
        .px_2()
        .rounded_md()
        .border_1()
        .border_color(gpui::transparent_black())
        .focus(|style| style.border_color(theme.focus_ring))
        .bg(if active {
            theme.surface_hover
        } else {
            gpui::transparent_black()
        })
        .text_sm()
        .font_weight(if active {
            gpui::FontWeight::MEDIUM
        } else {
            gpui::FontWeight::NORMAL
        })
        .text_color(if active { theme.text } else { theme.text_muted })
        .cursor_pointer()
        .hover(|style| style.bg(theme.surface_hover))
        .on_click(move |_, window, cx| {
            window.dispatch_action(action.boxed_clone(), cx);
        })
        .child(
            svg()
                .path(icon)
                .size(px(16.0))
                .flex_none()
                .text_color(if active {
                    theme.accent
                } else {
                    theme.text_faint
                }),
        )
        .when(!compact, |item| {
            item.child(div().flex_1().child(route.title())).child(
                div()
                    .text_xs()
                    .text_color(theme.text_faint)
                    .child(route.shortcut()),
            )
        })
}

// GPUI tooltips are views; this entity only holds the label for its lifetime.
struct NavigationTooltip(Route);

impl Render for NavigationTooltip {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::current(cx);
        div()
            .px_3()
            .py_2()
            .rounded_md()
            .border_1()
            .border_color(theme.border_strong)
            .bg(theme.surface)
            .text_sm()
            .text_color(theme.text)
            .child(format!("{}  {}", self.0.title(), self.0.shortcut()))
    }
}
