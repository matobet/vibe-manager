# Testing Patterns

**Analysis Date:** 2026-06-10

## Test Framework

**Runner:**
- Cargo's built-in test runner (`cargo test`)
- No separate test config file (uses Cargo.toml)

**Assertion Library:**
- Standard `assert!`, `assert_eq!`, `assert_ne!` macros — no third-party assertion crate

**Snapshot Library:**
- `insta` 1.0 is a declared dev-dependency in `Cargo.toml` but is not currently used (no `.snap` files exist)

**Run Commands:**
```bash
cargo test                       # Run all tests
cargo test storage               # Run storage module tests only
cargo test --all-features        # Run all tests with all features
```

## Test File Organization

**Two types of test placement:**

1. **Unit tests: inline in source files** using `#[cfg(test)]` modules at the bottom of the file
2. **Integration tests: separate files** in `tests/` directory at the project root

**Inline unit test pattern:**
```
src/model/meeting.rs        # Source file
  └── #[cfg(test)] mod tests  # Inline unit tests at bottom of same file
src/storage/mod.rs
  └── #[cfg(test)] mod tests  # Inline unit tests for parse_frontmatter
src/storage/repo/entry.rs
  └── #[cfg(test)] mod tests  # Inline unit tests for EntryRepository
src/storage/repo/report.rs
  └── #[cfg(test)] mod tests  # Inline unit tests for ReportRepository
src/storage/repo/workspace.rs
  └── #[cfg(test)] mod tests  # Inline unit tests for WorkspaceRepository
src/model/computed.rs
  └── #[cfg(test)] mod tests  # Inline unit tests for scoring functions
src/utils/slug.rs
  └── #[cfg(test)] mod tests  # Inline unit tests for name_to_slug
```

**Integration test files:**
```
tests/
├── storage_test.rs       # Storage layer integration tests (fixture-based)
├── app_test.rs           # App state management integration tests
└── fixtures/             # Test fixture workspace data
    ├── .vibe-manager     # Fixture workspace config
    ├── alex-chen/        # IC report with legacy + journal entries
    ├── chris-wong/       # Manager with team/ subdirectory
    ├── jonas/            # IC report with minimal profile
    ├── jordan-lee/       # IC report (biweekly frequency)
    └── manager-minimal/  # Manager with no team members and no manager_info
```

**Naming:**
- Test functions: `test_<what_is_being_tested>`, descriptive snake_case
  - `test_load_report_profile`, `test_manager_has_team_dir`, `test_legacy_filename_format`
- Unit test module: always named `tests`, nested under `#[cfg(test)]`
- Integration test files: `<module>_test.rs` (e.g., `storage_test.rs`, `app_test.rs`)

## Test Structure

**Inline module declaration:**
```rust
#[cfg(test)]
mod tests {
    use super::*;  // Always import parent module's items

    #[test]
    fn test_something() {
        // arrange
        let value = ...;
        // act + assert
        assert_eq!(result, expected);
    }
}
```

**Integration test module grouping** — related tests are grouped under named sub-modules:
```rust
// tests/storage_test.rs
#[cfg(test)]
mod workspace_tests {
    use super::*;
    #[test]
    fn test_is_workspace() { ... }
}

#[cfg(test)]
mod report_tests {
    use super::*;
    #[test]
    fn test_load_report_profile() { ... }
}

#[cfg(test)]
mod manager_tests {
    use super::*;
    ...
}
```

**Helper functions** are defined outside the `#[cfg(test)]` block at file scope (in integration tests) or inside the test module (in unit tests):
```rust
// Integration test helpers at file scope
fn fixtures_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

fn fixtures_repo() -> WorkspaceRepository {
    WorkspaceRepository::open(fixtures_path()).unwrap()
}
```

## Mocking

**Framework:** None — no mocking crate used.

**Strategy:** Dependency injection via real implementations:
- Storage tests use the real `WorkspaceRepository` against fixture files
- App tests use `TempDir` (from `tempfile` crate) to create writable copies of fixtures for mutation tests
- No trait objects or mock implementations

**Mutation test setup pattern:**
```rust
fn setup_temp_workspace() -> TempDir {
    let temp = TempDir::new().expect("Failed to create temp dir");
    let fixtures = fixtures_path();
    copy_dir_all(&fixtures, temp.path()).expect("Failed to copy fixtures");
    temp
}

#[test]
fn test_app_delete_entry_removes_file() {
    let temp = setup_temp_workspace();
    let mut app = vibe_manager::app::App::new(temp.path().to_path_buf())
        .expect("Failed to load app");
    // ... mutation operations on temp workspace
}
```

**What to mock:**
- Nothing is mocked — tests use real filesystem via `tempfile::TempDir`

**What NOT to mock:**
- Do not introduce mock trait objects for storage repositories — the real implementations are fast enough for unit tests

## Fixtures and Factories

**Fixture workspace** (`tests/fixtures/`):
- A real, committed workspace directory with `.vibe-manager` config
- Contains multiple report profiles covering all scenarios:
  - `alex-chen`: IC with legacy root-level entries AND timestamp entries, family info, weekly frequency
  - `jordan-lee`: IC with biweekly frequency, multiple meetings
  - `jonas`: IC with minimal profile (no title, no personal info)
  - `chris-wong`: Manager with `team/` subdirectory containing 3 members, journal entries
  - `manager-minimal`: Manager with empty team dir, no `manager_info` field
- Edge cases covered: hidden `.hidden` dir, `no-profile` dir (no `_profile.md`), legacy filename format

**In-source test factories** — used in unit tests within source files:
```rust
fn sample_profile() -> ReportProfile {
    ReportProfile {
        name: "Alex Chen".to_string(),
        title: Some("Software Engineer".to_string()),
        start_date: None,
        level: Some("P3".to_string()),
        meeting_frequency: "weekly".to_string(),
        active: true,
        report_type: ReportType::Individual,
        manager_info: None,
        birthday: None,
        partner: None,
        children: vec![],
        skills: None,
        skills_updated: None,
        color: None,
    }
}
```

**Computed summary factory** — used in `src/model/computed.rs` tests:
```rust
fn create_test_summary(mood: Option<u8>, trend: Option<MoodTrend>, overdue: bool) -> ReportSummary {
    ReportSummary {
        name: "Test".to_string(),
        level: "P3".to_string(),
        // ...all fields populated
    }
}
```

**Location:**
- Integration test fixtures: `tests/fixtures/`
- Unit test factories: private functions inside `#[cfg(test)] mod tests { ... }` in the source file

## Coverage

**Requirements:** None enforced — no minimum coverage threshold configured.

**Coverage tools:** Not configured (no `cargo-tarpaulin` or `cargo-llvm-cov` setup in `Cargo.toml` or CI config visible).

## Test Types

**Unit Tests (inline in source):**
- Test pure functions and methods in isolation
- Cover parsing (`parse_entry_timestamp`, `parse_frontmatter`), transformations (`name_to_slug`), and logic (`calculate_urgency_score`, `is_meeting()`, context cycle)
- Use `NaiveDateTime::default()` and `PathBuf::new()` as zero-value placeholders for irrelevant fields

**Integration Tests (in `tests/`):**
- Test full workflows end-to-end against real filesystem
- `storage_test.rs`: Covers all repository types (workspace, report, entry, manager hierarchy) using read-only fixture workspace
- `app_test.rs`: Covers `App` state transitions, delete operations, modal workflows using mutable temp workspace copies

## Common Patterns

**Async Testing:**
- Not applicable — all code is synchronous, no async tests.

**Error Testing:**
```rust
// Asserting Ok
let result = repo.load().unwrap();

// Asserting Err
let result = WorkspaceRepository::init(path);
assert!(result.is_err());
```

**Filesystem-dependent tests:**
```rust
// Always use TempDir for mutable tests — never mutate fixtures directly
let temp = TempDir::new().unwrap();  // auto-deleted on drop
let path = temp.path().join("alex-chen");
```

**Fixture-referencing tests:**
```rust
// Use CARGO_MANIFEST_DIR for portable paths
fn fixtures_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}
```

**Boundary/edge case tests:**
- Hidden directories (`.hidden`) are tested for exclusion
- Directories without `_profile.md` (`no-profile`) are tested for exclusion
- Legacy filename format (`YYYY-MM-DD.md`) tested alongside new timestamp format
- Empty/minimal profiles tested alongside full profiles
- Out-of-bounds indices: `assert!(app.meeting_display_to_entry_index(1000).is_none())`

**App workflow tests** — test full message sequences through `App::update()`:
```rust
app.update(Msg::ShowEntryInput).unwrap();
assert_eq!(app.view_mode, ViewMode::EntryInputModal);
app.update(Msg::SetEntryMood(4)).unwrap();
app.update(Msg::SaveEntry).unwrap();
assert_eq!(app.view_mode, ViewMode::ReportDetail);
assert_eq!(app.entries_by_report[report_idx].len(), initial_count + 1);
```

---

*Testing analysis: 2026-06-10*
