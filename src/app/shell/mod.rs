//! Stable application chrome around route-level pages.
//!
//! The shell owns presentation for navigation, the page header, and transient
//! notices. It receives state from `RootView` and dispatches typed actions.

mod header;
mod notice;
mod sidebar;

pub(super) use header::render as render_header;
pub(super) use notice::{Notice, render as render_notice};
pub(super) use sidebar::render as render_sidebar;
