# GPUI Starter

A GitHub template for native Rust apps and tools built with
[GPUI](https://gpui.rs). Clone it, run the setup script, and own the code.

The starter is a small desktop workbench: three screens, light and dark themes,
keyboard navigation, reusable controls, saved preferences, and an async example.
It uses GPUI directly, so the code you learn here carries into your application.

## Start a project

Use GitHub's **Use this template** button, or clone:

```bash
git clone https://github.com/binbandit/gpui_base_template my-app
cd my-app
./setup.sh my-app --app-only
cargo run
```

Install **Rust 1.94 or newer** and the [native build prerequisites](#native-build-prerequisites)
first. The minimum Rust version is declared in `Cargo.toml`.

| Setup | Result |
| --- | --- |
| `./setup.sh my-app --app-only` | App with a binary target; removes the library and learning examples |
| `./setup.sh my-app` | App, library target, and learning examples |
| `./setup.sh my-app --minimal` | One-file hello world; GPUI is the only dependency |
| `./setup.sh my-app --no-examples` | App and library, without examples |

Setup renames the package and configuration identity, replaces this README,
checks the result, and removes itself on success. Run it in your new project,
not in a template checkout you intend to maintain. See `./setup.sh --help` for
noninteractive setup and Git history options. Cargo and rustfmt are required;
setup restores the original files if validation fails and preserves existing
backup files. `--fresh-git` starts new history after validation and is unavailable
in a linked Git worktree.

## What to change

| Location | Responsibility |
| --- | --- |
| `src/app/mod.rs` | Startup, theme installation, and main window |
| `src/app/root.rs` | App state, action handlers, and owned background task |
| `src/app/state.rs` | Route and demo state types and labels |
| `src/app/pages/` | Overview, Components, and Settings screens |
| `src/app/shell/` | Navigation, header, and notices |
| `src/app/components/` | Small `RenderOnce` buttons, cards, badges, and toggles |
| `src/app/theme.rs` | Semantic colors and theme choice |
| `src/app/services/` | Settings persistence and future external IO |
| `src/app/assets.rs` | Embedded assets |

Replace the routes and pages with your product. Add shared controls only when
there is a real second caller. Keep state in the owning entity or plain Rust
struct; use GPUI globals for process-wide values such as the theme.

The demo sync uses a timer, not a network service. It shows task ownership and
completion handling without adding a runtime or fake backend. The settings page
saves real preferences in the OS configuration directory and reports failures.

## Learn GPUI

Each example is a complete program with no imports from the starter:

```bash
cargo run --example hello_world  # Window and Render
cargo run --example counter      # State, actions, focus, and keyboard input
cargo run --example async_task   # Retained task, completion, and cancellation
```

Use the [example guide](examples/README.md) and [cheat sheet](CHEATSHEET.md) while
building. GPUI handles entity lifetimes, rendering, actions, and async execution;
the starter adds no parallel component lifecycle.

## GPUI version

This project pins the published **`gpui = "=0.2.2"`** and commits `Cargo.lock`.
Use the [versioned API documentation](https://docs.rs/gpui/0.2.2/gpui/) and
[bundled upstream examples](https://docs.rs/crate/gpui/0.2.2/source/examples/).
Current Zed repository examples use a different platform API, including
`gpui_platform::application()`, and cannot be pasted into this release unchanged.
See [versioning and upgrades](docs/GPUI_VERSIONING.md).

GPUI 0.2.2 supports keyboard focus but does not expose semantic accessibility
roles/states for these custom controls. Visible focus and contrast checks are
included; assistive-technology support needs a separate product decision and
audit. See [accessibility](docs/ACCESSIBILITY.md).

For editable text, start with the versioned upstream
[input example](https://docs.rs/crate/gpui/0.2.2/source/examples/input.rs).
A key handler that appends characters is not an IME-aware text field.

## Native build prerequisites

- **macOS:** Xcode and its command-line tools (`xcode-select --install`). The
  `runtime_shaders` feature avoids the optional build-time Metal compiler.
- **Windows:** Rust's MSVC toolchain and Visual Studio Build Tools with Desktop
  Development for C++ and a Windows SDK.
- **Linux:** a C/C++ toolchain, font libraries, Vulkan, and Wayland/X11 development
  libraries. On Debian/Ubuntu:

```bash
sudo apt-get install build-essential clang libfontconfig-dev libwayland-dev \
  libxkbcommon-dev libxkbcommon-x11-dev libxcb1-dev libx11-xcb-dev \
  libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libvulkan-dev
```

A working graphics driver and desktop session are required to run the app.
Both Wayland and X11 are enabled by the dependency's default features.

## Check and release

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
cargo clippy --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
shellcheck setup.sh
```

CI checks native builds and the declared Rust minimum. Setup modes are tested
in scratch projects on Linux and macOS. Launch real windows on each supported
platform before shipping; compile checks do not exercise a graphics backend.

The release workflow starts a new release as a draft when you push a `v*` tag.
It uploads native archives and SHA-256 checksums, then publishes only after every
platform build and upload succeeds. A failed first run leaves the release as a
draft; rerun the failed jobs after fixing the cause. Configure bundle identifiers,
icons, signing, and notarization for your product before distribution.

## Further reading

- [Quick start](QUICKSTART.md): first edits after setup
- [Architecture](docs/ARCHITECTURE.md): ownership, lifetimes, and errors
- [Components](src/app/components/README.md): extending the UI kit
- [Contributor and agent guidance](AGENTS.md): template invariants and checks

Licensed under MIT or Apache-2.0, at your option.
