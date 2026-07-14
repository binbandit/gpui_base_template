//! Route-level screens.
//!
//! Pages compose reusable controls and receive an immutable snapshot of the
//! state they need. They dispatch actions instead of mutating the root entity
//! directly, which keeps rendering separate from application coordination.

mod components;
mod overview;
mod settings;

use crate::app::services::AppSettings;
use crate::app::state::{Route, SyncState};
use crate::app::theme::Theme;
use gpui::AnyElement;

/// Immutable render data shared by route-level pages.
pub(super) struct PageContext<'a> {
    pub(super) settings: &'a AppSettings,
    pub(super) sync: SyncState,
    pub(super) counter: u32,
    pub(super) narrow: bool,
    pub(super) theme: &'a Theme,
}

/// Maps application routes to their screen modules in one obvious place.
pub(super) fn render(route: Route, context: &PageContext<'_>) -> AnyElement {
    match route {
        Route::Overview => overview::render(context),
        Route::Components => components::render(context),
        Route::Settings => settings::render(context),
    }
}
