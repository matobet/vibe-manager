//! Journal entry model for meetings and mood observations
//!
//! A journal entry has a timestamp, optional mood, optional context, and optional notes.
//!
//! ## Entry Types
//!
//! - **Meeting**: Entry with context `Meeting` and non-empty content. These are
//!   formal 1-on-1 meetings that count toward the meeting schedule.
//! - **Mood observation**: Entry with mood but minimal/no content. Quick notes
//!   from standups, Slack, or other interactions.
//!
//! ## File Naming
//!
//! - Full timestamp: `YYYY-MM-DDTHHMMSS.md` (e.g., `2026-01-15T143000.md`)
//! - Legacy date-only: `YYYY-MM-DD.md` (treated as midnight)

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Context for a journal entry - what kind of interaction it was
///
/// Used to distinguish formal meetings from casual observations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Context {
    /// Formal 1-on-1 meeting (counts toward schedule)
    #[default]
    Meeting,
    /// Standup or team sync observation
    Standup,
    /// Written communication (Slack, email)
    Slack,
    /// Other informal interaction
    Other,
}

impl Context {
    pub fn as_str(&self) -> &'static str {
        match self {
            Context::Meeting => "Meeting",
            Context::Standup => "Standup",
            Context::Slack => "Slack",
            Context::Other => "Other",
        }
    }

    pub fn short(&self) -> &'static str {
        match self {
            Context::Meeting => "1:1",
            Context::Standup => "Stnd",
            Context::Slack => "Slck",
            Context::Other => "Othr",
        }
    }

    /// Cycle to the next context variant
    pub fn next(&self) -> Self {
        match self {
            Context::Meeting => Context::Standup,
            Context::Standup => Context::Slack,
            Context::Slack => Context::Other,
            Context::Other => Context::Meeting,
        }
    }

    /// Cycle to the previous context variant
    pub fn prev(&self) -> Self {
        match self {
            Context::Meeting => Context::Other,
            Context::Standup => Context::Meeting,
            Context::Slack => Context::Standup,
            Context::Other => Context::Slack,
        }
    }

    /// All context variants for iteration
    pub fn all() -> &'static [Context] {
        &[
            Context::Meeting,
            Context::Standup,
            Context::Slack,
            Context::Other,
        ]
    }
}

/// YAML frontmatter for a journal entry
///
/// Stored between `---` delimiters at the start of the markdown file.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JournalEntryFrontmatter {
    /// Mood rating from 1 (low) to 5 (high)
    #[serde(default)]
    pub mood: Option<u8>,
    /// Context of the interaction
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<Context>,
}

/// A journal entry (meeting or mood observation)
///
/// Loaded from a markdown file in the report's directory or `journal/` subdirectory.
#[derive(Debug, Clone)]
pub struct JournalEntry {
    /// When this entry was created
    pub timestamp: NaiveDateTime,
    /// Filesystem path to the markdown file
    pub path: PathBuf,
    /// Parsed frontmatter (mood, context)
    pub frontmatter: JournalEntryFrontmatter,
    /// Markdown content (body after frontmatter)
    pub content: String,
}

impl JournalEntry {
    pub fn new(
        timestamp: NaiveDateTime,
        path: PathBuf,
        frontmatter: JournalEntryFrontmatter,
        content: String,
    ) -> Self {
        Self {
            timestamp,
            path,
            frontmatter,
            content,
        }
    }

    /// Get validated mood (1-5 range)
    pub fn mood(&self) -> Option<u8> {
        self.frontmatter.mood.filter(|&m| (1..=5).contains(&m))
    }

    /// Get the context, defaulting to Meeting if not specified but has content
    pub fn context(&self) -> Option<Context> {
        self.frontmatter.context.or_else(|| {
            if !self.content.trim().is_empty() {
                Some(Context::Meeting)
            } else {
                None
            }
        })
    }

    /// Check if this entry represents a "meeting" for scheduling purposes
    /// A meeting is either:
    /// - Explicitly marked as context: meeting
    /// - Has content AND no explicit non-meeting context
    pub fn is_meeting(&self) -> bool {
        match self.frontmatter.context {
            Some(Context::Meeting) => true,
            Some(_) => false, // Explicit non-meeting context (standup, slack, other)
            None => !self.content.trim().is_empty(), // No context + has content = legacy meeting
        }
    }

    /// Get just the date portion
    pub fn date(&self) -> NaiveDate {
        self.timestamp.date()
    }

    /// Check if entry has time component (not midnight)
    pub fn has_time(&self) -> bool {
        self.timestamp.time() != NaiveTime::from_hms_opt(0, 0, 0).unwrap()
    }
}

/// Parse entry timestamp from filename
/// Supports:
/// - YYYY-MM-DDTHHMMSS.md (full timestamp)
/// - YYYY-MM-DD.md (legacy, treated as midnight)
pub fn parse_entry_timestamp(filename: &str) -> Option<NaiveDateTime> {
    let stem = filename.strip_suffix(".md")?;

    // Try full timestamp format first: YYYY-MM-DDTHHMMSS
    if let Ok(dt) = NaiveDateTime::parse_from_str(stem, "%Y-%m-%dT%H%M%S") {
        return Some(dt);
    }

    // Fall back to date-only format: YYYY-MM-DD (treat as midnight)
    if let Ok(date) = NaiveDate::parse_from_str(stem, "%Y-%m-%d") {
        return date.and_hms_opt(0, 0, 0);
    }

    None
}

/// Format timestamp for filename (filesystem-safe ISO 8601)
pub fn format_entry_filename(timestamp: NaiveDateTime) -> String {
    format!("{}.md", timestamp.format("%Y-%m-%dT%H%M%S"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_entry_timestamp_full() {
        let ts = parse_entry_timestamp("2026-01-20T143000.md");
        assert!(ts.is_some());
        let ts = ts.unwrap();
        assert_eq!(ts.date(), NaiveDate::from_ymd_opt(2026, 1, 20).unwrap());
        assert_eq!(ts.time(), NaiveTime::from_hms_opt(14, 30, 0).unwrap());
    }

    #[test]
    fn test_parse_entry_timestamp_legacy() {
        let ts = parse_entry_timestamp("2026-01-15.md");
        assert!(ts.is_some());
        let ts = ts.unwrap();
        assert_eq!(ts.date(), NaiveDate::from_ymd_opt(2026, 1, 15).unwrap());
        assert_eq!(ts.time(), NaiveTime::from_hms_opt(0, 0, 0).unwrap());
    }

    #[test]
    fn test_parse_entry_timestamp_invalid() {
        assert_eq!(parse_entry_timestamp("invalid.md"), None);
        assert_eq!(parse_entry_timestamp("_profile.md"), None);
    }

    #[test]
    fn test_format_entry_filename() {
        let ts = NaiveDate::from_ymd_opt(2026, 1, 20)
            .unwrap()
            .and_hms_opt(14, 30, 0)
            .unwrap();
        assert_eq!(format_entry_filename(ts), "2026-01-20T143000.md");
    }

    #[test]
    fn test_context_cycle() {
        assert_eq!(Context::Meeting.next(), Context::Standup);
        assert_eq!(Context::Standup.next(), Context::Slack);
        assert_eq!(Context::Slack.next(), Context::Other);
        assert_eq!(Context::Other.next(), Context::Meeting);
        assert_eq!(Context::Meeting.prev(), Context::Other);
        assert_eq!(Context::Standup.prev(), Context::Meeting);
    }

    #[test]
    fn test_entry_is_meeting() {
        let ts = NaiveDateTime::default();
        let path = PathBuf::new();

        // Empty content, no explicit context = not a meeting
        let entry = JournalEntry::new(
            ts,
            path.clone(),
            JournalEntryFrontmatter::default(),
            String::new(),
        );
        assert!(!entry.is_meeting());

        // Has content = meeting
        let entry = JournalEntry::new(
            ts,
            path.clone(),
            JournalEntryFrontmatter::default(),
            "Some notes".to_string(),
        );
        assert!(entry.is_meeting());

        // Explicit meeting context, no content = meeting
        let entry = JournalEntry::new(
            ts,
            path.clone(),
            JournalEntryFrontmatter {
                mood: Some(4),
                context: Some(Context::Meeting),
            },
            String::new(),
        );
        assert!(entry.is_meeting());

        // Explicit non-meeting context with content = NOT a meeting (mood observation)
        let entry = JournalEntry::new(
            ts,
            path,
            JournalEntryFrontmatter {
                mood: Some(1),
                context: Some(Context::Standup),
            },
            "Seemed angry".to_string(),
        );
        assert!(!entry.is_meeting());
    }

    #[test]
    fn test_entry_has_time() {
        let date = NaiveDate::from_ymd_opt(2026, 1, 20).unwrap();
        let midnight = date.and_hms_opt(0, 0, 0).unwrap();
        let afternoon = date.and_hms_opt(14, 30, 0).unwrap();
        let path = PathBuf::new();

        let entry = JournalEntry::new(
            midnight,
            path.clone(),
            JournalEntryFrontmatter::default(),
            String::new(),
        );
        assert!(!entry.has_time());

        let entry = JournalEntry::new(
            afternoon,
            path,
            JournalEntryFrontmatter::default(),
            String::new(),
        );
        assert!(entry.has_time());
    }
}
