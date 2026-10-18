# OpenIn — Threat Model & Security Specification

## 1. Scope & Objective

**OpenIn** handles untrusted input from the operating system: URLs passed via command-line arguments, deep-links (`openin://`, `openwith://`), desktop MIME launches, and browser extension communications. This document defines the threat landscape, trust boundaries, and hardening controls.

---

## 2. Trust Boundaries

```
[Untrusted: Incoming OS Arguments / Deep Links]
       │
       ▼ (Boundary 1: Scheme, Character, and Flag Validation)
[URL Ingestion & Sanitization Engine]
       │
       ▼ (Boundary 2: ReDoS Protection & Regex Safety)
[Rule Matching Engine]
       │
       ▼ (Boundary 3: Direct Executable Spawning without Shell)
[OS Process Execution] ────► [Browser / External Target Binary]
```

1. **Boundary 1: OS to OpenIn** (Untrusted inputs from external processes or browsers).
2. **Boundary 2: Frontend to Rust IPC** (Tauri Webview IPC bridge).
3. **Boundary 3: OpenIn to Host Filesystem & Subprocesses** (Executing external browser binaries).

---

## 3. Threat Analysis & Mitigations

### 3.1. Command and Argument Injection via Malformed URLs
- **Threat**: An attacker provides a URL containing argument flags (e.g. `--remote-debugging-port`, `-profile`, or shell metacharacters `&`, `|`, `;`, `` ` ``) intended to alter browser startup behavior or execute arbitrary code.
- **Mitigations**:
  - OpenIn strictly invokes binaries using `std::process::Command` without shell wrappers (`sh`, `bash`, `cmd.exe`).
  - Raw strings are treated as discrete argument elements in argv arrays; arguments are never string-concatenated with a shell interpreter.
  - URLs starting with `-` or `--` are rejected or sanitized to prevent them from being parsed as CLI options by the target browser.
  - Schemes are restricted to whitelist: `http`, `https`, `ftp`, `gemini`, `gopher`, `mailto`.

### 3.2. Catastrophic Regex Backtracking (ReDoS)
- **Threat**: A user or malicious link sets up a complex regex rule that hangs the UI thread or causes 100% CPU lockup upon link evaluation.
- **Mitigations**:
  - Use Rust's `regex` crate, which is mathematically guaranteed linear-time $O(mn)$ using finite automata (DFA/NFA) and immune to backtracking attacks.
  - Strict input size limits on rule strings.

### 3.3. Malicious Path Execution (Arbitrary Binary Execution)
- **Threat**: Tampered settings file pointing to an arbitrary binary or malicious script.
- **Mitigations**:
  - Settings file permissions restricted to the local user profile (`~/.config/openin/settings.json`).
  - Pre-execution validation verifies the binary exists, is an ordinary file (not a symlink loop or device node), and has executable permissions.
  - Shell binaries (`/bin/sh`, `/bin/bash`, `cmd.exe`, `powershell.exe`) are explicitly disallowed.

### 3.4. Tauri Frontend IPC Hardening
- **Threat**: Cross-site scripting (XSS) or injected webview content attempting to execute arbitrary host commands.
- **Mitigations**:
  - Strict Content Security Policy (CSP):
    `default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data: asset:;`
  - Tauri capability isolation: Only explicitly exposed Rust commands can be invoked from the frontend.
  - All IPC arguments undergo strong Rust-side deserialization and type/range validation.

### 3.5. Privacy & Data Leakage
- **Threat**: Sensitive parameters, tokens, session IDs, or private internal URLs logged to disk or sent over network.
- **Mitigations**:
  - Zero external telemetry, crash analytics, or tracking pings.
  - URL logging is disabled by default in release builds.
  - Sensitive query params are never retained in plain text debug logs.
