use crate::domain::models::BrowserTarget;
use crate::platform::traits::BrowserDiscoverer;
use std::path::PathBuf;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_macos_discoverer_creation() {
        let discoverer = MacOsBrowserDiscoverer::new();
        let browsers = discoverer.discover();
        // Discovered list only contains existing apps
        for b in browsers {
            assert!(Path::new(&b.executable_path).exists());
        }
    }
}
