//! Infrastructure boundaries for durable or external work.
//!
//! Add filesystem, network, database, and operating-system integrations here.
//! Services should return data/errors; GPUI entities decide how results affect
//! state and which feedback is shown to users.

mod settings;

pub use settings::{AppSettings, SettingsStore};
