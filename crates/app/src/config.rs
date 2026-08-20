use std::{
    fs, io,
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::theme::ThemePreference;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub server_url: String,
    pub username: String,
    pub auto_login: bool,
    pub theme: ThemePreference,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server_url: String::new(),
            username: String::new(),
            auto_login: false,
            theme: ThemePreference::System,
        }
    }
}

impl AppConfig {
    pub fn load() -> Self {
        config_path()
            .as_deref()
            .map(Self::load_from_path)
            .unwrap_or_default()
    }

    pub fn save(&self) -> io::Result<()> {
        let Some(path) = config_path() else {
            return Ok(());
        };
        self.save_to_path(&path)
    }

    fn load_from_path(path: &Path) -> Self {
        let content = match fs::read_to_string(path) {
            Ok(content) => Some(content),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                fs::read_to_string(backup_path(path)).ok()
            }
            Err(_) => None,
        };
        content
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default()
    }

    fn save_to_path(&self, path: &Path) -> io::Result<()> {
        let Some(parent) = path.parent() else {
            return Ok(());
        };
        fs::create_dir_all(parent)?;
        let temporary = path.with_extension(format!("json.tmp-{}", rand::random::<u64>()));
        let content = serde_json::to_vec_pretty(self).map_err(io::Error::other)?;
        if let Err(error) = fs::write(&temporary, content) {
            _ = fs::remove_file(&temporary);
            return Err(error);
        }
        replace_config_file(&temporary, path)
    }
}

fn config_path() -> Option<PathBuf> {
    ProjectDirs::from("com", "Memos Desktop", "Memos Desktop")
        .map(|directories| directories.config_dir().join("config.json"))
}

fn backup_path(path: &Path) -> PathBuf {
    path.with_extension("json.previous")
}

fn replace_config_file(temporary: &Path, destination: &Path) -> io::Result<()> {
    match fs::rename(temporary, destination) {
        Ok(()) => {
            _ = fs::remove_file(backup_path(destination));
            return Ok(());
        }
        Err(error) if !destination.is_file() => {
            _ = fs::remove_file(temporary);
            return Err(error);
        }
        Err(_) => {}
    }

    let backup = backup_path(destination);
    if let Err(error) = fs::remove_file(&backup)
        && error.kind() != io::ErrorKind::NotFound
    {
        _ = fs::remove_file(temporary);
        return Err(error);
    }
    if let Err(error) = fs::rename(destination, &backup) {
        _ = fs::remove_file(temporary);
        return Err(error);
    }
    if let Err(error) = fs::rename(temporary, destination) {
        _ = fs::remove_file(temporary);
        if !destination.exists() {
            _ = fs::rename(&backup, destination);
        }
        return Err(error);
    }
    _ = fs::remove_file(backup);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn old_config_defaults_to_system_theme_without_assuming_saved_credentials() {
        let config: AppConfig = serde_json::from_str(
            r#"{"server_url":"https://memos.example.com","username":"alice"}"#,
        )
        .unwrap();
        assert!(!config.auto_login);
        assert_eq!(config.theme, ThemePreference::System);
    }

    #[test]
    fn save_replaces_an_existing_config() {
        let root =
            std::env::temp_dir().join(format!("memos-config-test-{}", rand::random::<u64>()));
        let path = root.join("config.json");
        let first = AppConfig {
            server_url: "https://one.example.com".into(),
            username: "alice".into(),
            auto_login: true,
            theme: ThemePreference::Light,
        };
        first.save_to_path(&path).unwrap();

        let second = AppConfig {
            server_url: "https://two.example.com".into(),
            username: "bob".into(),
            auto_login: false,
            theme: ThemePreference::Dark,
        };
        second.save_to_path(&path).unwrap();

        let loaded = AppConfig::load_from_path(&path);
        assert_eq!(loaded.server_url, second.server_url);
        assert_eq!(loaded.username, second.username);
        assert_eq!(loaded.auto_login, second.auto_login);
        assert_eq!(loaded.theme, second.theme);
        assert!(!backup_path(&path).exists());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn load_recovers_the_backup_when_replacement_was_interrupted() {
        let root =
            std::env::temp_dir().join(format!("memos-config-test-{}", rand::random::<u64>()));
        let path = root.join("config.json");
        let config = AppConfig {
            server_url: "https://memos.example.com".into(),
            username: "alice".into(),
            auto_login: true,
            theme: ThemePreference::System,
        };
        fs::create_dir_all(&root).unwrap();
        fs::write(
            backup_path(&path),
            serde_json::to_vec_pretty(&config).unwrap(),
        )
        .unwrap();

        let loaded = AppConfig::load_from_path(&path);
        assert_eq!(loaded.server_url, config.server_url);
        assert_eq!(loaded.username, config.username);
        assert!(loaded.auto_login);
        fs::remove_dir_all(root).unwrap();
    }
}
