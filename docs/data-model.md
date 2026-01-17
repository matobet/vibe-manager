# Vibe Manager - Data Model

## Overview

This document defines the core data structures for Vibe Manager. The primary entity is the **Engineer** (direct report), with supporting entities for notes, mood entries, and career tracking.

## Entity Relationship Diagram

```
┌─────────────────┐       ┌─────────────────┐
│    Engineer     │───────│   MoodEntry     │
│                 │ 1   * │                 │
└─────────────────┘       └─────────────────┘
        │
        │ 1
        │
        │ *
┌─────────────────┐
│  MeetingNote    │
└─────────────────┘
        │
        │ 1
        │
        │ 1
┌─────────────────┐
│    Meeting      │
└─────────────────┘

┌─────────────────┐       ┌─────────────────┐
│    Engineer     │───────│ CareerProgress  │
│                 │ 1   * │                 │
└─────────────────┘       └─────────────────┘

┌─────────────────┐
│   CareerPath    │ (Reference data - R&D Career Path)
└─────────────────┘
```

## Core Entities

### Engineer

The primary entity representing a direct report.

```typescript
interface Engineer {
  // Identity
  id: string;                    // Unique identifier
  name: string;                  // Full name
  email?: string;                // Work email (optional)
  photoUrl?: string;             // Profile photo URL (optional)

  // Employment
  title: string;                 // Current job title
  startDate: Date;               // When they joined the team
  previousRoles?: string[];      // Previous roles/history
  howTheyJoined?: string;        // Notes on how they joined

  // 1-on-1 Settings
  oneOnOneCadence: Cadence;      // Target meeting frequency
  preferredDay?: DayOfWeek;      // Preferred day for 1-on-1s
  preferredTime?: string;        // Preferred time slot

  // Career
  currentLevel: CareerLevel;     // Current career path level (P1-P5)
  targetLevel?: CareerLevel;     // Target level they're working toward

  // Attributes
  seniorityCategory: SeniorityCategory;  // Junior/Mid/Senior
  currentChallenges: ChallengeLevel;     // For smart frequency suggestions
  performanceNotes?: string;             // Performance observations

  // Personal Knowledge Base
  personalInfo: PersonalInfo;

  // Preferences
  communicationStyle?: string;   // How they prefer to communicate
  workHours?: string;            // Typical working hours
  interests?: string[];          // Hobbies and interests

  // Metadata
  createdAt: Date;
  updatedAt: Date;
  isActive: boolean;             // Still on the team?
}
```

### PersonalInfo (Embedded in Engineer)

Personal details for the knowledge base.

```typescript
interface PersonalInfo {
  // Family
  partnerName?: string;
  hasChildren: boolean;
  childrenNames?: string[];
  petInfo?: string;              // "Dog named Max", etc.

  // Important Dates
  birthday?: Date;
  workAnniversary?: Date;        // Derived from startDate
  otherDates?: ImportantDate[];  // Custom important dates

  // Background
  hometown?: string;
  education?: string;
  previousCompanies?: string[];

  // Notes
  personalNotes?: string;        // Free-form markdown notes
}

interface ImportantDate {
  date: Date;
  label: string;                 // "Wedding anniversary", etc.
  recurring: boolean;            // Repeats yearly?
}
```

### Meeting

A scheduled or completed 1-on-1 meeting.

```typescript
interface Meeting {
  id: string;
  engineerId: string;            // Reference to Engineer

  // Scheduling
  scheduledDate: Date;
  duration: number;              // Minutes
  status: MeetingStatus;         // scheduled | completed | cancelled | skipped

  // Completion
  actualDate?: Date;             // When it actually happened
  completedAt?: Date;            // When marked complete

  // Metadata
  createdAt: Date;
  updatedAt: Date;
}

type MeetingStatus = 'scheduled' | 'completed' | 'cancelled' | 'skipped';
```

### MeetingNote

Markdown notes for a meeting.

```typescript
interface MeetingNote {
  id: string;
  meetingId: string;             // Reference to Meeting
  engineerId: string;            // Reference to Engineer (denormalized)

  // Content
  content: string;               // Markdown content

  // Optional Structure (extracted from content)
  actionItems?: ActionItem[];    // Parsed action items

  // Metadata
  createdAt: Date;
  updatedAt: Date;
}

interface ActionItem {
  id: string;
  text: string;
  isCompleted: boolean;
  dueDate?: Date;
  owner?: string;                // "me" or engineer name
}
```

### MoodEntry

A mood/health observation.

```typescript
interface MoodEntry {
  id: string;
  engineerId: string;            // Reference to Engineer

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
  | 'one_on_one'                 // Observed during 1-on-1
  | 'team_meeting'               // Observed in team meeting
  | 'daily_standup'              // Observed in standup
  | 'casual_chat'                // Informal observation
  | 'weekly_reflection'          // Manager's weekly reflection
  | 'other';
```

### CareerProgress

Tracks progress on career path skills/pillars.

```typescript
interface CareerProgress {
  id: string;
  engineerId: string;            // Reference to Engineer

  // Career Path Reference
  pillar: CareerPillar;          // Which pillar of the career path
  skill: string;                 // Specific skill within pillar

  // Assessment
  currentProficiency: ProficiencyLevel;
  targetProficiency?: ProficiencyLevel;

  // Evidence
  notes?: string;                // Markdown notes on progress
  lastAssessedAt: Date;

  // Metadata
  createdAt: Date;
  updatedAt: Date;
}
```

## Enumerations

### Cadence (1-on-1 Frequency)

```typescript
type Cadence =
  | 'weekly'           // Every week
  | 'biweekly'         // Every 2 weeks
  | 'monthly'          // Every month
  | 'quarterly'        // Every quarter
  | 'custom';          // Custom interval (days)

interface CadenceConfig {
  type: Cadence;
  customDays?: number; // Only for 'custom' type
}
```

### Career Path Levels

Based on the R&D Career Path structure.

```typescript
type CareerLevel = 'P1' | 'P2' | 'P3' | 'P4' | 'P5';

type CareerPillar =
  | 'technical_skills'
  | 'delivery'
  | 'collaboration'
  | 'leadership';

type ProficiencyLevel =
  | 'learning'         // Just starting
  | 'developing'       // Making progress
  | 'proficient'       // Meets expectations
  | 'advanced'         // Exceeds expectations
  | 'expert';          // Role model
```

### Seniority Categories

```typescript
type SeniorityCategory = 'junior' | 'mid' | 'senior' | 'staff';

type ChallengeLevel = 'low' | 'medium' | 'high' | 'critical';
```

### Days of Week

```typescript
type DayOfWeek =
  | 'monday'
  | 'tuesday'
  | 'wednesday'
  | 'thursday'
  | 'friday';
```

## Computed Properties

### Engineer Computed Fields

```typescript
// These are derived, not stored
interface EngineerComputed {
  // 1-on-1 Status
  lastOneOnOneDate: Date | null;
  daysSinceLastOneOnOne: number;
  isOverdue: boolean;
  daysUntilOverdue: number;
  nextSuggestedDate: Date;

  // Mood
  currentMood: MoodScore | null;     // Most recent
  moodTrend: 'improving' | 'stable' | 'declining' | 'unknown';
  averageMood30Days: number | null;

  // Career
  timeInCurrentLevel: number;        // Months
  progressToNextLevel: number;       // Percentage (0-100)

  // Tenure
  tenureMonths: number;
  upcomingAnniversary: Date | null;
  upcomingBirthday: Date | null;
}
```

## Data Storage

### Folder-Based Workspaces

Each team is stored in its own folder (like an Obsidian vault). The app opens a folder and treats it as the workspace:

```
my-team/                     # Any folder becomes a Vibe Manager workspace
├── .vibe-manager.json       # Workspace marker + settings
├── data/
│   ├── engineers.json       # All engineer records
│   ├── meetings.json        # All meeting records
│   ├── mood_entries.json    # All mood observations
│   └── career_progress.json # All career tracking entries
└── notes/
    ├── alex-chen/
    │   ├── 2026-01-15.md    # Meeting notes by date
    │   ├── 2026-01-22.md
    │   └── personal.md      # Free-form personal notes
    ├── jordan-lee/
    │   └── ...
    └── ...
```

### Multi-Team Support

Users can manage multiple teams by having separate folders:

```
~/work/
├── platform-team/           # One team
│   ├── .vibe-manager.json
│   ├── data/
│   └── notes/
├── mobile-team/             # Another team
│   ├── .vibe-manager.json
│   ├── data/
│   └── notes/
└── ...
```

**Usage:**
- `vibe-manager ./platform-team` - Open specific workspace
- `vibe-manager .` - Open current directory as workspace
- `vibe-manager init` - Initialize current directory as new workspace

### Design Principles

| Principle | Implementation |
|-----------|----------------|
| **Human-readable** | JSON with pretty-printing, standard markdown |
| **Git-friendly** | Text files that diff well, suitable for version control |
| **No database server** | Plain files, no SQLite/Postgres needed |
| **Easy backup** | Just copy the workspace folder |
| **Portable** | Move/sync folder between machines, store in Dropbox, etc. |
| **Multi-workspace** | Manage multiple teams in separate folders |

### Markdown-Native Notes

Meeting notes are stored as individual markdown files:
- One file per meeting: `{engineer-slug}/{date}.md`
- Files are directly editable in any text editor
- Can be browsed/searched with standard Unix tools
- Compatible with Obsidian, VS Code, vim, etc.

### JSON Data Format

Structured data uses JSON for simplicity:
- Pretty-printed for human readability
- Arrays of records with IDs for relationships
- Easy to inspect, debug, and manually edit if needed
- Standard format with wide tooling support

## Data Validation Rules

### Engineer
- `name` is required, non-empty
- `startDate` must be in the past
- `oneOnOneCadence` must be a valid cadence type
- `currentLevel` must be a valid career level

### Meeting
- `scheduledDate` is required
- `duration` must be positive (default: 30)
- `engineerId` must reference valid Engineer

### MoodEntry
- `score` must be 1-5
- `engineerId` must reference valid Engineer
- `recordedAt` cannot be in the future

### MeetingNote
- `content` can be empty but must exist
- `meetingId` must reference valid Meeting
