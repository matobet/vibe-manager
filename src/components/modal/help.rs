//! Help modal
//!
//! Modal dialog displaying keyboard shortcuts and usage help.

use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::theme::{focused_block, style_header, style_muted, COLOR_SECONDARY};

/// Help modal showing keyboard shortcuts
pub struct HelpModal;

impl HelpModal {
    /// Render the help modal
    pub fn render(frame: &mut Frame, area: Rect) {
        let modal_area = super::render_modal(frame, area, 60, 20);

        let block = focused_block("Help");
        let inner = block.inner(modal_area);
        frame.render_widget(block, modal_area);

        let sections = vec![
            (
                "Party View",
                vec![
                    ("h/l or ←/→", "Select party member"),
                    ("j/k or ↑/↓", "Navigate grid"),
                    ("Enter/Space", "View member details"),
                    ("n", "Recruit new member"),
                    ("g/G", "Jump to first/last"),
                    ("r", "Refresh data"),
                    ("q", "Quit"),
                ],
            ),
            (
                "Member Details",
                vec![
                    ("n", "New 1-on-1 meeting"),
                    ("m", "Record mood observation"),
                    ("Enter", "View entry notes"),
                    ("Del", "Delete entry"),
                    ("Esc", "Back to party view"),
                ],
            ),
            (
                "Meeting Notes",
                vec![
                    ("←↑↓→", "Move cursor"),
                    ("Home/End", "Start/end of line"),
                    ("Del", "Delete character"),
                    ("Ctrl+S", "Save note"),
                    ("F1-F5", "Set mood (1-5)"),
                    ("Esc", "Back"),
                ],
            ),
        ];

        let mut lines = Vec::new();
        for (section, bindings) in sections {
            lines.push(Line::from(Span::styled(section, style_header())));
            for (key, desc) in bindings {
                lines.push(Line::from(vec![
                    Span::styled(
                        format!("  {:12}", key),
                        Style::default().fg(COLOR_SECONDARY),
                    ),
                    Span::raw(desc),
                ]));
            }
            lines.push(Line::from(""));
        }

        lines.push(Line::from(Span::styled(
            "Press ? or Esc to close",
            style_muted(),
        )));

        let para = Paragraph::new(lines);
        frame.render_widget(para, inner);
    }
}
