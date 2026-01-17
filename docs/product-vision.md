# Vibe Manager - Product Vision

## Mission Statement

Vibe Manager is a **terminal-based application (TUI)** designed to help engineering managers maintain strong, consistent relationships with their direct reports through thoughtful 1-on-1 management, career development tracking, and holistic understanding of each team member.

## Target User

**Primary Persona: The Individual Engineering Manager**

- Manages a team of 2-10 engineers
- Values personal connection with each team member
- Currently uses fragmented tools (Notion, Obsidian, spreadsheets) for tracking
- Privacy-conscious - prefers local data storage
- Markdown-native workflow
- **Comfortable in the terminal** - uses CLI tools daily, appreciates keyboard-driven interfaces

## Core Value Proposition

### What Vibe Manager Does

1. **Never forget a 1-on-1** - Tracks meeting cadence per engineer with smart reminders
2. **Career path visibility** - Integrates with your R&D Career Path to track progress
3. **Organized meeting notes** - Markdown-native notes that align with existing workflows
4. **Personal knowledge base** - Remember the important details about each person
5. **Health/happiness trends** - Simple mood tracking to spot concerns early

### What Vibe Manager Is NOT

- **Not an HR tool** - This is for individual manager use, not organizational reporting
- **Not a performance review system** - Supports development, not evaluation
- **Not a project management tool** - Focused on people, not tasks
- **Not a communication platform** - Complements, doesn't replace, your existing channels

## Core Pain Points Addressed

| Pain Point | Solution |
|------------|----------|
| Forgetting 1-on-1s or letting them slip | Per-engineer cadence tracking with overdue alerts |
| Losing track of career progress | Career path integration with progress indicators |
| Scattered notes across tools | Centralized, markdown-native note management |
| Forgetting personal details | Structured knowledge base for family/background info |
| Missing early signs of burnout | Simple mood tracking with trend visualization |

## Design Principles

### 1. Privacy First
- All data stored locally
- No cloud sync (in initial version)
- Data stays on your machine

### 2. Simplicity Over Features
- Optimized for a single user managing 2-10 people
- No unnecessary complexity from enterprise features
- Quick to use - should take seconds, not minutes

### 3. Markdown Native
- Notes are plain markdown
- Easy export for portability
- Compatible with existing Obsidian/Notion workflows

### 4. Glanceable Dashboard
- At-a-glance team health status
- Immediate visibility into who needs attention
- Priorities clear without drilling down

### 5. Flexible, Not Prescriptive
- Adapt to your workflow, not the other way around
- Per-engineer customization (cadence, categories)
- Free-form notes with optional structure

## UX Vision: 8-Bit RPG Aesthetic via TUI

### Why Terminal UI?

A TUI (Terminal User Interface) is the **perfect platform** for this aesthetic:

| TUI Advantage | Benefit |
|---------------|---------|
| Box-drawing characters | Native RPG-style borders and frames |
| Limited color palette | Authentic 8-bit feel (16/256 colors) |
| Monospace fonts | Clean stat alignments, ASCII art |
| Keyboard-driven | Fast, efficient, no mouse needed |
| Instant startup | No browser, no loading screens |
| Truly local | No web server, pure filesystem |
| Developer-native | Target audience lives in terminals |

### Target Audience
The primary user grew up in the 80s/90s as a computer enthusiast - familiar with 8-bit graphics, early RPGs, DOS interfaces, and the golden age of gaming. Now in their 30s, they appreciate nostalgic design that's functional, not just decorative. They spend their days in terminals and IDEs.

### The Metaphor
Your team of engineers is your **party of characters**:

| Engineering Concept | RPG Equivalent |
|---------------------|----------------|
| Engineer | Party member / Character |
| Career level (P1-P5) | Character level |
| Skills & proficiencies | Stats & attributes |
| Mood/health score | HP / Morale |
| 1-on-1 meeting | Party check-in / Rest at inn |
| Skill development | Leveling up / Gaining XP |
| Team dashboard | Party management screen |
| Overdue 1-on-1 | Character needs attention |

### Visual Inspiration
- **Final Fantasy** (NES/SNES) - Party screens, stat displays, menu systems
- **Dragon Quest** - Character status, simple iconography
- **Chrono Trigger** - Clean UI, character portraits with status
- **Rogue/NetHack** - ASCII-based interfaces, information density
- **DOS-era apps** - Norton Commander, Turbo Pascal IDEs

### TUI Design Elements
- **Box-drawing borders** - `╔═══╗ ║ ╚═══╝` for panels and cards
- **Block characters** - `█▓▒░` for progress bars and HP gauges
- **Unicode symbols** - `★ ● ⚠ ♥ ⚔ 📦 🎯` for status icons
- **Color coding** - ANSI colors for status (red=critical, green=healthy)
- **Keyboard shortcuts** - Vim-style navigation (`j/k`, `h/l`, `gg`, `G`)
- **Modal interfaces** - Pop-up panels for editing, like classic RPG menus

### Tone
- **Playful but functional** - The aesthetic serves usability, not just nostalgia
- **Respectful** - Engineers are people, not literally game characters
- **Subtle references** - Easter eggs for those who get it, invisible to those who don't
- **Fast and efficient** - Terminal users expect speed

## Scope Boundaries

### In Scope (MVP)

- Engineer profile management (2-10 people)
- 1-on-1 scheduling and tracking
- Per-engineer meeting cadence settings
- Markdown note-taking for meetings
- Simple mood tracking (1-5 scale)
- Career path progress tracking
- Personal knowledge base
- Team overview dashboard
- Local data storage

### Out of Scope

- Google Calendar integration (future consideration)
- Multi-device sync
- Team collaboration features
- Native mobile applications
- AI-powered insights
- Integration with HR systems

### Simplicity Principles

- **Markdown-native storage** - Notes stored as markdown files, human-readable
- **Terminal-first** - Rich TUI, keyboard-driven, instant startup
- **Core functionality first** - Resist feature creep, solve the main pain points well
- **Plain files** - Data stored as JSON/markdown files, no database server needed

## Success Metrics

For a personal tool, success is measured by:

1. **Consistency** - No 1-on-1s missed or significantly overdue
2. **Awareness** - Early detection of team member concerns
3. **Recall** - Quickly access relevant context before meetings
4. **Career Progress** - Clear visibility into each person's development
5. **Adoption** - Tool becomes the go-to for people management tasks

## Technical Constraints

- **Terminal User Interface (TUI)** - Runs in any modern terminal emulator
- **Folder-based workspaces** - Each team in its own folder, like Obsidian vaults
- **Local filesystem storage** - JSON for structured data, markdown for notes
- **No external service dependencies** - Fully offline, no network required
- **Single binary distribution** - Easy to install and run
- **Cross-platform** - Works on macOS, Linux, Windows (WSL)
