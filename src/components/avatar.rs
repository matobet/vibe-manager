//! Engineer card components with color-coded borders and level badges
//!
//! Design Philosophy:
//! - Color-coded borders for quick engineer recognition
//! - Level badges (★ P3) instead of avatar portraits
//! - Compact cards showing name, mood, and meeting status

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::model::EngineerSummary;
use crate::theme::{
    format_days_ago, mood_color, mood_gauge, mood_trend_icon, overdue_color, overdue_icon,
    style_muted, style_title, COLOR_SECONDARY,
};

// ═══════════════════════════════════════════════════════════════════════════════
// ENGINEER CARD COMPONENT - For dashboard grid
// ═══════════════════════════════════════════════════════════════════════════════

pub struct AvatarCard<'a> {
    summary: &'a EngineerSummary,
    is_selected: bool,
}

impl<'a> AvatarCard<'a> {
    pub fn new(summary: &'a EngineerSummary, is_selected: bool) -> Self {
        Self { summary, is_selected }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        // Card border styled with engineer's color (color-coded for quick recognition)
        let block = if self.is_selected {
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(COLOR_SECONDARY))
                .title(format!(" ★ {} ★ ", self.summary.level))
                .title_alignment(Alignment::Center)
                .title_style(style_title())
        } else {
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(self.summary.color))
                .title(format!(" ★ {} ", self.summary.level))
                .title_alignment(Alignment::Center)
                .title_style(Style::default().fg(self.summary.color))
        };

        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Vertical layout: name, spacer, mood, status
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),  // Name
                Constraint::Min(1),     // Spacer
                Constraint::Length(1),  // Mood gauge
                Constraint::Length(1),  // Meeting status
            ])
            .split(inner);

        // Name (truncated, color-coded for quick recognition)
        let name: String = self.summary.name.chars().take(12).collect();
        let name_style = if self.is_selected {
            style_title()
        } else {
            Style::default()
                .fg(self.summary.color)
                .add_modifier(Modifier::BOLD)
        };
        let name_para = Paragraph::new(Line::from(Span::styled(name, name_style)))
            .alignment(Alignment::Center);
        frame.render_widget(name_para, chunks[0]);

        // Mood gauge with trend
        let mood_display = self.summary.recent_mood.map_or_else(
            || "─────".to_string(),
            |m| mood_gauge(m),
        );
        let trend_icon = mood_trend_icon(self.summary.mood_trend);
        let mood_style = self.summary.recent_mood.map_or(
            style_muted(),
            |m| Style::default().fg(mood_color(m)),
        );
        let mood_para = Paragraph::new(Line::from(vec![
            Span::styled(mood_display, mood_style),
            Span::styled(trend_icon, mood_style),
        ])).alignment(Alignment::Center);
        frame.render_widget(mood_para, chunks[2]);

        // Meeting status
        let status_icon = overdue_icon(self.summary.is_overdue);
        let status_color = overdue_color(self.summary.is_overdue);
        let days_text = format_days_ago(self.summary.days_since_meeting);
        let status_para = Paragraph::new(Line::from(vec![
            Span::styled(status_icon, Style::default().fg(status_color)),
            Span::raw(" "),
            Span::styled(days_text, Style::default().fg(status_color)),
        ])).alignment(Alignment::Center);
        frame.render_widget(status_para, chunks[3]);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// ENGINEER GRID - Dashboard layout
// ═══════════════════════════════════════════════════════════════════════════════

pub struct AvatarGrid<'a> {
    summaries: &'a [EngineerSummary],
    selected: usize,
}

impl<'a> AvatarGrid<'a> {
    pub fn new(summaries: &'a [EngineerSummary], selected: usize) -> Self {
        Self { summaries, selected }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        if self.summaries.is_empty() {
            return;
        }

        let card_width: u16 = 16;
        let card_height: u16 = 6;  // Compact height without avatar portrait

        let cards_per_row = (area.width / card_width).max(1) as usize;
        let num_rows = (self.summaries.len() + cards_per_row - 1) / cards_per_row;

        let row_constraints: Vec<Constraint> = (0..num_rows)
            .map(|_| Constraint::Length(card_height))
            .collect();

        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints(row_constraints)
            .split(area);

        for (row_idx, row_area) in rows.iter().enumerate() {
            let start_idx = row_idx * cards_per_row;
            let end_idx = (start_idx + cards_per_row).min(self.summaries.len());
            let row_summaries = &self.summaries[start_idx..end_idx];

            let col_constraints: Vec<Constraint> = row_summaries
                .iter()
                .map(|_| Constraint::Length(card_width))
                .collect();

            let cols = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(col_constraints)
                .split(*row_area);

            for (col_idx, (summary, col_area)) in row_summaries.iter().zip(cols.iter()).enumerate() {
                let global_idx = start_idx + col_idx;
                let is_selected = global_idx == self.selected;
                let card = AvatarCard::new(summary, is_selected);
                card.render(frame, *col_area);
            }
        }
    }
}
