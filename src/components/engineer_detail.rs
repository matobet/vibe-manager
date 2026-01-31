//! Engineer detail view component

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table, TableState},
    Frame,
};

use crate::model::{Engineer, EngineerSummary, JournalEntry, MoodTrend};
use crate::theme::{
    format_days_ago, format_meeting_frequency, mood_color, mood_gauge, mood_gauge_with_value,
    mood_trend_icon, overdue_color, rpg_block, simple_block, sprites, style_header, style_muted,
    style_title, COLOR_MUTED,
};


pub struct EngineerDetail<'a> {
    engineer: &'a Engineer,
    summary: &'a EngineerSummary,
    entries: &'a [JournalEntry],
    selected_entry: usize,
}

impl<'a> EngineerDetail<'a> {
    pub fn new(
        engineer: &'a Engineer,
        summary: &'a EngineerSummary,
        entries: &'a [JournalEntry],
        selected_entry: usize,
    ) -> Self {
        Self {
            engineer,
            summary,
            entries,
            selected_entry,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(16), // Profile info with 2-column layout
                Constraint::Min(8),     // Entry history
            ])
            .split(area);

        self.render_profile(frame, chunks[0]);
        self.render_entries(frame, chunks[1]);
    }

    fn render_profile(&self, frame: &mut Frame, area: Rect) {
        let block = rpg_block("Engineer Profile");
        let inner = block.inner(area);
        frame.render_widget(block, area);

        // 2-column layout
        // Left: avatar+name (compact) + mood chart
        // Right: stats + bio
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // Left column: avatar + mood chart
                Constraint::Percentage(50), // Right column: stats + bio
            ])
            .split(inner);

        self.render_left_column(frame, columns[0]);
        self.render_right_column(frame, columns[1]);
    }

    fn render_left_column(&self, frame: &mut Frame, area: Rect) {
        let profile = &self.engineer.profile;

        // Stack: avatar+name (compact) then mood chart
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5), // Avatar + name + title (compact)
                Constraint::Min(6),    // Mood chart
            ])
            .split(area);

        // Avatar + name + title in compact area
        let identity_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(12), // Avatar
                Constraint::Min(10),    // Name + title
            ])
            .split(chunks[0]);

        // Avatar (3 lines, centered vertically in 5-line area)
        let sprite = sprites::FaceSprite::from_summary(
            self.summary,
            Style::default().fg(self.summary.color),
        );
        let avatar_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Length(3), Constraint::Min(0)])
            .split(identity_chunks[0]);
        let avatar_para = Paragraph::new(sprite.lines()).alignment(Alignment::Center);
        frame.render_widget(avatar_para, avatar_area[1]);

        // Name + title (stacked vertically)
        let name_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // padding
                Constraint::Length(1), // name
                Constraint::Length(1), // title + level
                Constraint::Min(0),
            ])
            .split(identity_chunks[1]);

        let name_line = Line::from(Span::styled(
            profile.name.clone(),
            style_title().add_modifier(Modifier::BOLD),
        ));
        frame.render_widget(Paragraph::new(name_line), name_area[1]);

        let title_text = profile.title.as_deref().unwrap_or("Engineer");
        let level_text = profile.level.as_deref().unwrap_or("-");
        let title_level_line = Line::from(vec![
            Span::styled(title_text, style_muted()),
            Span::styled(" ★ ", Style::default().fg(self.summary.color)),
            Span::styled(level_text, Style::default().fg(self.summary.color)),
        ]);
        frame.render_widget(Paragraph::new(title_level_line), name_area[2]);

        // Mood chart below
        self.render_mood_history_panel(frame, chunks[1]);
    }

    fn render_right_column(&self, frame: &mut Frame, area: Rect) {
        let profile = &self.engineer.profile;
        let has_bio = profile.partner.is_some() || !profile.children.is_empty();

        let rows = if has_bio {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(5), Constraint::Length(4)])
                .split(area)
        } else {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(5)])
                .split(area)
        };

        self.render_stats_panel(frame, rows[0]);

        if has_bio {
            self.render_bio_panel(frame, rows[1]);
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
            let trend_style = match self.summary.mood_trend {
                Some(MoodTrend::Rising) => Style::default().fg(Color::Green).bold(),
                Some(MoodTrend::Falling) => Style::default().fg(Color::Rgb(255, 140, 0)).bold(),
                _ => Style::default(),
            };
            rows.push(Row::new(vec![
                Cell::from("Morale"),
                Cell::from(Line::from(vec![
                    Span::styled(mood_display, Style::default().fg(mood_color(mood))),
                    Span::raw(" "),
                    Span::styled(trend_icon, trend_style),
                ])),
            ]));
        }

        let table = Table::new(rows, [Constraint::Length(12), Constraint::Min(10)]);
        frame.render_widget(table, inner);
    }

    fn render_mood_history_panel(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(COLOR_MUTED))
            .title(" ♥ MORALE HISTORY ")
            .title_style(style_header());

        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Get mood data points
        let mood_data: Vec<(chrono::NaiveDateTime, u8)> = self
            .entries
            .iter()
            .filter_map(|e| e.mood().map(|m| (e.timestamp, m)))
            .collect();

        if mood_data.is_empty() {
            let text = Paragraph::new(Span::styled("No mood data yet", style_muted()))
                .alignment(Alignment::Center);
            frame.render_widget(text, inner);
            return;
        }

        // Chart dimensions
        let chart_width = inner.width.saturating_sub(3) as usize; // Leave room for Y-axis

        // Take last N data points that fit
        let display_count = mood_data.len().min(chart_width);
        let start = mood_data.len().saturating_sub(display_count);
        let data = &mood_data[start..];

        let mut lines: Vec<Line> = Vec::new();

        // Chart rows from top (5) to bottom (1)
        for level in (1..=5).rev() {
            let mut spans = vec![Span::styled(
                format!("{}│", level),
                Style::default().fg(COLOR_MUTED),
            )];

            for (_, mood) in data.iter() {
                if *mood == level {
                    spans.push(Span::styled("●", Style::default().fg(mood_color(*mood))));
                } else if *mood > level {
                    spans.push(Span::styled("│", Style::default().fg(mood_color(*mood))));
                } else {
                    spans.push(Span::raw(" "));
                }
            }

            lines.push(Line::from(spans));
        }

        // X-axis line
        let axis_width = data.len().min(chart_width);
        let x_axis = format!(" └{}", "─".repeat(axis_width));
        lines.push(Line::from(Span::styled(
            x_axis,
            Style::default().fg(COLOR_MUTED),
        )));

        // X-axis labels (first and last dates)
        if !data.is_empty() {
            let first_date = data.first().unwrap().0.format("%b %d").to_string();
            let last_date = data.last().unwrap().0.format("%b %d").to_string();

            // Build label line: "  Jan 01          Jan 20"
            let label_width = axis_width + 2; // +2 for "└" prefix space
            let padding = label_width.saturating_sub(first_date.len() + last_date.len());

            let label_line = Line::from(vec![
                Span::styled(format!("  {}", first_date), style_muted()),
                Span::raw(" ".repeat(padding)),
                Span::styled(last_date, style_muted()),
            ]);
            lines.push(label_line);
        }

        let para = Paragraph::new(lines);
        frame.render_widget(para, inner);
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
                Span::styled(partner.clone(), style_muted()),
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

    fn render_entries(&self, frame: &mut Frame, area: Rect) {
        // Only show meetings (entries with content), not pure mood observations
        let meetings: Vec<&JournalEntry> = self.entries.iter().filter(|e| e.is_meeting()).collect();

        if meetings.is_empty() {
            let text = vec![
                Line::from(""),
                Line::from("No 1-on-1s recorded yet"),
                Line::from(""),
                Line::from(Span::styled(
                    "Press 'n' for new meeting, 'm' for mood observation",
                    style_muted(),
                )),
            ];
            let para = Paragraph::new(text)
                .block(simple_block("1-on-1 History"))
                .alignment(ratatui::layout::Alignment::Center);
            frame.render_widget(para, area);
            return;
        }

        let header_cells = ["Date", "Mood", "Preview"]
            .iter()
            .map(|h| Cell::from(*h).style(style_header()));
        let header = Row::new(header_cells).height(1);

        // Show meetings in reverse chronological order
        let rows: Vec<Row> = meetings
            .iter()
            .rev()
            .map(|e| {
                let mood_display = e.mood().map_or_else(|| "─────".to_string(), mood_gauge);
                let mood_style = e
                    .mood()
                    .map_or(style_muted(), |mood| Style::default().fg(mood_color(mood)));

                // Get first non-empty line as preview
                let preview: String = e
                    .content
                    .lines()
                    .find(|l| !l.trim().is_empty() && !l.starts_with('#'))
                    .unwrap_or("")
                    .chars()
                    .take(40)
                    .collect();

                Row::new(vec![
                    Cell::from(e.date().format("%Y-%m-%d").to_string()),
                    Cell::from(mood_display).style(mood_style),
                    Cell::from(preview).style(style_muted()),
                ])
            })
            .collect();

        let widths = [
            Constraint::Length(12), // Date
            Constraint::Length(6),  // Mood
            Constraint::Min(20),    // Preview
        ];

        let table = Table::new(rows, widths)
            .header(header)
            .block(simple_block("1-on-1 History"))
            .row_highlight_style(Style::default().add_modifier(ratatui::style::Modifier::REVERSED));

        let mut state = TableState::default();
        // selected_entry 0 = first row in display (newest meeting)
        if !meetings.is_empty() {
            state.select(Some(self.selected_entry.min(meetings.len() - 1)));
        }

        frame.render_stateful_widget(table, area, &mut state);
    }

    /// Get the number of meetings (for external use)
    pub fn meeting_count(&self) -> usize {
        self.entries.iter().filter(|e| e.is_meeting()).count()
    }
}
