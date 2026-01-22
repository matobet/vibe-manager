//! Markdown note viewer component (read-only display with markdown highlighting)

use std::borrow::Cow;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::model::Meeting;
use crate::theme::{
    mood_color, mood_gauge, rpg_block, simple_block, style_header, style_muted, COLOR_PRIMARY,
    COLOR_SECONDARY,
};

/// Empty mood display placeholder
const EMPTY_MOOD_DISPLAY: &str = "─────";

pub struct NoteViewer<'a> {
    meeting: &'a Meeting,
    content: &'a str,
    mood: Option<u8>,
}

impl<'a> NoteViewer<'a> {
    pub fn new(meeting: &'a Meeting, content: &'a str, mood: Option<u8>) -> Self {
        Self {
            meeting,
            content,
            mood,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header with date and mood
                Constraint::Min(10),   // Content viewer
                Constraint::Length(2), // Help line
            ])
            .split(area);

        self.render_header(frame, chunks[0]);
        self.render_content(frame, chunks[1]);
        self.render_help(frame, chunks[2]);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let date_str = self.meeting.date.format("%B %d, %Y").to_string();

        let mood_display: Cow<'static, str> = self.mood.map_or_else(
            || Cow::Borrowed(EMPTY_MOOD_DISPLAY),
            |m| Cow::Owned(mood_gauge(m)),
        );
        let mood_style = self
            .mood
            .map_or(style_muted(), |m| Style::default().fg(mood_color(m)));

        let lines = vec![Line::from(vec![
            Span::styled("Date: ", style_muted()),
            Span::styled(date_str, style_header()),
            Span::raw("    "),
            Span::styled("Mood: ", style_muted()),
            Span::styled(mood_display, mood_style),
        ])];

        let para = Paragraph::new(lines).block(simple_block("Meeting Note"));
        frame.render_widget(para, area);
    }

    fn render_content(&self, frame: &mut Frame, area: Rect) {
        let lines: Vec<Line> = self
            .content
            .lines()
            .map(|line_text| self.render_line(line_text))
            .collect();

        let para = Paragraph::new(lines).block(rpg_block("Content"));
        frame.render_widget(para, area);
    }

    fn render_line<'b>(&self, line: &'b str) -> Line<'b> {
        // Basic markdown highlighting
        // Use Cow to avoid allocations when borrowing is sufficient
        if line.starts_with("# ") {
            Line::from(Span::styled(Cow::Borrowed(line), style_header()))
        } else if line.starts_with("## ") {
            Line::from(Span::styled(
                Cow::Borrowed(line),
                Style::default().fg(COLOR_SECONDARY),
            ))
        } else if let Some(rest) = line.strip_prefix("- [ ]") {
            Line::from(vec![
                Span::styled(Cow::Borrowed("☐ "), style_muted()),
                Span::raw(Cow::Borrowed(rest)),
            ])
        } else if let Some(rest) = line
            .strip_prefix("- [x]")
            .or_else(|| line.strip_prefix("- [X]"))
        {
            Line::from(vec![
                Span::styled(Cow::Borrowed("☑ "), Style::default().fg(COLOR_PRIMARY)),
                Span::raw(Cow::Borrowed(rest)),
            ])
        } else if let Some(rest) = line.strip_prefix("- ") {
            Line::from(vec![
                Span::styled(Cow::Borrowed("• "), style_muted()),
                Span::raw(Cow::Borrowed(rest)),
            ])
        } else {
            Line::from(Cow::Borrowed(line))
        }
    }

    fn render_help(&self, frame: &mut Frame, area: Rect) {
        let help = Line::from(vec![
            Span::styled("e", style_header()),
            Span::raw(" Edit  "),
            Span::styled("Del", style_header()),
            Span::raw(" Delete  "),
            Span::styled("F1-F5", style_header()),
            Span::raw(" Mood  "),
            Span::styled("Esc/Bksp", style_header()),
            Span::raw(" Back"),
        ]);

        let para = Paragraph::new(help);
        frame.render_widget(para, area);
    }
}
