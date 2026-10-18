use crate::domain::models::BrowserTarget;
use crate::platform::traits::{
    BrowserDiscoverer, HandlerStatus, ProtocolHandler, RegistrationResult,
};
use std::env;
use std::path::Path;
use std::process::Command;

#[derive(Default)]
pub struct WindowsBrowserDiscoverer;

impl WindowsBrowserDiscoverer {
    pub fn new() -> Self {
        Self
    }
}

impl BrowserDiscoverer for WindowsBrowserDiscoverer {
    fn discover(&self) -> Vec<BrowserTarget> {
        let mut discovered = Vec::new();
        let mut priority = 1;

        let program_files =
            env::var("ProgramFiles").unwrap_or_else(|_| r"C:\Program Files".to_string());
        let program_files_x86 =
            env::var("ProgramFiles(x86)").unwrap_or_else(|_| r"C:\Program Files (x86)".to_string());
        let local_app_data = env::var("LOCALAPPDATA")
            .unwrap_or_else(|_| r"C:\Users\Default\AppData\Local".to_string());

        let candidates = &[
            (
                "msedge",
                "Microsoft Edge",
                vec![
                    format!(
                        r"{}\Microsoft\Edge\Application\msedge.exe",
                        program_files_x86
                    ),
                    format!(r"{}\Microsoft\Edge\Application\msedge.exe", program_files),
                ],
            ),
            (
                "chrome",
                "Google Chrome",
                vec![
                    format!(r"{}\Google\Chrome\Application\chrome.exe", program_files),
                    format!(
                        r"{}\Google\Chrome\Application\chrome.exe",
                        program_files_x86
                    ),
                    format!(r"{}\Google\Chrome\Application\chrome.exe", local_app_data),
                ],
            ),
            (
                "firefox",
                "Mozilla Firefox",
                vec![
                    format!(r"{}\Mozilla Firefox\firefox.exe", program_files),
                    format!(r"{}\Mozilla Firefox\firefox.exe", program_files_x86),
                ],
            ),
            (
                "brave",
                "Brave Browser",
                vec![
                    format!(
                        r"{}\BraveSoftware\Brave-Browser\Application\brave.exe",
                        program_files
                    ),
                    format!(
                        r"{}\BraveSoftware\Brave-Browser\Application\brave.exe",
                        local_app_data
                    ),
                ],
            ),
            (
                "arc",
                "Arc",
                vec![format!(r"{}\TheBrowserCompany\Arc\Arc.exe", local_app_data)],
            ),
        ];

        for (id, name, paths) in candidates {
            for path_str in paths {
                let path = Path::new(path_str);
                if path.exists() {
                    discovered.push(BrowserTarget {
                        id: (*id).to_string(),
                        name: (*name).to_string(),
                        executable_path: path_str.clone(),
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
pub struct WindowsProtocolHandler;

impl WindowsProtocolHandler {
    pub fn new() -> Self {
        Self
    }
}

impl ProtocolHandler for WindowsProtocolHandler {
    fn check_status(&self) -> HandlerStatus {
        let mut current_handler = None;
        let mut is_default = false;

        // Query Windows Registry UserChoice for http scheme via reg query
        if let Ok(output) = Command::new("reg")
            .args([
                "query",
                r"HKCU\Software\Microsoft\Windows\Shell\Associations\UrlAssociations\http\UserChoice",
                "/v",
                "ProgId",
            ])
            .output()
        {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    if line.contains("ProgId") {
                        if let Some((_, val)) = line.split_once("REG_SZ") {
                            let prog_id = val.trim();
                            current_handler = Some(prog_id.to_string());
                            if prog_id.eq_ignore_ascii_case("OpenIn")
                                || prog_id.to_lowercase().contains("openin")
                            {
                                is_default = true;
                            }
                        }
                    }
                }
            }
        }

        let details = if is_default {
            "OpenIn is verified as the default Windows web link handler.".to_string()
        } else if let Some(ref h) = current_handler {
            format!("Current Windows default handler ProgId is '{}'.", h)
        } else {
            "Windows requires selecting OpenIn in Windows Settings > Default Apps.".to_string()
        };

        HandlerStatus {
            is_default,
            current_handler,
            os_type: "windows".to_string(),
            can_auto_register: false,
            details,
        }
    }

    fn register_as_default(&self) -> RegistrationResult {
        let _ = self.open_system_settings();
        RegistrationResult {
            success: false,
            message: "Windows requires selecting OpenIn in Windows Settings > Default Apps."
                .to_string(),
            verified: false,
        }
    }

    fn open_system_settings(&self) -> Result<(), String> {
        Command::new("cmd")
            .args(["/c", "start", "ms-settings:defaultapps"])
            .spawn()
            .map(|_| ())
            .map_err(|e| format!("Failed to open Windows Default Apps settings: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_windows_discoverer_creation() {
        let discoverer = WindowsBrowserDiscoverer::new();
        let browsers = discoverer.discover();
        for b in browsers {
            assert!(Path::new(&b.executable_path).exists());
        }
    }

    #[test]
    fn test_windows_protocol_handler_creation() {
        let handler = WindowsProtocolHandler::new();
        let status = handler.check_status();
        assert_eq!(status.os_type, "windows");
        assert!(!status.can_auto_register);
    }
}
