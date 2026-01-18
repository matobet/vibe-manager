//! Application state and TEA (The Elm Architecture) runtime

use std::path::PathBuf;
use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

use crate::model::{
    compute_engineer_summary, compute_workspace_summary, Engineer,
    EngineerSummary, Meeting, Workspace, WorkspaceSummary,
};
use crate::storage;

/// Application view modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Dashboard,
    EngineerDetail,
    NoteEditor,
    NewEngineerModal,
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

    // Note editor actions
    SaveNote,
    UpdateMood(u8),

    // Cursor movement
    CursorLeft,
    CursorRight,
    CursorUp,
    CursorDown,
    CursorHome,
    CursorEnd,
    DeleteChar,

    // Modal actions
    ShowNewEngineer,
    CreateEngineer { name: String, level: String, meeting_frequency: String },
    CancelModal,

    // Data refresh
    RefreshData,

    // Input handling
    Input(char),
    Backspace,
    Enter,
}

/// Main application state
pub struct App {
    pub workspace: Workspace,
    pub engineers: Vec<Engineer>,
    pub meetings_by_engineer: Vec<Vec<Meeting>>,
    pub summaries: Vec<EngineerSummary>,
    pub workspace_summary: WorkspaceSummary,

    // UI state
    pub view_mode: ViewMode,
    pub selected_index: usize,
    pub selected_engineer_index: Option<usize>,
    pub selected_meeting_index: Option<usize>,

    // Editor state
    pub editor_content: String,
    pub editor_cursor: usize,  // Cursor position in content
    pub editor_mood: Option<u8>,

    // Modal state
    pub modal_input: String,
    pub modal_field_index: usize,
    pub modal_fields: Vec<String>,

    // App state
    pub should_quit: bool,
    pub status_message: Option<String>,
}

impl App {
    /// Create new application from workspace path
    pub fn new(workspace_path: PathBuf) -> Result<Self> {
        let workspace = storage::load_workspace(&workspace_path)?;

        let mut app = App {
            workspace,
            engineers: Vec::new(),
            meetings_by_engineer: Vec::new(),
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
            selected_meeting_index: None,
            editor_content: String::new(),
            editor_cursor: 0,
            editor_mood: None,
            modal_input: String::new(),
            modal_field_index: 0,
            modal_fields: Vec::new(),
            should_quit: false,
            status_message: None,
        };

        app.load_data()?;
        Ok(app)
    }

    /// Load all data from workspace
    pub fn load_data(&mut self) -> Result<()> {
        let engineer_dirs = storage::list_engineer_dirs(&self.workspace)?;

        self.engineers.clear();
        self.meetings_by_engineer.clear();
        self.summaries.clear();

        // Collect all engineer data
        let mut all_data: Vec<_> = engineer_dirs
            .into_iter()
            .filter_map(|dir| {
                let engineer = storage::load_engineer(&dir).ok()?;
                let meetings = storage::load_meetings(&dir).unwrap_or_default();
                let summary = compute_engineer_summary(
                    &engineer,
                    &meetings,
                    self.workspace.config.settings.overdue_threshold_days,
                );
                Some((engineer, meetings, summary))
            })
            .collect();

        // Sort by urgency score (highest first = needs most attention)
        all_data.sort_by(|a, b| b.2.urgency_score.cmp(&a.2.urgency_score));

        // Unpack into separate vectors
        for (engineer, meetings, summary) in all_data {
            self.engineers.push(engineer);
            self.meetings_by_engineer.push(meetings);
            self.summaries.push(summary);
        }

        self.workspace_summary = compute_workspace_summary(&self.summaries);

        // Reset selection if out of bounds
        if self.selected_index >= self.engineers.len() && !self.engineers.is_empty() {
            self.selected_index = self.engineers.len() - 1;
        }

        Ok(())
    }

    /// Process a message and update state (TEA update function)
    pub fn update(&mut self, msg: Msg) -> Result<()> {
        match msg {
            Msg::Quit => {
                self.should_quit = true;
            }

            Msg::Back => {
                match self.view_mode {
                    ViewMode::EngineerDetail => {
                        self.view_mode = ViewMode::Dashboard;
                        self.selected_engineer_index = None;
                    }
                    ViewMode::NoteEditor => {
                        self.view_mode = ViewMode::EngineerDetail;
                        self.selected_meeting_index = None;
                    }
                    ViewMode::Help | ViewMode::NewEngineerModal => {
                        self.view_mode = ViewMode::Dashboard;
                    }
                    ViewMode::Dashboard => {}
                }
            }

            Msg::ShowHelp => {
                self.view_mode = ViewMode::Help;
            }

            Msg::HideHelp => {
                self.view_mode = ViewMode::Dashboard;
            }

            Msg::SelectNext => {
                if !self.engineers.is_empty() {
                    self.selected_index = (self.selected_index + 1) % self.engineers.len();
                }
            }

            Msg::SelectPrev => {
                if !self.engineers.is_empty() {
                    self.selected_index = if self.selected_index == 0 {
                        self.engineers.len() - 1
                    } else {
                        self.selected_index - 1
                    };
                }
            }

            Msg::SelectFirst => {
                self.selected_index = 0;
            }

            Msg::SelectLast => {
                if !self.engineers.is_empty() {
                    self.selected_index = self.engineers.len() - 1;
                }
            }

            Msg::ViewEngineer => {
                if !self.engineers.is_empty() {
                    self.selected_engineer_index = Some(self.selected_index);
                    self.view_mode = ViewMode::EngineerDetail;
                }
            }

            Msg::ViewMeeting(index) => {
                if let Some(eng_idx) = self.selected_engineer_index {
                    if index < self.meetings_by_engineer[eng_idx].len() {
                        self.selected_meeting_index = Some(index);
                        let meeting = &self.meetings_by_engineer[eng_idx][index];
                        self.editor_content = meeting.content.clone();
                        self.editor_cursor = self.editor_content.len(); // Start at end
                        self.editor_mood = meeting.mood();
                        self.view_mode = ViewMode::NoteEditor;
                    }
                }
            }

            Msg::NewMeeting => {
                if let Some(eng_idx) = self.selected_engineer_index {
                    let engineer = &self.engineers[eng_idx];
                    match storage::create_meeting(&engineer.path, None) {
                        Ok(meeting) => {
                            self.editor_content = meeting.content.clone();
                            self.editor_cursor = self.editor_content.len(); // Start at end
                            self.editor_mood = None;
                            self.meetings_by_engineer[eng_idx].push(meeting);
                            self.selected_meeting_index =
                                Some(self.meetings_by_engineer[eng_idx].len() - 1);
                            self.view_mode = ViewMode::NoteEditor;
                            self.status_message = Some("New meeting created".to_string());
                        }
                        Err(e) => {
                            self.status_message = Some(format!("Error: {}", e));
                        }
                    }
                }
            }

            Msg::SaveNote => {
                if let (Some(eng_idx), Some(meet_idx)) =
                    (self.selected_engineer_index, self.selected_meeting_index)
                {
                    let meeting = &mut self.meetings_by_engineer[eng_idx][meet_idx];
                    meeting.content = self.editor_content.clone();
                    if let Some(mood) = self.editor_mood {
                        meeting.frontmatter.mood = Some(mood);
                    }
                    if let Err(e) = storage::save_meeting(meeting) {
                        self.status_message = Some(format!("Error saving: {}", e));
                    } else {
                        self.status_message = Some("Saved".to_string());
                    }
                }
            }

            Msg::UpdateMood(mood) => {
                self.editor_mood = Some(mood);
            }

            Msg::ShowNewEngineer => {
                self.modal_input.clear();
                self.modal_field_index = 0;
                self.modal_fields = vec![String::new(), String::from("P3"), String::from("biweekly")];
                self.view_mode = ViewMode::NewEngineerModal;
            }

            Msg::CreateEngineer { name, level, meeting_frequency } => {
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
                        self.status_message = Some(format!("Created {}", name));
                    }
                    Err(e) => {
                        self.status_message = Some(format!("Error: {}", e));
                    }
                }
                self.view_mode = ViewMode::Dashboard;
            }

            Msg::CancelModal => {
                self.view_mode = ViewMode::Dashboard;
                self.modal_input.clear();
                self.modal_fields.clear();
            }

            Msg::RefreshData => {
                self.load_data()?;
            }

            Msg::Input(c) => {
                if self.view_mode == ViewMode::NewEngineerModal {
                    if self.modal_field_index < self.modal_fields.len() {
                        self.modal_fields[self.modal_field_index].push(c);
                    }
                } else if self.view_mode == ViewMode::NoteEditor {
                    // Insert at cursor position
                    self.editor_content.insert(self.editor_cursor, c);
                    self.editor_cursor += 1;
                }
            }

            Msg::Backspace => {
                if self.view_mode == ViewMode::NewEngineerModal {
                    if self.modal_field_index < self.modal_fields.len() {
                        self.modal_fields[self.modal_field_index].pop();
                    }
                } else if self.view_mode == ViewMode::NoteEditor {
                    // Delete character before cursor
                    if self.editor_cursor > 0 {
                        self.editor_cursor -= 1;
                        self.editor_content.remove(self.editor_cursor);
                    }
                }
            }

            Msg::DeleteChar => {
                if self.view_mode == ViewMode::NoteEditor {
                    // Delete character at cursor
                    if self.editor_cursor < self.editor_content.len() {
                        self.editor_content.remove(self.editor_cursor);
                    }
                }
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
                        self.update(Msg::CreateEngineer { name, level, meeting_frequency })?;
                    }
                } else if self.view_mode == ViewMode::NoteEditor {
                    // Insert newline at cursor position
                    self.editor_content.insert(self.editor_cursor, '\n');
                    self.editor_cursor += 1;
                }
            }

            Msg::CursorLeft => {
                if self.editor_cursor > 0 {
                    self.editor_cursor -= 1;
                }
            }

            Msg::CursorRight => {
                if self.editor_cursor < self.editor_content.len() {
                    self.editor_cursor += 1;
                }
            }

            Msg::CursorUp => {
                // Move to same column on previous line
                let (line, col) = self.cursor_line_col();
                if line > 0 {
                    let prev_line_start = self.line_start(line - 1);
                    let prev_line_len = self.line_length(line - 1);
                    self.editor_cursor = prev_line_start + col.min(prev_line_len);
                }
            }

            Msg::CursorDown => {
                // Move to same column on next line
                let (line, col) = self.cursor_line_col();
                let line_count = self.editor_content.lines().count();
                if line + 1 < line_count {
                    let next_line_start = self.line_start(line + 1);
                    let next_line_len = self.line_length(line + 1);
                    self.editor_cursor = next_line_start + col.min(next_line_len);
                }
            }

            Msg::CursorHome => {
                // Move to start of current line
                let (line, _) = self.cursor_line_col();
                self.editor_cursor = self.line_start(line);
            }

            Msg::CursorEnd => {
                // Move to end of current line
                let (line, _) = self.cursor_line_col();
                self.editor_cursor = self.line_start(line) + self.line_length(line);
            }
        }

        Ok(())
    }

    /// Get meetings for currently selected engineer
    pub fn selected_meetings(&self) -> Option<&Vec<Meeting>> {
        self.selected_engineer_index
            .and_then(|i| self.meetings_by_engineer.get(i))
    }

    /// Get current cursor line and column
    fn cursor_line_col(&self) -> (usize, usize) {
        let before_cursor = &self.editor_content[..self.editor_cursor];
        let line = before_cursor.matches('\n').count();
        let last_newline = before_cursor.rfind('\n').map(|i| i + 1).unwrap_or(0);
        let col = self.editor_cursor - last_newline;
        (line, col)
    }

    /// Get character offset for start of a line
    fn line_start(&self, line: usize) -> usize {
        if line == 0 {
            return 0;
        }
        self.editor_content
            .match_indices('\n')
            .nth(line - 1)
            .map(|(i, _)| i + 1)
            .unwrap_or(self.editor_content.len())
    }

    /// Get length of a line (not including newline)
    fn line_length(&self, line: usize) -> usize {
        let start = self.line_start(line);
        let rest = &self.editor_content[start..];
        rest.find('\n').unwrap_or(rest.len())
    }
}

/// Map keyboard event to message
pub fn handle_key_event(app: &App, key: KeyEvent) -> Option<Msg> {
    // Global shortcuts
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        match key.code {
            KeyCode::Char('c') | KeyCode::Char('q') => return Some(Msg::Quit),
            KeyCode::Char('s') => return Some(Msg::SaveNote),
            KeyCode::Char('r') => return Some(Msg::RefreshData),
            _ => {}
        }
    }

    match app.view_mode {
        ViewMode::Dashboard => match key.code {
            KeyCode::Char('q') => Some(Msg::Quit),
            // Grid navigation: h/l for horizontal, j/k for moving through party
            KeyCode::Char('h') | KeyCode::Left => Some(Msg::SelectPrev),
            KeyCode::Char('l') | KeyCode::Right => Some(Msg::SelectNext),
            KeyCode::Char('j') | KeyCode::Down => Some(Msg::SelectNext),
            KeyCode::Char('k') | KeyCode::Up => Some(Msg::SelectPrev),
            KeyCode::Char('g') => Some(Msg::SelectFirst),
            KeyCode::Char('G') => Some(Msg::SelectLast),
            KeyCode::Enter | KeyCode::Char(' ') => Some(Msg::ViewEngineer),
            KeyCode::Char('n') => Some(Msg::ShowNewEngineer),
            KeyCode::Char('?') => Some(Msg::ShowHelp),
            KeyCode::Char('r') => Some(Msg::RefreshData),
            _ => None,
        },

        ViewMode::EngineerDetail => match key.code {
            KeyCode::Char('q') => Some(Msg::Quit),
            KeyCode::Esc | KeyCode::Char('h') | KeyCode::Left => Some(Msg::Back),
            KeyCode::Char('j') | KeyCode::Down => Some(Msg::SelectNext),
            KeyCode::Char('k') | KeyCode::Up => Some(Msg::SelectPrev),
            KeyCode::Enter | KeyCode::Char('l') | KeyCode::Right => {
                if app.selected_index < app.selected_meetings().map(|m| m.len()).unwrap_or(0) {
                    Some(Msg::ViewMeeting(app.selected_index))
                } else {
                    None
                }
            }
            KeyCode::Char('n') => Some(Msg::NewMeeting),
            KeyCode::Char('?') => Some(Msg::ShowHelp),
            _ => None,
        },

        ViewMode::NoteEditor => match key.code {
            KeyCode::Esc => Some(Msg::Back),
            // Cursor movement
            KeyCode::Left => Some(Msg::CursorLeft),
            KeyCode::Right => Some(Msg::CursorRight),
            KeyCode::Up => Some(Msg::CursorUp),
            KeyCode::Down => Some(Msg::CursorDown),
            KeyCode::Home => Some(Msg::CursorHome),
            KeyCode::End => Some(Msg::CursorEnd),
            // Editing
            KeyCode::Char(c) => Some(Msg::Input(c)),
            KeyCode::Backspace => Some(Msg::Backspace),
            KeyCode::Delete => Some(Msg::DeleteChar),
            KeyCode::Enter => Some(Msg::Enter),
            // Mood
            KeyCode::F(1) => Some(Msg::UpdateMood(1)),
            KeyCode::F(2) => Some(Msg::UpdateMood(2)),
            KeyCode::F(3) => Some(Msg::UpdateMood(3)),
            KeyCode::F(4) => Some(Msg::UpdateMood(4)),
            KeyCode::F(5) => Some(Msg::UpdateMood(5)),
            _ => None,
        },

        ViewMode::NewEngineerModal => match key.code {
            KeyCode::Esc => Some(Msg::CancelModal),
            KeyCode::Char(c) => Some(Msg::Input(c)),
            KeyCode::Backspace => Some(Msg::Backspace),
            KeyCode::Enter | KeyCode::Tab => Some(Msg::Enter),
            _ => None,
        },

        ViewMode::Help => match key.code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('?') => Some(Msg::HideHelp),
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
