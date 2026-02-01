# Team Overview Dashboard - Feature Specification

## Implementation Status

| Requirement | Status | Notes |
|-------------|--------|-------|
| FR-401: View All Active Engineers | ✅ Implemented | Grid layout with horizontal scrolling |
| FR-402: Visual Status Indicators | ✅ Implemented | Traffic light colors, kaomoji expressions |
| FR-403: Sorting by Urgency Score | ✅ Implemented | Exact algorithm as documented |
| FR-404: Filtering Options | 📋 Planned | Not yet implemented |
| FR-405: Quick Actions from Dashboard | 📋 Planned | Navigation only; quick mood/note from dashboard planned |
| FR-406: Weekly Summary View | 📋 Planned | Not yet implemented |
| Help Modal | ✅ Implemented | Press `?` for keybinding reference |
| Sleep Indicators | ✅ Implemented | Overdue engineers show `zzz` sleep kaomoji |

---

## 1. Feature Overview

The Team Overview Dashboard is the primary landing page for Vibe Manager, providing an **at-a-glance status view** of all direct reports. Designed for engineering managers with 2-10 engineers, the dashboard enables quick identification of who needs immediate attention without requiring navigation to individual profiles.

### Key Principle
> See everything that matters in 5 seconds or less.

The dashboard prioritizes **actionability over completeness** - surfacing urgent items prominently while keeping secondary information accessible but not distracting.

### Design Aesthetic: RPG Party Management

The dashboard draws inspiration from **8-bit RPG party screens** - think Final Fantasy's party menu or Dragon Quest's character status. Your team of engineers is your party of adventurers:

| Dashboard Element | RPG Inspiration |
|-------------------|-----------------|
| Team overview | Party management screen |
| Engineer cards | Character stat cards with color-coded borders |
| Mood score (1-5) | HP bar / Morale meter |
| Career level (P1-P5) | Character level badge |
| 1-on-1 status | Status effects (healthy, poisoned, low HP) |
| Skills/proficiencies | Character stats (STR, INT, WIS...) |

**Visual Language:**
- Pixel art icons and status indicators
- HP-style bars for mood and time-since-1on1
- Level badges styled like RPG level indicators
- Status effects for attention states (overdue = warning glow)
- Limited color palette reminiscent of NES/SNES era

---

## 2. User Needs

| Need | Description |
|------|-------------|
| **Quick morning scan** | "Who do I need to meet with today/this week?" |
| **Identify concerns early** | "Is anyone struggling or showing signs of burnout?" |
| **Never miss a 1-on-1** | "Who am I overdue with?" |
| **Prepare before meetings** | "Quick context before I walk into a 1-on-1" |
| **Track team health** | "How is my team doing overall?" |
| **Act without friction** | "Log a quick mood observation without leaving the dashboard" |

### User Scenarios

**Scenario 1: Monday Morning Planning**
> Manager opens Vibe Manager to plan the week. Dashboard immediately shows 2 overdue 1-on-1s (red), 3 due this week (yellow), and highlights that Alex's mood has declined over the past 2 weeks.

**Scenario 2: Quick Mood Log**
> After a team standup, manager notices Jordan seems stressed. Without leaving the dashboard, they record a mood score of 2 with context "daily_standup" using inline quick action.

**Scenario 3: End of Week Review**
> Manager checks the weekly summary to see: 4 1-on-1s completed, mood trends stable except for one flag, no major changes to track.

---

## 3. Dashboard Priorities

### Primary Focus (At-a-Glance)

These elements are **immediately visible** without any interaction:

#### 1-on-1 Status
| Status | Visual | Meaning |
|--------|--------|---------|
| Overdue | Red indicator | Past meeting frequency threshold |
| Due Soon | Yellow indicator | Within 3 days of threshold |
| On Track | Green indicator | Not due yet |
| Scheduled | Blue indicator | Future meeting set |

**Display Information:**
- Days overdue or days until due
- Last meeting date
- Next suggested/scheduled date

#### Health/Mood Trends
| Status | Visual | Meaning |
|--------|--------|---------|
| Declining | Orange diagonal arrow (↘) | Mood dropped significantly |
| Needs Attention | Yellow alert | Low current mood (1-2) or volatility |
| Stable | No indicator | Within normal range (hidden for cleaner display) |
| Improving | Green diagonal arrow (↗) | Consistent improvement |

**Display Information:**
- Current mood score (most recent)
- Trend direction (last 30 days)
- Days since last observation

### Secondary Focus (Available but Not Primary)

These are visible but given less visual prominence:

| Information | Display |
|-------------|---------|
| Career Level | Badge showing P1-P5 |
| Skill Progress | Compact progress indicator |
| Time Since Assessment | "3 months ago" subtle text |
| Seniority Category | Junior/Mid/Senior label |
| Tenure | Time on team |

---

## 4. Functional Requirements

### 4.1 View All Active Engineers

**FR-401**: Display all engineers where `isActive = true` in a unified view.

- Support 2-10 engineers without scrolling on standard laptop screen (1366x768 minimum)
- Compact mode available for teams >6
- Archived engineers not shown (accessible via filter toggle)

### 4.2 Visual Status Indicators

**FR-402**: Use consistent traffic light color system across all status types.

```
Color Coding:
  Red     = Requires immediate attention (overdue, declining mood)
  Yellow  = Approaching attention needed (due soon, low mood)
  Green   = On track (healthy status)
  Blue    = Informational (scheduled, neutral)
  Gray    = No data / Not applicable
```

**Status Indicator Wireframe:**
```
 ┌──────────────────────────────────────────────────────────┐
 │  ★ P3  Alex Chen                                    Sr   │
 │        Software Engineer                                 │
 │  ─────────────────────────────────────────────────────── │
 │  1-on-1: [RED] 5 days overdue     Mood: [YELLOW] 3 (↓)  │
 │          Last: Jan 5              Trend: Declining       │
 └──────────────────────────────────────────────────────────┘
```

### 4.3 Sorting by Urgency Score

**FR-403**: Engineers are sorted by a computed **urgency score** (highest first = needs most attention).

The urgency score combines multiple factors to surface engineers who need immediate attention:

| Factor | Score | Rationale |
|--------|-------|-----------|
| Never had a meeting | +100 | New team members need onboarding priority |
| Days overdue (past frequency + threshold) | +10 per day (max 80) | Longer overdue = more urgent |
| Approaching due date (within 2 days) | +5 | Proactive reminder |
| Low mood (1-2) | +20 | Struggling team members need support |
| Falling mood trend | +15 | Declining morale is a warning sign |
| No mood data | +10 | Unknown state needs check-in |

**Example Urgency Scores:**
| Scenario | Score | Breakdown |
|----------|-------|-----------|
| New hire, never met | 110 | 100 (never met) + 10 (no mood) |
| 3 days overdue + falling mood | 45 | 30 (3×10) + 15 (falling) |
| Low mood (2) + falling | 35 | 20 (low) + 15 (falling) |
| Recently met, good mood | 0 | All healthy |
| Approaching due, stable | 5 | Just the reminder |

**Dashboard Order:**
Engineers appear left-to-right, top-to-bottom by descending urgency score. The first card is always the person who most needs your attention right now.

```
 ┌─────────────────────────────────────────────────────────────────────┐
 │  ★ P2  New Hire       │  ★ P3  Alex (overdue) │  ★ P4  Jordan     │
 │  Score: 110            │  Score: 45             │  Score: 5         │
 │  Never met!            │  3 days over, mood ↓   │  Due in 2 days    │
 └─────────────────────────────────────────────────────────────────────┘
```

### 4.4 Filtering Options

**FR-404**: Support filters to focus the view.

| Filter | Options |
|--------|---------|
| 1-on-1 Status | All / Overdue Only / Due This Week |
| Seniority | All / Junior / Mid / Senior / Staff |
| Mood Status | All / Needs Attention |

**Wireframe - Filter Controls:**
```
 Filters: [ All Engineers ▼ ] [ All Seniorities ▼ ] [ All Moods ▼ ]

          Active filters: [Overdue Only ×] [Senior ×]  ← chips showing active filters
```

### 4.5 Quick Actions

**FR-405**: Enable common actions without navigating away from dashboard.

| Action | Trigger | Behavior |
|--------|---------|----------|
| Mark 1-on-1 Complete | Click checkmark button | Opens minimal modal: date + optional note prompt |
| Record Mood | Click mood icon or number buttons | Inline 1-5 selector appears, saves immediately |
| View Details | Click engineer card/row | Navigates to full engineer profile |
| Quick Note | Click note icon | Opens quick note modal, associates with today |

**Wireframe - Quick Action Buttons:**
```
 ┌─────────────────────────────────────────────────────────────────┐
 │  Jordan Lee                                                     │
 │  1-on-1: [RED] 3 days overdue                                   │
 │                                                                 │
 │  Quick Actions:  [✓ Complete]  [😊 1-5]  [📝 Note]  [→ View]   │
 └─────────────────────────────────────────────────────────────────┘
```

**Inline Mood Entry Wireframe:**
```
 Record mood:  [ 1 ] [ 2 ] [ 3 ] [ 4 ] [ 5 ]    [Cancel]
                           ↑
                        Selected
```

### 4.6 Weekly Summary View

**FR-406**: Provide a summary of the past week's activity.

**Summary Includes:**
- Number of 1-on-1s completed this week
- 1-on-1s still overdue
- Mood changes flagged (significant declines)
- Upcoming important dates (birthdays, anniversaries)

**Wireframe - Weekly Summary Panel:**
```
 ┌──────────────────────────────────────────────────────────────┐
 │  WEEKLY SUMMARY (Jan 13 - Jan 17)                            │
 │  ────────────────────────────────────────────────────────────│
 │  1-on-1s:  4 completed  │  2 overdue  │  3 scheduled         │
 │                                                              │
 │  Mood Alerts:                                                │
 │    ⚠ Alex Chen - Mood declined from 4 to 2 this week        │
 │                                                              │
 │  Upcoming:                                                   │
 │    🎂 Sam Taylor - Birthday Jan 20                          │
 │    📅 Jordan Lee - 1 year anniversary Jan 22                │
 └──────────────────────────────────────────────────────────────┘
```

---

## 5. UI Components

### 5.1 Report Card/Row

The primary display unit for each report (IC or manager), styled as an RPG character card with a distinctive color border (derived from the report's name) and a kaomoji avatar.

**TUI Implementation - Kaomoji Avatar Cards:**
```
 ╭─ ★ P3 ★ ─╮        ╔═ ★ P4 ★ ═╗
 │  ╔═════╗  │        ║  ╔═ ☆ ═╗  ║
 │  ║ ◕‿◕ ║  │        ║  ║ ^‿^ ║  ║
 │  ╚═════╝  │        ║  ╚═════╝  ║
 │ Alex Chen │        ║Jordan Lee ║
 │  ♥♥♥♡♡    │        ║ ↗ ♥♥♥♥♡  ║
 │ ✓ 3d ago  │        ║ ✓ today   ║
 ╰───────────╯        ╚═══════════╝
```

#### Avatar Reference

**IC Track (P1-P5) - 3 lines:**
```
P1: ╭─────╮  P2: ┌─────┐  P3: ╔═════╗  P4: ╔══★══╗  P5: ╔═★═★═╗
    │ •_• │      │ •_• │      ║ •_• ║      ║ •_• ║      ║ •_• ║
    ╰─────╯      └─────┘      ╚═════╝      ╚═════╝      ╚═════╝
```

**Manager Track (M1-M5) - 4 lines:**
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

**Avatar Frame Styles by Level (IC Track):**
- P1: Rounded corners (`╭─────╮`) - newcomer
- P2: Square corners (`┌─────┐`) - solid contributor
- P3: Double-line box (`╔═════╗`) - senior
- P4: Double-line with single filled star (`╔══★══╗`) - staff
- P5: Double-line with double filled stars (`╔═★═★═╗`) - distinguished

**Kaomoji Expressions by Mood:**
- Mood 5: `^‿^` (blissful)
- Mood 4: `◕‿◕` (happy)
- Mood 3: `•_•` (neutral)
- Mood 2: `◦︵◦` (worried - shifted left for alignment due to fullwidth character)
- Mood 1: `x_x` (stressed)
- Overdue (no meeting in 2+ frequencies): `-_- zzz` (sleeping/neglected)

**Mood Trend Indicators:**
- `↗` Green = improving
- `↘` Orange = declining (hidden when stable for cleaner display)

**Card Layout - RPG Character Card Style (Alternative):**
```
 ╔═══════════════════════════════════════════════════════════════════╗
 ║  ┌───────────────────────────────────────────────────────────┐   ║
 ║  │  ★ LV.3   ALEX CHEN                           Senior  ★  │   ║
 ║  │           Software Engineer                               │   ║
 ║  │           Party member for 2y 3m                          │   ║
 ║  └───────────────────────────────────────────────────────────┘   ║
 ║  ─────────────────────────────────────────────────────────────── ║
 ║  MORALE  [████████░░] 4/5  ↑ improving                           ║
 ║  1-ON-1  [██░░░░░░░░] ⚠ 5 days overdue                          ║
 ║  ─────────────────────────────────────────────────────────────── ║
 ║  > Check In   > Record Morale   > View Stats                     ║
 ╚═══════════════════════════════════════════════════════════════════╝
```

**Compact Row Layout - Party List Style:**
```
 ╔════════════════════════════════════════════════════════════════════════╗
 ║  ⚠ Alex Chen      LV.3 Sr  │ ████░░ 4/5 │ 1-on-1: 5d over │ [>][♥][?] ║
 ║  ● Jordan Lee     LV.2 Mid │ ██████ 5/5 │ 1-on-1: TODAY   │ [>][♥][?] ║
 ║  ● Sam Taylor     LV.1 Jr  │ ████░░ 3/5 │ 1-on-1: 3 days  │ [>][♥][?] ║
 ╚════════════════════════════════════════════════════════════════════════╝

 Legend: [>] Check In  [♥] Record Morale  [?] View Stats
```

**Status Effect Indicators:**
```
 ● Healthy     - All good, no attention needed
 ⚠ Caution    - Due soon or mood declining
 ⛔ Critical   - Overdue or low morale
 ★ Leveling   - Recently promoted or skill growth
```

### 5.2 Status Indicators

**1-on-1 Status Badge:**
```
 [●] Overdue (5d)     ← Red circle with days count
 [●] Due Soon (2d)    ← Yellow circle with days until due
 [●] On Track         ← Green circle
 [●] Scheduled Jan 20 ← Blue circle with date
```

**Mood Status Badge:**
```
 ↗ ♥♥♥♥♡             ← Trend arrow (colored) + heart gauge
   ↗ Green = improving
   ↘ Orange = declining
   (no indicator when stable)
```

### 5.3 Quick Action Buttons

**Button States:**
```
 [ ✓ Complete ]      ← Primary action, prominent
 [ 1 2 3 4 5 ]       ← Mood entry (inline or expandable)
 [ 📝 ]              ← Quick note icon button
 [ → ]               ← Navigate to profile
```

### 5.4 Sort/Filter Controls

**Control Bar Wireframe:**
```
 ┌────────────────────────────────────────────────────────────────────────┐
 │  MY TEAM (6 engineers)                                                │
 │                                                                       │
 │  Sort: [1-on-1 Urgency ▼]    Filter: [Status ▼] [Seniority ▼]        │
 │                                                                       │
 │  Active: [Overdue Only ×]    View: [Cards] [Compact]                 │
 └────────────────────────────────────────────────────────────────────────┘
```

### 5.5 Summary Statistics Bar

**Party Status Bar - RPG Style:**
```
 ╔════════════════════════════════════════════════════════════════════════╗
 ║  ⚔ YOUR PARTY                                          6 adventurers  ║
 ╠════════════════════════════════════════════════════════════════════════╣
 ║  PARTY MORALE [██████████████░░░░░░] 72%    ⚠ 1 needs attention       ║
 ║  ─────────────────────────────────────────────────────────────────────║
 ║  ⛔ 2 overdue   ⚠ 3 due soon   ● 1 on track   🎂 Birthday in 3 days  ║
 ╚════════════════════════════════════════════════════════════════════════╝
```

**Alternative Minimal Stats:**
```
 ╔═══════════════════════════════════════════════════════════╗
 ║  PARTY: 6 members │ MORALE: 72% │ ⚠ 2 need check-in     ║
 ╚═══════════════════════════════════════════════════════════╝
```

### 5.6 Keyboard Navigation

**Dashboard Keybindings:**
| Key | Action |
|-----|--------|
| `h/←` | Move selection left |
| `l/→` | Move selection right |
| `j/↓` | Move selection down |
| `k/↑` | Move selection up |
| `Enter` | Open selected engineer detail |
| `n` | Create new engineer |
| `?` | Show help modal |
| `q` | Quit application |

**Help Modal:**
Press `?` from the dashboard to display a help modal showing all available keybindings. The modal includes:
- Navigation keys
- Action keys
- View-specific shortcuts
- General application controls

---

## 6. Responsive Design

### Design Principles for Quick Checks

The dashboard must support **quick mobile checks** (viewing status) even if full interaction is optimized for desktop.

### Breakpoints

| Screen Size | Layout Adaptation |
|-------------|-------------------|
| Desktop (>1200px) | Full card layout, 2-3 columns, all controls visible |
| Laptop (992-1200px) | Card layout, 2 columns, controls in header |
| Tablet (768-991px) | Compact rows, single column, collapsible filters |
| Mobile (<768px) | Minimal cards, critical info only, swipe actions |

### Mobile View Wireframe
```
 ┌─────────────────────────────────┐
 │ VIBE MANAGER        [≡] [+]    │
 │─────────────────────────────────│
 │ Team Summary: 2 overdue, 1 ⚠   │
 │─────────────────────────────────│
 │ ┌─────────────────────────────┐│
 │ │ [●] Alex Chen          [→] ││
 │ │     5d overdue │ Mood: 3↓  ││
 │ │     [Swipe for actions]    ││
 │ └─────────────────────────────┘│
 │ ┌─────────────────────────────┐│
 │ │ [●] Jordan Lee         [→] ││
 │ │     Due today │ Mood: 4→   ││
 │ └─────────────────────────────┘│
 │ ...                            │
 └─────────────────────────────────┘
```

### Mobile Quick Actions
- Swipe right: Mark 1-on-1 complete
- Swipe left: Quick mood entry
- Tap: View profile
- Long press: Context menu

---

## 7. Data Aggregation

The dashboard aggregates data from multiple entities to compute display values.

### Data Sources

```
Dashboard View
     │
     ├── Engineer entity
     │   ├── name, title, photoUrl
     │   ├── startDate → tenure calculation
     │   ├── currentLevel, seniorityCategory
     │   └── oneOnOneMeeting Frequency
     │
     ├── Meeting entity (filtered by engineerId)
     │   ├── Most recent completed → lastOneOnOneDate
     │   ├── Next scheduled → upcomingMeeting
     │   └── Computed: daysOverdue, daysUntilDue
     │
     ├── MoodEntry entity (filtered by engineerId)
     │   ├── Most recent → currentMood
     │   ├── Last 30 days → averageMood, trend
     │   └── Computed: moodTrend (improving/stable/declining)
     │
     └── CareerProgress entity (secondary)
         ├── Aggregated by pillar
         └── lastAssessedAt → time since assessment
```

### Computed Fields for Dashboard

| Field | Calculation |
|-------|-------------|
| `daysOverdue` | `today - (lastOneOnOneDate + meeting frequencyDays)` if positive |
| `daysUntilDue` | `(lastOneOnOneDate + meeting frequencyDays) - today` if positive |
| `oneOnOneStatus` | Derived from daysOverdue/daysUntilDue |
| `currentMood` | Most recent MoodEntry.score |
| `moodTrend` | Linear regression on last 5 entries or 30 days |
| `tenureMonths` | `(today - startDate) / 30` |
| `needsAttention` | `isOverdue OR moodTrend == 'declining' OR currentMood <= 2` |

### Refresh Strategy

| Trigger | Action |
|---------|--------|
| Page load | Full data fetch for all engineers |
| Quick action complete | Optimistic UI update + background sync |
| Return to tab | Refresh if >5 minutes since last fetch |
| Manual refresh | Pull-to-refresh on mobile, refresh button on desktop |

---

## Related Documents

- [Product Vision](/home/matobet/projects/vibe-manager/docs/product-vision.md)
- [Data Model](/home/matobet/projects/vibe-manager/docs/data-model.md)
- [User Stories](/home/matobet/projects/vibe-manager/docs/user-stories.md) - See US-601 through US-604

## Related User Stories

| Story ID | Title | Priority |
|----------|-------|----------|
| US-601 | Team Overview at a Glance | P0 |
| US-602 | Sort and Filter Team | P0 |
| US-603 | Quick Actions from Dashboard | P0 |
| US-604 | Weekly Summary View | P1 |
