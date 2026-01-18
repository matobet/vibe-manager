//! Engineer detail view layout

use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::app::{App, ViewMode};
use crate::components::{EngineerDetail, HelpModal, NoteEditor, StatusBar};

pub fn render_detail_view(app: &App, frame: &mut Frame) {
    let size = frame.area();

    // Get selected engineer data
    let eng_idx = match app.selected_engineer_index {
        Some(idx) => idx,
        None => return,
    };

    let engineer = &app.engineers[eng_idx];
    let summary = &app.summaries[eng_idx];
    let meetings = &app.meetings_by_engineer[eng_idx];

    // Main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(1)])
        .split(size);

    // Render engineer detail
    let detail = EngineerDetail::new(engineer, summary, meetings, app.selected_index);
    detail.render(frame, chunks[0]);

    // Render status bar
    let context = format!("{} • {} meetings", engineer.profile.name, meetings.len());
    let status = StatusBar::new(
        app.view_mode,
        &context,
        app.status_message.as_deref(),
    );
    status.render(frame, chunks[1]);

    // Render help modal if active
    if app.view_mode == ViewMode::Help {
        HelpModal::render(frame, size);
    }
}

pub fn render_editor_view(app: &App, frame: &mut Frame) {
    let size = frame.area();

    // Get selected meeting
    let eng_idx = match app.selected_engineer_index {
        Some(idx) => idx,
        None => return,
    };
    let meet_idx = match app.selected_meeting_index {
        Some(idx) => idx,
        None => return,
    };

    let engineer = &app.engineers[eng_idx];
    let meeting = &app.meetings_by_engineer[eng_idx][meet_idx];

    // Main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(1)])
        .split(size);

    // Render editor
    let editor = NoteEditor::new(meeting, &app.editor_content, app.editor_cursor, app.editor_mood);
    editor.render(frame, chunks[0]);

    // Render status bar
    let context = format!(
        "{} • {}",
        engineer.profile.name,
        meeting.date.format("%Y-%m-%d")
    );
    let status = StatusBar::new(
        app.view_mode,
        &context,
        app.status_message.as_deref(),
    );
    status.render(frame, chunks[1]);
}
