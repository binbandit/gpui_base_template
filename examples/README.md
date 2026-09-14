# Examples

Each file is a complete program using published GPUI 0.2.2. None imports the
starter app or needs its assets.

| Run | What to read |
| --- | --- |
| `cargo run --example hello_world` | Application startup, a window, `Render`, and last-window quit |
| `cargo run --example counter` | Entity state, typed actions, pointer input, focus, and Tab traversal |
| `cargo run --example async_task` | A retained timer task, weak updates, and cancellation by dropping the task |

Start with hello world, then the counter, then the async example. The counter
accepts ↑/↓ and focused Enter/Space activation. The async example uses Enter/Space
to start or cancel its timer and Escape to cancel; no records or network service
are involved. Focused `on_click` handlers receive Enter/Space activation directly
from GPUI, so the examples do not add duplicate key handlers. All examples quit
when their last window closes.

The small examples intentionally keep presentation in the same file. The full
app under `src/app/` demonstrates separating routes, shared controls, semantic
themes, and settings once that separation is useful.

For a project based on hello world:

```bash
./setup.sh my-app --minimal
```

Setup removes the showcase and unused dependencies. `cargo check --all-targets`
compiles every example while they remain in the project. GPUI is GPU accelerated,
so running them needs a supported desktop environment and graphics driver.

For more APIs, use the [versioned upstream examples](https://docs.rs/crate/gpui/0.2.2/source/examples/),
not the incompatible current Zed checkout. Custom control accessibility limits
are documented in [the accessibility guide](../docs/ACCESSIBILITY.md).
