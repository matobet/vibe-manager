# Vibe Manager

## What This Is

A Rust TUI application for engineering managers to track 1-on-1 meetings, team
health, and career progress — rendered with an 8-bit RPG aesthetic (ratatui).
Single-user, local-only, pure-markdown storage: the workspace folder *is* the
database. Built by and for its author to run their actual team.

## Core Value

Never lose track of a team member's wellbeing or meeting cadence — including
the 2nd-level reports behind your manager reports.

## Requirements

### Validated

<!-- Inferred from existing codebase (see .planning/codebase/) -->

- ✓ Team dashboard with RPG card grid, urgency sorting, overdue indicators — existing
- ✓ Report profiles (IC/Manager, P1-P5/M1-M5 levels, frequency, partner/children) — existing
- ✓ 1-on-1 meeting creation, viewing, editing via $EDITOR, deletion — existing
- ✓ Mood tracking: F1-F5 recording, standalone observations with context, trends, history chart — existing
- ✓ Pure-markdown workspace storage with YAML frontmatter, legacy + journal/ layouts — existing
- ✓ Managing Managers foundation: data model, nested team/ storage, TeamMetrics computation, manager sprites with headbands — existing
- ✓ New Report modal with type selector and live avatar preview — existing

### Active

<!-- This milestone: Guild Halls navigation + dashboard polish -->

- [ ] Manager doorway cards on dashboard (4-line card: identity, relationship, squad summary with named worst outlier, focus-following door hint)
- [ ] Hall drill-down: Space enters a manager's squad, rendered by the re-rooted dashboard with breadcrumb; Esc / boundary-`h` walk back up
- [ ] Roster path stack powering recursive navigation
- [ ] Skip-level notes: `n` inside a hall creates journal entries for 2nd-level reports
- [ ] Skip-level rotation: computed ordering, rotation strip in hall header, overdue vs default_2nd_level_frequency
- [ ] Density rules: outlier `(+N more)`, truncated rotation strip for large squads
- [ ] Esc hard no-op at root (no quit overshoot); help modal "Squads" section
- [ ] Dashboard polish: mood keys `1-5` replacing/augmenting F1-F5; small adjacent dashboard improvements scoped during requirements

### Out of Scope

- Banner panel in manager detail (Tab-focusable squad face strip) — deferred to a later milestone; doorway + halls are the smallest useful slice
- Quest Log (cross-tree attention queue) — future milestone; valuable at 10+ report scale but new machinery
- Team collaboration, HR integration, multi-user — personal tool by design (see docs/roadmap.md non-goals)
- Calendar/Slack integrations — future consideration, not this milestone
- Knowledge base dates widget, skill matrix UI — separate roadmap phases (3 and 4), not part of Managing Managers

## Context

- **Design contract:** `docs/features/managing-managers.md` contains the full
  Guild Halls UX spec (screens, anatomy, navigation model, density rules,
  ergonomics decisions) written and refined this session. HTML mockup at
  `docs/mockups/guild-halls.html`.
- **Codebase map:** `.planning/codebase/` (7 documents, mapped 2026-06-10).
  TEA architecture: `App` state in `app/`, messages in `Msg`, views in
  `views/`, reusable widgets in `components/`, theme in `theme/`.
- **Key existing leverage:** `TeamMetrics` is fully computed with tests but
  never rendered; `Report.team` is loaded but not navigable. This milestone is
  almost entirely view/state-layer work on top of a finished foundation.
- **Design constraints settled during discussion:** scales from 1 manager to
  org trees (one hall per screen); outliers over averages (name the worst
  member, never just a score); skip-levels run on a regular rotation.
- **Docs discipline:** CLAUDE.md mandates keeping `docs/implementation-status.md`,
  feature spec status tables, and `docs/roadmap.md` in sync with implementation.
- Uncommitted at milestone start: spec ergonomics updates in
  `docs/features/managing-managers.md`, mockup `docs/mockups/guild-halls.html`.

## Constraints

- **Tech stack**: Rust + ratatui, single binary, no new runtime dependencies expected — the milestone is UI/state work
- **Storage**: Pure markdown/YAML files; rotation and metrics stay computed, never stored
- **Quality gates**: CI enforces `cargo clippy --all-features -- -D warnings` (clippy 1.96+) and `cargo fmt --check`; tests use insta snapshots + fixtures in `tests/fixtures/`
- **Compatibility**: Legacy root-level meeting files must keep working; existing keybindings stay stable except the conscious Space rebind
- **Terminal**: Glyphs must render in common monospace fonts; `◦︵◦` alignment caveat already documented

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Guild Halls (drill-down) over accordion expansion | Inline grid-within-list is fiddly and degrades worst as teams grow; one hall per screen scales to any size and reuses the dashboard component | — Pending |
| `Enter` = person, `Space` = container | Enter consistency is the app's strongest navigation asset; Space is the only new verb | — Pending |
| Space rebind (was synonym for Enter) | Conscious break; focus-following door hint retrains it | — Pending |
| Esc hard no-op at root; boundary-`h` ascends | Prevents Esc-spam quit overshoot; home-row back-walk per ranger/lf convention | — Pending |
| Defer Banner panel and Quest Log | Smallest useful slice first: see squads, navigate in, take skip-level notes | — Pending |
| Rotation is computed, not stored | Derives from journal timestamps vs default_2nd_level_frequency; no schema change | — Pending |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `/gsd-transition`):
1. Requirements invalidated? → Move to Out of Scope with reason
2. Requirements validated? → Move to Validated with phase reference
3. New requirements emerged? → Add to Active
4. Decisions to log? → Add to Key Decisions
5. "What This Is" still accurate? → Update if drifted

**After each milestone** (via `/gsd:complete-milestone`):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-06-10 after initialization*
