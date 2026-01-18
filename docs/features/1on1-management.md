# Feature Specification: 1-on-1 Management

## 1. Feature Overview

### Purpose

The 1-on-1 Management feature is the core scheduling and tracking system for Vibe Manager. It ensures engineering managers never miss a 1-on-1 meeting and maintains consistent check-in rhythms with each team member based on their individual needs.

### Core Functionality

- **Per-engineer meeting frequency settings** - Different meeting frequencies for different people
- **Overdue detection** - Visual alerts when meetings are past due
- **Meeting lifecycle management** - Schedule, complete, reschedule, or skip meetings
- **At-a-glance status** - Dashboard indicators for quick prioritization
- **Smart frequency suggestions** - Context-aware recommendations (future enhancement)

### Why This Matters

Regular 1-on-1s are the foundation of effective people management. When an EM manages 5-10 engineers with varying needs, it is easy to let meetings slip. This feature provides the structure to ensure every team member gets appropriate attention.

---

## 2. User Needs

### Pain Points Addressed

| Pain Point | How This Feature Helps |
|------------|------------------------|
| Meetings slip without noticing | Clear overdue indicators on dashboard |
| One-size-fits-all meeting frequency does not work | Per-engineer frequency settings |
| Uncertainty about who to meet next | Urgency-based prioritization |
| No record of when meetings happened | Automatic tracking of meeting dates |
| Calendar chaos when rescheduling | Built-in reschedule and skip tracking |
| Forgetting to adjust for special circumstances | Manual overrides and custom meeting frequencys |

### User Goals

1. **Awareness** - Always know who is overdue for a 1-on-1
2. **Flexibility** - Customize frequency per engineer based on their needs
3. **Efficiency** - Quick recording of completed meetings without friction
4. **Planning** - See upcoming meetings to plan the week ahead
5. **Accountability** - Track patterns over time to improve consistency

---

## 3. Functional Requirements

### 3.1 Per-Engineer Meeting Frequency Settings

Each engineer can have an individual meeting meeting frequency configured.

| Meeting Frequency Option | Interval | Typical Use Case |
|----------------|----------|------------------|
| Weekly | 7 days | Junior engineers, high challenges, performance concerns |
| Bi-weekly | 14 days | Most mid-level engineers (default) |
| Monthly | 30 days | Senior/staff engineers, stable and autonomous |
| Quarterly | 90 days | Skip-level or very senior (rare) |
| Custom | N days | Special circumstances |

**Behaviors:**
- Default meeting frequency is configurable in settings (recommended: bi-weekly)
- Changing meeting frequency recalculates overdue status immediately
- Custom meeting frequency allows any number of days (minimum: 1, maximum: 365)
- Meeting Frequency is stored on the Engineer entity as `oneOnOneMeeting Frequency`

### 3.2 Tracking Last Meeting Date and Days Since

The system tracks when the last 1-on-1 occurred for each engineer.

**Computed Fields:**
- `lastOneOnOneDate` - Date of most recent completed meeting
- `daysSinceLastOneOnOne` - Integer days since last meeting
- `nextSuggestedDate` - Calculated as lastOneOnOneDate + meeting frequency interval

**Display Logic:**
```
Days Since = today - lastOneOnOneDate

If no meetings exist:
  Days Since = today - engineer.startDate
  (Treat start date as baseline)
```

### 3.3 Overdue Detection

A meeting is considered overdue when the days since last meeting exceeds the engineer's meeting frequency.

**Overdue Calculation:**
```
isOverdue = daysSinceLastOneOnOne > meeting frequencyInDays

daysOverdue = daysSinceLastOneOnOne - meeting frequencyInDays
daysUntilOverdue = meeting frequencyInDays - daysSinceLastOneOnOne
```

**Status Categories:**

| Status | Condition | Dashboard Color |
|--------|-----------|-----------------|
| Overdue | daysOverdue > 0 | Red |
| Due Soon | daysUntilOverdue <= 3 | Yellow/Amber |
| On Track | daysUntilOverdue > 3 | Green |
| No Data | No meetings recorded | Gray (with prompt) |

### 3.4 Scheduling Future Meetings

Users can schedule meetings in advance rather than just recording completed ones.

**Meeting Fields:**
- `scheduledDate` - When the meeting is planned
- `duration` - Meeting length in minutes (default: 30)
- `status` - One of: `scheduled`, `completed`, `cancelled`, `skipped`

**Behaviors:**
- Scheduling a meeting does not change overdue status (only completion does)
- Can have multiple scheduled meetings per engineer
- Scheduled meetings appear in upcoming meetings list
- Optional: Set preferred day/time per engineer for scheduling suggestions

### 3.5 Recording Completed Meetings

When a meeting is completed, it updates tracking and allows note-taking.

**Completion Flow:**
1. User clicks "Mark Complete" on scheduled or ad-hoc meeting
2. System records `actualDate` (defaults to today)
3. System sets `status` to `completed`
4. Optional: Opens note editor for meeting notes
5. Overdue status recalculates based on new last meeting date

**Quick Complete (Dashboard):**
- One-click completion without entering notes
- Can add notes later from meeting history

### 3.6 Reschedule and Skip Functionality

**Reschedule:**
- Change `scheduledDate` to a new date
- Meeting remains in `scheduled` status
- Does not affect meeting frequency tracking

**Skip:**
- Set `status` to `skipped`
- Optional reason field (stored as note)
- Skipped meetings do NOT reset meeting frequency tracking
- Useful for: PTO, holidays, conflict weeks, engineer preference

**Cancel:**
- Set `status` to `cancelled`
- Similar to skip but typically means meeting will not be rescheduled
- Does not reset meeting frequency tracking

---

## 4. Smart Frequency Logic

### 4.1 Factors Affecting Meeting Frequency

The appropriate 1-on-1 meeting frequency depends on several factors about the engineer:

| Factor | Data Source | Impact on Frequency |
|--------|-------------|---------------------|
| Seniority Level | `seniorityCategory` | Junior needs more frequent |
| Current Challenges | `currentChallenges` | High challenges need more attention |
| Performance Status | `performanceNotes` presence | Concerns warrant extra support |
| Time on Team | Computed from `startDate` | New hires need more frequent |
| Career Stage | `currentLevel` vs `targetLevel` | Active development needs check-ins |

### 4.2 Seniority-Based Defaults

| Seniority | Recommended Meeting Frequency | Rationale |
|-----------|---------------------|-----------|
| Junior (P1-P2) | Weekly | More guidance needed, faster feedback loops |
| Mid (P3) | Bi-weekly | Balance of autonomy and support |
| Senior (P4) | Bi-weekly to Monthly | More autonomous, strategic discussions |
| Staff (P5) | Monthly | Peer-level relationship, specific topics |

### 4.3 Challenge-Level Adjustments

| Challenge Level | Adjustment | Scenarios |
|-----------------|------------|-----------|
| Low | Maintain or extend meeting frequency | Stable, executing well |
| Medium | Maintain meeting frequency | Normal workload, typical challenges |
| High | Consider weekly | Difficult project, learning curve, conflict |
| Critical | Weekly or more | Performance concern, burnout risk, major blocker |

### 4.4 Performance Support

When there are performance concerns (indicated by `performanceNotes` being populated or explicit flag):
- Suggest weekly meeting frequency regardless of seniority
- Provide more structure and documentation
- Track progress more closely

### 4.5 Future Enhancement: Smart Suggestions

**Planned Feature (P2):**
The system will suggest meeting frequency adjustments based on:
- Engineer attributes changing (e.g., challenges increased)
- Patterns in mood tracking (declining mood = suggest more frequent)
- Time since last meeting frequency review
- Approaching milestones (anniversary, level change)

**Suggestion UI:**
```
[Suggestion Card]
Consider increasing 1-on-1 frequency for Sarah
Reason: Challenge level changed to "high" last week
Current: Bi-weekly | Suggested: Weekly
[Accept] [Dismiss] [Snooze]
```

---

## 5. UI Components

### 5.1 Dashboard Indicators

**Team Overview Card (per engineer):**
```
+------------------------------------------+
|  [Photo] Sarah Chen                      |
|  Senior Engineer | P4                    |
|                                          |
|  1-on-1: [RED] 5 days overdue            |
|  Last: Jan 10 (12 days ago)              |
|  Meeting Frequency: Bi-weekly                      |
|                                          |
|  [Quick Complete] [Schedule] [View]      |
+------------------------------------------+
```

**Status Badge Colors:**
- Red badge: Overdue
- Yellow/Amber badge: Due in 3 days or less
- Green badge: On track
- Gray badge: No meetings yet / new engineer

### 5.2 Overdue Alerts

**Dashboard Alert Banner:**
```
[!] 2 team members are overdue for 1-on-1s
    Sarah Chen (5 days) | Mike Johnson (2 days)
    [View All]
```

**Notification Triggers (Future):**
- Day meeting becomes overdue
- 3 days before meeting due (reminder)
- Weekly summary of 1-on-1 status

### 5.3 Quick Actions

Available from dashboard without navigation:

| Action | Description |
|--------|-------------|
| Quick Complete | Mark 1-on-1 done with one click, add notes later |
| Schedule | Open date picker to schedule next meeting |
| Skip | Mark as skipped with optional reason |
| View Profile | Navigate to full engineer profile |

### 5.4 Meeting History List

**Per-Engineer Meeting Timeline:**
```
Meetings with Sarah Chen
------------------------
[Completed] Jan 10, 2026 - 30 min
  Notes: Discussed project concerns...
  Action Items: 2 open

[Skipped] Jan 3, 2026
  Reason: Sarah on PTO

[Completed] Dec 20, 2025 - 45 min
  Notes: Year-end reflection...
```

### 5.5 Scheduling Interface

**Schedule New Meeting:**
```
Schedule 1-on-1 with Sarah Chen
-------------------------------
Date: [Jan 24, 2026    ] (picker)
Time: [2:00 PM         ] (optional)
Duration: [30 min      ] (dropdown)

Suggested: Friday (Sarah's preferred day)
Next due: Jan 24 (based on bi-weekly meeting frequency)

[Cancel] [Schedule]
```

---

## 6. Data Requirements

### 6.1 Meeting Entity

Reference: `Meeting` interface from data-model.md

```typescript
interface Meeting {
  id: string;
  engineerId: string;            // Reference to Engineer

  // Scheduling
  scheduledDate: Date;
  duration: number;              // Minutes (default: 30)
  status: MeetingStatus;         // scheduled | completed | cancelled | skipped

  // Completion
  actualDate?: Date;             // When it actually happened
  completedAt?: Date;            // When marked complete

  // Metadata
  createdAt: Date;
  updatedAt: Date;
}
```

### 6.2 Engineer Fields Used

From the Engineer entity:

| Field | Usage in 1-on-1 Management |
|-------|----------------------------|
| `oneOnOneMeeting Frequency` | Target frequency setting |
| `preferredDay` | Scheduling suggestions |
| `preferredTime` | Scheduling suggestions |
| `seniorityCategory` | Smart frequency recommendations |
| `currentChallenges` | Frequency adjustment suggestions |
| `startDate` | Baseline for new engineers |
| `isActive` | Filter out archived engineers |

### 6.3 Computed Properties

From EngineerComputed:

| Property | Calculation |
|----------|-------------|
| `lastOneOnOneDate` | Most recent completed meeting date |
| `daysSinceLastOneOnOne` | Days between today and last meeting |
| `isOverdue` | daysSince > meeting frequencyInDays |
| `daysUntilOverdue` | meeting frequencyInDays - daysSince |
| `nextSuggestedDate` | lastMeeting + meeting frequencyInterval |

### 6.4 Database Indexes

For efficient queries:

| Index | Purpose |
|-------|---------|
| `meetings(engineerId, scheduledDate)` | Find meetings by engineer |
| `meetings(status, scheduledDate)` | Find upcoming/overdue |
| `meetings(status)` where completed | Calculate last meeting dates |

---

## 7. Edge Cases and Special Scenarios

### 7.1 New Engineers

**Scenario:** Engineer just joined, no meetings recorded yet.

**Handling:**
- Use `startDate` as baseline for first meeting due date
- Show special "New - needs first 1-on-1" indicator
- Suggest meeting within first week
- Do not show as "overdue" immediately; use "first meeting pending" status

### 7.2 Returning from Leave

**Scenario:** Engineer returns from extended PTO or leave.

**Handling:**
- Consider adding "return from leave" meeting type
- Do not count leave period as "overdue" time
- Option: Pause meeting frequency tracking during leave (future feature)
- Practical MVP: Manager manually adjusts or skips during leave

### 7.3 Meeting Frequency Changes

**Scenario:** Manager changes meeting frequency from weekly to monthly.

**Handling:**
- Overdue status recalculates immediately
- If was overdue on weekly, may become on-track with monthly
- Show confirmation: "This will change status from Overdue to On Track"

### 7.4 Multiple Meetings Same Day

**Scenario:** Manager has two 1-on-1s with same engineer in one day.

**Handling:**
- Allow multiple meetings (edge case but valid)
- Last meeting date uses most recent
- Each meeting can have separate notes

### 7.5 Backdated Meetings

**Scenario:** Manager forgot to log a meeting, enters it days later.

**Handling:**
- Allow setting `actualDate` to past date when completing
- Recalculate overdue status based on actual date, not today
- Show in history at correct chronological position

### 7.6 Engineer Transfers

**Scenario:** Engineer moves to different team/manager.

**Handling:**
- Archive engineer (set `isActive: false`)
- Meeting history preserved
- If they return, reactivate and resume tracking
- Consider fresh start vs. continuing meeting frequency

### 7.7 Holiday Weeks

**Scenario:** Holiday period with company closure.

**Handling:**
- Use skip functionality with "Holiday" reason
- Meeting Frequency continues but skip is documented
- Alternative: Manager can adjust expected date manually (future feature)

### 7.8 Very Overdue Meetings

**Scenario:** Meeting is extremely overdue (e.g., 30+ days past weekly meeting frequency).

**Handling:**
- Show escalated visual warning
- Display actual days overdue, not capped
- Consider: Dashboard sort puts most overdue at top
- No automatic actions; manager decides priority

### 7.9 Timezone Considerations

**Scenario:** Single-user app, but dates matter.

**Handling:**
- All dates in user's local timezone
- Date comparisons use start of day
- "Today" means user's current date

---

## 8. Related User Stories

This feature implements the following user stories from the product backlog:

| Story ID | Title | Priority |
|----------|-------|----------|
| US-101 | View 1-on-1 Status | P0 |
| US-102 | Set Per-Engineer Meeting Frequency | P0 |
| US-103 | Record a Completed 1-on-1 | P0 |
| US-104 | View Upcoming 1-on-1s | P0 |
| US-105 | Schedule a 1-on-1 | P1 |
| US-106 | Reschedule or Skip a 1-on-1 | P1 |
| US-107 | Smart Meeting Frequency Suggestions | P2 |

---

## 9. Implementation Notes

### MVP Scope

For initial release, focus on:
1. Meeting Frequency settings per engineer
2. Recording completed meetings (one-click)
3. Dashboard overdue indicators
4. Days since tracking
5. Basic skip functionality

### Deferred to Post-MVP

- Smart frequency suggestions
- Calendar integration
- Reminders and notifications
- Pause meeting frequency during leave
- Meeting agenda templates

### Technical Considerations

- All date calculations client-side (local-first architecture)
- Computed properties recalculate on data change
- IndexedDB for meeting storage
- Dashboard queries should be efficient for 10 engineers
