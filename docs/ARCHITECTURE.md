# Architecture

The starter uses GPUI's entity, rendering, action, and executor model directly.
The code belongs to the generated application; there is no wrapper framework to
learn or preserve.

## Ownership

`app::run` initializes logging, loads preferences, installs the global theme,
binds shortcuts, and opens a window containing `Entity<RootView>`.

`RootView` owns route state, preferences, notices, and the demo's `Option<Task<()>>`.
Actions mutate that state and call `cx.notify()` when the view should redraw.
Route and demo state types live in `state.rs`; keep future domain transitions
there when they can be tested without a window. The timer is a demonstration; it performs no network work.

| Module | Put here |
| --- | --- |
| `root.rs` | State ownership, action handling, and async lifetime |
| `pages/` | Route composition and page-only helpers |
| `shell/` | Navigation, header, and global notices |
| `components/` | Shared stateless/controlled `RenderOnce` controls |
| `services/` | Filesystem, network, and OS boundaries |
| `theme.rs` | Semantic theme tokens and the process-wide theme global |

Use an `Entity<T>` when data or behavior needs a persistent owner. Plain values
are enough for render-local data and pure domain state. Stateful editors may need
`Render` or a custom `Element`; avoid forcing every control into a common trait.

## Task and subscription lifetimes

A dropped `Task` cancels its future. The root retains its task while work is
running, then clears it after completion. A weak entity update after `await`
returns an error if the entity has been released; that is a normal lifecycle
outcome, not a reason to panic. Expensive/blocking work belongs on the background
executor, with results returned through a foreground entity update.

A dropped `Subscription` unsubscribes. Store subscriptions on their owner when
adding entity observers or typed events. The starter's last-window-close observer
is deliberately detached because it belongs to the application lifetime. Add
`EventEmitter` only for actual communication between entities; a single view can
update its own state directly.

## Input and focus

Pointer and keyboard input dispatch typed actions to the same handlers. A root
focus handle establishes the action context. Interactive controls have stable
IDs, tab stops, and visible focus. Root actions call `focus_next`/`focus_prev`
for Tab traversal. GPUI invokes a focused div's `on_click` handler on Enter/Space
key release, so controls do not add a second keyboard activation handler.
Disabled buttons retain their focus identity with `.tab_index(0)` but use
`.tab_stop(false)` and omit click handlers. This keeps shortcuts reachable if a
focused button becomes disabled. Navigation restores the persistent root focus
before the old page disappears. Each route has its own scroll-container ID so
scrolling one page does not set the next page’s offset. Notices survive navigation
until dismissed or replaced by a later operation.

Use `secondary-` bindings for Command on macOS and Ctrl elsewhere. See
[the GPUI key syntax](https://docs.rs/gpui/0.2.2/gpui/struct.Keystroke.html#method.parse).
Focus support does not imply semantic accessibility; see
[the API boundary](ACCESSIBILITY.md).

## Persistence and errors

`SettingsStore` owns the configuration path and serialization. Missing files use
defaults. Invalid or unreadable files use defaults with a visible warning. Save
failures preserve the session change and report an error, rather than crashing.
Writes use a uniquely named `tempfile::NamedTempFile` in the destination
directory, flush its contents, and atomically replace the settings file with
`persist`. Corrupt input remains untouched until the user changes a preference.
Older settings can omit new fields, and removed fields are ignored. Only theme
and compact navigation are persisted; the counter, route, and job status reset
on launch. Configure the application identity before release and plan a
migration if it changes later.

Failure to create the native window is fatal. Ordinary operation, preference,
and filesystem failures are recoverable: attach useful tracing context and show
a user-facing notice. Do not silently swallow actual operation failures.

## Template portability

`src/app/` imports its siblings through `crate::app`, never the package name.
`src/lib.rs` only exports the app and the launcher imports through the crate root.
This lets `setup.sh --app-only` remove the library and declare `mod app` without
rewriting the implementation. Minimal setup instead copies the complete
`examples/hello_world.rs` into `src/main.rs` and removes the showcase.

## Verification

Pure tests cover state, themes, and preferences. GPUI simulation tests are for
behavior requiring entities, actions, windows, or the executor. Every standalone
example compiles with `cargo check --all-targets`. Keyboard regression tests
include key release, since `simulate_keystrokes` alone only sends key-down and
would miss GPUI’s built-in click activation. CI builds across native
platforms and tests setup in scratch copies. Before release, exercise real
windows on each supported platform, including keyboard and graphics behavior.
