use crate::domain::models::BrowserTarget;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HandlerStatus {
    pub is_default: bool,
    pub current_handler: Option<String>,
    pub os_type: String,
    pub can_auto_register: bool,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RegistrationResult {
    pub success: bool,
    pub message: String,
    pub verified: bool,
}

pub trait BrowserDiscoverer: Send + Sync {
    fn discover(&self) -> Vec<BrowserTarget>;
}

pub trait ProtocolHandler: Send + Sync {
    fn check_status(&self) -> HandlerStatus;
    fn register_as_default(&self) -> RegistrationResult;
    fn open_system_settings(&self) -> Result<(), String>;
}
