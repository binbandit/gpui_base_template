//! Pure application state. Keeping transitions independent from GPUI makes
//! domain behavior fast and straightforward to test.

/// Top-level destinations in the starter shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Route {
    #[default]
    Overview,
    Components,
    Settings,
}

impl Route {
    pub const fn title(self) -> &'static str {
        match self {
            Self::Overview => "Overview",
            Self::Components => "Components",
            Self::Settings => "Settings",
        }
    }

    pub const fn eyebrow(self) -> &'static str {
        match self {
            Self::Overview => "START HERE",
            Self::Components => "DESIGN SYSTEM",
            Self::Settings => "PREFERENCES",
        }
    }

    pub const fn nav_id(self) -> &'static str {
        match self {
            Self::Overview => "nav-overview",
            Self::Components => "nav-components",
            Self::Settings => "nav-settings",
        }
    }

    pub const fn compact_label(self) -> &'static str {
        match self {
            Self::Overview => "OV",
            Self::Components => "UI",
            Self::Settings => "SET",
        }
    }

    pub fn shortcut(self) -> &'static str {
        if cfg!(target_os = "macos") {
            match self {
                Self::Overview => "⌘1",
                Self::Components => "⌘2",
                Self::Settings => "⌘,",
            }
        } else {
            match self {
                Self::Overview => "Ctrl+1",
                Self::Components => "Ctrl+2",
                Self::Settings => "Ctrl+,",
            }
        }
    }
}

/// State of the example background task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SyncState {
    #[default]
    Idle,
    Running,
    Complete {
        records: u32,
    },
}

impl SyncState {
    pub const fn is_running(self) -> bool {
        matches!(self, Self::Running)
    }

    pub fn label(self) -> String {
        match self {
            Self::Idle => "Ready to sync".into(),
            Self::Running => "Syncing workspace…".into(),
            Self::Complete { records } => format!("Synced {records} records"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Route, SyncState};

    #[test]
    fn routes_have_stable_human_labels() {
        assert_eq!(Route::Overview.title(), "Overview");
        assert_eq!(Route::Components.eyebrow(), "DESIGN SYSTEM");
        assert_eq!(Route::Settings.title(), "Settings");
    }

    #[test]
    fn sync_state_reports_activity_and_results() {
        assert!(!SyncState::Idle.is_running());
        assert!(SyncState::Running.is_running());
        assert_eq!(
            SyncState::Complete { records: 42 }.label(),
            "Synced 42 records"
        );
    }
}
