//! Workspace repository
//!
//! High-level interface for workspace operations.

use std::fs;
use std::path::{Path, PathBuf};

use super::ReportRepository;
use crate::model::{Report, ReportProfile, Workspace, WorkspaceConfig};
use crate::storage::{StorageError, StorageResult};
use crate::utils::name_to_slug;

/// Name of the workspace configuration file
const WORKSPACE_FILE: &str = ".vibe-manager";

/// Repository for workspace operations
#[derive(Debug, Clone)]
pub struct WorkspaceRepository {
    path: PathBuf,
}

impl WorkspaceRepository {
    /// Open an existing workspace
    pub fn open(path: impl Into<PathBuf>) -> StorageResult<Self> {
        let path = path.into();
        if !Self::is_valid(&path) {
            return Err(StorageError::InvalidWorkspace(format!(
                "No {} file found in {:?}",
                WORKSPACE_FILE, path
            )));
        }
        Ok(Self { path })
    }

    /// Initialize a new workspace
    pub fn init(path: impl Into<PathBuf>) -> StorageResult<Self> {
        let path = path.into();
        let config_path = path.join(WORKSPACE_FILE);

        if config_path.exists() {
            return Err(StorageError::InvalidWorkspace(
                "Workspace already exists".to_string(),
            ));
        }

        fs::create_dir_all(&path)?;

        let config = WorkspaceConfig::default();
        let content = format!(
            "# Vibe Manager workspace\n\
             version: {}\n\n\
             settings:\n  \
             default_meeting_frequency: {}\n  \
             overdue_threshold_days: {}\n",
            config.version,
            config.settings.default_meeting_frequency,
            config.settings.overdue_threshold_days
        );

        fs::write(&config_path, content)?;

        Ok(Self { path })
    }

    /// Check if a path is a valid workspace
    pub fn is_valid(path: &Path) -> bool {
        path.join(WORKSPACE_FILE).exists()
    }

    /// Load the workspace configuration and metadata
    pub fn load(&self) -> StorageResult<Workspace> {
        let config_path = self.path.join(WORKSPACE_FILE);

        if !config_path.exists() {
            return Err(StorageError::InvalidWorkspace(format!(
                "No {} file found in {:?}",
                WORKSPACE_FILE, self.path
            )));
        }

        let content = fs::read_to_string(&config_path)?;
        let config: WorkspaceConfig = if content.trim().is_empty() {
            WorkspaceConfig::default()
        } else {
            serde_yaml::from_str(&content)?
        };

        Ok(Workspace::new(self.path.clone(), config))
    }

    /// Get the workspace path
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Get a repository for a specific report by slug
    pub fn report(&self, slug: &str) -> ReportRepository {
        let path = self.path.join(slug);
        ReportRepository::new(path, None)
    }

    /// List all direct report repositories in the workspace
    pub fn list_reports(&self) -> StorageResult<Vec<ReportRepository>> {
        let mut repos = Vec::new();

        for entry in fs::read_dir(&self.path)? {
            let path = entry?.path();

            if path.is_dir() {
                let name = path.file_name().and_then(|n| n.to_str());

                // Skip hidden directories and special folders
                if let Some(name) = name {
                    if !name.starts_with('.') && path.join("_profile.md").exists() {
                        repos.push(ReportRepository::new(path, None));
                    }
                }
            }
        }

        repos.sort_by(|a, b| a.slug().cmp(b.slug()));
        Ok(repos)
    }

    /// Create a new report in the workspace
    pub fn create_report(
        &self,
        name: &str,
        profile: ReportProfile,
    ) -> StorageResult<ReportRepository> {
        let slug = name_to_slug(name);
        let dir = self.path.join(&slug);

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
        let report = Report::new(slug, dir.clone(), profile, notes_content);

        let repo = ReportRepository::new(dir, None);
        repo.save(&report)?;
        Ok(repo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_init_and_load_workspace() {
        let temp = TempDir::new().unwrap();
        let path = temp.path();

        let repo = WorkspaceRepository::init(path).unwrap();
        assert!(WorkspaceRepository::is_valid(path));

        let workspace = repo.load().unwrap();
        assert_eq!(workspace.config.version, 1);
    }

    #[test]
    fn test_init_existing_fails() {
        let temp = TempDir::new().unwrap();
        let path = temp.path();

        WorkspaceRepository::init(path).unwrap();
        let result = WorkspaceRepository::init(path);
        assert!(result.is_err());
    }
}
