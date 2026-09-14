//! A small stateful view with actions shared by pointer and keyboard input.

use gpui::{
    App, AppContext, Application, Bounds, Context, FocusHandle, KeyBinding, Render, Window,
    WindowBounds, WindowOptions, actions, div, prelude::*, px, rgb, size,
};

actions!(counter, [Increment, Decrement, FocusNext, FocusPrevious]);

struct Counter {
    value: i32,
    focus: FocusHandle,
}

impl Counter {
    fn increment(&mut self, _: &Increment, _: &mut Window, cx: &mut Context<Self>) {
        self.value = self.value.saturating_add(1);
        cx.notify();
    }

    fn decrement(&mut self, _: &Decrement, _: &mut Window, cx: &mut Context<Self>) {
        self.value = self.value.saturating_sub(1);
        cx.notify();
    }
}

impl Render for Counter {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus)
            .key_context("Counter")
            .on_action(cx.listener(Self::increment))
            .on_action(cx.listener(Self::decrement))
            .on_action(|_: &FocusNext, window, _| window.focus_next())
            .on_action(|_: &FocusPrevious, window, _| window.focus_prev())
            .flex()
            .flex_col()
            .size_full()
            .gap_5()
            .items_center()
            .justify_center()
            .bg(rgb(0x16181D))
            .text_color(rgb(0xEDF0F5))
            .child(div().text_sm().text_color(rgb(0xA1A9B8)).child("COUNTER"))
            .child(div().text_size(px(64.0)).child(self.value.to_string()))
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(action_button("minus", "−", Decrement))
                    .child(action_button("plus", "+", Increment)),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0xA1A9B8))
                    .child("↑ / ↓ to change · Tab to focus · Enter to activate"),
            )
    }
}

fn action_button<A: gpui::Action>(
    id: &'static str,
    label: &'static str,
    action: A,
) -> impl IntoElement {
    div()
        .id(id)
        .tab_index(0)
        .flex()
        .size(px(48.0))
        .items_center()
        .justify_center()
        .rounded_md()
        .border_2()
        .border_color(gpui::transparent_black())
        .focus(|style| style.border_color(rgb(0xEDF0F5)))
        .bg(rgb(0x90B4FF))
        .hover(|style| style.bg(rgb(0xADC8FF)))
        .text_color(rgb(0x152444))
        .text_xl()
        .cursor_pointer()
        // GPUI also invokes on_click for Enter/Space when this div is focused.
        .on_click(move |_, window, cx| {
            window.dispatch_action(action.boxed_clone(), cx);
        })
        .child(label)
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.bind_keys([
            KeyBinding::new("up", Increment, Some("Counter")),
            KeyBinding::new("down", Decrement, Some("Counter")),
            KeyBinding::new("tab", FocusNext, Some("Counter")),
            KeyBinding::new("shift-tab", FocusPrevious, Some("Counter")),
        ]);
        let bounds = Bounds::centered(None, size(px(540.0), px(380.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                cx.new(|cx| {
                    let focus = cx.focus_handle();
                    window.focus(&focus);
                    Counter { value: 0, focus }
                })
            },
        )
        .expect("open counter window");
        // This observer belongs to the application, not an individual view.
        cx.on_window_closed(|cx| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();
        cx.activate(true);
    });
}
