use open_in_lib::domain::{UrlSanitizer, ValidationError};

#[test]
fn test_standard_https_link() {
    let url = "https://doc.rust-lang.org/book/ch01-01-installation.html";
    let res = UrlSanitizer::validate(url).expect("Should sanitize valid HTTPS URL");
    assert_eq!(res.canonical, url);
    assert_eq!(res.scheme, "https");
    assert_eq!(res.host.as_deref(), Some("doc.rust-lang.org"));
}

#[test]
fn test_query_params_and_fragments() {
    let url = "https://search.brave.com/search?q=tauri+v2+tutorial&source=web#results";
    let res = UrlSanitizer::validate(url).expect("Should preserve query parameters and fragments");
    assert_eq!(res.canonical, url);
}

#[test]
fn test_custom_protocol_unwrapping() {
    let openin_url = "openin://https://news.ycombinator.com/";
    let res = UrlSanitizer::validate(openin_url).expect("Should strip openin:// prefix");
    assert_eq!(res.canonical, "https://news.ycombinator.com/");

    let openwith_url = "openwith://https://news.ycombinator.com/";
    let res2 = UrlSanitizer::validate(openwith_url).expect("Should strip openwith:// prefix");
    assert_eq!(res2.canonical, "https://news.ycombinator.com/");
}

#[test]
fn test_cli_injection_rejection() {
    let flag1 = "--remote-debugging-port=9222";
    assert_eq!(
        UrlSanitizer::validate(flag1),
        Err(ValidationError::ArgumentInjectionAttempt)
    );

    let flag2 = "-profile /path/to/profile";
    assert_eq!(
        UrlSanitizer::validate(flag2),
        Err(ValidationError::ArgumentInjectionAttempt)
    );
}

#[test]
fn test_dangerous_schemes() {
    let javascript_uri = "javascript:alert(document.cookie)";
    assert!(matches!(
        UrlSanitizer::validate(javascript_uri),
        Err(ValidationError::DisallowedScheme(_))
    ));

    let file_uri = "file:///etc/shadow";
    assert!(matches!(
        UrlSanitizer::validate(file_uri),
        Err(ValidationError::DisallowedScheme(_))
    ));
}

#[test]
fn test_null_bytes_and_control_chars() {
    let null_byte = "https://example.com/login\0admin";
    assert_eq!(
        UrlSanitizer::validate(null_byte),
        Err(ValidationError::IllegalCharacters)
    );
}
