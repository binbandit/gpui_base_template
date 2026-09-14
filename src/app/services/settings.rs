//! Persistent user preferences and their filesystem boundary.

use crate::app::theme::ThemeMode;
use anyhow::{Context as _, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write as _;
use std::path::PathBuf;
use tempfile::NamedTempFile;

/// Preferences that survive restarts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct AppSettings {
    pub theme: ThemeMode,
    pub compact_sidebar: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: ThemeMode::Dark,
            compact_sidebar: false,
        }
    }
}

/// Owns the path and serialization policy for [`AppSettings`].
#[derive(Debug, Clone)]
pub struct SettingsStore {
    path: PathBuf,
}

impl SettingsStore {
    /// Resolves the platform configuration path. On unusual hosts without a
    /// configuration directory, startup remains available with a visible
    /// warning and a temporary fallback path.
    pub fn for_app() -> (Self, Option<String>) {
        if let Some(dirs) = ProjectDirs::from("dev", "GPUI", "gpui-base-framework") {
            return (
                Self {
                    path: dirs.config_dir().join("settings.json"),
                },
                None,
            );
        }

        (
            Self {
                path: std::env::temp_dir()
                    .join("gpui-base-framework")
                    .join("settings.json"),
            },
            Some(
                "The platform configuration directory is unavailable; settings will use a temporary location for this session."
                    .into(),
            ),
        )
    }

    #[cfg(test)]
    pub(crate) fn at(path: PathBuf) -> Self {
        Self { path }
    }

    /// Loads settings. Invalid or unreadable files become a warning and safe
    /// defaults so a damaged preference file never bricks application startup.
    pub fn load_or_default(&self) -> (AppSettings, Option<String>) {
        match self.load() {
            Ok(settings) => (settings, None),
            Err(error)
                if error
                    .downcast_ref::<std::io::Error>()
                    .is_some_and(|e| e.kind() == std::io::ErrorKind::NotFound) =>
            {
                (AppSettings::default(), None)
            }
            Err(error) => (
                AppSettings::default(),
                Some(format!(
                    "Could not load preferences; using defaults: {error:#}"
                )),
            ),
        }
    }

    /// Reads and decodes the preferences, preserving errors for the caller.
    pub fn load(&self) -> Result<AppSettings> {
        let bytes =
            fs::read(&self.path).with_context(|| format!("read {}", self.path.display()))?;
        serde_json::from_slice(&bytes).with_context(|| format!("parse {}", self.path.display()))
    }

    /// Atomically replaces preferences after flushing the new file contents.
    pub fn save(&self, settings: &AppSettings) -> Result<()> {
        let parent = self.path.parent().context("settings path has no parent")?;
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;

        let json = serde_json::to_vec_pretty(settings).context("serialize settings")?;
        // Use a unique file in the same directory: concurrent instances never
        // share a staging file, and persist replaces atomically on all hosts.
        let mut temporary = NamedTempFile::new_in(parent)
            .with_context(|| format!("create temporary preferences in {}", parent.display()))?;
        temporary.write_all(&json).context("write preferences")?;
        temporary
            .as_file()
            .sync_all()
            .context("flush preferences")?;
        temporary
            .persist(&self.path)
            .with_context(|| format!("replace {}", self.path.display()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{AppSettings, SettingsStore};
    use crate::app::theme::ThemeMode;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn settings_round_trip() {
        let directory = tempdir().unwrap();
        let root = directory.path();
        let store = SettingsStore::at(root.join("settings.json"));
        let expected = AppSettings {
            theme: ThemeMode::Light,
            compact_sidebar: true,
        };

        store.save(&AppSettings::default()).unwrap();
        store.save(&expected).unwrap();
        assert_eq!(store.load().unwrap(), expected);
        assert_eq!(fs::read_dir(root).unwrap().count(), 1);
    }

    #[test]
    fn missing_preferences_use_defaults_without_warning() {
        let directory = tempdir().unwrap();
        let store = SettingsStore::at(directory.path().join("missing/settings.json"));
        assert_eq!(store.load_or_default(), (AppSettings::default(), None));
    }

    #[test]
    fn older_preferences_keep_new_defaults_and_ignore_removed_fields() {
        let settings: AppSettings =
            serde_json::from_str(r#"{"theme":"light","animations":false}"#).unwrap();
        assert_eq!(settings.theme, ThemeMode::Light);
        assert!(!settings.compact_sidebar);
    }

    #[test]
    fn failed_replace_cleans_up_the_temporary_file() {
        let directory = tempdir().unwrap();
        let destination = directory.path().join("settings.json");
        fs::create_dir(&destination).unwrap();
        let store = SettingsStore::at(destination.clone());
        assert!(store.save(&AppSettings::default()).is_err());
        assert!(destination.is_dir());
        assert_eq!(fs::read_dir(directory.path()).unwrap().count(), 1);
    }

    #[test]
    fn malformed_settings_fall_back_with_a_warning() {
        let directory = tempdir().unwrap();
        let root = directory.path();
        fs::write(root.join("settings.json"), b"not json").unwrap();
        let store = SettingsStore::at(root.join("settings.json"));

        let (settings, warning) = store.load_or_default();

        assert_eq!(settings, AppSettings::default());
        assert!(warning.is_some());
        assert_eq!(fs::read(root.join("settings.json")).unwrap(), b"not json");
    }
}
