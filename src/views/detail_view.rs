//! Engineer detail view layout

use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::app::{App, ViewMode};
use crate::components::{
    DeleteConfirmModal, EngineerDetail, EntryInputModal, HelpModal, NoteViewer, StatusBar,
};

pub fn render_detail_view(app: &App, frame: &mut Frame) {
    let size = frame.area();

    // Get selected engineer data
    let eng_idx = match app.selected_engineer_index {
        Some(idx) => idx,
        None => return,
    };

    let engineer = &app.engineers[eng_idx];
    let summary = &app.summaries[eng_idx];
    let entries = &app.entries_by_engineer[eng_idx];

    // Main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(1)])
        .split(size);

    // Render engineer detail
    let detail = EngineerDetail::new(engineer, summary, entries, app.selected_index);
    detail.render(frame, chunks[0]);

    // Render status bar
    let meeting_count = entries.iter().filter(|e| e.is_meeting()).count();
    let context = format!("{} • {} meetings", engineer.profile.name, meeting_count);
    let status = StatusBar::new(app.view_mode, &context, app.status_text());
    status.render(frame, chunks[1]);

    // Render help modal if active
    if app.view_mode == ViewMode::Help {
        HelpModal::render(frame, size);
    }

    // Render entry input modal if active
    if app.view_mode == ViewMode::EntryInputModal {
        let modal = EntryInputModal::new(
            app.pending_entry_mood,
            app.pending_entry_context,
            &app.pending_entry_notes,
        );
        modal.render(frame, size);
    }

    // Render delete confirmation modal if active (triggered from entry list)
    if app.view_mode == ViewMode::DeleteConfirmModal {
        if let Some(entry_idx) = app.selected_entry_index {
            let entry = &entries[entry_idx];
            let date_str = entry.date().format("%Y-%m-%d").to_string();
            DeleteConfirmModal::new(&date_str).render(frame, size);
        }
    }
}

pub fn render_viewer_view(app: &App, frame: &mut Frame) {
    let size = frame.area();

    // Get selected entry
    let eng_idx = match app.selected_engineer_index {
        Some(idx) => idx,
        None => return,
    };
    let entry_idx = match app.selected_entry_index {
        Some(idx) => idx,
        None => return,
    };

    let engineer = &app.engineers[eng_idx];
    let entry = &app.entries_by_engineer[eng_idx][entry_idx];

    // Main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(1)])
        .split(size);

    // Render viewer
    let viewer = NoteViewer::new(entry, &app.editor_content, app.editor_mood);
    viewer.render(frame, chunks[0]);

    // Render status bar
    let context = format!(
        "{} • {}",
        engineer.profile.name,
        entry.date().format("%Y-%m-%d")
    );
    let status = StatusBar::new(app.view_mode, &context, app.status_text());
    status.render(frame, chunks[1]);

    // Render delete confirmation modal if active
    if app.view_mode == ViewMode::DeleteConfirmModal {
        let date_str = entry.date().format("%Y-%m-%d").to_string();
        DeleteConfirmModal::new(&date_str).render(frame, size);
    }
}
