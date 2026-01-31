//! Application state and TEA (The Elm Architecture) runtime

use std::path::PathBuf;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::model::{
    compute_engineer_summary, compute_workspace_summary, Context, Engineer, EngineerSummary,
    JournalEntry, Workspace, WorkspaceSummary,
};

/// Status message display duration
const STATUS_MESSAGE_DURATION: Duration = Duration::from_secs(3);

/// Default values for new engineer modal
pub const DEFAULT_LEVEL: &str = "P3";
pub const DEFAULT_MEETING_FREQUENCY: &str = "biweekly";

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
    EngineerDetail,
    NoteViewer,
    NewEngineerModal,
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
    ViewEngineer,

    // Engineer detail actions
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
    ShowNewEngineer,
    CreateEngineer {
        name: String,
        level: String,
        meeting_frequency: String,
    },
    CancelModal,

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
    pub engineers: Vec<Engineer>,
    pub entries_by_engineer: Vec<Vec<JournalEntry>>,
    pub summaries: Vec<EngineerSummary>,
    pub workspace_summary: WorkspaceSummary,

    // UI state
    pub view_mode: ViewMode,
    pub selected_index: usize,
    pub selected_engineer_index: Option<usize>,
    pub selected_entry_index: Option<usize>,

    // Note viewer state
    pub editor_content: String,
    pub editor_mood: Option<u8>,

    // Modal state
    pub modal_input: String,
    pub modal_field_index: usize,
    pub modal_fields: Vec<String>,

    // Entry input modal state
    pub pending_entry_mood: Option<u8>,
    pub pending_entry_context: Context,
    pub pending_entry_notes: String,

    // App state
    pub should_quit: bool,
    pub status_message: Option<(String, Instant)>,
    pub delete_from_list: bool, // Track if delete was initiated from entry list
}

// Backwards compatibility alias
impl App {
    pub fn meetings_by_engineer(&self) -> &Vec<Vec<JournalEntry>> {
        &self.entries_by_engineer
    }

    pub fn selected_meeting_index(&self) -> Option<usize> {
        self.selected_entry_index
    }
}

impl App {
    /// Create new application from workspace path
    pub fn new(workspace_path: PathBuf) -> Result<Self> {
        let workspace = storage::load_workspace(&workspace_path)?;

        let mut app = App {
            workspace,
            engineers: Vec::new(),
            entries_by_engineer: Vec::new(),
            summaries: Vec::new(),
            workspace_summary: WorkspaceSummary {
                team_size: 0,
                active_count: 0,
                overdue_count: 0,
                average_mood: None,
            },
            view_mode: ViewMode::Dashboard,
            selected_index: 0,
            selected_engineer_index: None,
            selected_entry_index: None,
            editor_content: String::new(),
            editor_mood: None,
            modal_input: String::new(),
            modal_field_index: 0,
            modal_fields: Vec::new(),
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
        let engineer_dirs = storage::list_engineer_dirs(&self.workspace)?;

        self.engineers.clear();
        self.entries_by_engineer.clear();
        self.summaries.clear();

        // Collect all engineer data
        let mut all_data: Vec<_> = engineer_dirs
            .into_iter()
            .filter_map(|dir| {
                let engineer = storage::load_engineer(&dir).ok()?;
                let entries = storage::load_entries(&dir).unwrap_or_default();
                let summary = compute_engineer_summary(
                    &engineer,
                    &entries,
                    self.workspace.config.settings.overdue_threshold_days,
                );
                Some((engineer, entries, summary))
            })
            .collect();

        // Sort by urgency score (highest first = needs most attention)
        all_data.sort_by(|a, b| b.2.urgency_score.cmp(&a.2.urgency_score));

        // Unpack into separate vectors
        for (engineer, entries, summary) in all_data {
            self.engineers.push(engineer);
            self.entries_by_engineer.push(entries);
            self.summaries.push(summary);
        }

        self.workspace_summary = compute_workspace_summary(&self.summaries);

        // Reset selection if out of bounds
        if self.selected_index >= self.engineers.len() && !self.engineers.is_empty() {
            self.selected_index = self.engineers.len() - 1;
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

    /// Delete an entry by engineer and entry index
    /// Returns Ok(()) on success, sets status message on error
    pub fn delete_entry(&mut self, eng_idx: usize, entry_idx: usize) -> Result<()> {
        let entry = &self.entries_by_engineer[eng_idx][entry_idx];
        let path = entry.path.clone();

        // Delete the file
        std::fs::remove_file(&path)?;

        // Remove from in-memory list
        self.entries_by_engineer[eng_idx].remove(entry_idx);
        self.selected_entry_index = None;

        // Recompute summary for this engineer
        let engineer = &self.engineers[eng_idx];
        let entries = &self.entries_by_engineer[eng_idx];
        self.summaries[eng_idx] = compute_engineer_summary(
            engineer,
            entries,
            self.workspace.config.settings.overdue_threshold_days,
        );
        self.workspace_summary = compute_workspace_summary(&self.summaries);

        Ok(())
    }

    /// Backwards compatibility alias
    pub fn delete_meeting(&mut self, eng_idx: usize, meet_idx: usize) -> Result<()> {
        self.delete_entry(eng_idx, meet_idx)
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
                    ViewMode::EngineerDetail => {
                        // Restore engineer selection for dashboard navigation
                        if let Some(eng_idx) = self.selected_engineer_index {
                            self.selected_index = eng_idx;
                        }
                        self.view_mode = ViewMode::Dashboard;
                        self.selected_engineer_index = None;
                    }
                    ViewMode::NoteViewer => {
                        self.view_mode = ViewMode::EngineerDetail;
                        self.selected_entry_index = None;
                    }
                    ViewMode::Help | ViewMode::NewEngineerModal => {
                        self.view_mode = ViewMode::Dashboard;
                    }
                    ViewMode::EntryInputModal => {
                        self.view_mode = ViewMode::EngineerDetail;
                        self.pending_entry_mood = None;
                        self.pending_entry_notes.clear();
                    }
                    ViewMode::DeleteConfirmModal => {
                        if self.delete_from_list {
                            self.selected_entry_index = None;
                            self.view_mode = ViewMode::EngineerDetail;
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

            Msg::ViewEngineer => {
                if !self.engineers.is_empty() {
                    self.selected_engineer_index = Some(self.selected_index);
                    self.selected_index = 0; // Reset for meeting navigation
                    self.view_mode = ViewMode::EngineerDetail;
                }
                Effect::None
            }

            Msg::ViewMeeting(display_index) => {
                if let Some(actual_index) = self.meeting_display_to_entry_index(display_index) {
                    if let Some(eng_idx) = self.selected_engineer_index {
                        self.selected_entry_index = Some(actual_index);
                        let entry = &self.entries_by_engineer[eng_idx][actual_index];
                        self.editor_content = entry.content.clone();
                        self.editor_mood = entry.mood();
                        self.view_mode = ViewMode::NoteViewer;
                    }
                }
                Effect::None
            }

            Msg::NewMeeting => {
                if let Some(eng_idx) = self.selected_engineer_index {
                    let engineer = &self.engineers[eng_idx];
                    match storage::create_meeting(&engineer.path, None) {
                        Ok(meeting) => {
                            self.editor_content = meeting.content.clone();
                            self.editor_mood = None;
                            self.entries_by_engineer[eng_idx].push(meeting);
                            self.selected_entry_index =
                                Some(self.entries_by_engineer[eng_idx].len() - 1);
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
                    if let Some(eng_idx) = self.selected_engineer_index {
                        self.selected_entry_index = Some(actual_index);
                        let entry = &self.entries_by_engineer[eng_idx][actual_index];
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
                if let (Some(eng_idx), Some(meet_idx)) =
                    (self.selected_engineer_index, self.selected_entry_index)
                {
                    let meeting = &mut self.entries_by_engineer[eng_idx][meet_idx];
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
                // Handle delete from both NoteViewer and EngineerDetail
                if self.view_mode == ViewMode::NoteViewer {
                    // Already viewing a meeting - selected_entry_index already set
                    self.delete_from_list = false;
                    self.view_mode = ViewMode::DeleteConfirmModal;
                } else if self.view_mode == ViewMode::EngineerDetail {
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
                if let (Some(eng_idx), Some(meet_idx)) =
                    (self.selected_engineer_index, self.selected_entry_index)
                {
                    match self.delete_meeting(eng_idx, meet_idx) {
                        Ok(()) => {
                            self.view_mode = ViewMode::EngineerDetail;
                            self.set_status("Meeting deleted");
                        }
                        Err(e) => {
                            self.set_status(format!("Error deleting meeting: {}", e));
                        }
                    }
                }
                Effect::None
            }

            Msg::ShowNewEngineer => {
                self.modal_input.clear();
                self.modal_field_index = 0;
                self.modal_fields = vec![
                    String::new(),
                    String::from(DEFAULT_LEVEL),
                    String::from(DEFAULT_MEETING_FREQUENCY),
                ];
                self.view_mode = ViewMode::NewEngineerModal;
                Effect::None
            }

            Msg::CreateEngineer {
                name,
                level,
                meeting_frequency,
            } => {
                let profile = crate::model::EngineerProfile {
                    name: name.clone(),
                    title: None,
                    start_date: Some(chrono::Local::now().date_naive()),
                    level: Some(level),
                    meeting_frequency,
                    active: true,
                    birthday: None,
                    partner: None,
                    children: vec![],
                    skills: None,
                    skills_updated: None,
                    color: None,
                };

                match storage::create_engineer(&self.workspace.path, &name, profile) {
                    Ok(_) => {
                        self.load_data()?;
                        self.set_status(format!("Created {}", name));
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
                    ViewMode::NewEngineerModal => {
                        self.view_mode = ViewMode::Dashboard;
                        self.modal_input.clear();
                        self.modal_fields.clear();
                    }
                    ViewMode::EntryInputModal => {
                        self.view_mode = ViewMode::EngineerDetail;
                        self.pending_entry_mood = None;
                        self.pending_entry_notes.clear();
                    }
                    ViewMode::DeleteConfirmModal => {
                        if self.delete_from_list {
                            self.selected_entry_index = None;
                            self.view_mode = ViewMode::EngineerDetail;
                        } else {
                            self.view_mode = ViewMode::NoteViewer;
                        }
                    }
                    _ => {}
                }
                Effect::None
            }

            // Entry input modal actions (mood observation)
            Msg::ShowEntryInput => {
                if self.selected_engineer_index.is_some() {
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
                if let Some(eng_idx) = self.selected_engineer_index {
                    let engineer = &self.engineers[eng_idx];
                    match storage::create_entry(
                        &engineer.path,
                        self.pending_entry_mood,
                        Some(self.pending_entry_context),
                        self.pending_entry_notes.clone(),
                    ) {
                        Ok(entry) => {
                            self.entries_by_engineer[eng_idx].push(entry);
                            // Recompute summary
                            let engineer = &self.engineers[eng_idx];
                            let entries = &self.entries_by_engineer[eng_idx];
                            self.summaries[eng_idx] = compute_engineer_summary(
                                engineer,
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
                    self.view_mode = ViewMode::EngineerDetail;
                }
                Effect::None
            }

            Msg::RefreshData => {
                self.load_data()?;
                Effect::None
            }

            Msg::Input(c) => {
                if self.view_mode == ViewMode::NewEngineerModal
                    && self.modal_field_index < self.modal_fields.len()
                {
                    self.modal_fields[self.modal_field_index].push(c);
                } else if self.view_mode == ViewMode::EntryInputModal {
                    self.pending_entry_notes.push(c);
                }
                Effect::None
            }

            Msg::Backspace => {
                if self.view_mode == ViewMode::NewEngineerModal
                    && self.modal_field_index < self.modal_fields.len()
                {
                    self.modal_fields[self.modal_field_index].pop();
                } else if self.view_mode == ViewMode::EntryInputModal {
                    self.pending_entry_notes.pop();
                }
                Effect::None
            }

            Msg::Enter => {
                if self.view_mode == ViewMode::NewEngineerModal {
                    if self.modal_field_index < 2 {
                        self.modal_field_index += 1;
                    } else if !self.modal_fields[0].is_empty() {
                        // Create the engineer directly
                        let name = self.modal_fields[0].clone();
                        let level = self.modal_fields[1].clone();
                        let meeting_frequency = self.modal_fields[2].clone();
                        return self.update(Msg::CreateEngineer {
                            name,
                            level,
                            meeting_frequency,
                        });
                    }
                }
                Effect::None
            }
        };

        Ok(effect)
    }

    /// Get entries for currently selected engineer
    pub fn selected_entries(&self) -> Option<&Vec<JournalEntry>> {
        self.selected_engineer_index
            .and_then(|i| self.entries_by_engineer.get(i))
    }

    /// Get only meetings (entries with content) for currently selected engineer
    pub fn selected_meetings(&self) -> Vec<&JournalEntry> {
        self.selected_entries()
            .map(|entries| entries.iter().filter(|e| e.is_meeting()).collect())
            .unwrap_or_default()
    }

    /// Get the number of meetings for currently selected engineer
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
            ViewMode::Dashboard => self.engineers.len(),
            ViewMode::EngineerDetail => self.selected_meeting_count(),
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
            KeyCode::Enter => Some(Msg::ViewEngineer),
            // Character keys (case-insensitive, except g/G)
            KeyCode::Char('g') => Some(Msg::SelectFirst),
            KeyCode::Char('G') => Some(Msg::SelectLast),
            KeyCode::Char(c) => match c.to_ascii_lowercase() {
                'q' => Some(Msg::Quit),
                'h' => Some(Msg::SelectPrev),
                'l' => Some(Msg::SelectNext),
                'j' => Some(Msg::SelectNext),
                'k' => Some(Msg::SelectPrev),
                ' ' => Some(Msg::ViewEngineer),
                'n' => Some(Msg::ShowNewEngineer),
                '?' => Some(Msg::ShowHelp),
                'r' => Some(Msg::RefreshData),
                _ => None,
            },
            _ => None,
        },

        ViewMode::EngineerDetail => match key.code {
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

        // Modals that accept text input preserve case
        ViewMode::NewEngineerModal => match key.code {
            KeyCode::Esc => Some(Msg::CancelModal),
            KeyCode::Char(c) => Some(Msg::Input(c)),
            KeyCode::Backspace => Some(Msg::Backspace),
            KeyCode::Enter | KeyCode::Tab => Some(Msg::Enter),
            _ => None,
        },

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
