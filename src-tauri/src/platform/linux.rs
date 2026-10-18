use crate::domain::models::BrowserTarget;
use crate::platform::traits::{
    BrowserDiscoverer, HandlerStatus, ProtocolHandler, RegistrationResult,
};
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Default)]
pub struct XdgBrowserDiscoverer;

impl XdgBrowserDiscoverer {
    pub fn new() -> Self {
        Self
    }

    /// Resolve all XDG application directories dynamically
    pub fn get_search_directories() -> Vec<PathBuf> {
        let mut dirs = Vec::new();

        // 1. User local applications ($XDG_DATA_HOME/applications)
        if let Ok(data_home) = env::var("XDG_DATA_HOME") {
            dirs.push(PathBuf::from(data_home).join("applications"));
        } else if let Some(home) = dirs::home_dir() {
            dirs.push(home.join(".local/share/applications"));
        }

        // 2. Flatpak user applications
        if let Some(home) = dirs::home_dir() {
            dirs.push(home.join(".local/share/flatpak/exports/share/applications"));
        }

        // 3. System data directories ($XDG_DATA_DIRS)
        if let Ok(data_dirs) = env::var("XDG_DATA_DIRS") {
            for dir in data_dirs.split(':') {
                if !dir.trim().is_empty() {
                    dirs.push(PathBuf::from(dir).join("applications"));
                }
            }
        } else {
            dirs.push(PathBuf::from("/usr/local/share/applications"));
            dirs.push(PathBuf::from("/usr/share/applications"));
        }

        // 4. System Flatpak & Snap paths
        dirs.push(PathBuf::from("/var/lib/flatpak/exports/share/applications"));
        dirs.push(PathBuf::from("/var/lib/snapd/desktop/applications"));

        dirs
    }

    pub fn parse_entry(path: &Path) -> Option<BrowserTarget> {
        let content = fs::read_to_string(path).ok()?;
        let mut in_main_section = false;
        let mut name = None;
        let mut exec = None;
        let mut icon = None;
        let mut categories = String::new();
        let mut mime_types = String::new();
        let mut no_display = false;
        let mut hidden = false;

        for line in content.lines() {
            let line = line.trim();
            if line.starts_with('[') && line.ends_with(']') {
                in_main_section = line == "[Desktop Entry]";
                continue;
            }

            if !in_main_section {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();
                match key {
                    "Name" if name.is_none() => name = Some(value.to_string()),
                    "Exec" if exec.is_none() => exec = Some(value.to_string()),
                    "Icon" if icon.is_none() => icon = Some(value.to_string()),
                    "Categories" => categories = value.to_lowercase(),
                    "MimeType" => mime_types = value.to_lowercase(),
                    "NoDisplay" => no_display = value.eq_ignore_ascii_case("true"),
                    "Hidden" => hidden = value.eq_ignore_ascii_case("true"),
                    _ => {}
                }
            }
        }

        if no_display || hidden {
            return None;
        }

        // Standard XDG check: Must be declared as WebBrowser or handle web schemes
        let is_browser = categories.contains("webbrowser")
            || mime_types.contains("x-scheme-handler/http")
            || mime_types.contains("x-scheme-handler/https");

        if !is_browser {
            return None;
        }

        let raw_exec = exec?;
        let cleaned_exec = Self::clean_exec_command(&raw_exec);
        let browser_name = name.unwrap_or_else(|| "Unknown Browser".to_string());
        let icon_path = icon.as_deref().and_then(Self::find_icon_asset);

        let id = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("browser")
            .to_string();

        Some(BrowserTarget {
            id,
            name: browser_name,
            executable_path: cleaned_exec,
            arguments: None,
            icon_path,
            alternate_profiles: vec![],
            enabled: true,
            priority: 0,
        })
    }

    pub fn parse_desktop_file(path: &Path) -> Option<BrowserTarget> {
        Self::parse_entry(path)
    }

    pub fn clean_exec_command(exec: &str) -> String {
        let mut parts = Vec::new();
        for part in exec.split_whitespace() {
            if !part.starts_with('%') {
                parts.push(part);
            }
        }
        parts.join(" ")
    }

    pub fn clean_exec(exec: &str) -> String {
        Self::clean_exec_command(exec)
    }

    pub fn find_icon_asset(icon_name: &str) -> Option<String> {
        if icon_name.starts_with('/') && Path::new(icon_name).exists() {
            return Some(icon_name.to_string());
        }

        let theme_dirs = [
            "/usr/share/icons/hicolor/scalable/apps",
            "/usr/share/icons/hicolor/128x128/apps",
            "/usr/share/icons/hicolor/48x48/apps",
            "/usr/share/pixmaps",
        ];

        let extensions = [".svg", ".png", ""];
        for dir in &theme_dirs {
            for ext in &extensions {
                let candidate = format!("{}/{}{}", dir, icon_name, ext);
                if Path::new(&candidate).exists() {
                    return Some(candidate);
                }
            }
        }

        None
    }
}

impl BrowserDiscoverer for XdgBrowserDiscoverer {
    fn discover(&self) -> Vec<BrowserTarget> {
        let mut discovered = Vec::new();
        let mut seen_binaries = HashSet::new();
        let mut priority = 1;

        for dir in Self::get_search_directories() {
            if !dir.is_dir() {
                continue;
            }

            let entries = match fs::read_dir(&dir) {
                Ok(e) => e,
                Err(_) => continue,
            };

            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|ext| ext.to_str()) == Some("desktop") {
                    if let Some(mut browser) = Self::parse_entry(&path) {
                        if !seen_binaries.contains(&browser.executable_path) {
                            seen_binaries.insert(browser.executable_path.clone());
                            browser.priority = priority;
                            priority += 1;
                            discovered.push(browser);
                        }
                    }
                }
            }
        }

        discovered
    }
}

#[derive(Default)]
pub struct LinuxProtocolHandler;

impl LinuxProtocolHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn get_mimeapps_path() -> Option<PathBuf> {
        if let Ok(config_home) = env::var("XDG_CONFIG_HOME") {
            let path = PathBuf::from(config_home).join("mimeapps.list");
            return Some(path);
        }
        dirs::home_dir().map(|h| h.join(".config/mimeapps.list"))
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
}

impl ProtocolHandler for LinuxProtocolHandler {
    fn check_status(&self) -> HandlerStatus {
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
            if let Some(mime_path) = Self::get_mimeapps_path() {
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

    fn register_as_default(&self) -> RegistrationResult {
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
        let current_exe = env::current_exe()
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
        if let Some(mime_path) = Self::get_mimeapps_path() {
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
        let verification = self.check_status();
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

    fn open_system_settings(&self) -> Result<(), String> {
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
}

pub type LinuxPlatform = XdgBrowserDiscoverer;

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_dynamic_xdg_parser() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("librewolf.desktop");
        let content = r#"[Desktop Entry]
Name=LibreWolf
Exec=/usr/bin/librewolf %u
Icon=librewolf
Categories=Network;WebBrowser;
MimeType=x-scheme-handler/http;x-scheme-handler/https;
"#;
        fs::write(&file, content).unwrap();

        let browser = XdgBrowserDiscoverer::parse_entry(&file).unwrap();
        assert_eq!(browser.name, "LibreWolf");
        assert_eq!(browser.executable_path, "/usr/bin/librewolf");
        assert_eq!(browser.id, "librewolf");
    }

    #[test]
    fn test_reject_non_browser_desktop_file() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("calculator.desktop");
        let content = r#"[Desktop Entry]
Name=Calculator
Exec=/usr/bin/gnome-calculator
Categories=Utility;Calculator;
"#;
        fs::write(&file, content).unwrap();

        let browser = XdgBrowserDiscoverer::parse_entry(&file);
        assert!(browser.is_none());
    }

    #[test]
    fn test_linux_protocol_handler_mimeapps() {
        let dir = tempdir().unwrap();
        let mime_path = dir.path().join("mimeapps.list");

        LinuxProtocolHandler::update_mimeapps_list(&mime_path, "openin.desktop").unwrap();
        let content = fs::read_to_string(&mime_path).unwrap();
        assert!(content.contains("[Default Applications]"));
        assert!(content.contains("x-scheme-handler/http=openin.desktop"));
    }
}
