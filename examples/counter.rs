//! Entity state, actions, mouse input, and keyboard bindings.

use gpui::{
    App, AppContext, Application, Bounds, Context, KeyBinding, MouseButton, Render, Window,
    WindowBounds, WindowOptions, actions, div, prelude::*, px, rgb, size,
};

actions!(counter, [Increment, Decrement]);

struct Counter {
    value: i32,
}

impl Counter {
    fn increment(&mut self, _: &Increment, _: &mut Window, cx: &mut Context<Self>) {
        self.value += 1;
        cx.notify();
    }

    fn decrement(&mut self, _: &Decrement, _: &mut Window, cx: &mut Context<Self>) {
        self.value -= 1;
        cx.notify();
    }
}

impl Render for Counter {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .key_context("Counter")
            .on_action(cx.listener(Self::increment))
            .on_action(cx.listener(Self::decrement))
            .flex()
            .flex_col()
            .size_full()
            .gap_4()
            .items_center()
            .justify_center()
            .bg(rgb(0x111820))
            .text_color(rgb(0xF5F1EA))
            .child(div().text_3xl().child(self.value.to_string()))
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(action_button("minus", "−", Decrement))
                    .child(action_button("plus", "+", Increment)),
            )
            .child("Use ↑ / ↓ or click")
    }
}

fn action_button<A: gpui::Action + Clone>(
    id: &'static str,
    label: &'static str,
    action: A,
) -> impl IntoElement {
    div()
        .id(id)
        .flex()
        .size(px(44.0))
        .items_center()
        .justify_center()
        .rounded_lg()
        .bg(rgb(0xF2A365))
        .text_color(rgb(0x17202B))
        .text_xl()
        .cursor_pointer()
        .on_mouse_up(MouseButton::Left, move |_, window, cx| {
            window.dispatch_action(action.boxed_clone(), cx);
        })
        .child(label)
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.bind_keys([
            KeyBinding::new("up", Increment, Some("Counter")),
            KeyBinding::new("down", Decrement, Some("Counter")),
        ]);
        let bounds = Bounds::centered(None, size(px(520.0), px(360.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_window, cx| cx.new(|_| Counter { value: 0 }),
        )
        .expect("open window");
        cx.on_window_closed(|cx| cx.quit()).detach();
        cx.activate(true);
    });
}
