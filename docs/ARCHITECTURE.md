# Architecture

## Intent

This template teaches GPUI rather than hiding it. It adds product scaffolding —
startup, a visual system, persistence, assets, logging, tests, packaging, and
examples — but does not add a competing component lifecycle or state runtime.

## Boundaries

```text
main.rs
  └─ app::run
       ├─ tracing initialization
       ├─ SettingsStore::load_or_default
       ├─ Application::new().with_assets(...)
       ├─ global Theme installation
       ├─ action/key binding registration
       └─ main window → Entity<RootView>

RootView (GPUI entity + coordinator)
  ├─ Route / SyncState        pure state
  ├─ AppSettings              serializable preferences
  ├─ SyncModel                typed event emitter
  ├─ retained Task            async lifetime
  ├─ pages/                   route-level composition
  ├─ shell/                   stable window chrome
  ├─ components/              reusable public controls
  └─ services/SettingsStore   filesystem boundary
```

## Why `src/app/` is self-contained

All implementation modules import siblings through `crate::app`. `src/lib.rs`
only exports that module. This makes `setup.sh --app-only` mechanical: remove the
library target, declare `mod app` in the binary, and keep every implementation
file unchanged. Do not import application internals through the package name
from inside `src/app/`.

## State placement

Use the narrowest owner that matches the lifetime:

- A local render-only value stays local.
- Stateful UI or domain data shared by views becomes `Entity<T>`.
- A view calls `cx.notify()` after a change that should redraw or notify
  observers.
- Typed facts crossing entity boundaries use `EventEmitter<E>`, `cx.emit`, and
  `cx.subscribe`.
- Truly process-wide values use `Global`. This starter uses a global only for
  the theme because overlays and additional windows must agree.
- Durable values are represented by `AppSettings` and written through
  `SettingsStore`; GPUI entities do not perform ad-hoc filesystem IO.

## Subscription and task lifetimes

Dropping a GPUI `Subscription` unsubscribes. `RootView` stores its sync
subscription in `_sync_subscription`, documenting that it must live exactly as
long as the view.

Dropping a GPUI `Task` cancels it. The async demo stores the task while active and
clears it when the typed completion event arrives. Use `.detach()` only when work
must outlive its initiating view and cannot update released UI.

## Rendering and feature placement

`RootView::render` selects a route module and assembles the window shell each
frame; it does not contain screen-sized presentation helpers. The boundaries are:

- `pages/`: one module per route, plus page-private helper views;
- `components/`: reusable controlled/stateless UI, publicly exported through
  `app::components`;
- `shell/`: navigation, header, notices, and other window-wide chrome;
- `services/`: settings and future filesystem/network/OS integrations;
- `root.rs`: state ownership, actions, subscriptions, and task lifetimes.

Reusable stateless controls implement `RenderOnce + IntoElement`; they do not
become entities. Persistent controls (text editing, virtualized data,
subscriptions) should be entities that implement `Render` or custom `Element`s
when lower-level layout/paint access is required. See
`src/app/components/README.md` before adding a control.

## Error policy

Failure to create the native window is fatal. An unavailable configuration
location, corrupt preferences, failed saves, and background operation failures
are recoverable and become visible notices plus tracing events. Settings writes
flush a temporary file and use atomic POSIX replacement; Windows preserves and
rolls back the previous file around its non-replacing rename. This keeps startup
resilient without swallowing information developers need.

## Accessibility boundary

The pinned GPUI release supports focus traversal and focused styling but does not
expose semantic role/state or live-announcement APIs for custom elements. The
starter supplies visible focus, keyboard activation, contrast tests, disabled-tab
behavior, and narrow stacking; it cannot make its custom controls screen-reader
complete on 0.2.2. `docs/ACCESSIBILITY.md` is the release checklist and must stay
accurate when controls or the GPUI version change.

## Testing layers

1. Pure state and serialization tests are fast and platform-neutral.
2. GPUI tests (`#[gpui::test]`) are appropriate for entity/action behavior once
   a feature needs simulation; enablement is already available through the
   dev-dependency.
3. `cargo check --all-targets` compiles every runnable example.
4. CI checks macOS, Linux, and Windows because windowing backends differ.
5. Before release, launch and exercise a packaged application on every platform;
   compilation is not a replacement for native smoke testing.
