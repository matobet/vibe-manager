//! Input handling
//!
//! This module handles keyboard event mapping and event polling.

use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

use super::{App, Msg, ViewMode};

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
        ViewMode::Dashboard => handle_dashboard_key(key, lowercase_char),
        ViewMode::ReportDetail => handle_report_detail_key(app, key, lowercase_char),
        ViewMode::NoteViewer => handle_note_viewer_key(key, lowercase_char),
        ViewMode::DeleteConfirmModal => handle_delete_confirm_key(key, lowercase_char),
        ViewMode::NewReportModal => handle_new_report_modal_key(app, key),
        ViewMode::EntryInputModal => handle_entry_input_modal_key(key),
        ViewMode::Help => handle_help_key(key, lowercase_char),
    }
}

/// Handle keys in Dashboard view
fn handle_dashboard_key(key: KeyEvent, _lowercase_char: Option<char>) -> Option<Msg> {
    match key.code {
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
    }
}

/// Handle keys in ReportDetail view
fn handle_report_detail_key(
    app: &App,
    key: KeyEvent,
    _lowercase_char: Option<char>,
) -> Option<Msg> {
    match key.code {
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
    }
}

/// Handle keys in NoteViewer view
fn handle_note_viewer_key(key: KeyEvent, _lowercase_char: Option<char>) -> Option<Msg> {
    match key.code {
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
    }
}

/// Handle keys in DeleteConfirmModal view
fn handle_delete_confirm_key(key: KeyEvent, _lowercase_char: Option<char>) -> Option<Msg> {
    match key.code {
        KeyCode::Enter => Some(Msg::ConfirmDelete),
        KeyCode::Esc => Some(Msg::CancelModal),
        KeyCode::Char(c) => match c.to_ascii_lowercase() {
            'y' => Some(Msg::ConfirmDelete),
            'n' => Some(Msg::CancelModal),
            _ => None,
        },
        _ => None,
    }
}

/// Handle keys in NewReportModal view
fn handle_new_report_modal_key(app: &App, key: KeyEvent) -> Option<Msg> {
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

/// Handle keys in EntryInputModal view
fn handle_entry_input_modal_key(key: KeyEvent) -> Option<Msg> {
    match key.code {
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
    }
}

/// Handle keys in Help view
fn handle_help_key(key: KeyEvent, _lowercase_char: Option<char>) -> Option<Msg> {
    match key.code {
        KeyCode::Esc => Some(Msg::HideHelp),
        KeyCode::Char(c) => match c.to_ascii_lowercase() {
            'q' | '?' => Some(Msg::HideHelp),
            _ => None,
        },
        _ => None,
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
