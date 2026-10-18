# OpenIn

[![CI](https://github.com/MossRune/OpenIn/actions/workflows/ci.yml/badge.svg)](https://github.com/MossRune/OpenIn/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

**OpenIn** is a fast, secure, cross-platform desktop link chooser and browser router written in Rust with Tauri 2.

When you click a link in an external application (Slack, Discord, terminal, email, documents), OpenIn intercepts the URL and lets you quickly route it to the right browser or profile—manually via instant keyboard shortcuts, or automatically via configurable domain and regex rules.

---

## Key Features

- ⚡ **Instant Startup & UI**: Designed to appear in under 100ms. Zero bloated web frameworks (< 30 KB frontend bundle).
- ⌨️ **Keyboard-First Design**:
  - Keys `1-9`: Launch the corresponding browser immediately.
  - Arrow keys: Navigate the grid.
  - `Enter`: Open the selected browser.
  - `Esc`: Dismiss the window.
  - `C`: Copy clean URL to clipboard.
  - `E`: Focus and edit URL bar.
  - `,`: Toggle settings.
- 🛡️ **Hardened Security**:
  - Direct non-shell process spawning (immune to shell injection).
  - CLI flag injection defense (rejects URLs starting with `-` or `--`).
  - ReDoS immunity via linear-time DFA regex matching.
  - Strict Content Security Policy (CSP).
  - Zero analytics, zero network telemetry, 100% private.
- 🎯 **Flexible URL Routing**:
  - Exact URL match (`s$https://github.com/login`)
  - Domain & Subdomain wildcard match (`d$*.google.com` or `d$figma.com`)
  - Safe Regex match (`r$^https://.*\.internal\..*$`)
- 🌐 **Dynamic Application Discovery**:
  - **Linux**: Dynamic XDG desktop entry detection (`$XDG_DATA_HOME`, `$XDG_DATA_DIRS`, Flatpak, Snap). Zero hardcoded brand lists.
  - **Windows**: Registry StartMenuInternet detection.
  - **macOS**: Launch Services bundle detection.
- 🔌 **Browser Extension Support**:
  - Built-in Native Messaging Host (`open-in-nmh`) for browser extensions.

---

## Installation

Download the latest installer or package for your operating system from the [Releases](https://github.com/MossRune/OpenIn/releases) page:

- **Linux**: `.deb`, `.rpm`, or `.AppImage`
- **Windows**: `.msi` or `.exe` installer
- **macOS**: `.dmg` disk image or `.app` bundle

### Setting as Default Browser
- **Linux**: Run `xdg-settings set default-web-browser open-in.desktop` or select OpenIn in your Desktop Environment's Default Applications settings.
- **Windows**: Go to **Settings > Apps > Default apps > OpenIn** and set as default web browser.
- **macOS**: Select OpenIn as Default Web Browser in **System Settings > Desktop & Dock**.

---

## Development

### Prerequisites
- [Rust](https://rustup.rs/) (Stable, Edition 2024)
- [Bun](https://bun.sh/) or Node.js

### Local Commands
```bash
# Run unit & integration test suites
cargo test --manifest-path src-tauri/Cargo.toml

# Check code formatting & linting
cargo fmt --all --check
cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings

# Run frontend in development mode
bun install
bun run dev

# Build release bundle
bun run build
cargo tauri build
```

---

## Documentation

- [Architecture Specification](docs/ARCHITECTURE.md)
- [Threat Model & Security](docs/THREAT_MODEL.md)
- [Contributing Guidelines](CONTRIBUTING.md)
- [Security Policy](SECURITY.md)
- [Changelog](CHANGELOG.md)

---

## License

This project is licensed under the [MIT License](LICENSE).
