# Codebase Concerns

**Analysis Date:** 2026-06-10

## Tech Debt

**`Level` and `MeetingFrequency` enums are dead code:**
- Issue: Both `Level` and `MeetingFrequency` enums are defined in `src/model/report.rs` and re-exported via `src/model/mod.rs`, but neither is used anywhere in the codebase. The app stores level as a raw `String` field (`profile.level: Option<String>`) and parses frequency with a hand-rolled match in `Report::meeting_frequency_days()`.
- Files: `src/model/report.rs` (lines 202-335)
- Impact: Dead code that could mislead future contributors who might attempt to use these types, only to discover the data layer doesn't actually use them. Creates a false sense of type-safety for a field that remains stringly-typed.
- Fix approach: Either remove both enums and the `meeting_frequency_days()` duplication, or refactor `ReportProfile.meeting_frequency` and `ReportProfile.level` to use the typed enums with proper serde handling.

**`compute_extended_workspace_summary` is never called:**
- Issue: `compute_extended_workspace_summary` in `src/model/computed.rs` (line 367) is exported from `src/model/mod.rs` but is never called. `WorkspaceSummary.total_report_count` is commented "Will be updated to include 2nd-level reports" (line 362) but this update never happens. The dashboard only uses `team_size` and `active_count`, not `total_report_count`.
- Files: `src/model/computed.rs` (lines 329-373), `src/app/state.rs` (line 34)
- Impact: 2nd-level team members (skip-levels) are loaded and displayed in the manager detail view, but are not counted in any workspace-level totals. The dashboard "team size" stat silently undercounts the org.
- Fix approach: Call `compute_extended_workspace_summary` in `App::load_data()`, accumulating second-level counts during the report loading loop, or remove the distinction if total headcount is not a planned dashboard metric.

**`serde_yaml` 0.9 is deprecated upstream:**
- Issue: `serde_yaml = "0.9"` in `Cargo.toml` pinned to the last version before the crate was deprecated by its author. The 0.9.x line no longer receives updates.
- Files: `Cargo.toml` (line 19), used in `src/storage/repo/entry.rs`, `src/storage/repo/report.rs`, `src/storage/repo/workspace.rs`, `src/main.rs`
- Impact: No security patches will come from upstream. The crate still works, but any YAML serialization bugs or edge cases cannot be fixed by upgrading within the same line.
- Fix approach: Migrate to `serde_yaml2` (community fork) or `marked-yaml`/`yaml-rust2` with a thin serde bridge. The surface area is narrow: frontmatter parsing in `storage/mod.rs` and two `to_string`/`from_str` call sites per repository type.

**Mood trend algorithm only compares first vs last data point:**
- Issue: `calculate_mood_trend` in `src/model/computed.rs` (line 310) computes trend by comparing only the newest (index 0) and oldest (last index) of up to 5 mood readings. If moods oscillate — e.g., `[4, 2, 5, 2, 3]` (newest to oldest) — the result is "rising" (+1 net) while the actual pattern shows distress.
- Files: `src/model/computed.rs` (lines 310-326)
- Impact: Misleading trend indicators on the dashboard and detail view. A team member experiencing volatile mood could show "stable" or "rising" when they need attention.
- Fix approach: Replace with a slope calculation (linear regression or simple weighted average of deltas) over the available data points.

**`WorkspaceConfig` written without YAML frontmatter delimiter:**
- Issue: `WorkspaceRepository::init()` in `src/storage/repo/workspace.rs` (line 49) writes the `.vibe-manager` config file using `format!()` with raw YAML, while `load()` (line 82) parses it with `serde_yaml::from_str` applied directly to the full content. The file starts with a comment `# Vibe Manager workspace\nversion: 1\n...`. Comments are valid YAML, but the workspace file format is inconsistent with the frontmatter format used by all other files (which use `---` delimiters).
- Files: `src/storage/repo/workspace.rs` (lines 47-63)
- Impact: Minor inconsistency that could confuse format extension. Not a functional bug today.
- Fix approach: Either document that `.vibe-manager` is pure YAML (no frontmatter), or align to the frontmatter pattern used elsewhere.

**Unused dependencies `walkdir` and `dirs`:**
- Issue: Both `walkdir = "2.0"` and `dirs = "5.0"` appear in `Cargo.toml` but have zero import sites in `src/`. The directory traversal is done with `std::fs::read_dir` instead.
- Files: `Cargo.toml` (dependencies section)
- Impact: Unnecessary compile-time dependency; `walkdir` adds ~3 crates to the build graph.
- Fix approach: Remove both from `Cargo.toml`.

## Known Bugs

**`HideHelp` message always returns to Dashboard, not to the previous view:**
- Symptoms: Pressing `?` from `ReportDetail` shows the help overlay. Pressing `?` or `Esc` to close help always sets `view_mode = ViewMode::Dashboard` (via `Msg::HideHelp` in `src/app/update.rs` line 33), discarding the user's position in the detail view.
- Files: `src/app/update.rs` (lines 27-35), `src/app/input.rs` (lines 197-206)
- Trigger: Navigate to any report detail → press `?` → press `Esc` to close help.
- Workaround: Navigate back to the report manually.

**Mood validation is one-sided: out-of-range mood values above 5 are stored silently:**
- Symptoms: `JournalEntry::mood()` in `src/model/meeting.rs` (line 132) filters out values outside 1–5 when reading, but `JournalEntryFrontmatter.mood` is `Option<u8>` with no write-time validation. A YAML file with `mood: 0` or `mood: 9` is parsed without error and then silently discarded on read, creating a gap between stored state and displayed state.
- Files: `src/model/meeting.rs` (lines 131-134), `src/storage/repo/entry.rs` (lines 86-89)
- Trigger: Manually editing a journal markdown file to set an out-of-range mood value.
- Workaround: Keep moods in valid range when editing files directly.

## Security Considerations

**Personal data stored as plain-text markdown with no access controls:**
- Risk: Profile files (`_profile.md`) contain sensitive personal information: birthday, partner name, children's names. Journal entries contain meeting notes which may include performance concerns, personal disclosures, or HR-adjacent content. All stored as plain text on disk with filesystem-level permissions only.
- Files: `src/model/report.rs` (lines 77-98), `tests/fixtures/alex-chen/_profile.md`
- Current mitigation: Standard filesystem permissions apply; no application-level encryption.
- Recommendations: Document the plain-text storage model in the README so users understand the security posture. For users managing sensitive conversations, recommend placing the workspace in an encrypted volume or git-crypt-protected repository.

**Editor spawn does not sanitize `$EDITOR`/`$VISUAL` environment variables:**
- Risk: `get_editor()` in `src/editor.rs` (line 27) reads `$EDITOR` verbatim and splits on whitespace to extract program and args. A malicious `$EDITOR` value like `bash -c 'rm -rf ~'` would execute arbitrary commands. This is a standard shell-delegation risk present in most TUI apps that spawn `$EDITOR`.
- Files: `src/editor.rs` (lines 27-61)
- Current mitigation: Only the user's own environment is used; no external input reaches `get_editor()`.
- Recommendations: Acceptable risk given the threat model. Worth documenting in a security note.

## Performance Bottlenecks

**Full workspace reload on every `RefreshData` and `CreateReport`:**
- Problem: `App::load_data()` in `src/app/state.rs` (line 56) re-reads every report directory and all journal entries from disk on each call. It is triggered by `Msg::RefreshData` and also after every `CreateReport` (line 404 of `src/app/update.rs`). For a workspace with 10+ reports each with 50+ entries, this is O(reports × entries) disk reads.
- Files: `src/app/state.rs` (lines 56-106), `src/app/update.rs` (line 404)
- Cause: No incremental update path; the entire data model is rebuilt from scratch.
- Improvement path: Track dirty state per-report; only reload the affected report after mutation operations. Only full reload is needed for external edits (`RefreshData`/`Ctrl+R`).

**`meeting_display_to_entry_index` is O(n) and called on every keypress:**
- Problem: `meeting_display_to_entry_index` in `src/app/state.rs` (line 183) scans all entries for the selected report to build a filtered meeting index on every call. It is called from multiple input handlers in `src/app/input.rs` (lines 81, 97, 104).
- Files: `src/app/state.rs` (lines 183-195), `src/app/input.rs` (lines 80-109)
- Cause: The meeting index is rebuilt inline without caching. For a report with many entries the scan is repeated on every keypress in detail view.
- Improvement path: Cache the meeting index per report and invalidate when entries change, or store meeting/observation entries in separate Vecs at load time.

## Fragile Areas

**Parallel index tracking between `reports`, `entries_by_report`, and `summaries`:**
- Files: `src/app/mod.rs` (lines 148-154), `src/app/state.rs` (lines 59-106)
- Why fragile: Three `Vec`s are kept synchronized by index: `reports[i]`, `entries_by_report[i]`, and `summaries[i]`. Any divergence — from an early `push` failure or a partial reload — causes a silent mismatch where index `i` in each Vec refers to different reports. There is no struct bundling them or runtime check enforcing alignment.
- Safe modification: Always modify all three Vecs together. The `load_data()` method (`src/app/state.rs` line 56) is the safe path; never push to one Vec without the others. Do not call `reports.push()` outside of `load_data()`.
- Test coverage: No test verifies Vec lengths remain equal after mutations.

**`delete_entry` uses raw Vec index from `entries_by_report` which shifts on removal:**
- Files: `src/app/state.rs` (lines 136-158)
- Why fragile: After `entries_by_report[report_idx].remove(entry_idx)`, all entries after `entry_idx` shift down by one. The display-to-entry index mapping (`meeting_display_to_entry_index`) is correct after the deletion because it recomputes from the live Vec, but any stale `selected_entry_index` stored elsewhere would now point to the wrong entry.
- Safe modification: Always set `selected_entry_index = None` immediately after deletion (done in `delete_entry`), but callers must not cache the old index value before calling.
- Test coverage: Covered by `test_app_delete_entry_removes_file` in `tests/app_test.rs`.

**YAML frontmatter parsing fails silently on malformed files:**
- Files: `src/storage/repo/entry.rs` (lines 71-76), `src/app/state.rs` (lines 66-79)
- Why fragile: `load_entries_from_dir` skips entries that fail to load (`if let Ok(entry) = self.load_entry(...)`). `load_data()` uses `filter_map` with `.ok()?` for report loads and `.unwrap_or_default()` for entry lists. Malformed YAML frontmatter silently produces an empty entry list or skips a report entirely, with no user-visible error.
- Safe modification: Accept the silent-skip behavior as intentional resilience, but be aware when debugging missing data that corrupt frontmatter is a likely cause.
- Test coverage: No test for malformed YAML behavior.

**`is_content_empty` in `src/main.rs` uses ad-hoc content detection:**
- Files: `src/main.rs` (lines 289-309)
- Why fragile: The heuristic that decides whether a newly-edited entry should be deleted (empty save = cancel) lives in `main.rs` as an inline function, not in the model or storage layer. It currently skips lines starting with `#`, `- [ ]`, and bare `-`. Any new template structure added to `create_meeting()` in `src/storage/repo/entry.rs` must also be reflected here or the cancel-on-empty heuristic will break.
- Safe modification: If changing the meeting template in `src/storage/repo/entry.rs` (lines 138-145), update `is_content_empty` in `src/main.rs` in the same commit.
- Test coverage: Well covered by unit tests in `src/main.rs` (lines 311-381).

## Scaling Limits

**Single-level manager hierarchy only:**
- Current capacity: The data model supports one level of skip-level (direct reports of direct reports via the `team/` subdirectory). `Report.team: Vec<Report>` is flat; nested managers within the team are not loaded.
- Limit: Organizations with manager-of-managers deeper than two levels cannot be represented.
- Scaling path: Make `load_data()` in `src/app/state.rs` recursive, following `team/` directories at arbitrary depth; update `App` state to handle a tree rather than two flat Vecs.

## Dependencies at Risk

**`serde_yaml` 0.9:**
- Risk: The upstream `dtolnay/serde-yaml` crate was deprecated at 0.9.34. No further bug fixes or security patches will be released.
- Impact: YAML serialization bugs in profile/entry persistence; potential issues with edge-case YAML inputs.
- Migration plan: `serde_yaml2` is a maintained community fork with a compatible API. Migration is a drop-in `Cargo.toml` change plus a `s/serde_yaml/serde_yaml2/g` in the six import sites across storage files.

## Missing Critical Features

**No report archiving or deactivation workflow:**
- Problem: `ReportProfile.active` field exists (`src/model/report.rs` line 67), but there is no UI to set it to `false`. Inactive reports still appear on the dashboard (they are loaded and sorted by urgency). The only way to remove someone from view is to manually delete their directory.
- Blocks: Clean management of team members who have left or transferred.

**No report profile editing in TUI:**
- Problem: Report profiles can only be created (via `NewReportModal`) and read. Editing name, title, level, frequency, or personal info requires manually editing the `_profile.md` YAML frontmatter in an external editor.
- Blocks: Basic profile maintenance without leaving the TUI.

**Birthday and upcoming dates widget not implemented:**
- Problem: `ReportProfile.birthday` is parsed and stored (`src/model/report.rs` line 82) but never surfaced in the UI. The knowledge-base feature spec (`docs/features/knowledge-base.md`) lists FR-2 through FR-5 as planned but not implemented.
- Blocks: The personal knowledge-base value proposition.

**Skill tracking UI not implemented:**
- Problem: `ReportProfile.skills: Option<Skills>` stores a full `HashMap<String,String>` per skill category in `src/model/report.rs` (lines 110-126). No UI exists to view or edit skills. The career-tracking feature spec (`docs/features/career-tracking.md`) lists 4.2–4.6 as planned.
- Blocks: Career development workflow.

## Test Coverage Gaps

**No tests for `App::update()` message dispatch:**
- What's not tested: The TEA update function in `src/app/update.rs` is only exercised indirectly through `test_entry_input_modal_workflow` and `test_entry_input_modal_cancel` in `tests/app_test.rs`. Messages like `Msg::ViewReport`, `Msg::ViewMeeting`, `Msg::UpdateMood`, `Msg::Back`, `Msg::ShowDeleteConfirm`, `Msg::ConfirmDelete`, and all `Modal*` variants have no direct test coverage.
- Files: `src/app/update.rs`
- Risk: Regressions in navigation or modal state machine go undetected.
- Priority: High

**No tests for `handle_key_event` input mapping:**
- What's not tested: `src/app/input.rs` keyboard-to-message mapping has zero unit tests. Key binding changes could silently break navigation.
- Files: `src/app/input.rs`
- Risk: Key binding regressions undetected until manual testing.
- Priority: Medium

**`insta` snapshot testing dependency is listed but unused:**
- What's not tested: `insta = "1.0"` appears in `[dev-dependencies]` in `Cargo.toml` but no `insta::assert_snapshot!` or `insta::assert_yaml_snapshot!` calls exist in either `tests/` or `src/`. Snapshot tests were planned for rendering output but not implemented.
- Files: `Cargo.toml`, `tests/app_test.rs`, `tests/storage_test.rs`
- Risk: Rendering regressions in TUI components are not caught by automated tests.
- Priority: Medium

**No tests for malformed workspace data:**
- What's not tested: Loading behavior when `_profile.md` has invalid YAML, when a journal entry filename is corrupted, or when `.vibe-manager` config is missing required fields.
- Files: `src/storage/repo/entry.rs`, `src/storage/repo/report.rs`
- Risk: Corrupted workspace data produces silent failures or confusing missing-data bugs.
- Priority: Medium

---

*Concerns audit: 2026-06-10*
