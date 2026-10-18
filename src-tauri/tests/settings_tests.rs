use open_in_lib::domain::{BrowserTarget, Configuration};
use open_in_lib::infrastructure::FileSettingsStore;
use tempfile::tempdir;

#[test]
fn test_settings_persistence_cycle() {
    let dir = tempdir().expect("Failed to create temporary directory");
    let file_path = dir.path().join("config.json");
    let store = FileSettingsStore::with_path(file_path.clone());

    let mut config = Configuration::default();
    config.preferences.theme = "dark".into();
    config.preferences.rule_routing_enabled = true;
    config.browsers.push(BrowserTarget {
        id: "vivaldi".into(),
        name: "Vivaldi".into(),
        executable_path: "/opt/vivaldi/vivaldi".into(),
        arguments: None,
        icon_path: None,
        alternate_profiles: vec![],
        enabled: true,
        priority: 1,
    });

    store.save(&config).expect("Saving settings should succeed");

    assert!(file_path.exists());

    let loaded = store.load().expect("Loading settings should succeed");
    assert_eq!(loaded.preferences.theme, "dark");
    assert_eq!(loaded.browsers.len(), 1);
    assert_eq!(loaded.browsers[0].id, "vivaldi");
}
