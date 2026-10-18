use crate::domain::errors::SettingsError;
use crate::domain::models::Configuration;
use crate::infrastructure::traits::SettingsStore;
use std::sync::Arc;

pub struct SettingsService {
    store: Arc<dyn SettingsStore>,
}

impl SettingsService {
    pub fn new(store: Arc<dyn SettingsStore>) -> Self {
        Self { store }
    }

    pub fn load_configuration(&self) -> Result<Configuration, SettingsError> {
        self.store.load()
    }

    pub fn save_configuration(&self, config: &Configuration) -> Result<(), SettingsError> {
        self.store.save(config)
    }
}
