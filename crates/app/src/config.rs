use std::{fs, io, path::PathBuf};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub server_url: String,
    pub username: String,
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
