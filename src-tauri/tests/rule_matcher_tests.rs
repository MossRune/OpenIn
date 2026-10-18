use open_in_lib::domain::{RuleMatcher, RuleSet, ValidatedUrl};

#[test]
fn test_domain_matching_variants() {
    let rulesets = vec![RuleSet {
        id: "github_rules".into(),
        name: "GitHub Route".into(),
        rules: vec!["d$github.com".into()],
        target_browser_id: "firefox".into(),
        launch_arguments: None,
        enabled: true,
        priority: 1,
    }];

    let url = ValidatedUrl {
        raw: "https://github.com/rust-lang".into(),
        canonical: "https://github.com/rust-lang".into(),
        scheme: "https".into(),
        host: Some("github.com".into()),
    };

    let res = RuleMatcher::evaluate(&rulesets, &url);
    assert!(res.is_some());
    assert_eq!(res.unwrap().target_browser_id, "firefox");

    // Subdomains should not match exact domain rule
    let sub_url = ValidatedUrl {
        raw: "https://gist.github.com/test".into(),
        canonical: "https://gist.github.com/test".into(),
        scheme: "https".into(),
        host: Some("gist.github.com".into()),
    };

    let sub = RuleMatcher::evaluate(&rulesets, &sub_url);
    assert!(sub.is_none());
}

#[test]
fn test_wildcard_matching() {
    let rulesets = vec![RuleSet {
        id: "wildcard_rule".into(),
        name: "Google Wildcard".into(),
        rules: vec!["d$*.google.com".into()],
        target_browser_id: "chrome".into(),
        launch_arguments: Some("--incognito".into()),
        enabled: true,
        priority: 1,
    }];

    let url1 = ValidatedUrl {
        raw: "https://mail.google.com/mail/u/0/".into(),
        canonical: "https://mail.google.com/mail/u/0/".into(),
        scheme: "https".into(),
        host: Some("mail.google.com".into()),
    };
    let res1 = RuleMatcher::evaluate(&rulesets, &url1);
    assert!(res1.is_some());
    assert_eq!(res1.unwrap().launch_arguments, Some("--incognito".into()));

    let url2 = ValidatedUrl {
        raw: "https://google.com/".into(),
        canonical: "https://google.com/".into(),
        scheme: "https".into(),
        host: Some("google.com".into()),
    };
    let res2 = RuleMatcher::evaluate(&rulesets, &url2);
    assert!(res2.is_some());

    let url3 = ValidatedUrl {
        raw: "https://spoofgoogle.com/".into(),
        canonical: "https://spoofgoogle.com/".into(),
        scheme: "https".into(),
        host: Some("spoofgoogle.com".into()),
    };
    let res3 = RuleMatcher::evaluate(&rulesets, &url3);
    assert!(res3.is_none());
}

#[test]
fn test_regex_matching_resilience() {
    let rulesets = vec![RuleSet {
        id: "regex_rule".into(),
        name: "Internal Network".into(),
        rules: vec![r"r$^https?://192\.168\.\d+\.\d+(:\d+)?/.*$".into()],
        target_browser_id: "edge".into(),
        launch_arguments: None,
        enabled: true,
        priority: 1,
    }];

    let url = ValidatedUrl {
        raw: "http://192.168.1.50:8080/dashboard".into(),
        canonical: "http://192.168.1.50:8080/dashboard".into(),
        scheme: "http".into(),
        host: Some("192.168.1.50".into()),
    };

    let res = RuleMatcher::evaluate(&rulesets, &url);
    assert!(res.is_some());
    assert_eq!(res.unwrap().target_browser_id, "edge");
}
