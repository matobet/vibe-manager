//! Dashboard view - main layout for team overview

use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::app::{App, ViewMode};
use crate::components::{
    render_empty_state, Dashboard, HallHeader, HelpModal, NewReportModal, StatusBar,
};
use crate::model::compute_team_metrics;

/// Build the hall header from the navigation stack (None at root)
fn hall_header(app: &App) -> Option<HallHeader> {
    let current = app.hall_stack.last()?;

    // YOU ▸ CHRIS ▸ TAYLOR'S SQUAD — first names, possessive on the current hall
    let mut breadcrumb = String::from("YOU");
    for frame in &app.hall_stack {
        let first_name = frame.name.split_whitespace().next().unwrap_or(&frame.name);
        breadcrumb.push_str(" ▸ ");
        breadcrumb.push_str(&first_name.to_uppercase());
    }
    breadcrumb.push_str("'S SQUAD");

    let metrics = compute_team_metrics(&app.summaries);
    let first_name = current
        .name
        .split_whitespace()
        .next()
        .unwrap_or(&current.name);
    Some(HallHeader {
        breadcrumb,
        member_count: app.summaries.len(),
        health_score: metrics.team_health_score,
        block_title: format!("{}'s Squad", first_name),
    })
}

pub fn render_dashboard_view(app: &App, frame: &mut Frame) {
    let size = frame.area();

    // Main layout: content area + status bar
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(1)])
        .split(size);

    // Render main content
    if app.reports.is_empty() {
        render_empty_state(frame, chunks[0]);
    } else {
        let mut dashboard =
            Dashboard::new(&app.summaries, &app.workspace_summary, app.selected_index);
        if let Some(hall) = hall_header(app) {
            dashboard = dashboard.with_hall(hall);
        }
        dashboard.render(frame, chunks[0]);
    }

    // Render status bar
    let context = if app.hall_stack.is_empty() {
        format!(
            "{} reports • {} overdue",
            app.workspace_summary.active_count, app.workspace_summary.overdue_count
        )
    } else {
        format!(
            "{} members • {} overdue",
            app.workspace_summary.active_count, app.workspace_summary.overdue_count
        )
    };
    let status = StatusBar::new(app.view_mode, &context, app.status_text())
        .in_hall(!app.hall_stack.is_empty());
    status.render(frame, chunks[1]);

    // Render modals on top
    match app.view_mode {
        ViewMode::Help => {
            HelpModal::render(frame, size);
        }
        ViewMode::NewReportModal => {
            let modal = NewReportModal::new(&app.new_report_state);
            modal.render(frame, size);
        }
        _ => {}
    }
}
