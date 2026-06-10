# Technology Stack

**Analysis Date:** 2026-06-10

## Languages

**Primary:**
- Rust 1.92.0 (2021 edition) - All application code in `src/`

## Runtime

**Environment:**
- Native binary - compiled to platform-native executable, no managed runtime

**Package Manager:**
- Cargo 1.92.0
- Lockfile: `Cargo.lock` - present and committed

## Frameworks

**Core:**
- `ratatui` 0.29 - Terminal UI framework (widgets, layout, rendering)
- `crossterm` 0.28 - Cross-platform terminal control (raw mode, alternate screen, mouse capture, key events)
- `clap` 4.0 (derive feature) - CLI argument parsing (`vibe-manager [path]` and `vibe-manager init [path]`)

**Serialization:**
- `serde` 1.0 (derive feature) - Serialization/deserialization framework used on all model types
- `serde_yaml` 0.9 - YAML parsing for workspace config (`.vibe-manager`) and markdown frontmatter in `_profile.md` and journal entry files

**Date/Time:**
- `chrono` 0.4 (serde feature) - Date/time types used throughout model and storage layers; `NaiveDate`, `NaiveDateTime`, `NaiveTime`, `Local`
- `chrono-humanize` 0.2 - Human-readable relative time strings (e.g., "2 weeks ago") used in `src/theme/rpg.rs`

**Filesystem:**
- `dirs` 5.0 - Platform-aware directory resolution (declared as dependency; not observed in active use as of this analysis)
- `walkdir` 2.0 - Recursive directory traversal (declared as dependency; not observed in active use as of this analysis — workspace uses `std::fs::read_dir` instead)

**Error Handling:**
- `thiserror` 1.0 - Derive macros for `StorageError` enum in `src/storage/mod.rs`
- `anyhow` 1.0 - Top-level error propagation in `main.rs`, `app/`, and `editor.rs`

**Text:**
- `unicode-width` 0.1 - Unicode character width calculation (declared dependency; not observed in direct source imports)

**Testing:**
- `tempfile` 3.0 - Temporary directories for mutation-safe integration tests (`tests/app_test.rs`, `src/storage/repo/workspace.rs` unit tests)
- `insta` 1.0 - Snapshot testing framework (declared dev-dependency; no snapshot calls observed in current test files)

## Key Dependencies

**Critical:**
- `ratatui` 0.29 - All UI rendering depends on this; version upgrades require API review
- `crossterm` 0.28 - Terminal lifecycle (raw mode, alternate screen) tied directly to `main.rs` startup/teardown
- `serde_yaml` 0.9 - All persistent data reads/writes use YAML frontmatter; version 0.9 is the last release of this crate (maintenance status: read-only)
- `chrono` 0.4 - Date parsing for journal entry filenames (`YYYY-MM-DDTHHMMSS.md`) and all meeting recency calculations

**Infrastructure:**
- `clap` 4.0 - CLI entrypoint; defines `init` subcommand and default path argument
- `thiserror` / `anyhow` - Dual error strategy: typed `StorageError` in storage layer, `anyhow::Result` at application boundary

## Configuration

**Environment:**
- `$EDITOR` / `$VISUAL` - Checked at runtime in `src/editor.rs` to select external editor for note editing; fallback chain: `$EDITOR` → `$VISUAL` → `nano` → `vim` → `vi`
- No `.env` files — application carries no secrets and requires no external service credentials

**Build:**
- `Cargo.toml` - Single workspace manifest at project root
- `.cargo/config.toml` - Defines `cargo cov` alias for `cargo llvm-cov --all-features --html --open`
- Release profile: LTO enabled, `codegen-units = 1`, binary stripping (`strip = true`)

## Platform Requirements

**Development:**
- Rust stable toolchain (1.92.0 or newer)
- `cargo-llvm-cov` for coverage reports (installed via CI via `taiki-e/install-action`)
- A Unix-compatible terminal for running the TUI; `$EDITOR` or `$VISUAL` set for note editing

**Production:**
- Pre-compiled native binaries distributed per platform:
  - Linux x86_64 (musl static)
  - Linux aarch64 (musl static, cross-compiled via `cross`)
  - macOS x86_64
  - macOS aarch64 (Apple Silicon)
  - Windows x86_64 (MSVC)
- No runtime dependencies beyond the OS terminal
- Workspace directory with a `.vibe-manager` config file required at launch

---

*Stack analysis: 2026-06-10*
