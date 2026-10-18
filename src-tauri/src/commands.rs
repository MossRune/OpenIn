use crate::domain::{
    BrowserTarget, Configuration, RouteDecision, RuleMatcher, UrlSanitizer, ValidatedUrl,
};
use crate::platform::{HandlerStatus, Platform, RegistrationResult};
use crate::AppState;
use serde::Serialize;
use tauri::{AppHandle, Manager, State};

#[derive(Serialize)]
pub struct AppBootstrapData {
    pub settings: Configuration,
    pub discovered_browsers: Vec<BrowserTarget>,
    pub incoming_url: Option<String>,
    pub handler_status: HandlerStatus,
}

#[derive(Serialize)]
pub struct RouteSimulationResult {
    pub valid: bool,
    pub sanitized_url: String,
    pub matched_rule: Option<String>,
    pub target_browser_id: Option<String>,
    pub target_browser_name: Option<String>,
}

#[tauri::command]
pub fn get_bootstrap_data(state: State<'_, AppState>) -> Result<AppBootstrapData, String> {
    let mut config = state
        .settings_service
        .load_configuration()
        .map_err(|e| e.to_string())?;

    let discovered_browsers = state.discovery_service.discover_installed_browsers();

    // Auto-populate browsers if this is first launch with empty configuration
    if config.browsers.is_empty() && !discovered_browsers.is_empty() {
        config.browsers = discovered_browsers.clone();
        let _ = state.settings_service.save_configuration(&config);
    }

    let incoming_url = state.incoming_url.lock().unwrap().clone();
    let handler_status = Platform::check_handler_status();

    Ok(AppBootstrapData {
        settings: config,
        discovered_browsers,
        incoming_url,
        handler_status,
    })
}

#[tauri::command]
pub fn get_default_handler_status() -> HandlerStatus {
    Platform::check_handler_status()
}

#[tauri::command]
pub fn register_default_handler() -> RegistrationResult {
    Platform::register_as_default_handler()
}

#[tauri::command]
pub fn open_os_protocol_settings() -> Result<(), String> {
    Platform::open_system_settings()
}

#[tauri::command]
pub fn save_onboarding_step(step: u32, state: State<'_, AppState>) -> Result<(), String> {
    let mut config = state
        .settings_service
        .load_configuration()
        .map_err(|e| e.to_string())?;
    config.preferences.onboarding_step = step;
    state
        .settings_service
        .save_configuration(&config)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn complete_onboarding(
    preferred_browser_id: Option<String>,
    fallback_behavior: Option<String>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut config = state
        .settings_service
        .load_configuration()
        .map_err(|e| e.to_string())?;
    config.preferences.onboarding_completed = true;
    config.preferences.preferred_browser_id = preferred_browser_id;
    if let Some(fb) = fallback_behavior {
        config.preferences.fallback_behavior = fb;
    }
    state
        .settings_service
        .save_configuration(&config)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn reset_onboarding(state: State<'_, AppState>) -> Result<(), String> {
    let mut config = state
        .settings_service
        .load_configuration()
        .map_err(|e| e.to_string())?;
    config.preferences.onboarding_completed = false;
    config.preferences.onboarding_step = 1;
    state
        .settings_service
        .save_configuration(&config)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn test_route_simulation(
    url: String,
    state: State<'_, AppState>,
) -> Result<RouteSimulationResult, String> {
    let validated = UrlSanitizer::validate(&url).map_err(|e| e.to_string())?;
    let config = state
        .settings_service
        .load_configuration()
        .map_err(|e| e.to_string())?;

    if config.preferences.rule_routing_enabled {
        if let Some(decision) = state.routing_service.evaluate(&config.rulesets, &validated) {
            let browser_name = config
                .browsers
                .iter()
                .find(|b| b.id == decision.target_browser_id)
                .map(|b| b.name.clone());

            return Ok(RouteSimulationResult {
                valid: true,
                sanitized_url: validated.canonical,
                matched_rule: Some(decision.matched_rule),
                target_browser_id: Some(decision.target_browser_id),
                target_browser_name: browser_name,
            });
        }
    }

    // Default / preferred fallback
    let fallback_id = config
        .preferences
        .preferred_browser_id
        .clone()
        .or_else(|| config.browsers.first().map(|b| b.id.clone()));

    let fallback_name = fallback_id.as_ref().and_then(|id| {
        config
            .browsers
            .iter()
            .find(|b| b.id == *id)
            .map(|b| b.name.clone())
    });

    Ok(RouteSimulationResult {
        valid: true,
        sanitized_url: validated.canonical,
        matched_rule: None,
        target_browser_id: fallback_id,
        target_browser_name: fallback_name,
    })
}

#[tauri::command]
pub fn sanitize_url(url: String) -> Result<ValidatedUrl, String> {
    UrlSanitizer::validate(&url).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn test_rule(rule: String, url: String) -> Result<bool, String> {
    let validated = UrlSanitizer::validate(&url).map_err(|e| e.to_string())?;
    let parsed_rule = match RuleMatcher::parse_rule(&rule) {
        Some(r) => r,
        None => return Ok(false),
    };
    Ok(RuleMatcher::matches(&parsed_rule, &validated))
}

#[tauri::command]
pub fn save_settings(settings: Configuration, state: State<'_, AppState>) -> Result<(), String> {
    state
        .settings_service
        .save_configuration(&settings)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn refresh_browsers(state: State<'_, AppState>) -> Vec<BrowserTarget> {
    state.discovery_service.discover_installed_browsers()
}

#[tauri::command]
pub fn launch_browser(
    browser_id: String,
    url: String,
    alternate_id: Option<String>,
    app_handle: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let validated = UrlSanitizer::validate(&url).map_err(|e| e.to_string())?;
    let config = state
        .settings_service
        .load_configuration()
        .map_err(|e| e.to_string())?;

    let target = config
        .browsers
        .iter()
        .find(|b| b.id == browser_id)
        .ok_or_else(|| format!("Target browser '{}' not found", browser_id))?;

    state
        .launch_service
        .launch(target, &validated.canonical, alternate_id.as_deref())
        .map_err(|e| e.to_string())?;

    if config.preferences.dismiss_after_launch {
        if let Some(window) = app_handle.get_webview_window("main") {
            let _ = window.hide();
        }
    }

    Ok(())
}

#[tauri::command]
pub fn evaluate_and_route(
    url: String,
    app_handle: AppHandle,
    state: State<'_, AppState>,
) -> Result<Option<RouteDecision>, String> {
    let validated = UrlSanitizer::validate(&url).map_err(|e| e.to_string())?;
    let config = state
        .settings_service
        .load_configuration()
        .map_err(|e| e.to_string())?;

    if !config.preferences.rule_routing_enabled {
        return Ok(None);
    }

    if let Some(decision) = state.routing_service.evaluate(&config.rulesets, &validated) {
        if let Some(target) = config
            .browsers
            .iter()
            .find(|b| b.id == decision.target_browser_id)
        {
            state
                .launch_service
                .launch(
                    target,
                    &validated.canonical,
                    decision.launch_arguments.as_deref(),
                )
                .map_err(|e| e.to_string())?;

            if config.preferences.dismiss_after_launch {
                if let Some(window) = app_handle.get_webview_window("main") {
                    let _ = window.hide();
                }
            }

            return Ok(Some(decision));
        }
    }

    Ok(None)
}

#[tauri::command]
pub fn dismiss_window(app_handle: AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.hide();
    }
    Ok(())
}
