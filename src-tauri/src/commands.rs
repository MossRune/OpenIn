use crate::domain::{
    BrowserTarget, Configuration, RouteDecision, RuleMatcher, UrlSanitizer, ValidatedUrl,
};
use crate::AppState;
use serde::Serialize;
use tauri::{AppHandle, Manager, State};

#[derive(Serialize)]
pub struct AppBootstrapData {
    pub settings: Configuration,
    pub discovered_browsers: Vec<BrowserTarget>,
    pub incoming_url: Option<String>,
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

    Ok(AppBootstrapData {
        settings: config,
        discovered_browsers,
        incoming_url,
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
