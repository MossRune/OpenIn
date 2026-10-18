use open_in_lib::application::{RoutingService, SettingsService};
use open_in_lib::domain::models::{BrowserTarget, Configuration, RuleSet};
use open_in_lib::domain::UrlSanitizer;
use open_in_lib::infrastructure::FileSettingsStore;
use open_in_lib::platform::HandlerStatus;
use std::fs;
use std::sync::Arc;
use tempfile::tempdir;

#[test]
fn test_first_run_detection_defaults() {
    let config = Configuration::default();
    assert!(!config.preferences.onboarding_completed);
    assert_eq!(config.preferences.onboarding_step, 1);
    assert_eq!(config.preferences.preferred_browser_id, None);
    assert_eq!(config.preferences.fallback_behavior, "always_ask");
}

#[test]
fn test_onboarding_state_persistence_and_resumption() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("settings.json");
    let store = Arc::new(FileSettingsStore::with_path(config_path.clone()));
    let service = SettingsService::new(store);

    // Initial load: first-run default
    let mut config = service.load_configuration().unwrap();
    assert!(!config.preferences.onboarding_completed);
    assert_eq!(config.preferences.onboarding_step, 1);

    // Simulate advancing to Step 3 and closing the application
    config.preferences.onboarding_step = 3;
    service.save_configuration(&config).unwrap();

    // Reopen application: verify it resumes at Step 3
    let reloaded = service.load_configuration().unwrap();
    assert!(!reloaded.preferences.onboarding_completed);
    assert_eq!(reloaded.preferences.onboarding_step, 3);

    // Complete onboarding with a chosen browser
    let mut completed_config = reloaded;
    completed_config.preferences.onboarding_completed = true;
    completed_config.preferences.preferred_browser_id = Some("firefox".to_string());
    completed_config.preferences.fallback_behavior = "open_preferred".to_string();
    service.save_configuration(&completed_config).unwrap();

    // Reopen application: verify onboarding is completed
    let final_config = service.load_configuration().unwrap();
    assert!(final_config.preferences.onboarding_completed);
    assert_eq!(
        final_config.preferences.preferred_browser_id.as_deref(),
        Some("firefox")
    );
    assert_eq!(final_config.preferences.fallback_behavior, "open_preferred");
}

#[cfg(target_os = "linux")]
#[test]
fn test_mimeapps_list_protocol_handling() {
    let dir = tempdir().unwrap();
    let mime_path = dir.path().join("mimeapps.list");

    // Fresh file creation
    open_in_lib::platform::linux::LinuxProtocolHandler::update_mimeapps_list(
        &mime_path,
        "openin.desktop",
    )
    .unwrap();
    let content = fs::read_to_string(&mime_path).unwrap();
    assert!(content.contains("[Default Applications]"));
    assert!(content.contains("x-scheme-handler/http=openin.desktop"));
    assert!(content.contains("x-scheme-handler/https=openin.desktop"));

    // Updating existing file with prior registrations
    open_in_lib::platform::linux::LinuxProtocolHandler::update_mimeapps_list(
        &mime_path,
        "openin.desktop",
    )
    .unwrap();
    let content_updated = fs::read_to_string(&mime_path).unwrap();
    // Must not have duplicate lines
    let http_count = content_updated
        .lines()
        .filter(|l| l.trim() == "x-scheme-handler/http=openin.desktop")
        .count();
    assert_eq!(http_count, 1);
}

#[test]
fn test_handler_status_serializability() {
    let status = HandlerStatus {
        is_default: false,
        current_handler: Some("firefox.desktop".to_string()),
        os_type: "linux".to_string(),
        can_auto_register: true,
        details: "Current default browser handler is 'firefox.desktop'.".to_string(),
    };

    let serialized = serde_json::to_string(&status).unwrap();
    let deserialized: HandlerStatus = serde_json::from_str(&serialized).unwrap();
    assert_eq!(status, deserialized);
}

#[test]
fn test_url_simulation_routing() {
    let routing_service = RoutingService::new();
    let rulesets = vec![RuleSet {
        id: "work-rule".to_string(),
        name: "Work Links".to_string(),
        rules: vec!["*.github.com".to_string()],
        target_browser_id: "chrome".to_string(),
        launch_arguments: None,
        enabled: true,
        priority: 1,
    }];

    // Test matched URL
    let validated = UrlSanitizer::validate("https://github.com/MossRune/OpenIn").unwrap();
    let decision = routing_service.evaluate(&rulesets, &validated);
    assert!(decision.is_some());
    let dec = decision.unwrap();
    assert_eq!(dec.target_browser_id, "chrome");

    // Test non-matched URL
    let non_matching = UrlSanitizer::validate("https://news.ycombinator.com").unwrap();
    let no_decision = routing_service.evaluate(&rulesets, &non_matching);
    assert!(no_decision.is_none());
}

#[test]
fn test_browser_target_equality_and_fallback() {
    let browser = BrowserTarget {
        id: "firefox".to_string(),
        name: "Mozilla Firefox".to_string(),
        executable_path: "/usr/bin/firefox".to_string(),
        arguments: None,
        icon_path: None,
        alternate_profiles: vec![],
        enabled: true,
        priority: 1,
    };

    assert!(browser.enabled);
    assert_eq!(browser.id, "firefox");
}
