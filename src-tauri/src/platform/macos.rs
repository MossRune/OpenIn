use crate::domain::models::BrowserTarget;
use crate::platform::traits::BrowserDiscoverer;

pub struct MacOsBrowserDiscoverer;

impl MacOsBrowserDiscoverer {
    pub fn new() -> Self {
        Self
    }
}

impl BrowserDiscoverer for MacOsBrowserDiscoverer {
    fn discover(&self) -> Vec<BrowserTarget> {
        let mut discovered = Vec::new();

        #[cfg(target_os = "macos")]
        {
            // macOS Launch Services implementation
        }

        let common = &[
            ("safari", "Safari", "/Applications/Safari.app"),
            ("chrome", "Google Chrome", "/Applications/Google Chrome.app"),
            ("firefox", "Firefox", "/Applications/Firefox.app"),
            ("arc", "Arc", "/Applications/Arc.app"),
            ("brave", "Brave Browser", "/Applications/Brave Browser.app"),
        ];

        let mut priority = 1;
        for (id, name, path) in common {
            discovered.push(BrowserTarget {
                id: id.to_string(),
                name: name.to_string(),
                executable_path: path.to_string(),
                arguments: None,
                icon_path: None,
                alternate_profiles: vec![],
                enabled: true,
                priority,
            });
            priority += 1;
        }

        discovered
    }
}
