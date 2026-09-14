# Quick start

## Create your app

Install Rust and the native prerequisites listed in [README](README.md), then:

```bash
git clone https://github.com/binbandit/gpui_base_template my-app
cd my-app
./setup.sh my-app --app-only
cargo run
```

`--app-only` keeps the full application as a binary crate. Use `--minimal` for
just a window and `Render` implementation. Omit the mode flag to retain the
library and standalone examples. Setup removes itself after successful validation.

If setup has already run, start with `cargo run`; no further setup is needed.
For a minimal project, edit `src/main.rs`. The remaining steps describe the full
starter layout.

## Make it yours

Setup renames the crate and configuration identity. Change the visible
“GPUI Starter” title in `src/app/mod.rs` and branding in `src/app/shell/sidebar.rs`
to your application name.

1. Change `Route` in `src/app/state.rs` and the screens in `src/app/pages/`.
2. Adjust navigation and the header in `src/app/shell/`.
3. Change semantic colors in `src/app/theme.rs`; keep raw colors out of pages.
4. Define logical commands in `src/app/actions.rs`; dispatch the same action
   from mouse and keyboard controls. Use `secondary-` for Command on macOS and
   Ctrl elsewhere. Focused `on_click` handlers already activate on Enter/Space
   release; adding another handler would fire the command twice.
5. Keep product state and task ownership in `src/app/root.rs`. Replace the demo
   timer with your operation, handling closed views after `await`.
6. Put external IO in `src/app/services/`. Set the `ProjectDirs` identity before
   users have saved preferences; changing it later requires a migration.
7. Add reusable UI to `src/app/components/` only when multiple screens need it.

Start with `examples/counter.rs` if entities/actions are new to you. Examples
remain available only when setup preserves them; the
[cheat sheet](CHEATSHEET.md) is retained in full app layouts.

## Check your changes

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
cargo clippy --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
```

Also launch the application, use the keyboard, resize the window, change themes,
and restart to verify saved preferences. Check shortcuts after changing pages
and after a focused button becomes disabled; keep the root focus restoration
and disabled-button focus path when adapting the controls.

## Before adding more

Use the exact [GPUI 0.2.2 docs](https://docs.rs/gpui/0.2.2/gpui/). Zed `main`
contains newer APIs; see [versioning](docs/GPUI_VERSIONING.md) before migrating.
For text editing, adapt the published upstream input example rather than a
character-at-a-time key handler. Read [accessibility](docs/ACCESSIBILITY.md)
before choosing controls for a product that needs assistive-technology support.
