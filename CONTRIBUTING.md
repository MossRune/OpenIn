# Contributing to OpenIn

Thank you for your interest in contributing to OpenIn! This document outlines our development workflows, code standards, and quality gates.

---

## Architecture Guidelines

OpenIn is structured into distinct architectural layers adhering to SOLID design principles:

1. **Domain (`src-tauri/src/domain/`)**: Pure business models, validation logic, routing rules, and domain errors. Must not depend on Tauri, UI frameworks, or operating system APIs.
2. **Application (`src-tauri/src/application/`)**: Application services and use cases (`DiscoveryService`, `LaunchService`, `RoutingService`, `SettingsService`). Must depend on abstractions (`BrowserDiscoverer`, `ProcessRunner`, `SettingsStore`).
3. **Infrastructure (`src-tauri/src/infrastructure/`)**: Concrete implementations for file persistence and process execution.
4. **Platform (`src-tauri/src/platform/`)**: OS-specific adapters for browser discovery across Linux, Windows, and macOS.
5. **Presentation (`src/`)**: Vanilla TypeScript and CSS design tokens. No heavy UI framework runtimes.

---

## Development Workflow

### Prerequisites
- **Rust**: Stable toolchain (Edition 2024)
- **Bun** or **Node.js**: For frontend assets

### Run Tests
```bash
cargo test --manifest-path src-tauri/Cargo.toml
```

### Formatting & Linting
```bash
cargo fmt --all --check
cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
```

### Frontend Development
```bash
bun install
bun run dev
```

---

## Security Invariants

When adding features or modifying code, ensure the following security invariants are strictly preserved:
- Never invoke processes via a system shell (`sh -c` or `cmd.exe`).
- Always validate incoming URLs through `UrlSanitizer::validate`.
- Never execute shell binaries (`/bin/sh`, `/bin/bash`, `cmd.exe`).
- Keep regex evaluation linear-time to prevent ReDoS.
- Maintain atomic file persistence to prevent configuration corruption.
