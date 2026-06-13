# Managing Managers Feature

Track managers as direct reports (managers of managers), with visibility into their teams.

## Implementation Status

| Requirement | Status | Notes |
|-------------|--------|-------|
| Data model: Report type (IC/Manager) | ✅ Done | `ReportType` enum in `model/report.rs` |
| Data model: M-track levels (M1-M5) | ✅ Done | `Level` enum extended |
| Data model: Team nesting | ✅ Done | `team/` subdirectory structure |
| Storage: Load nested team directories | ✅ Done | `list_team_member_dirs()` |
| Storage: Manager info in profile | ✅ Done | `manager_info` field |
| Computed: Team metrics | ✅ Done | `TeamMetrics` struct |
| Dashboard: Manager doorway cards | ✅ Done | `DoorwayCard` component; squad bar, named worst outlier, urgency bonus |
| Hall navigation (drill-down) | ✅ Done | Roster path stack, Space/Esc/boundary-h, breadcrumb + health bar |
| Skip-level rotation tracking | 📋 Planned | Rotation strip, overdue vs 2nd-level cadence |
| Manager detail: Banner panel | 📋 Planned | Squad face strip, Tab-focusable bridge |

---

## Overview

Extend Vibe Manager to support **managers as direct reports**, enabling:
1. **Direct manager reports** - Track 1-on-1s with your manager reports
2. **Second-level reports** - Visibility into people who report to your managers
3. **Skip-level meetings** - Periodic check-ins with indirect reports on a rotation

The UI follows the **Guild Halls** design: the dashboard only ever shows direct
reports; manager cards are *doorways* into their team's *hall*, which is rendered
by the same dashboard component re-rooted at that team. An interactive HTML
mockup of all screens lives at `docs/mockups/guild-halls.html`.

### Design Principles

1. **Scales gracefully** — works the same for 1 manager + 5 ICs or a full org
   tree. Every screen shows one roster; hierarchy caps on-screen content.
2. **Outliers, not averages** — a card's job is to *name the person* who needs
   attention. Averages hide problems; a team of one miserable person and four
   happy ones must not look "fine".
3. **Rotation-driven skip-levels** — the app tracks who you are due to see next
   against `default_2nd_level_frequency` and flags long-unseen people.
4. **Density degrades by summarizing harder, never by adding layout machinery**
   — counts replace lists, faces replace name-tags.

### Terminology

| Term | Description |
|------|-------------|
| Report | Anyone you manage (IC or manager) |
| Manager report | A direct report who manages others |
| 2nd-level report | Someone who reports to your manager report |
| Skip-level | A meeting with a 2nd-level report |
| Doorway card | A manager's dashboard card: summary outside, `Space` to enter |
| Hall | A team's roster screen (the dashboard re-rooted at that team) |
| Banner | The squad face-strip panel inside a manager's detail view |
| Rotation | The skip-level queue ordered by time since last meeting |

### RPG Theme
> Your party includes **lieutenants** who lead their own sub-parties. Each
> lieutenant's card is a doorway to their hall; their banner shows the faces
> of everyone marching behind them.

---

## 1. Data Model

### 1.1 Report Type

```yaml
# _profile.md frontmatter
---
name: Jordan Lee
title: Engineering Manager
level: M2              # M-track for managers (M1-M5)
report_type: manager   # manager | ic (default: ic)
meeting_frequency: weekly

# Manager-specific fields (only for report_type: manager)
manager_info:
  team_name: "Platform Team"
---
```

### 1.2 Level Tracks

| IC Track | Manager Track |
|----------|---------------|
| P1 - Junior | M1 - Team Lead (2-4 reports) |
| P2 - Mid | M2 - Engineering Manager (4-8 reports) |
| P3 - Senior | M3 - Senior Manager |
| P4 - Staff | M4 - Director |
| P5 - Principal | M5 - Senior Director/VP |

### 1.3 Workspace Structure

```
workspace/
├── .vibe-manager
├── jordan-lee/                   # Manager (direct report)
│   ├── _profile.md              # report_type: manager
│   ├── journal/                 # Your 1-on-1s with Jordan
│   │   └── 2026-01-15T100000.md
│   └── team/                    # Jordan's team (2nd-level)
│       ├── alex-chen/
│       │   ├── _profile.md     # 2nd-level report
│       │   └── journal/        # Skip-level meeting notes
│       │       └── 2026-01-20.md
│       └── sam-taylor/
│           └── _profile.md
├── chris-wong/                   # Another manager
│   ├── _profile.md
│   └── team/
│       └── ...
└── pat-kumar/                    # IC direct report (no team/)
    ├── _profile.md
    └── journal/
        └── 2026-01-18.md
```

**Key Rules:**
- Manager folders have a `team/` subdirectory
- 2nd-level profiles/meetings live inside `team/{report}/`
- Journal entries in `journal/` subdirectory (legacy root entries still supported)
- Skip-level notes stored in 2nd-level report's `journal/` folder

### 1.4 Team Metrics

**Per Manager:**
| Metric | Description |
|--------|-------------|
| `team_size` | Count of 2nd-level reports |
| `team_average_mood` | Mean mood across their team |
| `team_mood_trend` | Aggregate trend direction |
| `team_overdue_count` | Their reports overdue for 1-on-1s |
| `team_health_score` | Composite urgency metric (0-100) |

**Workspace-level:**
| Metric | Description |
|--------|-------------|
| `direct_report_count` | Count of direct reports (managers + ICs) |
| `total_report_count` | All reports including 2nd-level |
| `workspace_average_mood` | Mean mood across all reports |

---

## 2. UX Design — Guild Halls

### 2.1 Root Dashboard and Doorway Cards

The dashboard shows **direct reports only**. ICs keep their current cards;
managers render as wider 4-line **doorway cards**. Urgency sorting includes a
manager's worst squad outlier, so troubled teams float to the top.

```
═══════════════════ ⚔ VIBE MANAGER ⚔ ═══════════════════
  GUILD ROSTER · 5 direct · 14 in banner
═══════════════════════════════════════════════════════════

  ┌─────┐  JORDAN LEE — Eng Manager                    M2
  │══◆══│  ♥♥♥♥♡  ✓ you: 3d ago
  │ ^‿^ │  squad 4 ▕▓▓▓▓▓▓░░▏76%  ⚠ Sam T: mood ↘ · 6w
  └─────┘  ▸ Space to visit squad

  ╭─────╮  CHRIS WONG — Eng Manager                    M1
  │──◇──│  ♥♥♥♡♡  ⚠ you: 8d  zZ
  │ ◕‿◕ │  squad 5 ▕▓▓▓▓▓▓▓▓░▏90%  ★ all well · next: Ana P
  ╰─────╯  ▸ Space to visit squad

  ╔═════╗ Pat Kumar           ╭─────╮ Riley Fox
  ║ •_• ║ Senior Engineer     │ ◕‿◕ │ Junior Engineer
  ╚═════╝ ♥♥♥♥♡  ✓ 2d         ╰─────╯ ♥♥♥♥♥  ✓ 5d

 ─ j/k/h/l move · Enter detail · Space visit squad · n new ─
```

**Doorway card anatomy** — each of the four lines has a fixed job:

| Line | Job | Content |
|------|-----|---------|
| 1 | Who | Name, role, M-level badge |
| 2 | Your relationship | Their mood hearts + recency of *your* 1-on-1 (with `zZ` if overdue) |
| 3 | Their squad | Size, health bar, and the worst outlier *by name* — or `★ all well · next: <rotation>` when healthy |
| 4 | The door | Affordance hint — rendered **only on the selected card** (blank on unselected cards, keeping fixed height); becomes noise once learned, so it follows focus |

**Density rule:** with multiple outliers, name the worst and count the rest:
`⚠ Kim S: mood ↘ · 5w (+2 more)`. The card never grows beyond 4 lines.

### 2.2 Inside the Hall

`Space` on a doorway card enters the manager's hall: the **same dashboard
component re-rooted** at `manager/team/`. All keys behave identically; `n`
creates a meeting in the member's journal — that *is* the skip-level note.
Three things change: the breadcrumb title, a rotation strip, and the status bar.

```
═══════════════════ ⚔ VIBE MANAGER ⚔ ═══════════════════
  YOU ▸ JORDAN'S SQUAD · 4 members · ▓▓▓▓▓▓░░ 76%
═══════════════════════════════════════════════════════════
  rotation: Sam T (6w overdue!) → Morgan → Alex → Lee

  ┌─────┐ Sam Taylor          ╔═════╗ Morgan Day
  │◦︵◦↘│ Engineer        zZ  ║ •_• ║ Senior Engineer
  └─────┘ ♥♥♡♡♡  ⚠ skip: 6w  ╚═════╝ ♥♥♥♡♡  ✓ skip: 3w

  ╔═════╗ Alex Chen           ╭─────╮ Lee Kim
  ║ ◕‿◕ ║ Senior Engineer     │ ^‿^ │ Junior Engineer
  ╚═════╝ ♥♥♥♥♡  ✓ skip: 2w  ╰─────╯ ♥♥♥♥♥  ✓ skip: 1w

 ── Esc back to guild · Enter detail · n skip-level note ──
```

**Overdue semantics:** inside a hall, overdue is computed against
`default_2nd_level_frequency` (or the member's own `meeting_frequency`
override), *not* the cadence used for directs. Sam's `zZ` means "6 weeks
since a skip-level".

**Rotation strip:** members ordered by time since last skip-level, overdue
first. Density rule: show overdue people plus the next two, then a count —
`rotation: Kim S (5w!) → Jo P (5w!) → Ana P → Ben R · +8 queued`.

**Scale:** the hall inherits the dashboard's grid and scrolling. Urgency
sorting guarantees that what scrolls off-screen is, by construction, the
people who are fine.

### 2.3 Manager Detail: the Banner

`Enter` on a manager opens the existing report detail view plus one new
**Banner panel**: a mini face-strip of their squad, so 1-on-1 prep with the
manager includes their team's state at a glance.

```
╔══ JORDAN LEE · M2 · Eng Manager ═══════════════════════
║ ┌─────┐  mood ▁▂▄▅▆ ↗ rising    1-on-1s: weekly, ✓ 3d
║ │══◆══│ ─ BANNER ──────────────────────────────────────
║ │ ^‿^ │  ◕‿◕ Alex   ◦︵◦ Sam⚠   •_• Morgan   ^‿^ Lee
║ └─────┘  health 76% · 1 skip-level overdue (Sam, 6w)
╠═════════════════════════════════════════════════════════
║  MEETINGS                     │  2026-06-03  1-on-1
║ ▸ 2026-06-07  1-on-1          │  ...
╚══ n meeting · m mood · Space visit squad · Esc back ════
```

**The Banner is a bridge.** It is focusable: `Tab` moves focus between the
meetings list and the banner. With the banner focused, `←/→` selects a squad
member, `Enter` jumps straight to *that person's* detail ("mood dropped, go
look at Sam now"), and `Space` opens the whole hall. Even a user who never
learns the dashboard accelerator finds the team *through* its manager.

**Density rule:** at 8+ members, names drop and faces stay, sorted worst-first;
only outliers keep a label. Twelve faces with two frowns reads instantly:

```
║ │ •_• │  ◦︵◦ Kim⚠  ◦︵◦ Jo⚠  •_• •_• ◕‿◕ ◕‿◕ ◕‿◕ ^‿^ ^‿^ ^‿^ ^‿^ ^‿^
```

Focusing the banner reveals names one at a time as you `←/→` across.

### 2.4 Visual Distinction

**Adventurer Cards (ICs):** 3 lines tall (top frame, face, bottom frame).
Frame evolves: rounded → square → double-line.

```
P1: ╭─────╮  P2: ┌─────┐  P3: ╔═════╗  P4: ╔══★══╗  P5: ╔═★═★═╗
    │ •_• │      │ •_• │      ║ •_• ║      ║ •_• ║      ║ •_• ║
    ╰─────╯      └─────┘      ╚═════╝      ╚═════╝      ╚═════╝
```

**Lieutenant Cards (Managers):** 4 lines tall — a headband row between top
frame and face indicates rank. Headband gem evolves: `◇` → `◆` → `★` → `★★` → `★★★`.

```
M1: ╭─────╮  M2: ┌─────┐  M3: ╔═════╗  M4: ╔═════╗  M5: ╔═════╗
    │──◇──│      │══◆══│      ║══★══║      ║═★═★═║      ║★═★═★║
    │ •_• │      │ •_• │      ║ •_• ║      ║ •_• ║      ║ •_• ║
    ╰─────╯      └─────┘      ╚═════╝      ╚═════╝      ╚═════╝
```

**Face Expressions:**

| Mood | Expression | Note |
|------|------------|------|
| 5 | `^‿^` | Blissful |
| 4 | `◕‿◕` | Happy |
| 3 | `•_•` | Neutral |
| 2 | `◦︵◦` | Worried (shifted left for alignment) |
| 1 | `x_x` | Stressed |
| Overdue | `-_-` | Sleepy |

**Overdue Indicators:**
- Slightly overdue: ` z` / ` Z` on right
- Very overdue (>14d): ` zZ` / `ZzZ` on right

### 2.5 Navigation Model

| Key | Action |
|-----|--------|
| `Enter` | Open detail — same at every level; on a focused banner member, opens *their* detail |
| `Space` | Walk through a door (manager card → their hall); no-op on IC cards |
| `Tab` | In manager detail, move focus between meetings list and Banner |
| `Esc` | Walk back up one level (hall → parent roster); **hard no-op at root** — `q` is the only quit, so Esc-spamming out of a hall can never overshoot into a quit prompt |
| `h` at leftmost column | Also walks up one level (ranger/lf convention: `l` descends, `h` ascends) — home-row alternative to Esc for squad touring; documented in help, not self-discoverable |
| `j/k/h/l`, `g/G`, `n`, `m`, `?` | Identical at every level |

**Ergonomics notes:**
- The frequent path is thumb + home row: urgency sorting pre-selects the likely
  target, so drilling into the worst squad member is typically `Space, Enter`.
- `Space` is a **conscious rebind**: today's dashboard treats Space as a
  synonym for Enter (view detail). Accepted — the doorway hint retrains it.
- The `?` help modal gains a "Squads" section covering Space, Esc/boundary-`h`,
  and skip-level notes.

**State:** one new piece of state powers all of it — a **roster path stack**:
`[]` = root, `["jordan-lee"]` = inside Jordan's hall. The current roster is
resolved by walking the stack; breadcrumb and status bar derive from it. A
third org level someday is just a longer stack.

`Enter` is never overloaded: it always means "the person". `Space` always
means "the container". This keeps the app's strongest navigation asset —
Enter consistency — intact.

### 2.6 Density Rules Summary

| Surface | ≤ ~7 members | 8+ members |
|---------|--------------|------------|
| Doorway card line 3 | Worst outlier by name | Worst outlier `(+N more)` |
| Rotation strip | Full queue | Overdue + next 2 + `+N queued` |
| Banner | Face + name per member | Faces only, worst-first; outliers labeled |
| Hall grid | Full grid | Same grid, scrolls; urgency sort keeps trouble on screen |

---

## 3. Skip-Level Meetings

Skip-level meetings are regular meetings with 2nd-level reports. The
relationship is implicit from the file path:

- `jordan-lee/journal/2026-01-15T100000.md` → direct 1-on-1 with Jordan
- `jordan-lee/team/alex-chen/journal/2026-01-20.md` → skip-level with Alex

### Meeting Frequency Defaults

```yaml
# workspace/.vibe-manager
settings:
  default_meeting_frequency: biweekly      # For direct reports
  default_2nd_level_frequency: monthly     # For 2nd-level reports
```

Per-report overrides use the same `meeting_frequency` field.

### Rotation

The rotation is computed, not stored: 2nd-level reports ordered by time since
their last journal entry, overdue-first. It drives the hall's rotation strip,
the doorway card's `next: <name>` hint, and overdue (`zZ`) markers inside halls.

---

## 4. Implementation Phases

| Phase | Focus | Status |
|-------|-------|--------|
| Phase 1 | Data model: ReportType, M-track, TeamMetrics | ✅ Done |
| Phase 2 | Storage: Nested team/ directories | ✅ Done |
| Phase 3 | Doorway cards: render TeamMetrics + worst outlier on dashboard | ✅ Done |
| Phase 4 | Halls: roster path stack, Space/Esc navigation, breadcrumb, skip-level overdue | ✅ Done (rotation strip pending) |
| Phase 5 | Banner panel in manager detail + Tab focus bridge | 📋 Planned |
| Phase 6 | Quest Log: cross-tree attention queue (separate spec) | 🔮 Future |

---

## 5. Critical Files

| File | Purpose |
|------|---------|
| `src/model/report.rs` | ReportType, ManagerInfo, M-track levels |
| `src/model/computed.rs` | TeamMetrics, worst-outlier, rotation ordering |
| `src/storage/repo/workspace.rs` | Nested directory loading |
| `src/storage/repo/report.rs` | Profile loading incl. team members |
| `src/theme/sprites.rs` | Manager sprites with headband |
| `src/components/avatar.rs` | Card rendering (doorway card variant) |
| `src/components/dashboard.rs` | Roster grid, breadcrumb, rotation strip |
| `src/components/report_detail.rs` | Banner panel |
| `src/app/state.rs` | Roster path stack, team loading |
| `src/app/input.rs` | Space/Tab/Esc bindings |

---

## 6. Design Decisions

1. **Hierarchy via path stack**: 2 levels today (you → managers → their
   reports), but the roster path stack makes deeper nesting free.
2. **Drill-down over inline expansion**: an expanding dashboard (accordion)
   was considered and rejected — grid-within-list navigation is fiddly and it
   degrades worst as teams grow. One hall per screen scales to any size.
3. **`Enter` = person, `Space` = container**: Enter opens detail identically
   at every level; Space is the only new verb. The Banner panel doubles as a
   discoverable bridge for users who never learn the Space accelerator.
4. **Outliers over averages**: doorway cards and banners always name the worst
   member; aggregate scores are secondary context, never the headline.
5. **Rotation is computed**: skip-level ordering derives from journal
   timestamps vs `default_2nd_level_frequency`; nothing new is stored.
6. **Density degrades by summarizing**: counts replace lists, faces replace
   name-tags; no surface ever grows extra lines or panels at scale.
7. **Ergonomics over novelty**: Esc is a hard no-op at root (prevents
   Esc-spam overshoot into quit); boundary-`h` ascends like ranger/lf for
   home-row back-walking; the door hint follows selection instead of being
   static card furniture. The Space rebind (was: synonym for Enter) is a
   deliberate, accepted break.
8. **Manager detection**: explicit `report_type: manager` field OR existence
   of `team/` directory.
9. **Terminology**: generic "Report" for both ICs and managers.
10. **Frame styles**: headband row distinguishes managers; frame progression
    shared with ICs.
