//! Engineer detail view component

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table, TableState},
    Frame,
};

use crate::model::{Engineer, EngineerSummary, Meeting};
use crate::theme::{
    format_days_ago, format_meeting_frequency, mood_color, mood_gauge, mood_gauge_with_value,
    mood_trend_icon, overdue_color, rpg_block, simple_block, sprites, style_header, style_muted,
    style_title, COLOR_MUTED,
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
                Constraint::Length(12), // Profile info with 2-column layout
                Constraint::Min(10),    // Meeting history
            ])
            .split(area);

        self.render_profile(frame, chunks[0]);
        self.render_meetings(frame, chunks[1]);
    }

    fn render_profile(&self, frame: &mut Frame, area: Rect) {
        let block = rpg_block("Engineer Profile");
        let inner = block.inner(area);
        frame.render_widget(block, area);

        // 2-column layout: avatar+identity on left, stats+bio on right
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // Left column: avatar + identity
                Constraint::Percentage(50), // Right column: stats + bio
            ])
            .split(inner);

        self.render_avatar_and_identity(frame, columns[0]);
        self.render_stats_and_bio(frame, columns[1]);
    }

    fn render_avatar_and_identity(&self, frame: &mut Frame, area: Rect) {
        let profile = &self.engineer.profile;

        // Stack avatar and identity vertically
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Top padding
                Constraint::Length(3), // Avatar (3 lines)
                Constraint::Length(1), // Spacing
                Constraint::Length(1), // Name
                Constraint::Length(1), // Title + level
                Constraint::Min(0),    // Remaining space
            ])
            .split(area);

        // Avatar
        let sprite = sprites::FaceSprite::from_summary(
            self.summary,
            Style::default().fg(self.summary.color),
        );
        let avatar_para = Paragraph::new(sprite.lines()).alignment(Alignment::Center);
        frame.render_widget(avatar_para, chunks[1]);

        // Name
        let name_line = Line::from(Span::styled(
            profile.name.clone(),
            style_title().add_modifier(Modifier::BOLD),
        ));
        let name_para = Paragraph::new(name_line).alignment(Alignment::Center);
        frame.render_widget(name_para, chunks[3]);

        // Title + level
        let title_text = profile.title.as_deref().unwrap_or("Engineer");
        let level_text = profile.level.as_deref().unwrap_or("-");
        let title_level_line = Line::from(vec![
            Span::styled(title_text, style_muted()),
            Span::styled("  ★ ", Style::default().fg(self.summary.color)),
            Span::styled(level_text, Style::default().fg(self.summary.color)),
        ]);
        let title_para = Paragraph::new(title_level_line).alignment(Alignment::Center);
        frame.render_widget(title_para, chunks[4]);
    }

    fn render_stats_and_bio(&self, frame: &mut Frame, area: Rect) {
        let profile = &self.engineer.profile;
        let has_bio = profile.partner.is_some() || !profile.children.is_empty();

        // Stack stats and bio vertically
        let panels = if has_bio {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(5), Constraint::Min(3)])
                .split(area)
        } else {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(4)])
                .split(area)
        };

        self.render_stats_panel(frame, panels[0]);

        if has_bio {
            self.render_bio_panel(frame, panels[1]);
        }
    }

    fn render_stats_panel(&self, frame: &mut Frame, area: Rect) {
        let profile = &self.engineer.profile;

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(COLOR_MUTED))
            .title(" ⚔ STATS ")
            .title_style(style_header());

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let mut rows = vec![];

        // Frequency row
        let freq_text = format_meeting_frequency(&profile.meeting_frequency);
        rows.push(Row::new(vec![
            Cell::from("Frequency"),
            Cell::from(freq_text).style(style_title()),
        ]));

        // Last 1-on-1 row
        let last_meeting = format_days_ago(self.summary.days_since_meeting);
        let last_meeting_style = if self.summary.is_overdue {
            Style::default().fg(overdue_color(true))
        } else {
            Style::default()
        };
        let last_meeting_prefix = if self.summary.is_overdue { "⚠ " } else { "" };
        rows.push(Row::new(vec![
            Cell::from("Last 1-on-1"),
            Cell::from(format!("{}{}", last_meeting_prefix, last_meeting))
                .style(last_meeting_style),
        ]));

        // Morale row
        if let Some(mood) = self.summary.recent_mood {
            let mood_display = mood_gauge_with_value(mood);
            let trend_icon = mood_trend_icon(self.summary.mood_trend);
            rows.push(Row::new(vec![
                Cell::from("Morale"),
                Cell::from(Line::from(vec![
                    Span::styled(mood_display, Style::default().fg(mood_color(mood))),
                    Span::raw(" "),
                    Span::styled(trend_icon, Style::default().fg(mood_color(mood))),
                ])),
            ]));
        }

        let table = Table::new(rows, [Constraint::Length(12), Constraint::Min(10)]);
        frame.render_widget(table, inner);
    }

    fn render_bio_panel(&self, frame: &mut Frame, area: Rect) {
        let profile = &self.engineer.profile;

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(COLOR_MUTED))
            .title(" ♥ BIO ")
            .title_style(style_header());

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let mut lines = vec![];

        if let Some(partner) = &profile.partner {
            lines.push(Line::from(vec![
                Span::raw("Partner: "),
                Span::styled(partner, style_muted()),
            ]));
        }

        if !profile.children.is_empty() {
            lines.push(Line::from(vec![
                Span::raw("Kids: "),
                Span::styled(profile.children.join(", "), style_muted()),
            ]));
        }

        let para = Paragraph::new(lines);
        frame.render_widget(para, inner);
    }

    fn render_meetings(&self, frame: &mut Frame, area: Rect) {
        if self.meetings.is_empty() {
            let text = vec![
                Line::from(""),
                Line::from("No meetings recorded yet"),
                Line::from(""),
                Line::from(Span::styled(
                    "Press 'n' to create a new meeting note",
                    style_muted(),
                )),
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
                let mood_display = m.mood().map_or_else(|| "─────".to_string(), mood_gauge);
                let mood_style = m
                    .mood()
                    .map_or(style_muted(), |mood| Style::default().fg(mood_color(mood)));

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
            .row_highlight_style(Style::default().add_modifier(ratatui::style::Modifier::REVERSED));

        let mut state = TableState::default();
        // selected_meeting 0 = first row in display (newest meeting)
        if !self.meetings.is_empty() {
            state.select(Some(self.selected_meeting.min(self.meetings.len() - 1)));
        }

        frame.render_stateful_widget(table, area, &mut state);
    }
}
