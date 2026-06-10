//! Report card components with color-coded borders and kaomoji avatars
//!
//! Design Philosophy:
//! - Color-coded borders for quick report recognition
//! - Level badges (★ P3) with frame styles indicating level
//! - Kaomoji faces showing mood and overdue status
//! - Compact cards showing name, mood gauge, and meeting status

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::model::{MoodTrend, ReportSummary, ReportType};
use crate::theme::{
    format_days_ago, mood_color, mood_gauge, mood_trend_icon, overdue_color, overdue_icon, sprites,
    style_muted, style_title, COLOR_SECONDARY,
};

use super::doorway_card::{DoorwayCard, DOORWAY_CARD_HEIGHT};

// ═══════════════════════════════════════════════════════════════════════════════
// REPORT CARD COMPONENT - For dashboard grid
// ═══════════════════════════════════════════════════════════════════════════════

pub struct AvatarCard<'a> {
    summary: &'a ReportSummary,
    is_selected: bool,
}

impl<'a> AvatarCard<'a> {
    pub fn new(summary: &'a ReportSummary, is_selected: bool) -> Self {
        Self {
            summary,
            is_selected,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        // Card border styled with report's color (color-coded for quick recognition)
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
                .title(format!(" ★ {} ★ ", self.summary.level))
                .title_alignment(Alignment::Center)
                .title_style(Style::default().fg(self.summary.color))
        };

        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Vertical layout: face sprite, name, mood, status
        // Sprite area is 4 lines for all (managers have 4-line sprites, ICs have 3 + 1 padding)
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4), // Face sprite (4 lines, bottom-aligned)
                Constraint::Length(1), // Name
                Constraint::Length(1), // Mood gauge
                Constraint::Length(1), // Meeting status
            ])
            .split(inner);

        // Face sprite with level-based frame
        self.render_face(frame, chunks[0]);

        // Name (truncated, color-coded for quick recognition)
        let name: String = self.summary.name.chars().take(14).collect();
        let name_style = if self.is_selected {
            style_title()
        } else {
            Style::default()
                .fg(self.summary.color)
                .add_modifier(Modifier::BOLD)
        };
        let name_para =
            Paragraph::new(Line::from(Span::styled(name, name_style))).alignment(Alignment::Center);
        frame.render_widget(name_para, chunks[1]);

        // Mood gauge with trend
        let mood_display = self
            .summary
            .recent_mood
            .map_or_else(|| "─────".to_string(), mood_gauge);
        let trend_icon = mood_trend_icon(self.summary.mood_trend);
        let mood_style = self
            .summary
            .recent_mood
            .map_or(style_muted(), |m| Style::default().fg(mood_color(m)));
        let trend_style = match self.summary.mood_trend {
            Some(MoodTrend::Rising) => Style::default().fg(Color::Green).bold(),
            Some(MoodTrend::Falling) => Style::default().fg(Color::Rgb(255, 140, 0)).bold(),
            _ => style_muted(),
        };
        let mood_spans = if trend_icon.trim().is_empty() {
            vec![Span::styled(mood_display, mood_style)]
        } else {
            vec![
                Span::styled(trend_icon, trend_style),
                Span::raw(" "),
                Span::styled(mood_display, mood_style),
            ]
        };
        let mood_para = Paragraph::new(Line::from(mood_spans)).alignment(Alignment::Center);
        frame.render_widget(mood_para, chunks[2]);

        // Meeting status
        let status_icon = overdue_icon(self.summary.is_overdue);
        let status_color = overdue_color(self.summary.is_overdue);
        let days_text = format_days_ago(self.summary.days_since_meeting);
        let status_para = Paragraph::new(Line::from(vec![
            Span::styled(status_icon, Style::default().fg(status_color)),
            Span::raw(" "),
            Span::styled(days_text, Style::default().fg(status_color)),
        ]))
        .alignment(Alignment::Center);
        frame.render_widget(status_para, chunks[3]);
    }

    fn render_face(&self, frame: &mut Frame, area: Rect) {
        let face_style = if self.is_selected {
            style_title()
        } else {
            Style::default().fg(self.summary.color)
        };

        let sprite = sprites::FaceSprite::from_summary(self.summary, face_style);
        let mut lines = sprite.lines();

        // Add top padding for ICs (3-line sprites) to align at bottom with managers (4-line)
        if self.summary.report_type != ReportType::Manager {
            lines.insert(0, Line::from(""));
        }

        let para = Paragraph::new(lines).alignment(Alignment::Center);
        frame.render_widget(para, area);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// ENGINEER GRID - Dashboard layout
// ═══════════════════════════════════════════════════════════════════════════════

pub struct AvatarGrid<'a> {
    summaries: &'a [ReportSummary],
    selected: usize,
}

impl<'a> AvatarGrid<'a> {
    pub fn new(summaries: &'a [ReportSummary], selected: usize) -> Self {
        Self {
            summaries,
            selected,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        if self.summaries.is_empty() {
            return;
        }

        let card_width: u16 = 18;
        // Height: border(1) + sprite(3-4) + name(1) + mood(1) + status(1) + border(1)
        // Use 9 to accommodate manager sprites (4 lines with headband)
        let card_height: u16 = 9;

        let cards_per_row = (area.width / card_width).max(1) as usize;

        // Partition the urgency-sorted sequence into rows: each manager gets a
        // full-width doorway row, runs of consecutive ICs chunk into grid rows.
        // Navigation is linear over summary indices, so mixed row shapes keep
        // selection semantics unchanged.
        let mut grid_rows: Vec<GridRow> = Vec::new();
        let mut ic_run: Vec<usize> = Vec::new();
        for (idx, summary) in self.summaries.iter().enumerate() {
            if matches!(summary.report_type, ReportType::Manager) {
                if !ic_run.is_empty() {
                    grid_rows.push(GridRow::Ics(std::mem::take(&mut ic_run)));
                }
                grid_rows.push(GridRow::Doorway(idx));
            } else {
                ic_run.push(idx);
                if ic_run.len() == cards_per_row {
                    grid_rows.push(GridRow::Ics(std::mem::take(&mut ic_run)));
                }
            }
        }
        if !ic_run.is_empty() {
            grid_rows.push(GridRow::Ics(ic_run));
        }

        let row_constraints: Vec<Constraint> = grid_rows
            .iter()
            .map(|row| match row {
                GridRow::Doorway(_) => Constraint::Length(DOORWAY_CARD_HEIGHT),
                GridRow::Ics(_) => Constraint::Length(card_height),
            })
            .collect();

        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints(row_constraints)
            .split(area);

        for (grid_row, row_area) in grid_rows.iter().zip(rows.iter()) {
            match grid_row {
                GridRow::Doorway(idx) => {
                    let card = DoorwayCard::new(&self.summaries[*idx], *idx == self.selected);
                    card.render(frame, *row_area);
                }
                GridRow::Ics(indices) => {
                    let col_constraints: Vec<Constraint> = indices
                        .iter()
                        .map(|_| Constraint::Length(card_width))
                        .collect();

                    let cols = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(col_constraints)
                        .split(*row_area);

                    for (idx, col_area) in indices.iter().zip(cols.iter()) {
                        let card = AvatarCard::new(&self.summaries[*idx], *idx == self.selected);
                        card.render(frame, *col_area);
                    }
                }
            }
        }
    }
}

/// A dashboard grid row: one full-width manager doorway or a run of IC cards
enum GridRow {
    Doorway(usize),
    Ics(Vec<usize>),
}
