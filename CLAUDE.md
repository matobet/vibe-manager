# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Vibe Manager is a Rust TUI application for engineering managers to track 1-on-1 meetings, team health, and career progress. It uses an 8-bit RPG aesthetic with ratatui for terminal rendering.

See ./docs for product documentation.

## Build Commands

```bash
cargo build                           # Debug build
cargo build --release                 # Release build (LTO enabled)
cargo test --all-features             # Run all tests
cargo clippy --all-features -- -D warnings  # Lint (CI uses strict mode)
cargo fmt --all -- --check            # Check formatting
cargo run -- init /path/to/team       # Initialize new workspace
cargo run -- /path/to/team            # Run TUI on existing workspace
```

## Architecture

### TEA Pattern (The Elm Architecture)
- **Model**: `App` struct in `app.rs` holds all application state
- **Update**: `App::update(msg)` processes `Msg` enum variants
- **View**: `views/` modules render based on `ViewMode`

### Module Structure
- `app.rs` - Application state, key handling, message dispatch
- `model/` - Data structures (Report, ReportProfile, JournalEntry, Workspace)
- `storage/` - File I/O, YAML frontmatter parsing, workspace loading
- `components/` - Reusable UI widgets (avatar, dashboard, report_detail, modal)
- `views/` - Full-screen layouts (dashboard_view, detail_view)
- `theme/rpg.rs` - 8-bit color palette and styling

### Data Model (Pure Markdown)
```
workspace/
├── .vibe-manager           # Workspace config (YAML)
├── alex-chen/
│   ├── _profile.md         # Report profile (YAML frontmatter + markdown)
│   ├── 2026-01-15.md       # Legacy meeting (supported at root)
│   ├── journal/            # New journal entries
│   │   └── 2026-01-15T143000.md  # Meeting or mood observation
│   └── team/               # For managers: 2nd-level reports
│       └── sam-taylor/
│           ├── _profile.md # 2nd-level report
│           └── journal/    # Skip-level meeting notes
```

- Folders = reports (slug derived from name)
- Files = meetings (filename is date YYYY-MM-DD.md or timestamp YYYY-MM-DDTHHMMSS.md)
- Legacy entries at root level still supported, new entries in `journal/`
- `team/` subdirectory = manager with 2nd-level reports
- Computed fields (overdue status, mood trends, team metrics) calculated at runtime

### Key Types
- `ViewMode` enum: Dashboard, EngineerDetail, NoteViewer, NewEngineerModal, EntryInputModal, DeleteConfirmModal, Help
- `Msg` enum: All state update messages
- `Report`: slug, path, profile, notes_content, team (for managers)
- `ReportProfile`: name, level (P1-P5 or M1-M5), report_type (ic/manager), meeting_frequency, skills
- `ReportSummary`: computed metrics including team_metrics for managers
- `JournalEntry`: timestamp, path, content, mood, context

## Testing

Integration tests use fixtures in `tests/fixtures/` containing sample team data. Snapshot testing via `insta` crate.

```bash
cargo test                            # Run all tests
cargo test storage                    # Run storage tests only
```

## Documentation Maintenance

The `docs/` folder contains product specifications written before/during development. Keep these in sync with implementation:

- **docs/implementation-status.md** - Quick reference of what's implemented vs planned. Update this when completing features.
- **docs/features/*.md** - Each has an "Implementation Status" table at the top. Update status when implementing requirements.
- **docs/roadmap.md** - Phase status and feature tables. Mark features as done when complete.

When implementing a feature:
1. Check the relevant feature spec in `docs/features/` for requirements
2. After implementation, update the Implementation Status table in that doc
3. Update `docs/implementation-status.md` summary
4. If completing a roadmap item, update `docs/roadmap.md`

<!-- GSD:project-start source:PROJECT.md -->
## Project

**Vibe Manager**

A Rust TUI application for engineering managers to track 1-on-1 meetings, team
health, and career progress — rendered with an 8-bit RPG aesthetic (ratatui).
Single-user, local-only, pure-markdown storage: the workspace folder *is* the
database. Built by and for its author to run their actual team.

**Core Value:** Never lose track of a team member's wellbeing or meeting cadence — including
the 2nd-level reports behind your manager reports.

### Constraints

- **Tech stack**: Rust + ratatui, single binary, no new runtime dependencies expected — the milestone is UI/state work
- **Storage**: Pure markdown/YAML files; rotation and metrics stay computed, never stored
- **Quality gates**: CI enforces `cargo clippy --all-features -- -D warnings` (clippy 1.96+) and `cargo fmt --check`; tests use insta snapshots + fixtures in `tests/fixtures/`
- **Compatibility**: Legacy root-level meeting files must keep working; existing keybindings stay stable except the conscious Space rebind
- **Terminal**: Glyphs must render in common monospace fonts; `◦︵◦` alignment caveat already documented
<!-- GSD:project-end -->

<!-- GSD:stack-start source:codebase/STACK.md -->
## Technology Stack

## Languages
- Rust 1.92.0 (2021 edition) - All application code in `src/`
## Runtime
- Native binary - compiled to platform-native executable, no managed runtime
- Cargo 1.92.0
- Lockfile: `Cargo.lock` - present and committed
## Frameworks
- `ratatui` 0.29 - Terminal UI framework (widgets, layout, rendering)
- `crossterm` 0.28 - Cross-platform terminal control (raw mode, alternate screen, mouse capture, key events)
- `clap` 4.0 (derive feature) - CLI argument parsing (`vibe-manager [path]` and `vibe-manager init [path]`)
- `serde` 1.0 (derive feature) - Serialization/deserialization framework used on all model types
- `serde_yaml` 0.9 - YAML parsing for workspace config (`.vibe-manager`) and markdown frontmatter in `_profile.md` and journal entry files
- `chrono` 0.4 (serde feature) - Date/time types used throughout model and storage layers; `NaiveDate`, `NaiveDateTime`, `NaiveTime`, `Local`
- `chrono-humanize` 0.2 - Human-readable relative time strings (e.g., "2 weeks ago") used in `src/theme/rpg.rs`
- `dirs` 5.0 - Platform-aware directory resolution (declared as dependency; not observed in active use as of this analysis)
- `walkdir` 2.0 - Recursive directory traversal (declared as dependency; not observed in active use as of this analysis — workspace uses `std::fs::read_dir` instead)
- `thiserror` 1.0 - Derive macros for `StorageError` enum in `src/storage/mod.rs`
- `anyhow` 1.0 - Top-level error propagation in `main.rs`, `app/`, and `editor.rs`
- `unicode-width` 0.1 - Unicode character width calculation (declared dependency; not observed in direct source imports)
- `tempfile` 3.0 - Temporary directories for mutation-safe integration tests (`tests/app_test.rs`, `src/storage/repo/workspace.rs` unit tests)
- `insta` 1.0 - Snapshot testing framework (declared dev-dependency; no snapshot calls observed in current test files)
## Key Dependencies
- `ratatui` 0.29 - All UI rendering depends on this; version upgrades require API review
- `crossterm` 0.28 - Terminal lifecycle (raw mode, alternate screen) tied directly to `main.rs` startup/teardown
- `serde_yaml` 0.9 - All persistent data reads/writes use YAML frontmatter; version 0.9 is the last release of this crate (maintenance status: read-only)
- `chrono` 0.4 - Date parsing for journal entry filenames (`YYYY-MM-DDTHHMMSS.md`) and all meeting recency calculations
- `clap` 4.0 - CLI entrypoint; defines `init` subcommand and default path argument
- `thiserror` / `anyhow` - Dual error strategy: typed `StorageError` in storage layer, `anyhow::Result` at application boundary
## Configuration
- `$EDITOR` / `$VISUAL` - Checked at runtime in `src/editor.rs` to select external editor for note editing; fallback chain: `$EDITOR` → `$VISUAL` → `nano` → `vim` → `vi`
- No `.env` files — application carries no secrets and requires no external service credentials
- `Cargo.toml` - Single workspace manifest at project root
- `.cargo/config.toml` - Defines `cargo cov` alias for `cargo llvm-cov --all-features --html --open`
- Release profile: LTO enabled, `codegen-units = 1`, binary stripping (`strip = true`)
## Platform Requirements
- Rust stable toolchain (1.92.0 or newer)
- `cargo-llvm-cov` for coverage reports (installed via CI via `taiki-e/install-action`)
- A Unix-compatible terminal for running the TUI; `$EDITOR` or `$VISUAL` set for note editing
- Pre-compiled native binaries distributed per platform:
- No runtime dependencies beyond the OS terminal
- Workspace directory with a `.vibe-manager` config file required at launch
<!-- GSD:stack-end -->

<!-- GSD:conventions-start source:CONVENTIONS.md -->
## Conventions

## Naming Patterns
- Module directories use `mod.rs` as entry point: `src/app/mod.rs`, `src/storage/repo/mod.rs`
- Files are lowercase snake_case: `report_detail.rs`, `entry_modal.rs`, `new_report.rs`
- Special data files prefixed with `_`: `_profile.md` (distinguishes metadata from journal entries)
- All functions use `snake_case`: `name_to_slug`, `compute_report_summary`, `parse_entry_timestamp`
- Boolean accessors use `is_` prefix: `is_manager()`, `is_meeting()`, `is_second_level()`, `is_overdue`
- Accessor methods use noun form: `slug()`, `path()`, `entries()`, `mood()`
- Constructor functions: `new()` for standard construction, `new_with_manager()` for alternate constructors
- Serde default fns are private, prefixed with `default_`: `default_meeting_frequency()`, `default_active()`
- `snake_case` throughout: `report_idx`, `entry_idx`, `mood_trend`, `days_since_meeting`
- Index variables use `_idx` suffix: `report_idx`, `entry_idx`, `last_meeting_idx`
- Repository variables use `_repo` suffix: `report_repo`, `team_repo`, `manager_repo`
- `PascalCase` for all types: `ReportProfile`, `JournalEntry`, `WorkspaceRepository`, `MoodTrend`
- Enum variants are `PascalCase`: `ViewMode::ReportDetail`, `Context::Standup`, `MoodTrend::Rising`
- Error types suffixed with `Error`: `StorageError`
- Result type aliases suffixed with `Result`: `StorageResult<T>`
- Repository types suffixed with `Repository`: `WorkspaceRepository`, `ReportRepository`, `EntryRepository`
- `SCREAMING_SNAKE_CASE` for constants: `COLOR_PRIMARY`, `STATUS_MESSAGE_DURATION`, `PROFILE_FILE`, `WORKSPACE_FILE`
- Module-level constants declared with `const`: `const PROFILE_FILE: &str = "_profile.md";`
## Code Style
- `cargo fmt` (standard `rustfmt`)
- Max line width: default (100 chars)
- No trailing semicolons suppressed — `Effect::None` returned as expression without semicolon in match arms
- `cargo clippy --all-features -- -D warnings` (strict, warnings are errors in CI)
- Pattern: use `matches!()` macro for single-variant checks: `matches!(self, ReportType::Manager)`
- Prefer `filter_map` over `filter` + `map`
- Use `.unwrap_or_default()` and `.unwrap_or_else()` over explicit defaults where possible
## Module Organization
- Each major domain gets a subdirectory module with `mod.rs`
- Implementation split across focused submodules: `app/{mod,state,update,input}.rs`
- `mod.rs` declares submodules and handles re-exports
- `lib.rs` declares all top-level public modules
- Each module's `mod.rs` re-exports its public API with `pub use`:
- Consumers import from the module root, not the submodule:
## Import Organization
- Group `use` statements, separated from the code by a blank line
- Combine multiple items from same path: `use crate::model::{Report, ReportProfile, JournalEntry};`
## Error Handling
- Domain-specific error enum using `thiserror`: `StorageError` in `src/storage/mod.rs`
- Variants use `#[from]` for automatic conversion: `#[error("IO error: {0}")] Io(#[from] std::io::Error)`
- Type alias for convenience: `pub type StorageResult<T> = Result<T, StorageError>;`
- Storage layer returns `StorageResult<T>` (typed errors)
- App layer uses `anyhow::Result` for ergonomic propagation: `pub fn update(&mut self, msg: Msg) -> Result<Effect>`
- The `?` operator used throughout for propagation
- `set_status(format!("Error: {}", e))` pattern for displaying errors in TUI status bar
- Error messages are human-readable strings, not error codes
- Acceptable in `#[test]` functions: `TempDir::new().unwrap()`
- Acceptable for infallible operations known at compile time: `NaiveTime::from_hms_opt(0, 0, 0).unwrap()`
- Avoid in production code paths — use `?` or `unwrap_or_default()` instead
## Documentation Comments
- Every module starts with a `//!` module-level doc comment explaining purpose
- Multi-line doc comments use `//!` throughout (not `/* */`):
- Public types and non-trivial methods have `///` doc comments
- Complex functions include doc sections (`# Panics`, `# Errors`) when relevant
- Private helper functions use brief `///` comments or inline `//` comments
- `//` for explanations of non-obvious logic
- Section dividers with `// ═══` in theme files for visual organization
- `//` comments precede the code they explain (not trailing)
## Serde Patterns
- Always pair `Serialize` with `Deserialize` on YAML-backed types
- Use `#[serde(default)]` on optional fields for backwards compatibility
- Use `#[serde(rename_all = "lowercase")]` on enums written to YAML files
- Use `#[serde(skip_serializing_if = "Option::is_none")]` to keep files clean
- Private default functions for `#[serde(default = "fn_name")]`:
## Function Design
- Handlers extracted into dedicated `handle_*` private methods on `App`
- Large match arms for single-dispatch messages delegate to helpers
- Use `impl Into<PathBuf>` over `&Path` or `PathBuf` for flexible constructors
- Use `impl Into<String>` for status message setters: `pub fn set_status(&mut self, message: impl Into<String>)`
- Constructors return `Result<Self>` or domain `StorageResult<Self>`
- Void mutations return `()` wrapped in `Result` when fallible
- Pure computations return the value directly (no `Result`)
## Module Design
- `pub` on the API surface types and methods users need
- `pub(crate)` for internals used across modules: `pub(crate) fn new(report_path: PathBuf) -> Self`
- Private helpers are unadorned `fn`
- No barrel `mod.rs` that just re-exports children — each `mod.rs` contains the module declaration plus meaningful re-exports
- `lib.rs` is purely module declarations, no logic
<!-- GSD:conventions-end -->

<!-- GSD:architecture-start source:ARCHITECTURE.md -->
## Architecture

## System Overview
```text
```
## Component Responsibilities
| Component | Responsibility | File |
|-----------|----------------|------|
| `main.rs` | CLI, terminal setup/teardown, event loop, editor suspension | `src/main.rs` |
| `app::App` | All application state (model in TEA) | `src/app/mod.rs`, `src/app/state.rs` |
| `App::update()` | Process `Msg` → mutate state → return `Effect` (update in TEA) | `src/app/update.rs` |
| `handle_key_event()` | Map keyboard events to `Msg` per `ViewMode` | `src/app/input.rs` |
| `views/` | Full-screen layouts; compose components; pick correct view per mode | `src/views/dashboard_view.rs`, `src/views/detail_view.rs` |
| `components/` | Reusable rendering widgets (avatars, tables, modals, charts) | `src/components/` |
| `model/` | Pure data structs; computed summaries; no I/O | `src/model/` |
| `storage/repo/` | File I/O, YAML frontmatter parsing, markdown serialization | `src/storage/repo/` |
| `editor.rs` | Suspend TUI, delegate to `$EDITOR`, detect modification | `src/editor.rs` |
| `theme/rpg.rs` | Color palette constants, shared style helpers | `src/theme/rpg.rs` |
| `theme/sprites.rs` | Kaomoji avatar sprites keyed on level and mood | `src/theme/sprites.rs` |
| `utils/` | `name_to_slug()`, color hashing, hex color parsing | `src/utils/` |
## Pattern Overview
- `App` is the single source of truth for all UI state; no distributed state
- `update()` returns `Effect` values instead of executing side effects directly — keeps update pure
- Views receive `&App` as read-only; they never mutate state
- All I/O is isolated inside `storage/repo/`; model structs carry no file handles
- External editor integration suspends/resumes the TUI terminal cleanly around a child process
## Layers
- Purpose: Terminal lifecycle, event polling, effect execution
- Location: `src/main.rs`
- Contains: `run_app()`, `suspend_and_edit()`, CLI via `clap`
- Depends on: `app`, `views`, `storage`, `editor`
- Used by: (entry point — nothing uses it)
- Purpose: Application state machine
- Location: `src/app/`
- Contains: `App` struct, `Msg` enum, `ViewMode` enum, `Effect` enum, key handler
- Depends on: `model`, `storage`, `components::modal`
- Used by: `main.rs`, `views`
- Purpose: Full-screen layout composition
- Location: `src/views/`
- Contains: `render_dashboard_view()`, `render_detail_view()`, `render_viewer_view()`
- Depends on: `app`, `components`, `model`
- Used by: `main.rs` render loop
- Purpose: Reusable ratatui widgets
- Location: `src/components/`
- Contains: `Dashboard`, `AvatarGrid`, `ReportDetail`, `NoteViewer`, `StatusBar`, modal structs
- Depends on: `model`, `theme`
- Used by: `views`
- Purpose: Data structures and pure computation
- Location: `src/model/`
- Contains: `Report`, `ReportProfile`, `JournalEntry`, `Workspace`, `ReportSummary`, `TeamMetrics`
- Depends on: nothing (pure Rust + serde)
- Used by: all layers
- Purpose: Filesystem access; YAML frontmatter serialization/deserialization
- Location: `src/storage/`
- Contains: `WorkspaceRepository`, `ReportRepository`, `EntryRepository`, `parse_frontmatter()`
- Depends on: `model`, `utils`
- Used by: `app`
- Purpose: Visual constants and style helpers
- Location: `src/theme/`
- Contains: Color constants (`COLOR_PRIMARY`, etc.), `style_*()` functions, avatar sprite logic
- Depends on: `model` (sprites use `ReportSummary`)
- Used by: `components`
- Purpose: Stateless pure helpers
- Location: `src/utils/`
- Contains: `name_to_slug()`, `color_from_name()`, `report_color()`, hex color parsing
- Depends on: nothing
- Used by: `model`, `storage`, `components`
## Data Flow
### Primary Request Path (User Keystroke → State Update)
### Data Load Path (Startup / Refresh)
### External Editor Path
- All mutable state lives in one `App` struct on the stack in `main.rs`
- No `Arc<Mutex<>>` or shared state; the app is single-threaded
- Computed summaries (`ReportSummary`, `WorkspaceSummary`) are re-derived from source data after any mutation
## Key Abstractions
- Purpose: All possible state-changing actions expressed as data
- Examples: `src/app/mod.rs:62`
- Pattern: TEA message passing — input layer produces `Msg`, update layer consumes it
- Purpose: Side effects the runtime must perform (currently only `SpawnEditor`)
- Examples: `src/app/mod.rs:34`
- Pattern: Effect system keeping `update()` free of I/O
- Purpose: Controls which view and key-handler branch is active
- Examples: `src/app/mod.rs:43`
- Pattern: State machine — each variant has distinct rendering and input semantics
- Purpose: Domain-scoped file I/O with typed return values
- Examples: `src/storage/repo/workspace.rs`, `src/storage/repo/report.rs`, `src/storage/repo/entry.rs`
- Pattern: Repository pattern; callers never touch `fs::` directly
- Purpose: Computed view-models derived at load time and after mutations
- Examples: `src/model/computed.rs`
- Pattern: Derived/computed state — not stored on disk, recalculated from `Report` + `Vec<JournalEntry>`
## Entry Points
- Location: `src/main.rs:86` (`run_tui`)
- Triggers: `vibe-manager <path>` with no subcommand
- Responsibilities: Opens workspace, sets up terminal, runs event loop
- Location: `src/main.rs:61` (`init_workspace`)
- Triggers: `vibe-manager init <path>`
- Responsibilities: Creates `.vibe-manager` config file and workspace directory
## Architectural Constraints
- **Threading:** Single-threaded event loop; no async runtime; no threads spawned by the app (editor is a child process, not a thread)
- **Global state:** No module-level globals; all state flows through the `App` struct passed by mutable reference
- **Circular imports:** None detected; dependency direction is strictly `main → app → model ← storage`, with `views` and `components` depending on `app` and `model` only
- **Terminal ownership:** `main.rs` owns the `Terminal`; it is suspended around editor invocations and restored before resuming the render loop
- **Frontmatter format:** All on-disk data uses YAML frontmatter (`---`…`---`) atop Markdown; `parse_frontmatter()` in `src/storage/mod.rs:31` is the single parsing entry point
## Anti-Patterns
### Bypassing the Repository Layer
### Mutating `App` State Inside View/Component Functions
### Skipping Computed Summary Refresh
## Error Handling
- Storage layer returns `StorageResult<T>` (`StorageError` variants: `Io`, `Yaml`, `InvalidWorkspace`, `ProfileNotFound`)
- `App::update()` returns `anyhow::Result<Effect>`; I/O errors from storage propagate up to `run_app()` which exits
- User-visible non-fatal errors (e.g., "Error saving mood") call `app.set_status(msg)` which displays for 3 seconds and clears automatically
## Cross-Cutting Concerns
<!-- GSD:architecture-end -->

<!-- GSD:skills-start source:skills/ -->
## Project Skills

No project skills found. Add skills to any of: `.claude/skills/`, `.agents/skills/`, `.cursor/skills/`, `.github/skills/`, or `.codex/skills/` with a `SKILL.md` index file.
<!-- GSD:skills-end -->

<!-- GSD:workflow-start source:GSD defaults -->
## GSD Workflow Enforcement

Before using Edit, Write, or other file-changing tools, start work through a GSD command so planning artifacts and execution context stay in sync.

Use these entry points:
- `/gsd-quick` for small fixes, doc updates, and ad-hoc tasks
- `/gsd-debug` for investigation and bug fixing
- `/gsd-execute-phase` for planned phase work

Do not make direct repo edits outside a GSD workflow unless the user explicitly asks to bypass it.
<!-- GSD:workflow-end -->

<!-- GSD:profile-start -->
## Developer Profile

> Profile not yet configured. Run `/gsd-profile-user` to generate your developer profile.
> This section is managed by `generate-claude-profile` -- do not edit manually.
<!-- GSD:profile-end -->
