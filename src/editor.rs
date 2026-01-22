//! External editor delegation module
//!
//! Follows the UNIX philosophy of delegating text editing to purpose-built tools.
//! Similar to how `git commit` spawns $EDITOR for commit message editing.

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::SystemTime;

use anyhow::{Context, Result};

/// Result of editing a file with external editor
#[derive(Debug)]
pub struct EditorResult {
    /// Whether the file was modified
    pub modified: bool,
    /// New content of the file (if modified)
    pub content: Option<String>,
}

/// Get the user's preferred editor from environment variables
///
/// Checks in order: $EDITOR → $VISUAL → fallback to common editors
pub fn get_editor() -> String {
    // First check $EDITOR (standard for line-based editors)
    if let Ok(editor) = env::var("EDITOR") {
        if !editor.is_empty() {
            return editor;
        }
    }

    // Then check $VISUAL (standard for full-screen editors)
    if let Ok(visual) = env::var("VISUAL") {
        if !visual.is_empty() {
            return visual;
        }
    }

    // Fallback to common editors in order of likelihood
    // nano is often available and user-friendly
    // vi is almost always available as a last resort
    for editor in &["nano", "vim", "vi"] {
        if which_exists(editor) {
            return (*editor).to_string();
        }
    }

    // Ultimate fallback
    "vi".to_string()
}

/// Check if a command exists in PATH
fn which_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Open a file in the external editor and wait for completion
///
/// Returns information about whether the file was modified.
pub fn edit_file(path: &Path) -> Result<EditorResult> {
    let editor = get_editor();
    let mtime_before = get_mtime(path)?;

    // Spawn the editor
    // Split editor string to handle cases like "code --wait" or "vim -u NONE"
    let mut parts = editor.split_whitespace();
    let program = parts.next().context("Empty editor command")?;
    let args: Vec<&str> = parts.collect();

    let mut cmd = Command::new(program);
    cmd.args(&args);
    cmd.arg(path);

    // Spawn and wait for the editor to exit
    let status = cmd
        .status()
        .with_context(|| format!("Failed to spawn editor: {}", editor))?;

    if !status.success() {
        anyhow::bail!("Editor exited with error: {:?}", status.code());
    }

    // Check if file was modified
    let mtime_after = get_mtime(path)?;
    let modified = mtime_after != mtime_before;

    let content = if modified {
        Some(fs::read_to_string(path).context("Failed to read file after editing")?)
    } else {
        None
    };

    Ok(EditorResult { modified, content })
}

/// Get file modification time
fn get_mtime(path: &Path) -> Result<SystemTime> {
    fs::metadata(path)
        .and_then(|m| m.modified())
        .with_context(|| format!("Failed to get mtime for {:?}", path))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_get_editor_from_env() {
        // Save original values
        let orig_editor = env::var("EDITOR").ok();
        let orig_visual = env::var("VISUAL").ok();

        // Test $EDITOR takes precedence
        env::set_var("EDITOR", "test-editor");
        env::set_var("VISUAL", "test-visual");
        assert_eq!(get_editor(), "test-editor");

        // Test $VISUAL is used when $EDITOR is empty
        env::set_var("EDITOR", "");
        assert_eq!(get_editor(), "test-visual");

        // Restore original values
        match orig_editor {
            Some(v) => env::set_var("EDITOR", v),
            None => env::remove_var("EDITOR"),
        }
        match orig_visual {
            Some(v) => env::set_var("VISUAL", v),
            None => env::remove_var("VISUAL"),
        }
    }
}
