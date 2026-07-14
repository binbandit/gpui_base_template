//! A cancellable GPUI task held by the view and completed through a weak
//! entity handle. The UI thread never sleeps.

use gpui::{
    App, AppContext, Application, Bounds, Context, Render, Task, Window, WindowBounds,
    WindowOptions, div, prelude::*, px, rgb, size,
};
use std::time::Duration;

struct AsyncDemo {
    status: &'static str,
    task: Option<Task<()>>,
}

impl AsyncDemo {
    fn start(&mut self, _: &gpui::ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        if self.task.is_some() {
            return;
        }
        self.status = "Working…";
        let timer = cx.background_executor().timer(Duration::from_secs(1));
        self.task = Some(cx.spawn(async move |view, cx| {
            timer.await;
            let _ = view.update(cx, |view, cx| {
                view.status = "Finished — 384 records loaded";
                view.task = None;
                cx.notify();
            });
        }));
        cx.notify();
    }
}

impl Render for AsyncDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let running = self.task.is_some();
        div()
            .flex()
            .flex_col()
            .size_full()
            .gap_5()
            .items_center()
            .justify_center()
            .bg(rgb(0x111820))
            .text_color(rgb(0xF5F1EA))
            .child(div().text_xl().child(self.status))
            .child(
                div()
                    .id("start")
                    .px_4()
                    .py_2()
                    .rounded_lg()
                    .bg(rgb(0xF2A365))
                    .text_color(rgb(0x17202B))
                    .opacity(if running { 0.5 } else { 1.0 })
                    .cursor_pointer()
                    .on_click(cx.listener(Self::start))
                    .child(if running { "Running" } else { "Start task" }),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(620.0), px(380.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_window, cx| {
                cx.new(|_| AsyncDemo {
                    status: "Ready",
                    task: None,
                })
            },
        )
        .expect("open window");
        cx.on_window_closed(|cx| cx.quit()).detach();
        cx.activate(true);
    });
}
