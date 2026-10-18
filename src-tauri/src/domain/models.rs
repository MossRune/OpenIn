use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AlternateProfile {
    pub id: String,
    pub name: String,
    pub arguments: String,
    pub icon_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BrowserTarget {
    pub id: String,
    pub name: String,
    pub executable_path: String,
    pub arguments: Option<String>,
    pub icon_path: Option<String>,
    pub alternate_profiles: Vec<AlternateProfile>,
    pub enabled: bool,
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValidatedUrl {
    pub raw: String,
    pub canonical: String,
    pub scheme: String,
    pub host: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoutingRule {
    Exact(String),
    Domain {
        domain: String,
        allow_subdomains: bool,
    },
    Regex(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuleSet {
    pub id: String,
    pub name: String,
    pub rules: Vec<String>,
    pub target_browser_id: String,
    pub launch_arguments: Option<String>,
    pub enabled: bool,
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Preferences {
    #[serde(default)]
    pub launch_under_mouse: bool,
    #[serde(default = "default_true")]
    pub minimize_on_focus_loss: bool,
    #[serde(default = "default_theme")]
    pub theme: String, // "auto" | "dark" | "light"
    #[serde(default = "default_true")]
    pub rule_routing_enabled: bool,
    #[serde(default = "default_width")]
    pub window_width: u32,
    #[serde(default = "default_height")]
    pub window_height: u32,
    #[serde(default = "default_true")]
    pub dismiss_after_launch: bool,
    #[serde(default)]
    pub onboarding_completed: bool,
    #[serde(default)]
    pub onboarding_step: u32,
    #[serde(default)]
    pub preferred_browser_id: Option<String>,
    #[serde(default = "default_fallback_behavior")]
    pub fallback_behavior: String, // "always_ask" | "open_preferred"
}

fn default_true() -> bool {
    true
}

fn default_theme() -> String {
    "auto".to_string()
}

fn default_width() -> u32 {
    540
}

fn default_height() -> u32 {
    360
}

fn default_fallback_behavior() -> String {
    "always_ask".to_string()
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            launch_under_mouse: false,
            minimize_on_focus_loss: true,
            theme: default_theme(),
            rule_routing_enabled: true,
            window_width: default_width(),
            window_height: default_height(),
            dismiss_after_launch: true,
            onboarding_completed: false,
            onboarding_step: 1,
            preferred_browser_id: None,
            fallback_behavior: default_fallback_behavior(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct Configuration {
    pub preferences: Preferences,
    pub browsers: Vec<BrowserTarget>,
    pub rulesets: Vec<RuleSet>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RouteDecision {
    pub ruleset_id: String,
    pub ruleset_name: String,
    pub target_browser_id: String,
    pub launch_arguments: Option<String>,
    pub matched_rule: String,
}
