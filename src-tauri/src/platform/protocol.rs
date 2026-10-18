use crate::platform::traits::{HandlerStatus, ProtocolHandler, RegistrationResult};
use std::sync::Arc;

pub fn create_default_protocol_handler() -> Arc<dyn ProtocolHandler> {
    #[cfg(target_os = "linux")]
    {
        Arc::new(crate::platform::linux::LinuxProtocolHandler::new())
    }

    #[cfg(target_os = "macos")]
    {
        Arc::new(crate::platform::macos::MacOsProtocolHandler::new())
    }

    #[cfg(target_os = "windows")]
    {
        Arc::new(crate::platform::windows::WindowsProtocolHandler::new())
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        Arc::new(crate::platform::linux::LinuxProtocolHandler::new())
    }
}

pub struct ProtocolService;

impl ProtocolService {
    pub fn check_status() -> HandlerStatus {
        create_default_protocol_handler().check_status()
    }

    pub fn register_as_default() -> RegistrationResult {
        create_default_protocol_handler().register_as_default()
    }

    pub fn open_system_settings() -> Result<(), String> {
        create_default_protocol_handler().open_system_settings()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_service_smoke() {
        let status = ProtocolService::check_status();
        assert!(!status.os_type.is_empty());
    }
}
