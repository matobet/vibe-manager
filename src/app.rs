//! Application state and TEA (The Elm Architecture) runtime

use std::path::PathBuf;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::components::modal::NewReportState;
use crate::model::{
    compute_report_summary, compute_workspace_summary, Context, JournalEntry, ManagerInfo, Report,
    ReportSummary, Workspace, WorkspaceSummary,
};

/// Status message display duration
const STATUS_MESSAGE_DURATION: Duration = Duration::from_secs(3);

/// Side effects that the update function can request
/// This keeps the TEA pattern pure - update() returns what should happen,
/// the runtime (main.rs) executes the effects
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Effect {
    /// No side effect needed
    None,
    /// Spawn external editor for current meeting
    SpawnEditor { is_new: bool },
}
use crate::storage;

/// Application view modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Dashboard,
    ReportDetail,
    NoteViewer,
    NewReportModal,
    DeleteConfirmModal,
    EntryInputModal,
    Help,
}

/// Messages that can be sent to update application state (TEA pattern)
#[derive(Debug, Clone)]
pub enum Msg {
    // Navigation
    Quit,
    Back,
    ShowHelp,
    HideHelp,

    // Dashboard actions
    SelectNext,
    SelectPrev,
    SelectFirst,
    SelectLast,
    ViewReport,

    // Report detail actions
    ViewMeeting(usize),
    NewMeeting,

    // Note viewer actions
    EditMeeting,
    /// Edit meeting directly from list (index is display index, newest first)
    EditMeetingFromList(usize),
    UpdateMood(u8),
    ShowDeleteConfirm,
    ConfirmDelete,

    // Modal actions
    ShowNewReport,
    CreateReport,
    CancelModal,
    ModalLeft,
    ModalRight,
    ModalNextField,
    ModalPrevField,

    // Entry input modal actions (mood observation)
    ShowEntryInput,
    SetEntryMood(u8),
    CycleEntryContext,
    SaveEntry,

    // Data refresh
    RefreshData,

    // Input handling (for modals)
    Input(char),
    Backspace,
    Enter,
}

/// Main application state
pub struct App {
    pub workspace: Workspace,
    pub reports: Vec<Report>,
    pub entries_by_report: Vec<Vec<JournalEntry>>,
    pub summaries: Vec<ReportSummary>,
    pub workspace_summary: WorkspaceSummary,

    // UI state
    pub view_mode: ViewMode,
    pub selected_index: usize,
    pub selected_report_index: Option<usize>,
    pub selected_entry_index: Option<usize>,

    // Note viewer state
    pub editor_content: String,
    pub editor_mood: Option<u8>,

    // New report modal state
    pub new_report_state: NewReportState,

    // Entry input modal state
    pub pending_entry_mood: Option<u8>,
    pub pending_entry_context: Context,
    pub pending_entry_notes: String,

    // App state
    pub should_quit: bool,
    pub status_message: Option<(String, Instant)>,
    pub delete_from_list: bool, // Track if delete was initiated from entry list
}

impl App {
    /// Create new application from workspace path
    pub fn new(workspace_path: PathBuf) -> Result<Self> {
        let workspace = storage::load_workspace(&workspace_path)?;

        let mut app = App {
            workspace,
            reports: Vec::new(),
            entries_by_report: Vec::new(),
            summaries: Vec::new(),
            workspace_summary: WorkspaceSummary {
                team_size: 0,
                active_count: 0,
                overdue_count: 0,
                average_mood: None,
                total_report_count: 0,
            },
            view_mode: ViewMode::Dashboard,
            selected_index: 0,
            selected_report_index: None,
            selected_entry_index: None,
            editor_content: String::new(),
            editor_mood: None,
            new_report_state: NewReportState::default(),
            pending_entry_mood: None,
            pending_entry_context: Context::Standup,
            pending_entry_notes: String::new(),
            should_quit: false,
            status_message: None,
            delete_from_list: false,
        };

        app.load_data()?;
        Ok(app)
    }

    /// Load all data from workspace
    pub fn load_data(&mut self) -> Result<()> {
        let report_dirs = storage::list_report_dirs(&self.workspace)?;

        self.reports.clear();
        self.entries_by_report.clear();
        self.summaries.clear();

        // Collect all report data
        let mut all_data: Vec<_> = report_dirs
            .into_iter()
            .filter_map(|dir| {
                let mut report = storage::load_report(&dir).ok()?;

                // Load team members for managers
                if storage::has_team_dir(&dir) {
                    let team_dirs = storage::list_team_member_dirs(&dir).unwrap_or_default();
                    for team_dir in team_dirs {
                        if let Ok(team_member) =
                            storage::load_report_with_manager(&team_dir, &report.slug)
                        {
                            report.team.push(team_member);
                        }
                    }
                }

                let entries = storage::load_entries(&dir).unwrap_or_default();
                let summary = compute_report_summary(
                    &report,
                    &entries,
                    self.workspace.config.settings.overdue_threshold_days,
                );
                Some((report, entries, summary))
            })
            .collect();

        // Sort by urgency score (highest first = needs most attention)
        all_data.sort_by(|a, b| b.2.urgency_score.cmp(&a.2.urgency_score));

        // Unpack into separate vectors
        for (report, entries, summary) in all_data {
            self.reports.push(report);
            self.entries_by_report.push(entries);
            self.summaries.push(summary);
        }

        self.workspace_summary = compute_workspace_summary(&self.summaries);

        // Reset selection if out of bounds
        if self.selected_index >= self.reports.len() && !self.reports.is_empty() {
            self.selected_index = self.reports.len() - 1;
        }

        Ok(())
    }

    /// Set a status message with automatic expiry timestamp
    pub fn set_status(&mut self, message: impl Into<String>) {
        self.status_message = Some((message.into(), Instant::now()));
    }

    /// Clear status message if it has expired
    pub fn clear_expired_status(&mut self) {
        if let Some((_, timestamp)) = &self.status_message {
            if timestamp.elapsed() >= STATUS_MESSAGE_DURATION {
                self.status_message = None;
            }
        }
    }

    /// Get current status message text (if not expired)
    pub fn status_text(&self) -> Option<&str> {
        self.status_message.as_ref().and_then(|(msg, timestamp)| {
            if timestamp.elapsed() < STATUS_MESSAGE_DURATION {
                Some(msg.as_str())
            } else {
                None
            }
        })
    }

    /// Delete an entry by report and entry index
    /// Returns Ok(()) on success, sets status message on error
    pub fn delete_entry(&mut self, report_idx: usize, entry_idx: usize) -> Result<()> {
        let entry = &self.entries_by_report[report_idx][entry_idx];
        let path = entry.path.clone();

        // Delete the file
        std::fs::remove_file(&path)?;

        // Remove from in-memory list
        self.entries_by_report[report_idx].remove(entry_idx);
        self.selected_entry_index = None;

        // Recompute summary for this report
        let report = &self.reports[report_idx];
        let entries = &self.entries_by_report[report_idx];
        self.summaries[report_idx] = compute_report_summary(
            report,
            entries,
            self.workspace.config.settings.overdue_threshold_days,
        );
        self.workspace_summary = compute_workspace_summary(&self.summaries);

        Ok(())
    }

    /// Process a message and update state (TEA update function)
    /// Returns an Effect that the runtime should execute
    pub fn update(&mut self, msg: Msg) -> Result<Effect> {
        let effect = match msg {
            Msg::Quit => {
                self.should_quit = true;
                Effect::None
            }

            Msg::Back => {
                match self.view_mode {
                    ViewMode::ReportDetail => {
                        // Restore report selection for dashboard navigation
                        if let Some(report_idx) = self.selected_report_index {
                            self.selected_index = report_idx;
                        }
                        self.view_mode = ViewMode::Dashboard;
                        self.selected_report_index = None;
                    }
                    ViewMode::NoteViewer => {
                        self.view_mode = ViewMode::ReportDetail;
                        self.selected_entry_index = None;
                    }
                    ViewMode::Help | ViewMode::NewReportModal => {
                        self.view_mode = ViewMode::Dashboard;
                    }
                    ViewMode::EntryInputModal => {
                        self.view_mode = ViewMode::ReportDetail;
                        self.pending_entry_mood = None;
                        self.pending_entry_notes.clear();
                    }
                    ViewMode::DeleteConfirmModal => {
                        if self.delete_from_list {
                            self.selected_entry_index = None;
                            self.view_mode = ViewMode::ReportDetail;
                        } else {
                            self.view_mode = ViewMode::NoteViewer;
                        }
                    }
                    ViewMode::Dashboard => {}
                }
                Effect::None
            }

            Msg::ShowHelp => {
                self.view_mode = ViewMode::Help;
                Effect::None
            }

            Msg::HideHelp => {
                self.view_mode = ViewMode::Dashboard;
                Effect::None
            }

            Msg::SelectNext => {
                let max_len = self.current_list_len();
                if max_len > 0 {
                    self.selected_index = (self.selected_index + 1) % max_len;
                }
                Effect::None
            }

            Msg::SelectPrev => {
                let max_len = self.current_list_len();
                if max_len > 0 {
                    self.selected_index = if self.selected_index == 0 {
                        max_len - 1
                    } else {
                        self.selected_index - 1
                    };
                }
                Effect::None
            }

            Msg::SelectFirst => {
                self.selected_index = 0;
                Effect::None
            }

            Msg::SelectLast => {
                let max_len = self.current_list_len();
                if max_len > 0 {
                    self.selected_index = max_len - 1;
                }
                Effect::None
            }

            Msg::ViewReport => {
                if !self.reports.is_empty() {
                    self.selected_report_index = Some(self.selected_index);
                    self.selected_index = 0; // Reset for meeting navigation
                    self.view_mode = ViewMode::ReportDetail;
                }
                Effect::None
            }

            Msg::ViewMeeting(display_index) => {
                if let Some(actual_index) = self.meeting_display_to_entry_index(display_index) {
                    if let Some(report_idx) = self.selected_report_index {
                        self.selected_entry_index = Some(actual_index);
                        let entry = &self.entries_by_report[report_idx][actual_index];
                        self.editor_content = entry.content.clone();
                        self.editor_mood = entry.mood();
                        self.view_mode = ViewMode::NoteViewer;
                    }
                }
                Effect::None
            }

            Msg::NewMeeting => {
                if let Some(report_idx) = self.selected_report_index {
                    let report = &self.reports[report_idx];
                    match storage::create_meeting(&report.path, None) {
                        Ok(meeting) => {
                            self.editor_content = meeting.content.clone();
                            self.editor_mood = None;
                            self.entries_by_report[report_idx].push(meeting);
                            self.selected_entry_index =
                                Some(self.entries_by_report[report_idx].len() - 1);
                            self.view_mode = ViewMode::NoteViewer;
                            return Ok(Effect::SpawnEditor { is_new: true });
                        }
                        Err(e) => {
                            self.set_status(format!("Error: {}", e));
                        }
                    }
                }
                Effect::None
            }

            Msg::EditMeeting => Effect::SpawnEditor { is_new: false },

            Msg::EditMeetingFromList(display_index) => {
                if let Some(actual_index) = self.meeting_display_to_entry_index(display_index) {
                    if let Some(report_idx) = self.selected_report_index {
                        self.selected_entry_index = Some(actual_index);
                        let entry = &self.entries_by_report[report_idx][actual_index];
                        self.editor_content = entry.content.clone();
                        self.editor_mood = entry.mood();
                        // Don't change view mode - go straight to editor
                        return Ok(Effect::SpawnEditor { is_new: false });
                    }
                }
                Effect::None
            }

            Msg::UpdateMood(mood) => {
                self.editor_mood = Some(mood);
                // Save mood to disk immediately
                if let (Some(report_idx), Some(meet_idx)) =
                    (self.selected_report_index, self.selected_entry_index)
                {
                    let meeting = &mut self.entries_by_report[report_idx][meet_idx];
                    meeting.frontmatter.mood = Some(mood);
                    if let Err(e) = storage::save_meeting(meeting) {
                        self.set_status(format!("Error saving mood: {}", e));
                    } else {
                        self.set_status("Mood updated");
                    }
                }
                Effect::None
            }

            Msg::ShowDeleteConfirm => {
                // Handle delete from both NoteViewer and ReportDetail
                if self.view_mode == ViewMode::NoteViewer {
                    // Already viewing a meeting - selected_entry_index already set
                    self.delete_from_list = false;
                    self.view_mode = ViewMode::DeleteConfirmModal;
                } else if self.view_mode == ViewMode::ReportDetail {
                    // Deleting from the meeting list - map display index to entry index
                    if let Some(actual_index) =
                        self.meeting_display_to_entry_index(self.selected_index)
                    {
                        self.selected_entry_index = Some(actual_index);
                        self.delete_from_list = true;
                        self.view_mode = ViewMode::DeleteConfirmModal;
                    }
                }
                Effect::None
            }

            Msg::ConfirmDelete => {
                if let (Some(report_idx), Some(entry_idx)) =
                    (self.selected_report_index, self.selected_entry_index)
                {
                    match self.delete_entry(report_idx, entry_idx) {
                        Ok(()) => {
                            self.view_mode = ViewMode::ReportDetail;
                            self.set_status("Entry deleted");
                        }
                        Err(e) => {
                            self.set_status(format!("Error deleting entry: {}", e));
                        }
                    }
                }
                Effect::None
            }

            Msg::ShowNewReport => {
                self.new_report_state = NewReportState::default();
                self.view_mode = ViewMode::NewReportModal;
                Effect::None
            }

            Msg::CreateReport => {
                if !self.new_report_state.is_valid() {
                    self.set_status("Name and Title are required");
                    return Ok(Effect::None);
                }

                let name = self.new_report_state.name.clone();
                let title = self.new_report_state.title.clone();
                let level = self.new_report_state.level_str();
                let meeting_frequency = self.new_report_state.frequency_str().to_string();
                let report_type = self.new_report_state.report_type;

                let manager_info = if report_type.is_manager() {
                    Some(ManagerInfo { team_name: None })
                } else {
                    None
                };

                let profile = crate::model::ReportProfile {
                    name: name.clone(),
                    title: Some(title),
                    start_date: Some(chrono::Local::now().date_naive()),
                    level: Some(level),
                    meeting_frequency,
                    active: true,
                    report_type,
                    manager_info,
                    birthday: None,
                    partner: None,
                    children: vec![],
                    skills: None,
                    skills_updated: None,
                    color: None,
                };

                let type_label = if report_type.is_manager() {
                    "manager"
                } else {
                    "IC"
                };

                match storage::create_report(&self.workspace.path, &name, profile) {
                    Ok(_) => {
                        self.load_data()?;
                        self.set_status(format!("Recruited {} ({})", name, type_label));
                    }
                    Err(e) => {
                        self.set_status(format!("Error: {}", e));
                    }
                }
                self.view_mode = ViewMode::Dashboard;
                Effect::None
            }

            Msg::CancelModal => {
                match self.view_mode {
                    ViewMode::NewReportModal => {
                        self.view_mode = ViewMode::Dashboard;
                    }
                    ViewMode::EntryInputModal => {
                        self.view_mode = ViewMode::ReportDetail;
                        self.pending_entry_mood = None;
                        self.pending_entry_notes.clear();
                    }
                    ViewMode::DeleteConfirmModal => {
                        if self.delete_from_list {
                            self.selected_entry_index = None;
                            self.view_mode = ViewMode::ReportDetail;
                        } else {
                            self.view_mode = ViewMode::NoteViewer;
                        }
                    }
                    _ => {}
                }
                Effect::None
            }

            Msg::ModalLeft => {
                if self.view_mode == ViewMode::NewReportModal {
                    self.new_report_state.handle_left();
                }
                Effect::None
            }

            Msg::ModalRight => {
                if self.view_mode == ViewMode::NewReportModal {
                    self.new_report_state.handle_right();
                }
                Effect::None
            }

            Msg::ModalNextField => {
                if self.view_mode == ViewMode::NewReportModal {
                    self.new_report_state.next_field();
                }
                Effect::None
            }

            Msg::ModalPrevField => {
                if self.view_mode == ViewMode::NewReportModal {
                    self.new_report_state.prev_field();
                }
                Effect::None
            }

            // Entry input modal actions (mood observation)
            Msg::ShowEntryInput => {
                if self.selected_report_index.is_some() {
                    self.pending_entry_mood = None;
                    self.pending_entry_context = Context::Standup;
                    self.pending_entry_notes.clear();
                    self.view_mode = ViewMode::EntryInputModal;
                }
                Effect::None
            }

            Msg::SetEntryMood(mood) => {
                if self.view_mode == ViewMode::EntryInputModal {
                    self.pending_entry_mood = Some(mood);
                }
                Effect::None
            }

            Msg::CycleEntryContext => {
                if self.view_mode == ViewMode::EntryInputModal {
                    self.pending_entry_context = self.pending_entry_context.next();
                }
                Effect::None
            }

            Msg::SaveEntry => {
                if let Some(report_idx) = self.selected_report_index {
                    let report = &self.reports[report_idx];
                    match storage::create_entry(
                        &report.path,
                        self.pending_entry_mood,
                        Some(self.pending_entry_context),
                        self.pending_entry_notes.clone(),
                    ) {
                        Ok(entry) => {
                            self.entries_by_report[report_idx].push(entry);
                            // Recompute summary
                            let report = &self.reports[report_idx];
                            let entries = &self.entries_by_report[report_idx];
                            self.summaries[report_idx] = compute_report_summary(
                                report,
                                entries,
                                self.workspace.config.settings.overdue_threshold_days,
                            );
                            self.workspace_summary = compute_workspace_summary(&self.summaries);
                            self.set_status("Observation recorded");
                        }
                        Err(e) => {
                            self.set_status(format!("Error: {}", e));
                        }
                    }
                    self.pending_entry_mood = None;
                    self.pending_entry_notes.clear();
                    self.view_mode = ViewMode::ReportDetail;
                }
                Effect::None
            }

            Msg::RefreshData => {
                self.load_data()?;
                Effect::None
            }

            Msg::Input(c) => {
                if self.view_mode == ViewMode::NewReportModal {
                    self.new_report_state.handle_char(c);
                } else if self.view_mode == ViewMode::EntryInputModal {
                    self.pending_entry_notes.push(c);
                }
                Effect::None
            }

            Msg::Backspace => {
                if self.view_mode == ViewMode::NewReportModal {
                    self.new_report_state.handle_backspace();
                } else if self.view_mode == ViewMode::EntryInputModal {
                    self.pending_entry_notes.pop();
                }
                Effect::None
            }

            Msg::Enter => {
                if self.view_mode == ViewMode::NewReportModal {
                    if self.new_report_state.is_valid() {
                        return self.update(Msg::CreateReport);
                    } else {
                        self.set_status("Name is required");
                    }
                }
                Effect::None
            }
        };

        Ok(effect)
    }

    /// Get entries for currently selected report
    pub fn selected_entries(&self) -> Option<&Vec<JournalEntry>> {
        self.selected_report_index
            .and_then(|i| self.entries_by_report.get(i))
    }

    /// Get only meetings (entries with content) for currently selected report
    pub fn selected_meetings(&self) -> Vec<&JournalEntry> {
        self.selected_entries()
            .map(|entries| entries.iter().filter(|e| e.is_meeting()).collect())
            .unwrap_or_default()
    }

    /// Get the number of meetings for currently selected report
    pub fn selected_meeting_count(&self) -> usize {
        self.selected_entries()
            .map(|entries| entries.iter().filter(|e| e.is_meeting()).count())
            .unwrap_or(0)
    }

    /// Convert a display index (in the meetings list) to the actual entry index
    /// Display shows meetings in reverse chronological order (newest first)
    pub fn meeting_display_to_entry_index(&self, display_index: usize) -> Option<usize> {
        let entries = self.selected_entries()?;
        let meeting_indices: Vec<usize> = entries
            .iter()
            .enumerate()
            .filter(|(_, e)| e.is_meeting())
            .map(|(i, _)| i)
            .collect();

        // Display is reversed (newest first), so map accordingly
        let reversed_idx = meeting_indices.len().checked_sub(1 + display_index)?;
        meeting_indices.get(reversed_idx).copied()
    }

    /// Get the length of the currently navigable list based on view mode
    fn current_list_len(&self) -> usize {
        match self.view_mode {
            ViewMode::Dashboard => self.reports.len(),
            ViewMode::ReportDetail => self.selected_meeting_count(),
            _ => 0,
        }
    }
}

/// Map keyboard event to message
pub fn handle_key_event(app: &App, key: KeyEvent) -> Option<Msg> {
    // Helper to get lowercase char from key code (for case-insensitive matching)
    let lowercase_char = match key.code {
        KeyCode::Char(c) => Some(c.to_ascii_lowercase()),
        _ => None,
    };

    // Global shortcuts (Ctrl+key)
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        match lowercase_char {
            Some('c') | Some('q') => return Some(Msg::Quit),
            Some('r') => return Some(Msg::RefreshData),
            _ => {}
        }
    }

    match app.view_mode {
        ViewMode::Dashboard => match key.code {
            // Arrow keys
            KeyCode::Left => Some(Msg::SelectPrev),
            KeyCode::Right => Some(Msg::SelectNext),
            KeyCode::Down => Some(Msg::SelectNext),
            KeyCode::Up => Some(Msg::SelectPrev),
            KeyCode::Enter => Some(Msg::ViewReport),
            // Character keys (case-insensitive, except g/G)
            KeyCode::Char('g') => Some(Msg::SelectFirst),
            KeyCode::Char('G') => Some(Msg::SelectLast),
            KeyCode::Char(c) => match c.to_ascii_lowercase() {
                'q' => Some(Msg::Quit),
                'h' => Some(Msg::SelectPrev),
                'l' => Some(Msg::SelectNext),
                'j' => Some(Msg::SelectNext),
                'k' => Some(Msg::SelectPrev),
                ' ' => Some(Msg::ViewReport),
                'n' => Some(Msg::ShowNewReport),
                '?' => Some(Msg::ShowHelp),
                'r' => Some(Msg::RefreshData),
                _ => None,
            },
            _ => None,
        },

        ViewMode::ReportDetail => match key.code {
            // Non-char keys
            KeyCode::Esc | KeyCode::Backspace => Some(Msg::Back),
            KeyCode::Left => Some(Msg::Back),
            KeyCode::Down => Some(Msg::SelectNext),
            KeyCode::Up => Some(Msg::SelectPrev),
            KeyCode::Right | KeyCode::Enter => {
                if app.selected_index < app.selected_meeting_count() {
                    Some(Msg::ViewMeeting(app.selected_index))
                } else {
                    None
                }
            }
            KeyCode::Delete => Some(Msg::ShowDeleteConfirm),
            // Character keys (case-insensitive, except g/G)
            KeyCode::Char('g') => Some(Msg::SelectFirst),
            KeyCode::Char('G') => Some(Msg::SelectLast),
            KeyCode::Char(c) => match c.to_ascii_lowercase() {
                'q' => Some(Msg::Quit),
                'h' => Some(Msg::Back),
                'j' => Some(Msg::SelectNext),
                'k' => Some(Msg::SelectPrev),
                'l' => {
                    if app.selected_index < app.selected_meeting_count() {
                        Some(Msg::ViewMeeting(app.selected_index))
                    } else {
                        None
                    }
                }
                'e' => {
                    if app.selected_index < app.selected_meeting_count() {
                        Some(Msg::EditMeetingFromList(app.selected_index))
                    } else {
                        None
                    }
                }
                'n' => Some(Msg::NewMeeting),
                'm' => Some(Msg::ShowEntryInput),
                '?' => Some(Msg::ShowHelp),
                _ => None,
            },
            _ => None,
        },

        ViewMode::NoteViewer => match key.code {
            KeyCode::Esc | KeyCode::Backspace => Some(Msg::Back),
            KeyCode::Delete => Some(Msg::ShowDeleteConfirm),
            KeyCode::F(1) => Some(Msg::UpdateMood(1)),
            KeyCode::F(2) => Some(Msg::UpdateMood(2)),
            KeyCode::F(3) => Some(Msg::UpdateMood(3)),
            KeyCode::F(4) => Some(Msg::UpdateMood(4)),
            KeyCode::F(5) => Some(Msg::UpdateMood(5)),
            KeyCode::Char(c) => match c.to_ascii_lowercase() {
                'q' => Some(Msg::Quit),
                'e' => Some(Msg::EditMeeting),
                _ => None,
            },
            _ => None,
        },

        ViewMode::DeleteConfirmModal => match key.code {
            KeyCode::Enter => Some(Msg::ConfirmDelete),
            KeyCode::Esc => Some(Msg::CancelModal),
            KeyCode::Char(c) => match c.to_ascii_lowercase() {
                'y' => Some(Msg::ConfirmDelete),
                'n' => Some(Msg::CancelModal),
                _ => None,
            },
            _ => None,
        },

        // New report modal with enhanced navigation
        ViewMode::NewReportModal => {
            // Check if we're in a text input field
            let in_text_field = matches!(
                app.new_report_state.current_field,
                crate::components::modal::NewReportField::Name
                    | crate::components::modal::NewReportField::Title
            );

            match key.code {
                KeyCode::Esc => Some(Msg::CancelModal),
                KeyCode::Left => Some(Msg::ModalLeft),
                KeyCode::Right => Some(Msg::ModalRight),
                KeyCode::Up => Some(Msg::ModalPrevField),
                KeyCode::Down | KeyCode::Tab => Some(Msg::ModalNextField),
                KeyCode::Enter => Some(Msg::Enter),
                KeyCode::Backspace => Some(Msg::Backspace),
                // vim keys only work in non-text fields
                KeyCode::Char('h') if !in_text_field => Some(Msg::ModalLeft),
                KeyCode::Char('l') if !in_text_field => Some(Msg::ModalRight),
                KeyCode::Char('k') if !in_text_field => Some(Msg::ModalPrevField),
                KeyCode::Char('j') if !in_text_field => Some(Msg::ModalNextField),
                KeyCode::Char(c) => Some(Msg::Input(c)),
                _ => None,
            }
        }

        ViewMode::EntryInputModal => match key.code {
            KeyCode::Esc => Some(Msg::CancelModal),
            KeyCode::Char('1') => Some(Msg::SetEntryMood(1)),
            KeyCode::Char('2') => Some(Msg::SetEntryMood(2)),
            KeyCode::Char('3') => Some(Msg::SetEntryMood(3)),
            KeyCode::Char('4') => Some(Msg::SetEntryMood(4)),
            KeyCode::Char('5') => Some(Msg::SetEntryMood(5)),
            KeyCode::Tab => Some(Msg::CycleEntryContext),
            KeyCode::Enter => Some(Msg::SaveEntry),
            KeyCode::Backspace => Some(Msg::Backspace),
            KeyCode::Char(c) => Some(Msg::Input(c)),
            _ => None,
        },

        ViewMode::Help => match key.code {
            KeyCode::Esc => Some(Msg::HideHelp),
            KeyCode::Char(c) => match c.to_ascii_lowercase() {
                'q' | '?' => Some(Msg::HideHelp),
                _ => None,
            },
            _ => None,
        },
    }
}

/// Poll for events with timeout
pub fn poll_event(timeout: Duration) -> Result<Option<Event>> {
    if event::poll(timeout)? {
        Ok(Some(event::read()?))
    } else {
        Ok(None)
    }
}

pub type Term = Terminal<CrosstermBackend<std::io::Stdout>>;
