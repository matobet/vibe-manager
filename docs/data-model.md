# Vibe Manager - Data Model

## Overview

Pure markdown workspace. No JSON, no database. Everything is markdown files with YAML frontmatter.

**Design principles:**
- **Markdown only** - No JSON files, everything human-readable
- **Frontmatter for data** - Structured fields in YAML frontmatter
- **Folders are engineers** - Each person = one folder
- **Files are meetings** - Each note file = one 1-on-1 record
- **Editable anywhere** - vim, VS Code, Obsidian, any text editor

## Folder Structure

```
my-team/                         # Workspace root
├── .vibe-manager                # Workspace marker + config (YAML)
├── alex-chen/
│   ├── _profile.md              # Engineer data + personal info
│   ├── 2026-01-15.md            # Meeting note
│   ├── 2026-01-22.md
│   └── ...
├── jordan-lee/
│   ├── _profile.md
│   └── ...
└── ...
```

**That's it.** No `data/` folder, no JSON files.

## File Formats

### .vibe-manager

Workspace marker and configuration. Plain YAML file.

```yaml
# Vibe Manager workspace
version: 1

settings:
  default_meeting_frequency: biweekly
  overdue_threshold_days: 3
```

The presence of this file marks a directory as a Vibe Manager workspace.

### {engineer-slug}/_profile.md

All engineer data lives here. Frontmatter for structured fields, markdown for notes.

```markdown
---
name: Alex Chen
title: Software Engineer
start_date: 2024-03-15
level: P3
meeting_frequency: weekly    # weekly | biweekly | monthly
active: true

# Personal
birthday: 1992-05-20
partner: Sarah
children: [Emma, Jack]

# Career Progress
skills:
  technical:
    code: proficient
    architecture: advanced
    security: developing
    testing: proficient
  delivery:
    planning: proficient
    ownership: advanced
  collaboration:
    communication: proficient
    teamwork: proficient
  leadership:
    mentoring: developing
    knowledge_sharing: proficient
skills_updated: 2026-01-10

# Display Color (auto-generated from name hash if not set)
color: "#6495ED"
---

# Alex Chen

## Background
From Seattle. CS degree from UW. Previously at Stripe for 3 years.

## Working Style
- Prefers morning 1-on-1s
- Appreciates direct feedback
- Interested in system design and architecture

## Notes
Mentioned wanting to explore tech lead path eventually. Has been
particularly engaged since joining the payments project.
```

### {engineer-slug}/{date}.md

Meeting notes. Filename is the date. Optional mood in frontmatter.

```markdown
---
mood: 4
---

# 1-on-1 - January 15, 2026

## Discussion
- Sprint progress looking good
- Career goals - interested in tech lead track

## Notes
Alex seems energized about the new project. Discussed potential
tech lead opportunities in Q2. Wants more exposure to system design.

## Action Items
- [ ] Share tech lead role description @me
- [x] Review Alex's design doc
- [ ] Schedule skip-level with Sarah @alex
```

**Frontmatter fields (all optional):**
- `mood` - Morale observation 1-5

**File exists = meeting happened.** No file = no meeting. No need for status field.

## Derived Data (Computed at Runtime)

These are NOT stored, calculated when needed:

**Per Engineer:**
- `last_meeting_date` - From most recent note file
- `days_since_meeting` - Current date minus last meeting
- `is_overdue` - days_since > meeting frequency threshold
- `mood_trend` - From recent note frontmatter mood values (Rising/Stable/Falling)
- `recent_mood` - Most recent mood score (1-5)
- `urgency_score` - Composite score for sorting (higher = needs more attention)
- `color` - Display color (from profile or auto-generated from name hash)

**Urgency Score Calculation:**
| Factor | Points |
|--------|--------|
| Never had a meeting | +100 |
| Days overdue (past frequency + threshold) | +10 per day (max 80) |
| Approaching due date (within 2 days) | +5 |
| Low mood (1-2) | +20 |
| Falling mood trend | +15 |
| No mood data | +10 |

Engineers are sorted by urgency score descending, so the person needing most attention appears first.

**Workspace:**
- `team_size` - Count of active engineers
- `overdue_count` - Engineers past their meeting frequency
- `average_mood` - Mean of recent mood scores

## Multi-Team Support

Separate workspace folders for each team:

```
~/work/
├── platform-team/
│   ├── .vibe-manager
│   ├── alex-chen/
│   └── jordan-lee/
└── mobile-team/
    ├── .vibe-manager
    └── sam-taylor/
```

**CLI Usage:**
- `vibe-manager ./platform-team` - Open specific workspace
- `vibe-manager .` - Open current directory
- `vibe-manager init` - Initialize new workspace

## Design Principles

| Principle | Benefit |
|-----------|---------|
| **Pure markdown** | No JSON, no database, just text files |
| **Frontmatter = data** | Structured fields in readable YAML |
| **Folders = engineers** | `ls` shows your team |
| **Files = meetings** | `ls alex-chen/` shows meeting history |
| **Git-friendly** | Everything diffs well |
| **Editor-agnostic** | vim, VS Code, Obsidian, any tool works |

## Validation

**Profile (`_profile.md` frontmatter):**
- `name` - required
- `level` - P1 | P2 | P3 | P4 | P5
- `meeting_frequency` - weekly | biweekly | monthly
- `active` - true | false (default: true)

**Meeting note (`{date}.md`):**
- Filename - valid date: YYYY-MM-DD.md
- `mood` - 1-5 (optional)
- File exists = meeting happened

**Skills:**
- Proficiency values: learning | developing | proficient | advanced | expert

## Slug Rules

Folder name derived from engineer name:
- Lowercase
- Spaces → hyphens
- Remove special characters
- `"Alex Chen"` → `alex-chen/`
- `"María García"` → `maria-garcia/`
