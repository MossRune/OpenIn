# ADR 0001: Architecture, Technology Stack, and Security Boundaries

## Status
Accepted

## Context
The goal is to engineer **OpenIn**, a high-performance desktop link chooser and browser router, as an independent, secure, and cross-platform application in Rust.

## Decision

1. **Core Runtime**: Rust (Edition 2024) with Tauri 2.
   - **Rationale**: Memory safety, minimal memory footprint (< 30 MB idle), instant startup (< 100 ms), and strong cross-platform support across Linux, Windows, and macOS.
2. **Frontend Layer**: Vanilla TypeScript + Vite + Modern CSS tokens.
   - **Rationale**: Avoid heavy UI framework runtime overhead (React/Vue/Angular), eliminating hydration delays on the critical link opening path.
3. **Process Execution**: Direct process spawning using `std::process::Command` without shell wrappers.
   - **Rationale**: Eliminates shell injection vulnerabilities and arbitrary command execution.
4. **Regex Engine**: Standard Rust `regex` crate with guaranteed $O(mn)$ linear time.
   - **Rationale**: ReDoS immunity, preventing catastrophic backtracking.
5. **Persistence**: Atomic file write via `.tmp` file and rename.
   - **Rationale**: Guarantees configuration integrity even in case of abrupt system shutdown.

## Consequences
- Requires Tauri 2 prerequisites and WebKitGTK / WebView2 depending on platform.
- Clean separation between platform adapters and domain business logic.
