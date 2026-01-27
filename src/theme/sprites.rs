//! Kaomoji face sprites for engineer avatars
//!
//! Design Philosophy:
//! - Frame style indicates level (P1-P5)
//! - Face expression indicates mood (1-5)
//! - Floating z's indicate overdue status

use ratatui::{
    style::Style,
    text::{Line, Span},
};

use crate::model::EngineerSummary;

/// Frame top based on level
pub fn frame_top(level: Option<&str>) -> &'static str {
    match level {
        Some("P1") => "╭─────╮",
        Some("P2") => "┌─────┐",
        Some("P3") => "╔═════╗",
        Some("P4") => "╔═ ☆ ═╗",
        Some("P5") => "╔═ ★ ═╗",
        _ => "╭─────╮",
    }
}

/// Frame middle borders (left, right) based on level
pub fn frame_mid(level: Option<&str>) -> (&'static str, &'static str) {
    match level {
        Some("P1") | Some("P2") => ("│", "│"),
        _ => ("║", "║"),
    }
}

/// Frame bottom based on level
pub fn frame_bottom(level: Option<&str>) -> &'static str {
    match level {
        Some("P1") => "╰─────╯",
        Some("P2") => "└─────┘",
        _ => "╚═════╝",
    }
}

/// Face expressions based on mood (accessibility-focused - distinct eye shapes + mouths)
///
/// Each mood has unique eye shape + appropriate mouth:
/// - Mood 5 (thriving):  ^‿^   caret eyes + smile - blissful
/// - Mood 4 (happy):     ◕‿◕   big sparkly eyes + smile - bright
/// - Mood 3 (neutral):   •_•   dot eyes + line - neutral
/// - Mood 2 (worried):   ◦︵◦   circle eyes + frown - sad
/// - Mood 1 (stressed):  x_x   x eyes + line - distressed
pub fn face(mood: Option<u8>, is_overdue: bool) -> &'static str {
    if is_overdue {
        return "-_-"; // sleepy/tired
    }
    match mood {
        Some(5) => "^‿^",  // caret eyes - blissful (eyes closed from smiling)
        Some(4) => "◕‿◕",  // big sparkly eyes - bright and happy
        Some(3) => "•_•",  // dot eyes - neutral
        Some(2) => "◦︵◦", // circle eyes + frown - worried
        Some(1) => "x_x",  // x eyes - stressed
        _ => "•_•",        // default neutral
    }
}

/// Overdue sleep indicators
pub fn sleep_indicator(is_overdue: bool, days_overdue: Option<i64>) -> &'static str {
    if !is_overdue {
        return "";
    }
    match days_overdue {
        Some(d) if d > 14 => "zZ",
        _ => "z",
    }
}

/// Data for rendering a kaomoji face sprite
pub struct FaceSprite<'a> {
    pub level: Option<&'a str>,
    pub mood: Option<u8>,
    pub is_overdue: bool,
    pub days_since_meeting: Option<i64>,
    pub style: Style,
}

impl<'a> FaceSprite<'a> {
    /// Create from an EngineerSummary
    pub fn from_summary(summary: &'a EngineerSummary, style: Style) -> Self {
        Self {
            level: Some(summary.level.as_str()),
            mood: summary.recent_mood,
            is_overdue: summary.is_overdue,
            days_since_meeting: summary.days_since_meeting,
            style,
        }
    }

    /// Generate the 3 lines of the face sprite
    pub fn lines(&self) -> Vec<Line<'static>> {
        let frame_top_str = frame_top(self.level);
        let frame_bottom_str = frame_bottom(self.level);
        let (left, right) = frame_mid(self.level);
        let face_expr = face(self.mood, self.is_overdue);
        let face_line = format!("{} {} {}", left, face_expr, right);

        if !self.is_overdue {
            vec![
                Line::from(Span::styled(frame_top_str.to_string(), self.style)),
                Line::from(Span::styled(face_line, self.style)),
                Line::from(Span::styled(frame_bottom_str.to_string(), self.style)),
            ]
        } else {
            // Add padding on both sides to keep face centered
            let very_overdue = self.days_since_meeting.is_some_and(|d| d > 14);
            let (left_pad, top_z, mid_z, bottom_pad) = if very_overdue {
                ("   ", " zZ", "ZzZ", "   ")
            } else {
                ("  ", " z", " Z", "  ")
            };
            vec![
                Line::from(Span::styled(
                    format!("{}{}{}", left_pad, frame_top_str, top_z),
                    self.style,
                )),
                Line::from(Span::styled(
                    format!("{}{}{}", left_pad, face_line, mid_z),
                    self.style,
                )),
                Line::from(Span::styled(
                    format!("{}{}{}", left_pad, frame_bottom_str, bottom_pad),
                    self.style,
                )),
            ]
        }
    }
}
