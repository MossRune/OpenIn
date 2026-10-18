# ADR 0002: Dependency Inversion and Dynamic Browser Discovery

## Status
Accepted

## Context
In the initial baseline, browser discovery relied on hardcoded binary name fallbacks in `platform/linux.rs`, and application services called concrete OS and file system structs directly without abstraction. This violated the Dependency Inversion Principle (DIP) and Open/Closed Principle (OCP), preventing unit testing of application workflows without live OS side effects.

## Decision

1. **Dependency Inversion via Traits**:
   - Introduced `BrowserDiscoverer`, `ProcessRunner`, and `SettingsStore` traits.
   - Application services (`DiscoveryService`, `LaunchService`, `SettingsService`) depend on these traits via `Arc<dyn Trait>` rather than concrete implementations.
   - Wired at the composition root (`src-tauri/src/lib.rs`).

2. **Dynamic XDG Discovery**:
   - Replaced all hardcoded browser lists with dynamic XDG directory enumeration (`$XDG_DATA_HOME/applications`, `$XDG_DATA_DIRS/applications`, Flatpak, Snap).
   - Any application declaring `Categories=WebBrowser` or handling `x-scheme-handler/http` / `x-scheme-handler/https` is automatically discovered.
   - Desktop entry field codes (`%u`, `%U`, `%f`, `%F`) are safely stripped.
   - High-resolution SVG/PNG icons are dynamically resolved from standard icon themes.

3. **Isolated Testability**:
   - Added mock fixtures (`MockDiscoverer`, `MockProcessRunner`, `MockSettingsStore`) in `tests/service_mock_tests.rs` allowing complete verification of application coordination logic in isolation.

## Consequences
- Clean separation between core business rules and OS interactions.
- Enhanced maintainability and zero brand bias in browser detection.
