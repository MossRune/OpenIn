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
    pub launch_under_mouse: bool,
    pub minimize_on_focus_loss: bool,
    pub theme: String, // "auto" | "dark" | "light"
    pub rule_routing_enabled: bool,
    pub window_width: u32,
    pub window_height: u32,
    pub dismiss_after_launch: bool,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            launch_under_mouse: false,
            minimize_on_focus_loss: true,
            theme: "auto".to_string(),
            rule_routing_enabled: true,
            window_width: 540,
            window_height: 320,
            dismiss_after_launch: true,
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
