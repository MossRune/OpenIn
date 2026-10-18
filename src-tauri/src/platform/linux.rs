use crate::domain::models::BrowserTarget;
use crate::platform::traits::BrowserDiscoverer;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

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
}
