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
- `model/` - Data structures (EngineerProfile, Meeting, Workspace)
- `storage/` - File I/O, YAML frontmatter parsing, workspace loading
- `components/` - Reusable UI widgets (avatar, dashboard, note_editor, modal)
- `views/` - Full-screen layouts (dashboard_view, detail_view)
- `theme/rpg.rs` - 8-bit color palette and styling

### Data Model (Pure Markdown)
```
workspace/
├── .vibe-manager           # Workspace config (YAML)
├── alex-chen/
│   ├── _profile.md         # Engineer profile (YAML frontmatter + markdown)
│   ├── 2026-01-15.md       # Meeting note (date = filename, optional mood in frontmatter)
```

- Folders = engineers (slug derived from name)
- Files = meetings (filename is date YYYY-MM-DD.md)
- File exists = meeting happened (no status field needed)
- Computed fields (overdue status, mood trends) calculated at runtime

### Key Types
- `ViewMode` enum: Dashboard, EngineerDetail, NoteEditor, NewEngineerModal, Help
- `Msg` enum: All state update messages
- `EngineerProfile`: name, level (P1-P5), meeting_frequency, skills
- `Meeting`: date, content, optional mood (1-5)

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
