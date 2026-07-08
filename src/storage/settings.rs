use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Context;

use crate::models::AppSettings;

pub fn default_data_dir() -> anyhow::Result<PathBuf> {
    let dirs = directories::ProjectDirs::from("dev", "playfairs", "xtm")
        .context("failed to resolve application data directory")?;
    Ok(dirs.data_dir().to_path_buf())
}

pub fn settings_path(dir: &Path) -> PathBuf {
    dir.join("settings.json")
}

pub fn load_settings_from_dir(dir: &Path) -> anyhow::Result<AppSettings> {
    let path = settings_path(dir);
    if !path.exists() {
        return Ok(AppSettings::default());
    }

    let content = fs::read_to_string(&path).with_context(|| format!("failed to read settings from {}", path.display()))?;
    serde_json::from_str(&content).with_context(|| format!("failed to parse settings from {}", path.display()))
}

pub fn save_settings_to_dir(settings: &AppSettings, dir: &Path) -> anyhow::Result<()> {
    fs::create_dir_all(dir).with_context(|| format!("failed to create settings directory {}", dir.display()))?;
    let path = settings_path(dir);
    let content = serde_json::to_string_pretty(settings)?;
    fs::write(&path, content).with_context(|| format!("failed to write settings to {}", path.display()))?;
    Ok(())
}

pub fn load_settings() -> anyhow::Result<AppSettings> {
    let dir = default_data_dir()?;
    load_settings_from_dir(&dir)
}

pub fn save_settings(settings: &AppSettings) -> anyhow::Result<()> {
    let dir = default_data_dir()?;
    save_settings_to_dir(settings, &dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("xtm-settings-test-{nanos}"))
    }

    #[test]
    fn settings_round_trip() {
        let dir = temp_dir();
        fs::create_dir_all(&dir).expect("temp dir should be created");

        let settings = AppSettings {
            theme: crate::theme::ThemeMode::Light,
            ui_scale: 1.15,
            notifications_enabled: true,
            auto_refresh: true,
            compact_mode: true,
            selected_token: Some("NovaByte".to_string()),
        };

        super::save_settings_to_dir(&settings, &dir).expect("settings should be saved");
        let loaded = super::load_settings_from_dir(&dir).expect("settings should be loaded");

        assert_eq!(loaded.theme, settings.theme);
        assert_eq!(loaded.ui_scale, settings.ui_scale);
        assert_eq!(loaded.selected_token, settings.selected_token);

        let _ = fs::remove_dir_all(dir);
    }
}
