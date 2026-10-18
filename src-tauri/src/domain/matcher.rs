use crate::domain::models::{RouteDecision, RoutingRule, RuleSet, ValidatedUrl};
use regex::Regex;

pub struct RuleMatcher;

impl RuleMatcher {
    const MAX_REGEX_LEN: usize = 512;

    pub fn parse_rule(raw: &str) -> Option<RoutingRule> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return None;
        }

        if let Some(pattern) = trimmed.strip_prefix("s$").or_else(|| trimmed.strip_prefix("s:")) {
            Some(RoutingRule::Exact(pattern.trim().to_string()))
        } else if let Some(pattern) = trimmed.strip_prefix("d$").or_else(|| trimmed.strip_prefix("d:")) {
            let pat = pattern.trim();
            if let Some(sub) = pat.strip_prefix("*.") {
                Some(RoutingRule::Domain {
                    domain: sub.to_lowercase(),
                    allow_subdomains: true,
                })
            } else {
                Some(RoutingRule::Domain {
                    domain: pat.to_lowercase(),
                    allow_subdomains: false,
                })
            }
        } else if let Some(pattern) = trimmed.strip_prefix("r$").or_else(|| trimmed.strip_prefix("r:")) {
            let pat = pattern.trim();
            if pat.len() > Self::MAX_REGEX_LEN {
                return None;
            }
            Some(RoutingRule::Regex(pat.to_string()))
        } else if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
            Some(RoutingRule::Exact(trimmed.to_string()))
        } else {
            // Default to domain or wildcard match
            if let Some(sub) = trimmed.strip_prefix("*.") {
                Some(RoutingRule::Domain {
                    domain: sub.to_lowercase(),
                    allow_subdomains: true,
                })
            } else {
                Some(RoutingRule::Domain {
                    domain: trimmed.to_lowercase(),
                    allow_subdomains: false,
                })
            }
        }
    }

    pub fn matches(rule: &RoutingRule, url: &ValidatedUrl) -> bool {
        match rule {
            RoutingRule::Exact(exact) => exact.trim() == url.canonical.trim(),
            RoutingRule::Domain { domain, allow_subdomains } => {
                if let Some(host) = &url.host {
                    let host_lower = host.to_lowercase();
                    if &host_lower == domain {
                        return true;
                    }
                    if *allow_subdomains {
                        let dot_domain = format!(".{}", domain);
                        host_lower.ends_with(&dot_domain)
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            RoutingRule::Regex(pattern) => {
                if let Ok(re) = Regex::new(pattern) {
                    re.is_match(&url.canonical)
                } else {
                    false
                }
            }
        }
    }

    pub fn evaluate(rulesets: &[RuleSet], url: &ValidatedUrl) -> Option<RouteDecision> {
        let mut active_rulesets: Vec<&RuleSet> = rulesets.iter().filter(|r| r.enabled).collect();
        active_rulesets.sort_by_key(|r| r.priority);

        for ruleset in active_rulesets {
            for raw_rule in &ruleset.rules {
                if let Some(parsed) = Self::parse_rule(raw_rule) {
                    if Self::matches(&parsed, url) {
                        return Some(RouteDecision {
                            ruleset_id: ruleset.id.clone(),
                            ruleset_name: ruleset.name.clone(),
                            target_browser_id: ruleset.target_browser_id.clone(),
                            launch_arguments: ruleset.launch_arguments.clone(),
                            matched_rule: raw_rule.clone(),
                        });
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_matching_exact_and_subdomain() {
        let rule_exact = RuleMatcher::parse_rule("d$github.com").unwrap();
        let rule_wildcard = RuleMatcher::parse_rule("d$*.github.com").unwrap();

        let root_url = ValidatedUrl {
            raw: "https://github.com/".into(),
            canonical: "https://github.com/".into(),
            scheme: "https".into(),
            host: Some("github.com".into()),
        };

        let sub_url = ValidatedUrl {
            raw: "https://gist.github.com/test".into(),
            canonical: "https://gist.github.com/test".into(),
            scheme: "https".into(),
            host: Some("gist.github.com".into()),
        };

        assert!(RuleMatcher::matches(&rule_exact, &root_url));
        assert!(!RuleMatcher::matches(&rule_exact, &sub_url));

        assert!(RuleMatcher::matches(&rule_wildcard, &root_url));
        assert!(RuleMatcher::matches(&rule_wildcard, &sub_url));
    }

    #[test]
    fn test_regex_matching_linear_time() {
        let rule = RuleMatcher::parse_rule(r"r$^https://.*\.internal\..*$").unwrap();
        let url = ValidatedUrl {
            raw: "https://portal.internal.company.com/home".into(),
            canonical: "https://portal.internal.company.com/home".into(),
            scheme: "https".into(),
            host: Some("portal.internal.company.com".into()),
        };

        assert!(RuleMatcher::matches(&rule, &url));
    }
}
