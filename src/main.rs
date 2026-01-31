//! Vibe Manager - TUI tool for engineering managers
//!
//! Track 1-on-1s, team health, and career progress with an 8-bit RPG aesthetic.

use std::io;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use vibe_manager::app::{self, handle_key_event, poll_event, App, Effect, ViewMode};
use vibe_manager::editor;
use vibe_manager::storage;
use vibe_manager::views::{render_dashboard_view, render_detail_view, render_viewer_view};

#[derive(Parser)]
#[command(name = "vibe-manager")]
#[command(about = "TUI tool for engineering managers to track 1-on-1s and team health")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Path to workspace directory
    #[arg(default_value = ".")]
    path: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new workspace
    Init {
        /// Path for the new workspace
        #[arg(default_value = ".")]
        path: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init { path }) => {
            init_workspace(&path)?;
        }
        None => {
            run_tui(&cli.path)?;
        }
    }

    Ok(())
}

fn init_workspace(path: &PathBuf) -> Result<()> {
    let abs_path = if path.is_absolute() {
        path.clone()
    } else {
        std::env::current_dir()?.join(path)
    };

    match storage::init_workspace(&abs_path) {
        Ok(workspace) => {
            println!(
                "✓ Initialized Vibe Manager workspace at {:?}",
                workspace.path
            );
            println!();
            println!("Next steps:");
            println!("  1. Run 'vibe-manager {:?}' to open the TUI", path);
            println!("  2. Press 'n' to add your first team member");
            println!("  3. Press '?' for help");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

fn run_tui(path: &PathBuf) -> Result<()> {
    let abs_path = if path.is_absolute() {
        path.clone()
    } else {
        std::env::current_dir()?.join(path)
    };

    // Check if workspace exists
    if !storage::is_workspace(&abs_path) {
        eprintln!("Error: Not a Vibe Manager workspace");
        eprintln!();
        eprintln!(
            "Run 'vibe-manager init {:?}' to create a new workspace",
            path
        );
        std::process::exit(1);
    }

    // Setup terminal
    enable_raw_mode().context("Failed to enable raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .context("Failed to enter alternate screen")?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).context("Failed to create terminal")?;

    // Create app
    let mut app = App::new(abs_path).context("Failed to load workspace")?;

    // Main loop
    let result = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode().context("Failed to disable raw mode")?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .context("Failed to leave alternate screen")?;
    terminal.show_cursor().context("Failed to show cursor")?;

    result
}

fn run_app(terminal: &mut app::Term, app: &mut App) -> Result<()> {
    loop {
        // Render
        terminal.draw(|frame| match app.view_mode {
            ViewMode::Dashboard | ViewMode::Help | ViewMode::NewEngineerModal => {
                render_dashboard_view(app, frame);
            }
            ViewMode::EngineerDetail | ViewMode::EntryInputModal => {
                render_detail_view(app, frame);
            }
            ViewMode::NoteViewer => {
                render_viewer_view(app, frame);
            }
            ViewMode::DeleteConfirmModal => {
                // Render the appropriate view based on where delete was triggered
                if app.delete_from_list {
                    render_detail_view(app, frame);
                } else {
                    render_viewer_view(app, frame);
                }
            }
        })?;

        // Handle events
        if let Some(event) = poll_event(Duration::from_millis(100))? {
            match event {
                Event::Key(key) => {
                    if let Some(msg) = handle_key_event(app, key) {
                        // Process the returned effect from update
                        match app.update(msg)? {
                            Effect::None => {}
                            Effect::SpawnEditor { is_new } => {
                                suspend_and_edit(terminal, app, is_new)?;
                            }
                        }
                    }
                }
                Event::Resize(_, _) => {
                    // Terminal will redraw automatically
                }
                _ => {}
            }
        }

        // Clear expired status messages
        app.clear_expired_status();

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

/// Suspend the TUI and spawn an external editor for the current entry note
///
/// If `is_new` is true, this is a newly created entry. If the user saves an empty
/// file (whitespace only), the entry is canceled and the file is deleted.
fn suspend_and_edit(terminal: &mut app::Term, app: &mut App, is_new: bool) -> Result<()> {
    // Get the entry file path
    let (eng_idx, entry_idx) = match (app.selected_engineer_index, app.selected_entry_index) {
        (Some(e), Some(m)) => (e, m),
        _ => {
            app.set_status("No entry selected");
            return Ok(());
        }
    };

    let entry = &app.entries_by_engineer[eng_idx][entry_idx];
    let file_path = entry.path.clone();

    // Leave alternate screen and disable raw mode
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    disable_raw_mode()?;

    // Spawn external editor
    let result = editor::edit_file(&file_path);

    // Re-enable raw mode and enter alternate screen
    enable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        EnterAlternateScreen,
        EnableMouseCapture
    )?;

    // Clear screen for clean redraw
    terminal.clear()?;

    // Handle editor result
    match result {
        Ok(editor_result) => {
            // Read the current content (may have been modified)
            let raw_content = editor_result
                .content
                .or_else(|| std::fs::read_to_string(&file_path).ok())
                .unwrap_or_default();

            // Check if content is empty/whitespace only (user wants to cancel)
            if raw_content.trim().is_empty() || is_content_empty(&raw_content) {
                // Delete the entry using shared helper
                match app.delete_entry(eng_idx, entry_idx) {
                    Ok(()) => {
                        app.view_mode = app::ViewMode::EngineerDetail;
                        app.set_status(if is_new {
                            "Entry creation canceled"
                        } else {
                            "Entry deleted"
                        });
                    }
                    Err(e) => {
                        app.set_status(format!("Error deleting file: {}", e));
                    }
                }
            } else if editor_result.modified {
                // Parse frontmatter and body separately
                let (frontmatter_yaml, body) = storage::parse_frontmatter(&raw_content);

                // Update frontmatter if present (user may have edited mood in file)
                if let Some(yaml) = frontmatter_yaml {
                    if let Ok(fm) = serde_yaml::from_str(yaml) {
                        app.entries_by_engineer[eng_idx][entry_idx].frontmatter = fm;
                        app.editor_mood =
                            app.entries_by_engineer[eng_idx][entry_idx].frontmatter.mood;
                    }
                }

                // Update with body content only (without frontmatter)
                let body_content = body.to_string();
                app.editor_content = body_content.clone();
                app.entries_by_engineer[eng_idx][entry_idx].content = body_content;
                app.set_status("Note updated");
            } else {
                app.set_status("No changes");
            }
        }
        Err(e) => {
            // If editor failed on a new entry, clean up the file
            if is_new {
                let _ = app.delete_entry(eng_idx, entry_idx);
                app.view_mode = app::ViewMode::EngineerDetail;
            }
            app.set_status(format!("Editor error: {}", e));
        }
    }

    Ok(())
}

/// Check if meeting note content is effectively empty
/// (only contains frontmatter and whitespace/template headers with no actual content)
fn is_content_empty(content: &str) -> bool {
    // Remove YAML frontmatter
    let body = if content.starts_with("---") {
        content.splitn(3, "---").nth(2).unwrap_or("")
    } else {
        content
    };

    // Check if remaining content is just whitespace and empty markdown structure
    body.lines()
        .filter(|line| {
            let trimmed = line.trim();
            // Skip empty lines, headers, and common template markers
            !trimmed.is_empty()
                && !trimmed.starts_with('#')
                && !trimmed.starts_with("- [ ]")
                && trimmed != "-"
        })
        .count()
        == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_content_empty_with_only_whitespace() {
        assert!(is_content_empty(""));
        assert!(is_content_empty("   "));
        assert!(is_content_empty("\n\n\n"));
    }

    #[test]
    fn test_is_content_empty_with_frontmatter_only() {
        let content = "---\nmood: 3\n---\n\n";
        assert!(is_content_empty(content));
    }

    #[test]
    fn test_is_content_empty_with_headers_only() {
        let content = "# Title\n## Section\n### Subsection\n";
        assert!(is_content_empty(content));
    }

    #[test]
    fn test_is_content_empty_with_template_structure() {
        let content = "---\n---\n\n# 1-on-1 - January 22, 2026\n\n## Discussion\n\n## Notes\n\n## Action Items\n- [ ] \n";
        assert!(is_content_empty(content));
    }

    #[test]
    fn test_is_content_empty_with_empty_checkbox() {
        let content = "- [ ] \n- [ ] \n";
        assert!(is_content_empty(content));
    }

    #[test]
    fn test_is_content_empty_with_bare_dash() {
        let content = "# Header\n-\n";
        assert!(is_content_empty(content));
    }

    #[test]
    fn test_is_content_empty_false_with_actual_content() {
        let content = "Some actual notes here";
        assert!(!is_content_empty(content));
    }

    #[test]
    fn test_is_content_empty_false_with_content_after_frontmatter() {
        let content = "---\nmood: 3\n---\n\nDiscussed project timeline.";
        assert!(!is_content_empty(content));
    }

    #[test]
    fn test_is_content_empty_false_with_filled_checkbox() {
        let content = "- [x] Completed task\n";
        assert!(!is_content_empty(content));
    }

    #[test]
    fn test_is_content_empty_false_with_list_items() {
        let content = "# Notes\n- First item\n- Second item\n";
        assert!(!is_content_empty(content));
    }

    #[test]
    fn test_is_content_empty_false_with_content_under_headers() {
        let content = "---\n---\n\n# 1-on-1\n\n## Discussion\n\nWe talked about the roadmap.\n";
        assert!(!is_content_empty(content));
    }
}
