//! Storage layer for Vibe Manager
//!
//! Handles loading and saving data to the filesystem using markdown files
//! with YAML frontmatter.

pub mod meeting;
pub mod profile;
pub mod workspace;

// Journal entry functions
pub use meeting::{create_entry, create_meeting, load_entries, save_entry, update_entry_mood};

// Report functions
pub use profile::{
    archive_report, create_report, load_report, load_report_with_manager, save_report,
};

// Workspace functions
pub use workspace::{
    has_team_dir, init_workspace, is_workspace, list_report_dirs, list_team_member_dirs,
    load_workspace,
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("Invalid workspace: {0}")]
    InvalidWorkspace(String),

    #[error("Profile not found: {0}")]
    ProfileNotFound(String),
}

pub type StorageResult<T> = Result<T, StorageError>;

/// Parse YAML frontmatter from markdown content
/// Returns (frontmatter, body) where frontmatter is the YAML between --- delimiters
pub fn parse_frontmatter(content: &str) -> (Option<&str>, &str) {
    let content = content.trim_start();

    if !content.starts_with("---") {
        return (None, content);
    }

    let after_start = &content[3..];
    if let Some(end_pos) = after_start.find("\n---") {
        let frontmatter = after_start[..end_pos].trim();
        let body = after_start[end_pos + 4..].trim_start();
        (Some(frontmatter), body)
    } else {
        (None, content)
    }
}

/// Generate frontmatter string from serializable data
pub fn generate_frontmatter<T: serde::Serialize>(data: &T) -> StorageResult<String> {
    let yaml = serde_yaml::to_string(data)?;
    Ok(format!("---\n{}---\n\n", yaml))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frontmatter_basic() {
        let content = r#"---
name: Alex Chen
level: P3
---

# Alex Chen

Some notes here.
"#;
        let (fm, body) = parse_frontmatter(content);
        assert!(fm.is_some());
        assert!(fm.unwrap().contains("name: Alex Chen"));
        assert!(body.starts_with("# Alex Chen"));
    }

    #[test]
    fn test_parse_frontmatter_none() {
        let content = "# No frontmatter\n\nJust markdown.";
        let (fm, body) = parse_frontmatter(content);
        assert!(fm.is_none());
        assert_eq!(body, content);
    }

    #[test]
    fn test_parse_frontmatter_empty() {
        let content = "---\n---\n\nBody";
        let (fm, body) = parse_frontmatter(content);
        assert!(fm.is_some());
        assert_eq!(fm.unwrap(), "");
        assert_eq!(body, "Body");
    }
}
