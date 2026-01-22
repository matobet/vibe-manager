use std::fs;
use std::path::Path;

use chrono::{Local, NaiveDate};

use super::{parse_frontmatter, StorageError, StorageResult};
use crate::model::{parse_meeting_date, Meeting, MeetingFrontmatter};

/// Load all meetings for an engineer
pub fn load_meetings(engineer_dir: &Path) -> StorageResult<Vec<Meeting>> {
    let mut meetings = Vec::new();

    for entry in fs::read_dir(engineer_dir)? {
        let path = entry?.path();

        if !path.is_file() {
            continue;
        }

        let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        // Skip non-meeting files
        if filename.starts_with('_') || !filename.ends_with(".md") {
            continue;
        }

        // Try to parse as meeting date
        if let Some(date) = parse_meeting_date(filename) {
            if let Ok(meeting) = load_meeting(&path, date) {
                meetings.push(meeting);
            }
        }
    }

    // Sort by date (oldest first)
    meetings.sort_by_key(|m| m.date);

    Ok(meetings)
}

/// Load a single meeting
pub fn load_meeting(path: &Path, date: NaiveDate) -> StorageResult<Meeting> {
    let content = fs::read_to_string(path)?;
    let (frontmatter, body) = parse_frontmatter(&content);

    let fm: MeetingFrontmatter = match frontmatter {
        Some(yaml) if !yaml.is_empty() => serde_yaml::from_str(yaml)?,
        _ => MeetingFrontmatter::default(),
    };

    Ok(Meeting::new(date, path.to_path_buf(), fm, body.to_string()))
}

/// Save meeting
pub fn save_meeting(meeting: &Meeting) -> StorageResult<()> {
    let yaml = serde_yaml::to_string(&meeting.frontmatter)?;
    let content = format!("---\n{}---\n\n{}", yaml, meeting.content);
    fs::write(&meeting.path, content)?;
    Ok(())
}

/// Create new meeting for today
pub fn create_meeting(engineer_dir: &Path, date: Option<NaiveDate>) -> StorageResult<Meeting> {
    let date = date.unwrap_or_else(|| Local::now().date_naive());
    let filename = format!("{}.md", date.format("%Y-%m-%d"));
    let path = engineer_dir.join(&filename);

    if path.exists() {
        return Err(StorageError::InvalidWorkspace(format!(
            "Meeting already exists for {}",
            date
        )));
    }

    let content = format!(
        "# 1-on-1 - {}\n\n\
         ## Discussion\n\n\
         ## Notes\n\n\
         ## Action Items\n- [ ] \n",
        date.format("%B %d, %Y")
    );

    let meeting = Meeting::new(date, path, MeetingFrontmatter::default(), content);

    save_meeting(&meeting)?;
    Ok(meeting)
}

/// Update meeting mood
pub fn update_meeting_mood(meeting: &mut Meeting, mood: u8) -> StorageResult<()> {
    if !(1..=5).contains(&mood) {
        return Err(StorageError::InvalidWorkspace(
            "Mood must be between 1 and 5".to_string(),
        ));
    }
    meeting.frontmatter.mood = Some(mood);
    save_meeting(meeting)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_create_and_load_meeting() {
        let temp = TempDir::new().unwrap();
        let date = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();

        let created = create_meeting(temp.path(), Some(date)).unwrap();
        assert_eq!(created.date, date);
        assert!(created.content.contains("January 15, 2026"));

        let meetings = load_meetings(temp.path()).unwrap();
        assert_eq!(meetings.len(), 1);
        assert_eq!(meetings[0].date, date);
    }

    #[test]
    fn test_meeting_mood() {
        let temp = TempDir::new().unwrap();
        let date = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();

        let mut meeting = create_meeting(temp.path(), Some(date)).unwrap();
        update_meeting_mood(&mut meeting, 4).unwrap();

        let loaded = load_meetings(temp.path()).unwrap();
        assert_eq!(loaded[0].mood(), Some(4));
    }
}
