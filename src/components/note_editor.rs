//! Markdown note editor component with cursor support

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::model::Meeting;
use crate::theme::{
    mood_color, mood_gauge, rpg_block, simple_block, style_header, style_muted,
    COLOR_PRIMARY, COLOR_SECONDARY,
};

pub struct NoteEditor<'a> {
    meeting: &'a Meeting,
    content: &'a str,
    cursor: usize,
    mood: Option<u8>,
}

impl<'a> NoteEditor<'a> {
    pub fn new(meeting: &'a Meeting, content: &'a str, cursor: usize, mood: Option<u8>) -> Self {
        Self { meeting, content, cursor, mood }
    }

    /// Get cursor line and column from position
    fn cursor_line_col(&self) -> (usize, usize) {
        let before_cursor = &self.content[..self.cursor.min(self.content.len())];
        let line = before_cursor.matches('\n').count();
        let last_newline = before_cursor.rfind('\n').map(|i| i + 1).unwrap_or(0);
        let col = self.cursor.min(self.content.len()) - last_newline;
        (line, col)
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Header with date and mood
                Constraint::Min(10),    // Editor
                Constraint::Length(2),  // Help line
            ])
            .split(area);

        self.render_header(frame, chunks[0]);
        self.render_editor(frame, chunks[1]);
        self.render_help(frame, chunks[2]);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let date_str = self.meeting.date.format("%B %d, %Y").to_string();
        let (line, col) = self.cursor_line_col();

        let mood_display = self.mood.map_or_else(
            || "─────".to_string(),
            |m| mood_gauge(m),
        );
        let mood_style = self.mood.map_or(
            style_muted(),
            |m| Style::default().fg(mood_color(m)),
        );

        let lines = vec![
            Line::from(vec![
                Span::styled("Date: ", style_muted()),
                Span::styled(&date_str, style_header()),
                Span::raw("    "),
                Span::styled("Mood: ", style_muted()),
                Span::styled(mood_display, mood_style),
                Span::raw("    "),
                Span::styled(format!("Ln {}, Col {}", line + 1, col + 1), style_muted()),
            ]),
        ];

        let para = Paragraph::new(lines).block(simple_block("Meeting Note"));
        frame.render_widget(para, area);
    }

    fn render_editor(&self, frame: &mut Frame, area: Rect) {
        let (cursor_line, cursor_col) = self.cursor_line_col();

        let lines: Vec<Line> = self
            .content
            .lines()
            .enumerate()
            .map(|(line_idx, line_text)| {
                // Check if cursor is on this line
                let is_cursor_line = line_idx == cursor_line;

                if is_cursor_line {
                    // Insert cursor into this line
                    self.render_line_with_cursor(line_text, cursor_col)
                } else {
                    // Regular line rendering with markdown highlighting
                    self.render_line(line_text)
                }
            })
            .collect();

        // Handle empty content or cursor at end after last newline
        let mut display_lines = lines;
        if display_lines.is_empty() || self.content.ends_with('\n') {
            // Cursor is on a new empty line
            let last_line_idx = display_lines.len();
            if cursor_line == last_line_idx {
                display_lines.push(Line::from(vec![
                    Span::styled("█", Style::default().fg(COLOR_PRIMARY).add_modifier(Modifier::SLOW_BLINK)),
                ]));
            } else if display_lines.is_empty() {
                display_lines.push(Line::from(vec![
                    Span::styled("█", Style::default().fg(COLOR_PRIMARY).add_modifier(Modifier::SLOW_BLINK)),
                ]));
            }
        }

        let para = Paragraph::new(display_lines).block(rpg_block("Content"));
        frame.render_widget(para, area);
    }

    fn render_line(&self, line: &str) -> Line<'static> {
        // Basic markdown highlighting
        if line.starts_with("# ") {
            Line::from(Span::styled(line.to_string(), style_header()))
        } else if line.starts_with("## ") {
            Line::from(Span::styled(line.to_string(), Style::default().fg(COLOR_SECONDARY)))
        } else if line.starts_with("- [ ]") {
            Line::from(vec![
                Span::styled("☐ ", style_muted()),
                Span::raw(line[5..].to_string()),
            ])
        } else if line.starts_with("- [x]") || line.starts_with("- [X]") {
            Line::from(vec![
                Span::styled("☑ ", Style::default().fg(COLOR_PRIMARY)),
                Span::raw(line[5..].to_string()),
            ])
        } else if line.starts_with("- ") {
            Line::from(vec![
                Span::styled("• ", style_muted()),
                Span::raw(line[2..].to_string()),
            ])
        } else {
            Line::from(line.to_string())
        }
    }

    fn render_line_with_cursor(&self, line: &str, cursor_col: usize) -> Line<'static> {
        let cursor_style = Style::default()
            .fg(COLOR_PRIMARY)
            .add_modifier(Modifier::SLOW_BLINK);

        // Handle markdown prefixes
        let (prefix, content, prefix_offset) = if line.starts_with("# ") {
            (Some(Span::styled("# ".to_string(), style_header())), &line[2..], 2)
        } else if line.starts_with("## ") {
            (Some(Span::styled("## ".to_string(), Style::default().fg(COLOR_SECONDARY))), &line[3..], 3)
        } else if line.starts_with("- [ ]") {
            (Some(Span::styled("☐ ".to_string(), style_muted())), &line[5..], 5)
        } else if line.starts_with("- [x]") || line.starts_with("- [X]") {
            (Some(Span::styled("☑ ".to_string(), Style::default().fg(COLOR_PRIMARY))), &line[5..], 5)
        } else if line.starts_with("- ") {
            (Some(Span::styled("• ".to_string(), style_muted())), &line[2..], 2)
        } else {
            (None, line, 0)
        };

        let mut spans = Vec::new();

        // Add prefix if present
        if let Some(p) = prefix {
            spans.push(p);
        }

        // Adjust cursor column for prefix transformation
        let adjusted_col = if cursor_col >= prefix_offset {
            cursor_col - prefix_offset
        } else {
            // Cursor is in the prefix area, put it at start of content
            0
        };

        // Split content at cursor position
        let char_count = content.chars().count();
        if adjusted_col >= char_count {
            // Cursor at end of line
            spans.push(Span::raw(content.to_string()));
            spans.push(Span::styled("█", cursor_style));
        } else {
            // Cursor in middle of line
            let before: String = content.chars().take(adjusted_col).collect();
            let cursor_char: String = content.chars().skip(adjusted_col).take(1).collect();
            let after: String = content.chars().skip(adjusted_col + 1).collect();

            if !before.is_empty() {
                spans.push(Span::raw(before));
            }
            // Highlight the character under cursor
            spans.push(Span::styled(
                if cursor_char.is_empty() { "█".to_string() } else { cursor_char },
                cursor_style.add_modifier(Modifier::REVERSED),
            ));
            if !after.is_empty() {
                spans.push(Span::raw(after));
            }
        }

        Line::from(spans)
    }

    fn render_help(&self, frame: &mut Frame, area: Rect) {
        let help = Line::from(vec![
            Span::styled("Ctrl+S", style_header()),
            Span::raw(" Save  "),
            Span::styled("←↑↓→", style_header()),
            Span::raw(" Move  "),
            Span::styled("Home/End", style_header()),
            Span::raw(" Line  "),
            Span::styled("F1-F5", style_header()),
            Span::raw(" Mood  "),
            Span::styled("Esc", style_header()),
            Span::raw(" Back"),
        ]);

        let para = Paragraph::new(help);
        frame.render_widget(para, area);
    }
}
