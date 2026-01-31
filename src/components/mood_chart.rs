//! Mood history chart component

use chrono::NaiveDateTime;
use ratatui::{
    style::Style,
    text::{Line, Span},
};

use crate::model::JournalEntry;
use crate::theme::{mood_color, style_muted, COLOR_MUTED};

/// Render a simple ASCII chart with axis
pub fn render_mood_chart_with_axis(
    entries: &[JournalEntry],
    width: usize,
    _height: usize,
) -> Vec<Line<'static>> {
    let mood_data: Vec<(NaiveDateTime, u8)> = entries
        .iter()
        .filter_map(|e| e.mood().map(|m| (e.timestamp, m)))
        .collect();

    if mood_data.is_empty() {
        return vec![Line::from(Span::styled("No mood data yet", style_muted()))];
    }

    let display_count = mood_data.len().min(width.saturating_sub(2)); // Leave room for Y axis
    let start = mood_data.len().saturating_sub(display_count);
    let data = &mood_data[start..];

    let mut lines = Vec::new();

    // Chart rows from top (5) to bottom (1)
    for level in (1..=5).rev() {
        let mut row_spans = vec![Span::styled(
            format!("{}│", level),
            Style::default().fg(COLOR_MUTED),
        )];

        for (_, mood) in data.iter() {
            if *mood >= level {
                let ch = if *mood == level { '●' } else { '│' };
                row_spans.push(Span::styled(
                    ch.to_string(),
                    Style::default().fg(mood_color(*mood)),
                ));
            } else {
                row_spans.push(Span::raw(" "));
            }
        }

        lines.push(Line::from(row_spans));
    }

    // X axis
    let x_axis = format!(" └{}", "─".repeat(data.len().min(width)));
    lines.push(Line::from(Span::styled(
        x_axis,
        Style::default().fg(COLOR_MUTED),
    )));

    lines
}
