# Changelog

All notable changes to the **OpenIn** project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.0] - 2026-10-18

### Added
- **Dynamic Application Discovery**: Automatic discovery of installed browsers across Linux (XDG Desktop entries, Flatpak, Snap), Windows (StartMenuInternet), and macOS (Launch Services).
- **Hardened URL Sanitization**: Strict scheme validation, CLI argument flag injection defense (`-` and `--`), control character filtering, and query/fragment preservation.
- **Flexible URL Routing Engine**: Support for exact URL routing, domain/subdomain wildcards (`*.domain.com`), and linear-time regular expression patterns.
- **Safe Process Spawning**: Direct `std::process::Command` invocation with argument templating (`%URL%`, `%u`, `%U`) eliminating shell injection vulnerabilities.
- **Atomic Persistence**: Reliable settings management with atomic sync/rename and corrupted file backup recovery.
- **Keyboard-First User Interface**: Zero-overhead Vanilla TypeScript frontend with instant 1-9 direct hotkeys, arrow navigation, clean dark/light themes, and under 100ms visual render.
- **Native Messaging Bridge**: Bundled browser extension host (`open-in-nmh`) for seamless link redirection from web browsers.
- **Automated CI/CD**: Multi-platform matrix testing, linting, formatting checks, and automated GitHub Releases for Linux, Windows, and macOS.
