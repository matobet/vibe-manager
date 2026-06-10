---
phase: 1
slug: doorway-cards
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-06-10
---

# Phase 1 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | cargo test (Rust built-in) + insta 1.x snapshots |
| **Config file** | Cargo.toml (insta declared; no snapshot tests exist yet — Wave 0 creates first) |
| **Quick run command** | `cargo test computed` |
| **Full suite command** | `cargo test --all-features` |
| **Estimated runtime** | ~30 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cargo test computed`
- **After every plan wave:** Run `cargo test --all-features` plus `cargo clippy --all-features -- -D warnings` and `cargo fmt --all -- --check`
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 60 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| (populated by planner) | — | — | DOOR-01..04 | — | N/A (local-only TUI, no untrusted input) | unit + snapshot | `cargo test --all-features` | ✅ | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] First `insta` snapshot test using `ratatui::backend::TestBackend` with constructed literals (NOT fixture dates — fixtures use Jan 2026, time-dependent)
- [ ] Unit-test scaffolding for `OutlierInfo` / `manager_urgency_bonus` in `src/model/computed.rs`

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Glyph rendering in common monospace fonts (hearts, ⚠, ★, ▸, health bar) | DOOR-01, DOOR-02 | Font rendering is terminal-dependent; snapshot tests catch layout but not glyph width in a real terminal | `cargo run -- tests/fixtures/<team>` and visually inspect manager doorway cards |
| Card height stability when selection moves | DOOR-03 | Visual confirmation of no jitter | Navigate dashboard with arrows; confirm unselected manager cards keep blank fourth line |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 60s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
