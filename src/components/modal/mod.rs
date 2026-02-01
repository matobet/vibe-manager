//! Modal dialog system
//!
//! This module provides modal dialogs for user input and information display.
//! Modals are rendered as centered overlays that capture keyboard input.

mod help;
mod new_report;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Clear,
    Frame,
};

// Re-export public API
pub use help::HelpModal;
pub use new_report::{NewReportField, NewReportModal, NewReportState};

/// Render a centered modal dialog and return the inner area
///
/// This function calculates the centered position for a modal of the given
/// dimensions, clears the background, and returns the modal area for rendering.
///
/// # Arguments
/// * `frame` - The frame to render into
/// * `area` - The total available area
/// * `width` - Desired modal width
/// * `height` - Desired modal height
///
/// # Returns
/// The centered Rect where the modal content should be rendered
pub fn render_modal(frame: &mut Frame, area: Rect, width: u16, height: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((area.height.saturating_sub(height)) / 2),
            Constraint::Length(height),
            Constraint::Min(0),
        ])
        .split(area);

    let popup_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length((area.width.saturating_sub(width)) / 2),
            Constraint::Length(width),
            Constraint::Min(0),
        ])
        .split(popup_layout[1])[1];

    // Clear the area behind the modal
    frame.render_widget(Clear, popup_area);

    popup_area
}
