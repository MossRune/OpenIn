use crate::domain::errors::{LaunchError, SettingsError};
use crate::domain::models::Configuration;

pub trait SettingsStore: Send + Sync {
    fn load(&self) -> Result<Configuration, SettingsError>;
    fn save(&self, config: &Configuration) -> Result<(), SettingsError>;
}

pub trait ProcessRunner: Send + Sync {
    fn run(&self, executable: &str, arguments: &[String]) -> Result<(), LaunchError>;
}
