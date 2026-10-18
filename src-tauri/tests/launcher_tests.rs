use open_in_lib::application::LaunchService;
use open_in_lib::domain::{AlternateProfile, BrowserTarget, LaunchError};
use open_in_lib::infrastructure::{NativeProcessRunner, ProcessRunner};
use std::sync::Arc;

#[test]
fn test_arg_preparation_url_substitution() {
    let args = LaunchService::prepare_arguments(
        Some("--app=%URL% --window-size=800,600"),
        "https://example.com/login",
    );
    assert_eq!(
        args,
        vec!["--app=https://example.com/login", "--window-size=800,600"]
    );
}

#[test]
fn test_arg_preparation_default_append() {
    let args =
        LaunchService::prepare_arguments(Some("--new-window --private"), "https://example.com");
    assert_eq!(
        args,
        vec!["--new-window", "--private", "https://example.com"]
    );
}

#[test]
fn test_alternate_launch_profile() {
    let runner = Arc::new(NativeProcessRunner::new());
    let service = LaunchService::new(runner);

    let browser = BrowserTarget {
        id: "chrome".into(),
        name: "Google Chrome".into(),
        executable_path: "/usr/bin/google-chrome".into(),
        arguments: None,
        icon_path: None,
        alternate_profiles: vec![AlternateProfile {
            id: "incognito".into(),
            name: "New Incognito Window".into(),
            arguments: "--incognito %URL%".into(),
            icon_path: None,
        }],
        enabled: true,
        priority: 1,
    };

    // Nonexistent alternate profile ID returns typed error
    let err = service.launch(&browser, "https://example.com", Some("missing_profile"));
    assert_eq!(
        err,
        Err(LaunchError::AlternateProfileNotFound(
            "missing_profile".into()
        ))
    );
}

#[test]
fn test_rejection_of_shell_executables() {
    let runner = NativeProcessRunner::new();
    let bad_binaries = &["/bin/bash", "/usr/bin/sh", "cmd.exe", "powershell.exe"];

    for bad in bad_binaries {
        let err = runner.run(bad, &["https://example.com".to_string()]);
        assert_eq!(err, Err(LaunchError::DisallowedExecutable(bad.to_string())));
    }
}
