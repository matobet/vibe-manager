<!-- refreshed: 2026-06-10 -->
# Architecture

**Analysis Date:** 2026-06-10

## System Overview

```text
┌─────────────────────────────────────────────────────────────────────┐
│                         main.rs (Runtime)                           │
│  CLI parsing (clap) • Terminal setup (crossterm) • Event loop       │
│  suspend_and_edit() • run_app()                                      │
└────────────────┬────────────────────────────────────────────────────┘
                 │ owns / calls
                 ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    app/ (TEA Core)                                   │
│  App struct (model)  │  App::update(Msg) → Effect  │  handle_key_event │
│  state.rs            │  update.rs                  │  input.rs          │
└───────┬──────────────┴──────────────────────────────────────────────┘
        │ reads                              │ returns Effect::SpawnEditor
        ▼                                   ▼
┌────────────────────────┐    ┌────────────────────────────────────────┐
│   model/  (Pure data)  │    │   editor.rs (External editor bridge)   │
│  Report, ReportProfile │    │  Suspends TUI → spawns $EDITOR → resumes│
│  JournalEntry, Workspace│   └────────────────────────────────────────┘
│  ReportSummary (computed)│
└────────────────────────┘
        │ loaded by
        ▼
┌─────────────────────────────────────────────────────────────────────┐
│               storage/repo/ (Repository Pattern)                    │
│  WorkspaceRepository  │  ReportRepository  │  EntryRepository       │
│  workspace.rs         │  report.rs         │  entry.rs              │
└─────────────────────────────────────────────────────────────────────┘
        │ reads/writes
        ▼
┌─────────────────────────────────────────────────────────────────────┐
│   Filesystem (Markdown + YAML frontmatter)                          │
│   workspace/.vibe-manager  •  <slug>/_profile.md                    │
│   <slug>/journal/YYYY-MM-DDTHHMMSS.md  •  <slug>/team/<slug>/       │
└─────────────────────────────────────────────────────────────────────┘

Rendering path (separate from state path):
App (read-only) → views/ → components/ → ratatui widgets → terminal frame
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

**Overall:** The Elm Architecture (TEA) — Model/Update/View separation

**Key Characteristics:**
- `App` is the single source of truth for all UI state; no distributed state
- `update()` returns `Effect` values instead of executing side effects directly — keeps update pure
- Views receive `&App` as read-only; they never mutate state
- All I/O is isolated inside `storage/repo/`; model structs carry no file handles
- External editor integration suspends/resumes the TUI terminal cleanly around a child process

## Layers

**Runtime Layer:**
- Purpose: Terminal lifecycle, event polling, effect execution
- Location: `src/main.rs`
- Contains: `run_app()`, `suspend_and_edit()`, CLI via `clap`
- Depends on: `app`, `views`, `storage`, `editor`
- Used by: (entry point — nothing uses it)

**App Layer (TEA Core):**
- Purpose: Application state machine
- Location: `src/app/`
- Contains: `App` struct, `Msg` enum, `ViewMode` enum, `Effect` enum, key handler
- Depends on: `model`, `storage`, `components::modal`
- Used by: `main.rs`, `views`

**View Layer:**
- Purpose: Full-screen layout composition
- Location: `src/views/`
- Contains: `render_dashboard_view()`, `render_detail_view()`, `render_viewer_view()`
- Depends on: `app`, `components`, `model`
- Used by: `main.rs` render loop

**Component Layer:**
- Purpose: Reusable ratatui widgets
- Location: `src/components/`
- Contains: `Dashboard`, `AvatarGrid`, `ReportDetail`, `NoteViewer`, `StatusBar`, modal structs
- Depends on: `model`, `theme`
- Used by: `views`

**Model Layer:**
- Purpose: Data structures and pure computation
- Location: `src/model/`
- Contains: `Report`, `ReportProfile`, `JournalEntry`, `Workspace`, `ReportSummary`, `TeamMetrics`
- Depends on: nothing (pure Rust + serde)
- Used by: all layers

**Storage Layer:**
- Purpose: Filesystem access; YAML frontmatter serialization/deserialization
- Location: `src/storage/`
- Contains: `WorkspaceRepository`, `ReportRepository`, `EntryRepository`, `parse_frontmatter()`
- Depends on: `model`, `utils`
- Used by: `app`

**Theme Layer:**
- Purpose: Visual constants and style helpers
- Location: `src/theme/`
- Contains: Color constants (`COLOR_PRIMARY`, etc.), `style_*()` functions, avatar sprite logic
- Depends on: `model` (sprites use `ReportSummary`)
- Used by: `components`

**Utils Layer:**
- Purpose: Stateless pure helpers
- Location: `src/utils/`
- Contains: `name_to_slug()`, `color_from_name()`, `report_color()`, hex color parsing
- Depends on: nothing
- Used by: `model`, `storage`, `components`

## Data Flow

### Primary Request Path (User Keystroke → State Update)

1. `crossterm` event polled in `run_app()` (`src/main.rs:156`)
2. `handle_key_event(app, key)` maps key to `Option<Msg>` based on `app.view_mode` (`src/app/input.rs:13`)
3. `app.update(msg)` pattern-matches on `Msg`, mutates `App` fields, returns `Effect` (`src/app/update.rs:15`)
4. Runtime inspects `Effect`; if `Effect::None`, continues to next render; if `Effect::SpawnEditor`, calls `suspend_and_edit()` (`src/main.rs:162`)
5. `terminal.draw(|frame| ...)` selects the appropriate view function by `app.view_mode` (`src/main.rs:135`)
6. View function reads `&App`, constructs widgets, renders to `Frame` (`src/views/`)

### Data Load Path (Startup / Refresh)

1. `App::new(workspace_path)` opens `WorkspaceRepository` (`src/app/state.rs:19`)
2. `repo.load()` reads `.vibe-manager` config file → `Workspace` (`src/storage/repo/workspace.rs:71`)
3. `app.load_data()` calls `repo.list_reports()` → `Vec<ReportRepository>` (`src/app/state.rs:56`)
4. Each `ReportRepository::load()` reads `_profile.md` frontmatter → `Report` (`src/storage/repo/report.rs:41`)
5. `report_repo.entries().list()` scans `journal/` + root for `*.md` files → `Vec<JournalEntry>` (`src/storage/repo/entry.rs:28`)
6. `compute_report_summary()` calculates urgency score, overdue status, mood trend per report (`src/model/computed.rs:82`)
7. Reports sorted by `urgency_score` descending into `app.reports`, `app.entries_by_report`, `app.summaries`

### External Editor Path

1. `Msg::NewMeeting` or `Msg::EditMeeting` — `update()` returns `Effect::SpawnEditor { is_new }`
2. `suspend_and_edit()` in `main.rs`: leave alternate screen, disable raw mode
3. `editor::edit_file(&path)` spawns `$EDITOR`/`$VISUAL`/fallback, waits for exit, checks mtime
4. Re-enables raw mode, re-enters alternate screen, clears terminal
5. Parses frontmatter from modified file; updates `app.entries_by_report` in place
6. Empty-content check (`is_content_empty()`) auto-deletes files the user saved blank

**State Management:**
- All mutable state lives in one `App` struct on the stack in `main.rs`
- No `Arc<Mutex<>>` or shared state; the app is single-threaded
- Computed summaries (`ReportSummary`, `WorkspaceSummary`) are re-derived from source data after any mutation

## Key Abstractions

**`Msg` enum:**
- Purpose: All possible state-changing actions expressed as data
- Examples: `src/app/mod.rs:62`
- Pattern: TEA message passing — input layer produces `Msg`, update layer consumes it

**`Effect` enum:**
- Purpose: Side effects the runtime must perform (currently only `SpawnEditor`)
- Examples: `src/app/mod.rs:34`
- Pattern: Effect system keeping `update()` free of I/O

**`ViewMode` enum:**
- Purpose: Controls which view and key-handler branch is active
- Examples: `src/app/mod.rs:43`
- Pattern: State machine — each variant has distinct rendering and input semantics

**Repository Types:**
- Purpose: Domain-scoped file I/O with typed return values
- Examples: `src/storage/repo/workspace.rs`, `src/storage/repo/report.rs`, `src/storage/repo/entry.rs`
- Pattern: Repository pattern; callers never touch `fs::` directly

**`ReportSummary` / `TeamMetrics`:**
- Purpose: Computed view-models derived at load time and after mutations
- Examples: `src/model/computed.rs`
- Pattern: Derived/computed state — not stored on disk, recalculated from `Report` + `Vec<JournalEntry>`

## Entry Points

**TUI mode:**
- Location: `src/main.rs:86` (`run_tui`)
- Triggers: `vibe-manager <path>` with no subcommand
- Responsibilities: Opens workspace, sets up terminal, runs event loop

**Init mode:**
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

**What happens:** Writing to `entry.path` or report directories with `std::fs` outside of `storage/repo/`
**Why it's wrong:** The repositories handle path construction, directory creation, and format serialization; bypassing them produces inconsistent file structure
**Do this instead:** Use `app.repo.report(&slug).entries().create_meeting()` / `.save()` / `.delete()` — all entry mutations go through `EntryRepository` (`src/storage/repo/entry.rs`)

### Mutating `App` State Inside View/Component Functions

**What happens:** View or component functions taking `&mut App` and modifying fields
**Why it's wrong:** Views are the View in TEA — they must be pure render functions; mutating state in render breaks the audit trail (all changes should come through `Msg` → `update()`)
**Do this instead:** Views take `&App` (immutable reference); all state changes flow through `handle_key_event → Msg → App::update()`

### Skipping Computed Summary Refresh

**What happens:** Modifying `app.entries_by_report` without re-calling `compute_report_summary()` and `compute_workspace_summary()`
**Why it's wrong:** `ReportSummary` fields like `urgency_score`, `is_overdue`, and `recent_mood` become stale; dashboard display shows incorrect data
**Do this instead:** After any entry mutation, call the pattern used in `delete_entry()` and `handle_save_entry()` in `src/app/state.rs` and `src/app/update.rs`

## Error Handling

**Strategy:** `anyhow::Result<T>` propagated to the top-level event loop in `main.rs`; storage errors use typed `StorageError` via `thiserror`

**Patterns:**
- Storage layer returns `StorageResult<T>` (`StorageError` variants: `Io`, `Yaml`, `InvalidWorkspace`, `ProfileNotFound`)
- `App::update()` returns `anyhow::Result<Effect>`; I/O errors from storage propagate up to `run_app()` which exits
- User-visible non-fatal errors (e.g., "Error saving mood") call `app.set_status(msg)` which displays for 3 seconds and clears automatically

## Cross-Cutting Concerns

**Logging:** None; errors surface as status bar messages or terminal output on fatal exit
**Validation:** Input validation occurs in modal state (`NewReportState::is_valid()`) and in `update.rs` before calling storage
**Authentication:** Not applicable; local filesystem only

---

*Architecture analysis: 2026-06-10*
