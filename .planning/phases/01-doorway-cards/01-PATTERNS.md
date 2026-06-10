# Phase 1: Doorway Cards - Pattern Map

**Mapped:** 2026-06-10
**Files analyzed:** 11 new/modified files
**Analogs found:** 10 / 11 (only the TestBackend+insta harness has no in-repo analog — RESEARCH.md provides it)

## File Classification

| New/Modified File | Change | Role | Data Flow | Closest Analog | Match Quality |
|-------------------|--------|------|-----------|----------------|---------------|
| `src/model/computed.rs` | EXTEND | model (pure computation) | transform | itself (`compute_team_metrics`, `compute_report_summary`, test module) | exact (self) |
| `src/app/state.rs` | EXTEND | app state / orchestration | event-driven (TEA) | itself (`load_data`, `delete_entry`) | exact (self) |
| `src/app/update.rs` | EDIT | app update handler | event-driven (TEA) | itself (`handle_save_entry`) | exact (self) |
| `src/components/doorway_card.rs` | NEW | component (read-only widget) | request-response (render) | `src/components/avatar.rs` (`AvatarCard`) | exact (same role + flow) |
| `src/components/avatar.rs` | EXTEND | component (grid layout) | request-response (render) | itself (`AvatarGrid::render`) | exact (self) |
| `src/components/mod.rs` | EXTEND | config (module barrel) | — | itself | exact (self) |
| `src/theme/rpg.rs` | EXTEND | utility (style/format helpers) | transform | itself (`progress_bar`, `mood_gauge`, `format_days_ago`) | exact (self) |
| `src/utils/` (abbreviate_name) | EXTEND | utility (pure string fn) | transform | `src/utils/slug.rs` (`name_to_slug`) | exact (same role + flow) |
| `src/model/mod.rs` | EXTEND | config (re-exports) | — | itself | exact (self) |
| `tests/app_test.rs` | EXTEND | test (integration, fixture-backed) | batch | itself | exact (self) |
| `tests/ui_test.rs` | NEW | test (snapshot rendering) | batch | structural: `tests/app_test.rs`; harness: none in repo | role-match / no-analog |

Note for planner: `ReportProfile` **already has** a `title: Option<String>` field (verified at `src/storage/repo/report.rs:142` test literal). DOOR-01 only needs to copy it onto `ReportSummary` — no profile/serde migration.

## Pattern Assignments

### `src/model/computed.rs` (model, pure transform) — EXTEND

**Analog:** itself. Every addition slots next to an existing twin.

**Struct + doc-comment pattern** — `TeamMetrics` (lines 44-59). New `OutlierInfo` and new fields copy this exact shape: `//!`-style struct doc, `///` per field, `#[derive(Debug, Clone)]`:
```rust
/// Aggregated team metrics for managers
///
/// Computed from the summaries of a manager's 2nd-level reports.
#[derive(Debug, Clone)]
pub struct TeamMetrics {
    /// Number of 2nd-level reports
    pub team_size: usize,
    ...
    /// Composite team health score (0-100, higher is healthier)
    pub team_health_score: u8,
}
```

**Pure aggregation over summaries** — `compute_team_metrics` (lines 146-184) is the function to extend in place. It already receives `&[ReportSummary]` carrying everything `OutlierInfo` needs (`name`, `urgency_score`, `mood_trend`, `recent_mood`, `is_overdue`, `days_since_meeting`, `active`). Copy its iterator style:
```rust
let active_summaries: Vec<_> = team_summaries.iter().filter(|s| s.active).collect();
let team_overdue_count = active_summaries.iter().filter(|s| s.is_overdue).count();
```
Build `outliers` and `next_in_rotation` with the same `filter`/`filter_map` idiom, then add the two new fields to the `TeamMetrics { ... }` literal at lines 177-183.

**Small private helper pattern** — `aggregate_mood_trends` (lines 187-208) and `calculate_team_health_score` (lines 212-248): private `fn` with `///` one-liner, takes slices, returns value. `is_outlier(&ReportSummary) -> bool` and the rotation pick follow this shape.

**New public scoring fn** — model on `compute_extended_workspace_summary` (lines 367-374): small, pure, `pub`, doc-commented:
```rust
/// Extended workspace summary including 2nd-level reports
pub fn compute_extended_workspace_summary(
    direct_summaries: &[ReportSummary],
    second_level_count: usize,
) -> WorkspaceSummary {
    let mut summary = compute_workspace_summary(direct_summaries);
    summary.total_report_count = direct_summaries.len() + second_level_count;
    summary
}
```
`manager_urgency_bonus(metrics: &TeamMetrics) -> i32` is the same species. **DO NOT touch `calculate_urgency_score`** (lines 260-308) — it is private and pinned by exact-value tests (`assert_eq!(score, 30)` at line 416, etc.).

**Frequency-parameterized summary variant** — `compute_report_summary` (lines 82-143) derives `frequency_days` from the report at line 99 (`report.meeting_frequency_days()`, defined `src/model/report.rs:177-184`: weekly=7, biweekly=14, monthly=30, default 14). The Phase-1 variant (`compute_report_summary_with_frequency` or similar) extracts the body with `frequency_days` as a parameter; the existing fn delegates, keeping its signature byte-identical. Also: line 141 `team_metrics: None, // Set separately for managers` is the field the load path overwrites, and the `ReportSummary { ... }` literal (lines 125-142) is where `title` gets added (source: `report.profile.title.clone()`).

**Test pattern** — test module (lines 376-482). Copy `create_test_summary` (lines 462-481): a private helper returning a fully-populated `ReportSummary` struct literal. **Pitfall 6**: adding `title`/`outliers`/`next_in_rotation` breaks this literal and the `TeamMetrics` construction — update them in the same task. New unit tests copy the existing naming (`test_urgency_*`, `test_team_health_score_*`) and behavioral-assertion style:
```rust
#[test]
fn test_team_health_score_healthy() {
    let summaries = vec![
        create_test_summary(Some(4), Some(MoodTrend::Stable), false),
        ...
    ];
    let metrics = compute_team_metrics(&summaries);
    assert!(metrics.team_health_score >= 90);
}
```

---

### `src/app/state.rs` (app orchestration, TEA) — EXTEND

**Analog:** itself.

**Imports pattern** (lines 1-15) — `//!` module doc, blank line, std imports, `anyhow::Result`, then crate imports grouped from module roots:
```rust
use anyhow::Result;

use super::{App, ViewMode, STATUS_MESSAGE_DURATION};
use crate::model::{
    compute_report_summary, compute_workspace_summary, Context, JournalEntry, WorkspaceSummary,
};
use crate::storage::WorkspaceRepository;
```
New imports (`compute_team_metrics`, `manager_urgency_bonus`, etc.) merge into the existing `crate::model::{...}` group.

**Team-loading site to extend** — `load_data` (lines 56-106). The exact `filter_map` closure with the tolerant-load idiom (`.ok()?`, `unwrap_or_default()`, `if let Ok`) — team-entry loading goes inside this existing `if report_repo.has_team()` block:
```rust
// lines 64-86
let mut all_data: Vec<_> = report_repos
    .into_iter()
    .filter_map(|report_repo| {
        let mut report = report_repo.load().ok()?;

        // Load team members for managers
        if report_repo.has_team() {
            for team_repo in report_repo.list_team_members().unwrap_or_default() {
                if let Ok(team_member) = team_repo.load() {
                    report.team.push(team_member);
                }
            }
        }

        let entries = report_repo.entries().list().unwrap_or_default();
        let summary = compute_report_summary(
            &report,
            &entries,
            self.workspace.config.settings.overdue_threshold_days,
        );
        Some((report, entries, summary))
    })
    .collect();

// Sort by urgency score (highest first = needs most attention)
all_data.sort_by_key(|d| std::cmp::Reverse(d.2.urgency_score));
```
Per-member entries come from `team_repo.entries().list().unwrap_or_default()` — `ReportRepository::entries()` exists (`src/storage/repo/report.rs:128-130`); never `fs::read_dir`. The urgency bonus (`summary.urgency_score += manager_urgency_bonus(&tm); summary.team_metrics = Some(tm);`) must land inside the closure, **before** the sort at line 89. Member summaries are consumed here to build `TeamMetrics` and dropped — never pushed into the parallel Vecs (lines 92-96 unpack `reports`/`entries_by_report`/`summaries` index-aligned).

**The recompute site with the wipe bug** — `delete_entry` (lines 136-158), specifically lines 147-155:
```rust
// Recompute summary for this report
let report = &self.reports[report_idx];
let entries = &self.entries_by_report[report_idx];
self.summaries[report_idx] = compute_report_summary(
    report,
    entries,
    self.workspace.config.settings.overdue_threshold_days,
);
self.workspace_summary = compute_workspace_summary(&self.summaries);
```
This block (and its twin in `update.rs`) is what the new centralized `recompute_summary(&mut self, report_idx: usize)` helper replaces — preserving `team_metrics` via `.take()` and re-applying the bonus (RESEARCH.md Pattern 3 gives the full body). Method placement/visibility analog: `current_list_len` (lines 198-204) shows the `pub(crate) fn` style for cross-module App helpers; doc-comment style per `delete_entry` (lines 133-135).

---

### `src/app/update.rs` (app update handler, TEA) — EDIT

**Analog:** itself. One site changes: `handle_save_entry` (lines 416-445). The duplicated recompute block at lines 426-434 is byte-equivalent to `state.rs:147-155`:
```rust
Ok(entry) => {
    self.entries_by_report[report_idx].push(entry);
    // Recompute summary
    let report = &self.reports[report_idx];
    let entries = &self.entries_by_report[report_idx];
    self.summaries[report_idx] = compute_report_summary(
        report,
        entries,
        self.workspace.config.settings.overdue_threshold_days,
    );
    self.workspace_summary = compute_workspace_summary(&self.summaries);
    self.set_status("Observation recorded");
}
Err(e) => {
    self.set_status(format!("Error: {}", e));
}
```
Replace lines 427-434 with `self.recompute_summary(report_idx);`. Keep the surrounding `Ok`/`Err` + `set_status` pattern untouched — it is the project's standard non-fatal error display (`format!("Error: {}", e)`, 3s auto-clear).

---

### `src/components/doorway_card.rs` (component, render) — NEW

**Analog:** `src/components/avatar.rs` (`AvatarCard`) — same role, same flow, same caller.

**Module header + imports pattern** (avatar.rs lines 1-21) — `//!` doc with Design Philosophy bullets, ratatui grouped import, then crate imports from module roots:
```rust
//! Report card components with color-coded borders and kaomoji avatars
//!
//! Design Philosophy:
//! - Color-coded borders for quick report recognition
//! ...

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::model::{MoodTrend, ReportSummary, ReportType};
use crate::theme::{
    format_days_ago, mood_color, mood_gauge, mood_trend_icon, overdue_color, overdue_icon, sprites,
    style_muted, style_title, COLOR_SECONDARY,
};
```

**Widget struct + constructor + render signature** (avatar.rs lines 27-40) — copy verbatim shape; this is the contract `AvatarGrid` calls:
```rust
pub struct AvatarCard<'a> {
    summary: &'a ReportSummary,
    is_selected: bool,
}

impl<'a> AvatarCard<'a> {
    pub fn new(summary: &'a ReportSummary, is_selected: bool) -> Self {
        Self { summary, is_selected }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) { ... }
}
```

**Layout-split rendering** (avatar.rs lines 65-76) — fixed-`Length` constraints, render each chunk; DoorwayCard does the same but `Direction::Horizontal` first (`[Length(9), Min(0)]` sprite|text per UI-SPEC), then 5 vertical `Length(1)` rows in the text column, plus a `[Min(0), Length(4)]` split on line 1 for the right-aligned badge:
```rust
let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Length(4), // Face sprite (4 lines, bottom-aligned)
        Constraint::Length(1), // Name
        Constraint::Length(1), // Mood gauge
        Constraint::Length(1), // Meeting status
    ])
    .split(inner);
```

**Selection styling** (avatar.rs lines 80-89) — the exact selected/unselected branch DoorwayCard reuses for sprite + name (UI-SPEC decision: yellow convention, not mockup cyan):
```rust
let name: String = self.summary.name.chars().take(14).collect();  // char-truncation idiom; doorway uses take(24)
let name_style = if self.is_selected {
    style_title()
} else {
    Style::default()
        .fg(self.summary.color)
        .add_modifier(Modifier::BOLD)
};
```

**Mood gauge + fallback spans** (avatar.rs lines 92-116) — line 2's hearts segment copies this `map_or_else` + conditional-span composition:
```rust
let mood_display = self
    .summary
    .recent_mood
    .map_or_else(|| "─────".to_string(), mood_gauge);
let mood_style = self
    .summary
    .recent_mood
    .map_or(style_muted(), |m| Style::default().fg(mood_color(m)));
```

**Status icon + color pairing** (avatar.rs lines 119-128) — line 2's `✓/⚠ you: 3d` segment copies this:
```rust
let status_icon = overdue_icon(self.summary.is_overdue);
let status_color = overdue_color(self.summary.is_overdue);
let days_text = format_days_ago(self.summary.days_since_meeting);  // doorway: format_compact_age
let status_para = Paragraph::new(Line::from(vec![
    Span::styled(status_icon, Style::default().fg(status_color)),
    Span::raw(" "),
    Span::styled(days_text, Style::default().fg(status_color)),
]))
```

**Sprite rendering** (avatar.rs lines 131-148) — copy `render_face`, minus the IC padding branch (managers are always 4 lines, `src/theme/sprites.rs:181-187`), and with `Alignment::Left` per UI-SPEC:
```rust
let face_style = if self.is_selected {
    style_title()
} else {
    Style::default().fg(self.summary.color)
};
let sprite = sprites::FaceSprite::from_summary(self.summary, face_style);
let lines = sprite.lines();
```
**Critical deviation (UI-SPEC geometry):** `FaceSprite::lines()` branches on `is_overdue` (sprites.rs lines 129-177) — the overdue branch pads to up to 13 cells wide with floating z's and swaps the face to `-_-` (sprites.rs line 79-80). The doorway card **must always get the non-overdue render path** (overdue is shown as `zZ` text on line 2 instead). Construct `FaceSprite { level, mood, is_overdue: false, days_since_meeting, style }` directly (struct fields are `pub`, see sprites.rs lines 93-99 and the test constructor at lines 194-202) rather than `from_summary`, OR pass a summary clone with `is_overdue` forced false — the direct struct literal is the cleaner copy of sprites.rs's own test pattern. The mood-2 `◦︵◦` fullwidth compensation lives inside `FaceSprite` (sprites.rs lines 121-126) — never re-pad around it.

**Squad line (line 3)** — no direct analog; copy the span-vector composition style from avatar.rs lines 106-115 / dashboard.rs lines 93-111 (conditional `Vec<Span>` building) with the exact segments/styles from UI-SPEC §Line 3 and RESEARCH.md Pattern 5. Empty-squad shape (`team_metrics: None` **or** `team_size == 0`): single muted span `squad 0 · no members yet`; never `unwrap()` on `team_metrics` or index `outliers[0]` — use `tm.outliers.first()`.

**Line 4 invariant (DOOR-03)** — always emit the row; selected renders `▸ Space to visit squad` in `style_muted()`, unselected renders `Line::from("")` (same idiom as avatar.rs line 143's padding line).

---

### `src/components/avatar.rs` — `AvatarGrid` heterogeneous layout — EXTEND

**Analog:** itself — `AvatarGrid::render` (lines 168-213) is the algorithm being generalized.

**Current uniform-grid algorithm to preserve for IC runs** (lines 173-213):
```rust
let card_width: u16 = 18;
let card_height: u16 = 9;

let cards_per_row = (area.width / card_width).max(1) as usize;
let num_rows = self.summaries.len().div_ceil(cards_per_row);

let row_constraints: Vec<Constraint> = (0..num_rows)
    .map(|_| Constraint::Length(card_height))
    .collect();

let rows = Layout::default()
    .direction(Direction::Vertical)
    .constraints(row_constraints)
    .split(area);

for (row_idx, row_area) in rows.iter().enumerate() {
    let start_idx = row_idx * cards_per_row;
    let end_idx = (start_idx + cards_per_row).min(self.summaries.len());
    ...
    for (col_idx, (summary, col_area)) in row_summaries.iter().zip(cols.iter()).enumerate() {
        let global_idx = start_idx + col_idx;
        let is_selected = global_idx == self.selected;
        let card = AvatarCard::new(summary, is_selected);
        card.render(frame, *col_area);
    }
}
```
New shape: pre-walk `summaries` partitioning into rows (`Manager` → one full-width `Length(DOORWAY_CARD_HEIGHT)` row; consecutive-IC run → chunks of `cards_per_row` with `Length(9)`), build the constraint Vec, then render each row dispatching `matches!(summary.report_type, ReportType::Manager)` → `DoorwayCard` vs `AvatarCard`. The `global_idx == self.selected` linear-index check carries over unchanged (navigation is linear — `input.rs` maps h/k/l/j to SelectPrev/SelectNext; geometry-independent). Keep the empty-guard at lines 169-171. Type-dispatch idiom precedent: avatar.rs line 142 `if self.summary.report_type != ReportType::Manager`.

---

### `src/components/mod.rs` — EXTEND

**Analog:** itself (lines 5-16). Add declaration and grouped re-export in the existing commented sections:
```rust
pub mod avatar;
pub mod dashboard;
...
// Avatar and card components
pub use avatar::{AvatarCard, AvatarGrid};
```
New: `pub mod doorway_card;` + `pub use doorway_card::DoorwayCard;` under "Avatar and card components".

---

### `src/theme/rpg.rs` — `health_bar`, `format_compact_age` — EXTEND

**Analog:** itself.

**Gauge helper pattern** — `progress_bar` (lines 141-146) and `mood_gauge` (lines 149-154): `///` doc with example output, clamp, `format!` + `repeat`:
```rust
/// Create a block progress bar: ████░░░░░░
pub fn progress_bar(value: u8, max: u8) -> String {
    let filled = value.min(max) as usize;
    let empty = (max as usize).saturating_sub(filled);
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}
```
`health_bar(score: u8, cells: usize)` sits beside these in the "Progress & Gauges" `// ═══` section (line 137); body per RESEARCH.md (`div_ceil`, `▕▓░▏` caps).

**Format helper pattern** — `format_days_ago` (lines 234-244): `Option<i64>` match with `None`/guard arms:
```rust
/// Format days as human-readable relative time (e.g., "5 days ago", "now")
pub fn format_days_ago(days: Option<i64>) -> String {
    match days {
        None => "Never".to_string(),
        Some(d) if d < 0 => "Future?".to_string(),
        Some(d) => HumanTime::from(TimeDelta::days(-d)).to_string(),
    }
}
```
`format_compact_age` is the same shape (`None` → `"never"`, `<14` → `"{d}d"`, else `"{d/7}w"`) in the "Formatting Helpers" section (line 230).

**Unit-test pattern** — exact-string assertions in the `#[cfg(test)]` module (lines 285-325):
```rust
#[test]
fn test_progress_bar() {
    assert_eq!(progress_bar(3, 5), "███░░");
    assert_eq!(progress_bar(0, 5), "░░░░░");
    assert_eq!(progress_bar(10, 5), "█████"); // Clamped
}
```
Add `test_health_bar` (including 0, 100, and rounding edges) and `test_format_compact_age` (None/0/13/14 boundaries) in this style.

**Glyph constants** — existing `ICON_WARNING: &str = "⚠"` (line 181) is reused bare (no U+FE0E). If a `▸` hint marker constant is wanted, follow the `pub const ICON_*: &str` pattern at lines 180-190; `★` is already used as a bare literal (dashboard.rs line 101), so literals in component code are also acceptable. Note `theme/mod.rs` is `pub use rpg::*;` — new pub helpers are exported automatically.

---

### `src/utils/` — `abbreviate_name` — EXTEND

**Analog:** `src/utils/slug.rs` (`name_to_slug`) — pure string fn, doc with input→output examples, iterator-chain body, exhaustive small test module:
```rust
/// Convert a name to a URL-friendly slug
/// "Alex Chen" -> "alex-chen"
/// "María García" -> "maria-garcia"
pub fn name_to_slug(name: &str) -> String {
    name.to_lowercase()
        .chars()
        ...
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_name() {
        assert_eq!(name_to_slug("Alex Chen"), "alex-chen");
    }
    #[test]
    fn test_special_chars() {
        assert_eq!(name_to_slug("O'Brien"), "o-brien");
    }
}
```
Placement options matching `src/utils/mod.rs` (lines 1-3: `mod slug; pub use slug::*;`): either a new `mod name; pub use name::*;` sibling file (preferred — mirrors slug.rs exactly) or directly in mod.rs alongside `color_from_name`. Test edge cases: `"Sam Taylor"` → `"Sam T"`, single name, three-part name (`"Mary Jane Watson"` → `"Mary W"` per RESEARCH.md).

---

### `src/model/mod.rs` — re-exports — EXTEND

**Analog:** itself (lines 12-15) — grouped `pub use computed::{...}`:
```rust
pub use computed::{
    compute_extended_workspace_summary, compute_report_summary, compute_team_metrics,
    compute_workspace_summary, MoodTrend, ReportSummary, TeamMetrics, WorkspaceSummary,
};
```
Add `manager_urgency_bonus`, `OutlierInfo`, and any new compute variant to this group (alphabetical-ish ordering, fns then types).

---

### `tests/app_test.rs` — integration regression tests — EXTEND

**Analog:** itself.

**Fixture + temp-workspace harness** (lines 11-38) — reuse as-is for the team_metrics-survival and urgency-sort tests:
```rust
fn fixtures_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

/// Copy fixtures to a temp directory for mutation tests
fn setup_temp_workspace() -> TempDir {
    let temp = TempDir::new().expect("Failed to create temp dir");
    let fixtures = fixtures_path();
    copy_dir_all(&fixtures, temp.path()).expect("Failed to copy fixtures");
    temp
}
```

**Find-report-by-name idiom** (lines 58-62) — locate `chris-wong` the same way tests locate Alex Chen:
```rust
let report_idx = app
    .reports
    .iter()
    .position(|e| e.profile.name == "Alex Chen")
    .expect("Alex Chen not found");
```

**Msg-driven mutation workflow** (lines 173-226, `test_entry_input_modal_workflow`) — the team_metrics-survival test copies this exact flow against the manager (select report, `ShowEntryInput`, `SetEntryMood`, `SaveEntry`), then asserts `app.summaries[idx].team_metrics.is_some()`:
```rust
app.selected_report_index = Some(report_idx);
app.view_mode = ViewMode::ReportDetail;
app.update(Msg::ShowEntryInput).unwrap();
app.update(Msg::SetEntryMood(4)).unwrap();
app.update(Msg::SaveEntry).unwrap();
```
A `delete_entry` companion copies `test_app_delete_entry_updates_summary` (lines 88-131). **Pitfall 3 discipline:** assert structure only (`team_metrics.is_some()`, `outliers` non-empty, "Morgan Smith" is `outliers[0]` — mood 2 is date-independent), never exact day counts.

---

### `tests/ui_test.rs` — snapshot harness — NEW

**Analog:** none in repo (first insta + TestBackend usage — verified zero `assert_snapshot` call sites). Use RESEARCH.md's harness verbatim:
```rust
use ratatui::{backend::TestBackend, Terminal};

#[test]
fn doorway_card_selected_shows_hint() {
    let summary = manager_summary_fixture(); // constructed struct literal — NOT loaded fixtures
    let backend = TestBackend::new(60, 5);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|f| {
            vibe_manager::components::DoorwayCard::new(&summary, true).render(f, f.area());
        })
        .unwrap();
    insta::assert_snapshot!(terminal.backend());
}
```
Borrow these in-repo patterns for the file's structure:
- File-level `//!` doc + `vibe_manager::` paths from `tests/app_test.rs` (lines 1-3, 43)
- Summary-literal builder from `create_test_summary` in `src/model/computed.rs` (lines 462-481) — extend it locally with `team_metrics: Some(TeamMetrics { ... })`, `title`, manager `level`/`report_type`
- Required snapshot variants per UI-SPEC Validation Hooks: selected, unselected, healthy-squad (`★ all well · next:`), multi-outlier (`(+N more)`), no-team (`squad 0 · no members yet`); plus the DOOR-03 height/diff assertion (both buffers 5 rows, differ only in content line 4)
- First-run snapshot generation: `INSTA_UPDATE=accept cargo test` then commit `.snap` files (no cargo-insta CLI installed)

---

## Shared Patterns

### Selection styling (yellow convention)
**Source:** `src/components/avatar.rs:42-57` (block) and `:80-89` (name), `src/theme/rpg.rs:62-66` (`style_title`)
**Apply to:** `doorway_card.rs` (sprite, name), `avatar.rs` grid dispatch
```rust
let style = if self.is_selected {
    style_title() // yellow + bold
} else {
    Style::default().fg(self.summary.color)
};
```
UI-SPEC locked this over the mockup's cyan. Hint line stays `style_muted()` even when selected.

### Tolerant data loading (corrupt files never crash the dashboard)
**Source:** `src/app/state.rs:64-86`
**Apply to:** team-entry loading in `load_data`
```rust
let mut report = report_repo.load().ok()?;
for team_repo in report_repo.list_team_members().unwrap_or_default() {
    if let Ok(team_member) = team_repo.load() { ... }
}
let entries = report_repo.entries().list().unwrap_or_default();
```

### Non-fatal error display
**Source:** `src/app/update.rs:437-439`, `src/app/state.rs:109-111`
**Apply to:** any fallible path added in app layer
```rust
Err(e) => {
    self.set_status(format!("Error: {}", e));
}
```

### Span-vector line composition (icon + color pairing, never color-alone)
**Source:** `src/components/avatar.rs:122-127`, `src/components/dashboard.rs:93-111`
**Apply to:** all four doorway-card lines
```rust
Paragraph::new(Line::from(vec![
    Span::styled(icon, Style::default().fg(color)),
    Span::raw(" "),
    Span::styled(text, Style::default().fg(color)),
]))
```

### Layout splits over manual padding
**Source:** `src/components/avatar.rs:65-76`, `src/components/dashboard.rs:63-70`
**Apply to:** doorway card geometry (sprite|text split, badge right-column)
Fixed `Constraint::Length` vectors; alignment is `Layout`'s job — never space-pad strings containing multi-byte glyphs (`♥`, `◦︵◦`, `⚠`). Right-alignment via `Paragraph::alignment(Alignment::Right)` in its own column, not width math.

### Module documentation
**Source:** every touched file, e.g. `src/components/avatar.rs:1-7`, `src/model/computed.rs:1-4`
**Apply to:** `doorway_card.rs`, `ui_test.rs`
```rust
//! Report card components with color-coded borders and kaomoji avatars
//!
//! Design Philosophy:
//! - ...
```

### Manager detection
**Source:** `src/components/avatar.rs:142` (`self.summary.report_type != ReportType::Manager`), conventions require `matches!()` for single-variant checks
**Apply to:** `AvatarGrid` dispatch
```rust
matches!(summary.report_type, ReportType::Manager)
```

## No Analog Found

| File | Role | Data Flow | Reason | Fallback |
|------|------|-----------|--------|----------|
| `tests/ui_test.rs` (harness mechanics) | test | batch | Zero existing TestBackend/insta usage in repo | RESEARCH.md "Snapshot test harness" example (verified against ratatui 0.29 local source: `TestBackend` impls `Display`); structural conventions borrowed from `tests/app_test.rs` |

## Metadata

**Analog search scope:** `src/model/`, `src/app/`, `src/components/`, `src/theme/`, `src/utils/`, `src/storage/repo/`, `src/views/`, `tests/`
**Files scanned:** 14 read in full or targeted (computed.rs, avatar.rs, state.rs, update.rs:390-446, rpg.rs, sprites.rs, dashboard.rs, dashboard_view.rs, report.rs (storage), report.rs:170-194 (model), workspace.rs (grep), utils/mod.rs, utils/slug.rs, components/mod.rs, model/mod.rs, theme/mod.rs, app_test.rs)
**Pattern extraction date:** 2026-06-10
