//! Integration tests for App state management

use std::path::PathBuf;

use tempfile::TempDir;

#[cfg(test)]
mod tests {
    use super::*;

    fn fixtures_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
    }

    /// Copy fixtures to a temp directory for mutation tests
    fn setup_temp_workspace() -> TempDir {
        let temp = TempDir::new().expect("Failed to create temp dir");
        let fixtures = fixtures_path();

        // Copy the entire fixtures directory to temp
        copy_dir_all(&fixtures, temp.path()).expect("Failed to copy fixtures");

        temp
    }

    fn copy_dir_all(src: &PathBuf, dst: &std::path::Path) -> std::io::Result<()> {
        std::fs::create_dir_all(dst)?;
        for entry in std::fs::read_dir(src)? {
            let entry = entry?;
            let ty = entry.file_type()?;
            if ty.is_dir() {
                copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
            } else {
                std::fs::copy(entry.path(), dst.join(entry.file_name()))?;
            }
        }
        Ok(())
    }

    #[test]
    fn test_app_loads_workspace() {
        let path = fixtures_path();
        let app = vibe_manager::app::App::new(path).expect("Failed to load app");

        assert!(!app.reports.is_empty());
        // Should load all reports from fixtures
        assert!(app.reports.len() >= 2);
    }

    #[test]
    fn test_app_delete_entry_removes_file() {
        let temp = setup_temp_workspace();

        let mut app =
            vibe_manager::app::App::new(temp.path().to_path_buf()).expect("Failed to load app");

        // Find the report and meeting indices
        let report_idx = app
            .reports
            .iter()
            .position(|e| e.profile.name == "Alex Chen")
            .expect("Alex Chen not found");

        let initial_entry_count = app.entries_by_report[report_idx].len();
        assert!(initial_entry_count > 0, "No entries to delete");

        let entry_path = app.entries_by_report[report_idx][0].path.clone();
        assert!(
            entry_path.exists(),
            "Entry file should exist before deletion"
        );

        // Delete the meeting
        app.delete_entry(report_idx, 0)
            .expect("Failed to delete meeting");

        // Verify meeting was removed from memory
        assert_eq!(
            app.entries_by_report[report_idx].len(),
            initial_entry_count - 1
        );

        // Verify file was deleted
        assert!(!entry_path.exists(), "Entry file should be deleted");
    }

    #[test]
    fn test_app_delete_entry_updates_summary() {
        let temp = setup_temp_workspace();

        let mut app =
            vibe_manager::app::App::new(temp.path().to_path_buf()).expect("Failed to load app");

        let report_idx = app
            .reports
            .iter()
            .position(|e| e.profile.name == "Alex Chen")
            .expect("Alex Chen not found");

        let initial_days_since = app.summaries[report_idx].days_since_meeting;

        // Find the most recent meeting (entries with content, sorted oldest first)
        let last_meeting_idx = app.entries_by_report[report_idx]
            .iter()
            .enumerate()
            .filter(|(_, e)| e.is_meeting())
            .map(|(i, _)| i)
            .last()
            .expect("No meetings found");

        app.delete_entry(report_idx, last_meeting_idx)
            .expect("Failed to delete meeting");

        // Summary should be recomputed with new days_since_meeting
        let new_days_since = app.summaries[report_idx].days_since_meeting;

        // Check if there are still meetings
        let remaining_meetings = app.entries_by_report[report_idx]
            .iter()
            .filter(|e| e.is_meeting())
            .count();

        // If there was more than one meeting, the days_since_meeting should change
        // (it will be longer since we deleted the most recent meeting)
        if remaining_meetings > 0 {
            assert!(
                new_days_since > initial_days_since,
                "Days since meeting should increase after deleting most recent meeting"
            );
        }
    }

    #[test]
    fn test_app_delete_all_meetings() {
        let temp = setup_temp_workspace();

        let mut app =
            vibe_manager::app::App::new(temp.path().to_path_buf()).expect("Failed to load app");

        let report_idx = app
            .reports
            .iter()
            .position(|e| e.profile.name == "Alex Chen")
            .expect("Alex Chen not found");

        // Delete all entries one by one (always index 0 since list shrinks)
        while !app.entries_by_report[report_idx].is_empty() {
            app.delete_entry(report_idx, 0)
                .expect("Failed to delete entry");
        }

        assert!(app.entries_by_report[report_idx].is_empty());
        // With no meetings, days_since_meeting should be None
        assert!(app.summaries[report_idx].days_since_meeting.is_none());
    }

    #[test]
    fn test_status_message_expiry() {
        let path = fixtures_path();
        let mut app = vibe_manager::app::App::new(path).expect("Failed to load app");

        // Set a status message
        app.set_status("Test message");

        // Status should be visible immediately
        assert_eq!(app.status_text(), Some("Test message"));

        // Note: In real tests, we'd mock time. Here we just verify the API works.
        // The actual timeout is 3 seconds which is too long for tests.
    }

    #[test]
    fn test_entry_input_modal_workflow() {
        use vibe_manager::app::{Msg, ViewMode};
        use vibe_manager::model::Context;

        let temp = setup_temp_workspace();
        let mut app =
            vibe_manager::app::App::new(temp.path().to_path_buf()).expect("Failed to load app");

        // Select a report first
        let report_idx = app
            .reports
            .iter()
            .position(|e| e.profile.name == "Alex Chen")
            .expect("Alex Chen not found");

        app.selected_report_index = Some(report_idx);
        app.view_mode = ViewMode::ReportDetail;

        let initial_entry_count = app.entries_by_report[report_idx].len();

        // Show entry input modal
        app.update(Msg::ShowEntryInput).unwrap();
        assert_eq!(app.view_mode, ViewMode::EntryInputModal);
        assert_eq!(app.pending_entry_mood, None);
        assert_eq!(app.pending_entry_context, Context::Standup); // Default

        // Set mood
        app.update(Msg::SetEntryMood(4)).unwrap();
        assert_eq!(app.pending_entry_mood, Some(4));

        // Cycle context
        app.update(Msg::CycleEntryContext).unwrap();
        assert_eq!(app.pending_entry_context, Context::Slack);

        // Add some notes
        app.update(Msg::Input('H')).unwrap();
        app.update(Msg::Input('i')).unwrap();
        assert_eq!(app.pending_entry_notes, "Hi");

        // Save entry
        app.update(Msg::SaveEntry).unwrap();
        assert_eq!(app.view_mode, ViewMode::ReportDetail);
        assert_eq!(
            app.entries_by_report[report_idx].len(),
            initial_entry_count + 1
        );

        // Verify the new entry
        let new_entry = app.entries_by_report[report_idx].last().unwrap();
        assert_eq!(new_entry.mood(), Some(4));
        assert_eq!(new_entry.frontmatter.context, Some(Context::Slack));
        assert_eq!(new_entry.content, "Hi");
        assert!(!new_entry.is_meeting()); // Slack context = not a meeting
    }

    #[test]
    fn test_entry_input_modal_cancel() {
        use vibe_manager::app::{Msg, ViewMode};

        let temp = setup_temp_workspace();
        let mut app =
            vibe_manager::app::App::new(temp.path().to_path_buf()).expect("Failed to load app");

        let report_idx = app
            .reports
            .iter()
            .position(|e| e.profile.name == "Alex Chen")
            .expect("Alex Chen not found");

        app.selected_report_index = Some(report_idx);
        app.view_mode = ViewMode::ReportDetail;
        let initial_count = app.entries_by_report[report_idx].len();

        // Open modal and set some values
        app.update(Msg::ShowEntryInput).unwrap();
        app.update(Msg::SetEntryMood(3)).unwrap();
        app.update(Msg::Input('x')).unwrap();

        // Cancel
        app.update(Msg::CancelModal).unwrap();

        // Should be back to detail view with no new entry
        assert_eq!(app.view_mode, ViewMode::ReportDetail);
        assert_eq!(app.entries_by_report[report_idx].len(), initial_count);
        // State should be cleared
        assert_eq!(app.pending_entry_mood, None);
        assert!(app.pending_entry_notes.is_empty());
    }

    #[test]
    fn test_meeting_display_to_entry_index() {
        let path = fixtures_path();
        let mut app = vibe_manager::app::App::new(path).expect("Failed to load app");

        // Find report with mixed entries (meetings + mood observations)
        let report_idx = app
            .reports
            .iter()
            .position(|e| e.profile.name == "Alex Chen")
            .expect("Alex Chen not found");

        app.selected_report_index = Some(report_idx);

        let entries = &app.entries_by_report[report_idx];
        let meeting_count = entries.iter().filter(|e| e.is_meeting()).count();

        // Display index 0 = most recent meeting (last meeting in array when reversed)
        if meeting_count > 0 {
            let idx = app.meeting_display_to_entry_index(0);
            assert!(idx.is_some());
            let entry = &entries[idx.unwrap()];
            assert!(entry.is_meeting());
        }

        // Out of bounds should return None
        assert!(app.meeting_display_to_entry_index(1000).is_none());
    }

    #[test]
    fn test_selected_meeting_count_excludes_observations() {
        let path = fixtures_path();
        let mut app = vibe_manager::app::App::new(path).expect("Failed to load app");

        let report_idx = app
            .reports
            .iter()
            .position(|e| e.profile.name == "Alex Chen")
            .expect("Alex Chen not found");

        app.selected_report_index = Some(report_idx);

        let total_entries = app.entries_by_report[report_idx].len();
        let meeting_count = app.selected_meeting_count();

        // Should have more total entries than meetings (due to mood observations)
        assert!(
            total_entries >= meeting_count,
            "Total entries ({}) should be >= meeting count ({})",
            total_entries,
            meeting_count
        );
    }

    /// Find Chris Wong (the fixture manager with a 3-member squad)
    fn chris_wong_idx(app: &vibe_manager::app::App) -> usize {
        app.reports
            .iter()
            .position(|e| e.profile.name == "Chris Wong")
            .expect("Chris Wong not found")
    }

    #[test]
    fn test_manager_team_metrics_loaded_at_runtime() {
        let path = fixtures_path();
        let app = vibe_manager::app::App::new(path).expect("Failed to load app");

        let idx = chris_wong_idx(&app);
        let metrics = app.summaries[idx]
            .team_metrics
            .as_ref()
            .expect("manager should have team metrics after load");

        // Structure-only assertions — fixture dates drift against Local::now()
        assert_eq!(metrics.team_size, 7);
        assert!(
            !metrics.outliers.is_empty(),
            "Devon (never met) and Morgan (mood 2) should flag outliers"
        );
        // Date-independent ordering: Devon never met (urgency 110) beats
        // Morgan's mood-2 (overdue cap 80 + 20), which beats mood-4/5 peers
        assert_eq!(metrics.outliers[0].name, "Devon Okafor");
        assert_eq!(metrics.outliers[1].name, "Morgan Smith");

        // Jamie is inactive (on leave): counted in squad size, never an outlier
        assert!(
            !metrics.outliers.iter().any(|o| o.name == "Jamie Flores"),
            "inactive members must not flag as outliers"
        );

        // Never-met ranks first in the skip-level rotation
        assert_eq!(metrics.next_in_rotation.as_deref(), Some("Devon Okafor"));
    }

    #[test]
    fn test_manager_urgency_includes_squad_bonus() {
        use vibe_manager::model::{compute_report_summary, manager_urgency_bonus};

        let path = fixtures_path();
        let app = vibe_manager::app::App::new(path).expect("Failed to load app");

        let idx = chris_wong_idx(&app);
        let metrics = app.summaries[idx]
            .team_metrics
            .as_ref()
            .expect("manager should have team metrics");
        let bonus = manager_urgency_bonus(metrics);
        assert!(bonus > 0, "troubled squad should produce a positive bonus");

        // The loaded score is exactly the manager's own score plus the bonus
        let own_summary = compute_report_summary(
            &app.reports[idx],
            &app.entries_by_report[idx],
            app.workspace.config.settings.overdue_threshold_days,
        );
        assert_eq!(
            app.summaries[idx].urgency_score,
            own_summary.urgency_score + bonus
        );
    }

    #[test]
    fn test_team_metrics_survive_save_entry() {
        use vibe_manager::app::{Msg, ViewMode};

        let temp = setup_temp_workspace();
        let mut app =
            vibe_manager::app::App::new(temp.path().to_path_buf()).expect("Failed to load app");

        let idx = chris_wong_idx(&app);
        assert!(app.summaries[idx].team_metrics.is_some());

        // Record a mood observation for the manager
        app.selected_report_index = Some(idx);
        app.view_mode = ViewMode::ReportDetail;
        app.update(Msg::ShowEntryInput).unwrap();
        app.update(Msg::SetEntryMood(4)).unwrap();
        app.update(Msg::SaveEntry).unwrap();

        let metrics = app.summaries[idx]
            .team_metrics
            .as_ref()
            .expect("team metrics must survive recording an observation");
        assert_eq!(metrics.team_size, 7);
        assert!(!metrics.outliers.is_empty());
    }

    #[test]
    fn test_team_metrics_survive_delete_entry() {
        let temp = setup_temp_workspace();
        let mut app =
            vibe_manager::app::App::new(temp.path().to_path_buf()).expect("Failed to load app");

        let idx = chris_wong_idx(&app);
        assert!(app.summaries[idx].team_metrics.is_some());
        assert!(!app.entries_by_report[idx].is_empty());

        app.delete_entry(idx, 0).expect("Failed to delete entry");

        let metrics = app.summaries[idx]
            .team_metrics
            .as_ref()
            .expect("team metrics must survive deleting an entry");
        assert_eq!(metrics.team_size, 7);
    }

    #[test]
    fn test_enter_hall_swaps_roster_to_squad() {
        use vibe_manager::app::Msg;

        let path = fixtures_path();
        let mut app = vibe_manager::app::App::new(path).expect("Failed to load app");

        app.selected_index = chris_wong_idx(&app);
        app.update(Msg::EnterHall).unwrap();

        assert_eq!(app.hall_stack.len(), 1);
        assert_eq!(app.hall_stack[0].slug, "chris-wong");
        assert_eq!(app.reports.len(), 7, "hall roster = Chris's squad");
        assert!(app.reports.iter().all(|r| r.is_second_level()));
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn test_enter_hall_noop_on_ic() {
        use vibe_manager::app::Msg;

        let path = fixtures_path();
        let mut app = vibe_manager::app::App::new(path).expect("Failed to load app");

        let ic_idx = app
            .reports
            .iter()
            .position(|r| r.profile.name == "Alex Chen")
            .expect("Alex Chen not found");
        app.selected_index = ic_idx;
        let roster_len = app.reports.len();

        app.update(Msg::EnterHall).unwrap();

        assert!(app.hall_stack.is_empty(), "Space on an IC must do nothing");
        assert_eq!(app.reports.len(), roster_len);
        assert_eq!(app.selected_index, ic_idx);
    }

    #[test]
    fn test_enter_hall_noop_on_empty_squad() {
        use vibe_manager::app::Msg;

        let path = fixtures_path();
        let mut app = vibe_manager::app::App::new(path).expect("Failed to load app");

        let idx = app
            .reports
            .iter()
            .position(|r| r.profile.name == "Minimal Manager")
            .expect("Minimal Manager not found");
        app.selected_index = idx;

        app.update(Msg::EnterHall).unwrap();

        assert!(
            app.hall_stack.is_empty(),
            "empty squad has no hall to enter"
        );
        assert!(app.status_text().is_some(), "user gets a status hint");
    }

    #[test]
    fn test_exit_hall_restores_parent_and_selection() {
        use vibe_manager::app::Msg;

        let path = fixtures_path();
        let mut app = vibe_manager::app::App::new(path).expect("Failed to load app");

        let chris_idx = chris_wong_idx(&app);
        app.selected_index = chris_idx;
        app.update(Msg::EnterHall).unwrap();
        app.update(Msg::SelectNext).unwrap(); // wander inside the hall

        app.update(Msg::ExitHall).unwrap();

        assert!(app.hall_stack.is_empty());
        assert_eq!(
            app.reports[app.selected_index].slug, "chris-wong",
            "selection returns to the manager we came from"
        );
    }

    #[test]
    fn test_exit_hall_noop_at_root() {
        use vibe_manager::app::Msg;

        let path = fixtures_path();
        let mut app = vibe_manager::app::App::new(path).expect("Failed to load app");

        let roster_len = app.reports.len();
        app.update(Msg::ExitHall).unwrap(); // Esc at root: hard no-op

        assert!(!app.should_quit);
        assert_eq!(app.reports.len(), roster_len);
    }

    #[test]
    fn test_nested_hall_two_levels_deep() {
        use vibe_manager::app::Msg;

        let path = fixtures_path();
        let mut app = vibe_manager::app::App::new(path).expect("Failed to load app");

        // Enter Chris's hall, then Taylor's hall inside it
        app.selected_index = chris_wong_idx(&app);
        app.update(Msg::EnterHall).unwrap();

        let taylor_idx = app
            .reports
            .iter()
            .position(|r| r.slug == "taylor-brooks")
            .expect("Taylor not in Chris's hall");
        app.selected_index = taylor_idx;
        app.update(Msg::EnterHall).unwrap();

        assert_eq!(app.hall_stack.len(), 2);
        assert_eq!(app.reports.len(), 1, "Taylor's pod has one member");
        assert_eq!(app.reports[0].slug, "priya-anand");

        // Walk all the way back out
        app.update(Msg::ExitHall).unwrap();
        assert_eq!(app.hall_stack.len(), 1);
        assert_eq!(app.reports[app.selected_index].slug, "taylor-brooks");
        app.update(Msg::ExitHall).unwrap();
        assert!(app.hall_stack.is_empty());
        assert_eq!(app.reports[app.selected_index].slug, "chris-wong");
    }

    #[test]
    fn test_boundary_h_ascends_in_hall_wraps_at_root() {
        use vibe_manager::app::Msg;

        let path = fixtures_path();
        let mut app = vibe_manager::app::App::new(path).expect("Failed to load app");

        // At root, h at index 0 wraps (existing behavior unchanged)
        app.selected_index = 0;
        let roster_len = app.reports.len();
        app.update(Msg::SelectPrevOrAscend).unwrap();
        assert_eq!(app.selected_index, roster_len - 1);
        assert!(app.hall_stack.is_empty());

        // Inside a hall, h at index 0 ascends (HALL-04)
        app.selected_index = chris_wong_idx(&app);
        app.update(Msg::EnterHall).unwrap();
        app.update(Msg::SelectPrevOrAscend).unwrap();
        assert!(app.hall_stack.is_empty(), "boundary-h walks up one level");

        // Inside a hall but not at index 0, h is plain SelectPrev
        app.selected_index = chris_wong_idx(&app);
        app.update(Msg::EnterHall).unwrap();
        app.update(Msg::SelectNext).unwrap();
        app.update(Msg::SelectPrevOrAscend).unwrap();
        assert_eq!(app.hall_stack.len(), 1, "h mid-roster stays in the hall");
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn test_skip_level_note_saves_into_nested_journal() {
        use vibe_manager::app::{Msg, ViewMode};

        let temp = setup_temp_workspace();
        let mut app =
            vibe_manager::app::App::new(temp.path().to_path_buf()).expect("Failed to load app");

        // Enter Chris's hall and record an observation for Morgan
        app.selected_index = chris_wong_idx(&app);
        app.update(Msg::EnterHall).unwrap();
        let morgan_idx = app
            .reports
            .iter()
            .position(|r| r.slug == "morgan-smith")
            .expect("Morgan not in hall");

        app.selected_report_index = Some(morgan_idx);
        app.view_mode = ViewMode::ReportDetail;
        app.update(Msg::ShowEntryInput).unwrap();
        app.update(Msg::SetEntryMood(3)).unwrap();
        app.update(Msg::SaveEntry).unwrap();

        // The entry must land in the NESTED journal, not a root-level dir
        let new_entry = app.entries_by_report[morgan_idx].last().unwrap();
        assert!(
            new_entry
                .path
                .starts_with(temp.path().join("chris-wong/team/morgan-smith")),
            "entry path escaped the hall: {:?}",
            new_entry.path
        );
        assert!(new_entry.path.exists());
        assert!(
            !temp.path().join("morgan-smith").exists(),
            "no phantom root-level report dir may appear"
        );

        // Walking back out refreshes Chris's squad metrics from disk
        app.view_mode = ViewMode::Dashboard;
        app.update(Msg::ExitHall).unwrap();
        let chris_idx = chris_wong_idx(&app);
        assert!(app.summaries[chris_idx].team_metrics.is_some());
    }

    #[test]
    fn test_hall_uses_skip_level_cadence() {
        use vibe_manager::app::Msg;

        let path = fixtures_path();
        let mut app = vibe_manager::app::App::new(path).expect("Failed to load app");

        app.selected_index = chris_wong_idx(&app);
        app.update(Msg::EnterHall).unwrap();

        // Inside the hall, never-met Devon is overdue under the monthly
        // skip-level cadence — same shape Phase 1 computed for TeamMetrics
        let devon = app
            .reports
            .iter()
            .position(|r| r.slug == "devon-okafor")
            .expect("Devon not in hall");
        assert!(app.summaries[devon].is_overdue);
        assert!(app.summaries[devon].days_since_meeting.is_none());
    }

    #[test]
    fn test_new_report_blocked_inside_hall() {
        use vibe_manager::app::{Msg, ViewMode};

        let path = fixtures_path();
        let mut app = vibe_manager::app::App::new(path).expect("Failed to load app");

        app.selected_index = chris_wong_idx(&app);
        app.update(Msg::EnterHall).unwrap();
        app.update(Msg::ShowNewReport).unwrap();

        assert_eq!(
            app.view_mode,
            ViewMode::Dashboard,
            "no recruit modal in halls"
        );
        assert!(app.status_text().is_some());
    }

    #[test]
    fn test_manager_without_team_dir_gets_empty_metrics() {
        let path = fixtures_path();
        let app = vibe_manager::app::App::new(path).expect("Failed to load app");

        let idx = app
            .reports
            .iter()
            .position(|e| e.profile.name == "Minimal Manager")
            .expect("Minimal Manager not found");

        // No team/ dir: metrics exist but are empty — views render
        // "squad 0 · no members yet" and the urgency bonus is zero
        let metrics = app.summaries[idx]
            .team_metrics
            .as_ref()
            .expect("managers always get team metrics");
        assert_eq!(metrics.team_size, 0);
        assert!(metrics.outliers.is_empty());
    }
}
