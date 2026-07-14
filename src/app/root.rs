//! Root entity and application coordination.
//!
//! Rendering details live in `pages/` and `shell/`; this type owns state,
//! subscriptions, tasks, and action handlers.

use crate::app::actions::{
    CopyInstallCommand, DismissNotice, FocusNext, FocusPrevious, IncrementCounter,
    NavigateComponents, NavigateOverview, NavigateSettings, Quit, RunSync, ToggleAnimations,
    ToggleCompactSidebar, ToggleTheme,
};
use crate::app::pages::{self, PageContext};
use crate::app::services::{AppSettings, SettingsStore};
use crate::app::shell::{Notice, render_header, render_notice, render_sidebar};
use crate::app::state::{Route, SyncState};
use crate::app::theme::Theme;
use gpui::{
    App, ClipboardItem, Context, Entity, EventEmitter, FocusHandle, Focusable, IntoElement, Render,
    Subscription, Task, Window, div, prelude::*, px,
};
use std::time::Duration;
use tracing::{info, warn};

struct SyncFinished {
    records: u32,
}

struct SyncModel;

impl EventEmitter<SyncFinished> for SyncModel {}

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
    sync_model: Entity<SyncModel>,
    sync_task: Option<Task<()>>,
    _sync_subscription: Subscription,
    counter: u32,
    notice: Option<Notice>,
    focus_handle: FocusHandle,
}

impl RootView {
    pub fn new(
        settings: AppSettings,
        settings_store: SettingsStore,
        warning: Option<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let sync_model = cx.new(|_| SyncModel);
        let sync_subscription =
            cx.subscribe(&sync_model, |view, _model, event: &SyncFinished, cx| {
                view.sync = SyncState::Complete {
                    records: event.records,
                };
                view.notice = Some(Notice::success(format!(
                    "Background sync finished: {} records are ready.",
                    event.records
                )));
                view.sync_task = None;
                cx.notify();
            });
        let focus_handle = cx.focus_handle();
        window.focus(&focus_handle);

        Self {
            route: Route::Overview,
            settings,
            settings_store,
            sync: SyncState::Idle,
            sync_model,
            sync_task: None,
            _sync_subscription: sync_subscription,
            counter: 12,
            notice: warning.map(Notice::warning),
            focus_handle,
        }
    }

    fn navigate(&mut self, route: Route, cx: &mut Context<Self>) {
        self.route = route;
        self.notice = None;
        cx.notify();
    }

    fn navigate_overview(&mut self, _: &NavigateOverview, _: &mut Window, cx: &mut Context<Self>) {
        self.navigate(Route::Overview, cx);
    }

    fn navigate_components(
        &mut self,
        _: &NavigateComponents,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.navigate(Route::Components, cx);
    }

    fn navigate_settings(&mut self, _: &NavigateSettings, _: &mut Window, cx: &mut Context<Self>) {
        self.navigate(Route::Settings, cx);
    }

    fn toggle_theme(&mut self, _: &ToggleTheme, _: &mut Window, cx: &mut Context<Self>) {
        self.settings.theme = self.settings.theme.toggled();
        Theme::set_mode(self.settings.theme, cx);
        self.persist_settings();
        cx.notify();
    }

    fn toggle_animations(&mut self, _: &ToggleAnimations, _: &mut Window, cx: &mut Context<Self>) {
        self.settings.animations = !self.settings.animations;
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
        self.notice = Some(Notice::info(
            "Sync is running on GPUI's executor; the window stays responsive.",
        ));

        let timer = cx.background_executor().timer(Duration::from_millis(1_250));
        let sync_model = self.sync_model.clone();
        self.sync_task = Some(cx.spawn(async move |_view, cx| {
            timer.await;
            let _ = sync_model.update(cx, |_model, cx| {
                cx.emit(SyncFinished { records: 384 });
                cx.notify();
            });
        }));
        cx.notify();
    }

    fn copy_install_command(
        &mut self,
        _: &CopyInstallCommand,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.write_to_clipboard(ClipboardItem::new_string(
            "./setup.sh my-gpui-app --app-only".into(),
        ));
        self.notice = Some(Notice::success("Setup command copied to the clipboard."));
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
            .on_action(cx.listener(Self::toggle_animations))
            .on_action(cx.listener(Self::toggle_compact_sidebar))
            .on_action(cx.listener(Self::increment_counter))
            .on_action(cx.listener(Self::run_sync))
            .on_action(cx.listener(Self::copy_install_command))
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
                            .id("main-scroll")
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
