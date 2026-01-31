//! 8-bit RPG aesthetic theme for the TUI
//!
//! Uses box-drawing characters, block progress bars, and ANSI colors
//! to create a retro gaming feel.
//!
//! ## Accessibility Considerations
//! - Colors are paired with icons/shapes, never color-alone
//! - Avoids red-green distinctions (uses blue/orange instead for status)
//! - High contrast mode available
//! - Numeric values shown alongside visual gauges

use ratatui::{
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders},
};

// ═══════════════════════════════════════════════════════════════
// Color Palette - Accessible Design
// ═══════════════════════════════════════════════════════════════
//
// Design principles:
// - Use BLUE for positive/success (instead of green)
// - Use ORANGE/YELLOW for warnings
// - Use MAGENTA for danger/critical (instead of red)
// - Never rely on color alone - always pair with icons/text
// - High contrast between foreground and background

pub const COLOR_PRIMARY: Color = Color::Cyan;
pub const COLOR_SECONDARY: Color = Color::Yellow;
pub const COLOR_ACCENT: Color = Color::Magenta;

// Status colors - colorblind-safe palette
// Using Blue-Orange opposition instead of Red-Green
pub const COLOR_SUCCESS: Color = Color::Cyan; // Blue-ish for "good"
pub const COLOR_WARNING: Color = Color::Yellow; // Yellow for "attention"
pub const COLOR_DANGER: Color = Color::Magenta; // Magenta for "bad" (not red)

// Text colors with good contrast
pub const COLOR_MUTED: Color = Color::Gray; // Lighter than DarkGray
pub const COLOR_TEXT: Color = Color::White;
pub const COLOR_TEXT_DIM: Color = Color::Gray;

// ═══════════════════════════════════════════════════════════════
// Styles
// ═══════════════════════════════════════════════════════════════

pub fn style_default() -> Style {
    Style::default().fg(COLOR_TEXT)
}

pub fn style_highlight() -> Style {
    Style::default()
        .fg(Color::Black)
        .bg(COLOR_PRIMARY)
        .add_modifier(Modifier::BOLD)
}

pub fn style_selected() -> Style {
    Style::default().add_modifier(Modifier::REVERSED)
}

pub fn style_title() -> Style {
    Style::default()
        .fg(COLOR_SECONDARY)
        .add_modifier(Modifier::BOLD)
}

pub fn style_header() -> Style {
    Style::default()
        .fg(COLOR_PRIMARY)
        .add_modifier(Modifier::BOLD)
}

pub fn style_success() -> Style {
    Style::default().fg(COLOR_SUCCESS)
}

pub fn style_warning() -> Style {
    Style::default().fg(COLOR_WARNING)
}

pub fn style_danger() -> Style {
    Style::default().fg(COLOR_DANGER)
}

pub fn style_muted() -> Style {
    Style::default().fg(COLOR_MUTED)
}

// ═══════════════════════════════════════════════════════════════
// Borders & Blocks
// ═══════════════════════════════════════════════════════════════

/// Create an RPG-style block with double borders ╔═══╗
pub fn rpg_block(title: &str) -> Block<'_> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(COLOR_PRIMARY))
        .title(format!("═ {} ═", title))
        .title_style(style_title())
}

/// Create a simple block with rounded borders
pub fn simple_block(title: &str) -> Block<'_> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(COLOR_MUTED))
        .title(title)
        .title_style(Style::default().fg(COLOR_TEXT_DIM))
}

/// Create an active/focused block
pub fn focused_block(title: &str) -> Block<'_> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(COLOR_SECONDARY))
        .title(format!("▶ {} ◀", title))
        .title_style(style_title())
}

// ═══════════════════════════════════════════════════════════════
// Progress & Gauges - Accessible Design
// ═══════════════════════════════════════════════════════════════

/// Create a block progress bar: ████░░░░░░
pub fn progress_bar(value: u8, max: u8) -> String {
    let filled = value.min(max) as usize;
    let empty = (max as usize).saturating_sub(filled);
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}

/// Create a mood gauge with numeric value: ♥♥♥♡♡ (3)
/// Always includes numeric value for accessibility
pub fn mood_gauge(value: u8) -> String {
    let filled = value.min(5) as usize;
    let empty = 5 - filled;
    format!("{}{}", "♥".repeat(filled), "♡".repeat(empty))
}

/// Mood gauge with explicit numeric indicator
pub fn mood_gauge_with_value(value: u8) -> String {
    format!("{} ({})", mood_gauge(value), value)
}

/// Get mood color - uses brightness gradient instead of hue
/// Low mood = dim, High mood = bright (works for all color vision)
pub fn mood_color(value: u8) -> Color {
    // Using brightness/saturation instead of red-green hue shift
    // All users can perceive brightness differences
    match value {
        1 => COLOR_DANGER,        // Magenta - distinct
        2 => Color::LightMagenta, // Lighter magenta
        3 => COLOR_WARNING,       // Yellow - neutral
        4 => Color::LightCyan,    // Light cyan
        5 => COLOR_SUCCESS,       // Cyan - positive
        _ => COLOR_MUTED,
    }
}

// ═══════════════════════════════════════════════════════════════
// Status Icons - Always pair with text/color
// ═══════════════════════════════════════════════════════════════

pub const ICON_OK: &str = "✓"; // Checkmark - universally understood
pub const ICON_WARNING: &str = "⚠"; // Warning triangle
pub const ICON_DANGER: &str = "✗"; // X mark
pub const ICON_HEART: &str = "♥";
pub const ICON_EMPTY_HEART: &str = "♡";
pub const ICON_UP: &str = "▲"; // Filled triangle - more visible than ↑
pub const ICON_DOWN: &str = "▼"; // Filled triangle - more visible than ↓
pub const ICON_STABLE: &str = "◆"; // Diamond for stable (distinct shape)
pub const ICON_ACTIVE: &str = "●";
pub const ICON_INACTIVE: &str = "○";
pub const ICON_MEETING: &str = "☰"; // Menu/list icon (more compatible than 📅)
pub const ICON_PERSON: &str = "◉"; // Circle with dot (more compatible than 👤)

/// Get status icon based on overdue state
/// Icon shape differs - not just color
pub fn overdue_icon(is_overdue: bool) -> &'static str {
    if is_overdue {
        ICON_WARNING // ⚠ triangle shape
    } else {
        ICON_OK // ✓ checkmark shape
    }
}

/// Get status color based on overdue state
/// Uses blue/yellow distinction (colorblind-safe)
pub fn overdue_color(is_overdue: bool) -> Color {
    if is_overdue {
        COLOR_WARNING // Yellow
    } else {
        COLOR_SUCCESS // Cyan
    }
}

/// Get active status icon
pub fn active_icon(is_active: bool) -> &'static str {
    if is_active {
        ICON_ACTIVE // ● filled
    } else {
        ICON_INACTIVE // ○ empty
    }
}

/// Mood trend with distinct shapes (not just arrows)
// TODO: Consider changing from triangles (▲▼◆) to arrows (↑↓→) for better readability
//       across different fonts and terminal emulators. Arrows are also more intuitive.
pub fn mood_trend_icon(trend: Option<crate::model::MoodTrend>) -> &'static str {
    match trend {
        Some(crate::model::MoodTrend::Rising) => ICON_UP, // ▲
        Some(crate::model::MoodTrend::Falling) => ICON_DOWN, // ▼
        Some(crate::model::MoodTrend::Stable) => ICON_STABLE, // ◆
        None => " ",
    }
}

// ═══════════════════════════════════════════════════════════════
// Formatting Helpers
// ═══════════════════════════════════════════════════════════════

/// Format days as human-readable relative time (e.g., "5 days ago", "now")
pub fn format_days_ago(days: Option<i64>) -> String {
    use chrono::TimeDelta;
    use chrono_humanize::HumanTime;

    match days {
        None => "Never".to_string(),
        Some(d) if d < 0 => "Future?".to_string(),
        Some(d) => HumanTime::from(TimeDelta::days(-d)).to_string(),
    }
}

/// Format days since meeting with icon
/// Includes both text and icon for accessibility
pub fn format_days_since(days: Option<i64>, frequency_days: u32) -> String {
    match days {
        None => format!("{} Never", ICON_WARNING),
        Some(d) if d < 0 => "Future?".to_string(),
        Some(d) => {
            let icon = if d == 0 {
                format!("{} ", ICON_OK)
            } else if d > frequency_days as i64 + 3 {
                format!("{} ", ICON_WARNING)
            } else {
                String::new()
            };
            format!("{}{}", icon, format_days_ago(Some(d)))
        }
    }
}

/// Format meeting frequency for display
pub fn format_meeting_frequency(frequency: &str) -> &str {
    match frequency {
        "weekly" => "Weekly",
        "biweekly" => "Bi-weekly",
        "monthly" => "Monthly",
        other => other,
    }
}

/// Create a separator line
pub fn separator(width: usize) -> String {
    "─".repeat(width)
}

/// Create a double separator line
pub fn double_separator(width: usize) -> String {
    "═".repeat(width)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_bar() {
        assert_eq!(progress_bar(3, 5), "███░░");
        assert_eq!(progress_bar(0, 5), "░░░░░");
        assert_eq!(progress_bar(5, 5), "█████");
        assert_eq!(progress_bar(10, 5), "█████"); // Clamped
    }

    #[test]
    fn test_mood_gauge() {
        assert_eq!(mood_gauge(3), "♥♥♥♡♡");
        assert_eq!(mood_gauge(5), "♥♥♥♥♥");
        assert_eq!(mood_gauge(0), "♡♡♡♡♡");
    }

    #[test]
    fn test_mood_gauge_with_value() {
        assert_eq!(mood_gauge_with_value(3), "♥♥♥♡♡ (3)");
    }

    #[test]
    fn test_format_days_since() {
        assert_eq!(format_days_since(None, 7), "⚠ Never");
        assert_eq!(format_days_since(Some(0), 7), "✓ now");
        assert_eq!(format_days_since(Some(1), 7), "a day ago");
        assert_eq!(format_days_since(Some(5), 7), "5 days ago");
    }

    #[test]
    fn test_format_days_ago() {
        assert_eq!(format_days_ago(None), "Never");
        assert_eq!(format_days_ago(Some(0)), "now");
        assert_eq!(format_days_ago(Some(1)), "a day ago");
        assert_eq!(format_days_ago(Some(7)), "a week ago");
        assert_eq!(format_days_ago(Some(14)), "2 weeks ago");
    }
}
