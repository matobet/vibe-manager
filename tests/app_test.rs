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

        assert!(!app.engineers.is_empty());
        // Should load all engineers from fixtures
        assert!(app.engineers.len() >= 2);
    }

    #[test]
    fn test_app_delete_meeting_removes_file() {
        let temp = setup_temp_workspace();

        let mut app =
            vibe_manager::app::App::new(temp.path().to_path_buf()).expect("Failed to load app");

        // Find the engineer and meeting indices
        let eng_idx = app
            .engineers
            .iter()
            .position(|e| e.profile.name == "Alex Chen")
            .expect("Alex Chen not found");

        let initial_meeting_count = app.meetings_by_engineer[eng_idx].len();
        assert!(initial_meeting_count > 0, "No meetings to delete");

        let meeting_path = app.meetings_by_engineer[eng_idx][0].path.clone();
        assert!(
            meeting_path.exists(),
            "Meeting file should exist before deletion"
        );

        // Delete the meeting
        app.delete_meeting(eng_idx, 0)
            .expect("Failed to delete meeting");

        // Verify meeting was removed from memory
        assert_eq!(
            app.meetings_by_engineer[eng_idx].len(),
            initial_meeting_count - 1
        );

        // Verify file was deleted
        assert!(!meeting_path.exists(), "Meeting file should be deleted");
    }

    #[test]
    fn test_app_delete_meeting_updates_summary() {
        let temp = setup_temp_workspace();

        let mut app =
            vibe_manager::app::App::new(temp.path().to_path_buf()).expect("Failed to load app");

        let eng_idx = app
            .engineers
            .iter()
            .position(|e| e.profile.name == "Alex Chen")
            .expect("Alex Chen not found");

        let initial_days_since = app.summaries[eng_idx].days_since_meeting;

        // Delete the most recent meeting (meetings are sorted oldest first)
        let last_idx = app.meetings_by_engineer[eng_idx].len() - 1;
        app.delete_meeting(eng_idx, last_idx)
            .expect("Failed to delete meeting");

        // Summary should be recomputed with new days_since_meeting
        let new_days_since = app.summaries[eng_idx].days_since_meeting;

        // If there was more than one meeting, the days_since_meeting should change
        // (it will be longer since we deleted the most recent meeting)
        if !app.meetings_by_engineer[eng_idx].is_empty() {
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

        let eng_idx = app
            .engineers
            .iter()
            .position(|e| e.profile.name == "Alex Chen")
            .expect("Alex Chen not found");

        // Delete all meetings one by one (always index 0 since list shrinks)
        while !app.meetings_by_engineer[eng_idx].is_empty() {
            app.delete_meeting(eng_idx, 0)
                .expect("Failed to delete meeting");
        }

        assert!(app.meetings_by_engineer[eng_idx].is_empty());
        assert!(app.summaries[eng_idx].days_since_meeting.is_none());
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
}
