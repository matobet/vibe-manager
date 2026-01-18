use std::fs;
use std::path::Path;

use crate::model::{Engineer, EngineerProfile};
use super::{parse_frontmatter, StorageError, StorageResult};

const PROFILE_FILE: &str = "_profile.md";

/// Load engineer profile from directory
pub fn load_engineer(dir: &Path) -> StorageResult<Engineer> {
    let profile_path = dir.join(PROFILE_FILE);

    if !profile_path.exists() {
        return Err(StorageError::ProfileNotFound(
            format!("{:?}", profile_path)
        ));
    }

    let content = fs::read_to_string(&profile_path)?;
    let (frontmatter, body) = parse_frontmatter(&content);

    let profile: EngineerProfile = match frontmatter {
        Some(fm) => serde_yaml::from_str(fm)?,
        None => return Err(StorageError::InvalidWorkspace(
            "Profile missing frontmatter".to_string()
        )),
    };

    let slug = dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    Ok(Engineer::new(slug, dir.to_path_buf(), profile, body.to_string()))
}

/// Save engineer profile
pub fn save_engineer(engineer: &Engineer) -> StorageResult<()> {
    let profile_path = engineer.path.join(PROFILE_FILE);

    // Ensure directory exists
    fs::create_dir_all(&engineer.path)?;

    let yaml = serde_yaml::to_string(&engineer.profile)?;
    let content = format!("---\n{}---\n\n{}", yaml, engineer.notes_content);

    fs::write(&profile_path, content)?;
    Ok(())
}

/// Create new engineer directory and profile
pub fn create_engineer(workspace_path: &Path, name: &str, profile: EngineerProfile) -> StorageResult<Engineer> {
    let slug = crate::utils::name_to_slug(name);
    let dir = workspace_path.join(&slug);

    if dir.exists() {
        return Err(StorageError::InvalidWorkspace(
            format!("Engineer directory already exists: {}", slug)
        ));
    }

    fs::create_dir_all(&dir)?;

    let notes_content = format!("# {}\n\n## Background\n\n## Working Style\n\n## Notes\n", name);
    let engineer = Engineer::new(slug, dir, profile, notes_content);

    save_engineer(&engineer)?;
    Ok(engineer)
}

/// Archive engineer (set active=false)
pub fn archive_engineer(engineer: &mut Engineer) -> StorageResult<()> {
    engineer.profile.active = false;
    save_engineer(engineer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn sample_profile() -> EngineerProfile {
        EngineerProfile {
            name: "Alex Chen".to_string(),
            title: Some("Software Engineer".to_string()),
            start_date: None,
            level: Some("P3".to_string()),
            meeting_frequency: "weekly".to_string(),
            active: true,
            birthday: None,
            partner: None,
            children: vec![],
            skills: None,
            skills_updated: None,
            color: None,
        }
    }

    #[test]
    fn test_create_and_load_engineer() {
        let temp = TempDir::new().unwrap();

        let profile = sample_profile();
        let created = create_engineer(temp.path(), "Alex Chen", profile).unwrap();

        assert_eq!(created.slug, "alex-chen");
        assert_eq!(created.profile.name, "Alex Chen");

        let loaded = load_engineer(&created.path).unwrap();
        assert_eq!(loaded.profile.name, "Alex Chen");
        assert_eq!(loaded.profile.level, Some("P3".to_string()));
    }

    #[test]
    fn test_fixture_family_info() {
        use std::path::PathBuf;
        let fixture_path = PathBuf::from("tests/fixtures/alex-chen");
        let engineer = load_engineer(&fixture_path).unwrap();

        assert_eq!(engineer.profile.name, "Alex Chen");
        assert_eq!(engineer.profile.partner, Some("Sarah".to_string()));
        assert_eq!(engineer.profile.children, vec!["Emma", "Jack"]);
    }
}
