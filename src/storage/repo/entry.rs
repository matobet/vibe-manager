//! Entry repository
//!
//! High-level interface for journal entry operations.

use std::fs;
use std::path::PathBuf;

use chrono::{Local, NaiveDate, NaiveDateTime};

use crate::model::{
    format_entry_filename, parse_entry_timestamp, Context, JournalEntry, JournalEntryFrontmatter,
};
use crate::storage::{parse_frontmatter, StorageError, StorageResult};

/// Repository for journal entry operations
#[derive(Debug, Clone)]
pub struct EntryRepository {
    report_path: PathBuf,
}

impl EntryRepository {
    /// Create a new entry repository for a report
    pub(crate) fn new(report_path: PathBuf) -> Self {
        Self { report_path }
    }

    /// List all journal entries for the report
    pub fn list(&self) -> StorageResult<Vec<JournalEntry>> {
        let mut entries = Vec::new();

        // Load from root directory (legacy)
        self.load_entries_from_dir(&self.report_path, &mut entries)?;

        // Load from journal/ subdirectory (new structure)
        let journal_dir = self.report_path.join("journal");
        if journal_dir.is_dir() {
            self.load_entries_from_dir(&journal_dir, &mut entries)?;
        }

        // Sort by timestamp (oldest first)
        entries.sort_by_key(|e| e.timestamp);

        Ok(entries)
    }

    /// Load entries from a specific directory
    fn load_entries_from_dir(
        &self,
        dir: &PathBuf,
        entries: &mut Vec<JournalEntry>,
    ) -> StorageResult<()> {
        if !dir.is_dir() {
            return Ok(());
        }

        for dir_entry in fs::read_dir(dir)? {
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
                if let Ok(entry) = self.load_entry(&path, timestamp) {
                    entries.push(entry);
                }
            }
        }

        Ok(())
    }

    /// Load a single entry
    fn load_entry(&self, path: &PathBuf, timestamp: NaiveDateTime) -> StorageResult<JournalEntry> {
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

    /// Create a new 1-on-1 meeting entry
    pub fn create_meeting(&self, date: Option<NaiveDate>) -> StorageResult<JournalEntry> {
        let timestamp = if let Some(d) = date {
            // For explicit dates, check if there's already a legacy file (at root or journal/)
            let legacy_filename = format!("{}.md", d.format("%Y-%m-%d"));
            let legacy_path = self.report_path.join(&legacy_filename);
            let journal_path = self.report_path.join("journal").join(&legacy_filename);
            if legacy_path.exists() || journal_path.exists() {
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

        // Use journal/ subdirectory for new meetings
        let journal_dir = self.report_path.join("journal");
        fs::create_dir_all(&journal_dir)?;

        // For new meetings, use timestamp-based filename
        let filename = if date.is_some() {
            // Legacy format for explicit dates
            format!("{}.md", timestamp.date().format("%Y-%m-%d"))
        } else {
            format_entry_filename(timestamp)
        };
        let path = journal_dir.join(&filename);

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

        self.save(&entry)?;
        Ok(entry)
    }

    /// Create a new mood observation entry
    pub fn create_observation(
        &self,
        mood: Option<u8>,
        context: Option<Context>,
        notes: String,
    ) -> StorageResult<JournalEntry> {
        let timestamp = Local::now().naive_local();
        let filename = format_entry_filename(timestamp);

        // Use journal/ subdirectory for new entries
        let journal_dir = self.report_path.join("journal");
        fs::create_dir_all(&journal_dir)?;
        let path = journal_dir.join(&filename);

        if path.exists() {
            return Err(StorageError::InvalidWorkspace(format!(
                "Entry already exists for {}",
                timestamp
            )));
        }

        let frontmatter = JournalEntryFrontmatter { mood, context };
        let entry = JournalEntry::new(timestamp, path, frontmatter, notes);

        self.save(&entry)?;
        Ok(entry)
    }

    /// Save an entry to disk
    pub fn save(&self, entry: &JournalEntry) -> StorageResult<()> {
        let yaml = serde_yaml::to_string(&entry.frontmatter)?;
        let content = format!("---\n{}---\n\n{}", yaml, entry.content);
        fs::write(&entry.path, content)?;
        Ok(())
    }

    /// Delete an entry from disk
    pub fn delete(&self, entry: &JournalEntry) -> StorageResult<()> {
        if entry.path.exists() {
            fs::remove_file(&entry.path)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_create_and_load_meeting() {
        let temp = TempDir::new().unwrap();
        let date = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();

        let repo = EntryRepository::new(temp.path().to_path_buf());
        let created = repo.create_meeting(Some(date)).unwrap();
        assert_eq!(created.date(), date);
        assert!(created.content.contains("January 15, 2026"));

        let entries = repo.list().unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].date(), date);
    }

    #[test]
    fn test_entry_mood() {
        let temp = TempDir::new().unwrap();
        let date = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();

        let repo = EntryRepository::new(temp.path().to_path_buf());
        let mut entry = repo.create_meeting(Some(date)).unwrap();
        entry.frontmatter.mood = Some(4);
        repo.save(&entry).unwrap();

        let loaded = repo.list().unwrap();
        assert_eq!(loaded[0].mood(), Some(4));
    }

    #[test]
    fn test_create_mood_entry() {
        let temp = TempDir::new().unwrap();

        let repo = EntryRepository::new(temp.path().to_path_buf());
        let entry = repo
            .create_observation(
                Some(4),
                Some(Context::Standup),
                "Quick check-in, seemed energized.".to_string(),
            )
            .unwrap();

        assert_eq!(entry.mood(), Some(4));
        assert_eq!(entry.frontmatter.context, Some(Context::Standup));
        assert!(entry.has_time()); // Should have actual time

        let loaded = repo.list().unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].mood(), Some(4));
        assert_eq!(loaded[0].frontmatter.context, Some(Context::Standup));
    }

    #[test]
    fn test_load_legacy_filename() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("2026-01-20.md");

        fs::write(&path, "---\nmood: 3\n---\n\n# 1-on-1\n\nSome content.").unwrap();

        let repo = EntryRepository::new(temp.path().to_path_buf());
        let entries = repo.list().unwrap();
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

        let repo = EntryRepository::new(temp.path().to_path_buf());
        let entries = repo.list().unwrap();
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

        let repo = EntryRepository::new(temp.path().to_path_buf());
        let entries = repo.list().unwrap();
        assert_eq!(entries.len(), 2);

        // Should be sorted by timestamp
        assert!(!entries[0].has_time()); // Midnight first
        assert!(entries[1].has_time()); // Afternoon second
    }
}
