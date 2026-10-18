use open_in_lib::application::{DiscoveryService, LaunchService, SettingsService};
use open_in_lib::domain::{
    BrowserTarget, Configuration, LaunchError, Preferences, SettingsError,
};
use open_in_lib::infrastructure::traits::{ProcessRunner, SettingsStore};
use open_in_lib::platform::traits::BrowserDiscoverer;
use std::sync::{Arc, Mutex};

// Mock BrowserDiscoverer
struct MockDiscoverer {
    browsers: Vec<BrowserTarget>,
}

impl BrowserDiscoverer for MockDiscoverer {
    fn discover(&self) -> Vec<BrowserTarget> {
        self.browsers.clone()
    }
}

// Mock ProcessRunner
struct MockProcessRunner {
    invocations: Mutex<Vec<(String, Vec<String>)>>,
}

impl ProcessRunner for MockProcessRunner {
    fn run(&self, executable: &str, arguments: &[String]) -> Result<(), LaunchError> {
        self.invocations
            .lock()
            .unwrap()
            .push((executable.to_string(), arguments.to_vec()));
        Ok(())
    }
}

// Mock SettingsStore
struct MockSettingsStore {
    data: Mutex<Configuration>,
}

impl SettingsStore for MockSettingsStore {
    fn load(&self) -> Result<Configuration, SettingsError> {
        Ok(self.data.lock().unwrap().clone())
    }

    fn save(&self, config: &Configuration) -> Result<(), SettingsError> {
        *self.data.lock().unwrap() = config.clone();
        Ok(())
    }
}

#[test]
fn test_discovery_service_with_injected_discoverer() {
    let mock = Arc::new(MockDiscoverer {
        browsers: vec![BrowserTarget {
            id: "mock_browser".into(),
            name: "Mock Browser".into(),
            executable_path: "/opt/mock/browser".into(),
            arguments: None,
            icon_path: None,
            alternate_profiles: vec![],
            enabled: true,
            priority: 1,
        }],
    });

    let service = DiscoveryService::new(mock);
    let discovered = service.discover_installed_browsers();
    assert_eq!(discovered.len(), 1);
    assert_eq!(discovered[0].id, "mock_browser");
}

#[test]
fn test_launch_service_with_injected_runner() {
    let mock_runner = Arc::new(MockProcessRunner {
        invocations: Mutex::new(Vec::new()),
    });

    let service = LaunchService::new(mock_runner.clone());

    let target = BrowserTarget {
        id: "firefox".into(),
        name: "Firefox".into(),
        executable_path: "/usr/bin/firefox".into(),
        arguments: Some("--new-tab %URL%".into()),
        icon_path: None,
        alternate_profiles: vec![],
        enabled: true,
        priority: 1,
    };

    service
        .launch(&target, "https://openin.dev", None)
        .expect("Launch should succeed via mock runner");

    let invocations = mock_runner.invocations.lock().unwrap();
    assert_eq!(invocations.len(), 1);
    assert_eq!(invocations[0].0, "/usr/bin/firefox");
    assert_eq!(
        invocations[0].1,
        vec!["--new-tab", "https://openin.dev"]
    );
}

#[test]
fn test_settings_service_with_injected_store() {
    let store = Arc::new(MockSettingsStore {
        data: Mutex::new(Configuration {
            preferences: Preferences {
                theme: "light".into(),
                ..Default::default()
            },
            browsers: vec![],
            rulesets: vec![],
        }),
    });

    let service = SettingsService::new(store.clone());
    let loaded = service.load_configuration().unwrap();
    assert_eq!(loaded.preferences.theme, "light");

    let mut updated = loaded.clone();
    updated.preferences.theme = "dark".into();
    service.save_configuration(&updated).unwrap();

    let reloaded = service.load_configuration().unwrap();
    assert_eq!(reloaded.preferences.theme, "dark");
}
