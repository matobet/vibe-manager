# Vibe Manager - Product Roadmap

## Overview

This roadmap outlines the phased development approach for Vibe Manager, from MVP to full-featured personal engineering management tool.

## Guiding Principles

1. **Ship early, iterate often** - Get a working tool in use quickly
2. **Solve one problem well** - Each phase focuses on core pain points
3. **Privacy first** - Local storage from day one
4. **Simplicity over features** - Avoid feature creep

---

## Phase 1: MVP Foundation ✅ COMPLETE

**Focus**: Core data management and 1-on-1 tracking

### Goals
- Solve the primary pain point: "I keep forgetting to have 1-on-1s"
- Establish data model and storage
- Create usable interface for daily management

### Features

| Feature | Description | Priority | Status |
|---------|-------------|----------|--------|
| Engineer profiles | Create/edit/archive team members | P0 | ✅ Done |
| 1-on-1 frequency | Set target meeting frequency per engineer | P0 | ✅ Done |
| Overdue tracking | Visual indicators for missed meetings | P0 | ✅ Done |
| Record meetings | Mark 1-on-1s as complete with date | P0 | ✅ Done |
| Team dashboard | Overview of all engineers | P0 | ✅ Done |
| Basic notes | Markdown notes per meeting | P0 | ✅ Done |
| Local storage | JSON/markdown file persistence | P0 | ✅ Done |

### Technical Foundation
- **Terminal User Interface (TUI)** - Rich interactive terminal app
- **Local filesystem storage** - JSON for data, markdown for notes
- **Keyboard-driven navigation** - Vim-style keybindings
- **Single binary** - Easy install, no dependencies
- **8-bit RPG aesthetic** - Box-drawing, ANSI colors, status bars

### Success Criteria
- Can add all team members
- Dashboard shows who needs a 1-on-1
- Can record meetings and view history
- Data persists in workspace folder

---

## Phase 2: Enhanced Tracking 🔄 PARTIAL

**Focus**: Mood tracking and improved note management

### Goals
- Add health/happiness monitoring
- Make notes more useful with history and search
- Improve dashboard with mood indicators

### Features

| Feature | Description | Priority | Status |
|---------|-------------|----------|--------|
| Mood recording | 1-5 scale observations | P0 | ✅ Done |
| Mood history | View trends per engineer | P0 | ✅ Done |
| Mood on dashboard | Trend indicators in overview | P0 | ✅ Done |
| Note history | View all notes per engineer | P0 | ✅ Done |
| Note search | Find across all notes | P1 | 📋 Planned |
| Action items | Track follow-ups from notes | P1 | 📋 Planned |
| Reschedule/skip | Handle meeting changes | P1 | 📋 Planned |

### Success Criteria
- Can record and view mood trends
- Dashboard shows mood alerts
- Can search through historical notes
- Action items can be tracked

---

## Phase 3: Knowledge Base 🔄 PARTIAL

**Focus**: Personal information and relationship building

### Goals
- Remember important personal details
- Never miss birthdays or anniversaries
- Store preferences and background info

### Features

| Feature | Description | Priority | Status |
|---------|-------------|----------|--------|
| Personal info | Family, pets, important dates | P0 | 🔄 Partial (partner/children in profile) |
| Upcoming dates | Birthday/anniversary reminders | P0 | 📋 Planned |
| Work history | Previous roles, how they joined | P1 | 📋 Planned |
| Preferences | Communication style, work hours | P1 | 📋 Planned |
| Free-form notes | General personal notes | P1 | 📋 Planned |

### Success Criteria
- Can store personal details for each engineer
- Dashboard shows upcoming important dates
- Information easily accessible before 1-on-1s

---

## Phase 4: Career Development 🔄 PARTIAL

**Focus**: Career path tracking and progress visibility

### Goals
- Integrate with R&D Career Path framework
- Track skill development over time
- Support career conversations

### Features

| Feature | Description | Priority | Status |
|---------|-------------|----------|--------|
| Career levels | Set P1-P5 level per engineer | P0 | ✅ Done (stored in profile) |
| Skill matrix | View skills by pillar | P0 | 📋 Planned (data model exists) |
| Proficiency tracking | Record skill levels | P0 | 📋 Planned |
| Assessment history | Track changes over time | P1 | 📋 Planned |
| Development goals | Mark focus areas | P1 | 📋 Planned |
| Progression summary | Time in level, progress | P1 | 📋 Planned |

### Success Criteria
- Can view career progress for each engineer
- Skills and proficiency levels tracked
- Ready for promotion discussions with data

---

## Phase 5: Smart Features 📋 PLANNED

**Focus**: Intelligent suggestions and automation

### Goals
- Reduce cognitive load for the manager
- Surface insights automatically
- Suggest optimal meeting frequency

### Features

| Feature | Description | Priority | Status |
|---------|-------------|----------|--------|
| Smart frequency | Suggest meeting frequency based on seniority/challenges | P1 | 📋 Planned |
| Weekly summary | Auto-generated weekly report | P1 | 📋 Planned |
| Pattern detection | Alert on mood trends | P1 | 📋 Planned |
| Meeting prep | Surface relevant context | P2 | 📋 Planned |
| Templates | Note templates for common scenarios | P2 | 📋 Planned |

### Success Criteria
- Receives useful suggestions for meeting frequency
- Weekly summary helps with reflection
- App proactively surfaces concerns

---

## Future Considerations

These items are out of scope for initial phases but may be considered later:

### Integrations
| Integration | Description | Complexity |
|-------------|-------------|------------|
| Google Calendar | Sync 1-on-1 scheduling | Medium |
| Outlook Calendar | Alternative calendar sync | Medium |
| Slack reminders | Push notifications | Low |

### TUI Enhancements
| Feature | Description | Complexity |
|---------|-------------|------------|
| Custom themes | User-configurable color schemes | Low |
| Mouse support | Optional mouse interaction | Low |
| Configurable keybindings | User-defined shortcuts | Medium |
| tmux integration | Status line integration | Low |

### Advanced Features
| Feature | Description | Complexity |
|---------|-------------|------------|
| Multi-device sync | Sync across devices | High |
| Encryption | Encrypt local data | Medium |
| Team sharing | Share with co-managers | High |
| AI insights | GPT-powered analysis | Medium |

---

## Release Strategy

### Phase Approach
```
Phase 1 (MVP) ──→ Phase 2 ──→ Phase 3 ──→ Phase 4 ──→ Phase 5
   Foundation      Mood        Knowledge    Career      Smart
                   Tracking    Base         Tracking    Features
```

### Iteration Within Phases
- Each phase: Build → Test personally → Refine → Complete
- Gather feedback through personal use
- Adjust priorities based on real usage

### Definition of Done (per phase)

**Phase 1 (MVP Foundation):**
- [x] All P0 features functional
- [x] Data persists correctly
- [x] No blocking bugs
- [x] Usable for daily management
- [x] Documentation updated

**Other Phases:**
- [ ] All P0 features functional
- [ ] Data persists correctly
- [ ] No blocking bugs
- [ ] Usable for daily management
- [ ] Documentation updated

---

## Non-Goals (Explicit Exclusions)

To maintain focus, these are explicitly **not** on the roadmap:

1. **Team collaboration features** - This is a personal tool
2. **HR integration** - Not feeding into HR systems
3. **Performance review automation** - Supports but doesn't replace
4. **Multi-user/authentication** - Single user, local only
5. **Analytics dashboards** - Simple views, not BI tools
6. **AI meeting transcription** - Out of scope
7. **Video integration** - Not a meeting tool

---

## Success Metrics by Phase

| Phase | Key Metric |
|-------|------------|
| Phase 1 | Zero missed 1-on-1s per month |
| Phase 2 | Early detection of mood concerns |
| Phase 3 | Never forget an important date |
| Phase 4 | Clear visibility into career progress |
| Phase 5 | Reduced time on meeting prep |

---

## Technical Debt Considerations

As development progresses, watch for:

- **Data migration** - Plan schema migrations early
- **Performance** - Index queries as data grows
- **Workspace storage** - Monitor disk usage for large teams
- **Code organization** - Refactor as patterns emerge
- **Test coverage** - Add tests as features stabilize
