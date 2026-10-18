pub mod application;
pub mod commands;
pub mod domain;
pub mod infrastructure;
pub mod platform;

use application::{DiscoveryService, LaunchService, RoutingService, SettingsService};
use infrastructure::{FileSettingsStore, NativeProcessRunner};
use platform::create_default_discoverer;
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub discovery_service: DiscoveryService,
    pub routing_service: RoutingService,
    pub launch_service: LaunchService,
    pub settings_service: SettingsService,
    pub incoming_url: Mutex<Option<String>>,
}

pub fn run() {
    // Composition Root: assemble infrastructure, adapters, and application services
    let store = Arc::new(FileSettingsStore::new().expect("Failed to initialize file settings store"));
    let runner = Arc::new(NativeProcessRunner::new());
    let discoverer = create_default_discoverer();

    let settings_service = SettingsService::new(store);
    let launch_service = LaunchService::new(runner);
    let routing_service = RoutingService::new();
    let discovery_service = DiscoveryService::new(discoverer);

    let mut initial_url = None;
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let candidate = &args[1];
        if !candidate.starts_with('-') {
            initial_url = Some(candidate.clone());
        }
    }

    let app_state = AppState {
        discovery_service,
        routing_service,
        launch_service,
        settings_service,
        incoming_url: Mutex::new(initial_url),
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            commands::get_bootstrap_data,
            commands::sanitize_url,
            commands::test_rule,
            commands::save_settings,
            commands::refresh_browsers,
            commands::launch_browser,
            commands::evaluate_and_route,
            commands::dismiss_window,
        ])
        .run(tauri::generate_context!())
        .expect("Error initializing OpenIn desktop shell");
}
