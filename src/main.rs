//! Vibe Manager - TUI tool for engineering managers
//!
//! Track 1-on-1s, team health, and career progress with an 8-bit RPG aesthetic.

mod app;
mod components;
mod model;
mod storage;
mod theme;
mod utils;
mod views;

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

use app::{handle_key_event, poll_event, App, ViewMode};
use views::{render_dashboard_view, render_detail_view, render_editor_view};

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
            println!("✓ Initialized Vibe Manager workspace at {:?}", workspace.path);
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
        eprintln!("Run 'vibe-manager init {:?}' to create a new workspace", path);
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
        terminal.draw(|frame| {
            match app.view_mode {
                ViewMode::Dashboard | ViewMode::Help | ViewMode::NewEngineerModal => {
                    render_dashboard_view(app, frame);
                }
                ViewMode::EngineerDetail => {
                    render_detail_view(app, frame);
                }
                ViewMode::NoteEditor => {
                    render_editor_view(app, frame);
                }
            }
        })?;

        // Handle events
        if let Some(event) = poll_event(Duration::from_millis(100))? {
            match event {
                Event::Key(key) => {
                    if let Some(msg) = handle_key_event(app, key) {
                        app.update(msg)?;
                    }
                }
                Event::Resize(_, _) => {
                    // Terminal will redraw automatically
                }
                _ => {}
            }
        }

        // Clear status message after a bit (in real app, use timestamp)
        if app.status_message.is_some() {
            // For now, keep message visible
        }

        if app.should_quit {
            // Auto-save if in editor
            if app.view_mode == ViewMode::NoteEditor {
                app.update(app::Msg::SaveNote)?;
            }
            break;
        }
    }

    Ok(())
}
