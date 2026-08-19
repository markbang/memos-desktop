use std::{fs, io, path::PathBuf};

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
        let Some(path) = config_path() else {
            return Self::default();
        };
        let Ok(content) = fs::read_to_string(path) else {
            return Self::default();
        };
        serde_json::from_str(&content).unwrap_or_default()
    }

    pub fn save(&self) -> io::Result<()> {
        let Some(path) = config_path() else {
            return Ok(());
        };
        let Some(parent) = path.parent() else {
            return Ok(());
        };
        fs::create_dir_all(parent)?;
        let temporary = path.with_extension("json.tmp");
        let content = serde_json::to_vec_pretty(self).map_err(io::Error::other)?;
        fs::write(&temporary, content)?;
        fs::rename(temporary, path)
    }
}

fn config_path() -> Option<PathBuf> {
    ProjectDirs::from("com", "Memos Desktop", "Memos Desktop")
        .map(|directories| directories.config_dir().join("config.json"))
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
}
