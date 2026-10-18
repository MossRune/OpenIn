use crate::domain::models::BrowserTarget;
use crate::platform::traits::BrowserDiscoverer;
use std::env;
use std::path::Path;

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
}
