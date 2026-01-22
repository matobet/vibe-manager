//! Delete confirmation modal

use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use super::render_modal;
use crate::theme::{focused_block, style_header, style_muted, COLOR_SECONDARY};

/// Confirmation dialog for deleting a meeting
pub struct DeleteConfirmModal<'a> {
    meeting_date: &'a str,
}

impl<'a> DeleteConfirmModal<'a> {
    pub fn new(meeting_date: &'a str) -> Self {
        Self { meeting_date }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let modal_area = render_modal(frame, area, 40, 7);

        let block = focused_block("Delete Meeting");
        let inner = block.inner(modal_area);
        frame.render_widget(block, modal_area);

        let lines = vec![
            Line::from(""),
            Line::from(vec![
                Span::raw("Delete meeting from "),
                Span::styled(self.meeting_date, style_header()),
                Span::raw("?"),
            ]),
            Line::from(Span::styled("This cannot be undone.", style_muted())),
            Line::from(""),
            Line::from(vec![
                Span::styled("[y]", style_header()),
                Span::styled(
                    " Delete  ",
                    ratatui::style::Style::default().fg(COLOR_SECONDARY),
                ),
                Span::styled("[n]", style_header()),
                Span::raw(" Cancel"),
            ]),
        ];

        let para = Paragraph::new(lines).centered();
        frame.render_widget(para, inner);
    }
}
