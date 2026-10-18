use crate::domain::models::BrowserTarget;
use crate::platform::traits::BrowserDiscoverer;

pub struct WindowsBrowserDiscoverer;

impl WindowsBrowserDiscoverer {
    pub fn new() -> Self {
        Self
    }
}

impl BrowserDiscoverer for WindowsBrowserDiscoverer {
    fn discover(&self) -> Vec<BrowserTarget> {
        let mut discovered = Vec::new();

        #[cfg(target_os = "windows")]
        {
            // Windows Registry StartMenuInternet implementation
        }

        let common = &[
            ("msedge", "Microsoft Edge", r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe"),
            ("chrome", "Google Chrome", r"C:\Program Files\Google\Chrome\Application\chrome.exe"),
            ("firefox", "Mozilla Firefox", r"C:\Program Files\Mozilla Firefox\firefox.exe"),
            ("brave", "Brave Browser", r"C:\Program Files\BraveSoftware\Brave-Browser\Application\brave.exe"),
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
