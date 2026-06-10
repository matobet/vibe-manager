# Research Analysis: What Makes a Great Manager of Software Engineers?

**Paper**: Kalliamvakou, E., Bird, C., Zimmermann, T., Begel, A., DeLine, R., & German, D.M. (2018). "What Makes a Great Manager of Software Engineers?" *IEEE Transactions on Software Engineering*, 44(10).

**Study**: Mixed-methods empirical study at Microsoft — 37 semi-structured interviews + survey of 3,646 engineers and managers (563 responses). Compared software engineering to 5 other knowledge-work disciplines (1,082 additional responses).

**Purpose of this analysis**: Extract insights from the research that can inform Vibe Manager's feature development, data model, and UX design.

---

## The 15 Attributes Framework

The study identified 15 attributes of great engineering managers, organized into three functions and two interaction levels:

### Manager Functions

| Function | Individual-Level Attributes | Team-Level Attributes |
|----------|---------------------------|----------------------|
| **Cultivates** | Enables autonomy, Supports experimentation, Grows talent | Builds team culture, Guides the team |
| **Motivates** | Promotes fairness, Builds relationship with team members, Recognizes individuality | Maintains positive working environment, Inspires the team |
| **Mediates** | Clears path to execution | Facilitates external communication, Drives alignment |

Two cross-cutting attributes span all functions: **Is available** and **Is technical**.

### Attribute Ranking by Importance (Survey Results)

| Rank | Attribute | Mean Score (1-10) |
|------|-----------|-------------------|
| 1 | Maintains positive working environment | 9.05 |
| 2 | Grows talent | 8.98 |
| 3 | Enables autonomy | 8.91 |
| 4 | Promotes fairness | — |
| 5 | Recognizes individuality | — |
| 6 | Inspires the team | — |
| 7 | Supports experimentation | — |
| 8 | Clears path to execution | 8.30 |
| 9 | Drives alignment | 8.33 |
| 10 | Builds team culture | 8.13 |
| 11 | Guides the team | 8.19 |
| 12 | Is available | — |
| 13 | Facilitates external communication | 7.86 |
| 14 | Is technical | 7.84 |
| 15 | Builds relationship with team members | 7.47 |

All 15 attributes scored above 7.4/10, meaning none are unimportant — they differ in degree.

---

## Key Findings Relevant to Vibe Manager

### 1. Technical Skills Are Not the Sign of Greatness

The paper's most striking finding: **75% of respondents** would hire a manager with average technical skills and excellent social skills over the reverse. "Being technical" ranked 14th of 15 attributes. Technical knowledge is necessary but it's a baseline, not a differentiator.

> *"Even though he is not technically 100% great, this is something which he/she can learn fast. Inspiring others is not something you can learn overnight and this skill is precious."*

**Implication for Vibe Manager**: The app's focus on people tracking (mood, meetings, career growth) rather than technical metrics is well-aligned with the research. The app should continue to avoid technical performance tracking (code metrics, velocity, etc.) and double down on relationship and growth-oriented features.

### 2. The Top Three: Environment, Growth, Autonomy

The highest-rated attributes are:
- **Positive working environment** (9.05) — flexibility, work-life balance, celebrating successes, team morale
- **Grows talent** (8.98) — challenging work opportunities, actionable feedback, on-the-job learning
- **Enables autonomy** (8.91) — freedom in how engineers work, trust, involvement in decision-making

**Implication for Vibe Manager**: The app's Phase 4 (Career Development) directly supports "grows talent." The mood tracking in Phase 2 serves as a proxy for "positive working environment." The app currently has no features supporting autonomy tracking — this could be a gap worth addressing.

### 3. Managers and Engineers See Things Differently

Managers rate certain attributes significantly higher than engineers:
- Builds relationship with team members: **+1.21 points**
- Inspires the team: **+0.98**
- Builds team culture: **+0.97**
- Is available: **+0.60**
- Grows talent: **+0.54**

**Implication for Vibe Manager**: Since the app's user is the manager, its feature set naturally reflects the manager's perspective. The research validates that managers care deeply about relationship-building and team culture — features that support tracking these dimensions will resonate with the target user.

### 4. "Recognizes Individuality" Is Critical

Great managers understand each engineer's unique strengths, weaknesses, interests, and definition of success. This was linked to both productivity and satisfaction:

> *"I had a manager asking me what I was interested in and giving me work related to that and I felt a lot more comfortable and happier. I had a manager try to mold me to their definition of what a good engineer does and I was probably working the hardest and yet my output was probably the least."*

**Implication for Vibe Manager**: This strongly validates Phase 3 (Knowledge Base) and Phase 4 (Career Development). The app's per-person profile with skills, interests, and personal details directly enables "recognizing individuality." Consider expanding the profile to include: communication preferences, working style, intrinsic motivators, and what "success" means to each person.

### 5. 1-on-1 Meetings as Relationship Infrastructure

The paper describes 1-on-1s as the primary venue for several attributes — building relationships, growing talent, providing feedback, and recognizing individuality:

> *"Having 1-1 meetings in their office is better, it gives them home field advantage so that they don't feel like they are going to the principal's office. We might talk about life and things at work, it goes back and forth."*

**Implication for Vibe Manager**: The app's core 1-on-1 tracking is validated as the right foundation. The meeting note format could be enhanced with structured prompts aligned to the framework's attributes (see Feature Suggestions below).

### 6. Feedback Must Be Timely and Actionable

Managers described postponing negative feedback as a property of *bad* managers. Growing talent requires regular, specific feedback:

> *"Managers commented that giving negative feedback is a tough process but that postponing giving negative feedback is a property of bad managers; the delay only leaves less time to the engineer to address their performance issues."*

**Implication for Vibe Manager**: Phase 5's meeting prep and note templates could include feedback tracking — what feedback was given, what was the follow-up? Action item tracking (currently a Phase 2 deferred item) becomes more important in this context.

### 7. Clearing Path to Execution / Protecting Flow

Engineers place high importance on managers shielding them from interruptions and "randomization" (changing requirements):

> *"The operational space for engineers is that flow moment where they are deep into writing code. The last thing they need is some random person going 'hey, have you got a second?', you just killed half their day."*

**Implication for Vibe Manager**: This is an attribute the app doesn't currently track. A manager could benefit from noting blockers they've cleared for each report, or tracking sources of randomization affecting the team.

### 8. Safe Experimentation and Psychological Safety

Supporting experimentation requires signaling safety. Both Google's Project Aristotle and this study emphasize psychological safety as foundational:

> *"Nobody wants to report failures. Then it's less stressful to do what other people do, rather than try something new. It has to be somehow communicated that we will let you try stuff with the assumption that if it doesn't work it's fine."*

**Implication for Vibe Manager**: Mood tracking partially captures this — declining mood could signal a psychologically unsafe environment. But the app could go further by letting managers note experimentation/risk-taking they've encouraged, and outcomes.

### 9. Manager-to-Manager Differences Across Demographics

Notable demographic findings:
- **Female respondents** rated "is technical" significantly higher (+0.83) — they seek to learn from technically stronger managers
- **Respondents from China** also valued "is technical" more (+1.02)
- **Respondents from India** valued "builds team culture" more (+0.99) and "builds relationships" more (+0.85)
- **Larger teams** reduced importance of "clears path to execution" and "enables autonomy" per person

**Implication for Vibe Manager**: For managers of diverse, distributed teams, the app could support tracking cultural context and adapting management approach per person. The "recognizes individuality" attribute extends to recognizing cultural differences in what team members value from their manager.

---

## Feature Suggestions Derived from Research

### High Priority (Directly Validated)

#### 1. Meeting Note Templates Aligned to Framework
Structured prompts for 1-on-1 notes based on the three manager functions:

- **Cultivate**: What growth opportunities discussed? Any skill development feedback given? Autonomy check — are they blocked or micromanaged?
- **Motivate**: How's their energy/morale? Recognition given? Fairness concerns?
- **Mediate**: Blockers I need to clear? Alignment with team/org goals clear? External dependencies?

This would be a natural fit for Phase 5 (Smart Features / Note Templates).

#### 2. Growth Tracking (Enhance Phase 4)
The #2 most important attribute. Beyond skills and levels, track:
- Development goals and progress
- Challenging assignments given and outcomes
- Feedback given (with follow-up dates)
- Learning opportunities provided (conferences, courses, stretch projects)

#### 3. Blocker/Path-Clearing Log
Track what the manager has done to shield and unblock each report:
- Blockers identified and resolved
- Randomization deflected
- External negotiations on team's behalf

This makes the manager's invisible work visible and trackable.

#### 4. Personal Context Expansion (Enhance Phase 3)
Expand the knowledge base to better support "recognizes individuality":
- Communication preferences (async vs sync, written vs verbal)
- Working style (morning person, deep work blocks, collaboration preference)
- Intrinsic motivators (impact, learning, autonomy, mastery, belonging)
- Definition of success (what does "great" look like for this person?)

### Medium Priority (Research-Supported)

#### 5. Team Health Dashboard Enhancements
The study's framework maps well to a richer team health model beyond mood:
- **Morale** (current mood tracking)
- **Growth** (are they being challenged? stagnation risk?)
- **Autonomy** (are they empowered or blocked?)
- **Alignment** (do they understand and buy into the mission?)

This could evolve the dashboard from a mood-centric view to a holistic team health view.

#### 6. Relationship Strength Indicator
Managers rated "builds relationship" as far more important than engineers did (+1.21). The app could compute relationship indicators:
- Meeting consistency (regular vs sporadic)
- Note depth (are meetings substantive or perfunctory?)
- Personal details captured (does the manager know about their life outside work?)

#### 7. Fairness and Recognition Tracking
"Promotes fairness" ranked #4 and includes "praise publicly, correct privately." Track:
- Public recognition given (shout-outs, meeting presentations)
- Credit attribution (who presented their own work?)
- Distribution of opportunities across the team (are stretch projects going to the same people?)

### Lower Priority (Interesting but Speculative)

#### 8. Manager Self-Assessment
A periodic self-reflection prompt based on the 15 attributes. The manager rates themselves or reflects on how they're doing in each area. Not tracked over time in a quantitative way — just a journaling prompt.

#### 9. Culture Notes (Team-Level)
A team-level journal for documenting team norms, values, and cultural decisions. The paper found "builds team culture" important but undervalued in software engineering relative to other disciplines.

---

## Alignment with Current Roadmap

| Roadmap Phase | Research Alignment | Gap |
|---|---|---|
| **Phase 1: MVP** (done) | Core 1-on-1 tracking validated as the right foundation | None |
| **Phase 2: Mood** (done) | Supports "positive working environment" attribute | Could add psychological safety signals |
| **Phase 3: Knowledge Base** (partial) | Directly supports "recognizes individuality" | Expand beyond personal facts to motivators, work style, communication preferences |
| **Phase 4: Career Development** (partial) | Directly supports "grows talent" (#2 attribute) | Add feedback tracking, opportunity distribution, growth trajectory |
| **Phase 5: Smart Features** (planned) | Meeting prep + templates align with framework | Templates should be structured around the three manager functions |
| **Managing Managers** (done) | Team health metrics map to team-level attributes | Could add team culture health, alignment tracking |

---

## Key Takeaway

The research validates Vibe Manager's core thesis: **the most important things a manager does are about people, not technology.** The app is well-positioned because it focuses on relationships, mood, and career growth rather than engineering metrics.

The biggest opportunity is to move from **passive tracking** (recording what happened) to **active scaffolding** (prompting the manager to exercise specific attributes). The 15-attribute framework provides a research-backed structure for guiding what managers should pay attention to in every interaction with their reports.

---

## Citation

Kalliamvakou, E., Bird, C., Zimmermann, T., Begel, A., DeLine, R., & German, D.M. (2018). What Makes a Great Manager of Software Engineers? *IEEE Transactions on Software Engineering*, 44(10), 981-999. DOI: [10.1109/TSE.2017.2768368](https://doi.org/10.1109/TSE.2017.2768368)
