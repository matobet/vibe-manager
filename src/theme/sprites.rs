//! Kaomoji face sprites for report avatars
//!
//! Design Philosophy:
//! - Frame style indicates level (P1-P5)
//! - Face expression indicates mood (1-5)
//! - Floating z's indicate overdue status

use ratatui::{
    style::Style,
    text::{Line, Span},
};

use crate::model::ReportSummary;

/// Frame top based on level (IC and Manager tracks)
fn frame_top(level: Option<&str>) -> &'static str {
    match level {
        // IC track
        Some("P1") => "╭─────╮",
        Some("P2") => "┌─────┐",
        Some("P3") => "╔═════╗",
        Some("P4") => "╔══★══╗",
        Some("P5") => "╔═★═★═╗",
        // Manager track (no stars in top border - headband is the status symbol)
        Some("M1") => "╭─────╮",
        Some("M2") => "┌─────┐",
        Some("M3") | Some("M4") | Some("M5") => "╔═════╗",
        _ => "╭─────╮",
    }
}

/// Frame middle borders (left, right) based on level (IC and Manager tracks)
fn frame_mid(level: Option<&str>) -> (&'static str, &'static str) {
    match level {
        Some("P1") | Some("P2") | Some("M1") | Some("M2") => ("│", "│"),
        _ => ("║", "║"),
    }
}

/// Frame bottom based on level (IC and Manager tracks)
fn frame_bottom(level: Option<&str>) -> &'static str {
    match level {
        Some("P1") | Some("M1") => "╰─────╯",
        Some("P2") | Some("M2") => "└─────┘",
        _ => "╚═════╝",
    }
}

/// Headband line for manager avatars (M1-M5)
/// Returns None for ICs, Some(headband) for managers
fn headband_line(level: Option<&str>) -> Option<&'static str> {
    match level {
        Some("M1") => Some("│──◇──│"),
        Some("M2") => Some("│══◆══│"),
        Some("M3") => Some("║══★══║"),
        Some("M4") => Some("║═★═★═║"),
        Some("M5") => Some("║★═★═★║"),
        _ => None,
    }
}

/// Check if a level is a manager (M-track)
fn is_manager(level: Option<&str>) -> bool {
    matches!(
        level,
        Some("M1") | Some("M2") | Some("M3") | Some("M4") | Some("M5")
    )
}

/// Face expressions based on mood (accessibility-focused - distinct eye shapes + mouths)
///
/// Each mood has unique eye shape + appropriate mouth:
/// - Mood 5 (thriving):  ^‿^   caret eyes + smile - blissful
/// - Mood 4 (happy):     ◕‿◕   big sparkly eyes + smile - bright
/// - Mood 3 (neutral):   •_•   dot eyes + line - neutral
/// - Mood 2 (worried):   ◦︵◦   circle eyes + frown - sad
/// - Mood 1 (stressed):  x_x   x eyes + line - distressed
fn face(mood: Option<u8>, is_overdue: bool) -> &'static str {
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

/// Data for rendering a kaomoji face sprite
pub struct FaceSprite<'a> {
    pub level: Option<&'a str>,
    pub mood: Option<u8>,
    pub is_overdue: bool,
    pub days_since_meeting: Option<i64>,
    pub style: Style,
}

impl<'a> FaceSprite<'a> {
    /// Create from an ReportSummary
    pub fn from_summary(summary: &'a ReportSummary, style: Style) -> Self {
        Self {
            level: Some(summary.level.as_str()),
            mood: summary.recent_mood,
            is_overdue: summary.is_overdue,
            days_since_meeting: summary.days_since_meeting,
            style,
        }
    }

    /// Generate the lines of the face sprite
    /// ICs are 3 lines: top frame, face, bottom frame
    /// Managers are 4 lines: top frame, headband, face, bottom frame
    pub fn lines(&self) -> Vec<Line<'static>> {
        let frame_top_str = frame_top(self.level);
        let frame_bottom_str = frame_bottom(self.level);
        let (left, right) = frame_mid(self.level);
        let face_expr = face(self.mood, self.is_overdue);
        // Mood 2 uses fullwidth `︵` character - shift face left to compensate
        let face_line = if self.mood == Some(2) {
            format!("{}{} {}", left, face_expr, right) // no leading space
        } else {
            format!("{} {} {}", left, face_expr, right) // normal spacing
        };
        let headband = headband_line(self.level);

        if !self.is_overdue {
            let mut result = vec![Line::from(Span::styled(
                frame_top_str.to_string(),
                self.style,
            ))];

            // Add headband line for managers
            if let Some(hb) = headband {
                result.push(Line::from(Span::styled(hb.to_string(), self.style)));
            }

            result.push(Line::from(Span::styled(face_line, self.style)));
            result.push(Line::from(Span::styled(
                frame_bottom_str.to_string(),
                self.style,
            )));
            result
        } else {
            // Add padding on both sides to keep face centered
            let very_overdue = self.days_since_meeting.is_some_and(|d| d > 14);
            let (left_pad, top_z, mid_z, headband_z, bottom_pad) = if very_overdue {
                ("   ", " zZ", "ZzZ", " zZ", "   ")
            } else {
                ("  ", " z", " Z", " z", "  ")
            };

            let mut result = vec![Line::from(Span::styled(
                format!("{}{}{}", left_pad, frame_top_str, top_z),
                self.style,
            ))];

            // Add headband line for managers (with Zs)
            if let Some(hb) = headband {
                result.push(Line::from(Span::styled(
                    format!("{}{}{}", left_pad, hb, headband_z),
                    self.style,
                )));
            }

            result.push(Line::from(Span::styled(
                format!("{}{}{}", left_pad, face_line, mid_z),
                self.style,
            )));
            result.push(Line::from(Span::styled(
                format!("{}{}{}", left_pad, frame_bottom_str, bottom_pad),
                self.style,
            )));
            result
        }
    }

    /// Returns the number of lines this sprite will produce
    pub fn line_count(&self) -> u16 {
        if is_manager(self.level) {
            4
        } else {
            3
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sprite_for_level(level: &str) -> FaceSprite<'_> {
        FaceSprite {
            level: Some(level),
            mood: Some(3),
            is_overdue: false,
            days_since_meeting: Some(5),
            style: Style::default(),
        }
    }

    #[test]
    fn test_ic_sprite_line_count() {
        for level in ["P1", "P2", "P3", "P4", "P5"] {
            let sprite = sprite_for_level(level);
            assert_eq!(sprite.lines().len(), 3, "IC {} should have 3 lines", level);
            assert_eq!(sprite.line_count(), 3);
        }
    }

    #[test]
    fn test_manager_sprite_line_count() {
        for level in ["M1", "M2", "M3", "M4", "M5"] {
            let sprite = sprite_for_level(level);
            assert_eq!(
                sprite.lines().len(),
                4,
                "Manager {} should have 4 lines",
                level
            );
            assert_eq!(sprite.line_count(), 4);
        }
    }

    #[test]
    fn test_ic_frame_progression() {
        // P1: rounded
        assert_eq!(frame_top(Some("P1")), "╭─────╮");
        assert_eq!(frame_bottom(Some("P1")), "╰─────╯");

        // P2: square
        assert_eq!(frame_top(Some("P2")), "┌─────┐");
        assert_eq!(frame_bottom(Some("P2")), "└─────┘");

        // P3: double-line
        assert_eq!(frame_top(Some("P3")), "╔═════╗");
        assert_eq!(frame_bottom(Some("P3")), "╚═════╝");

        // P4: double-line with single star
        assert_eq!(frame_top(Some("P4")), "╔══★══╗");

        // P5: double-line with double stars
        assert_eq!(frame_top(Some("P5")), "╔═★═★═╗");
    }

    #[test]
    fn test_manager_frame_progression() {
        // M1: rounded (same as P1)
        assert_eq!(frame_top(Some("M1")), "╭─────╮");
        assert_eq!(frame_bottom(Some("M1")), "╰─────╯");

        // M2: square (same as P2)
        assert_eq!(frame_top(Some("M2")), "┌─────┐");
        assert_eq!(frame_bottom(Some("M2")), "└─────┘");

        // M3-M5: double-line without stars (headband is the status symbol)
        for level in ["M3", "M4", "M5"] {
            assert_eq!(frame_top(Some(level)), "╔═════╗");
            assert_eq!(frame_bottom(Some(level)), "╚═════╝");
        }
    }

    #[test]
    fn test_manager_headband_progression() {
        // M1: hollow diamond
        assert_eq!(headband_line(Some("M1")), Some("│──◇──│"));

        // M2: filled diamond
        assert_eq!(headband_line(Some("M2")), Some("│══◆══│"));

        // M3: single star
        assert_eq!(headband_line(Some("M3")), Some("║══★══║"));

        // M4: double star
        assert_eq!(headband_line(Some("M4")), Some("║═★═★═║"));

        // M5: triple star
        assert_eq!(headband_line(Some("M5")), Some("║★═★═★║"));

        // ICs should have no headband
        for level in ["P1", "P2", "P3", "P4", "P5"] {
            assert_eq!(headband_line(Some(level)), None);
        }
    }

    #[test]
    fn test_is_manager_detection() {
        for level in ["M1", "M2", "M3", "M4", "M5"] {
            assert!(
                is_manager(Some(level)),
                "{} should be detected as manager",
                level
            );
        }

        for level in ["P1", "P2", "P3", "P4", "P5"] {
            assert!(
                !is_manager(Some(level)),
                "{} should not be detected as manager",
                level
            );
        }

        assert!(!is_manager(None));
    }
}
