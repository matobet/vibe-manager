//! Dashboard view - main layout for team overview

use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::app::{App, ViewMode};
use crate::components::{Dashboard, HelpModal, NewEngineerModal, StatusBar, render_empty_state};

pub fn render_dashboard_view(app: &App, frame: &mut Frame) {
    let size = frame.area();

    // Main layout: content area + status bar
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(1)])
        .split(size);

    // Render main content
    if app.engineers.is_empty() {
        render_empty_state(frame, chunks[0]);
    } else {
        let dashboard = Dashboard::new(
            &app.summaries,
            &app.workspace_summary,
            app.selected_index,
        );
        dashboard.render(frame, chunks[0]);
    }

    // Render status bar
    let context = format!(
        "{} engineers • {} overdue",
        app.workspace_summary.active_count,
        app.workspace_summary.overdue_count
    );
    let status = StatusBar::new(
        app.view_mode,
        &context,
        app.status_message.as_deref(),
    );
    status.render(frame, chunks[1]);

    // Render modals on top
    match app.view_mode {
        ViewMode::Help => {
            HelpModal::render(frame, size);
        }
        ViewMode::NewEngineerModal => {
            let modal = NewEngineerModal::new(&app.modal_fields, app.modal_field_index);
            modal.render(frame, size);
        }
        _ => {}
    }
}
