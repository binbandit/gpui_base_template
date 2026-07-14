//! Persistent user preferences and their filesystem boundary.

use crate::app::theme::ThemeMode;
use anyhow::{Context as _, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write as _;
use std::path::{Path, PathBuf};

/// Preferences that survive restarts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct AppSettings {
    pub theme: ThemeMode,
    pub animations: bool,
    pub compact_sidebar: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: ThemeMode::Dark,
            animations: true,
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
    fn at(path: PathBuf) -> Self {
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
                Some(format!("Settings were reset: {error:#}")),
            ),
        }
    }

    pub fn load(&self) -> Result<AppSettings> {
        #[cfg(target_os = "windows")]
        recover_interrupted_replace(&self.path)?;

        let bytes =
            fs::read(&self.path).with_context(|| format!("read {}", self.path.display()))?;
        serde_json::from_slice(&bytes).with_context(|| format!("parse {}", self.path.display()))
    }

    pub fn save(&self, settings: &AppSettings) -> Result<()> {
        let parent = self.path.parent().context("settings path has no parent")?;
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;

        let json = serde_json::to_vec_pretty(settings).context("serialize settings")?;
        write_atomically(&self.path, &json)
    }
}

fn write_atomically(path: &Path, bytes: &[u8]) -> Result<()> {
    let temporary = path.with_extension("json.tmp");
    let mut file =
        File::create(&temporary).with_context(|| format!("create {}", temporary.display()))?;
    file.write_all(bytes)
        .with_context(|| format!("write {}", temporary.display()))?;
    file.sync_all()
        .with_context(|| format!("flush {}", temporary.display()))?;
    drop(file);

    replace_file(&temporary, path)
}

#[cfg(not(target_os = "windows"))]
fn replace_file(temporary: &Path, destination: &Path) -> Result<()> {
    // POSIX rename replaces the destination atomically, so the previous valid
    // settings file remains intact until the new one is ready.
    fs::rename(temporary, destination).with_context(|| format!("commit {}", destination.display()))
}

#[cfg(target_os = "windows")]
fn recover_interrupted_replace(destination: &Path) -> Result<()> {
    let backup = destination.with_extension("json.previous");
    if !destination.exists() && backup.exists() {
        fs::rename(&backup, destination)
            .with_context(|| format!("recover {}", destination.display()))?;
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn replace_file(temporary: &Path, destination: &Path) -> Result<()> {
    // `std::fs::rename` cannot replace an existing Windows file. Preserve the
    // last valid copy and roll it back if committing the replacement fails.
    let backup = destination.with_extension("json.previous");
    let had_destination = destination.exists();

    if had_destination {
        let _ = fs::remove_file(&backup);
        fs::rename(destination, &backup)
            .with_context(|| format!("preserve {}", destination.display()))?;
    }

    match fs::rename(temporary, destination) {
        Ok(()) => {
            if had_destination {
                let _ = fs::remove_file(backup);
            }
            Ok(())
        }
        Err(error) => {
            if had_destination {
                let _ = fs::rename(&backup, destination);
            }
            Err(error).with_context(|| format!("commit {}", destination.display()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AppSettings, SettingsStore};
    use crate::app::theme::ThemeMode;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn test_path(name: &str) -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("gpui-base-{name}-{}-{nonce}", std::process::id()))
    }

    #[test]
    fn settings_round_trip() {
        let root = test_path("round-trip");
        let store = SettingsStore::at(root.join("settings.json"));
        let expected = AppSettings {
            theme: ThemeMode::Light,
            animations: false,
            compact_sidebar: true,
        };

        store.save(&AppSettings::default()).unwrap();
        store.save(&expected).unwrap();
        assert_eq!(store.load().unwrap(), expected);
        assert!(!root.join("settings.json.tmp").exists());
        assert!(!root.join("settings.json.previous").exists());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn malformed_settings_fall_back_with_a_warning() {
        let root = test_path("malformed");
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join("settings.json"), b"not json").unwrap();
        let store = SettingsStore::at(root.join("settings.json"));

        let (settings, warning) = store.load_or_default();

        assert_eq!(settings, AppSettings::default());
        assert!(warning.is_some());
        let _ = fs::remove_dir_all(root);
    }
}
