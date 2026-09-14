//! The complete starter application.
//!
//! This folder is self-contained: it depends only on external crates and other
//! modules below `crate::app`. That invariant lets `setup.sh --app-only` fold it
//! into the binary unchanged.

mod actions;
mod assets;
pub mod components;
mod pages;
mod root;
pub mod services;
mod shell;
mod state;
mod theme;

use anyhow::{Context as _, Result};
use gpui::{App, AppContext, Application, Bounds, WindowBounds, WindowOptions, px, size};
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

pub use root::RootView;
pub use services::{AppSettings, SettingsStore};
pub use state::{Route, SyncState};
pub use theme::{Theme, ThemeMode};

/// Starts logging, loads settings, registers key bindings, and opens the main
/// window. Recoverable settings errors are shown inside the app instead of
/// preventing startup.
pub fn run() -> Result<()> {
    init_tracing();

    let (settings_store, path_warning) = SettingsStore::for_app();
    let (settings, load_warning) = settings_store.load_or_default();
    let settings_warning = match (path_warning, load_warning) {
        (Some(path), Some(load)) => Some(format!("{path} {load}")),
        (Some(warning), None) | (None, Some(warning)) => Some(warning),
        (None, None) => None,
    };

    if let Some(warning) = &settings_warning {
        warn!(%warning, "could not load preferences");
    }

    info!(gpui_version = "0.2.2", "starting GPUI application");

    Application::new()
        .with_assets(assets::EmbeddedAssets)
        .run(move |cx: &mut App| {
            actions::bind_keys(cx);
            Theme::install(settings.theme, cx);

            let bounds = Bounds::centered(None, size(px(1180.0), px(760.0)), cx);
            let store = settings_store.clone();
            let app_settings = settings.clone();
            let warning = settings_warning.clone();

            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    window_min_size: Some(size(px(720.0), px(520.0))),
                    titlebar: Some(gpui::TitlebarOptions {
                        title: Some("GPUI Starter".into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                move |window, cx| {
                    cx.new(|cx| RootView::new(app_settings, store, warning, window, cx))
                },
            )
            .context("open main window")
            .expect("GPUI could not open the main window");

            cx.on_window_closed(|cx| {
                if cx.windows().is_empty() {
                    cx.quit();
                }
            })
            .detach();

            cx.activate(true);
        });

    Ok(())
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("gpui_base_framework=info,warn"));

    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .compact()
        .try_init();
}
