//! Application state management
//!
//! This module handles loading, initializing, and managing application state.

use std::path::PathBuf;
use std::time::Instant;

use anyhow::Result;

use super::{App, ViewMode, STATUS_MESSAGE_DURATION};
use crate::components::modal::NewReportState;
use crate::model::{
    compute_report_summary, compute_workspace_summary, Context, JournalEntry, WorkspaceSummary,
};
use crate::storage;

impl App {
    /// Create new application from workspace path
    pub fn new(workspace_path: PathBuf) -> Result<Self> {
        let workspace = storage::load_workspace(&workspace_path)?;

        let mut app = App {
            workspace,
            reports: Vec::new(),
            entries_by_report: Vec::new(),
            summaries: Vec::new(),
            workspace_summary: WorkspaceSummary {
                team_size: 0,
                active_count: 0,
                overdue_count: 0,
                average_mood: None,
                total_report_count: 0,
            },
            view_mode: ViewMode::Dashboard,
            selected_index: 0,
            selected_report_index: None,
            selected_entry_index: None,
            editor_content: String::new(),
            editor_mood: None,
            new_report_state: NewReportState::default(),
            pending_entry_mood: None,
            pending_entry_context: Context::Standup,
            pending_entry_notes: String::new(),
            should_quit: false,
            status_message: None,
            delete_from_list: false,
        };

        app.load_data()?;
        Ok(app)
    }

    /// Load all data from workspace
    pub fn load_data(&mut self) -> Result<()> {
        let report_dirs = storage::list_report_dirs(&self.workspace)?;

        self.reports.clear();
        self.entries_by_report.clear();
        self.summaries.clear();

        // Collect all report data
        let mut all_data: Vec<_> = report_dirs
            .into_iter()
            .filter_map(|dir| {
                let mut report = storage::load_report(&dir).ok()?;

                // Load team members for managers
                if storage::has_team_dir(&dir) {
                    let team_dirs = storage::list_team_member_dirs(&dir).unwrap_or_default();
                    for team_dir in team_dirs {
                        if let Ok(team_member) =
                            storage::load_report_with_manager(&team_dir, &report.slug)
                        {
                            report.team.push(team_member);
                        }
                    }
                }

                let entries = storage::load_entries(&dir).unwrap_or_default();
                let summary = compute_report_summary(
                    &report,
                    &entries,
                    self.workspace.config.settings.overdue_threshold_days,
                );
                Some((report, entries, summary))
            })
            .collect();

        // Sort by urgency score (highest first = needs most attention)
        all_data.sort_by(|a, b| b.2.urgency_score.cmp(&a.2.urgency_score));

        // Unpack into separate vectors
        for (report, entries, summary) in all_data {
            self.reports.push(report);
            self.entries_by_report.push(entries);
            self.summaries.push(summary);
        }

        self.workspace_summary = compute_workspace_summary(&self.summaries);

        // Reset selection if out of bounds
        if self.selected_index >= self.reports.len() && !self.reports.is_empty() {
            self.selected_index = self.reports.len() - 1;
        }

        Ok(())
    }

    /// Set a status message with automatic expiry timestamp
    pub fn set_status(&mut self, message: impl Into<String>) {
        self.status_message = Some((message.into(), Instant::now()));
    }

    /// Clear status message if it has expired
    pub fn clear_expired_status(&mut self) {
        if let Some((_, timestamp)) = &self.status_message {
            if timestamp.elapsed() >= STATUS_MESSAGE_DURATION {
                self.status_message = None;
            }
        }
    }

    /// Get current status message text (if not expired)
    pub fn status_text(&self) -> Option<&str> {
        self.status_message.as_ref().and_then(|(msg, timestamp)| {
            if timestamp.elapsed() < STATUS_MESSAGE_DURATION {
                Some(msg.as_str())
            } else {
                None
            }
        })
    }

    /// Delete an entry by report and entry index
    ///
    /// Returns Ok(()) on success, sets status message on error
    pub fn delete_entry(&mut self, report_idx: usize, entry_idx: usize) -> Result<()> {
        let entry = &self.entries_by_report[report_idx][entry_idx];
        let path = entry.path.clone();

        // Delete the file
        std::fs::remove_file(&path)?;

        // Remove from in-memory list
        self.entries_by_report[report_idx].remove(entry_idx);
        self.selected_entry_index = None;

        // Recompute summary for this report
        let report = &self.reports[report_idx];
        let entries = &self.entries_by_report[report_idx];
        self.summaries[report_idx] = compute_report_summary(
            report,
            entries,
            self.workspace.config.settings.overdue_threshold_days,
        );
        self.workspace_summary = compute_workspace_summary(&self.summaries);

        Ok(())
    }

    /// Get entries for currently selected report
    pub fn selected_entries(&self) -> Option<&Vec<JournalEntry>> {
        self.selected_report_index
            .and_then(|i| self.entries_by_report.get(i))
    }

    /// Get only meetings (entries with content) for currently selected report
    pub fn selected_meetings(&self) -> Vec<&JournalEntry> {
        self.selected_entries()
            .map(|entries| entries.iter().filter(|e| e.is_meeting()).collect())
            .unwrap_or_default()
    }

    /// Get the number of meetings for currently selected report
    pub fn selected_meeting_count(&self) -> usize {
        self.selected_entries()
            .map(|entries| entries.iter().filter(|e| e.is_meeting()).count())
            .unwrap_or(0)
    }

    /// Convert a display index (in the meetings list) to the actual entry index
    ///
    /// Display shows meetings in reverse chronological order (newest first)
    pub fn meeting_display_to_entry_index(&self, display_index: usize) -> Option<usize> {
        let entries = self.selected_entries()?;
        let meeting_indices: Vec<usize> = entries
            .iter()
            .enumerate()
            .filter(|(_, e)| e.is_meeting())
            .map(|(i, _)| i)
            .collect();

        // Display is reversed (newest first), so map accordingly
        let reversed_idx = meeting_indices.len().checked_sub(1 + display_index)?;
        meeting_indices.get(reversed_idx).copied()
    }

    /// Get the length of the currently navigable list based on view mode
    pub(crate) fn current_list_len(&self) -> usize {
        match self.view_mode {
            ViewMode::Dashboard => self.reports.len(),
            ViewMode::ReportDetail => self.selected_meeting_count(),
            _ => 0,
        }
    }
}
