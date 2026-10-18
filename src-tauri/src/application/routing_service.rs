use crate::domain::matcher::RuleMatcher;
use crate::domain::models::{RouteDecision, RuleSet, ValidatedUrl};

#[derive(Default)]
pub struct RoutingService;

impl RoutingService {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(&self, rulesets: &[RuleSet], url: &ValidatedUrl) -> Option<RouteDecision> {
        RuleMatcher::evaluate(rulesets, url)
    }
}
