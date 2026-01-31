use std::fs;
use std::path::Path;

use chrono::{Local, NaiveDate, NaiveDateTime};

use super::{parse_frontmatter, StorageError, StorageResult};
use crate::model::{
    format_entry_filename, parse_entry_timestamp, Context, JournalEntry, JournalEntryFrontmatter,
};

/// Load all entries for an engineer
pub fn load_entries(engineer_dir: &Path) -> StorageResult<Vec<JournalEntry>> {
    let mut entries = Vec::new();

    for dir_entry in fs::read_dir(engineer_dir)? {
        let path = dir_entry?.path();

        if !path.is_file() {
            continue;
        }

        let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        // Skip non-entry files
        if filename.starts_with('_') || !filename.ends_with(".md") {
            continue;
        }

        // Try to parse as entry timestamp
        if let Some(timestamp) = parse_entry_timestamp(filename) {
            if let Ok(entry) = load_entry(&path, timestamp) {
                entries.push(entry);
            }
        }
    }

    // Sort by timestamp (oldest first)
    entries.sort_by_key(|e| e.timestamp);

    Ok(entries)
}

/// Load a single entry
pub fn load_entry(path: &Path, timestamp: NaiveDateTime) -> StorageResult<JournalEntry> {
    let content = fs::read_to_string(path)?;
    let (frontmatter, body) = parse_frontmatter(&content);

    let fm: JournalEntryFrontmatter = match frontmatter {
        Some(yaml) if !yaml.is_empty() => serde_yaml::from_str(yaml)?,
        _ => JournalEntryFrontmatter::default(),
    };

    Ok(JournalEntry::new(
        timestamp,
        path.to_path_buf(),
        fm,
        body.to_string(),
    ))
}

/// Save entry to disk
pub fn save_entry(entry: &JournalEntry) -> StorageResult<()> {
    let yaml = serde_yaml::to_string(&entry.frontmatter)?;
    let content = format!("---\n{}---\n\n{}", yaml, entry.content);
    fs::write(&entry.path, content)?;
    Ok(())
}

/// Create a new entry (mood observation or meeting)
pub fn create_entry(
    engineer_dir: &Path,
    mood: Option<u8>,
    context: Option<Context>,
    notes: String,
) -> StorageResult<JournalEntry> {
    let timestamp = Local::now().naive_local();
    let filename = format_entry_filename(timestamp);
    let path = engineer_dir.join(&filename);

    if path.exists() {
        return Err(StorageError::InvalidWorkspace(format!(
            "Entry already exists for {}",
            timestamp
        )));
    }

    let frontmatter = JournalEntryFrontmatter { mood, context };
    let entry = JournalEntry::new(timestamp, path, frontmatter, notes);

    save_entry(&entry)?;
    Ok(entry)
}

/// Create a new 1-on-1 meeting with template content
pub fn create_meeting(engineer_dir: &Path, date: Option<NaiveDate>) -> StorageResult<JournalEntry> {
    let timestamp = if let Some(d) = date {
        // For explicit dates, check if there's already a legacy file
        let legacy_filename = format!("{}.md", d.format("%Y-%m-%d"));
        let legacy_path = engineer_dir.join(&legacy_filename);
        if legacy_path.exists() {
            return Err(StorageError::InvalidWorkspace(format!(
                "Meeting already exists for {}",
                d
            )));
        }
        // Use midnight for explicit dates (backwards compat for tests)
        d.and_hms_opt(0, 0, 0).unwrap()
    } else {
        Local::now().naive_local()
    };

    // For new meetings, use timestamp-based filename
    let filename = if date.is_some() {
        // Legacy format for explicit dates
        format!("{}.md", timestamp.date().format("%Y-%m-%d"))
    } else {
        format_entry_filename(timestamp)
    };
    let path = engineer_dir.join(&filename);

    if path.exists() {
        return Err(StorageError::InvalidWorkspace(format!(
            "Meeting already exists for {}",
            timestamp
        )));
    }

    let content = format!(
        "# 1-on-1 - {}\n\n\
         ## Discussion\n\n\
         ## Notes\n\n\
         ## Action Items\n- [ ] \n",
        timestamp.date().format("%B %d, %Y")
    );

    let frontmatter = JournalEntryFrontmatter {
        mood: None,
        context: Some(Context::Meeting),
    };
    let entry = JournalEntry::new(timestamp, path, frontmatter, content);

    save_entry(&entry)?;
    Ok(entry)
}

/// Update entry mood
pub fn update_entry_mood(entry: &mut JournalEntry, mood: u8) -> StorageResult<()> {
    if !(1..=5).contains(&mood) {
        return Err(StorageError::InvalidWorkspace(
            "Mood must be between 1 and 5".to_string(),
        ));
    }
    entry.frontmatter.mood = Some(mood);
    save_entry(entry)
}

// Backwards compatibility aliases
pub fn load_meetings(engineer_dir: &Path) -> StorageResult<Vec<JournalEntry>> {
    load_entries(engineer_dir)
}

pub fn save_meeting(entry: &JournalEntry) -> StorageResult<()> {
    save_entry(entry)
}

pub fn update_meeting_mood(entry: &mut JournalEntry, mood: u8) -> StorageResult<()> {
    update_entry_mood(entry, mood)
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
        assert_eq!(created.date(), date);
        assert!(created.content.contains("January 15, 2026"));

        let entries = load_entries(temp.path()).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].date(), date);
    }

    #[test]
    fn test_entry_mood() {
        let temp = TempDir::new().unwrap();
        let date = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();

        let mut entry = create_meeting(temp.path(), Some(date)).unwrap();
        update_entry_mood(&mut entry, 4).unwrap();

        let loaded = load_entries(temp.path()).unwrap();
        assert_eq!(loaded[0].mood(), Some(4));
    }

    #[test]
    fn test_create_mood_entry() {
        let temp = TempDir::new().unwrap();

        let entry = create_entry(
            temp.path(),
            Some(4),
            Some(Context::Standup),
            "Quick check-in, seemed energized.".to_string(),
        )
        .unwrap();

        assert_eq!(entry.mood(), Some(4));
        assert_eq!(entry.frontmatter.context, Some(Context::Standup));
        assert!(entry.has_time()); // Should have actual time

        let loaded = load_entries(temp.path()).unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].mood(), Some(4));
        assert_eq!(loaded[0].frontmatter.context, Some(Context::Standup));
    }

    #[test]
    fn test_load_legacy_filename() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("2026-01-20.md");

        fs::write(&path, "---\nmood: 3\n---\n\n# 1-on-1\n\nSome content.").unwrap();

        let entries = load_entries(temp.path()).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(
            entries[0].date(),
            NaiveDate::from_ymd_opt(2026, 1, 20).unwrap()
        );
        assert!(!entries[0].has_time()); // Legacy files have no time
        assert_eq!(entries[0].mood(), Some(3));
    }

    #[test]
    fn test_load_timestamp_filename() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("2026-01-20T143000.md");

        fs::write(
            &path,
            "---\nmood: 4\ncontext: standup\n---\n\nQuick observation.",
        )
        .unwrap();

        let entries = load_entries(temp.path()).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(
            entries[0].date(),
            NaiveDate::from_ymd_opt(2026, 1, 20).unwrap()
        );
        assert!(entries[0].has_time());
        assert_eq!(entries[0].mood(), Some(4));
        assert_eq!(entries[0].frontmatter.context, Some(Context::Standup));
    }

    #[test]
    fn test_multiple_entries_same_day() {
        let temp = TempDir::new().unwrap();

        // Create legacy file (midnight)
        fs::write(
            temp.path().join("2026-01-20.md"),
            "---\nmood: 3\n---\n\n# Morning 1-on-1",
        )
        .unwrap();

        // Create timestamp file (afternoon)
        fs::write(
            temp.path().join("2026-01-20T143000.md"),
            "---\nmood: 4\ncontext: other\n---\n\nAfternoon chat.",
        )
        .unwrap();

        let entries = load_entries(temp.path()).unwrap();
        assert_eq!(entries.len(), 2);

        // Should be sorted by timestamp
        assert!(!entries[0].has_time()); // Midnight first
        assert!(entries[1].has_time()); // Afternoon second
    }
}
