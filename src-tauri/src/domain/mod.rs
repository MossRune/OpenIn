pub mod errors;
pub mod matcher;
pub mod models;
pub mod sanitizer;

// Compatibility aliases
pub mod launcher;
pub mod rule_matcher;
pub mod settings;
pub mod url_sanitizer;

pub use errors::{LaunchError, RoutingError, SettingsError, ValidationError};
pub use matcher::RuleMatcher;
pub use models::{
    AlternateProfile, BrowserTarget, Configuration, Preferences, RouteDecision, RoutingRule,
    RuleSet, ValidatedUrl,
};
pub use sanitizer::UrlSanitizer;
