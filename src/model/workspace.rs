use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    pub version: u32,
    #[serde(default)]
    pub settings: WorkspaceSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceSettings {
    #[serde(default = "default_meeting_frequency", alias = "default_cadence")]
    pub default_meeting_frequency: String,
    #[serde(default = "default_overdue_threshold")]
    pub overdue_threshold_days: u32,
    /// Default meeting frequency for 2nd-level reports (skip-levels)
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

#[derive(Debug, Clone)]
pub struct Workspace {
    pub path: PathBuf,
    pub config: WorkspaceConfig,
}

impl Workspace {
    pub fn new(path: PathBuf, config: WorkspaceConfig) -> Self {
        Self { path, config }
    }
}
