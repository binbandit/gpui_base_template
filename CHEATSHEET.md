# GPUI 0.2.2 Cheat Sheet

Copy-ready patterns for the exact crates.io API pinned by this template. If a
current Zed example uses `gpui_platform::application()`, it belongs to a newer
unreleased API line; do not mix it into this project.

## Imports

```rust
use gpui::{
    App, AppContext, Application, Bounds, Context, Entity, IntoElement,
    Render, Window, WindowBounds, WindowOptions, div, prelude::*, px, size,
};
```

`AppContext` must be in scope for `cx.new(...)`.

## Application and window

```rust
Application::new().run(|cx: &mut App| {
    let bounds = Bounds::centered(None, size(px(900.0), px(640.0)), cx);
    cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            ..Default::default()
        },
        |_window, cx| cx.new(|cx| RootView::new(cx)),
    )
    .expect("open main window");

    cx.on_window_closed(|cx| {
        if cx.windows().is_empty() {
            cx.quit();
        }
    })
    .detach();
    cx.activate(true);
});
```

## Root view

```rust
struct RootView {
    count: u32,
}

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(format!("Count: {}", self.count))
    }
}
```

## Update an entity

```rust
let model: Entity<Model> = cx.new(|_| Model::default());

model.update(cx, |model, cx| {
    model.value += 1;
    cx.notify();
});

let current = model.read(cx).value;
```

Entity handles do not dereference by themselves; access requires a context.

## Actions and key bindings

```rust
use gpui::{KeyBinding, actions};

actions!(my_app, [Save, Refresh]);

cx.bind_keys([
    KeyBinding::new("cmd-s", Save, Some("MyApp")),
    KeyBinding::new("cmd-r", Refresh, Some("MyApp")),
]);
```

Handle actions on an element:

```rust
impl MyView {
    fn save(&mut self, _: &Save, _: &mut Window, cx: &mut Context<Self>) {
        // mutate state
        cx.notify();
    }
}

// In render:
div()
    .key_context("MyApp")
    .on_action(cx.listener(Self::save))
```

Dispatch the same action from a click:

```rust
div()
    .id("save")
    .on_click(|_, window, cx| {
        window.dispatch_action(Box::new(Save), cx);
    })
    .child("Save")
```

## Focus

```rust
struct MyView {
    focus_handle: gpui::FocusHandle,
}

impl gpui::Focusable for MyView {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

// During construction:
let focus_handle = cx.focus_handle();
window.focus(&focus_handle);

// In render:
div()
    .track_focus(&self.focus_handle(cx))
    .key_context("MyView")
```

For a custom focusable control, assign `.id(...)` and `.tab_index(0)`. GPUI 0.2.2
does not install Tab traversal for you: bind `tab`/`shift-tab` actions at the
root and handle them with `window.focus_next()`/`window.focus_prev()` (see
`src/app/actions.rs` and `src/app/root.rs`). Add a focused style such as
`.focus(|style| style.border_2().border_color(theme.focus_ring))`, and omit
`.tab_index(0)` when a control is disabled. A generic `div` is not automatically
a keyboard button; handle Enter/Space by dispatching the same logical action as
its click path.

## Typed entity events

```rust
struct Saved {
    records: usize,
}

struct Model;
impl gpui::EventEmitter<Saved> for Model {}

// Emitter:
cx.emit(Saved { records: 12 });

// Subscriber during owner construction:
let subscription = cx.subscribe(&model, |owner, _model, event: &Saved, cx| {
    owner.status = format!("Saved {}", event.records);
    cx.notify();
});
```

Store `subscription` on the owner. Dropping it unsubscribes. Use `.detach()` only
for a deliberate longer lifetime.

## Async task owned by a view

```rust
use std::time::Duration;

let timer = cx.background_executor().timer(Duration::from_secs(1));
self.task = Some(cx.spawn(async move |view, cx| {
    timer.await;
    let _ = view.update(cx, |view, cx| {
        view.status = "Done";
        view.task = None;
        cx.notify();
    });
}));
```

Store `Task<()>`; dropping it cancels. Calls through an async/weak context are
fallible because the app or entity may have closed while awaiting.

For CPU/blocking work, spawn a `Send` future on `cx.background_executor()` and
return the result to a foreground entity update. Do not block `render` or an input
listener.

## Global state

```rust
use gpui::{Global, ReadGlobal, UpdateGlobal};

#[derive(Clone)]
struct Theme { /* tokens */ }
impl Global for Theme {}

Theme::set_global(cx, theme);
let theme = Theme::global(cx).clone();
```

Globals are for process-wide services/tokens. Prefer entities for mutable product
state and plain values for render-local state.

## `RenderOnce` component

```rust
#[derive(IntoElement)]
struct Badge {
    label: gpui::SharedString,
}

impl RenderOnce for Badge {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().px_2().rounded_full().child(self.label)
    }
}
```

Use `RenderOnce` for owned, stateless builders. Shared controls belong under
`src/app/components/` and should be exported from its `mod.rs`; keep one-off
composition in its `pages/` module. Use `Entity<T> + Render` for persistent state,
subscriptions, focus, or asynchronous work. The component checklist lives in
`src/app/components/README.md`.

## Layout and style

```rust
div()
    .flex()
    .flex_col()
    .size_full()
    .gap_4()
    .p_6()
    .bg(gpui::rgb(0x111820))
    .text_color(gpui::rgb(0xF5F1EA))
    .child(
        div()
            .flex()
            .items_center()
            .justify_between()
            .child("Left")
            .child("Right"),
    )
```

Scrollable divs need an identity in GPUI 0.2.2:

```rust
div()
    .id("scroll-region")
    .flex_1()
    .overflow_scroll()
```

## Conditional composition

```rust
div()
    .when(is_active, |el| el.bg(theme.accent))
    .when_some(subtitle, |el, text| el.child(text))
    .children(items.into_iter().map(render_item))
```

## Clipboard

```rust
cx.write_to_clipboard(gpui::ClipboardItem::new_string("copied".into()));

if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
    // use text
}
```

## Embedded assets

```rust
#[derive(rust_embed::RustEmbed)]
#[folder = "assets/"]
struct Assets;

// Implement gpui::AssetSource, then:
Application::new().with_assets(Assets).run(|cx| { /* ... */ });

// In a view, be explicit about embedded raster resources in GPUI 0.2.2:
use gpui::{ImageSource, Resource};

gpui::img(ImageSource::Resource(Resource::Embedded("mark.png".into())))
```

GPUI 0.2.2's URI parser accepts some relative strings, so `img("mark.png")`
can be treated as a network URI. `src/app/assets.rs` provides the complete
implementation and an `embedded_image` helper that avoids this ambiguity.

## Settings boundary

Keep serialization outside views:

```rust
#[derive(Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
struct Settings {
    compact: bool,
}
```

Use `directories::ProjectDirs`, contextual errors, safe defaults, and a visible
warning for malformed files. Decide persistent identifiers before release.

## GPUI tests

The dev-dependency enables `test-support`:

```rust
#[gpui::test]
fn creates_model(cx: &mut gpui::TestAppContext) {
    let model = cx.update(|cx| cx.new(|_| Model::default()));
    assert_eq!(model.read_with(cx, |model, _| model.value), 0);
}
```

Prefer plain `#[test]` for pure state/serialization. Add GPUI simulation only when
the behavior needs entities, windows, key dispatch, or the deterministic executor.

## Text input

Do not implement desktop text input by appending `KeyDownEvent` characters. Correct
input requires IME/marked ranges, UTF-16 conversion, grapheme navigation, shaped
text hit testing, selection, and clipboard behavior. Start from the exact 0.2.2
source:

<https://docs.rs/crate/gpui/0.2.2/source/examples/input.rs>

## Debugging

```bash
RUST_LOG=gpui_base_framework=debug,gpui=info cargo run
RUST_BACKTRACE=1 cargo run
```

On macOS this template uses `runtime_shaders`. If you disable it, install Apple's
optional build-time compiler with `xcodebuild -downloadComponent MetalToolchain`.
