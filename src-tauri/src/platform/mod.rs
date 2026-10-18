pub mod traits;

#[cfg(any(
    target_os = "linux",
    not(any(target_os = "macos", target_os = "windows"))
))]
pub mod linux;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "windows")]
pub mod windows;

pub mod protocol;

pub use protocol::{create_default_protocol_handler, ProtocolService};
pub use traits::{BrowserDiscoverer, HandlerStatus, ProtocolHandler, RegistrationResult};

use crate::domain::models::BrowserTarget;
use std::sync::Arc;

pub fn create_default_discoverer() -> Arc<dyn BrowserDiscoverer> {
    #[cfg(target_os = "linux")]
    {
        Arc::new(linux::XdgBrowserDiscoverer::new())
    }
    #[cfg(target_os = "windows")]
    {
        Arc::new(windows::WindowsBrowserDiscoverer::new())
    }
    #[cfg(target_os = "macos")]
    {
        Arc::new(macos::MacOsBrowserDiscoverer::new())
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        Arc::new(linux::XdgBrowserDiscoverer::new())
    }
}

pub struct Platform;

impl Platform {
    pub fn discover_installed_browsers() -> Vec<BrowserTarget> {
        create_default_discoverer().discover()
    }

    pub fn check_handler_status() -> HandlerStatus {
        ProtocolService::check_status()
    }

    pub fn register_as_default_handler() -> RegistrationResult {
        ProtocolService::register_as_default()
    }

    pub fn open_system_settings() -> Result<(), String> {
        ProtocolService::open_system_settings()
    }
}
