//! Application state and TEA (The Elm Architecture) runtime
//!
//! This module implements the core application logic using The Elm Architecture:
//! - **Model**: The `App` struct holds all application state
//! - **Update**: `App::update(msg)` processes messages and returns effects
//! - **View**: Rendering is handled by the `views` module
//!
//! The TEA pattern keeps the application pure - side effects are returned as
//! `Effect` values that the runtime (main.rs) executes.

mod input;
mod state;
mod update;

use std::time::Instant;

use ratatui::{backend::CrosstermBackend, Terminal};

use crate::components::modal::NewReportState;
use crate::model::{Context, JournalEntry, Report, ReportSummary, Workspace, WorkspaceSummary};

// Re-export public API
pub use input::{handle_key_event, poll_event};

/// Status message display duration
pub(crate) const STATUS_MESSAGE_DURATION: std::time::Duration = std::time::Duration::from_secs(3);

/// Side effects that the update function can request.
///
/// This keeps the TEA pattern pure - update() returns what should happen,
/// the runtime (main.rs) executes the effects.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Effect {
    /// No side effect needed
    None,
    /// Spawn external editor for current meeting
    SpawnEditor { is_new: bool },
}

/// Application view modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    /// Main dashboard showing all reports
    Dashboard,
    /// Detailed view of a single report
    ReportDetail,
    /// Viewing meeting notes
    NoteViewer,
    /// Modal for creating a new report
    NewReportModal,
    /// Modal for confirming deletion
    DeleteConfirmModal,
    /// Modal for quick entry input (mood observation)
    EntryInputModal,
    /// Help overlay
    Help,
}

/// Messages that can be sent to update application state (TEA pattern)
#[derive(Debug, Clone)]
pub enum Msg {
    // Navigation
    /// Quit the application
    Quit,
    /// Go back to previous view
    Back,
    /// Show help overlay
    ShowHelp,
    /// Hide help overlay
    HideHelp,

    // Dashboard actions
    /// Select next item in list
    SelectNext,
    /// Select previous item in list
    SelectPrev,
    /// Jump to first item
    SelectFirst,
    /// Jump to last item
    SelectLast,
    /// View the selected report
    ViewReport,

    // Report detail actions
    /// View a specific meeting by display index
    ViewMeeting(usize),
    /// Create a new meeting
    NewMeeting,

    // Note viewer actions
    /// Edit the current meeting in external editor
    EditMeeting,
    /// Edit meeting directly from list (index is display index, newest first)
    EditMeetingFromList(usize),
    /// Update mood rating (1-5)
    UpdateMood(u8),
    /// Show delete confirmation modal
    ShowDeleteConfirm,
    /// Confirm and execute deletion
    ConfirmDelete,

    // Modal actions
    /// Show new report modal
    ShowNewReport,
    /// Create the report from modal state
    CreateReport,
    /// Cancel and close the current modal
    CancelModal,
    /// Navigate left in modal
    ModalLeft,
    /// Navigate right in modal
    ModalRight,
    /// Move to next field in modal
    ModalNextField,
    /// Move to previous field in modal
    ModalPrevField,

    // Entry input modal actions (mood observation)
    /// Show entry input modal
    ShowEntryInput,
    /// Set mood for the entry being created
    SetEntryMood(u8),
    /// Cycle through entry context options
    CycleEntryContext,
    /// Save the current entry
    SaveEntry,

    // Data refresh
    /// Reload all data from disk
    RefreshData,

    // Input handling (for modals)
    /// Character input
    Input(char),
    /// Backspace key
    Backspace,
    /// Enter key
    Enter,
}

/// Main application state
pub struct App {
    /// The loaded workspace
    pub workspace: Workspace,
    /// All reports in the workspace
    pub reports: Vec<Report>,
    /// Journal entries indexed by report
    pub entries_by_report: Vec<Vec<JournalEntry>>,
    /// Computed summaries for each report
    pub summaries: Vec<ReportSummary>,
    /// Aggregate workspace summary
    pub workspace_summary: WorkspaceSummary,

    // UI state
    /// Current view mode
    pub view_mode: ViewMode,
    /// Currently selected index in the active list
    pub selected_index: usize,
    /// Index of the currently viewed report (when in detail view)
    pub selected_report_index: Option<usize>,
    /// Index of the currently viewed entry
    pub selected_entry_index: Option<usize>,

    // Note viewer state
    /// Content being viewed/edited
    pub editor_content: String,
    /// Current mood rating
    pub editor_mood: Option<u8>,

    // New report modal state
    /// State for the new report modal form
    pub new_report_state: NewReportState,

    // Entry input modal state
    /// Mood for the entry being created
    pub pending_entry_mood: Option<u8>,
    /// Context for the entry being created
    pub pending_entry_context: Context,
    /// Notes for the entry being created
    pub pending_entry_notes: String,

    // App state
    /// Flag to signal the app should quit
    pub should_quit: bool,
    /// Current status message with timestamp
    pub status_message: Option<(String, Instant)>,
    /// Track if delete was initiated from entry list
    pub delete_from_list: bool,
}

/// Terminal type alias for convenience
pub type Term = Terminal<CrosstermBackend<std::io::Stdout>>;
