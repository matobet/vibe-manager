# Coding Conventions

**Analysis Date:** 2026-06-10

## Naming Patterns

**Files:**
- Module directories use `mod.rs` as entry point: `src/app/mod.rs`, `src/storage/repo/mod.rs`
- Files are lowercase snake_case: `report_detail.rs`, `entry_modal.rs`, `new_report.rs`
- Special data files prefixed with `_`: `_profile.md` (distinguishes metadata from journal entries)

**Functions:**
- All functions use `snake_case`: `name_to_slug`, `compute_report_summary`, `parse_entry_timestamp`
- Boolean accessors use `is_` prefix: `is_manager()`, `is_meeting()`, `is_second_level()`, `is_overdue`
- Accessor methods use noun form: `slug()`, `path()`, `entries()`, `mood()`
- Constructor functions: `new()` for standard construction, `new_with_manager()` for alternate constructors
- Serde default fns are private, prefixed with `default_`: `default_meeting_frequency()`, `default_active()`

**Variables:**
- `snake_case` throughout: `report_idx`, `entry_idx`, `mood_trend`, `days_since_meeting`
- Index variables use `_idx` suffix: `report_idx`, `entry_idx`, `last_meeting_idx`
- Repository variables use `_repo` suffix: `report_repo`, `team_repo`, `manager_repo`

**Types (structs/enums):**
- `PascalCase` for all types: `ReportProfile`, `JournalEntry`, `WorkspaceRepository`, `MoodTrend`
- Enum variants are `PascalCase`: `ViewMode::ReportDetail`, `Context::Standup`, `MoodTrend::Rising`
- Error types suffixed with `Error`: `StorageError`
- Result type aliases suffixed with `Result`: `StorageResult<T>`
- Repository types suffixed with `Repository`: `WorkspaceRepository`, `ReportRepository`, `EntryRepository`

**Constants:**
- `SCREAMING_SNAKE_CASE` for constants: `COLOR_PRIMARY`, `STATUS_MESSAGE_DURATION`, `PROFILE_FILE`, `WORKSPACE_FILE`
- Module-level constants declared with `const`: `const PROFILE_FILE: &str = "_profile.md";`

## Code Style

**Formatting:**
- `cargo fmt` (standard `rustfmt`)
- Max line width: default (100 chars)
- No trailing semicolons suppressed — `Effect::None` returned as expression without semicolon in match arms

**Linting:**
- `cargo clippy --all-features -- -D warnings` (strict, warnings are errors in CI)
- Pattern: use `matches!()` macro for single-variant checks: `matches!(self, ReportType::Manager)`
- Prefer `filter_map` over `filter` + `map`
- Use `.unwrap_or_default()` and `.unwrap_or_else()` over explicit defaults where possible

## Module Organization

**Structure:**
- Each major domain gets a subdirectory module with `mod.rs`
- Implementation split across focused submodules: `app/{mod,state,update,input}.rs`
- `mod.rs` declares submodules and handles re-exports
- `lib.rs` declares all top-level public modules

**Re-exports:**
- Each module's `mod.rs` re-exports its public API with `pub use`:
  ```rust
  // src/model/mod.rs
  pub use computed::{compute_report_summary, ReportSummary};
  pub use report::{Report, ReportProfile};
  ```
- Consumers import from the module root, not the submodule:
  ```rust
  use crate::model::{Report, ReportSummary};  // correct
  use crate::model::report::Report;            // avoid
  ```

## Import Organization

**Order (top to bottom):**
1. `std` library: `use std::path::PathBuf;`
2. External crates: `use chrono::NaiveDate;`, `use serde::{Deserialize, Serialize};`
3. Crate-internal: `use crate::model::Report;`, `use super::*;`

**Style:**
- Group `use` statements, separated from the code by a blank line
- Combine multiple items from same path: `use crate::model::{Report, ReportProfile, JournalEntry};`

## Error Handling

**Error types:**
- Domain-specific error enum using `thiserror`: `StorageError` in `src/storage/mod.rs`
- Variants use `#[from]` for automatic conversion: `#[error("IO error: {0}")] Io(#[from] std::io::Error)`
- Type alias for convenience: `pub type StorageResult<T> = Result<T, StorageError>;`

**Propagation:**
- Storage layer returns `StorageResult<T>` (typed errors)
- App layer uses `anyhow::Result` for ergonomic propagation: `pub fn update(&mut self, msg: Msg) -> Result<Effect>`
- The `?` operator used throughout for propagation

**User-facing errors:**
- `set_status(format!("Error: {}", e))` pattern for displaying errors in TUI status bar
- Error messages are human-readable strings, not error codes

**`unwrap()` usage:**
- Acceptable in `#[test]` functions: `TempDir::new().unwrap()`
- Acceptable for infallible operations known at compile time: `NaiveTime::from_hms_opt(0, 0, 0).unwrap()`
- Avoid in production code paths — use `?` or `unwrap_or_default()` instead

## Documentation Comments

**Modules:**
- Every module starts with a `//!` module-level doc comment explaining purpose
- Multi-line doc comments use `//!` throughout (not `/* */`):
  ```rust
  //! Journal entry model for meetings and mood observations
  //!
  //! A journal entry has a timestamp, optional mood...
  ```

**Types and functions:**
- Public types and non-trivial methods have `///` doc comments
- Complex functions include doc sections (`# Panics`, `# Errors`) when relevant
- Private helper functions use brief `///` comments or inline `//` comments

**Inline comments:**
- `//` for explanations of non-obvious logic
- Section dividers with `// ═══` in theme files for visual organization
- `//` comments precede the code they explain (not trailing)

## Serde Patterns

**Derive:**
- Always pair `Serialize` with `Deserialize` on YAML-backed types
- Use `#[serde(default)]` on optional fields for backwards compatibility
- Use `#[serde(rename_all = "lowercase")]` on enums written to YAML files
- Use `#[serde(skip_serializing_if = "Option::is_none")]` to keep files clean
- Private default functions for `#[serde(default = "fn_name")]`:
  ```rust
  #[serde(default = "default_meeting_frequency", alias = "cadence")]
  pub meeting_frequency: String,
  ```

## Function Design

**Size:**
- Handlers extracted into dedicated `handle_*` private methods on `App`
- Large match arms for single-dispatch messages delegate to helpers

**Parameters:**
- Use `impl Into<PathBuf>` over `&Path` or `PathBuf` for flexible constructors
- Use `impl Into<String>` for status message setters: `pub fn set_status(&mut self, message: impl Into<String>)`

**Return Values:**
- Constructors return `Result<Self>` or domain `StorageResult<Self>`
- Void mutations return `()` wrapped in `Result` when fallible
- Pure computations return the value directly (no `Result`)

## Module Design

**Visibility:**
- `pub` on the API surface types and methods users need
- `pub(crate)` for internals used across modules: `pub(crate) fn new(report_path: PathBuf) -> Self`
- Private helpers are unadorned `fn`

**Barrel files:**
- No barrel `mod.rs` that just re-exports children — each `mod.rs` contains the module declaration plus meaningful re-exports
- `lib.rs` is purely module declarations, no logic

---

*Convention analysis: 2026-06-10*
