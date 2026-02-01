use std::fs;
use std::path::{Path, PathBuf};

use super::{StorageError, StorageResult};
use crate::model::{Workspace, WorkspaceConfig};

const WORKSPACE_FILE: &str = ".vibe-manager";

/// Check if a directory is a valid workspace
pub fn is_workspace(path: &Path) -> bool {
    path.join(WORKSPACE_FILE).exists()
}

/// Find workspace root by walking up the directory tree
pub fn find_workspace(start: &Path) -> Option<PathBuf> {
    let mut current = start.to_path_buf();

    loop {
        if is_workspace(&current) {
            return Some(current);
        }

        if !current.pop() {
            return None;
        }
    }
}

/// Load workspace configuration
pub fn load_workspace(path: &Path) -> StorageResult<Workspace> {
    let config_path = path.join(WORKSPACE_FILE);

    if !config_path.exists() {
        return Err(StorageError::InvalidWorkspace(format!(
            "No {} file found in {:?}",
            WORKSPACE_FILE, path
        )));
    }

    let content = fs::read_to_string(&config_path)?;
    let config: WorkspaceConfig = if content.trim().is_empty() {
        WorkspaceConfig::default()
    } else {
        serde_yaml::from_str(&content)?
    };

    Ok(Workspace::new(path.to_path_buf(), config))
}

/// Initialize a new workspace
pub fn init_workspace(path: &Path) -> StorageResult<Workspace> {
    let config_path = path.join(WORKSPACE_FILE);

    if config_path.exists() {
        return Err(StorageError::InvalidWorkspace(
            "Workspace already exists".to_string(),
        ));
    }

    // Create directory if needed
    fs::create_dir_all(path)?;

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

    Ok(Workspace::new(path.to_path_buf(), config))
}

/// Save workspace configuration
pub fn save_workspace(workspace: &Workspace) -> StorageResult<()> {
    let config_path = workspace.path.join(WORKSPACE_FILE);
    let content = format!(
        "# Vibe Manager workspace\n\
         version: {}\n\n\
         settings:\n  \
         default_meeting_frequency: {}\n  \
         overdue_threshold_days: {}\n",
        workspace.config.version,
        workspace.config.settings.default_meeting_frequency,
        workspace.config.settings.overdue_threshold_days
    );
    fs::write(&config_path, content)?;
    Ok(())
}

/// List all report directories in workspace (direct reports only)
pub fn list_report_dirs(workspace: &Workspace) -> StorageResult<Vec<PathBuf>> {
    let mut dirs = Vec::new();

    for entry in fs::read_dir(&workspace.path)? {
        let path = entry?.path();

        if path.is_dir() {
            let name = path.file_name().and_then(|n| n.to_str());

            // Skip hidden directories and special folders
            if let Some(name) = name {
                if !name.starts_with('.') {
                    // Check if it has a _profile.md
                    if path.join("_profile.md").exists() {
                        dirs.push(path);
                    }
                }
            }
        }
    }

    dirs.sort();
    Ok(dirs)
}

/// List 2nd-level report directories under a manager's team/ folder
pub fn list_team_member_dirs(manager_path: &Path) -> StorageResult<Vec<PathBuf>> {
    let team_dir = manager_path.join("team");
    let mut dirs = Vec::new();

    if !team_dir.exists() {
        return Ok(dirs);
    }

    for entry in fs::read_dir(&team_dir)? {
        let path = entry?.path();

        if path.is_dir() {
            let name = path.file_name().and_then(|n| n.to_str());

            // Skip hidden directories
            if let Some(name) = name {
                if !name.starts_with('.') {
                    // Check if it has a _profile.md
                    if path.join("_profile.md").exists() {
                        dirs.push(path);
                    }
                }
            }
        }
    }

    dirs.sort();
    Ok(dirs)
}

/// Check if a report directory has a team/ subdirectory (indicating a manager)
pub fn has_team_dir(report_path: &Path) -> bool {
    report_path.join("team").is_dir()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_init_and_load_workspace() {
        let temp = TempDir::new().unwrap();
        let path = temp.path();

        // Initialize
        let workspace = init_workspace(path).unwrap();
        assert_eq!(workspace.config.version, 1);
        assert!(is_workspace(path));

        // Load
        let loaded = load_workspace(path).unwrap();
        assert_eq!(loaded.config.version, 1);
    }

    #[test]
    fn test_init_existing_fails() {
        let temp = TempDir::new().unwrap();
        let path = temp.path();

        init_workspace(path).unwrap();
        let result = init_workspace(path);
        assert!(result.is_err());
    }
}
