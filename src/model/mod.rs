//! Data models for Vibe Manager
//!
//! This module contains the core data structures for reports, meetings,
//! workspaces, and computed summaries.

pub mod computed;
pub mod meeting;
pub mod report;
pub mod workspace;

// Re-export types
pub use computed::{
    compute_extended_workspace_summary, compute_report_summary, compute_team_metrics,
    compute_workspace_summary, MoodTrend, ReportSummary, TeamMetrics, WorkspaceSummary,
};
pub use meeting::{
    format_entry_filename, parse_entry_timestamp, Context, JournalEntry, JournalEntryFrontmatter,
};
pub use report::{Level, ManagerInfo, MeetingFrequency, Report, ReportProfile, ReportType, Skills};
pub use workspace::{Workspace, WorkspaceConfig, WorkspaceSettings};
