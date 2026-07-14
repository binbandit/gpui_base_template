# AGENTS.md

Guidance for AI coding agents and contributors working in this repository.

<!-- template-only:start -->
## What this repository is

This is a **template repository**, not a general-purpose GPUI wrapper crate.
People clone it, run `./setup.sh <name>`, and own the generated application.
Optimize changes for a developer reading the code for the first time.

## Template invariants — do not break these

1. **`src/app/` is self-contained.** It may import `std`, dependencies, and
   `crate::app::...`, never the package name or implementation outside the module.
   This lets `setup.sh --app-only` keep the folder unchanged in a binary crate.
2. **The launcher imports only through the crate root.** `src/main.rs` uses
   `gpui_base_framework::run`; app-only setup rewrites that mechanically to
   `crate::app::run`.
3. **Names are literal replacement anchors.** Keep `gpui-base-framework` and
   `gpui_base_framework` unsplit anywhere they must be renamed by setup.
4. **`setup.sh` supports GNU and BSD userlands.** Avoid GNU-only `sed`, `awk`, or
   `find` extensions. CI runs all modes on Ubuntu and macOS.
5. **`src/main.rs` keeps a top-level `use ` line.** App-only mode inserts `mod app;`
   before the first such line.
6. **Marker blocks are load-bearing.** Setup removes the template comment block in
   Cargo.toml and `template-only` blocks in this file. Generated-app guidance stays
   outside them.
7. **Examples are self-contained.** Each `examples/*.rs` is one complete program,
   uses the published 0.2.2 API, and is listed in README plus examples/README.
8. **The MSRV lives in Cargo.toml.** CI reads `rust-version`; update README when it
   changes.
9. **The GPUI dependency stays one coherent API line.** Do not combine published
   0.2.2 with current Zed `gpui_platform` crates. Update the exact pin, lockfile,
   docs, examples, and startup API together.
10. **Minimal setup must remain truly minimal.** It copies `examples/hello_world.rs`,
    removes `src/app`, assets, the library, examples, showcase dependencies, and
    then compiles.

## Checks before committing

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
cargo clippy --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
shellcheck setup.sh
```

Test mutating setup modes in scratch copies; never run setup in this working tree:

```bash
for mode in default app-only minimal; do
  scratch="$(mktemp -d)"
  cp -R . "$scratch/project"
  case "$mode" in
    default) flags="--yes" ;;
    app-only) flags="--app-only --yes" ;;
    minimal) flags="--minimal --yes" ;;
  esac
  (cd "$scratch/project" && ./setup.sh test-app $flags && cargo test --all-targets)
done
```

If source behavior changes, update README, QUICKSTART, CHEATSHEET, architecture,
examples, and generated-app guidance together.
<!-- template-only:end -->

## Project layout

```text
src/app/
  actions.rs          Typed commands and default shortcuts
  assets.rs           Embedded GPUI AssetSource
  components/         Public reusable controls + ownership guide
  pages/              Route-level screen composition
  services/           Settings and future external IO boundaries
  shell/              Sidebar, header, and global notices
  root.rs             State, tasks, subscriptions, action handlers
  state.rs            Pure product state
  theme.rs            Semantic design tokens and Theme global
  mod.rs              Startup/bootstrap and public surface
src/lib.rs             Thin app export (removed by --app-only)
src/main.rs            Tiny launcher
examples/              Self-contained GPUI learning programs
```

## Architecture in one paragraph

`app::run` configures tracing, loads settings, installs the global semantic theme,
registers actions, and opens one window whose root is `Entity<RootView>`. RootView
owns page state, a retained async `Task`, and a retained typed-event `Subscription`.
Route rendering lives in `pages/`, stable chrome in `shell/`, reusable UI in
`components/`, and external IO in `services/`. Pure transitions live in `state.rs`;
assets are embedded. No custom lifecycle shadows GPUI.

## GPUI 0.2.2 rules

- Startup is `Application::new()`, not `gpui_platform::application()`.
- Import `AppContext` when calling `cx.new` from an `App`.
- Mutate entities through `update` and call `cx.notify()` when observers/rendering
  need invalidation.
- Dropping a `Task` cancels it. Store tasks or detach deliberately.
- Dropping a `Subscription` unsubscribes. Store it or detach deliberately.
- Action handlers should represent logical commands; pointer and keyboard input
  dispatch the same actions.
- Use globals only for process-wide state. Product state belongs in entities or
  plain structs.
- A scrollable div needs an element id before `.overflow_scroll()`.
- GPUI 0.2.2 has focus/tab APIs but no public semantic accessibility-role/state
  API. Preserve visible focus and contrast tests, keep `docs/ACCESSIBILITY.md`
  honest, and do not claim assistive-technology compliance without a real audit.
- Do not fake text input with key-down characters. Use `EntityInputHandler`, IME,
  UTF-16 ranges, grapheme boundaries, selection, and shaping, starting from the
  exact published example.

## Error and lifetime policy

Window creation failures are fatal. Preference corruption/save failures and normal
operation errors are recoverable: log context and show a user-facing notice. Calls
after `await` are fallible because the entity/window/app may have closed; do not
unwrap them. Do not detach a task merely to silence `must_use`.

## Testing patterns

Use plain unit tests for `Route`, settings serialization, theme choice, and other
pure behavior. Use `#[gpui::test]` only when entity/action/executor simulation is
material. Every example must compile under `cargo check --all-targets`. CI host
checks are still not runtime smoke tests; launch native windows before a release.

## Style

- Rust 2024, `unsafe_code = "forbid"` for project code.
- Clippy warnings are errors; fix warnings rather than broadly allowing them.
- Public API gets rustdoc; `RUSTDOCFLAGS=-D warnings` must pass.
- Prefer semantic theme roles over raw colors outside `theme.rs`.
- Keep route screens in `pages/`, shared stateless UI in `components/`, stable
  window chrome in `shell/`, and external IO in `services/`. Do not move a
  page-only helper into the reusable kit before it has a real second caller.
- Explain ownership, portability, lifetime, and error-policy reasons — not syntax.
- Keep dependencies intentional. GPUI already supplies an executor and state model.
