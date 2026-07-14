# Examples

Each example is one self-contained GPUI program that can be read top to bottom.
They deliberately use the published `gpui 0.2.2` API rather than Zed `main`.

## Run

```bash
cargo run --example hello_world
cargo run --example counter
cargo run --example async_task
```

| Example | Concepts |
| --- | --- |
| `hello_world` | `Application`, window creation, root entity, `Render`, clean last-window quit |
| `counter` | Typed actions, key bindings, pointer events, `cx.listener`, `cx.notify()` |
| `async_task` | GPUI timer, retained `Task`, weak-entity update after `await` |

## Learning path

1. Start with `hello_world`; it is the minimum native app.
2. Use `counter` to learn the action system instead of wiring every input directly.
3. Use `async_task` before adding network, filesystem, or subprocess work.
4. Return to the showcase in `src/app/` for a real multi-page structure, reusable
   `RenderOnce` components, themes, settings, assets, and typed subscriptions.

## Start from an example

While the library target is still present:

```bash
cp examples/hello_world.rs src/main.rs
cargo run
```

For a clean bare project, prefer the automated path:

```bash
./setup.sh my-app --minimal
```

The setup script removes the showcase and unused dependencies, validates the
result, and then deletes itself.
