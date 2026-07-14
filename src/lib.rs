//! A production-minded starting point for desktop applications built with GPUI.
//!
//! The application is deliberately contained in [`app`]. The crate root stays
//! thin so `./setup.sh --app-only` can turn the template into a binary-only
//! project without changing the application architecture.

pub mod app;

pub use app::run;
