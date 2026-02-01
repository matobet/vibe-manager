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
| Dashboard: Manager cards with team health | ✅ Done | Team health bar + overdue count |
| Dashboard: Expand/collapse manager teams | ✅ Done | Space key toggles expansion |
| Manager detail view | ✅ Done | Team health panel + roster |
| Skip-level support | ✅ Done | Navigate into team members, create skip-level meetings |

---

## Overview

Extend Vibe Manager to support **managers as direct reports**, enabling:
1. **Direct manager reports** - Track 1-on-1s with your manager reports
2. **Second-level reports** - Visibility into people who report to your managers
3. **Skip-level meetings** - Optional periodic check-ins with indirect reports

### Terminology

| Term | Description |
|------|-------------|
| Report | Anyone you manage (IC or manager) |
| Manager report | A direct report who manages others |
| 2nd-level report | Someone who reports to your manager report |
| Skip-level | A meeting with a 2nd-level report |

### RPG Theme
> Your party now includes **lieutenants** who lead their own sub-parties.

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

## 2. UX Design

### 2.1 Dashboard Layout

Managers expand inline to reveal their squads:

```
══════════════════ GUILD ROSTER ═══════════════════

▼ ┌─────┐ JORDAN LEE ─────────────────────────── M2
  │══◆══│ Engineering Manager    ♥♥♥♥♡    ✓ 3d ago
  │ ^‿^ │ Team: ▓▓▓▓▓▓▓▓░░ 82%   ⚠ 1 needs attention
  └─────┘
  ├──┬─────────┬─────────┬─────────┬─────────┐
  │  │  ◕‿◕    │  ◦︵◦ ↘ │  •_•    │  ^‿^    │
  │  │ Alex C  │ Sam T   │ Morgan  │ Lee K   │
  │  │ P3 ♥♥♥♥♡│ P2 ♥♥♡♡♡│ P3 ♥♥♥♡♡│ P1 ♥♥♥♥♥│
  │  │ ✓ 2w    │ ⚠ 6w!   │ ✓ 3w    │ ✓ 1w    │
  │  └─────────┴─────────┴─────────┴─────────┘

▶ ╭─────╮ CHRIS WONG ─────────────────────────── M1
  │──◇──│ Engineering Manager    ♥♥♥♡♡    ⚠ 8d ago
  │ ◕‿◕ │ Team: ▓▓▓▓▓▓▓▓▓░ 90%   ★ healthy
  ╰─────╯

── ══════════════ DIRECT ADVENTURERS ═══════════ ──

  ╭─ ★ P3 ★ ─╮ Pat Kumar
  │   •_•     │ Senior Engineer
  │ ♥♥♥♥♡     │ ✓ 2d ago
  ╰───────────╯
```

**Key Elements:**
- `PARTY: 4/12` header - Direct reports / total reports
- `▼/▶` - Expand/collapse indicator
- `▓▓▓▓▓▓▓▓░░ 82%` - Team health bar
- `⚠ 1 needs attention` / `★ healthy` - Quick alert

### 2.2 Visual Distinction

**Adventurer Cards (ICs):**
- 3 lines tall (top frame, face, bottom frame)
- Frame evolves: rounded → square → double-line
- P1: `╭─────╮` (rounded, newcomer)
- P2: `┌─────┐` (square, solid)
- P3: `╔═════╗` (double-line, senior)
- P4: `╔══★══╗` (single filled star, staff)
- P5: `╔═★═★═╗` (double filled stars, distinguished)

**IC Track Avatar Reference (3 lines):**
```
P1: ╭─────╮  P2: ┌─────┐  P3: ╔═════╗  P4: ╔══★══╗  P5: ╔═★═★═╗
    │ •_• │      │ •_• │      ║ •_• ║      ║ •_• ║      ║ •_• ║
    ╰─────╯      └─────┘      ╚═════╝      ╚═════╝      ╚═════╝
```

**Lieutenant Cards (Managers):**
- 4 lines tall (extra headband row between top frame and face)
- Same frame progression as ICs for levels 1-3
- Headband between top border and face indicates rank
- Headband gem evolves: `◇` → `◆` → `★` → `★★` → `★★★`
- M1: Rounded frame + hollow diamond headband `│──◇──│` (team lead)
- M2: Square frame + filled diamond headband `│══◆══│` (EM)
- M3: Double-line frame + single star headband `║══★══║` (senior manager)
- M4: Double-line frame + double star headband `║═★═★═║` (director)
- M5: Double-line frame + triple star headband `║★═★═★║` (VP)
- Team metrics section

**Manager Track Avatar Reference (4 lines):**
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

### 2.3 Navigation

| Key | Action |
|-----|--------|
| `j/k` | Navigate up/down |
| `Space` | Expand/collapse manager |
| `Enter` | View details |
| `l` or `→` | Enter expanded squad |
| `h` or `←` | Back to parent |
| `n` | New 1-on-1 meeting |
| `Esc` | Back to dashboard |

---

## 3. Skip-Level Meetings

Skip-level meetings are regular meetings with 2nd-level reports. The relationship is implicit from the file path:

- `jordan-lee/2026-01-15.md` → direct 1-on-1 with Jordan
- `jordan-lee/team/alex-chen/2026-01-20.md` → skip-level with Alex

### Meeting Frequency Defaults

```yaml
# workspace/.vibe-manager
settings:
  default_meeting_frequency: biweekly      # For direct reports
  default_2nd_level_frequency: monthly     # For 2nd-level reports
```

Per-report overrides use the same `meeting_frequency` field.

---

## 4. Implementation Phases

| Phase | Focus | Status |
|-------|-------|--------|
| Phase 1 | Data model: ReportType, M-track, TeamMetrics | ✅ Done |
| Phase 2 | Storage: Nested team/ directories | ✅ Done |
| Phase 3 | Dashboard: Manager cards, expand/collapse | 📋 Planned |
| Phase 4 | Manager detail view with team roster | 📋 Planned |
| Phase 5 | Skip-level tracking and alerts | 📋 Planned |

---

## 5. Critical Files

| File | Purpose |
|------|---------|
| `src/model/report.rs` | ReportType, ManagerInfo, M-track levels |
| `src/model/computed.rs` | TeamMetrics computation |
| `src/storage/workspace.rs` | Nested directory loading |
| `src/storage/profile.rs` | load_report_with_manager() |
| `src/components/avatar.rs` | Manager card rendering |
| `src/app.rs` | Team loading, ViewMode updates |

---

## 6. Design Decisions

1. **Hierarchy depth**: 2 levels max (you → managers → their reports)
2. **Manager detection**: Explicit `report_type: manager` field OR existence of `team/` directory
3. **Skip-level frequency**: Workspace default + per-report override
4. **Terminology**: Generic "Report" for both ICs and managers
5. **Frame styles**: Double-line for managers, rounded for ICs
