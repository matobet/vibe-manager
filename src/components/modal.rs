//! Modal dialog system

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Clear, Paragraph},
    Frame,
};

use crate::theme::{
    focused_block, style_header, style_muted, COLOR_PRIMARY, COLOR_SECONDARY,
};

/// Render a centered modal dialog
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

/// New engineer modal
pub struct NewEngineerModal<'a> {
    fields: &'a [String],
    current_field: usize,
}

impl<'a> NewEngineerModal<'a> {
    pub fn new(fields: &'a [String], current_field: usize) -> Self {
        Self { fields, current_field }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let modal_area = render_modal(frame, area, 50, 14);

        let block = focused_block("New Engineer");
        let inner = block.inner(modal_area);
        frame.render_widget(block, modal_area);

        let field_labels = ["Name:", "Level:", "Meeting Frequency:"];
        let field_hints = ["e.g. Alex Chen", "P1-P5", "weekly/biweekly/monthly"];

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(2),
            ])
            .margin(1)
            .split(inner);

        for (i, (label, hint)) in field_labels.iter().zip(field_hints.iter()).enumerate() {
            let value = self.fields.get(i).map(|s| s.as_str()).unwrap_or("");
            let is_active = i == self.current_field;

            let style = if is_active {
                Style::default().fg(COLOR_SECONDARY)
            } else {
                Style::default()
            };

            let cursor = if is_active { "█" } else { "" };

            let lines = vec![
                Line::from(Span::styled(*label, style_header())),
                Line::from(vec![
                    Span::styled(value, style),
                    Span::styled(cursor, Style::default().fg(COLOR_PRIMARY)),
                    if value.is_empty() {
                        Span::styled(format!(" {}", hint), style_muted())
                    } else {
                        Span::raw("")
                    },
                ]),
            ];

            let para = Paragraph::new(lines);
            frame.render_widget(para, chunks[i]);
        }

        // Help text
        let help = Line::from(vec![
            Span::styled("Tab/Enter", style_header()),
            Span::raw(" Next field  "),
            Span::styled("Esc", style_header()),
            Span::raw(" Cancel"),
        ]);
        let help_para = Paragraph::new(help);
        frame.render_widget(help_para, chunks[3]);
    }
}

/// Help modal
pub struct HelpModal;

impl HelpModal {
    pub fn render(frame: &mut Frame, area: Rect) {
        let modal_area = render_modal(frame, area, 60, 20);

        let block = focused_block("Help");
        let inner = block.inner(modal_area);
        frame.render_widget(block, modal_area);

        let sections = vec![
            ("Party View", vec![
                ("h/l or ←/→", "Select party member"),
                ("j/k or ↑/↓", "Navigate grid"),
                ("Enter/Space", "View member details"),
                ("n", "Recruit new member"),
                ("g/G", "Jump to first/last"),
                ("r", "Refresh data"),
                ("q", "Quit"),
            ]),
            ("Member Details", vec![
                ("n", "New 1-on-1 meeting"),
                ("Enter", "View meeting notes"),
                ("Esc", "Back to party view"),
            ]),
            ("Meeting Notes", vec![
                ("←↑↓→", "Move cursor"),
                ("Home/End", "Start/end of line"),
                ("Del", "Delete character"),
                ("Ctrl+S", "Save note"),
                ("F1-F5", "Set mood (1-5)"),
                ("Esc", "Back"),
            ]),
        ];

        let mut lines = Vec::new();
        for (section, bindings) in sections {
            lines.push(Line::from(Span::styled(section, style_header())));
            for (key, desc) in bindings {
                lines.push(Line::from(vec![
                    Span::styled(format!("  {:12}", key), Style::default().fg(COLOR_SECONDARY)),
                    Span::raw(desc),
                ]));
            }
            lines.push(Line::from(""));
        }

        lines.push(Line::from(Span::styled("Press ? or Esc to close", style_muted())));

        let para = Paragraph::new(lines);
        frame.render_widget(para, inner);
    }
}

