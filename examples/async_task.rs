//! Start and cancel a timer without blocking the UI. The view owns the task;
//! dropping that task cancels its future, including when the window closes.

use gpui::{
    App, AppContext, Application, Bounds, Context, FocusHandle, KeyBinding, Render, Task, Window,
    WindowBounds, WindowOptions, actions, div, prelude::*, px, rgb, size,
};
use std::time::Duration;

actions!(async_demo, [ToggleTask, CancelTask]);

struct AsyncDemo {
    status: &'static str,
    task: Option<Task<()>>,
    focus: FocusHandle,
}

impl AsyncDemo {
    fn toggle(&mut self, _: &ToggleTask, window: &mut Window, cx: &mut Context<Self>) {
        if self.task.is_some() {
            self.cancel(&CancelTask, window, cx);
            return;
        }
        self.status = "Waiting for a two-second timer…";
        let timer = cx.background_executor().timer(Duration::from_secs(2));
        self.task = Some(cx.spawn(async move |view, cx| {
            timer.await;
            // The view may have closed while the timer was running.
            let _ = view.update(cx, |view, cx| {
                view.status = "Timer finished. Ready to run again.";
                view.task = None;
                cx.notify();
            });
        }));
        cx.notify();
    }

    fn cancel(&mut self, _: &CancelTask, _: &mut Window, cx: &mut Context<Self>) {
        if self.task.take().is_some() {
            self.status = "Cancelled. Ready to run again.";
            cx.notify();
        }
    }
}

impl Render for AsyncDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .key_context("AsyncDemo")
            .on_action(cx.listener(Self::toggle))
            .on_action(cx.listener(Self::cancel))
            .flex()
            .flex_col()
            .size_full()
            .gap_5()
            .items_center()
            .justify_center()
            .bg(rgb(0x16181D))
            .text_color(rgb(0xEDF0F5))
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0xA1A9B8))
                    .child("ASYNC TASK"),
            )
            .child(div().text_xl().child(self.status))
            .child(
                div()
                    .id("toggle-task")
                    .track_focus(&self.focus)
                    .px_5()
                    .py_3()
                    .rounded_md()
                    .border_2()
                    .border_color(gpui::transparent_black())
                    .focus(|style| style.border_color(rgb(0xEDF0F5)))
                    .bg(rgb(0x90B4FF))
                    .hover(|style| style.bg(rgb(0xADC8FF)))
                    .text_color(rgb(0x152444))
                    .cursor_pointer()
                    .on_click(|_, window, cx| {
                        window.dispatch_action(Box::new(ToggleTask), cx);
                    })
                    .child(if self.task.is_some() {
                        "Cancel timer"
                    } else {
                        "Start timer"
                    }),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0xA1A9B8))
                    .child("Enter / Space to activate · Escape to cancel"),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        // Enter/Space activation comes from the focused div's on_click handler.
        cx.bind_keys([KeyBinding::new("escape", CancelTask, Some("AsyncDemo"))]);
        let bounds = Bounds::centered(None, size(px(620.0), px(380.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                cx.new(|cx| {
                    let focus = cx.focus_handle();
                    window.focus(&focus);
                    AsyncDemo {
                        status: "Ready when you are.",
                        task: None,
                        focus,
                    }
                })
            },
        )
        .expect("open async example window");
        cx.on_window_closed(|cx| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();
        cx.activate(true);
    });
}
