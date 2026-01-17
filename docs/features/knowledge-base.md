# Knowledge Base Feature Specification

## Feature Overview

The Knowledge Base is a core feature of Vibe Manager that enables engineering managers to build comprehensive profiles for each team member. It goes beyond basic employment information to capture the personal details that help managers form genuine, meaningful relationships with their direct reports.

Great management is not just about project delivery and career progression. It is about seeing each person as a whole human being with a life outside of work, personal milestones worth celebrating, and unique preferences that shape how they work best. The Knowledge Base provides a structured yet flexible way to remember these important details.

By maintaining a personal knowledge base for each engineer, managers can:

- Ask about their partner by name in casual conversation
- Remember to congratulate them on their child's birthday
- Respect their communication preferences and work hours
- Acknowledge work anniversaries and tenure milestones
- Reference their journey to the team and career history

This transforms routine check-ins into opportunities for authentic connection.

---

## User Needs

### Be a More Thoughtful Manager

Engineering managers often oversee 5-10 direct reports, each with their own personal circumstances, family situations, and preferences. Remembering all these details without a system is nearly impossible, yet forgetting them can make interactions feel impersonal.

The Knowledge Base addresses the fundamental challenge: **How do I show each person that I see and value them as an individual?**

### Remember the Details That Matter

| User Need | How Knowledge Base Helps |
|-----------|-------------------------|
| Remember family details | Store partner names, children's names and ages, pet information |
| Celebrate milestones | Track birthdays, anniversaries, and custom important dates |
| Understand their journey | Document previous roles, how they joined, career history |
| Respect preferences | Record communication style, work hours, interests |
| Capture context | Free-form notes for anything else that matters |

### Build Genuine Relationships

The goal is not surveillance or over-familiarity. It is having the context to be a genuinely supportive manager who:

- Follows up appropriately ("How was your daughter's graduation?")
- Respects boundaries ("I know you prefer async communication")
- Acknowledges tenure ("Congrats on three years with the team next week!")
- Shows interest in the whole person ("How's the marathon training going?")

This information is shared voluntarily by team members over time through natural conversation. The Knowledge Base simply helps you remember it.

---

## Information Categories

### Family Basics

Understanding an engineer's family situation helps managers be more empathetic and supportive.

| Field | Description | Example |
|-------|-------------|---------|
| Partner Name | Name of spouse/partner | "Sarah" |
| Kids | Children's names and ages | "Emma (8), James (5)" |
| Pets | Pet information | "Dog named Max, two cats" |
| Important Dates | Birthdays, anniversaries | "Birthday: March 15" |

**Use cases:**
- Ask how Sarah is doing when they mention her in conversation
- Remember that Emma started third grade this year
- Know they might need flexibility when the dog has a vet appointment

### Work History

Understanding someone's professional journey provides valuable context for career discussions and day-to-day collaboration.

| Field | Description | Example |
|-------|-------------|---------|
| Previous Roles | Past positions on the team | "Started as junior, promoted to senior in 2023" |
| Tenure | How long on the team | Auto-calculated from start date |
| How They Joined | Story of joining the team | "Referred by Alex, interviewed for backend role" |
| Previous Companies | Prior employment | "Google (3 years), Startup XYZ (2 years)" |

**Use cases:**
- Reference their growth when discussing career progression
- Acknowledge their anniversary approaching
- Draw on their prior experience in relevant discussions
- Understand their perspective based on professional background

### Preferences

Knowing how someone prefers to work enables more effective collaboration and shows respect for their individual style.

| Field | Description | Example |
|-------|-------------|---------|
| Communication Style | How they prefer to interact | "Prefers written over verbal, needs time to process" |
| Work Hours | Typical schedule | "9-5, strict boundaries, no weekend messages" |
| Interests/Hobbies | Personal interests | "Marathon runner, board games, sci-fi novels" |

**Use cases:**
- Send a detailed Slack message instead of calling them into a meeting
- Avoid scheduling outside their preferred hours
- Connect over shared interests or ask about their latest hobby project

---

## Functional Requirements

### FR-1: All Fields Optional

**Requirement:** Every field in the Knowledge Base must be optional with no required personal information.

**Rationale:** Different team members share different amounts of personal information, and that is completely appropriate. Some people are naturally more private, and managers must respect those boundaries. The system should work well whether an engineer has one field populated or twenty.

**Acceptance Criteria:**
- Engineer profiles can be created with zero personal info
- UI clearly indicates all fields are optional
- No warnings or prompts to "complete" the profile
- Profiles with minimal info display cleanly without empty field clutter

### FR-2: View Upcoming Important Dates

**Requirement:** The system must provide a consolidated view of upcoming important dates across all team members.

**Rationale:** Proactively knowing that someone's birthday or anniversary is approaching enables thoughtful gestures without last-minute scrambling.

**Acceptance Criteria:**
- Dashboard widget shows important dates in the next 30 days
- Dates are sorted chronologically
- Each date shows the engineer's name and date type
- Clicking a date navigates to the engineer's profile

### FR-3: Work Anniversary Auto-Calculation

**Requirement:** Work anniversaries must be automatically calculated from the engineer's start date.

**Rationale:** Start date is already captured for tenure tracking. Automatically deriving the anniversary eliminates duplicate data entry and ensures accuracy.

**Acceptance Criteria:**
- Anniversary date derived from `startDate` field
- Upcoming anniversaries appear in the important dates view
- Anniversary includes tenure milestone (e.g., "5 Year Anniversary")
- No manual entry required for work anniversary

### FR-4: Custom Recurring Dates

**Requirement:** Users must be able to add custom recurring dates beyond birthdays and anniversaries.

**Rationale:** Different cultures and individuals have various milestones worth tracking: wedding anniversaries, adoption dates, citizenship anniversaries, religious observances, or other personally significant dates.

**Acceptance Criteria:**
- Add custom dates with label, date, and recurring flag
- Recurring dates automatically roll to next year
- Non-recurring dates can be optionally kept or removed after passing
- Custom dates appear in the unified upcoming dates view

### FR-5: Free-Form Personal Notes

**Requirement:** Each engineer profile must include a free-form markdown notes field for unstructured personal context.

**Rationale:** Not everything fits neatly into structured fields. A notes section captures context like "Going through a difficult divorce - be sensitive" or "Mentioned wanting to learn Rust" or "Has anxiety about public speaking."

**Acceptance Criteria:**
- Markdown-formatted text area for personal notes
- Full markdown support (headers, lists, links, emphasis)
- Notes render as formatted markdown when viewing
- No character limit
- Notes are separate from meeting notes

---

## UI Components

### Profile Sections

The engineer profile page should organize Knowledge Base information into clear, collapsible sections:

```
+----------------------------------+
|  [Photo]  Engineer Name          |
|           Senior Engineer        |
|           3 years, 2 months      |
+----------------------------------+

+----------------------------------+
| Family                      [-]  |
+----------------------------------+
| Partner: Sarah                   |
| Kids: Emma (8), James (5)        |
| Pets: Dog named Max              |
+----------------------------------+

+----------------------------------+
| Important Dates             [-]  |
+----------------------------------+
| Birthday: March 15               |
| Work Anniversary: June 3 (5 yrs) |
| Wedding Anniversary: Sept 20     |
+----------------------------------+

+----------------------------------+
| Work History                [-]  |
+----------------------------------+
| Joined: Referred by Alex         |
| Previous: Jr. Engineer (2021)    |
| Prior: Google (3 yrs)            |
+----------------------------------+

+----------------------------------+
| Preferences                 [-]  |
+----------------------------------+
| Communication: Written, async    |
| Hours: 9-5, no weekends          |
| Interests: Running, board games  |
+----------------------------------+

+----------------------------------+
| Personal Notes              [-]  |
+----------------------------------+
| - Training for first marathon    |
| - Mentioned interest in Rust     |
| - Prefers detailed feedback      |
+----------------------------------+
```

**Design considerations:**
- Sections collapse/expand to reduce visual clutter
- Empty sections are hidden entirely (not shown as empty)
- Edit mode inline or via dedicated edit view
- Consistent with overall app design language

### Date Reminders Widget

A dashboard component showing upcoming important dates:

```
+----------------------------------+
| Upcoming Dates                   |
+----------------------------------+
| TODAY                            |
|   [avatar] Alex - Birthday       |
+----------------------------------+
| THIS WEEK                        |
|   Jan 22 - Jordan - 3yr Anniv    |
|   Jan 24 - Sam - Wedding Anniv   |
+----------------------------------+
| THIS MONTH                       |
|   Feb 3 - Taylor - Birthday      |
|   Feb 15 - Morgan - 1yr Anniv    |
+----------------------------------+
```

**Design considerations:**
- Visual hierarchy emphasizes today and this week
- Avatars for quick recognition
- Clicking navigates to engineer profile
- Configurable look-ahead window (7, 14, 30 days)

---

## Data Requirements

### PersonalInfo Entity Reference

The Knowledge Base data is stored within the `PersonalInfo` interface embedded in each `Engineer` entity. See the [Data Model](/docs/data-model.md) for complete specifications.

```typescript
interface PersonalInfo {
  // Family
  partnerName?: string;
  hasChildren: boolean;
  childrenNames?: string[];
  petInfo?: string;

  // Important Dates
  birthday?: Date;
  workAnniversary?: Date;        // Derived from startDate
  otherDates?: ImportantDate[];

  // Background
  hometown?: string;
  education?: string;
  previousCompanies?: string[];

  // Notes
  personalNotes?: string;        // Free-form markdown
}

interface ImportantDate {
  date: Date;
  label: string;
  recurring: boolean;
}
```

### Additional Fields in Engineer Entity

The following fields in the `Engineer` entity also contribute to the Knowledge Base:

```typescript
// From Engineer entity
communicationStyle?: string;
workHours?: string;
interests?: string[];
startDate: Date;              // Source for work anniversary
previousRoles?: string[];
howTheyJoined?: string;
```

### Computed Properties

```typescript
// Derived fields (not stored)
tenureMonths: number;           // From startDate
upcomingAnniversary: Date;      // Next occurrence of startDate
upcomingBirthday: Date;         // Next occurrence of birthday
```

---

## Privacy Considerations

### Sensitive Data Classification

The Knowledge Base contains sensitive personal information that requires careful handling:

| Data Type | Sensitivity | Considerations |
|-----------|-------------|----------------|
| Family information | High | Names of partners and children |
| Birthdates | Medium | Can be used for identity verification |
| Personal notes | High | May contain health, relationship, or other private matters |
| Work history | Low | Generally professional information |
| Preferences | Low | Work-related preferences |

### Local-Only Storage

**All Knowledge Base data is stored locally on the manager's device.** This is a fundamental architectural decision that provides strong privacy protection:

- No cloud synchronization
- No server-side storage
- No third-party access
- Data remains under the manager's direct control

This local-only approach means:
- Data cannot be accessed by the organization
- No risk of data breach from server compromise
- Complete manager ownership of the information
- Compliance with personal data handling best practices

### Data Handling Guidelines

Managers should follow these principles when using the Knowledge Base:

1. **Voluntary disclosure only** - Only record information that team members voluntarily share
2. **Professional purpose** - Use information to be a better manager, not for other purposes
3. **Discretion** - Do not share personal details with others without permission
4. **Accuracy** - Keep information current; remove outdated or incorrect data
5. **Minimal collection** - Only record what is useful for the manager-report relationship

### Offboarding

When a team member leaves:
- Consider what information to retain
- Archive rather than delete if needed for reference
- Respect their privacy even after departure
- Use the `isActive` flag rather than deleting records

---

## Summary

The Knowledge Base feature embodies Vibe Manager's core philosophy: **great management is about seeing people as individuals, not just resources.** By providing a structured yet flexible way to remember personal details, work history, and preferences, the Knowledge Base helps engineering managers build genuine relationships with their team members.

Key principles:
- **Privacy-respecting** - All fields optional, local-only storage
- **Relationship-focused** - Information that helps managers connect authentically
- **Practical** - Date reminders and quick reference enable timely, thoughtful interactions
- **Flexible** - Structured fields plus free-form notes accommodate any information

When used thoughtfully, the Knowledge Base transforms a manager from someone who just tracks work to someone who genuinely knows and cares about their team.
