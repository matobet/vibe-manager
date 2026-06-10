# Phase 1: Doorway Cards - Research

**Researched:** 2026-06-10
**Domain:** Rust TUI (ratatui 0.29) — brownfield dashboard rendering, computed metrics, urgency sorting
**Confidence:** HIGH (all findings grounded in direct codebase inspection of this repo)

## Summary

Phase 1 renders managers on the dashboard as 4-line "doorway cards" (identity / relationship / squad summary with named worst outlier / focus-following door hint) and folds the worst squad outlier into the manager's urgency score. The foundation is genuinely ready: `TeamMetrics` exists with unit tests (`src/model/computed.rs:146`), manager sprites with headbands exist (`src/theme/sprites.rs`), nested `team/` storage loads correctly, and fixtures contain a manager (`chris-wong`) with a 3-member squad including a natural worst outlier (Morgan Smith, mood 2).

However, three load-bearing gaps must be closed — all verified by grep, not assumption: (1) **`compute_team_metrics` is never called in production code** — only its own unit tests call it; `App::load_data` loads team member *profiles* into `report.team` but never their journal entries, so no team member summaries exist at runtime. (2) **`TeamMetrics` lacks everything DOOR-02 needs by name**: no outlier names, no outlier reasons, no next-in-rotation — only aggregate counts and a health score. (3) **Two per-report summary recompute sites (`delete_entry` at `state.rs:150`, `handle_save_entry` at `update.rs:429`) construct summaries via `compute_report_summary`, which hard-codes `team_metrics: None`** — any naive wiring of team metrics into `load_data` will be silently wiped the first time the user records a mood for a manager.

**Primary recommendation:** Extend `TeamMetrics` with `outliers: Vec<OutlierInfo>` (worst-first) and `next_in_rotation: Option<String>`; wire team-summary computation into `App::load_data` (team entries via the already-existing `ReportRepository::entries()` on `list_team_members()` repos); add a pure `manager_urgency_bonus(&TeamMetrics) -> i32` applied after `compute_report_summary`; render a new `DoorwayCard` component selected by `report_type` inside a heterogeneity-aware `AvatarGrid`. No new runtime dependencies. Test rendering with `ratatui::backend::TestBackend` + the already-declared-but-unused `insta` dev-dependency.

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| DOOR-01 | Manager renders as a 4-line doorway card (identity / relationship / squad summary / door hint) | Mockup anatomy extracted (docs/mockups/guild-halls.html:96-110); existing card geometry documented (18×9 grid, `avatar.rs`); `ReportSummary` lacks `title` field — must be added for the role on line 1; heterogeneous grid layout strategy below |
| DOOR-02 | Squad line: size, health bar, worst outlier by name; `(+N more)`; `★ all well · next: <name>` | Gap verified: `TeamMetrics` has no names/outliers/rotation — extension design below; team entries never loaded at runtime — `load_data` wiring design below; health-bar glyph spec (`▕▓▓░▏`) and new theme helpers specified |
| DOOR-03 | Door hint only on selected card; blank fourth line keeps height fixed | Selection plumbing documented (`AvatarGrid` passes `is_selected`, `app.selected_index` is linear); fixed-height card layout pattern below; snapshot test asserts equal height selected/unselected |
| DOOR-04 | Manager urgency incorporates worst squad outlier | Current scoring documented (`calculate_urgency_score`, computed.rs:260 — pure, tested with exact-value assertions); additive bonus design that leaves the IC path and existing tests untouched |
</phase_requirements>

## Project Constraints (from CLAUDE.md)

These carry the same authority as locked decisions:

- **GSD workflow enforcement**: file changes go through GSD commands (`/gsd-execute-phase` for this work)
- **Tech stack**: Rust + ratatui, single binary, **no new runtime dependencies expected** (this phase is UI/state work)
- **Storage**: metrics stay computed, never stored — `TeamMetrics`/outliers must remain runtime-derived
- **Quality gates**: CI enforces `cargo clippy --all-features -- -D warnings` and `cargo fmt --all -- --check`; tests via insta snapshots + fixtures in `tests/fixtures/`
- **Compatibility**: legacy root-level meeting files keep working; existing keybindings stay stable except the conscious Space rebind (which is Phase 2 behavior — see Open Questions)
- **Terminal**: glyphs must render in common monospace fonts; `◦︵◦` alignment caveat already handled in `sprites.rs:122` (fullwidth `︵` compensated by dropping a leading space)
- **Docs discipline**: after implementation, update `docs/implementation-status.md`, the status table in `docs/features/managing-managers.md` (note: this file currently has **uncommitted edits** — do not revert them), and `docs/roadmap.md`
- **Conventions** (from `.planning/codebase/CONVENTIONS.md` / CLAUDE.md): TEA purity (views never mutate `App`), repository pattern for all I/O, `snake_case`/`_idx` suffixes, `//!` module docs, serde `#[serde(default)]` for compatibility, `matches!()` for variant checks

## Architectural Responsibility Map

This is a single-process TUI; "tiers" map to the codebase's strict layers (`main → app → model ← storage`, `views`/`components` read-only over `app`+`model`):

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Outlier detection, rotation-next, `TeamMetrics` extension | `model/computed.rs` (pure) | — | Pure computation over `ReportSummary` slices; no I/O; unit-testable; metrics never stored (constraint) |
| Loading team member entries | `storage/repo/` (existing API) | `app/state.rs` orchestrates | `ReportRepository::list_team_members()` + `.entries().list()` already exist; only the call site in `load_data` is new |
| Urgency bonus + sort | `model/computed.rs` (bonus fn) | `app/state.rs` (applies before sort) | Keep `calculate_urgency_score` untouched (exact-value tests); bonus is additive, manager-only |
| Doorway card rendering | `components/` (new `doorway_card.rs`) | `theme/` for glyph helpers | Components are read-only widgets over summaries; matches `AvatarCard` pattern |
| Card-type dispatch + heterogeneous grid | `components/avatar.rs` (`AvatarGrid`) | — | Grid already owns geometry and selection mapping |
| Health bar / compact-week / name-abbreviation helpers | `theme/rpg.rs` + `utils/` | — | Matches existing `mood_gauge`/`progress_bar`/`name_to_slug` placement |
| Survival of team_metrics across recomputes | `app/state.rs` + `app/update.rs` | — | The two recompute sites live there; centralize into one helper |

## Standard Stack

### Core (all existing — no additions)

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `ratatui` | 0.29 (locked) | Layout, `Line`/`Span` composition, `Paragraph` | Existing UI framework; doorway card is plain `Line`s — no new widget types needed [VERIFIED: Cargo.toml + local registry source] |
| `crossterm` | 0.28 | Key events | Unchanged this phase [VERIFIED: Cargo.toml] |
| `chrono` | 0.4 | `days_since_meeting` already computed | Reused as-is [VERIFIED: Cargo.toml] |
| `unicode-width` | 0.1 | Width-aware truncation/padding if needed for line 1 | Already a dependency, currently unused — exactly its purpose if span-width math is needed [VERIFIED: Cargo.toml; grep shows no current import] |

### Supporting (dev-only, already declared)

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `insta` | 1.x (declared, **zero current call sites**) | Snapshot tests of rendered card buffers | DOOR-01/02/03 rendering verification — Wave 0 creates the first snapshot tests in this repo [VERIFIED: grep tests/ + src/ shows no `assert_snapshot` calls] |
| `ratatui::backend::TestBackend` | in ratatui 0.29 | Render into an in-memory buffer; `impl Display` lets insta snapshot it directly | Verified locally: `~/.cargo/registry/.../ratatui-0.29.0/src/backend/test.rs:229` has `impl fmt::Display for TestBackend` [VERIFIED: local crate source] |
| `tempfile` | 3.0 | Mutation-safe integration tests | Existing pattern in `tests/app_test.rs` [VERIFIED] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `TestBackend` + insta snapshots | Hand-written `Buffer` cell assertions | Snapshots are faster to write/review and match the project's declared (unused) insta dependency; cell assertions are brittle |
| New `DoorwayCard` component | Branching inside `AvatarCard::render` | `AvatarCard` is a vertical 18-wide card; the doorway card is a horizontally-composed full-width card — shapes differ enough that a separate component is cleaner and keeps IC rendering literally untouched (success criterion 5) |
| `cargo-insta` CLI for snapshot review | Plain `cargo test` + manual `.snap.new` promotion | `cargo-insta` is NOT installed locally [VERIFIED: `command -v cargo-insta` fails]. insta works without it (writes `.snap.new` files; rename to accept, or set `INSTA_UPDATE=accept` once). Do not add an install step to the plan unless the user asks. |

**Installation:** None. No new runtime or dev dependencies.

## Package Legitimacy Audit

**No external packages are installed in this phase.** All work uses crates already in `Cargo.lock`. slopcheck not run (nothing to check).

**Packages removed due to slopcheck [SLOP] verdict:** none
**Packages flagged as suspicious [SUS]:** none

## Architecture Patterns

### System Architecture Diagram (Phase 1 data flow)

```
                          ┌─────────────────────────────────────────────┐
 workspace/ (markdown)    │ App::load_data  (src/app/state.rs)          │
 ┌──────────────────┐     │                                             │
 │ jordan-lee/      │     │ for each report_repo:                       │
 │  _profile.md ────┼────▶│   load Report + entries                     │
 │  journal/*.md    │     │   compute_report_summary ──────────────┐    │
 │ chris-wong/      │     │   if has_team():                       │    │
 │  team/lee-kim/ ──┼────▶│     for each team_repo:                │    │
 │   _profile.md    │     │       load member Report + entries  ◀──┼── NEW: entries
 │   journal/*.md   │     │       compute member summary           │    │   (2nd-level cadence)
 └──────────────────┘     │     compute_team_metrics(+outliers) ───┤◀── NEW: outliers,
                          │     summary.team_metrics = Some(tm)    │    │   next_in_rotation
                          │     summary.urgency_score              │    │
                          │       += manager_urgency_bonus(tm) ◀───┼── NEW: DOOR-04
                          │   sort_by_key(Reverse(urgency_score))  │    │
                          └────────────────────────────────────────┼────┘
                                                                   ▼
                          ┌─────────────────────────────────────────────┐
                          │ render loop (read-only over &App)           │
                          │ views/dashboard_view → Dashboard            │
                          │   → AvatarGrid (src/components/avatar.rs)   │
                          │      for each summary, in sorted order:     │
                          │        report_type == Manager?              │
                          │        ├─ yes → DoorwayCard (NEW, full row) │
                          │        │    line1: name — title       M2    │
                          │        │    line2: ♥♥♥♥♡ ✓ you: 3d          │
                          │        │    line3: squad N ▕▓▓░▏76% ⚠ …     │
                          │        │    line4: hint iff is_selected     │
                          │        └─ no  → AvatarCard (UNCHANGED 18×9) │
                          └─────────────────────────────────────────────┘
 mutation paths (must NOT wipe team_metrics):
   delete_entry (state.rs:150) ─┐
   handle_save_entry (update.rs:429) ─┴─▶ centralized re-summary helper (NEW)
```

### Recommended Project Structure (changes only)

```
src/
├── model/
│   └── computed.rs        # EXTEND: OutlierInfo, TeamMetrics fields, outlier/rotation
│                          #   computation, manager_urgency_bonus; keep all existing fns intact
├── app/
│   ├── state.rs           # EXTEND: load_data wires team entries + metrics + bonus;
│   │                      #   extract one re-summary helper used by all recompute sites
│   └── update.rs          # EDIT: handle_save_entry uses the helper (no metric wipe)
├── components/
│   ├── doorway_card.rs    # NEW: DoorwayCard widget (4 lines, sprite left, text right)
│   ├── avatar.rs          # EXTEND: AvatarGrid heterogeneous layout (manager rows vs IC grid)
│   └── mod.rs             # EXTEND: pub use doorway_card::DoorwayCard
├── theme/
│   └── rpg.rs             # EXTEND: health_bar(), format_compact_age() ("3d"/"6w")
└── utils/                 # EXTEND: abbreviate_name("Sam Taylor") -> "Sam T"
tests/
└── ui_test.rs             # NEW (Wave 0): TestBackend + insta snapshot harness
```

### Pattern 1: Extend TeamMetrics with named outliers (DOOR-02)

**What:** `compute_team_metrics` already receives `&[ReportSummary]` which contains `name`, `urgency_score`, `is_overdue`, `recent_mood`, `mood_trend`, `days_since_meeting` — everything needed. Extend the struct, don't bolt on a parallel computation.

**When to use:** computed at load time, never stored (project constraint).

**Example (shape, grounded in existing fields):**
```rust
// src/model/computed.rs — extension
#[derive(Debug, Clone)]
pub struct OutlierInfo {
    pub name: String,                     // full name; view abbreviates
    pub urgency_score: i32,               // for worst-first ordering + DOOR-04 bonus
    pub mood_trend: Option<MoodTrend>,    // drives "mood ↘" label
    pub recent_mood: Option<u8>,          // drives "mood 2" label
    pub is_overdue: bool,
    pub days_since_meeting: Option<i64>,  // drives "· 6w"
}

pub struct TeamMetrics {
    // ...existing five fields unchanged...
    /// Members needing attention, worst-first (empty = all well)
    pub outliers: Vec<OutlierInfo>,
    /// Member longest since last meeting (rotation head) — "next: <name>"
    pub next_in_rotation: Option<String>,
}

/// Recommended outlier predicate (Claude's discretion — see Open Questions):
fn is_outlier(s: &ReportSummary) -> bool {
    s.is_overdue
        || s.recent_mood.is_some_and(|m| m <= 2)
        || s.mood_trend == Some(MoodTrend::Falling)
}
// worst-first: sort outliers by urgency_score descending (stable sort, ties keep order)
// next_in_rotation: active member with max days_since_meeting; None days (never met) ranks first
```

### Pattern 2: Manager urgency bonus that can't break IC tests (DOOR-04)

**What:** `calculate_urgency_score` (computed.rs:260) is private, pure, and pinned by exact-value unit tests (`test_urgency_overdue` asserts `== 30`, etc.). Do not change its signature or values. Add a separate additive bonus.

**Example:**
```rust
// src/model/computed.rs
/// Half the worst outlier's urgency, capped so a manager's own 1-on-1 state
/// still dominates their score. Any positive bonus satisfies DOOR-04's
/// "otherwise-comparable managers" ordering requirement.
pub fn manager_urgency_bonus(metrics: &TeamMetrics) -> i32 {
    metrics
        .outliers
        .first()
        .map(|o| (o.urgency_score / 2).min(50))
        .unwrap_or(0)
}

// src/app/state.rs::load_data — applied before the existing sort:
// summary.urgency_score += manager_urgency_bonus(&tm);
// all_data.sort_by_key(|d| std::cmp::Reverse(d.2.urgency_score));  // unchanged
```

### Pattern 3: Centralized summary recompute (prevents the metric-wipe bug)

**What:** Three call sites construct `ReportSummary` today: `load_data` (state.rs:79), `delete_entry` (state.rs:150), `handle_save_entry` (update.rs:429). The latter two would silently reset `team_metrics: None` and drop the urgency bonus. Direct-report entry mutations don't change squad data, so preserving the previous metrics is correct and cheap.

**Example:**
```rust
// src/app/state.rs
/// Recompute the summary for one report, preserving team metrics
/// (squad data is unaffected by the manager's own entries).
pub(crate) fn recompute_summary(&mut self, report_idx: usize) {
    let prev_team = self.summaries[report_idx].team_metrics.take();
    let mut summary = compute_report_summary(
        &self.reports[report_idx],
        &self.entries_by_report[report_idx],
        self.workspace.config.settings.overdue_threshold_days,
    );
    if let Some(tm) = prev_team {
        summary.urgency_score += manager_urgency_bonus(&tm);
        summary.team_metrics = Some(tm);
    }
    self.summaries[report_idx] = summary;
    self.workspace_summary = compute_workspace_summary(&self.summaries);
}
```
Both existing recompute sites delegate to this. (Note: those sites already do not re-sort the roster after mood changes — that is existing behavior; keep it, per success criterion 5.)

### Pattern 4: Doorway card as horizontal composition (DOOR-01, DOOR-03)

**What:** Sprite column on the left (manager sprites are exactly 4 lines — same as the card's 4 text lines), text column on the right. No outer `Block` (the mockup has none; the sprite frame is the visual anchor). Right-aligned `M2` badge via a horizontal split rather than manual width math.

**Example:**
```rust
// src/components/doorway_card.rs (sketch)
pub const DOORWAY_CARD_HEIGHT: u16 = 5; // 4 content lines + 1 spacer (mockup spacing)

pub struct DoorwayCard<'a> { summary: &'a ReportSummary, is_selected: bool }

impl DoorwayCard<'_> {
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(9), Constraint::Min(0)]) // sprite | text
            .split(area);
        // left: FaceSprite::from_summary(...).lines() — 4 lines for M-levels
        // right line 1: "JORDAN LEE — Eng Manager" + right-aligned level badge
        //   (split the line-1 rect [Min(0), Length(4)] and render two Paragraphs)
        // right line 2: mood_gauge + overdue icon + "you: " + format_compact_age(days) (+ "zZ")
        // right line 3: squad line (see Pattern 5)
        // right line 4: if self.is_selected { "▸ Space to visit squad" } else { Line::from("") }
        //   — ALWAYS emit 4 lines so card height never changes (DOOR-03)
    }
}
```
Selection styling: follow existing conventions — selected uses `style_title()` / `COLOR_SECONDARY` (yellow) like `AvatarCard`; unselected uses `summary.color` for name/sprite and `style_muted()` for the rest (mockup renders unselected cards dim).

### Pattern 5: Squad line composition (DOOR-02)

```rust
// squad 4 ▕▓▓▓▓▓▓░░▏76%  ⚠ Sam T: mood ↘ · 6w (+2 more)
// squad 5 ▕▓▓▓▓▓▓▓▓░▏90%  ★ all well · next: Ana P
let tm = summary.team_metrics.as_ref()?;
let mut spans = vec![
    Span::styled(format!("squad {} ", tm.team_size), style_muted()),
    Span::styled(health_bar(tm.team_health_score, 8), style_header()),
    Span::styled(format!("{}%", tm.team_health_score), Style::default().fg(COLOR_TEXT)),
    Span::raw("  "),
];
match tm.outliers.first() {
    Some(worst) => {
        let label = outlier_label(worst); // "mood ↘" | "mood 2" | "overdue" priority order
        let age = format_compact_age(worst.days_since_meeting); // "6w" / "9d" / "never"
        spans.push(Span::styled(
            format!("⚠ {}: {} · {}", abbreviate_name(&worst.name), label, age),
            style_danger(), // magenta per palette ("bad" is never red)
        ));
        if tm.outliers.len() > 1 {
            spans.push(Span::styled(format!(" (+{} more)", tm.outliers.len() - 1), style_muted()));
        }
    }
    None => {
        let next = tm.next_in_rotation.as_deref().map(abbreviate_name);
        spans.push(Span::styled(
            match next {
                Some(n) => format!("★ all well · next: {}", n),
                None => "★ all well".to_string(),
            },
            style_success(),
        ));
    }
}
```

### Pattern 6: Heterogeneous grid layout (criterion 5: ICs untouched)

**What:** `AvatarGrid::render` currently lays uniform 18×9 cards. Managers need full-width 5-row cards while ICs keep their exact current cards, in a single urgency-sorted sequence. Key insight verified in `input.rs`: dashboard navigation is **purely linear** (`h`/`k` → `SelectPrev`, `l`/`j` → `SelectNext`) — selection semantics do not depend on grid geometry, so mixed row shapes break nothing.

**Algorithm:** walk `summaries` in sorted order; a manager emits one full-width row (`Length(DOORWAY_CARD_HEIGHT)`); a run of consecutive ICs is chunked into rows of `(width / 18).max(1)` cards (`Length(9)`), exactly as today. `global_idx == selected` logic carries over unchanged.

### Anti-Patterns to Avoid

- **Mutating `App` from views/components** — existing project anti-pattern; doorway card takes `&ReportSummary` only
- **Bypassing the repository layer** — team entries come from `team_repo.entries().list()`, never `fs::read_dir`
- **Storing computed data** — outliers/rotation/health are derived in `computed.rs`, never written to disk
- **Changing `calculate_urgency_score` internals** — its tests assert exact values; ICs must score identically before/after (criterion 5)
- **Pushing team-member entries into `entries_by_report`** — that Vec is index-aligned with `reports`/`summaries` (documented brownfield concern); team data must live inside the manager's own slot (e.g., consumed immediately to build `TeamMetrics`), not as extra parallel elements

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Buffer-level render assertions | Custom terminal capture | `ratatui::backend::TestBackend` + `insta::assert_snapshot!` | TestBackend implements `Display` (verified in 0.29 source); insta already declared |
| Span width math for the right-aligned `M2` badge | Manual char counting (`.len()` is bytes, wrong for `♥`/`◕`) | Horizontal `Layout` split + `Alignment::Right` paragraph; if string math is unavoidable, `unicode-width` (already a dep) | Multi-byte glyphs (`♥`=3 bytes, `︵` fullwidth) make byte-length math wrong; layout splitting avoids it entirely |
| Relative time text | New date formatting | Existing `format_days_ago` (chrono-humanize) for long form; new tiny `format_compact_age` only for the "6w"/"3d" compact form (mockup-specific, ~6 lines) | The compact form is genuinely new; everything else exists |
| Health bar | New rendering concept | Pattern-match existing `progress_bar`/`mood_gauge` helpers in `theme/rpg.rs` (same `format!`+`repeat` style, same test style) | Conventions doc requires numeric value alongside gauges — mockup's `76%` suffix already satisfies it |
| Manager detection in the view | Re-deriving from level string | `summary.report_type.is_manager()` / `matches!` per conventions | `ReportType` is already on `ReportSummary` |

**Key insight:** every input DOOR-01..04 needs already exists on `ReportSummary` or is one struct-extension away; this phase is composition, not invention. The risk is integration (wiring, recompute paths, layout), not algorithms.

## Common Pitfalls

### Pitfall 1: team_metrics silently wiped after wiring (the big one)
**What goes wrong:** Doorway cards render correctly on launch, then a manager's squad line goes blank (and their urgency drops) after recording a mood observation or deleting an entry for that manager.
**Why it happens:** `delete_entry` (`src/app/state.rs:150`) and `handle_save_entry` (`src/app/update.rs:429`) rebuild the summary via `compute_report_summary`, which sets `team_metrics: None` (computed.rs:141).
**How to avoid:** Centralize recompute (Pattern 3); both sites delegate to it.
**Warning signs:** An integration test that records an observation for `chris-wong` and then asserts `summaries[idx].team_metrics.is_some()` — make this an explicit test.

### Pitfall 2: Parallel-Vec misalignment
**What goes wrong:** Selecting card N opens a different person's detail.
**Why it happens:** `reports`, `entries_by_report`, `summaries` are three index-aligned Vecs (documented in `.planning/STATE.md` concerns). Adding team data as extra elements, or filtering one Vec but not the others, breaks alignment.
**How to avoid:** Team member summaries are consumed inside the manager's loop iteration to produce `TeamMetrics`; nothing new is appended to the parallel Vecs in Phase 1.
**Warning signs:** Any plan task that says "push team member summaries into `App.summaries`".

### Pitfall 3: Time-dependent tests via fixtures
**What goes wrong:** Snapshot/assertion tests pass today, fail next month.
**Why it happens:** Fixture journal entries are dated January 2026; `compute_report_summary` measures against `Local::now()`. Today every fixture member is months overdue; `days_since_meeting`, overdue flags, and urgency all drift daily.
**How to avoid:** Rendering snapshot tests construct `ReportSummary`/`TeamMetrics` literals with fixed values (the pattern already used by `create_test_summary` in computed.rs tests). Fixture-backed integration tests assert *structure* (e.g., "chris-wong has team_metrics with ≥1 outlier", "Morgan is the worst outlier" — mood 2 is date-independent), never exact day counts.
**Warning signs:** A snapshot containing "20 weeks" or any absolute day count derived from fixtures.

### Pitfall 4: Glyph width and font rendering on the squad line
**What goes wrong:** Misaligned columns or doubled-width warning glyphs in some terminals.
**Why it happens:** `⚠` (U+26A0) is East-Asian-ambiguous width and emoji-presentation-eligible; the HTML mockup explicitly appends U+FE0E (text presentation selector) to force narrow rendering. `︵` in the mood-2 face is fullwidth (already compensated in sprites.rs:122). `▕` (U+2595) / `▏` (U+258F) health-bar caps are Block Elements — same range as the already-used `█░`, so support is equivalent [ASSUMED for exotic fonts; the range is already shipped in this app].
**How to avoid:** Reuse the existing bare `ICON_WARNING` constant (already rendered on the dashboard today without FE0E — consistency beats theoretical correctness here); keep the health bar pure Block Elements; never compose fixed-width strings around the mood-2 face outside `FaceSprite`.
**Warning signs:** New string literals with `⚠️` (emoji form) or manually padded strings containing kaomoji.

### Pitfall 5: Breaking exact-value urgency tests / IC ordering
**What goes wrong:** `test_urgency_overdue` etc. fail, or IC cards reorder (violating criterion 5).
**Why it happens:** Editing `calculate_urgency_score` weights instead of adding a manager-only bonus.
**How to avoid:** Pattern 2 — additive `manager_urgency_bonus`, applied only when `team_metrics.is_some()`.
**Warning signs:** Any diff inside `calculate_urgency_score`'s body.

### Pitfall 6: ReportSummary struct-literal breakage
**What goes wrong:** Compile errors in tests after extending structs.
**Why it happens:** `ReportSummary` is constructed as a literal in `create_test_summary` (computed.rs:467) and `TeamMetrics` in its own tests; adding fields (`title`, `outliers`, `next_in_rotation`) breaks every literal.
**How to avoid:** Update the test constructors in the same task that extends the structs; consider `..Default::default()` only if a `Default` impl is added deliberately (not required).
**Warning signs:** A plan that extends `computed.rs` structs without listing its test module in the same task.

### Pitfall 7: Help text / Space semantics drift (scope trap)
**What goes wrong:** Phase 1 accidentally implements Phase 2.
**Why it happens:** The door hint says "Space to visit squad", but Space currently maps to `Msg::ViewReport` (input.rs:58) and the help modal says "Enter/Space — View member details" (modal/help.rs:33). Halls don't exist yet.
**How to avoid:** Phase 1 renders the hint only (success criterion 3 mandates the exact text). Space behavior and help text change in Phase 2 (HALL-01/HALL-07 territory). Accept the one-phase mismatch — it's the roadmap's deliberate staging.
**Warning signs:** Phase 1 tasks touching `input.rs` Space handling or `ViewMode` variants.

### Pitfall 8: Zero-member squads
**What goes wrong:** Panic-free but nonsense line 3 (`squad 0 ▕░░░░░░░░▏0%`, health score 0 reads as "maximally troubled").
**Why it happens:** `manager-minimal` fixture has `report_type: manager` but **no `team/` directory**; `calculate_team_health_score` returns 0 for empty teams; `has_team()` is false so metrics may be `None` entirely.
**How to avoid:** Decide and test both shapes: manager with no `team/` dir (`team_metrics: None` — recommend rendering line 3 muted, e.g., `squad 0 · no members yet`) and manager with empty `team/` dir. Both exist in fixtures/storage tests today.
**Warning signs:** `unwrap()` on `team_metrics` or indexing `outliers[0]`.

## Code Examples

### Health bar helper (mockup-faithful, convention-faithful)
```rust
// src/theme/rpg.rs — alongside progress_bar()
/// Health bar with end caps: ▕▓▓▓▓▓▓░░▏ (score 0-100 over `cells` cells)
pub fn health_bar(score: u8, cells: usize) -> String {
    let filled = (score.min(100) as usize * cells).div_ceil(100).min(cells);
    format!("▕{}{}▏", "▓".repeat(filled), "░".repeat(cells - filled))
}

/// Compact age: 0-13 days -> "Nd", 14+ -> "Nw", None -> "never"
pub fn format_compact_age(days: Option<i64>) -> String {
    match days {
        None => "never".to_string(),
        Some(d) if d < 14 => format!("{}d", d.max(0)),
        Some(d) => format!("{}w", d / 7),
    }
}
```

### Name abbreviation (utils, with edge cases)
```rust
// src/utils/ — alongside name_to_slug
/// "Sam Taylor" -> "Sam T", "Jonas" -> "Jonas", "Mary Jane Watson" -> "Mary W"
pub fn abbreviate_name(name: &str) -> String {
    let mut parts = name.split_whitespace();
    let first = parts.next().unwrap_or(name);
    match parts.last().and_then(|l| l.chars().next()) {
        Some(initial) => format!("{} {}", first, initial),
        None => first.to_string(),
    }
}
```

### Snapshot test harness (Wave 0 — first insta usage in repo)
```rust
// tests/ui_test.rs
// Source: ratatui 0.29 TestBackend (local registry source, backend/test.rs) + insta docs
use ratatui::{backend::TestBackend, Terminal};

#[test]
fn doorway_card_selected_shows_hint() {
    let summary = manager_summary_fixture(); // constructed literal — NOT loaded from fixtures
    let backend = TestBackend::new(60, 5);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|f| {
            vibe_manager::components::DoorwayCard::new(&summary, true).render(f, f.area());
        })
        .unwrap();
    insta::assert_snapshot!(terminal.backend());
}
// DOOR-03 companion: render selected and unselected, assert both buffers are 5 rows
// and differ ONLY in the fourth content line.
```

### Wiring team metrics in load_data (integration sketch)
```rust
// src/app/state.rs::load_data — inside the existing filter_map closure
if report_repo.has_team() {
    let mut member_summaries = Vec::new();
    for team_repo in report_repo.list_team_members().unwrap_or_default() {
        if let Ok(member) = team_repo.load() {
            let member_entries = team_repo.entries().list().unwrap_or_default();
            member_summaries.push(compute_report_summary_with_frequency(
                &member,
                &member_entries,
                second_level_frequency_days, // see Open Question 2
                overdue_threshold,
            ));
            report.team.push(member); // existing behavior preserved
        }
    }
    let tm = compute_team_metrics(&member_summaries);
    summary.urgency_score += manager_urgency_bonus(&tm);
    summary.team_metrics = Some(tm);
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `TeamMetrics` computed but unrendered (averages only) | Outlier-first metrics with names ("never just a score") | This milestone (design contract §Design Principles) | Struct extension, not replacement — existing fields/tests stay |
| Uniform 18×9 card grid | Heterogeneous grid: full-width doorway rows + IC grid rows | This phase | `AvatarGrid` gains layout branching; `AvatarCard` untouched |
| Space = synonym for Enter | Space = container verb (hint in Phase 1, behavior in Phase 2) | Phases 1–2 | Phase 1 touches rendering only |

**Deprecated/outdated (context, not this phase's work):**
- `serde_yaml` 0.9 is upstream-archived (documented in CONCERNS.md) — no YAML changes in this phase, no action
- `Level`/`MeetingFrequency` enums are dead code (CONCERNS.md) — doorway card uses the string `summary.level` like everything else; do not resurrect the enums in this phase
- `walkdir`/`dirs` unused deps — out of scope
- ratatui 0.30+ exists upstream [ASSUMED]; project locks 0.29 and "no new runtime dependencies / upgrades" is a constraint — do not upgrade

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `▕` (U+2595) and `▏` (U+258F) render single-width in the user's terminal fonts like the already-used `█░` | Pitfall 4 / health_bar | Squad line misaligns by 1-2 cells; trivial fallback: replace caps with nothing or `[`/`]` |
| A2 | Outlier predicate = `is_overdue || mood <= 2 || trend == Falling`, worst = highest urgency_score | Pattern 1 | Different members flagged than user expects; pure-function change, cheap to retune |
| A3 | Urgency bonus = `worst.urgency_score / 2` capped at 50 | Pattern 2 | Sort feels wrong; single-constant retune, criterion 4 only needs "any positive bonus" |
| A4 | Doorway card height 5 (4 content + 1 spacer) and sprite column width 9 | Pattern 4 | Visual density mismatch vs mockup; constants, snapshot tests make retuning safe |
| A5 | "next in rotation" ranks by `days_since_meeting` (meetings) rather than any-entry timestamp | Pattern 1 | Spec §3 says "time since last journal entry"; for one-entry-per-member fixtures these coincide. Phase 3 (SKIP-03) formalizes rotation — flag for planner |
| A6 | ratatui 0.30+ exists upstream | State of the Art | None — upgrades are out of scope by constraint |

## Open Questions

1. **Which cadence governs 2nd-level overdue/outlier status in Phase 1?**
   - What we know: spec says skip-level overdue uses `default_2nd_level_frequency` ("monthly" default, field exists in `WorkspaceSettings` [VERIFIED]) with per-member `meeting_frequency` override — but that requirement (SKIP-02) is Phase 3. `ReportProfile.meeting_frequency` is a `String` with a serde-injected `"biweekly"` default, so "absent" vs "explicitly biweekly" is indistinguishable without an `Option<String>` migration.
   - What's unclear: using each member's loaded `meeting_frequency` (biweekly default) would mark nearly every skip-level overdue (skip-levels run monthly+), making every squad look troubled — defeating DOOR-02's signal.
   - Recommendation: Phase 1 computes team member summaries against `default_2nd_level_frequency` days for **all** members via a frequency-parameterized variant of `compute_report_summary` (existing fn delegates, signature preserved). Defer the per-member-override `Option<String>` migration to Phase 3/SKIP-02. (All current team fixtures say `monthly` explicitly, which coincides with the default — low fixture risk.)

2. **Door hint vs actual Space behavior for one phase.**
   - What we know: criterion 3 mandates rendering `▸ Space to visit squad` on the selected manager card; Space currently equals Enter (`input.rs:58`) and halls don't exist until Phase 2; criterion 5 says existing keys unchanged.
   - Recommendation: render the hint exactly as specced, change nothing in `input.rs`, leave help-modal text for Phase 2 (HALL-07 covers help in Phase 3). Accept the documented one-phase mismatch.

3. **Is the `GUILD ROSTER · 5 direct · 14 in banner` header in Phase 1 scope?**
   - What we know: it appears in the mockup's screen 1; it is NOT in DOOR-01..04 or the success criteria; STATE.md carries a concern that `compute_extended_workspace_summary` (never called) should be wired in "Phase 1/2" rather than duplicated.
   - Recommendation: treat as a small optional sub-task — wiring `compute_extended_workspace_summary` in `load_data` is ~5 lines (second-level counts are already loaded) and retires a documented concern; the header *text* change can ride along or wait for Phase 2's breadcrumb work. Planner's call; do not let it grow the phase.

4. **Manager-with-no-team rendering (line 3).**
   - What we know: `manager-minimal` fixture (M1, no `team/` dir) will have `team_metrics: None`; `team_health_score` is 0 for empty teams.
   - Recommendation: muted `squad 0 · no members yet` for both `None` metrics and `team_size == 0`; never render a 0% health bar as if it were data. Needs an explicit snapshot test.

5. **Selected-card styling for doorway cards.**
   - What we know: mockup says "cyan frame + background" for selection; existing `AvatarCard` selection uses double border + yellow (`COLOR_SECONDARY`).
   - Recommendation: follow the codebase convention (yellow `style_title()` on sprite + name + hint) over the mockup's cyan — consistency with IC card selection matters more than mockup fidelity. Claude's discretion at plan time.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| rustc / cargo | build, test | ✓ | 1.92.0 | — |
| clippy | CI gate parity | ✓ | 0.1.92 (rustc 1.92) | — (PROJECT.md says "clippy 1.96+"; local is the rustc-1.92 bundle — likely a doc typo; `-D warnings` runs fine locally) |
| ratatui 0.29 source (TestBackend) | snapshot tests | ✓ | 0.29.0 in local registry | — |
| insta | snapshot tests | ✓ (dev-dep declared) | "1.0" (resolves 1.x) | — |
| cargo-insta CLI | snapshot review ergonomics | ✗ | — | Not required: insta writes `.snap.new` files reviewable/promotable by hand, or `INSTA_UPDATE=accept cargo test` for first generation |
| cargo-llvm-cov | coverage (`cargo cov` alias) | not checked | — | Optional; CI installs it; not needed for this phase's gates |

**Missing dependencies with no fallback:** none.
**Missing dependencies with fallback:** cargo-insta (manual `.snap.new` promotion works).

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | cargo test (libtest) + insta 1.x snapshots (first use in repo) |
| Config file | none needed (insta defaults; snapshots land in `tests/snapshots/`) |
| Quick run command | `cargo test <module-or-test-name>` (e.g., `cargo test computed`, `cargo test doorway`) |
| Full suite command | `cargo test --all-features` |

### Phase Requirements → Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| DOOR-01 | 4-line doorway card renders (identity/relationship/squad/hint) | snapshot | `cargo test doorway_card` | ❌ Wave 0 (`tests/ui_test.rs`) |
| DOOR-02 | Outlier naming, `(+N more)`, `★ all well · next:` and health bar | unit + snapshot | `cargo test computed` + `cargo test doorway_card` | unit: ✅ extend `src/model/computed.rs` tests; snapshot: ❌ Wave 0 |
| DOOR-03 | Hint only on selected card; equal height selected/unselected | snapshot + assertion | `cargo test doorway_card_height` | ❌ Wave 0 |
| DOOR-04 | Manager with troubled squad sorts above comparable healthy-squad manager | unit + integration | `cargo test manager_urgency` + `cargo test app_` (fixture-based sort assertion using chris-wong) | unit: ✅ extend computed.rs tests; integration: ✅ extend `tests/app_test.rs` |
| (regression) | team_metrics survives `SaveEntry`/`delete_entry` | integration | `cargo test team_metrics_survive` | ❌ Wave 0 (add to `tests/app_test.rs`) |
| (regression) | IC card rendering byte-identical; existing tests green | full suite | `cargo test --all-features` | ✅ existing |
| Gates | lint + format | static | `cargo clippy --all-features -- -D warnings && cargo fmt --all -- --check` | ✅ |

### Sampling Rate
- **Per task commit:** `cargo test <touched module>` + `cargo clippy --all-features -- -D warnings`
- **Per wave merge:** `cargo test --all-features && cargo fmt --all -- --check`
- **Phase gate:** full suite + clippy + fmt green before `/gsd:verify-work`

### Wave 0 Gaps
- [ ] `tests/ui_test.rs` — TestBackend + insta harness with constructed-literal summary builders (covers DOOR-01, DOOR-02 rendering, DOOR-03); first snapshot generation via `INSTA_UPDATE=accept cargo test` then commit `.snap` files
- [ ] `tests/app_test.rs` additions — team_metrics survival test; urgency-sort integration test (date-independent assertions only, per Pitfall 3)
- [ ] Optional fixture: a manager with an all-healthy squad to exercise `★ all well` end-to-end — NOT required if snapshot tests use constructed literals (recommended); fixture moods are date-fragile

## Security Domain

Local single-user TUI; no network, no auth, no new input surfaces this phase (reads the same YAML frontmatter through the existing `parse_frontmatter` path).

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | single-user local tool by design |
| V3 Session Management | no | — |
| V4 Access Control | no | filesystem permissions only (documented in CONCERNS.md) |
| V5 Input Validation | yes (existing) | serde_yaml typed deserialization with `#[serde(default)]`; this phase adds no new parsing — team member profiles/entries go through the existing repository + frontmatter path |
| V6 Cryptography | no | no secrets, no crypto |

### Known Threat Patterns for this stack

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Malformed YAML in team member `_profile.md` / journal frontmatter | DoS (panic) | Existing: `serde_yaml` errors map to `StorageError::Yaml`; load paths use `.ok()`/`unwrap_or_default()` — replicate that tolerance when loading team entries (a corrupt 2nd-level file must not crash the dashboard) |
| Out-of-range mood values in hand-edited files | Tampering (logic) | Existing one-sided validation (CONCERNS.md known bug): `JournalEntry::mood()` filters 1–5 on read — team summaries inherit this safely |
| Sensitive personal data on the new card surface | Information disclosure | Doorway card shows name + mood + recency only — same exposure class as existing cards; no new fields (birthday/partner stay off the dashboard) |

## Sources

### Primary (HIGH confidence — direct inspection of this repository, 2026-06-10)
- `src/model/computed.rs` — ReportSummary/TeamMetrics/urgency internals, exact-value tests
- `src/app/state.rs`, `src/app/update.rs`, `src/app/input.rs`, `src/app/mod.rs` — load/sort/recompute sites, linear navigation, Msg/ViewMode inventory
- `src/components/avatar.rs`, `src/components/dashboard.rs`, `src/views/dashboard_view.rs` — current card geometry (18×9), grid algorithm, selection plumbing
- `src/theme/rpg.rs`, `src/theme/sprites.rs` — palette, gauge helpers, 4-line manager sprites, `◦︵◦` compensation
- `src/storage/repo/report.rs`, `src/model/report.rs`, `src/model/workspace.rs` — `list_team_members`/`entries()` APIs, `default_2nd_level_frequency` field
- `docs/features/managing-managers.md` (uncommitted working copy — design contract) + `docs/mockups/guild-halls.html` (exact card text extracted)
- `tests/fixtures/` (chris-wong squad incl. Morgan mood-2 outlier; manager-minimal no-team edge case), `tests/app_test.rs`, `tests/storage_test.rs`
- `.planning/codebase/CONCERNS.md`, `.planning/STATE.md` — parallel-Vec concern, `compute_extended_workspace_summary` never called, HideHelp bug
- `~/.cargo/registry/.../ratatui-0.29.0/src/backend/test.rs` — TestBackend + `impl Display` verified in the locked version's source

### Secondary (MEDIUM confidence)
- None required — no external libraries researched beyond locked local sources.

### Tertiary (LOW confidence)
- A1 (cap-glyph font coverage), A6 (newer ratatui exists) — flagged in Assumptions Log; neither blocks planning.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — zero new dependencies; all tools verified present and at locked versions
- Architecture: HIGH — every integration point read in source; the three gaps (unwired metrics, missing outlier fields, recompute wipe) verified by grep, not inferred
- Pitfalls: HIGH — each pitfall cites the exact file:line that causes it
- Open questions: deliberate plan-time decisions (cadence semantics, optional header, styling), not unknowns

**Research date:** 2026-06-10
**Valid until:** 2026-07-10 (stable: locked dependencies, single-developer repo; re-verify only if `main` moves under the phase)
