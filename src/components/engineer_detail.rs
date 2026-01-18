//! Engineer detail view component

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Cell, Paragraph, Row, Table, TableState},
    Frame,
};

use crate::model::{Engineer, EngineerSummary, Meeting};
use crate::theme::{
    format_meeting_frequency, format_days_since, mood_color, mood_gauge, rpg_block, simple_block,
    style_header, style_muted, style_success, style_title,
};

pub struct EngineerDetail<'a> {
    engineer: &'a Engineer,
    summary: &'a EngineerSummary,
    meetings: &'a [Meeting],
    selected_meeting: usize,
}

impl<'a> EngineerDetail<'a> {
    pub fn new(
        engineer: &'a Engineer,
        summary: &'a EngineerSummary,
        meetings: &'a [Meeting],
        selected_meeting: usize,
    ) -> Self {
        Self {
            engineer,
            summary,
            meetings,
            selected_meeting,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(11), // Profile info (including family on separate lines)
                Constraint::Min(10),    // Meeting history
            ])
            .split(area);

        self.render_profile(frame, chunks[0]);
        self.render_meetings(frame, chunks[1]);
    }

    fn render_profile(&self, frame: &mut Frame, area: Rect) {
        let profile = &self.engineer.profile;

        // Header with level badge: ★ P3  Engineer Name
        let level_badge = format!("★ {}", profile.level.as_deref().unwrap_or("-"));
        let mut lines = vec![
            Line::from(vec![
                Span::styled(level_badge, Style::default().fg(self.summary.color)),
                Span::raw("   "),
                Span::styled(&profile.name, style_title()),
            ]),
        ];

        if let Some(title) = &profile.title {
            lines.push(Line::from(Span::styled(title, style_muted())));
        }

        lines.push(Line::from(""));

        // 1-on-1 frequency and status
        lines.push(Line::from(vec![
            Span::raw("Meeting Frequency: "),
            Span::styled(format_meeting_frequency(&profile.meeting_frequency), style_success()),
            Span::raw("  │  "),
            Span::raw("Last 1-on-1: "),
            Span::raw(format_days_since(
                self.summary.days_since_meeting,
                self.engineer.meeting_frequency_days(),
            )),
        ]));

        // Mood
        if let Some(mood) = self.summary.recent_mood {
            lines.push(Line::from(vec![
                Span::raw("Mood: "),
                Span::styled(
                    mood_gauge(mood),
                    Style::default().fg(mood_color(mood)),
                ),
            ]));
        }

        // Personal info
        if profile.partner.is_some() || !profile.children.is_empty() {
            lines.push(Line::from(""));
            if let Some(partner) = &profile.partner {
                lines.push(Line::from(Span::styled(
                    format!("Partner: {}", partner),
                    style_muted(),
                )));
            }
            if !profile.children.is_empty() {
                lines.push(Line::from(Span::styled(
                    format!("Kids: {}", profile.children.join(", ")),
                    style_muted(),
                )));
            }
        }

        let para = Paragraph::new(lines).block(rpg_block("Profile"));
        frame.render_widget(para, area);
    }

    fn render_meetings(&self, frame: &mut Frame, area: Rect) {
        if self.meetings.is_empty() {
            let text = vec![
                Line::from(""),
                Line::from("No meetings recorded yet"),
                Line::from(""),
                Line::from(Span::styled("Press 'n' to create a new meeting note", style_muted())),
            ];
            let para = Paragraph::new(text)
                .block(simple_block("Meeting History"))
                .alignment(ratatui::layout::Alignment::Center);
            frame.render_widget(para, area);
            return;
        }

        let header_cells = ["Date", "Mood", "Preview"]
            .iter()
            .map(|h| Cell::from(*h).style(style_header()));
        let header = Row::new(header_cells).height(1);

        // Show meetings in reverse chronological order
        let rows: Vec<Row> = self
            .meetings
            .iter()
            .rev()
            .map(|m| {
                let mood_display = m.mood().map_or_else(
                    || "─────".to_string(),
                    |mood| mood_gauge(mood),
                );
                let mood_style = m.mood().map_or(
                    style_muted(),
                    |mood| Style::default().fg(mood_color(mood)),
                );

                // Get first non-empty line as preview
                let preview: String = m
                    .content
                    .lines()
                    .find(|l| !l.trim().is_empty() && !l.starts_with('#'))
                    .unwrap_or("")
                    .chars()
                    .take(40)
                    .collect();

                Row::new(vec![
                    Cell::from(m.date.format("%Y-%m-%d").to_string()),
                    Cell::from(mood_display).style(mood_style),
                    Cell::from(preview).style(style_muted()),
                ])
            })
            .collect();

        let widths = [
            Constraint::Length(12),
            Constraint::Length(8),
            Constraint::Min(20),
        ];

        let table = Table::new(rows, widths)
            .header(header)
            .block(simple_block("Meeting History"))
            .row_highlight_style(
                Style::default()
                    .add_modifier(ratatui::style::Modifier::REVERSED)
            );

        let mut state = TableState::default();
        // Adjust selection index since we reversed the list
        if !self.meetings.is_empty() {
            state.select(Some(self.meetings.len() - 1 - self.selected_meeting.min(self.meetings.len() - 1)));
        }

        frame.render_stateful_widget(table, area, &mut state);
    }
}
