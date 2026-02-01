use std::fs;
use std::path::Path;

use super::{parse_frontmatter, StorageError, StorageResult};
use crate::model::{Report, ReportProfile};

const PROFILE_FILE: &str = "_profile.md";

/// Load report profile from directory
pub fn load_report(dir: &Path) -> StorageResult<Report> {
    let profile_path = dir.join(PROFILE_FILE);

    if !profile_path.exists() {
        return Err(StorageError::ProfileNotFound(format!("{:?}", profile_path)));
    }

    let content = fs::read_to_string(&profile_path)?;
    let (frontmatter, body) = parse_frontmatter(&content);

    let profile: ReportProfile = match frontmatter {
        Some(fm) => serde_yaml::from_str(fm)?,
        None => {
            return Err(StorageError::InvalidWorkspace(
                "Profile missing frontmatter".to_string(),
            ))
        }
    };

    let slug = dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    Ok(Report::new(
        slug,
        dir.to_path_buf(),
        profile,
        body.to_string(),
    ))
}

/// Load report profile from directory with manager context (for 2nd-level reports)
pub fn load_report_with_manager(dir: &Path, manager_slug: &str) -> StorageResult<Report> {
    let profile_path = dir.join(PROFILE_FILE);

    if !profile_path.exists() {
        return Err(StorageError::ProfileNotFound(format!("{:?}", profile_path)));
    }

    let content = fs::read_to_string(&profile_path)?;
    let (frontmatter, body) = parse_frontmatter(&content);

    let profile: ReportProfile = match frontmatter {
        Some(fm) => serde_yaml::from_str(fm)?,
        None => {
            return Err(StorageError::InvalidWorkspace(
                "Profile missing frontmatter".to_string(),
            ))
        }
    };

    let slug = dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    Ok(Report::new_with_manager(
        slug,
        dir.to_path_buf(),
        profile,
        body.to_string(),
        manager_slug.to_string(),
    ))
}

/// Save report profile
pub fn save_report(report: &Report) -> StorageResult<()> {
    let profile_path = report.path.join(PROFILE_FILE);

    // Ensure directory exists
    fs::create_dir_all(&report.path)?;

    let yaml = serde_yaml::to_string(&report.profile)?;
    let content = format!("---\n{}---\n\n{}", yaml, report.notes_content);

    fs::write(&profile_path, content)?;
    Ok(())
}

/// Create new report directory and profile
pub fn create_report(
    workspace_path: &Path,
    name: &str,
    profile: ReportProfile,
) -> StorageResult<Report> {
    let slug = crate::utils::name_to_slug(name);
    let dir = workspace_path.join(&slug);

    if dir.exists() {
        return Err(StorageError::InvalidWorkspace(format!(
            "Report directory already exists: {}",
            slug
        )));
    }

    fs::create_dir_all(&dir)?;

    // Create team/ subdirectory for managers
    if profile.report_type.is_manager() {
        fs::create_dir_all(dir.join("team"))?;
    }

    let notes_content = format!(
        "# {}\n\n## Background\n\n## Working Style\n\n## Notes\n",
        name
    );
    let report = Report::new(slug, dir, profile, notes_content);

    save_report(&report)?;
    Ok(report)
}

/// Archive report (set active=false)
pub fn archive_report(report: &mut Report) -> StorageResult<()> {
    report.profile.active = false;
    save_report(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn sample_profile() -> ReportProfile {
        ReportProfile {
            name: "Alex Chen".to_string(),
            title: Some("Software Engineer".to_string()),
            start_date: None,
            level: Some("P3".to_string()),
            meeting_frequency: "weekly".to_string(),
            active: true,
            report_type: crate::model::ReportType::Individual,
            manager_info: None,
            birthday: None,
            partner: None,
            children: vec![],
            skills: None,
            skills_updated: None,
            color: None,
        }
    }

    #[test]
    fn test_create_and_load_report() {
        let temp = TempDir::new().unwrap();

        let profile = sample_profile();
        let created = create_report(temp.path(), "Alex Chen", profile).unwrap();

        assert_eq!(created.slug, "alex-chen");
        assert_eq!(created.profile.name, "Alex Chen");

        let loaded = load_report(&created.path).unwrap();
        assert_eq!(loaded.profile.name, "Alex Chen");
        assert_eq!(loaded.profile.level, Some("P3".to_string()));
    }

    #[test]
    fn test_fixture_family_info() {
        use std::path::PathBuf;
        let fixture_path = PathBuf::from("tests/fixtures/alex-chen");
        let report = load_report(&fixture_path).unwrap();

        assert_eq!(report.profile.name, "Alex Chen");
        assert_eq!(report.profile.partner, Some("Sarah".to_string()));
        assert_eq!(report.profile.children, vec!["Emma", "Jack"]);
    }

    #[test]
    fn test_create_manager_creates_team_dir() {
        let temp = TempDir::new().unwrap();

        let mut profile = sample_profile();
        profile.name = "Chris Wong".to_string();
        profile.level = Some("M2".to_string());
        profile.report_type = crate::model::ReportType::Manager;
        profile.manager_info = Some(crate::model::ManagerInfo {
            team_name: Some("Platform Team".to_string()),
        });

        let created = create_report(temp.path(), "Chris Wong", profile).unwrap();

        assert_eq!(created.slug, "chris-wong");
        assert!(created.profile.report_type.is_manager());

        // Verify team directory was created
        let team_dir = created.path.join("team");
        assert!(team_dir.exists());
        assert!(team_dir.is_dir());
    }
}
