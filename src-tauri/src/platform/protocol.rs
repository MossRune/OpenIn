use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

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

pub struct ProtocolService;

impl ProtocolService {
    pub fn check_status() -> HandlerStatus {
        #[cfg(target_os = "linux")]
        {
            Self::check_status_linux()
        }

        #[cfg(target_os = "macos")]
        {
            Self::check_status_macos()
        }

        #[cfg(target_os = "windows")]
        {
            Self::check_status_windows()
        }

        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            Self::check_status_linux()
        }
    }

    pub fn register_as_default() -> RegistrationResult {
        #[cfg(target_os = "linux")]
        {
            Self::register_linux()
        }

        #[cfg(target_os = "macos")]
        {
            Self::register_macos()
        }

        #[cfg(target_os = "windows")]
        {
            Self::register_windows()
        }

        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            Self::register_linux()
        }
    }

    pub fn open_system_settings() -> Result<(), String> {
        #[cfg(target_os = "linux")]
        {
            // Try standard desktop settings
            if Command::new("gnome-control-center")
                .arg("default-apps")
                .spawn()
                .is_ok()
            {
                return Ok(());
            }
            if Command::new("kcmshell6").arg("defaultapps").spawn().is_ok() {
                return Ok(());
            }
            if Command::new("kcmshell5")
                .arg("kcm_componentchooser")
                .spawn()
                .is_ok()
            {
                return Ok(());
            }
            Err("No supported desktop settings control center found".to_string())
        }

        #[cfg(target_os = "macos")]
        {
            let _ = Command::new("open")
                .arg("x-apple.systempreferences:com.apple.preference.general")
                .spawn();
            Ok(())
        }

        #[cfg(target_os = "windows")]
        {
            let _ = Command::new("cmd")
                .args(["/c", "start", "ms-settings:defaultapps"])
                .spawn();
            Ok(())
        }

        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            Err("Unsupported operating system for settings shortcut".to_string())
        }
    }

    // --- Linux implementation ---

    fn get_linux_mimeapps_path() -> Option<PathBuf> {
        if let Ok(config_home) = std::env::var("XDG_CONFIG_HOME") {
            let path = PathBuf::from(config_home).join("mimeapps.list");
            return Some(path);
        }
        dirs::home_dir().map(|h| h.join(".config/mimeapps.list"))
    }

    fn check_status_linux() -> HandlerStatus {
        let mut current_handler = None;
        let mut is_default = false;

        // 1. Check via gio if available
        if let Ok(output) = Command::new("gio")
            .args(["mime", "x-scheme-handler/http"])
            .output()
        {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    let line = line.trim();
                    if line.starts_with("Default application for")
                        || line.starts_with("Recommended application")
                    {
                        if let Some((_, app)) = line.split_once(':') {
                            let app_name = app.trim().trim_matches('“').trim_matches('”');
                            if !app_name.is_empty() {
                                current_handler = Some(app_name.to_string());
                                if app_name.contains("openin") || app_name.contains("open-in") {
                                    is_default = true;
                                }
                            }
                        }
                    }
                }
            }
        }

        // 2. Check ~/.config/mimeapps.list directly (FreeDesktop specification)
        if !is_default {
            if let Some(mime_path) = Self::get_linux_mimeapps_path() {
                if mime_path.exists() {
                    if let Ok(content) = fs::read_to_string(&mime_path) {
                        let mut in_default_section = false;
                        for line in content.lines() {
                            let line = line.trim();
                            if line.starts_with('[') && line.ends_with(']') {
                                in_default_section = line == "[Default Applications]";
                                continue;
                            }
                            if in_default_section {
                                if let Some((key, val)) = line.split_once('=') {
                                    let key = key.trim();
                                    let val = val.trim().trim_end_matches(';');
                                    if key == "x-scheme-handler/http"
                                        || key == "x-scheme-handler/https"
                                    {
                                        current_handler = Some(val.to_string());
                                        if val.contains("openin") || val.contains("open-in") {
                                            is_default = true;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // 3. Fallback check via xdg-settings if available
        if !is_default {
            if let Ok(output) = Command::new("xdg-settings")
                .args(["check", "default-web-browser", "openin.desktop"])
                .output()
            {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout)
                        .trim()
                        .to_lowercase();
                    if stdout == "yes" {
                        is_default = true;
                        current_handler = Some("openin.desktop".to_string());
                    }
                }
            }
        }

        let details = if is_default {
            "OpenIn is verified as the system default web link handler.".to_string()
        } else if let Some(ref h) = current_handler {
            format!("Current default browser handler is '{}'.", h)
        } else {
            "No default handler currently registered for web links.".to_string()
        };

        HandlerStatus {
            is_default,
            current_handler,
            os_type: "linux".to_string(),
            can_auto_register: true,
            details,
        }
    }

    fn register_linux() -> RegistrationResult {
        // Step 1: Ensure openin.desktop exists in user applications directory
        let apps_dir = match dirs::data_local_dir() {
            Some(d) => d.join("applications"),
            None => match dirs::home_dir() {
                Some(h) => h.join(".local/share/applications"),
                None => PathBuf::from("/tmp/applications"),
            },
        };

        if let Err(e) = fs::create_dir_all(&apps_dir) {
            return RegistrationResult {
                success: false,
                message: format!("Failed to create applications directory: {}", e),
                verified: false,
            };
        }

        let desktop_file_path = apps_dir.join("openin.desktop");
        let current_exe = std::env::current_exe()
            .ok()
            .and_then(|p| p.to_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "openin".to_string());

        let desktop_content = format!(
            "[Desktop Entry]\n\
Type=Application\n\
Name=OpenIn\n\
Comment=Lightweight URL router and browser chooser\n\
Exec={} %u\n\
Icon=openin\n\
Terminal=false\n\
Categories=Network;WebBrowser;\n\
MimeType=x-scheme-handler/http;x-scheme-handler/https;\n\
StartupWMClass=openin\n",
            current_exe
        );

        if let Err(e) = fs::write(&desktop_file_path, desktop_content) {
            return RegistrationResult {
                success: false,
                message: format!("Failed to write openin.desktop entry: {}", e),
                verified: false,
            };
        }

        // Step 2: Update ~/.config/mimeapps.list
        if let Some(mime_path) = Self::get_linux_mimeapps_path() {
            let _ = Self::update_mimeapps_list(&mime_path, "openin.desktop");
        }

        // Step 3: Run gio mime if available
        let _ = Command::new("gio")
            .args(["mime", "x-scheme-handler/http", "openin.desktop"])
            .output();
        let _ = Command::new("gio")
            .args(["mime", "x-scheme-handler/https", "openin.desktop"])
            .output();

        // Step 4: Run xdg-settings / xdg-mime if available
        let _ = Command::new("xdg-settings")
            .args(["set", "default-web-browser", "openin.desktop"])
            .output();
        let _ = Command::new("xdg-mime")
            .args(["default", "openin.desktop", "x-scheme-handler/http"])
            .output();
        let _ = Command::new("xdg-mime")
            .args(["default", "openin.desktop", "x-scheme-handler/https"])
            .output();

        // Step 5: Verify the resulting state explicitly
        let verification = Self::check_status_linux();
        if verification.is_default {
            RegistrationResult {
                success: true,
                message: "OpenIn successfully registered and verified as the default browser."
                    .to_string(),
                verified: true,
            }
        } else {
            RegistrationResult {
                success: false,
                message: format!(
                    "Desktop entry installed, but system verification returned false. Current handler: {}",
                    verification.current_handler.as_deref().unwrap_or("unknown")
                ),
                verified: false,
            }
        }
    }

    pub fn update_mimeapps_list(path: &Path, desktop_filename: &str) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = if path.exists() {
            fs::read_to_string(path)?
        } else {
            String::new()
        };

        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        let mut default_section_idx = None;

        for (idx, line) in lines.iter().enumerate() {
            if line.trim() == "[Default Applications]" {
                default_section_idx = Some(idx);
                break;
            }
        }

        let http_entry = format!("x-scheme-handler/http={}", desktop_filename);
        let https_entry = format!("x-scheme-handler/https={}", desktop_filename);

        if let Some(sec_idx) = default_section_idx {
            // Remove existing http/https entries in Default Applications section
            let mut insert_idx = sec_idx + 1;
            let mut i = sec_idx + 1;
            while i < lines.len() {
                let trimmed = lines[i].trim();
                if trimmed.starts_with('[') && trimmed.ends_with(']') {
                    break;
                }
                if trimmed.starts_with("x-scheme-handler/http=")
                    || trimmed.starts_with("x-scheme-handler/https=")
                {
                    lines.remove(i);
                    continue;
                }
                i += 1;
            }
            lines.insert(insert_idx, http_entry);
            insert_idx += 1;
            lines.insert(insert_idx, https_entry);
        } else {
            lines.push("[Default Applications]".to_string());
            lines.push(http_entry);
            lines.push(https_entry);
        }

        fs::write(path, lines.join("\n") + "\n")
    }

    // --- macOS stub / implementation ---

    #[allow(dead_code)]
    fn check_status_macos() -> HandlerStatus {
        HandlerStatus {
            is_default: false,
            current_handler: None,
            os_type: "macos".to_string(),
            can_auto_register: false,
            details: "macOS requires confirming default browser selection in System Settings."
                .to_string(),
        }
    }

    #[allow(dead_code)]
    fn register_macos() -> RegistrationResult {
        let _ = Self::open_system_settings();
        RegistrationResult {
            success: false,
            message: "macOS requires user confirmation in System Settings > Desktop & Dock > Default web browser.".to_string(),
            verified: false,
        }
    }

    // --- Windows stub / implementation ---

    #[allow(dead_code)]
    fn check_status_windows() -> HandlerStatus {
        HandlerStatus {
            is_default: false,
            current_handler: None,
            os_type: "windows".to_string(),
            can_auto_register: false,
            details: "Windows requires user confirmation in Windows Settings > Default Apps."
                .to_string(),
        }
    }

    #[allow(dead_code)]
    fn register_windows() -> RegistrationResult {
        let _ = Self::open_system_settings();
        RegistrationResult {
            success: false,
            message: "Windows requires selecting OpenIn in Windows Settings > Default Apps."
                .to_string(),
            verified: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_mimeapps_update_creates_default_section() {
        let dir = tempdir().unwrap();
        let mime_path = dir.path().join("mimeapps.list");

        ProtocolService::update_mimeapps_list(&mime_path, "openin.desktop").unwrap();

        let content = fs::read_to_string(&mime_path).unwrap();
        assert!(content.contains("[Default Applications]"));
        assert!(content.contains("x-scheme-handler/http=openin.desktop"));
        assert!(content.contains("x-scheme-handler/https=openin.desktop"));
    }

    #[test]
    fn test_mimeapps_update_replaces_existing_entry() {
        let dir = tempdir().unwrap();
        let mime_path = dir.path().join("mimeapps.list");

        let initial = "[Default Applications]\nx-scheme-handler/http=firefox.desktop\n";
        fs::write(&mime_path, initial).unwrap();

        ProtocolService::update_mimeapps_list(&mime_path, "openin.desktop").unwrap();

        let content = fs::read_to_string(&mime_path).unwrap();
        assert!(content.contains("x-scheme-handler/http=openin.desktop"));
        assert!(!content.contains("firefox.desktop"));
    }
}
