use open_in_lib::platform::linux::XdgBrowserDiscoverer;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_desktop_file_parsing() {
    let dir = tempdir().expect("Temporary directory");
    let desktop_file = dir.path().join("firefox.desktop");

    let content = r#"[Desktop Entry]
Version=1.0
Name=Firefox Web Browser
Comment=Browse the World Wide Web
GenericName=Web Browser
Exec=/usr/bin/firefox %u
Terminal=false
Type=Application
Icon=firefox
Categories=GNOME;GTK;Network;WebBrowser;
MimeType=text/html;text/xml;application/xhtml+xml;x-scheme-handler/http;x-scheme-handler/https;
"#;

    fs::write(&desktop_file, content).expect("Write desktop file");

    let browser = XdgBrowserDiscoverer::parse_entry(&desktop_file)
        .expect("Should identify Firefox as a valid browser");

    assert_eq!(browser.name, "Firefox Web Browser");
    assert_eq!(browser.executable_path, "/usr/bin/firefox");
}

#[test]
fn test_ignore_no_display_desktop_files() {
    let dir = tempdir().expect("Temporary directory");
    let desktop_file = dir.path().join("hidden_browser.desktop");

    let content = r#"[Desktop Entry]
Name=Hidden Internal Browser
Exec=/usr/bin/hidden-browser %u
Categories=Network;WebBrowser;
NoDisplay=true
"#;

    fs::write(&desktop_file, content).expect("Write desktop file");

    let browser = XdgBrowserDiscoverer::parse_entry(&desktop_file);
    assert!(browser.is_none());
}
