//! Integration tests for storage layer using fixtures

use std::path::PathBuf;

use chrono::NaiveDate;
use vibe_manager::storage::{
    is_workspace, list_engineer_dirs, load_engineer, load_meetings, load_workspace,
};

fn fixtures_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

#[cfg(test)]
mod workspace_tests {
    use super::*;

    #[test]
    fn test_is_workspace() {
        assert!(is_workspace(&fixtures_path()));
        assert!(!is_workspace(&fixtures_path().join("alex-chen")));
    }

    #[test]
    fn test_load_workspace_config() {
        let workspace = load_workspace(&fixtures_path()).unwrap();

        assert_eq!(workspace.config.version, 1);
        assert_eq!(
            workspace.config.settings.default_meeting_frequency,
            "biweekly"
        );
        assert_eq!(workspace.config.settings.overdue_threshold_days, 3);
    }

    #[test]
    fn test_list_engineer_dirs() {
        let workspace = load_workspace(&fixtures_path()).unwrap();
        let dirs = list_engineer_dirs(&workspace).unwrap();

        // Should find all engineers with _profile.md
        assert_eq!(dirs.len(), 3);

        let names: Vec<_> = dirs
            .iter()
            .filter_map(|p| p.file_name())
            .filter_map(|n| n.to_str())
            .collect();

        assert!(names.contains(&"alex-chen"));
        assert!(names.contains(&"jordan-lee"));
        assert!(names.contains(&"jonas"));
    }
}

#[cfg(test)]
mod engineer_tests {
    use super::*;

    #[test]
    fn test_load_engineer_profile() {
        let engineer = load_engineer(&fixtures_path().join("alex-chen")).unwrap();

        assert_eq!(engineer.slug, "alex-chen");
        assert_eq!(engineer.profile.name, "Alex Chen");
        assert_eq!(
            engineer.profile.title,
            Some("Software Engineer".to_string())
        );
        assert_eq!(engineer.profile.level, Some("P3".to_string()));
        assert_eq!(engineer.profile.meeting_frequency, "weekly");
        assert!(engineer.profile.active);
    }

    #[test]
    fn test_load_engineer_personal_info() {
        let engineer = load_engineer(&fixtures_path().join("alex-chen")).unwrap();

        assert_eq!(engineer.profile.partner, Some("Sarah".to_string()));
        assert_eq!(engineer.profile.children, vec!["Emma", "Jack"]);
        assert_eq!(
            engineer.profile.birthday,
            Some(NaiveDate::from_ymd_opt(1992, 5, 20).unwrap())
        );
    }

    #[test]
    fn test_load_engineer_notes_content() {
        let engineer = load_engineer(&fixtures_path().join("alex-chen")).unwrap();

        assert!(engineer.notes_content.contains("# Alex Chen"));
        assert!(engineer.notes_content.contains("## Background"));
        assert!(engineer.notes_content.contains("From Seattle"));
    }

    #[test]
    fn test_load_engineer_with_minimal_profile() {
        let engineer = load_engineer(&fixtures_path().join("jonas")).unwrap();

        assert_eq!(engineer.profile.name, "Jonas");
        assert_eq!(engineer.profile.title, None);
        assert_eq!(engineer.profile.level, Some("P2".to_string()));
        assert!(engineer.profile.children.is_empty());
    }

    #[test]
    fn test_meeting_frequency_days() {
        let alex = load_engineer(&fixtures_path().join("alex-chen")).unwrap();
        let jordan = load_engineer(&fixtures_path().join("jordan-lee")).unwrap();

        assert_eq!(alex.meeting_frequency_days(), 7); // weekly
        assert_eq!(jordan.meeting_frequency_days(), 14); // biweekly
    }
}

#[cfg(test)]
mod meeting_tests {
    use super::*;

    #[test]
    fn test_load_entries_sorted_by_timestamp() {
        let entries = load_meetings(&fixtures_path().join("alex-chen")).unwrap();

        // Should have meetings + mood observations
        assert!(entries.len() >= 2);
        // Should be sorted oldest first
        for i in 1..entries.len() {
            assert!(entries[i - 1].timestamp <= entries[i].timestamp);
        }
    }

    #[test]
    fn test_load_meetings_only() {
        let entries = load_meetings(&fixtures_path().join("alex-chen")).unwrap();
        let meetings: Vec<_> = entries.iter().filter(|e| e.is_meeting()).collect();

        // Should have at least the 2 original meetings
        assert!(meetings.len() >= 2);
        assert_eq!(
            meetings[0].date(),
            NaiveDate::from_ymd_opt(2026, 1, 8).unwrap()
        );
    }

    #[test]
    fn test_load_meeting_mood() {
        let entries = load_meetings(&fixtures_path().join("alex-chen")).unwrap();
        let meetings: Vec<_> = entries.iter().filter(|e| e.is_meeting()).collect();

        assert_eq!(meetings[0].mood(), Some(3)); // Jan 8
        assert_eq!(meetings[1].mood(), Some(5)); // Jan 15
    }

    #[test]
    fn test_load_meeting_content() {
        let entries = load_meetings(&fixtures_path().join("alex-chen")).unwrap();
        let meetings: Vec<_> = entries.iter().filter(|e| e.is_meeting()).collect();

        assert!(meetings[1].content.contains("Career goals"));
        assert!(meetings[1].content.contains("tech lead track"));
    }

    #[test]
    fn test_load_mood_observations() {
        let entries = load_meetings(&fixtures_path().join("alex-chen")).unwrap();
        let observations: Vec<_> = entries.iter().filter(|e| !e.is_meeting()).collect();

        // Should have mood observations (pure mood entries without content)
        assert!(!observations.is_empty());
        // All mood observations should have a mood set
        for obs in &observations {
            assert!(obs.mood().is_some());
        }
    }

    #[test]
    fn test_load_multiple_entries() {
        let entries = load_meetings(&fixtures_path().join("jordan-lee")).unwrap();
        let meetings: Vec<_> = entries.iter().filter(|e| e.is_meeting()).collect();

        // Should have at least 3 meetings
        assert!(meetings.len() >= 3);
    }

    #[test]
    fn test_legacy_filename_format() {
        // Legacy files (YYYY-MM-DD.md) should still load
        let entries = load_meetings(&fixtures_path().join("alex-chen")).unwrap();
        let legacy: Vec<_> = entries.iter().filter(|e| !e.has_time()).collect();

        // Should have legacy date-only entries
        assert!(!legacy.is_empty());
        for entry in &legacy {
            let filename = entry.path.file_name().unwrap().to_str().unwrap();
            assert!(filename.len() == "YYYY-MM-DD.md".len());
        }
    }
}
