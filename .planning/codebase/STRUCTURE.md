# Codebase Structure

**Analysis Date:** 2026-06-10

## Directory Layout

```
vibe-manager/
├── src/
│   ├── main.rs               # Binary entry point: CLI, terminal setup, event loop
│   ├── lib.rs                # Library root: re-exports all public modules
│   ├── app/                  # TEA core — application state machine
│   │   ├── mod.rs            # App struct, Msg enum, ViewMode enum, Effect enum
│   │   ├── state.rs          # App::new(), load_data(), status helpers, delete_entry()
│   │   ├── update.rs         # App::update(Msg) → Effect (all state transitions)
│   │   └── input.rs          # handle_key_event(), poll_event()
│   ├── model/                # Pure data structures (no I/O)
│   │   ├── mod.rs            # Re-exports all model types
│   │   ├── report.rs         # Report, ReportProfile, Level, ReportType, Skills
│   │   ├── meeting.rs        # JournalEntry, JournalEntryFrontmatter, Context
│   │   ├── computed.rs       # ReportSummary, TeamMetrics, WorkspaceSummary + compute_*
│   │   └── workspace.rs      # Workspace, WorkspaceConfig, WorkspaceSettings
│   ├── storage/              # File I/O layer
│   │   ├── mod.rs            # StorageError, StorageResult, parse_frontmatter()
│   │   └── repo/
│   │       ├── mod.rs        # Re-exports WorkspaceRepository, ReportRepository, EntryRepository
│   │       ├── workspace.rs  # WorkspaceRepository: init/open/load/list_reports/create_report
│   │       ├── report.rs     # ReportRepository: load/save/list_team_members/entries()
│   │       └── entry.rs      # EntryRepository: list/create_meeting/create_observation/save/delete
│   ├── views/                # Full-screen layout composition
│   │   ├── mod.rs            # Re-exports
│   │   ├── dashboard_view.rs # render_dashboard_view() — team overview + modals
│   │   └── detail_view.rs    # render_detail_view(), render_viewer_view()
│   ├── components/           # Reusable ratatui widgets
│   │   ├── mod.rs            # Re-exports all public components
│   │   ├── dashboard.rs      # Dashboard widget, render_vibe_manager_title(), render_empty_state()
│   │   ├── avatar.rs         # AvatarCard, AvatarGrid (kaomoji sprites)
│   │   ├── report_detail.rs  # ReportDetail widget (entries list, profile, mood chart)
│   │   ├── note_viewer.rs    # NoteViewer widget (meeting content viewer)
│   │   ├── status_bar.rs     # StatusBar widget (bottom context bar)
│   │   ├── mood_chart.rs     # render_mood_chart_with_axis()
│   │   ├── entry_modal.rs    # EntryInputModal (quick mood observation form)
│   │   ├── delete_modal.rs   # DeleteConfirmModal
│   │   └── modal/
│   │       ├── mod.rs        # render_modal() helper, re-exports
│   │       ├── new_report.rs # NewReportModal, NewReportState, NewReportField
│   │       └── help.rs       # HelpModal
│   ├── theme/                # Visual constants and styling
│   │   ├── mod.rs            # Re-exports rpg::*
│   │   ├── rpg.rs            # Color constants, style_*() functions, rpg_block(), mood_gauge()
│   │   └── sprites.rs        # Kaomoji face/frame rendering by level and mood
│   ├── utils/                # Stateless helpers
│   │   ├── mod.rs            # color_from_name(), report_color(), parse_hex_color()
│   │   └── slug.rs           # name_to_slug()
│   └── editor.rs             # External editor delegation (get_editor, edit_file)
├── tests/
│   ├── app_test.rs           # Integration tests for app state
│   ├── storage_test.rs       # Integration tests for storage layer
│   └── fixtures/             # Sample workspace data for tests
│       ├── alex-chen/        # IC with legacy root-level entries
│       ├── chris-wong/       # Manager with team/ subdirectory
│       │   └── team/         # 2nd-level reports (lee-kim, morgan-smith, robin-patel)
│       ├── jordan-lee/       # Another IC fixture
│       ├── jonas/            # IC fixture
│       └── manager-minimal/  # Minimal manager fixture
├── docs/                     # Product specifications and roadmap
│   ├── features/             # Per-feature specs with implementation status tables
│   ├── mockups/              # UI wireframes
│   └── roadmap.md            # Phase status and feature tables
├── .github/
│   └── workflows/            # CI pipelines
├── Cargo.toml                # Package manifest and dependencies
├── Cargo.lock                # Locked dependency tree
└── CLAUDE.md                 # Project guidance for Claude Code
```

## Directory Purposes

**`src/app/`:**
- Purpose: The TEA state machine core
- Contains: Application state (`App`), message types (`Msg`), view mode enum (`ViewMode`), effects (`Effect`), key event dispatch
- Key files: `src/app/mod.rs` (type definitions), `src/app/update.rs` (all state transitions), `src/app/input.rs` (key → Msg mapping)

**`src/model/`:**
- Purpose: Pure data structures with no I/O dependencies
- Contains: Domain types for reports, journal entries, workspaces, computed summaries
- Key files: `src/model/report.rs`, `src/model/meeting.rs`, `src/model/computed.rs`

**`src/storage/repo/`:**
- Purpose: All filesystem access, scoped to domain repositories
- Contains: Three repository types with clearly scoped responsibilities
- Key files: `src/storage/repo/entry.rs` (entry CRUD), `src/storage/repo/workspace.rs` (workspace init/load)

**`src/views/`:**
- Purpose: Top-level layout orchestration for each major screen
- Contains: Two files covering the three rendered layouts (dashboard, detail, viewer)
- Key files: `src/views/dashboard_view.rs`, `src/views/detail_view.rs`

**`src/components/`:**
- Purpose: Reusable widgets consumed by views
- Contains: Data-presenting widgets and modal dialogs
- Key files: `src/components/dashboard.rs`, `src/components/report_detail.rs`, `src/components/modal/new_report.rs`

**`src/theme/`:**
- Purpose: Visual identity constants and helpers
- Contains: Color palette, typography helpers, RPG sprite rendering
- Key files: `src/theme/rpg.rs` (styles/colors), `src/theme/sprites.rs` (avatar faces)

**`tests/fixtures/`:**
- Purpose: Real-looking sample workspace data for integration tests
- Contains: Markdown files with YAML frontmatter in the actual on-disk workspace format
- Generated: No (hand-crafted, checked in)
- Committed: Yes

## Key File Locations

**Entry Points:**
- `src/main.rs`: Binary entry point, CLI (`clap`), TUI setup/teardown, main event loop
- `src/lib.rs`: Library crate root, declares all public modules

**Application State:**
- `src/app/mod.rs`: `App` struct definition, `Msg`, `ViewMode`, `Effect` enums
- `src/app/state.rs`: `App::new()`, `App::load_data()`, `App::delete_entry()`
- `src/app/update.rs`: `App::update(Msg) → Result<Effect>` — the entire state transition table

**Data Model:**
- `src/model/report.rs`: `Report`, `ReportProfile`, `Level`, `ReportType`
- `src/model/meeting.rs`: `JournalEntry`, `Context`, filename parsing/formatting
- `src/model/computed.rs`: `ReportSummary`, `TeamMetrics`, `WorkspaceSummary`, urgency scoring

**Storage:**
- `src/storage/mod.rs`: `parse_frontmatter()`, `StorageError`
- `src/storage/repo/entry.rs`: `EntryRepository` — the most complex storage file (listing, creation, save, delete)
- `src/storage/repo/workspace.rs`: `WorkspaceRepository` — workspace init and report discovery

**Rendering:**
- `src/views/dashboard_view.rs`: Main dashboard layout
- `src/views/detail_view.rs`: Report detail and note viewer layouts
- `src/components/modal/new_report.rs`: Largest component file (new report form state + rendering)

**Theme:**
- `src/theme/rpg.rs`: All color constants and `style_*()` functions — import from here for all styling

**Testing:**
- `tests/storage_test.rs`: Storage layer integration tests
- `tests/app_test.rs`: App state integration tests
- `tests/fixtures/`: Workspace fixture directories

## Naming Conventions

**Files:**
- Snake case: `report_detail.rs`, `new_report.rs`, `dashboard_view.rs`
- Repository files named after the domain entity: `workspace.rs`, `report.rs`, `entry.rs`
- View files suffixed `_view`: `dashboard_view.rs`, `detail_view.rs`
- Modal files in `components/modal/` subdirectory

**Directories:**
- Snake case: `src/components/modal/`, `src/storage/repo/`
- Plural for collections: `src/views/`, `src/components/`

**Types:**
- Structs: `PascalCase` — `ReportSummary`, `WorkspaceRepository`, `NewReportState`
- Enums: `PascalCase` — `ViewMode`, `Msg`, `Effect`, `Context`, `Level`
- Functions: `snake_case` — `render_dashboard_view`, `handle_key_event`, `compute_report_summary`
- Constants: `SCREAMING_SNAKE_CASE` — `COLOR_PRIMARY`, `STATUS_MESSAGE_DURATION`

**On-Disk:**
- Report directories: kebab-case slug derived from name — `alex-chen/`, `chris-wong/`
- Profile file: always `_profile.md` (underscore prefix marks non-entry files)
- Entry files: `YYYY-MM-DDTHHMMSS.md` (new) or `YYYY-MM-DD.md` (legacy)
- Workspace config: `.vibe-manager` (hidden dotfile, no extension)

## Where to Add New Code

**New `Msg` variant (new user action):**
1. Add variant to `Msg` enum in `src/app/mod.rs`
2. Add match arm in `App::update()` in `src/app/update.rs`
3. Map key(s) to the new `Msg` in the relevant `handle_*_key()` function in `src/app/input.rs`

**New modal dialog:**
1. Create `src/components/modal/<name>.rs` with a struct and `render()` method
2. Add `pub mod <name>` and re-export in `src/components/modal/mod.rs`
3. Re-export from `src/components/mod.rs`
4. Add `ViewMode::<ModalName>` variant in `src/app/mod.rs`
5. Render the modal in the appropriate view file in `src/views/`

**New UI component (non-modal):**
1. Create `src/components/<name>.rs`
2. Add `pub mod <name>` and re-export in `src/components/mod.rs`
3. Use in the appropriate view file in `src/views/`

**New model field (stored on disk):**
1. Add field to the relevant struct in `src/model/report.rs` or `src/model/meeting.rs`
2. Add `#[serde(default)]` for backward compatibility with existing files
3. Update `compute_report_summary()` in `src/model/computed.rs` if the field affects metrics

**New storage operation:**
- Entry operations → `src/storage/repo/entry.rs` (`EntryRepository`)
- Report operations → `src/storage/repo/report.rs` (`ReportRepository`)
- Workspace operations → `src/storage/repo/workspace.rs` (`WorkspaceRepository`)

**New computed metric:**
- Add to `ReportSummary` or `WorkspaceSummary` structs in `src/model/computed.rs`
- Update `compute_report_summary()` or `compute_workspace_summary()` in the same file
- Refresh calls in `src/app/state.rs` (`load_data`) and `src/app/update.rs` (after mutations)

**New styling constant:**
- Add to `src/theme/rpg.rs` alongside existing `COLOR_*` constants and `style_*()` functions

**New integration test:**
- Add to `tests/storage_test.rs` (storage layer) or `tests/app_test.rs` (app layer)
- Add fixture data under `tests/fixtures/<slug>/` following the workspace format

**New utility function:**
- Pure, stateless helpers → `src/utils/slug.rs` or `src/utils/mod.rs`

## Special Directories

**`tests/fixtures/`:**
- Purpose: Sample team workspace data used by integration tests
- Generated: No (hand-authored)
- Committed: Yes
- Format: Follows the exact on-disk workspace format (real markdown + YAML frontmatter)

**`target/`:**
- Purpose: Cargo build artifacts
- Generated: Yes
- Committed: No (in `.gitignore`)

**`.github/workflows/`:**
- Purpose: CI pipeline definitions
- Contains: Build, test, lint (clippy with `-D warnings`), format check

**`.planning/codebase/`:**
- Purpose: Architecture analysis documents for GSD tooling
- Contains: This document and ARCHITECTURE.md
- Generated: Yes (by GSD map-codebase)
- Committed: Yes

---

*Structure analysis: 2026-06-10*
