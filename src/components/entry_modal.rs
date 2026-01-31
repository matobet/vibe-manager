//! Entry input modal for quick mood observations

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::model::Context;
use crate::theme::{
    focused_block, mood_color, mood_gauge, style_header, style_muted, COLOR_PRIMARY,
    COLOR_SECONDARY,
};

use super::modal::render_modal;

/// Entry input modal for recording mood observations
pub struct EntryInputModal<'a> {
    mood: Option<u8>,
    context: Context,
    notes: &'a str,
}

impl<'a> EntryInputModal<'a> {
    pub fn new(mood: Option<u8>, context: Context, notes: &'a str) -> Self {
        Self {
            mood,
            context,
            notes,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let modal_area = render_modal(frame, area, 50, 14);

        let block = focused_block("Record Observation");
        let inner = block.inner(modal_area);
        frame.render_widget(block, modal_area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Mood
                Constraint::Length(3), // Context
                Constraint::Length(3), // Notes
                Constraint::Length(2), // Help
            ])
            .margin(1)
            .split(inner);

        // Mood row
        self.render_mood_row(frame, chunks[0]);

        // Context row
        self.render_context_row(frame, chunks[1]);

        // Notes row
        self.render_notes_row(frame, chunks[2]);

        // Help text
        let help = Line::from(vec![
            Span::styled("1-5", style_header()),
            Span::raw(" Mood  "),
            Span::styled("Tab", style_header()),
            Span::raw(" Context  "),
            Span::styled("Enter", style_header()),
            Span::raw(" Save  "),
            Span::styled("Esc", style_header()),
            Span::raw(" Cancel"),
        ]);
        let help_para = Paragraph::new(help);
        frame.render_widget(help_para, chunks[3]);
    }

    fn render_mood_row(&self, frame: &mut Frame, area: Rect) {
        let mood_display = match self.mood {
            Some(m) => {
                let gauge = mood_gauge(m);
                Line::from(vec![
                    Span::styled(gauge, Style::default().fg(mood_color(m))),
                    Span::styled(format!(" ({}) ", m), Style::default().fg(mood_color(m))),
                ])
            }
            None => Line::from(Span::styled("Not set", style_muted())),
        };

        let lines = vec![
            Line::from(Span::styled("Mood:", style_header())),
            mood_display,
        ];

        let para = Paragraph::new(lines);
        frame.render_widget(para, area);
    }

    fn render_context_row(&self, frame: &mut Frame, area: Rect) {
        let contexts: Vec<Span> = Context::all()
            .iter()
            .map(|ctx| {
                let is_selected = *ctx == self.context;
                let style = if is_selected {
                    Style::default().fg(COLOR_SECONDARY)
                } else {
                    style_muted()
                };
                let prefix = if is_selected { "[" } else { " " };
                let suffix = if is_selected { "]" } else { " " };
                Span::styled(format!("{}{}{} ", prefix, ctx.as_str(), suffix), style)
            })
            .collect();

        let lines = vec![
            Line::from(Span::styled("Context:", style_header())),
            Line::from(contexts),
        ];

        let para = Paragraph::new(lines);
        frame.render_widget(para, area);
    }

    fn render_notes_row(&self, frame: &mut Frame, area: Rect) {
        let cursor = "█";
        let notes_display = if self.notes.is_empty() {
            Line::from(vec![
                Span::styled(cursor, Style::default().fg(COLOR_PRIMARY)),
                Span::styled(" (optional)", style_muted()),
            ])
        } else {
            // Truncate notes if too long
            let display_notes: String = self.notes.chars().take(35).collect();
            Line::from(vec![
                Span::styled(display_notes, Style::default().fg(COLOR_SECONDARY)),
                Span::styled(cursor, Style::default().fg(COLOR_PRIMARY)),
            ])
        };

        let lines = vec![
            Line::from(Span::styled("Notes:", style_header())),
            notes_display,
        ];

        let para = Paragraph::new(lines);
        frame.render_widget(para, area);
    }
}
