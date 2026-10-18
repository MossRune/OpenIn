use crate::domain::errors::SettingsError;
use crate::domain::models::Configuration;
use crate::infrastructure::traits::SettingsStore;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct FileSettingsStore {
    path: PathBuf,
}

impl FileSettingsStore {
    pub fn new() -> Result<Self, SettingsError> {
        let base_dir = dirs::config_dir()
            .or_else(dirs::home_dir)
            .ok_or(SettingsError::NoConfigDirectory)?;
        let config_dir = base_dir.join("openin");
        let path = config_dir.join("settings.json");
        Ok(Self { path })
    }

    pub fn with_path(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn load(&self) -> Result<Configuration, SettingsError> {
        <Self as SettingsStore>::load(self)
    }

    pub fn save(&self, config: &Configuration) -> Result<(), SettingsError> {
        <Self as SettingsStore>::save(self, config)
    }
}

impl SettingsStore for FileSettingsStore {
    fn load(&self) -> Result<Configuration, SettingsError> {
        if !self.path.exists() {
            let default_config = Configuration::default();
            self.save(&default_config)?;
            return Ok(default_config);
        }

        let content =
            fs::read_to_string(&self.path).map_err(|e| SettingsError::Io(e.to_string()))?;
        match serde_json::from_str::<Configuration>(&content) {
            Ok(config) => Ok(config),
            Err(e) => {
                let backup_path = self.path.with_extension("corrupt.json");
                let _ = fs::rename(&self.path, &backup_path);

                let default_config = Configuration::default();
                let _ = self.save(&default_config);
                Err(SettingsError::Serialization(e.to_string()))
            }
        }
    }

    fn save(&self, config: &Configuration) -> Result<(), SettingsError> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(|e| SettingsError::Io(e.to_string()))?;
        }

        let serialized = serde_json::to_string_pretty(config)
            .map_err(|e| SettingsError::Serialization(e.to_string()))?;

        let temp_path = self.path.with_extension("tmp");

        {
            let mut file =
                File::create(&temp_path).map_err(|e| SettingsError::Io(e.to_string()))?;
            file.write_all(serialized.as_bytes())
                .map_err(|e| SettingsError::Io(e.to_string()))?;
            file.sync_all()
                .map_err(|e| SettingsError::Io(e.to_string()))?;
        }

        fs::rename(&temp_path, &self.path).map_err(|e| SettingsError::Io(e.to_string()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_file_store_cycle() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("config.json");
        let store = FileSettingsStore::with_path(path);

        let mut config = Configuration::default();
        config.preferences.theme = "dark".into();

        store.save(&config).unwrap();
        let loaded = store.load().unwrap();
        assert_eq!(loaded.preferences.theme, "dark");
    }
}
