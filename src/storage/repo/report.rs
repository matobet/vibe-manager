//! Report repository
//!
//! High-level interface for report operations.

use std::fs;
use std::path::{Path, PathBuf};

use super::EntryRepository;
use crate::model::{Report, ReportProfile};
use crate::storage::{parse_frontmatter, StorageError, StorageResult};

const PROFILE_FILE: &str = "_profile.md";

/// Repository for report operations
#[derive(Debug, Clone)]
pub struct ReportRepository {
    path: PathBuf,
    manager_slug: Option<String>,
}

impl ReportRepository {
    /// Create a new report repository
    pub fn new(path: PathBuf, manager_slug: Option<String>) -> Self {
        Self { path, manager_slug }
    }

    /// Get the report directory path
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Get the report slug (directory name)
    pub fn slug(&self) -> &str {
        self.path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
    }

    /// Load the report from disk
    pub fn load(&self) -> StorageResult<Report> {
        let profile_path = self.path.join(PROFILE_FILE);

        if !profile_path.exists() {
            return Err(StorageError::ProfileNotFound(format!("{:?}", profile_path)));
        }

        let content = fs::read_to_string(&profile_path)?;
        let (frontmatter, body) = parse_frontmatter(&content);

        let profile: ReportProfile = match frontmatter {
            Some(fm) if !fm.is_empty() => serde_yaml::from_str(fm)?,
            _ => {
                return Err(StorageError::InvalidWorkspace(
                    "Profile missing frontmatter".to_string(),
                ))
            }
        };

        let slug = self.slug().to_string();

        match &self.manager_slug {
            Some(manager) => Ok(Report::new_with_manager(
                slug,
                self.path.clone(),
                profile,
                body.to_string(),
                manager.clone(),
            )),
            None => Ok(Report::new(
                slug,
                self.path.clone(),
                profile,
                body.to_string(),
            )),
        }
    }

    /// Save a report to disk
    pub fn save(&self, report: &Report) -> StorageResult<()> {
        let profile_path = self.path.join(PROFILE_FILE);

        fs::create_dir_all(&self.path)?;

        let yaml = serde_yaml::to_string(&report.profile)?;
        let content = format!("---\n{}---\n\n{}", yaml, report.notes_content);

        fs::write(&profile_path, content)?;
        Ok(())
    }

    /// Check if this report has a team/ subdirectory (is a manager)
    pub fn has_team(&self) -> bool {
        self.path.join("team").is_dir()
    }

    /// List team member repositories (for managers)
    pub fn list_team_members(&self) -> StorageResult<Vec<ReportRepository>> {
        let team_dir = self.path.join("team");
        let mut repos = Vec::new();

        if !team_dir.exists() {
            return Ok(repos);
        }

        let slug = self.slug().to_string();

        for entry in fs::read_dir(&team_dir)? {
            let path = entry?.path();

            if path.is_dir() {
                let name = path.file_name().and_then(|n| n.to_str());

                // Skip hidden directories
                if let Some(name) = name {
                    if !name.starts_with('.') && path.join(PROFILE_FILE).exists() {
                        repos.push(ReportRepository::new(path, Some(slug.clone())));
                    }
                }
            }
        }

        repos.sort_by(|a, b| a.slug().cmp(b.slug()));
        Ok(repos)
    }

    /// Get the entry repository for this report
    pub fn entries(&self) -> EntryRepository {
        EntryRepository::new(self.path.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::ReportType;
    use tempfile::TempDir;

    fn sample_profile() -> ReportProfile {
        ReportProfile {
            name: "Alex Chen".to_string(),
            title: Some("Software Engineer".to_string()),
            start_date: None,
            level: Some("P3".to_string()),
            meeting_frequency: "weekly".to_string(),
            active: true,
            report_type: ReportType::Individual,
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
    fn test_save_and_load_report() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("alex-chen");

        let profile = sample_profile();
        let report = Report::new(
            "alex-chen".to_string(),
            path.clone(),
            profile,
            "# Alex Chen\n\nNotes here.".to_string(),
        );

        let repo = ReportRepository::new(path.clone(), None);
        repo.save(&report).unwrap();

        let loaded = repo.load().unwrap();
        assert_eq!(loaded.slug, "alex-chen");
        assert_eq!(loaded.profile.name, "Alex Chen");
        assert_eq!(loaded.profile.level, Some("P3".to_string()));
    }

    #[test]
    fn test_fixture_family_info() {
        use std::path::PathBuf;
        let fixture_path = PathBuf::from("tests/fixtures/alex-chen");
        let repo = ReportRepository::new(fixture_path, None);
        let report = repo.load().unwrap();

        assert_eq!(report.profile.name, "Alex Chen");
        assert_eq!(report.profile.partner, Some("Sarah".to_string()));
        assert_eq!(report.profile.children, vec!["Emma", "Jack"]);
    }
}
