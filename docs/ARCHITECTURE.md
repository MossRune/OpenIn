# OpenIn — Architecture Specification

## 1. Executive Summary

**OpenIn** is an independent, modular, secure, and cross-platform desktop link chooser and browser router engineered in Rust (Edition 2024) with Tauri 2 and a lightweight Vanilla TypeScript frontend.

The application adheres strictly to SOLID engineering principles, establishing clean boundaries between domain models, application orchestration, persistence infrastructure, operating system adapters, and frontend presentation.

---

## 2. Layered Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                 Presentation Layer (TypeScript + CSS)       │
│  - Index View: Compact Browser Grid, URL Bar, Hotkeys       │
│  - Settings Modal: Registered Targets, Rule Editor, Theme   │
│  - Accessibility: Keyboard Focus Navigation (1-9, Arrows)   │
└──────────────────────────────┬──────────────────────────────┘
                               │ Tauri IPC (Strongly Typed Commands)
┌──────────────────────────────▼──────────────────────────────┐
│                    Composition Root (src/lib.rs)            │
│  - Wires traits to concrete implementations at startup      │
│  - Manages AppState & thread-safe Mutex guards              │
└──────────────────────────────┬──────────────────────────────┘
                               │ Coordinates Services
┌──────────────────────────────▼──────────────────────────────┐
│                   Application Layer (src/application/)      │
│  ├── DiscoveryService: Coordinates dynamic browser scans    │
│  ├── RoutingService: Evaluates rules against incoming URLs  │
│  ├── LaunchService: Tokenizes arguments & invokes targets   │
│  └── SettingsService: Coordinates loading/saving configs    │
└──────────────┬──────────────────────────────┬───────────────┘
               │ Uses Traits                  │ Manipulates
┌──────────────▼──────────────┐┌──────────────▼───────────────┐
│ Infrastructure / Platform   ││        Domain Layer          │
│        (Adapters)           ││       (src/domain/)          │
│ ├── FileSettingsStore       ││ ├── BrowserTarget, RuleSet   │
│ ├── NativeProcessRunner     ││ ├── ValidatedUrl, RuleMatcher│
│ └── XdgBrowserDiscoverer    ││ └── Pure Validation & Errors │
└─────────────────────────────┘└──────────────────────────────┘
```

---

## 3. SOLID Principles Practical Enforcement

### 3.1. Single Responsibility Principle (SRP)
- **Domain Models & Validation (`domain/sanitizer.rs`, `domain/models.rs`)**: Responsible solely for validating URI syntax, enforcing scheme whitelists, and defending against CLI argument injection. Zero awareness of file systems or Tauri.
- **Rule Matching (`domain/matcher.rs`)**: Responsible solely for pattern matching (Exact, Domain/Wildcard, Linear-time Regex) against a `ValidatedUrl`.
- **Discovery Service (`application/discovery_service.rs`)**: Responsible solely for coordinating the enumeration of installed browsers without coupling to specific OS APIs.
- **Process Launcher (`application/launch_service.rs`)**: Responsible solely for argument tokenization and placeholder replacement (`%URL%`, `%u`, `%U`).

### 3.2. Open/Closed Principle (OCP)
- **Extensible Browser Discovery**: New platform discovery strategies (e.g., BSD desktop entries, portable browser scanners) implement the `BrowserDiscoverer` trait without requiring changes to `DiscoveryService` or UI code.
- **Storage Extensibility**: The `SettingsStore` trait allows swapping local JSON file storage for encrypted storage or custom storage backends without modifying `SettingsService`.

### 3.3. Liskov Substitution Principle (LSP)
- **Interchangeable Adapters**: `XdgBrowserDiscoverer`, `WindowsBrowserDiscoverer`, and `MacOsBrowserDiscoverer` all honor the identical `BrowserDiscoverer` contract and produce normalized `BrowserTarget` records. In tests, `MockDiscoverer` substitutes seamlessly for live platform scans.

### 3.4. Interface Segregation Principle (ISP)
- Rather than a monolithic "PlatformService" or "SystemManager", interfaces are small and focused:
  - `BrowserDiscoverer`: Only handles application discovery.
  - `ProcessRunner`: Only handles process spawning.
  - `SettingsStore`: Only handles persistence.

### 3.5. Dependency Inversion Principle (DIP)
- High-level application services (`LaunchService`, `DiscoveryService`, `SettingsService`) depend on abstract traits (`ProcessRunner`, `BrowserDiscoverer`, `SettingsStore`) rather than concrete platform or file system structs. Concrete implementations are injected at the composition root (`src-tauri/src/lib.rs`).

---

## 4. Module Responsibilities

| Module | Responsibility | Dependencies |
|---|---|---|
| `domain::models` | Core data structures (`BrowserTarget`, `ValidatedUrl`, `RuleSet`) | `serde` |
| `domain::errors` | Domain error definitions (`ValidationError`, `LaunchError`, etc.) | `thiserror` |
| `domain::sanitizer` | URL validation, scheme whitelisting, CLI flag injection defense | `url` |
| `domain::matcher` | Linear-time regex and domain routing evaluation | `regex` |
| `application::launch_service` | Safe argument substitution and process coordination | `infrastructure::traits` |
| `application::discovery_service` | Browser discovery coordination | `platform::traits` |
| `application::routing_service` | URL routing evaluation use case | `domain::models` |
| `application::settings_service` | Configuration persistence use case | `infrastructure::traits` |
| `infrastructure::file_settings_store` | Atomic file persistence (`.tmp` + sync + rename) | `std::fs`, `dirs` |
| `infrastructure::native_process_runner` | Safe direct process invocation (no shell wrappers) | `std::process::Command` |
| `platform::linux` | Dynamic XDG desktop entry scanner & icon resolver | `std::fs`, `dirs` |
| `platform::windows` | Windows StartMenuInternet adapter | `domain::models` |
| `platform::macos` | macOS Launch Services adapter | `domain::models` |
| `lib.rs` | Composition root, dependency injection, and Tauri IPC handlers | `tauri` |
