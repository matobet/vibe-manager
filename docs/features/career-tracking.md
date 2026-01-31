# Career Tracking Feature Specification

## Implementation Status

| Requirement | Status | Notes |
|-------------|--------|-------|
| 4.1: Set Current Career Level | ✅ Implemented | P1-P5 level in profile (`level` field) |
| 4.2: Track Skill Proficiency | 📋 Planned | Data model exists (`skills` array) |
| 4.3: Update Assessments with Evidence | 📋 Planned | Not yet implemented |
| 4.4: View Progress Over Time | 📋 Planned | Not yet implemented |
| 4.5: Set Development Goals | 📋 Planned | Not yet implemented |
| 4.6: Time in Level Tracking | 📋 Planned | Start date exists, UI not implemented |
| Level Badge Display | ✅ Implemented | P1-P5 shown in dashboard cards |
| Skill Matrix UI | 📋 Planned | Full character sheet view not implemented |

---

## 1. Feature Overview

Career Tracking enables engineering managers to monitor and support the professional development of their direct reports. The feature provides a structured way to track where each engineer stands on the R&D Career Path, assess proficiency across key skills, and identify growth opportunities.

This is a personal tracking tool for the manager's own use - not a formal HR performance review system. It helps maintain awareness of each team member's development journey and ensures career conversations remain grounded in concrete observations.

### Key Capabilities

- Track current career level for each engineer (P1-P5)
- Assess proficiency across skills within each career pillar
- Record evidence and notes supporting assessments
- Visualize progress and identify development areas
- Set and track development goals
- Monitor time spent at each level

### Design Aesthetic: RPG Character Progression

Career tracking maps naturally to RPG character development:

| Career Concept | RPG Equivalent |
|----------------|----------------|
| Career level (P1-P5) | Character level (LV.1-5) |
| Skills matrix | Character stats (STR, INT, WIS, CHA) |
| Proficiency levels | Stat values / skill points |
| Skill pillars | Stat categories |
| Level promotion | Level up! |
| Time in level | XP progress bar |
| Development goals | Skills to train |

**Visual Treatment:**
- Levels displayed as `★ LV.3` with XP-style progress to next level
- Skills shown as stat bars: `CODE [████████░░] Advanced`
- Pillars styled as stat categories (like STR/INT/WIS/CHA)
- Level-up celebrations with retro flair
- Character sheet layout for engineer profile view

---

## 2. User Needs

### Primary Needs

| Need | Description |
|------|-------------|
| **Visibility into development** | Understand where each engineer stands on their career path without relying on memory or scattered notes |
| **Promotion readiness assessment** | Quickly evaluate if an engineer is ready for the next level by seeing which skills meet expectations and which need development |
| **Structured career conversations** | Have data-driven 1-on-1 discussions about growth rather than vague feedback |
| **Development planning** | Identify specific skills to focus on and create actionable development plans |
| **Historical tracking** | See how engineers have grown over time and celebrate progress |

### Secondary Needs

- Prepare for formal performance review cycles with readily available evidence
- Identify patterns across the team (common skill gaps, training needs)
- Track time in level to anticipate promotion timelines

---

## 3. Career Path Structure

### 3.1 Career Levels

The R&D Career Path defines five levels with distinct expectations and focus areas:

| Level | Name | Focus | Description |
|-------|------|-------|-------------|
| **P1** | Entry | Work execution | Delivers with guidance, actively seeks mentorship, learning fundamentals |
| **P2** | Developing | Problem solving | Works somewhat independently, limited direction needed, becoming familiar with processes |
| **P3** | Proficiency | Product building | Independently identifies and delivers solutions, understands department strategy |
| **P4** | Mastery | Leadership | Delivers ongoing business impact autonomously, takes accountability, often leads |
| **P5** | Expert | Technical strategy | Delivers against strategic company goals, high technical expertise, organization-wide impact |

### 3.2 Career Pillars

Skills are organized into four pillars aligned with company values:

| Pillar | Value | Focus Areas |
|--------|-------|-------------|
| **Technical Skills** | We Challenge | Code quality, architecture, security, testing & observability |
| **Delivery** | We Commit | Planning, prioritization, ownership, execution, process improvement |
| **Collaboration** | We Collaborate | Communication, teamwork, stakeholder relationships |
| **Leadership** | We Care | Mentoring, knowledge sharing, product & business thinking |

### 3.3 Skills Matrix

#### Technical Skills (We Challenge)

| Skill | P1 | P2 | P3 | P4 | P5 |
|-------|----|----|----|----|-----|
| **Code** | Writes testable code with support | Writes code others understand easily | Consistently accounts for edge cases and errors | (see P3) | (see P3) |
| **Architecture** | Aware of service architecture | Contributes aligned designs | Designs with clear interfaces/abstractions | Architects scalable systems | Guides architecture across teams |
| **Security & Privacy** | Understands importance | Asks for help on security decisions | Consistently applies security lens | Refines team's security approach | Sets org-wide security strategy |
| **Observability & Testing** | Writes unit tests with help | Understands testing pyramid | Writes tests for edge cases, tunes monitoring | Works with QA on testing solutions | Fosters observability culture across teams |

#### Delivery (We Commit)

| Skill | P1 | P2 | P3 | P4 | P5 |
|-------|----|----|----|----|-----|
| **Planning & Prioritization** | Understands task breakdown, follows priorities | Sizes tasks for incremental delivery | Reviews epics critically, ensures correct prioritization | Fosters priority-setting culture in team | Manages cross-team prioritization |
| **Ownership & Execution** | Daily progress updates, executes with support | Commits realistically, escalates blockers | Sets expectations proactively, anticipates blockers | Sets expectations with external stakeholders | Leads organization-wide commitments |
| **Process Thinking** | Understands team practices | Discusses improvements with team | Often improves team practices | Drives process implementation | Sets organizational practices |

#### Collaboration (We Collaborate)

| Skill | P1 | P2 | P3 | P4 | P5 |
|-------|----|----|----|----|-----|
| **Effective Communication** | Shares progress daily, contributes respectfully | Communicates effectively, listens actively | Communicates clearly within and outside team | Communicates across diverse teams | Communicates effectively across company |
| **Teamwork** | Helps when requested | Helps overcome obstacles when asked | Actively helps teammates, addresses morale issues | Consistently helps, fosters sharing culture | Enables teams across organization |
| **Working with Stakeholders** | Gathers context from product | Refines tasks with product | Actively seeks feedback, provides technical context | Builds relationships across engineering & product | Leverages relationships for org positioning |

#### Leadership (We Care)

| Skill | P1 | P2 | P3 | P4 | P5 |
|-------|----|----|----|----|-----|
| **Leadership** | n/a | n/a | Facilitates discussions, mentors juniors | Mentors openly, provides technical mentorship | Fosters mentoring culture, may have direct reports |
| **Knowledge Sharing** | Seeks mentorship | Seeks mentorship, helps new joiners | Shares knowledge frequently, contributes to docs | Fosters documentation culture within team | Fosters knowledge sharing across org |
| **Product & Business Thinking** | Understands basic product utility | Basic understanding of domain and business | Thorough understanding, simplifies designs | Evaluates and creates features with product | Recognizes opportunities, refines roadmaps |

### 3.4 Proficiency Levels

Each skill is assessed using a five-level proficiency scale:

| Proficiency | Description | Indicator |
|-------------|-------------|-----------|
| **Learning** | Just starting to develop this skill; needs significant guidance | Requires hands-on support |
| **Developing** | Making progress; can apply skill with some oversight | Shows growth, occasional gaps |
| **Proficient** | Meets expectations for current level; reliable execution | Consistent, dependable |
| **Advanced** | Exceeds expectations; demonstrates skill beyond current level | Ready for stretch assignments |
| **Expert** | Role model for this skill; could teach others | Mentors others in this area |

---

## 4. Functional Requirements

### 4.1 Set Current Career Level

- Assign a career level (P1-P5) to each engineer
- Optionally set a target level they are working toward
- Record the date when they entered the current level
- View time spent in current level (auto-calculated)

### 4.2 Track Skill Proficiency

- For each engineer, assess proficiency in specific skills within each pillar
- Default view shows skills relevant to current and target levels
- Allow assessment of any skill regardless of level
- Support bulk updates when doing comprehensive reviews

### 4.3 Update Assessments with Evidence

- Add notes/evidence when updating a proficiency assessment
- Notes support markdown formatting for flexibility
- Record timestamp of each assessment update
- Maintain history of previous assessments (not just current state)

### 4.4 View Progress Over Time

- Timeline view showing assessment changes
- Compare current state to previous assessment dates
- Highlight skills that have improved
- Show skills that may have regressed or stagnated

### 4.5 Set Development Goals

- Mark specific skills as "focus areas" for development
- Add notes on development approach/plan for each focus area
- Set optional target dates for achieving proficiency improvements
- Track progress on development goals over time

### 4.6 Time in Level Tracking

- Automatically calculate months in current level from level start date
- Display time-in-level on engineer profile and dashboard
- Optional: set expected time ranges for each level to highlight when someone may be ready for promotion discussion

---

## 5. UI Components

### 5.1 Character Sheet View

The primary interface for viewing and editing career progress, styled as an RPG character sheet.

```
╔══════════════════════════════════════════════════════════════════════╗
║  ┌─────┐                                                             ║
║  │▓▓▓▓▓│  ALEX CHEN                                                  ║
║  │▓▓▓▓▓│  Software Engineer                                          ║
║  └─────┘                                                             ║
║  ★ LEVEL 3 - Proficiency                  XP: ████████████░░░░ 72%  ║
║  Party member for 2y 3m                   Time at LV.3: 14 months    ║
╠══════════════════════════════════════════════════════════════════════╣
║                        ~ CHARACTER STATS ~                           ║
╠══════════════════════════════════════════════════════════════════════╣
║  ⚔ TECHNICAL (We Challenge)                                         ║
║    CODE       [████████░░] Proficient     ← meets LV.3              ║
║    ARCH       [██████████] Advanced       ★ exceeds!                ║
║    SECURITY   [████░░░░░░] Developing     ⚠ focus area              ║
║    TESTING    [████████░░] Proficient     ← meets LV.3              ║
║                                                                      ║
║  📦 DELIVERY (We Commit)                                            ║
║    PLANNING   [████████░░] Proficient     ← meets LV.3              ║
║    OWNERSHIP  [██████████] Advanced       ★ exceeds!                ║
║    EXECUTION  [████████░░] Proficient     ← meets LV.3              ║
║    PROCESS    [████████░░] Proficient     ← meets LV.3              ║
║                                                                      ║
║  🤝 COLLABORATION (We Collaborate)  │  👑 LEADERSHIP (We Care)      ║
║    COMMS      [████████░░] Prof.    │    MENTORING  [██████░░] Dev. ║
║    TEAMWORK   [████████░░] Prof.    │    KNOWLEDGE  [████████] Prof.║
║    STAKEHLDRS [██████░░░░] Dev.     │    PRODUCT    [████░░░░] Dev. ║
╠══════════════════════════════════════════════════════════════════════╣
║  🎯 TRAINING FOCUS: Security, Stakeholders, Mentoring               ║
╚══════════════════════════════════════════════════════════════════════╝
```

Features:
- Click stat bar to adjust proficiency level
- Hover for evidence/notes on each skill
- Visual highlighting for training focus areas
- Star (★) indicator for skills exceeding level expectations
- Warning (⚠) indicator for skills below expectations

### 5.2 Level Progress

**XP Bar to Next Level:**
```
 ★ LV.3  ████████████████████░░░░░░░░  72%  → LV.4
         ↑ skills at P3+ level
```

**Pillar Summary - Stat Category View:**
```
 ╔═══════════════════════════════════════════════════════════════╗
 ║  STAT SUMMARY                                                  ║
 ╠═══════════════════════════════════════════════════════════════╣
 ║  ⚔ TECHNICAL   [████████░░] 78%    📦 DELIVERY   [██████████] 85%  ║
 ║  🤝 COLLAB     [██████░░░░] 65%    👑 LEADERSHIP [████░░░░░░] 45%  ║
 ╚═══════════════════════════════════════════════════════════════╝
      ↑ Weakest - prioritize        Strongest ↑
```

### 5.3 Assessment History

- Timeline of proficiency changes per skill
- Expandable to show notes/evidence from each assessment
- Filter by pillar or skill

### 5.4 Development Goals Panel

```
+------------------------------------------------------------------+
|  DEVELOPMENT FOCUS AREAS                                          |
+------------------------------------------------------------------+
|  [ ] Security & Privacy                                           |
|      Current: Developing -> Target: Proficient                    |
|      Plan: Complete security training, lead threat modeling       |
|      Target date: Q2 2026                                         |
|                                                                    |
|  [ ] Working with Stakeholders                                    |
|      Current: Proficient -> Target: Advanced                      |
|      Plan: Lead feature spec discussions with Product             |
|      Target date: Q3 2026                                         |
+------------------------------------------------------------------+
```

---

## 6. Data Requirements

### 6.1 CareerProgress Entity

Reference the `CareerProgress` entity defined in the data model:

```typescript
interface CareerProgress {
  id: string;
  engineerId: string;            // Reference to Engineer

  // Career Path Reference
  pillar: CareerPillar;          // technical_skills | delivery | collaboration | leadership
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

### 6.2 Additional Data Needs

**Engineer Entity Extensions** (already defined):
- `currentLevel: CareerLevel` - Current career path level (P1-P5)
- `targetLevel?: CareerLevel` - Target level working toward

**New: Level History** (for time-in-level tracking):
```typescript
interface LevelChange {
  id: string;
  engineerId: string;
  fromLevel: CareerLevel | null;  // null for initial assignment
  toLevel: CareerLevel;
  changedAt: Date;
  notes?: string;
}
```

**New: Development Goal**:
```typescript
interface DevelopmentGoal {
  id: string;
  engineerId: string;
  careerProgressId: string;       // Links to specific skill
  targetProficiency: ProficiencyLevel;
  developmentPlan?: string;       // Markdown notes on approach
  targetDate?: Date;
  status: 'active' | 'achieved' | 'deferred';
  createdAt: Date;
  updatedAt: Date;
}
```

### 6.3 Reference Data

Career path definition is stored as reference data:
- Pillars and their skills
- Level expectations per skill
- Not editable through the app (defined in R&D Career Path)

---

## 7. Integration with 1-on-1s

Career tracking integrates naturally with 1-on-1 meetings to support ongoing development conversations.

### 7.1 Career Discussion Prompts

When preparing for a 1-on-1, the system can surface:
- Skills marked as focus areas
- Recent assessment changes
- Time in current level
- Upcoming development goal target dates

### 7.2 Meeting Note Templates

Include optional career discussion section in meeting notes:

```markdown
## Career Development

### Focus Areas Discussed
- [ ] Security training progress
- [ ] Stakeholder communication feedback

### Observations
(Notes on demonstrated skills, growth, or areas needing attention)

### Actions
- [ ] Schedule security training review
- [ ] Shadow product sync meeting next sprint
```

### 7.3 Linking Evidence to Meetings

- Tag specific skill observations in meeting notes
- Auto-suggest updating proficiency after noting relevant evidence
- View meeting notes that mention specific skills when reviewing assessments

### 7.4 Career Conversation Meeting Frequency

Suggest periodic career-focused 1-on-1s:
- Monthly: Quick check on development goal progress
- Quarterly: Comprehensive skill assessment review
- Semi-annually: Level progression discussion

---

## Implementation Notes

### MVP Scope
1. Basic skill matrix view with proficiency tracking
2. Notes/evidence per skill
3. Current level assignment
4. Time-in-level calculation

### Future Enhancements
- Assessment history timeline
- Development goals with target dates
- Meeting note integration
- Progress visualization (charts)
- Team-wide skill gap analysis
