# GPUI 0.2.2 cheat sheet

These fragments use the exact crates.io API pinned in `Cargo.toml`. For complete
programs, run the files in `examples/` if your setup mode retained them. For API
details, use the
[versioned documentation](https://docs.rs/gpui/0.2.2/gpui/).

## Window and root

Import `gpui::prelude::*` for element, rendering, and context extension traits.
`AppContext` provides `cx.new`. Start with `Application::new()`, then open a window
whose constructor returns an entity:

```rust
Application::new().run(|cx: &mut App| {
    cx.open_window(WindowOptions::default(), |_window, cx| {
        cx.new(|_| RootView { count: 0 })
    })
    .expect("open main window");
    cx.on_window_closed(|cx| {
        if cx.windows().is_empty() {
            cx.quit();
        }
    })
    .detach(); // Application-lifetime observer.
    cx.activate(true);
});
```

A view builds an element tree each time GPUI renders it:

```rust
struct RootView { count: u32 }

impl Render for RootView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(format!("Count: {}", self.count))
    }
}
```

## Entity updates

```rust
let model = cx.new(|_| Model::default());
model.update(cx, |model, cx| {
    model.value += 1;
    cx.notify();
});
let value = model.read(cx).value;
```

Read and update entities through a context. Call `cx.notify()` for changes that
should invalidate rendering or notify observers. Plain Rust values are sufficient
when state does not need an entity lifetime.

## Actions and keyboard focus

Define logical operations, then bind input to them:

```rust
gpui::actions!(my_app, [Save]);
cx.bind_keys([gpui::KeyBinding::new("secondary-s", Save, Some("MyApp"))]);
```

`secondary` is Command on macOS and Ctrl elsewhere. `cmd` is the platform modifier
itself; it does not become Ctrl on other systems. See
[the key syntax](https://docs.rs/gpui/0.2.2/gpui/struct.Keystroke.html#method.parse).

Create and retain a `FocusHandle` on the view; call `window.focus(&focus)` at
window creation. Include it in the rendered action path:

```rust
div()
    .track_focus(&self.focus)
    .key_context("MyApp")
    .on_action(cx.listener(Self::save))
    .child(
        div().id("save").tab_index(0).on_click(|_, window, cx| {
            window.dispatch_action(Box::new(Save), cx);
        }).child("Save"),
    )
```

An action handler has this shape:

```rust
fn save(&mut self, _: &Save, _: &mut Window, cx: &mut Context<Self>) {
    // Update state or begin an operation.
    cx.notify();
}
```

Make the clickable div focusable with `.tab_index(0)` or a retained focus handle,
and add a visible `.focus(...)` style. GPUI 0.2.2 invokes a focused div's
`.on_click(...)` on Enter/Space key release. Do not also dispatch the action in
an Enter/Space binding or key-down handler: that activates the control twice.
Bind Tab and Shift-Tab to root actions calling `window.focus_next()` and
`window.focus_prev()`. Keep `.tab_index(0)` on a button that can become disabled
while focused, use `.tab_stop(!disabled)` to exclude it from further traversal,
and omit its click handler while disabled. Removing its focus identity can also
remove the root key context from the focus path. Restore the persistent root
focus before navigation removes a page that contains the focused control.
See `examples/counter.rs` or the app's `Button` for the complete pattern. The
built-in keyboard click behavior is in the published
[`div` implementation](https://docs.rs/crate/gpui/0.2.2/source/src/elements/div.rs).

## View-owned async work

Store the task in `Option<gpui::Task<()>>`:

```rust
let timer = cx.background_executor().timer(std::time::Duration::from_secs(1));
self.task = Some(cx.spawn(async move |view, cx| {
    timer.await;
    let _ = view.update(cx, |view, cx| {
        view.status = "Done";
        view.task = None;
        cx.notify();
    });
}));
```

Dropping the task cancels the future. Use `self.task = None` to cancel deliberately;
closing the view also drops its task. Dropping a future does not undo IO already
performed or necessarily stop work already handed to another thread.

`view` is weak here: the update may fail after the entity has closed. Handle real
operation failures separately. For expensive work, use the background executor
and bring the result back through a foreground entity update. Never block a
render or input callback.

## Events and subscriptions

Use typed events when one entity needs to tell another about a change:

```rust
struct Saved { records: usize }
struct Model;
impl gpui::EventEmitter<Saved> for Model {}

// In the model's Context<Model>:
cx.emit(Saved { records: 12 });

// While constructing the subscriber:
let subscription = cx.subscribe(&model, |owner, _model, event: &Saved, cx| {
    owner.status = format!("Saved {}", event.records);
    cx.notify();
});
```

Store the `Subscription` on the subscriber. Dropping it unsubscribes; detaching
is a deliberate lifetime choice. Direct state updates are simpler when there is
only one owner.

## Stateless components and themes

```rust
#[derive(IntoElement)]
struct Badge { label: gpui::SharedString }

impl RenderOnce for Badge {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div().px_2().rounded_full().child(self.label)
    }
}
```

Use `RenderOnce` for stateless/controlled builders and `Entity<T> + Render` for
persistent state. Shared controls live in `src/app/components/`; keep page-only
composition in its page.

The app's `Theme` implements `gpui::Global`. `Theme::current(cx)` reads semantic
tokens such as `text`, `surface`, and `accent`; pages should use those tokens.
Mutable product state belongs in its owning entity, not in a new global.

## Layout

```rust
div()
    .flex()
    .flex_col()
    .gap_4()
    .p_6()
    .child(div().flex().justify_between().child("Left").child("Right"))
    .when(is_active, |el| el.bg(theme.surface))
    .children(items.into_iter().map(render_item))
```

Scrollable divs need an identity and a constrained layout:

```rust
div().id("content").flex_1().min_h_0().overflow_y_scroll()
```

Give separate pages distinct scroll-container IDs. Reusing one ID for unrelated
routes can carry a previous page’s scroll offset into the next page. The starter
keys the main scroll container by route.

## Assets and clipboard

`src/app/assets.rs` implements `AssetSource` and startup installs it with
`Application::new().with_assets(...)`. SVGs resolve through that asset source:

```rust
gpui::svg().path("mark.svg").size_8().text_color(theme.accent)
```

For embedded raster images, make the source explicit. Relative strings can be
parsed as URIs in this release:

```rust
gpui::img(gpui::ImageSource::Resource(
    gpui::Resource::Embedded("photo.png".into()),
))

cx.write_to_clipboard(gpui::ClipboardItem::new_string("copied".into()));
let text = cx.read_from_clipboard().and_then(|item| item.text());
```

## Persistence, text input, and tests

Keep serialization and IO outside views. The starter's `SettingsStore` loads
missing fields through `#[serde(default)]`, reports malformed files, and replaces
settings atomically. Change its configuration identity before users create data.

Correct text editing includes IME, marked ranges, UTF-16 conversion, grapheme
navigation, shaping, and selection. Adapt the exact
[upstream input example](https://docs.rs/crate/gpui/0.2.2/source/examples/input.rs),
not a key handler that appends characters.

Prefer ordinary `#[test]` for state and serialization. The dev dependency enables
`#[gpui::test]` for entity/action/executor simulation. `cargo check --all-targets`
compiles every example. `TestAppContext::simulate_keystrokes` sends key-down
events only; test focused click activation with a matching `KeyUpEvent` through
`VisualTestContext::simulate_event`, as the root regression test does.

Use `RUST_LOG=gpui_base_framework=debug,gpui=info cargo run`
for application logs, and `RUST_BACKTRACE=1 cargo run` for panic backtraces.
