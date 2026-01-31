# Health/Mood Tracking Feature

## Implementation Status

| Requirement | Status | Notes |
|-------------|--------|-------|
| FR-1: Record Mood Observation | ✅ Implemented | F1-F5 keys in note viewer, or `m` key for quick entry |
| FR-2: Context Selection | ✅ Implemented | Meeting/Standup/Slack/Other via Tab key |
| FR-3: Optional Notes | ✅ Implemented | Notes field in mood observation modal |
| FR-4: Mood History View | ✅ Implemented | ASCII chart in engineer detail view |
| FR-5: Trend Visualization | ✅ Implemented | Rising/Stable/Falling indicators |
| FR-6: Dashboard Alerts | ✅ Implemented | Urgency score includes mood factors |
| Standalone Mood Entry | ✅ Implemented | `m` key opens modal, not tied to meetings |

---

## Feature Overview

Health/Mood Tracking provides a simple way to record observations about team members' wellbeing over time. The goal is to spot early warning signs before they become serious problems.

This is **observation tracking, not a clinical tool**. Engineering managers are not therapists. This feature helps you notice patterns and have better conversations with your team.

Key principles:
- Quick to record (seconds, not minutes)
- Simple scale (no complex questionnaires)
- Observations, not diagnoses
- Spot trends, not single data points

### Design Aesthetic: RPG Morale/HP System

In RPG terms, mood tracking is your **party morale system**:

| Health Concept | RPG Equivalent |
|----------------|----------------|
| Mood score (1-5) | HP / Morale points |
| Declining trend | Taking damage / Morale drain |
| Improving trend | Healing / Morale boost |
| Score of 1-2 | Low HP warning |
| Score of 5 | Full HP / High morale |
| Weekly check-in | Rest at the inn |

**Visual Treatment:**
- Mood displayed as HP-style bar: `MORALE [████████░░] 4/5`
- Trend shown with status effect icons (poison = declining, regen = improving)
- Dashboard alerts styled as "party member needs rest"
- Color progression: red (1) → orange (2) → yellow (3) → green (4) → bright green (5)

---

## User Needs

### Early Burnout Detection

Burnout doesn't happen overnight. It builds gradually through:
- Increasing exhaustion
- Growing cynicism about work
- Declining sense of accomplishment

By tracking mood over time, managers can notice downward trends before they become crises.

### Happiness Trend Tracking

Understanding team health requires looking at patterns:
- Is someone consistently declining over weeks?
- Did a particular project or change affect morale?
- Are there seasonal patterns?
- How quickly do people recover after tough periods?

### Proactive Support

With trend data, managers can:
- Intervene early with support
- Adjust workload before burnout hits
- Celebrate sustained positive periods
- Have data-informed conversations

---

## Mood Scale Design

### The 1-5 Scale

A simple 5-point scale balances ease of use with meaningful differentiation:

| Score | Label | Description | When to Record |
|-------|-------|-------------|----------------|
| **1** | Very Low | Concerning, needs attention | Visible distress, withdrawal, or explicit struggle |
| **2** | Below Normal | Not themselves | Quieter than usual, less engaged, minor frustrations showing |
| **3** | Neutral/OK | Baseline | Normal day, nothing notable either way |
| **4** | Good | Positive signs | Engaged, contributing well, seems content |
| **5** | Excellent | Thriving | Energized, enthusiastic, going above and beyond |

### Why 5 Points?

- **3 is too few**: Can't distinguish "struggling" from "crisis"
- **10 is too many**: False precision, harder to be consistent
- **5 is just right**: Easy to calibrate, quick to select, meaningful differences

### Recording Guidelines

- Record what you observe, not what you assume
- "3" is not a cop-out; most days genuinely are neutral
- Use 1 and 5 sparingly; they should stand out in the data
- When uncertain, lean toward the middle

---

## Functional Requirements

### FR-1: Record Mood Observation

**Description**: Record a mood observation for any engineer at any time.

**Not tied to events**: Observations can be recorded anytime, not just during meetings. You might notice something in Slack, a PR review, or a hallway conversation.

**Input fields**:
- Engineer (required)
- Mood score 1-5 (required)
- Context (optional)
- Notes (optional)

### FR-2: Context Selection

**Description**: Optionally specify what prompted the observation.

**Context options**:
- After 1-on-1
- In standup
- Casual chat
- Weekly reflection
- Team meeting
- Other

Context helps when reviewing history: "Was this their actual mood, or were they just tired in that 8am standup?"

### FR-3: Optional Notes

**Description**: Add free-text notes to provide context for the observation.

**Use cases**:
- "Mentioned feeling overwhelmed by deadline pressure"
- "Excited about new project assignment"
- "Seemed frustrated in code review discussion"
- "Just got back from vacation, refreshed"

Notes are optional. Quick entries without notes are valuable too.

### FR-4: Mood History View

**Description**: View mood history for a specific engineer.

**Display elements**:
- Timeline/chart of scores over time
- List of recent entries with notes
- Filter by date range
- Filter by context

### FR-5: Trend Visualization

**Description**: Show whether mood is improving, stable, or declining.

**Trend calculation**:
- Compare recent average (last 30 days) to prior period
- Indicate direction: improving, stable, declining
- Show visually on engineer profile

**Trend indicators**:
- Improving: 0.5+ point increase in rolling average
- Declining: 0.5+ point decrease in rolling average
- Stable: Within 0.5 points

### FR-6: Dashboard Alerts

**Description**: Surface engineers who may need attention on the main dashboard.

**Alert triggers**:
- Current trend is "declining"
- Most recent score is 1 or 2
- No mood recorded in 30+ days (for active tracking)

**Alert display**:
- Highlight on dashboard
- Quick link to engineer profile
- Show recent trend

---

## Research Basis

### Simple Scales Work

Research consistently shows that single-item measures correlate highly (r > 0.7) with complex multi-item assessments. A simple "How are you doing?" often captures what lengthy questionnaires measure.

This doesn't mean simple scales are perfect, but for the purpose of **noticing change over time**, they're effective and sustainable.

### Maslach Burnout Dimensions

The Maslach Burnout Inventory identifies three dimensions:

1. **Emotional Exhaustion**: Feeling drained, depleted, worn out
2. **Cynicism/Depersonalization**: Negative, detached attitude toward work
3. **Reduced Personal Efficacy**: Feeling ineffective, lack of accomplishment

While our simple scale doesn't measure these separately, declining mood scores often reflect issues in one or more dimensions. The notes field allows capturing specifics.

### eNPS as Industry Standard

Employee Net Promoter Score (eNPS) uses a simple 0-10 scale to gauge engagement. Its success demonstrates that simple scales, tracked consistently, provide actionable insights.

Our 1-5 scale is even simpler because:
- It's for individual tracking, not surveys
- It's observation-based, not self-reported
- It needs to be quick enough to use frequently

---

## UI Components

### Quick Morale Input - RPG Style

A fast entry component styled as healing/status menu.

```
╔═══════════════════════════════════════════════════════════════╗
║  RECORD MORALE - Alex Chen                                    ║
╠═══════════════════════════════════════════════════════════════╣
║                                                               ║
║  How is their morale?                                         ║
║                                                               ║
║    [1]       [2]       [3]       [4]       [5]               ║
║   ♥░░░░    ♥♥░░░    ♥♥♥░░    ♥♥♥♥░    ♥♥♥♥♥              ║
║  Critical   Low     Neutral   Good    Excellent              ║
║                                                               ║
║  Context: [After 1-on-1 ▼]  (optional)                       ║
║  Notes:   [                ] (optional)                       ║
║                                                               ║
╚═══════════════════════════════════════════════════════════════╝
```

Accessible from:
- Engineer profile page (character sheet)
- Dashboard (party screen)
- After completing a 1-on-1 (check-in complete)

### Morale History - HP Timeline

Visual timeline styled as character health log.

```
╔═══════════════════════════════════════════════════════════════╗
║  MORALE HISTORY - Alex Chen                    Last 30 days  ║
╠═══════════════════════════════════════════════════════════════╣
║                                                               ║
║  5 ♥♥♥♥♥ │            ●                                      ║
║  4 ♥♥♥♥░ │    ●   ●       ●   ●                              ║
║  3 ♥♥♥░░ │  ●   ●   ●           ●   ●                        ║
║  2 ♥♥░░░ │                            ●                       ║
║  1 ♥░░░░ │                                                    ║
║          └─────────────────────────────────────────────────   ║
║            Jan 1                              Jan 30          ║
║                                                               ║
║  TREND: ↓ Declining (-0.8 over 30 days)  ⚠ Needs attention  ║
╚═══════════════════════════════════════════════════════════════╝
```

### Status Effect Indicator

Shows current morale state as RPG status effects.

```
 ● HEALTHY   - Morale stable or improving (no indicator needed)
 ⚠ FATIGUED  - Morale declining gradually
 ☠ POISONED  - Consistent low morale (1-2) over multiple observations
 ★ BOOSTED   - Morale improving consistently
 ? UNKNOWN   - Not enough recent data (last observation >14 days)
```

### Party Alert Card

Compact alert styled as party member status.

```
╔════════════════════════════════════════════════════════════╗
║  ⚠ PARTY MEMBER NEEDS ATTENTION                           ║
╠════════════════════════════════════════════════════════════╣
║  ┌───┐                                                     ║
║  │▓▓▓│  Alex Chen              MORALE [██░░░░░░░░] 2/5    ║
║  └───┘  LV.3 Senior            ↓ Declining (was 4 → 2)    ║
║                                Last check: 5 days ago      ║
║         [Check In]  [Record Morale]  [View Stats →]       ║
╚════════════════════════════════════════════════════════════╝
```

---

## Data Requirements

### MoodEntry Entity

Reference: See [Data Model](../data-model.md) for full entity definition.

```typescript
interface MoodEntry {
  id: string;
  engineerId: string;

  // Mood Data
  score: MoodScore;              // 1-5 scale
  context?: MoodContext;         // What prompted the observation
  notes?: string;                // Additional notes

  // Timing
  recordedAt: Date;              // When observation was recorded
  observedAt?: Date;             // When the mood was observed (if different)

  // Metadata
  createdAt: Date;
}

type MoodScore = 1 | 2 | 3 | 4 | 5;

type MoodContext =
  | 'one_on_one'
  | 'team_meeting'
  | 'daily_standup'
  | 'casual_chat'
  | 'weekly_reflection'
  | 'other';
```

### Computed Fields

From the Engineer entity:

```typescript
interface EngineerMoodComputed {
  currentMood: MoodScore | null;     // Most recent
  moodTrend: 'improving' | 'stable' | 'declining' | 'unknown';
  averageMood30Days: number | null;
}
```

### Storage

- Mood stored in YAML frontmatter of journal entry files (`mood: 1-5`)
- Context stored in frontmatter (`context: meeting|standup|slack|other`)
- Entries use timestamp-based filenames (`YYYY-MM-DDTHHMMSS.md`)
- Legacy date-only filenames (`YYYY-MM-DD.md`) still supported
- Multiple entries per day supported (different timestamps)
- No remote sync (local-only by design)

---

## Privacy Considerations

### Manager's Private Notes

Mood observations are the manager's private notes about their team. They are:

- **Not shared with engineers**: These are your observations, not their self-assessments
- **Not shared with HR**: This is your personal tool, not a performance system
- **Not used for reviews**: Observations help you support people, not evaluate them
- **Local storage only**: Data never leaves your device

### Ethical Use

This feature helps you be a better manager by:
- Noticing when someone needs support
- Tracking whether your interventions help
- Having informed, caring conversations

It should never be used for:
- Performance management decisions
- Justifying terminations
- Comparing team members
- Surveillance

### Data Retention

Consider periodically reviewing and cleaning old data. Historical mood trends beyond 12-18 months may not be relevant and could create liability.

---

## Related Documents

- [Data Model](../data-model.md) - Full entity definitions
- [Product Vision](../product-vision.md) - Overall product direction
- [User Stories](../user-stories.md) - Feature requirements in user story format
