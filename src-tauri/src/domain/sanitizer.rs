use crate::domain::errors::ValidationError;
use crate::domain::models::ValidatedUrl;
use url::Url;

pub struct UrlSanitizer;

impl UrlSanitizer {
    /// Allowed URL schemes for external links
    const ALLOWED_SCHEMES: &'static [&'static str] = &[
        "http", "https", "ftp", "gemini", "gopher", "mailto"
    ];

    /// Registered application URI prefixes that can wrap actual URLs
    const STRIPPABLE_PREFIXES: &'static [&'static str] = &[
        "openin://",
        "openin:",
        "openwith://",
        "openwith:",
        "linkchooser://",
        "linkchooser:",
    ];

    pub fn validate(input: &str) -> Result<ValidatedUrl, ValidationError> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err(ValidationError::EmptyInput);
        }

        // Reject null bytes and non-printable control characters
        if trimmed.chars().any(|c| c.is_control() && c != '\t') {
            return Err(ValidationError::IllegalCharacters);
        }

        // Prevent CLI argument flag injection (e.g. `--remote-debugging-port`)
        if trimmed.starts_with('-') {
            return Err(ValidationError::ArgumentInjectionAttempt);
        }

        // Strip surrounding matching single or double quotes
        let mut stripped = trimmed;
        if (stripped.starts_with('"') && stripped.ends_with('"'))
            || (stripped.starts_with('\'') && stripped.ends_with('\''))
        {
            if stripped.len() >= 2 {
                stripped = &stripped[1..stripped.len() - 1].trim();
            }
        }

        // Unwrap application-specific deep-link prefixes
        for prefix in Self::STRIPPABLE_PREFIXES {
            if stripped.to_ascii_lowercase().starts_with(prefix) {
                stripped = &stripped[prefix.len()..].trim();
                break;
            }
        }

        if stripped.is_empty() {
            return Err(ValidationError::EmptyInput);
        }

        // Re-check for flag injection after prefix strip
        if stripped.starts_with('-') {
            return Err(ValidationError::ArgumentInjectionAttempt);
        }

        // If the URL already parses with a scheme (e.g. http:, javascript:, file:), use it.
        // Otherwise, attempt prepending https:// for bare domain/path inputs.
        let parsed = match Url::parse(stripped) {
            Ok(url) => url,
            Err(_) => {
                let candidate = format!("https://{}", stripped);
                Url::parse(&candidate).map_err(|e| ValidationError::ParseFailure(e.to_string()))?
            }
        };

        let scheme = parsed.scheme().to_ascii_lowercase();

        // Validate scheme whitelist
        if !Self::ALLOWED_SCHEMES.contains(&scheme.as_str()) {
            return Err(ValidationError::DisallowedScheme(scheme));
        }

        let host = parsed.host_str().map(|h| h.to_lowercase());

        // Network schemes require a non-empty host
        if (scheme == "http" || scheme == "https" || scheme == "ftp" || scheme == "gemini") && host.is_none() {
            return Err(ValidationError::MissingHost(parsed.to_string()));
        }

        Ok(ValidatedUrl {
            raw: input.to_string(),
            canonical: parsed.to_string(),
            scheme,
            host,
        })
    }

    pub fn sanitize(input: &str) -> Result<ValidatedUrl, ValidationError> {
        Self::validate(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_https_url() {
        let res = UrlSanitizer::validate("https://rust-lang.org/learn").unwrap();
        assert_eq!(res.canonical, "https://rust-lang.org/learn");
        assert_eq!(res.scheme, "https");
        assert_eq!(res.host, Some("rust-lang.org".to_string()));
    }

    #[test]
    fn test_bare_domain_auto_prefix() {
        let res = UrlSanitizer::validate("crates.io/crates/tauri").unwrap();
        assert_eq!(res.canonical, "https://crates.io/crates/tauri");
        assert_eq!(res.scheme, "https");
        assert_eq!(res.host, Some("crates.io".to_string()));
    }

    #[test]
    fn test_unwrap_openwith_prefix() {
        let res = UrlSanitizer::validate("openwith://https://github.com/rust-lang").unwrap();
        assert_eq!(res.canonical, "https://github.com/rust-lang");
    }

    #[test]
    fn test_reject_flag_injection() {
        assert_eq!(
            UrlSanitizer::validate("--incognito"),
            Err(ValidationError::ArgumentInjectionAttempt)
        );
        assert_eq!(
            UrlSanitizer::validate("-P default"),
            Err(ValidationError::ArgumentInjectionAttempt)
        );
    }

    #[test]
    fn test_reject_dangerous_schemes() {
        assert!(matches!(
            UrlSanitizer::validate("javascript:void(0)"),
            Err(ValidationError::DisallowedScheme(_))
        ));
        assert!(matches!(
            UrlSanitizer::validate("file:///etc/passwd"),
            Err(ValidationError::DisallowedScheme(_))
        ));
        assert!(matches!(
            UrlSanitizer::validate("data:text/html,<script>"),
            Err(ValidationError::DisallowedScheme(_))
        ));
    }

    #[test]
    fn test_mailto_url() {
        let res = UrlSanitizer::validate("mailto:team@openwithrust.org").unwrap();
        assert_eq!(res.scheme, "mailto");
        assert_eq!(res.canonical, "mailto:team@openwithrust.org");
    }
}
