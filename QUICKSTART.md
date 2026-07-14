# Quick Start

## 1. Create your project

```bash
git clone https://github.com/binbandit/gpui-base-framework my-desktop-app
cd my-desktop-app
./setup.sh my-desktop-app --app-only
cargo run
```

Use `--minimal` instead of `--app-only` for a one-file hello world with no
showcase architecture. Run `./setup.sh --help` for all modes.

## 2. Know where to edit

```text
src/main.rs                    Tiny launcher
src/app/root.rs                State, tasks, subscriptions, action handlers
src/app/actions.rs             Logical commands and shortcuts
src/app/components/            Public reusable controls and their guide
src/app/pages/                 One module per route-level screen
src/app/shell/                 Navigation, header, and global notices
src/app/services/settings.rs   Persistent preferences boundary
src/app/state.rs               Pure, unit-testable domain state
src/app/theme.rs               Semantic light/dark tokens
src/app/assets.rs              Embedded AssetSource
```

In `--app-only` projects, `src/app/` becomes a module of the binary. Nothing
else about the architecture changes.

## 3. Make the starter yours

1. Replace the showcase routes in `src/app/state.rs`.
2. Build route-level screens as modules under `src/app/pages/`.
3. Put genuinely shared UI in `src/app/components/`; follow its colocated README.
4. Keep semantic color roles in `theme.rs`; change their values to rebrand.
5. Add actions first, then bind keys and pointer controls to those actions.
6. Add slow work with `cx.spawn`; retain or detach every returned `Task`
   deliberately, or move growing IO into `src/app/services/`.
7. Change the `ProjectDirs` identity in `services/settings.rs` if you need to
   migrate existing user data.
8. Read `docs/ACCESSIBILITY.md`; GPUI 0.2.2 cannot expose semantic roles/states
   for these custom controls, so resolve that boundary before claiming compliance.

## Commands

```bash
cargo run
cargo run --example counter
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
cargo clippy --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
```

## GPUI version warning

This template pins crates.io `gpui = "=0.2.2"`. Current Zed repository examples
use a newer, unreleased platform split (`gpui_platform::application()`) and are
not source-compatible even though the workspace package may still report
`0.2.2`. Use the examples bundled with this project or the immutable docs.rs
0.2.2 source when an API differs.

See [docs/GPUI_VERSIONING.md](docs/GPUI_VERSIONING.md) before upgrading.
