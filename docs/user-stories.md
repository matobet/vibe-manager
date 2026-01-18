# Vibe Manager - User Stories

## Overview

This document contains prioritized user stories for Vibe Manager, organized by feature area. Stories are written from the perspective of an Engineering Manager (EM).

## Priority Levels

- **P0**: Must have for MVP
- **P1**: Important, should include if possible
- **P2**: Nice to have, can defer
- **P3**: Future consideration

---

## 1-on-1 Management

### P0 - Core Functionality

**US-101: View 1-on-1 Status**
> As an EM, I want to see at a glance which team members are overdue for a 1-on-1, so I can prioritize my time.

Acceptance Criteria:
- Dashboard shows overdue 1-on-1s prominently
- Overdue defined as past the meeting frequency threshold for that engineer
- Clear visual distinction between overdue, due soon, and on track

**US-102: Set Per-Engineer Meeting Frequency**
> As an EM, I want to set a different 1-on-1 frequency for each engineer, because junior engineers need more frequent check-ins than senior ones.

Acceptance Criteria:
- Can set weekly, bi-weekly, monthly, or custom frequency per engineer
- Default meeting frequency is configurable
- Changing frequency updates overdue calculations immediately

**US-103: Record a Completed 1-on-1**
> As an EM, I want to mark a 1-on-1 as completed and add notes, so I have a record of our conversation.

Acceptance Criteria:
- One-click to mark meeting complete
- Can optionally add notes in markdown
- Meeting date is recorded
- Updates "days since last 1-on-1" calculation

**US-104: View Upcoming 1-on-1s**
> As an EM, I want to see who I should meet with next based on their meeting frequency, so I can plan my week.

Acceptance Criteria:
- List of engineers sorted by "urgency" (how overdue/due soon)
- Shows target date based on last meeting + frequency
- Can filter to this week's priorities

### P1 - Enhanced Features

**US-105: Schedule a 1-on-1**
> As an EM, I want to schedule a future 1-on-1 with a specific date, so I can plan ahead.

Acceptance Criteria:
- Can set date/time for future meeting
- Shows in calendar view
- Reminder before the meeting (configurable)

**US-106: Reschedule or Skip a 1-on-1**
> As an EM, I want to reschedule or skip a 1-on-1 with a reason, so I can track why meetings were moved.

Acceptance Criteria:
- Can mark as skipped with optional reason
- Can reschedule to new date
- Skipped meetings don't reset frequency tracking

### P2 - Future Enhancements

**US-107: Smart Frequency Suggestions**
> As an EM, I want the app to suggest meeting frequency based on engineer attributes, so I don't have to manually calibrate.

Acceptance Criteria:
- Suggests more frequent for junior engineers
- Suggests more frequent when challenges are high
- User can accept or override suggestion

---

## Meeting Notes

### P0 - Core Functionality

**US-201: Write Meeting Notes**
> As an EM, I want to write free-form markdown notes during or after a 1-on-1, so I can capture important discussion points.

Acceptance Criteria:
- Full markdown support
- Auto-saves as I type
- Associated with specific meeting
- Can edit notes after saving

**US-202: View Past Notes**
> As an EM, I want to quickly view notes from previous 1-on-1s before my next meeting, so I can prepare and follow up.

Acceptance Criteria:
- Notes listed chronologically per engineer
- Search within notes
- Preview without full page load

**US-203: View All Notes for an Engineer**
> As an EM, I want to see all historical notes for an engineer in one place, so I can understand patterns over time.

Acceptance Criteria:
- Scrollable timeline of all notes
- Can expand/collapse individual entries
- Date and meeting info visible

### P1 - Enhanced Features

**US-204: Action Item Tracking**
> As an EM, I want to track action items from 1-on-1s, so I can follow up on commitments.

Acceptance Criteria:
- Can mark items as action items in notes
- See open action items across all engineers
- Mark action items as complete

**US-205: Note Templates**
> As an EM, I want optional templates for my notes, so I can maintain consistency.

Acceptance Criteria:
- Create/edit templates
- Apply template when starting new note
- Template is just a starting point, fully editable

---

## Health/Mood Tracking

### P0 - Core Functionality

**US-301: Record Mood Observation**
> As an EM, I want to record a mood score (1-5) for an engineer at any time, so I can track their happiness over time.

Acceptance Criteria:
- Quick input (1-5 scale)
- Optional context (when observed)
- Optional notes
- Not tied to specific meeting

**US-302: View Mood History**
> As an EM, I want to see mood history for an engineer, so I can identify trends.

Acceptance Criteria:
- Visual chart of mood over time
- Shows trend direction (improving/declining/stable)
- Can see individual entries with notes

**US-303: Dashboard Mood Alerts**
> As an EM, I want to see on my dashboard if any engineer's mood is declining, so I can proactively reach out.

Acceptance Criteria:
- Visual indicator for declining mood
- Threshold is configurable
- Shows current vs. average mood

### P1 - Enhanced Features

**US-304: Mood Context**
> As an EM, I want to note the context when I recorded a mood (after 1-on-1, in standup, etc.), so I can understand the data better.

Acceptance Criteria:
- Dropdown for common contexts
- Can add custom context
- Context shown in history

---

## Career Tracking

### P0 - Core Functionality

**US-401: Set Engineer's Current Level**
> As an EM, I want to record each engineer's current career level (P1-P5), so I can track their progress.

Acceptance Criteria:
- Select from defined career levels
- Stored in engineer profile
- Change history tracked

**US-402: View Career Path Progress**
> As an EM, I want to see an engineer's progress against the R&D Career Path skills, so I can guide their development.

Acceptance Criteria:
- Shows all 4 pillars
- Shows skills within each pillar
- Shows proficiency level for each skill

**US-403: Update Skill Assessment**
> As an EM, I want to update an engineer's proficiency in a specific skill, so I can track their growth.

Acceptance Criteria:
- Select skill and proficiency level
- Add notes on evidence/observations
- Timestamp of assessment recorded

### P1 - Enhanced Features

**US-404: Career Progression Summary**
> As an EM, I want to see a summary of an engineer's career progression over time, so I can prepare for promotion discussions.

Acceptance Criteria:
- Timeline of level changes
- Progress on skills over time
- Time in current level

**US-405: Set Development Goals**
> As an EM, I want to set target skills for an engineer to develop, so we can focus their growth.

Acceptance Criteria:
- Mark skills as "focus areas"
- Track progress toward targets
- Show in engineer profile

---

## Knowledge Base

### P0 - Core Functionality

**US-501: Store Personal Information**
> As an EM, I want to store personal details about each engineer (family, important dates), so I can be a more thoughtful manager.

Acceptance Criteria:
- Partner name, kids, pets fields
- Birthday and custom important dates
- All fields optional

**US-502: View Upcoming Important Dates**
> As an EM, I want to see upcoming birthdays and anniversaries, so I can acknowledge them.

Acceptance Criteria:
- Shows dates in next 30 days
- Work anniversary auto-calculated from start date
- Can add custom recurring dates

**US-503: Store Work History**
> As an EM, I want to record how each engineer joined the team and their background, so I remember their journey.

Acceptance Criteria:
- Previous roles field
- How they joined notes
- Previous companies

### P1 - Enhanced Features

**US-504: Store Preferences**
> As an EM, I want to record each engineer's preferences (communication style, work hours), so I can work with them effectively.

Acceptance Criteria:
- Communication style notes
- Work hours
- Interests/hobbies list

**US-505: Free-Form Personal Notes**
> As an EM, I want a general notes area for each engineer, so I can capture anything that doesn't fit structured fields.

Acceptance Criteria:
- Markdown text area
- No character limit
- Auto-saves

---

## Team Overview Dashboard

### P0 - Core Functionality

**US-601: Team Overview at a Glance**
> As an EM, I want a dashboard showing my whole team's status, so I can quickly see who needs attention.

Acceptance Criteria:
- Shows all active engineers
- 1-on-1 status for each (overdue, due soon, OK)
- Mood indicator for each

**US-602: Sort and Filter Team**
> As an EM, I want to sort my team by different criteria (overdue, mood, tenure), so I can focus on what matters.

Acceptance Criteria:
- Sort by: 1-on-1 urgency, mood score, name, tenure
- Filter by: overdue only, seniority level
- Persists my preference

**US-603: Quick Actions from Dashboard**
> As an EM, I want to quickly record a 1-on-1 or mood from the dashboard, so I don't have to navigate away.

Acceptance Criteria:
- Quick "mark complete" button for 1-on-1
- Quick mood entry (1-5) inline
- Opens detail view for notes

### P1 - Enhanced Features

**US-604: Weekly Summary View**
> As an EM, I want a weekly summary showing who I met with and mood changes, so I can reflect on the week.

Acceptance Criteria:
- Meetings completed this week
- Mood changes flagged
- Action items created/completed

---

## Engineer Profile

### P0 - Core Functionality

**US-701: Create Engineer Profile**
> As an EM, I want to add a new engineer to my team, so I can start tracking them.

Acceptance Criteria:
- Name (required)
- Title, start date
- Career level
- 1-on-1 meeting frequency

**US-702: View Engineer Profile**
> As an EM, I want to view all information about an engineer in one place, so I can prepare for conversations.

Acceptance Criteria:
- Profile info
- Recent 1-on-1s and notes
- Mood history
- Career progress
- Knowledge base info

**US-703: Edit Engineer Profile**
> As an EM, I want to update engineer information, so I can keep it current.

Acceptance Criteria:
- Edit all fields
- Changes tracked with timestamps
- Can update career level

**US-704: Archive Engineer**
> As an EM, I want to archive an engineer who has left or moved teams, so my dashboard stays focused.

Acceptance Criteria:
- Mark as inactive/archived
- Removed from dashboard
- Data preserved for reference
- Can reactivate if they return

---

## Settings & Data

### P0 - Core Functionality

**US-801: Configure Defaults**
> As an EM, I want to set default values (meeting frequency, etc.), so new engineers have sensible starting points.

Acceptance Criteria:
- Default 1-on-1 meeting frequency
- Default career level
- Overdue threshold

---

## Story Map Summary

```
                    MVP (P0)                 Enhanced (P1)              Future (P2/P3)
                    --------                 -------------              --------------
1-on-1s            View status              Schedule ahead             Smart suggestions
                   Set frequency            Reschedule/skip            Calendar sync
                   Record complete
                   View upcoming

Notes              Write markdown           Action items               --
                   View history             Templates
                   Per-engineer view

Mood               Record score             Context tracking           AI insights
                   View history
                   Dashboard alerts

Career             Set level                Progression summary        Gap analysis
                   View progress            Development goals
                   Update skills

Knowledge          Store personal info      Preferences                --
                   Important dates          Free-form notes
                   Work history

Dashboard          Team overview            Weekly summary             --
                   Sort/filter
                   Quick actions

Profile            Create engineer          --                         --
                   View/edit profile
                   Archive
```
