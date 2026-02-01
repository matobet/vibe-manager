//! Integration tests for storage layer using fixtures

use std::path::PathBuf;

use chrono::NaiveDate;
use vibe_manager::model::ReportType;
use vibe_manager::storage::{
    has_team_dir, is_workspace, list_report_dirs, list_team_member_dirs, load_entries, load_report,
    load_report_with_manager, load_workspace,
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
    fn test_list_report_dirs() {
        let workspace = load_workspace(&fixtures_path()).unwrap();
        let dirs = list_report_dirs(&workspace).unwrap();

        // Should find all top-level reports with _profile.md
        assert_eq!(dirs.len(), 5);

        let names: Vec<_> = dirs
            .iter()
            .filter_map(|p| p.file_name())
            .filter_map(|n| n.to_str())
            .collect();

        assert!(names.contains(&"alex-chen"));
        assert!(names.contains(&"jordan-lee"));
        assert!(names.contains(&"jonas"));
        assert!(names.contains(&"chris-wong"));
        assert!(names.contains(&"manager-minimal"));
    }
}

#[cfg(test)]
mod report_tests {
    use super::*;

    #[test]
    fn test_load_report_profile() {
        let report = load_report(&fixtures_path().join("alex-chen")).unwrap();

        assert_eq!(report.slug, "alex-chen");
        assert_eq!(report.profile.name, "Alex Chen");
        assert_eq!(report.profile.title, Some("Software Engineer".to_string()));
        assert_eq!(report.profile.level, Some("P3".to_string()));
        assert_eq!(report.profile.meeting_frequency, "weekly");
        assert!(report.profile.active);
    }

    #[test]
    fn test_load_report_personal_info() {
        let report = load_report(&fixtures_path().join("alex-chen")).unwrap();

        assert_eq!(report.profile.partner, Some("Sarah".to_string()));
        assert_eq!(report.profile.children, vec!["Emma", "Jack"]);
        assert_eq!(
            report.profile.birthday,
            Some(NaiveDate::from_ymd_opt(1992, 5, 20).unwrap())
        );
    }

    #[test]
    fn test_load_report_notes_content() {
        let report = load_report(&fixtures_path().join("alex-chen")).unwrap();

        assert!(report.notes_content.contains("# Alex Chen"));
        assert!(report.notes_content.contains("## Background"));
        assert!(report.notes_content.contains("From Seattle"));
    }

    #[test]
    fn test_load_report_with_minimal_profile() {
        let report = load_report(&fixtures_path().join("jonas")).unwrap();

        assert_eq!(report.profile.name, "Jonas");
        assert_eq!(report.profile.title, None);
        assert_eq!(report.profile.level, Some("P2".to_string()));
        assert!(report.profile.children.is_empty());
    }

    #[test]
    fn test_meeting_frequency_days() {
        let alex = load_report(&fixtures_path().join("alex-chen")).unwrap();
        let jordan = load_report(&fixtures_path().join("jordan-lee")).unwrap();

        assert_eq!(alex.meeting_frequency_days(), 7); // weekly
        assert_eq!(jordan.meeting_frequency_days(), 14); // biweekly
    }
}

#[cfg(test)]
mod meeting_tests {
    use super::*;

    #[test]
    fn test_load_entries_sorted_by_timestamp() {
        let entries = load_entries(&fixtures_path().join("alex-chen")).unwrap();

        // Should have meetings + mood observations
        assert!(entries.len() >= 2);
        // Should be sorted oldest first
        for i in 1..entries.len() {
            assert!(entries[i - 1].timestamp <= entries[i].timestamp);
        }
    }

    #[test]
    fn test_load_entries_only() {
        let entries = load_entries(&fixtures_path().join("alex-chen")).unwrap();
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
        let entries = load_entries(&fixtures_path().join("alex-chen")).unwrap();
        let meetings: Vec<_> = entries.iter().filter(|e| e.is_meeting()).collect();

        assert_eq!(meetings[0].mood(), Some(3)); // Jan 8
        assert_eq!(meetings[1].mood(), Some(5)); // Jan 15
    }

    #[test]
    fn test_load_meeting_content() {
        let entries = load_entries(&fixtures_path().join("alex-chen")).unwrap();
        let meetings: Vec<_> = entries.iter().filter(|e| e.is_meeting()).collect();

        assert!(meetings[1].content.contains("Career goals"));
        assert!(meetings[1].content.contains("tech lead track"));
    }

    #[test]
    fn test_load_mood_observations() {
        let entries = load_entries(&fixtures_path().join("alex-chen")).unwrap();
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
        let entries = load_entries(&fixtures_path().join("jordan-lee")).unwrap();
        let meetings: Vec<_> = entries.iter().filter(|e| e.is_meeting()).collect();

        // Should have at least 3 meetings
        assert!(meetings.len() >= 3);
    }

    #[test]
    fn test_legacy_filename_format() {
        // Legacy files (YYYY-MM-DD.md) should still load
        let entries = load_entries(&fixtures_path().join("alex-chen")).unwrap();
        let legacy: Vec<_> = entries.iter().filter(|e| !e.has_time()).collect();

        // Should have legacy date-only entries
        assert!(!legacy.is_empty());
        for entry in &legacy {
            let filename = entry.path.file_name().unwrap().to_str().unwrap();
            assert!(filename.len() == "YYYY-MM-DD.md".len());
        }
    }
}

#[cfg(test)]
mod manager_tests {
    use super::*;

    #[test]
    fn test_load_manager_profile() {
        let manager = load_report(&fixtures_path().join("chris-wong")).unwrap();

        assert_eq!(manager.slug, "chris-wong");
        assert_eq!(manager.profile.name, "Chris Wong");
        assert_eq!(manager.profile.level, Some("M2".to_string()));
        assert_eq!(manager.profile.report_type, ReportType::Manager);
        assert!(manager.profile.manager_info.is_some());

        let info = manager.profile.manager_info.unwrap();
        assert_eq!(info.team_name, Some("Platform Team".to_string()));
    }

    #[test]
    fn test_manager_has_team_dir() {
        assert!(has_team_dir(&fixtures_path().join("chris-wong")));
        assert!(!has_team_dir(&fixtures_path().join("alex-chen")));
    }

    #[test]
    fn test_list_team_members() {
        let dirs = list_team_member_dirs(&fixtures_path().join("chris-wong")).unwrap();

        assert_eq!(dirs.len(), 3);

        let names: Vec<_> = dirs
            .iter()
            .filter_map(|p| p.file_name())
            .filter_map(|n| n.to_str())
            .collect();

        assert!(names.contains(&"morgan-smith"));
        assert!(names.contains(&"lee-kim"));
        assert!(names.contains(&"robin-patel"));
    }

    #[test]
    fn test_load_team_member_profile() {
        let team_member =
            load_report(&fixtures_path().join("chris-wong/team/robin-patel")).unwrap();

        assert_eq!(team_member.slug, "robin-patel");
        assert_eq!(team_member.profile.name, "Robin Patel");
        assert_eq!(team_member.profile.level, Some("P3".to_string()));
        assert_eq!(team_member.profile.report_type, ReportType::Individual);
    }

    #[test]
    fn test_load_manager_journal_entries() {
        let entries = load_entries(&fixtures_path().join("chris-wong")).unwrap();

        // Manager should have 4 journal entries
        assert_eq!(entries.len(), 4);

        // All should be meetings with mood
        for entry in &entries {
            assert!(entry.is_meeting());
            assert!(entry.mood().is_some());
        }
    }

    #[test]
    fn test_load_skip_level_entries() {
        // Skip-level meetings are in team member's journal folder
        let entries = load_entries(&fixtures_path().join("chris-wong/team/robin-patel")).unwrap();

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].mood(), Some(5));
        assert!(entries[0].content.contains("Staff Engineer track"));
    }

    #[test]
    fn test_team_member_levels() {
        let lee = load_report(&fixtures_path().join("chris-wong/team/lee-kim")).unwrap();
        let morgan = load_report(&fixtures_path().join("chris-wong/team/morgan-smith")).unwrap();
        let robin = load_report(&fixtures_path().join("chris-wong/team/robin-patel")).unwrap();

        assert_eq!(lee.profile.level, Some("P1".to_string()));
        assert_eq!(morgan.profile.level, Some("P2".to_string()));
        assert_eq!(robin.profile.level, Some("P3".to_string()));
    }

    #[test]
    fn test_load_report_with_manager_sets_manager_slug() {
        let team_member = load_report_with_manager(
            &fixtures_path().join("chris-wong/team/robin-patel"),
            "chris-wong",
        )
        .unwrap();

        assert_eq!(team_member.slug, "robin-patel");
        assert_eq!(team_member.manager_slug, Some("chris-wong".to_string()));
    }

    #[test]
    fn test_is_second_level_returns_true_for_team_member() {
        let team_member = load_report_with_manager(
            &fixtures_path().join("chris-wong/team/robin-patel"),
            "chris-wong",
        )
        .unwrap();

        assert!(team_member.is_second_level());
    }

    #[test]
    fn test_is_second_level_returns_false_for_direct_report() {
        let direct_report = load_report(&fixtures_path().join("alex-chen")).unwrap();

        assert!(!direct_report.is_second_level());
    }

    #[test]
    fn test_list_team_members_empty_team_dir() {
        let dirs = list_team_member_dirs(&fixtures_path().join("manager-minimal")).unwrap();

        assert!(dirs.is_empty());
    }

    #[test]
    fn test_manager_without_manager_info() {
        let manager = load_report(&fixtures_path().join("manager-minimal")).unwrap();

        assert_eq!(manager.slug, "manager-minimal");
        assert_eq!(manager.profile.name, "Minimal Manager");
        assert_eq!(manager.profile.level, Some("M1".to_string()));
        assert_eq!(manager.profile.report_type, ReportType::Manager);
        assert!(manager.profile.manager_info.is_none());
    }

    #[test]
    fn test_team_dir_ignores_hidden_directories() {
        // .hidden directory exists in chris-wong/team/ with a valid _profile.md
        // but should be skipped
        let dirs = list_team_member_dirs(&fixtures_path().join("chris-wong")).unwrap();

        let names: Vec<_> = dirs
            .iter()
            .filter_map(|p| p.file_name())
            .filter_map(|n| n.to_str())
            .collect();

        assert!(!names.contains(&".hidden"));
        // Should still have the 3 valid team members
        assert_eq!(dirs.len(), 3);
    }

    #[test]
    fn test_team_dir_ignores_dirs_without_profile() {
        // no-profile directory exists in chris-wong/team/ but has no _profile.md
        let dirs = list_team_member_dirs(&fixtures_path().join("chris-wong")).unwrap();

        let names: Vec<_> = dirs
            .iter()
            .filter_map(|p| p.file_name())
            .filter_map(|n| n.to_str())
            .collect();

        assert!(!names.contains(&"no-profile"));
        // Should still have the 3 valid team members
        assert_eq!(dirs.len(), 3);
    }

    #[test]
    fn test_load_manager_with_full_team() {
        // Load chris-wong and manually populate team (simulating app.rs load_data behavior)
        let manager_path = fixtures_path().join("chris-wong");
        let mut manager = load_report(&manager_path).unwrap();

        // Load team members
        let team_dirs = list_team_member_dirs(&manager_path).unwrap();
        for team_dir in team_dirs {
            if let Ok(team_member) = load_report_with_manager(&team_dir, &manager.slug) {
                manager.team.push(team_member);
            }
        }

        // Verify team is populated
        assert_eq!(manager.team.len(), 3);

        // Verify each team member has correct manager_slug
        for team_member in &manager.team {
            assert_eq!(team_member.manager_slug, Some("chris-wong".to_string()));
            assert!(team_member.is_second_level());
        }

        // Verify team member slugs
        let team_slugs: Vec<_> = manager.team.iter().map(|r| r.slug.as_str()).collect();
        assert!(team_slugs.contains(&"lee-kim"));
        assert!(team_slugs.contains(&"morgan-smith"));
        assert!(team_slugs.contains(&"robin-patel"));
    }
}
