pub use crate::domain::errors::SettingsError;
pub use crate::domain::models::{
    AlternateProfile as AlternateLaunch, BrowserTarget as Browser, Configuration as Settings,
    Preferences as AppSettings,
};
pub use crate::infrastructure::file_settings_store::FileSettingsStore as SettingsManager;
