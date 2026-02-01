//! Workspace configuration model
//!
//! A workspace is a directory containing team data, marked by a `.vibe-manager` config file.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Workspace configuration from `.vibe-manager` file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    /// Config file format version
    pub version: u32,
    /// Workspace settings
    #[serde(default)]
    pub settings: WorkspaceSettings,
}

/// Configurable workspace settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceSettings {
    /// Default meeting frequency for new reports
    #[serde(default = "default_meeting_frequency", alias = "default_cadence")]
    pub default_meeting_frequency: String,
    /// Days past due before marking as overdue
    #[serde(default = "default_overdue_threshold")]
    pub overdue_threshold_days: u32,
    /// Default frequency for skip-level (2nd-level) meetings
    #[serde(default = "default_2nd_level_frequency")]
    pub default_2nd_level_frequency: String,
}

fn default_2nd_level_frequency() -> String {
    "monthly".to_string()
}

fn default_meeting_frequency() -> String {
    "biweekly".to_string()
}

fn default_overdue_threshold() -> u32 {
    3
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            version: 1,
            settings: WorkspaceSettings::default(),
        }
    }
}

/// A loaded workspace with path and configuration
#[derive(Debug, Clone)]
pub struct Workspace {
    /// Filesystem path to the workspace directory
    pub path: PathBuf,
    /// Parsed configuration from `.vibe-manager`
    pub config: WorkspaceConfig,
}

impl Workspace {
    /// Create a new workspace with the given path and config
    pub fn new(path: PathBuf, config: WorkspaceConfig) -> Self {
        Self { path, config }
    }
}
