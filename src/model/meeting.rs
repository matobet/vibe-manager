use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MeetingFrontmatter {
    #[serde(default)]
    pub mood: Option<u8>,
}

#[derive(Debug, Clone)]
pub struct Meeting {
    pub date: NaiveDate,
    pub path: PathBuf,
    pub frontmatter: MeetingFrontmatter,
    pub content: String,
}

impl Meeting {
    pub fn new(date: NaiveDate, path: PathBuf, frontmatter: MeetingFrontmatter, content: String) -> Self {
        Self {
            date,
            path,
            frontmatter,
            content,
        }
    }

    pub fn mood(&self) -> Option<u8> {
        self.frontmatter.mood.filter(|&m| m >= 1 && m <= 5)
    }
}

pub fn parse_meeting_date(filename: &str) -> Option<NaiveDate> {
    let stem = filename.strip_suffix(".md")?;
    NaiveDate::parse_from_str(stem, "%Y-%m-%d").ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_meeting_date() {
        assert_eq!(
            parse_meeting_date("2026-01-15.md"),
            Some(NaiveDate::from_ymd_opt(2026, 1, 15).unwrap())
        );
        assert_eq!(parse_meeting_date("invalid.md"), None);
        assert_eq!(parse_meeting_date("_profile.md"), None);
    }
}
