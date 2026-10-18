pub mod linux;
pub mod macos;
pub mod traits;
pub mod windows;

pub use traits::BrowserDiscoverer;

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
}
