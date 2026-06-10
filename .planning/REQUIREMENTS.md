# Requirements: Vibe Manager — Guild Halls Milestone

**Defined:** 2026-06-10
**Core Value:** Never lose track of a team member's wellbeing or meeting cadence — including the 2nd-level reports behind your manager reports.

## v1 Requirements

Requirements for this milestone. Each maps to roadmap phases.
Design contract: `docs/features/managing-managers.md` (Guild Halls UX spec).

### Doorway Cards

- [ ] **DOOR-01**: Manager renders as a 4-line doorway card on the dashboard (identity / relationship / squad summary / door hint)
- [ ] **DOOR-02**: Squad line shows size, health bar, and worst outlier by name — `(+N more)` when several, `★ all well · next: <name>` when healthy
- [ ] **DOOR-03**: Door hint renders only on the selected card (fixed card height, blank line when unselected)
- [ ] **DOOR-04**: Manager urgency score incorporates the worst squad outlier so troubled teams sort upward

### Hall Navigation

- [ ] **HALL-01**: `Space` on a manager card enters their hall (re-rooted dashboard); no-op on IC cards
- [ ] **HALL-02**: Hall header shows breadcrumb (`YOU ▸ JORDAN'S SQUAD`), member count, and team health bar
- [ ] **HALL-03**: `Esc` walks up one level; hard no-op at the root dashboard (`q` is the only quit)
- [ ] **HALL-04**: `h` at the leftmost column also ascends one level (ranger/lf convention)
- [ ] **HALL-05**: All existing dashboard keys behave identically inside halls (`j/k/h/l`, `g/G`, `Enter`, `?`)
- [ ] **HALL-06**: Navigation is powered by a roster path stack supporting arbitrary nesting depth
- [ ] **HALL-07**: Help modal gains a "Squads" section covering `Space`, `Esc`/boundary-`h`, and skip-level notes

### Skip-Levels

- [ ] **SKIP-01**: `n` inside a hall creates a journal entry for the selected 2nd-level report (skip-level note)
- [ ] **SKIP-02**: Overdue status inside halls is computed against `default_2nd_level_frequency` (or the member's `meeting_frequency` override)
- [ ] **SKIP-03**: Rotation strip in hall header — members ordered by time since last entry, overdue first; truncates to overdue + next 2 + `+N queued` for squads of 8+

### Dashboard Polish

- [ ] **POLISH-01**: Mood recordable with plain `1–5` digit keys (alongside existing F1–F5)
- [ ] **POLISH-02**: Dashboard filtering by status/seniority (e.g. overdue-only, managers-only)
- [ ] **POLISH-03**: Quick actions from a dashboard card without entering detail (e.g. record mood)

## v2 Requirements

Deferred to future release. Tracked but not in current roadmap.

### Dashboard

- **DASH-V2-01**: Weekly summary panel — at-a-glance week recap on the dashboard

## Out of Scope

| Feature | Reason |
|---------|--------|
| Banner panel (Tab-focusable squad face strip in manager detail) | Deferred to a later milestone; doorway + halls are the smallest useful slice |
| Quest Log (cross-tree attention queue) | Future milestone — valuable at 10+ report scale but entirely new machinery |
| Knowledge base dates widget, skill matrix UI | Separate roadmap phases (3 and 4), not part of Managing Managers |
| Team collaboration, HR integration, multi-user | Personal tool by design (docs/roadmap.md non-goals) |
| Calendar/Slack integrations | Future consideration, not this milestone |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| DOOR-01 | Phase 1 | Pending |
| DOOR-02 | Phase 1 | Pending |
| DOOR-03 | Phase 1 | Pending |
| DOOR-04 | Phase 1 | Pending |
| HALL-01 | Phase 2 | Pending |
| HALL-02 | Phase 2 | Pending |
| HALL-03 | Phase 2 | Pending |
| HALL-04 | Phase 2 | Pending |
| HALL-05 | Phase 2 | Pending |
| HALL-06 | Phase 2 | Pending |
| HALL-07 | Phase 3 | Pending |
| SKIP-01 | Phase 3 | Pending |
| SKIP-02 | Phase 3 | Pending |
| SKIP-03 | Phase 3 | Pending |
| POLISH-01 | Phase 4 | Pending |
| POLISH-02 | Phase 4 | Pending |
| POLISH-03 | Phase 4 | Pending |

**Coverage:**
- v1 requirements: 17 total
- Mapped to phases: 17
- Unmapped: 0 ✓

---
*Requirements defined: 2026-06-10*
*Last updated: 2026-06-10 after roadmap creation (traceability populated)*
