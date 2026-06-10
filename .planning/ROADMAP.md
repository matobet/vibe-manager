# Roadmap: Vibe Manager — Guild Halls Milestone

## Overview

The Managing Managers foundation (data model, nested `team/` storage, TeamMetrics
computation) is built and tested but invisible. This milestone makes it usable in
four vertical slices: first managers become informative on the dashboard (doorway
cards rendering TeamMetrics with named worst outliers), then squads become
reachable (hall drill-down via a roster path stack), then skip-level rotations
become trackable (notes, 2nd-level overdue semantics, rotation strip), and finally
the dashboard gets independent ergonomic polish (digit mood keys, filtering, quick
actions). The app is fully working and useful after every phase. Design contract:
`docs/features/managing-managers.md`.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [ ] **Phase 1: Doorway Cards** - Managers render as 4-line doorway cards surfacing squad health and the named worst outlier
- [ ] **Phase 2: Hall Navigation** - Space enters a manager's squad as a re-rooted dashboard; Esc / boundary-`h` walk back up via a roster path stack
- [ ] **Phase 3: Skip-Level Workflow** - Take skip-level notes in halls, see 2nd-level overdue status, rotation strip, and help docs
- [ ] **Phase 4: Dashboard Polish** - Digit mood keys, dashboard filtering, and quick actions from cards

## Phase Details

### Phase 1: Doorway Cards
**Goal**: Every manager on the dashboard shows their squad's state at a glance — size, health, and the person who most needs attention — and troubled teams sort upward
**Mode:** mvp
**Depends on**: Nothing (first phase — TeamMetrics already computed and tested)
**Requirements**: DOOR-01, DOOR-02, DOOR-03, DOOR-04
**Success Criteria** (what must be TRUE):
  1. A manager renders on the dashboard as a 4-line doorway card: identity (name, role, M-level), relationship (mood hearts + your 1-on-1 recency), squad summary, and door-hint line
  2. The squad line shows squad size, a health bar, and the worst outlier by name (e.g. `⚠ Sam T: mood ↘ · 6w`); multiple outliers collapse to `(+N more)`; a healthy squad shows `★ all well · next: <name>`
  3. The `▸ Space to visit squad` hint renders only on the selected manager card; unselected cards keep a blank fourth line so card height never changes
  4. A manager with a troubled squad member sorts above an otherwise-comparable manager with a healthy squad (urgency score incorporates worst squad outlier)
  5. IC cards and all existing dashboard behavior (sorting, keys, overdue markers) remain unchanged
**Plans**: TBD

### Phase 2: Hall Navigation
**Goal**: Users can walk into any manager's squad and tour it with the exact keys they already know, then walk back out — without ever overshooting into a quit
**Mode:** mvp
**Depends on**: Phase 1 (doorway cards provide the Space affordance and the squads worth visiting)
**Requirements**: HALL-01, HALL-02, HALL-03, HALL-04, HALL-05, HALL-06
**Success Criteria** (what must be TRUE):
  1. Pressing `Space` on a manager card enters their hall — the dashboard re-rooted at their team — and `Space` on an IC card does nothing
  2. The hall header shows a breadcrumb (`YOU ▸ JORDAN'S SQUAD`), member count, and team health bar
  3. `Esc` inside a hall returns to the parent roster; `Esc` at the root dashboard is a hard no-op (`q` remains the only quit)
  4. `h` at the leftmost column ascends one level (ranger/lf convention), equivalent to `Esc`
  5. All existing dashboard keys (`j/k/h/l`, `g/G`, `Enter`, `?`) behave identically inside halls, powered by a roster path stack that supports arbitrary nesting depth
**Plans**: TBD

### Phase 3: Skip-Level Workflow
**Goal**: Skip-level rotations become trackable — users can take notes on 2nd-level reports from inside a hall, see who is overdue against the 2nd-level cadence, and see who is next in the rotation
**Mode:** mvp
**Depends on**: Phase 2 (notes, overdue markers, and the rotation strip all live inside halls)
**Requirements**: SKIP-01, SKIP-02, SKIP-03, HALL-07
**Success Criteria** (what must be TRUE):
  1. Pressing `n` inside a hall creates a journal entry for the selected 2nd-level report via `$EDITOR`; the saved skip-level note appears in that member's journal and detail view
  2. Overdue markers (`zZ`) inside halls are computed against `default_2nd_level_frequency` (or the member's own `meeting_frequency` override), not the direct-report cadence
  3. The hall header shows a rotation strip ordering members by time since last entry, overdue first; squads of 8+ truncate to overdue + next 2 + `+N queued`
  4. The `?` help modal has a "Squads" section documenting `Space`, `Esc`/boundary-`h`, and skip-level `n` notes
**Plans**: TBD

### Phase 4: Dashboard Polish
**Goal**: Recording moods and triaging the roster gets faster — fewer keystrokes, no detour through detail views
**Mode:** mvp
**Depends on**: Phase 3 (no technical dependency; sequenced last so the milestone's keybinding surface is settled before adding digit keys and quick actions)
**Requirements**: POLISH-01, POLISH-02, POLISH-03
**Success Criteria** (what must be TRUE):
  1. Plain `1`–`5` digit keys record mood everywhere `F1`–`F5` currently works, and `F1`–`F5` keep working
  2. User can filter the dashboard by status/seniority (e.g. overdue-only, managers-only) and clear the filter to return to the full roster
  3. User can perform a quick action (e.g. record mood) directly from a selected dashboard card without entering the detail view
**Plans**: TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 3 → 4

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Doorway Cards | 0/TBD | Not started | - |
| 2. Hall Navigation | 0/TBD | Not started | - |
| 3. Skip-Level Workflow | 0/TBD | Not started | - |
| 4. Dashboard Polish | 0/TBD | Not started | - |
