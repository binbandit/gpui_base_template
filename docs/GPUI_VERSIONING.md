# GPUI Versioning and Source of Truth

GPUI is pre-1.0 and evolves with Zed. This template intentionally optimizes for
a reproducible starting point rather than silently following a moving branch.

## The two API lines

### Published crates.io 0.2.2 — used here

The crates.io metadata and docs.rs release listing still identify 0.2.2 as the
latest published release at the 2026-09-14 audit. Updating dependencies therefore keeps
this exact pin; switching to Zed Git would be a separate API migration.

- Dependency: `gpui = "=0.2.2"`
- Startup: `Application::new()`
- Platform implementations are part of the published crate.
- Immutable reference examples:
  <https://docs.rs/crate/gpui/0.2.2/source/examples/>

### Current Zed repository

Zed `main` has continued changing after the 0.2.2 publication. Its current
examples use `gpui_platform::application()` and a split set of platform crates.
That source can still carry a `0.2.2` package version, so the version string alone
does not prove compatibility with crates.io 0.2.2.

Do not copy current `main` examples into this project without porting them. Do
not add `gpui_platform` to a published-0.2.2 project.

## Reference sources

Check the package source, not just a version string displayed on GitHub:

- [Published crate metadata](https://crates.io/crates/gpui)
- [0.2.2 application startup](https://docs.rs/gpui/0.2.2/gpui/struct.Application.html)
- [0.2.2 examples](https://docs.rs/crate/gpui/0.2.2/source/examples/)
- [Current Zed hello world](https://github.com/zed-industries/zed/blob/main/crates/gpui/examples/hello_world.rs)

These are the references used for this cleanup. `Application::new()`,
context-based entity access, explicit task/subscription ownership, and the
published focus APIs remain the correct patterns for the selected release.

## Why the exact pin and lockfile

GPUI is pre-1.0, and its own README warns that breaking changes are frequent.
The exact requirement plus committed `Cargo.lock` makes examples and setup-script
checks reproducible. Dependabot can propose upgrades visibly instead of changing
new clones unexpectedly.

## Other dependencies

Update compatible dependencies with `cargo update`, review `Cargo.lock`, and run
the validation commands and setup modes before keeping the result. Test the
`rust-version` declared in `Cargo.toml` as well as current stable Rust; package
metadata alone does not prove minimum-version compatibility. Dependabot groups
Cargo and GitHub Actions updates separately.

The settings service uses `tempfile` for atomic replacement; minimal setup removes
this and the other showcase dependencies, leaving only GPUI.

## macOS runtime shaders

The dependency enables GPUI's `runtime_shaders` feature. Without it, published
0.2.2 compiles Metal shaders at build time and recent Xcode installations may
require this extra component:

```bash
xcodebuild -downloadComponent MetalToolchain
```

Runtime shaders remove that first-run hurdle. If release startup measurements
justify build-time shaders, install the component and remove the feature in a
controlled change.

## Upgrade checklist

1. Choose either a new crates.io release or one full Zed Git commit. Never track
   `main` and never mix Git GPUI crates with published platform crates.
2. Read the selected source's `Cargo.toml`, README, examples, and platform crate
   layout before editing application code.
3. Update startup, context, window, entity, and `Element` APIs as one migration.
4. Run every command in `AGENTS.md` and all setup-script modes.
5. Run the platform CI matrix.
6. Launch a real window on macOS, Linux (Wayland and X11 if supported), and
   Windows.
7. Update every version reference in README, QUICKSTART, CHEATSHEET, examples,
   and this document.
