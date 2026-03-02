pub mod file;
pub mod profiles;

use crate::error::{Result, XeroCliError};
use std::path::PathBuf;

pub use file::ConfigFile;
pub use profiles::Profile;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub config_file: ConfigFile,
    pub config_path: PathBuf,
}

impl AppConfig {
    pub fn load(config_path: Option<&str>) -> Result<Self> {
        let path = match config_path {
            Some(p) => PathBuf::from(p),
            None => default_config_path()?,
        };

        let config_file = if path.exists() {
            let content = std::fs::read_to_string(&path)
                .map_err(|e| XeroCliError::config(format!("Failed to read config file: {e}")))?;
            toml::from_str(&content)?
        } else {
            ConfigFile::default()
        };

        Ok(Self {
            config_file,
            config_path: path,
        })
    }

    pub fn active_profile(&self, name: Option<&str>) -> Option<&Profile> {
        let profile_name = name
            .or(self.config_file.default.active_profile.as_deref())
            .unwrap_or("default");
        self.config_file.profiles.get(profile_name)
    }

    pub fn save(&self) -> Result<()> {
        if let Some(parent) = self.config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(&self.config_file)
            .map_err(|e| XeroCliError::config(format!("Failed to serialize config: {e}")))?;
        std::fs::write(&self.config_path, content)?;
        Ok(())
    }
}

pub fn default_config_dir() -> Result<PathBuf> {
    dirs::config_dir()
        .map(|d| d.join("xero-cli"))
        .ok_or_else(|| XeroCliError::config("Could not determine config directory"))
}

pub fn default_config_path() -> Result<PathBuf> {
    default_config_dir().map(|d| d.join("config.toml"))
}

pub fn default_cache_dir() -> Result<PathBuf> {
    dirs::cache_dir()
        .map(|d| d.join("xero-cli"))
        .ok_or_else(|| XeroCliError::config("Could not determine cache directory"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_missing_config_returns_defaults() {
        let config = AppConfig::load(Some("/tmp/nonexistent-xero-config.toml")).unwrap();
        assert!(config.config_file.profiles.is_empty());
    }

    #[test]
    fn active_profile_returns_none_when_empty() {
        let config = AppConfig::load(Some("/tmp/nonexistent-xero-config.toml")).unwrap();
        assert!(config.active_profile(None).is_none());
    }
}
