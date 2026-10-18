use crate::domain::models::BrowserTarget;
use crate::platform::traits::{
    BrowserDiscoverer, HandlerStatus, ProtocolHandler, RegistrationResult,
};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Default)]
pub struct MacOsBrowserDiscoverer;

impl MacOsBrowserDiscoverer {
    pub fn new() -> Self {
        Self
    }
}

impl BrowserDiscoverer for MacOsBrowserDiscoverer {
    fn discover(&self) -> Vec<BrowserTarget> {
        let mut discovered = Vec::new();
        let mut priority = 1;

        let candidate_dirs = [
            PathBuf::from("/Applications"),
            dirs::home_dir()
                .map(|h| h.join("Applications"))
                .unwrap_or_default(),
        ];

        let browser_specs = &[
            ("safari", "Safari", "Safari.app"),
            ("chrome", "Google Chrome", "Google Chrome.app"),
            ("firefox", "Firefox", "Firefox.app"),
            ("arc", "Arc", "Arc.app"),
            ("brave", "Brave Browser", "Brave Browser.app"),
            ("edge", "Microsoft Edge", "Microsoft Edge.app"),
            ("opera", "Opera", "Opera.app"),
            ("vivaldi", "Vivaldi", "Vivaldi.app"),
            ("zen", "Zen Browser", "Zen Browser.app"),
            ("tor", "Tor Browser", "Tor Browser.app"),
        ];

        for (id, name, app_name) in browser_specs {
            for dir in &candidate_dirs {
                let app_path = dir.join(app_name);
                if app_path.exists() {
                    discovered.push(BrowserTarget {
                        id: (*id).to_string(),
                        name: (*name).to_string(),
                        executable_path: app_path.to_string_lossy().to_string(),
                        arguments: None,
                        icon_path: None,
                        alternate_profiles: vec![],
                        enabled: true,
                        priority,
                    });
                    priority += 1;
                    break;
                }
            }
        }

        discovered
    }
}

#[derive(Default)]
pub struct MacOsProtocolHandler;

impl MacOsProtocolHandler {
    pub fn new() -> Self {
        Self
    }
}

impl ProtocolHandler for MacOsProtocolHandler {
    fn check_status(&self) -> HandlerStatus {
        let mut current_handler = None;
        let mut is_default = false;

        // Query default handler for http scheme via macOS defaults
        if let Ok(output) = Command::new("defaults")
            .args([
                "read",
                "com.apple.LaunchServices/com.apple.launchservices.secure",
                "LSHandlers",
            ])
            .output()
        {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if stdout.contains("openin") || stdout.contains("open-in") {
                    is_default = true;
                    current_handler = Some("OpenIn".to_string());
                }
            }
        }

        let details = if is_default {
            "OpenIn is verified as the default macOS web link handler.".to_string()
        } else {
            "macOS requires confirming default browser selection in System Settings.".to_string()
        };

        HandlerStatus {
            is_default,
            current_handler,
            os_type: "macos".to_string(),
            can_auto_register: false,
            details,
        }
    }

    fn register_as_default(&self) -> RegistrationResult {
        let _ = self.open_system_settings();
        RegistrationResult {
            success: false,
            message: "macOS requires user confirmation in System Settings > Desktop & Dock > Default web browser.".to_string(),
            verified: false,
        }
    }

    fn open_system_settings(&self) -> Result<(), String> {
        Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.general")
            .spawn()
            .map(|_| ())
            .map_err(|e| format!("Failed to open macOS System Settings: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macos_discoverer_creation() {
        let discoverer = MacOsBrowserDiscoverer::new();
        let browsers = discoverer.discover();
        for b in browsers {
            assert!(Path::new(&b.executable_path).exists());
        }
    }

    #[test]
    fn test_macos_protocol_handler_creation() {
        let handler = MacOsProtocolHandler::new();
        let status = handler.check_status();
        assert_eq!(status.os_type, "macos");
        assert!(!status.can_auto_register);
    }
}
