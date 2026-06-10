//! Doorway card component - full-width manager cards on the dashboard
//!
//! Design Philosophy:
//! - 4 content lines: identity / your relationship / their squad / the door
//! - The sprite frame is the visual anchor — no outer block or border
//! - Card height never changes: selection only fills in the door-hint line
//! - The squad line names the worst outlier, never just a score

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::model::{MoodTrend, OutlierInfo, ReportSummary};
use crate::theme::{
    format_compact_age, health_bar, mood_color, mood_gauge, overdue_color, overdue_icon,
    sprites::FaceSprite, style_danger, style_header, style_muted, style_success, style_title,
    COLOR_TEXT, ICON_WARNING,
};
use crate::utils::abbreviate_name;

/// Doorway card height: 4 content rows + 1 blank spacer row (invariant)
pub const DOORWAY_CARD_HEIGHT: u16 = 5;

/// Sprite column: 7-cell sprite + 2-cell gutter
const SPRITE_COL_WIDTH: u16 = 9;

/// Right-aligned level badge column on the identity line
const BADGE_COL_WIDTH: u16 = 4;

pub struct DoorwayCard<'a> {
    summary: &'a ReportSummary,
    is_selected: bool,
}

impl<'a> DoorwayCard<'a> {
    pub fn new(summary: &'a ReportSummary, is_selected: bool) -> Self {
        Self {
            summary,
            is_selected,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(SPRITE_COL_WIDTH), Constraint::Min(0)])
            .split(area);

        self.render_sprite(frame, cols[0]);

        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // identity: name — title, level badge
                Constraint::Length(1), // relationship: hearts, your 1-on-1 recency
                Constraint::Length(1), // squad: size, health bar, worst outlier
                Constraint::Length(1), // door hint (selected only)
                Constraint::Length(1), // spacer
            ])
            .split(cols[1]);

        self.render_identity(frame, rows[0]);
        self.render_relationship(frame, rows[1]);
        self.render_squad(frame, rows[2]);
        self.render_hint(frame, rows[3]);
    }

    fn render_sprite(&self, frame: &mut Frame, area: Rect) {
        let face_style = if self.is_selected {
            style_title()
        } else {
            Style::default().fg(self.summary.color)
        };
        // Always the non-overdue render path: the floating-z overdue variant is
        // up to 13 cells wide and would clip in the 9-cell column. Overdue is
        // shown as `zZ` text on the relationship line instead.
        let sprite = FaceSprite {
            level: Some(self.summary.level.as_str()),
            mood: self.summary.recent_mood,
            is_overdue: false,
            days_since_meeting: self.summary.days_since_meeting,
            style: face_style,
        };
        let para = Paragraph::new(sprite.lines()).alignment(Alignment::Left);
        frame.render_widget(para, area);
    }

    fn render_identity(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(0), Constraint::Length(BADGE_COL_WIDTH)])
            .split(area);

        let name: String = self
            .summary
            .name
            .chars()
            .take(24)
            .collect::<String>()
            .to_uppercase();
        let name_style = if self.is_selected {
            style_title()
        } else {
            Style::default()
                .fg(self.summary.color)
                .add_modifier(Modifier::BOLD)
        };

        let mut spans = vec![Span::styled(name, name_style)];
        if let Some(title) = self.summary.title.as_deref().filter(|t| !t.is_empty()) {
            spans.push(Span::styled(format!(" — {}", title), style_muted()));
        }
        frame.render_widget(Paragraph::new(Line::from(spans)), chunks[0]);

        let badge = Paragraph::new(Line::from(Span::styled(
            self.summary.level.clone(),
            style_title(),
        )))
        .alignment(Alignment::Right);
        frame.render_widget(badge, chunks[1]);
    }

    fn render_relationship(&self, frame: &mut Frame, area: Rect) {
        let mood_display = self
            .summary
            .recent_mood
            .map_or_else(|| "─────".to_string(), mood_gauge);
        let mood_style = self
            .summary
            .recent_mood
            .map_or(style_muted(), |m| Style::default().fg(mood_color(m)));

        let status_color = overdue_color(self.summary.is_overdue);
        let mut spans = vec![
            Span::styled(mood_display, mood_style),
            Span::raw("  "),
            Span::styled(
                format!(
                    "{} you: {}",
                    overdue_icon(self.summary.is_overdue),
                    format_compact_age(self.summary.days_since_meeting)
                ),
                Style::default().fg(status_color),
            ),
        ];
        if self.summary.is_overdue {
            spans.push(Span::styled(
                "  zZ",
                style_danger().add_modifier(Modifier::BOLD),
            ));
        }
        frame.render_widget(Paragraph::new(Line::from(spans)), area);
    }

    fn render_squad(&self, frame: &mut Frame, area: Rect) {
        let team_metrics = self
            .summary
            .team_metrics
            .as_ref()
            .filter(|tm| tm.team_size > 0);

        let Some(tm) = team_metrics else {
            // No team (or empty team): muted copy, never a 0% health bar
            let para = Paragraph::new(Line::from(Span::styled(
                "squad 0 · no members yet",
                style_muted(),
            )));
            frame.render_widget(para, area);
            return;
        };

        let mut spans = vec![
            Span::styled(format!("squad {} ", tm.team_size), style_muted()),
            Span::styled(health_bar(tm.team_health_score, 8), style_header()),
            Span::styled(
                format!("{}%", tm.team_health_score),
                Style::default().fg(COLOR_TEXT),
            ),
            Span::raw("  "),
        ];

        match tm.outliers.first() {
            Some(worst) => {
                spans.push(Span::styled(
                    format!(
                        "{} {}: {} · {}",
                        ICON_WARNING,
                        abbreviate_name(&worst.name),
                        outlier_label(worst),
                        format_compact_age(worst.days_since_meeting)
                    ),
                    style_danger().add_modifier(Modifier::BOLD),
                ));
                if tm.outliers.len() > 1 {
                    spans.push(Span::styled(
                        format!(" (+{} more)", tm.outliers.len() - 1),
                        style_muted(),
                    ));
                }
            }
            None => {
                let text = match tm.next_in_rotation.as_deref() {
                    Some(next) => format!("★ all well · next: {}", abbreviate_name(next)),
                    None => "★ all well".to_string(),
                };
                spans.push(Span::styled(text, style_success()));
            }
        }
        frame.render_widget(Paragraph::new(Line::from(spans)), area);
    }

    fn render_hint(&self, frame: &mut Frame, area: Rect) {
        // Always emit the line so card height never changes (DOOR-03)
        let line = if self.is_selected {
            Line::from(Span::styled("▸ Space to visit squad", style_muted()))
        } else {
            Line::from("")
        };
        frame.render_widget(Paragraph::new(line), area);
    }
}

/// Label for the named outlier, by severity of signal:
/// falling trend > low mood value > overdue
fn outlier_label(outlier: &OutlierInfo) -> String {
    if outlier.mood_trend == Some(MoodTrend::Falling) {
        "mood ↘".to_string()
    } else if let Some(mood) = outlier.recent_mood.filter(|m| *m <= 2) {
        format!("mood {}", mood)
    } else {
        "overdue".to_string()
    }
}
