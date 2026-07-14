//! Reusable GPUI-native UI building blocks.
//!
//! Put stateless controls that are shared by multiple pages in this directory.
//! They implement `RenderOnce + IntoElement`, use semantic [`Theme`](crate::app::Theme)
//! tokens, and dispatch typed GPUI actions. Stateful controls should instead own
//! an `Entity<T>` and implement `Render`; do not invent a second lifecycle.
//!
//! ```no_run
//! use gpui_base_framework::app::components::{Button, ButtonVariant};
//! # gpui::actions!(docs, [Save]);
//! let button = Button::new("save", "Save", Save).variant(ButtonVariant::Primary);
//! # let _ = button;
//! ```

mod badge;
mod button;
mod card;
mod toggle;

pub use badge::{Badge, BadgeTone};
pub use button::{Button, ButtonSize, ButtonVariant};
pub use card::Card;
pub use toggle::Toggle;
