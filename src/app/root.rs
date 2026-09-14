//! Root entity and application coordination.
//!
//! Rendering details live in `pages/` and `shell/`; this type owns state,
//! tasks and action handlers.

use crate::app::actions::{
    DismissNotice, FocusNext, FocusPrevious, IncrementCounter, NavigateComponents,
    NavigateOverview, NavigateSettings, Quit, RunSync, ToggleCompactSidebar, ToggleTheme,
};
use crate::app::pages::{self, PageContext};
use crate::app::services::{AppSettings, SettingsStore};
use crate::app::shell::{Notice, render_header, render_notice, render_sidebar};
use crate::app::state::{Route, SyncState};
use crate::app::theme::Theme;
use gpui::{
    App, Context, FocusHandle, Focusable, IntoElement, Render, Task, Window, div, prelude::*, px,
};
use std::time::Duration;
use tracing::{info, warn};

/// The starter's root GPUI view.
///
/// `RootView` is deliberately a coordinator rather than a screen-sized render
/// function: route screens live in `pages/`, reusable controls in `components/`,
/// application chrome in `shell/`, and filesystem boundaries in `services/`.
pub struct RootView {
    route: Route,
    settings: AppSettings,
    settings_store: SettingsStore,
    sync: SyncState,
    sync_task: Option<Task<()>>,
    counter: u32,
    notice: Option<Notice>,
    focus_handle: FocusHandle,
}

impl RootView {
    /// Creates the root view and establishes initial keyboard focus.
    pub fn new(
        settings: AppSettings,
        settings_store: SettingsStore,
        warning: Option<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        window.focus(&focus_handle);

        Self {
            route: Route::Overview,
            settings,
            settings_store,
            sync: SyncState::Idle,
            sync_task: None,
            counter: 0,
            notice: warning.map(Notice::warning),
            focus_handle,
        }
    }

    fn navigate(&mut self, route: Route, window: &mut Window, cx: &mut Context<Self>) {
        self.route = route;
        // The previous page may own the focused control. Restore the persistent
        // root focus before that control disappears so shortcuts keep working.
        window.focus(&self.focus_handle);
        cx.notify();
    }

    fn navigate_overview(
        &mut self,
        _: &NavigateOverview,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.navigate(Route::Overview, window, cx);
    }

    fn navigate_components(
        &mut self,
        _: &NavigateComponents,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.navigate(Route::Components, window, cx);
    }

    fn navigate_settings(
        &mut self,
        _: &NavigateSettings,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.navigate(Route::Settings, window, cx);
    }

    fn toggle_theme(&mut self, _: &ToggleTheme, _: &mut Window, cx: &mut Context<Self>) {
        self.settings.theme = self.settings.theme.toggled();
        Theme::set_mode(self.settings.theme, cx);
        self.persist_settings();
        cx.notify();
    }

    fn toggle_compact_sidebar(
        &mut self,
        _: &ToggleCompactSidebar,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.settings.compact_sidebar = !self.settings.compact_sidebar;
        self.persist_settings();
        cx.notify();
    }

    fn increment_counter(&mut self, _: &IncrementCounter, _: &mut Window, cx: &mut Context<Self>) {
        self.counter = self.counter.saturating_add(1);
        cx.notify();
    }

    fn run_sync(&mut self, _: &RunSync, _: &mut Window, cx: &mut Context<Self>) {
        if self.sync.is_running() {
            return;
        }

        self.sync = SyncState::Running;
        self.notice = Some(Notice::info("Running the demo sync…"));

        let timer = cx.background_executor().timer(Duration::from_millis(1_250));
        // Keep the task alive with the view. The weak handle supplied by spawn
        // lets closing the window cancel work without keeping the view alive.
        self.sync_task = Some(cx.spawn(async move |view, cx| {
            timer.await;
            let _ = view.update(cx, |view, cx| {
                view.sync = SyncState::Complete { records: 384 };
                view.notice = Some(Notice::success("Demo sync complete: 384 sample records."));
                view.sync_task = None;
                cx.notify();
            });
        }));
        cx.notify();
    }

    fn dismiss_notice(&mut self, _: &DismissNotice, _: &mut Window, cx: &mut Context<Self>) {
        self.notice = None;
        cx.notify();
    }

    fn focus_next(&mut self, _: &FocusNext, window: &mut Window, _: &mut Context<Self>) {
        window.focus_next();
    }

    fn focus_previous(&mut self, _: &FocusPrevious, window: &mut Window, _: &mut Context<Self>) {
        window.focus_prev();
    }

    fn quit(&mut self, _: &Quit, _: &mut Window, cx: &mut Context<Self>) {
        cx.quit();
    }

    fn persist_settings(&mut self) {
        if let Err(error) = self.settings_store.save(&self.settings) {
            warn!(%error, "could not persist settings");
            self.notice = Some(Notice::warning(format!(
                "Preference changed for this session, but could not be saved: {error:#}"
            )));
        } else {
            info!(?self.settings, "saved preferences");
        }
    }
}

impl Focusable for RootView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for RootView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::current(cx);
        let narrow = window.viewport_size().width < px(1040.0);
        let compact = narrow || self.settings.compact_sidebar;
        let page_context = PageContext {
            settings: &self.settings,
            sync: self.sync,
            counter: self.counter,
            narrow,
            theme: &theme,
        };
        let content = pages::render(self.route, &page_context);

        div()
            .key_context("Starter")
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(Self::navigate_overview))
            .on_action(cx.listener(Self::navigate_components))
            .on_action(cx.listener(Self::navigate_settings))
            .on_action(cx.listener(Self::toggle_theme))
            .on_action(cx.listener(Self::toggle_compact_sidebar))
            .on_action(cx.listener(Self::increment_counter))
            .on_action(cx.listener(Self::run_sync))
            .on_action(cx.listener(Self::dismiss_notice))
            .on_action(cx.listener(Self::focus_next))
            .on_action(cx.listener(Self::focus_previous))
            .on_action(cx.listener(Self::quit))
            .flex()
            .size_full()
            .overflow_hidden()
            .bg(theme.canvas)
            .text_color(theme.text)
            .font_family("system-ui")
            .child(render_sidebar(self.route, compact, &theme))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .min_w(px(0.0))
                    .h_full()
                    .child(render_header(
                        self.route,
                        self.sync,
                        &self.settings,
                        narrow,
                        &theme,
                    ))
                    .child(
                        div()
                            .id(("main-scroll", self.route as usize))
                            .flex()
                            .flex_col()
                            .flex_1()
                            .overflow_scroll()
                            .p_6()
                            .child(content),
                    )
                    .children(render_notice(self.notice.as_ref(), &theme)),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::{actions, theme::ThemeMode};
    use gpui::TestAppContext;

    fn press_key(cx: &mut TestAppContext, window: gpui::AnyWindowHandle, key: &str) {
        // simulate_keystrokes only emits KeyDown. GPUI's native click behavior
        // activates focused controls on KeyUp, so exercise both event phases.
        cx.simulate_keystrokes(window, key);
        gpui::VisualTestContext::from_window(window, cx).simulate_event(gpui::KeyUpEvent {
            keystroke: gpui::Keystroke::parse(key).unwrap(),
        });
        cx.run_until_parked();
    }

    #[gpui::test]
    fn sync_runs_once_and_keeps_actions_responsive(cx: &mut TestAppContext) {
        let directory = tempfile::tempdir().unwrap();
        cx.update(|cx| {
            Theme::install(ThemeMode::Dark, cx);
            actions::bind_keys(cx);
        });
        let window = cx.add_window(|window, cx| {
            RootView::new(
                AppSettings::default(),
                SettingsStore::at(directory.path().join("settings.json")),
                None,
                window,
                cx,
            )
        });
        cx.dispatch_action(window.into(), RunSync);
        cx.executor().advance_clock(Duration::from_millis(750));
        cx.dispatch_action(window.into(), RunSync);
        cx.dispatch_action(window.into(), IncrementCounter);
        window
            .update(cx, |view, _, _| {
                assert_eq!(view.sync, SyncState::Running);
                assert_eq!(view.counter, 1);
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_millis(500));
        cx.run_until_parked();
        window
            .update(cx, |view, _, _| {
                assert_eq!(view.sync, SyncState::Complete { records: 384 });
                assert!(view.sync_task.is_none());
            })
            .unwrap();
    }

    #[gpui::test]
    fn shortcuts_survive_removed_and_disabled_controls(cx: &mut TestAppContext) {
        let directory = tempfile::tempdir().unwrap();
        cx.update(|cx| {
            Theme::install(ThemeMode::Dark, cx);
            actions::bind_keys(cx);
        });
        let window = cx.add_window(|window, cx| {
            RootView::new(
                AppSettings::default(),
                SettingsStore::at(directory.path().join("settings.json")),
                None,
                window,
                cx,
            )
        });
        // Traverse the shell and focus Overview's counter button. This goes
        // through real keyboard dispatch instead of invoking action handlers.
        cx.simulate_keystrokes(window.into(), "tab tab tab tab tab tab");
        press_key(cx, window.into(), "enter");
        press_key(cx, window.into(), "space");
        window
            .update(cx, |view, _, _| assert_eq!(view.counter, 2))
            .unwrap();
        let settings_key = if cfg!(target_os = "macos") {
            "cmd-,"
        } else {
            "ctrl-,"
        };
        cx.simulate_keystrokes(window.into(), settings_key);
        window
            .update(cx, |view, _, _| assert_eq!(view.route, Route::Settings))
            .unwrap();
        cx.simulate_keystrokes(window.into(), "secondary-2");
        window
            .update(cx, |view, _, _| assert_eq!(view.route, Route::Components))
            .unwrap();
        cx.simulate_keystrokes(window.into(), "secondary-1");
        window
            .update(cx, |view, _, _| assert_eq!(view.route, Route::Overview))
            .unwrap();
        // Starting a task disables its focused button. The app key context
        // must remain reachable while that control is unavailable.
        cx.simulate_keystrokes(window.into(), "tab tab tab tab tab");
        press_key(cx, window.into(), "enter");
        window
            .update(cx, |view, _, _| assert_eq!(view.sync, SyncState::Running))
            .unwrap();
        cx.simulate_keystrokes(window.into(), "secondary-2");
        window
            .update(cx, |view, _, _| assert_eq!(view.route, Route::Components))
            .unwrap();
    }

    #[gpui::test]
    fn navigation_preserves_notice_until_dismissed(cx: &mut TestAppContext) {
        let directory = tempfile::tempdir().unwrap();
        cx.update(|cx| Theme::install(ThemeMode::Dark, cx));
        let window = cx.add_window(|window, cx| {
            RootView::new(
                AppSettings::default(),
                SettingsStore::at(directory.path().join("settings.json")),
                Some("Preferences could not be read".into()),
                window,
                cx,
            )
        });
        cx.dispatch_action(window.into(), NavigateSettings);
        window
            .update(cx, |view, _, _| {
                assert_eq!(view.route, Route::Settings);
                assert!(view.notice.is_some());
            })
            .unwrap();
        cx.dispatch_action(window.into(), DismissNotice);
        window
            .update(cx, |view, _, _| assert!(view.notice.is_none()))
            .unwrap();
    }
}
