use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Type of report - Individual Contributor or Manager
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ReportType {
    #[default]
    Individual,
    Manager,
}

impl ReportType {
    pub fn is_manager(&self) -> bool {
        matches!(self, ReportType::Manager)
    }

    pub fn is_individual(&self) -> bool {
        matches!(self, ReportType::Individual)
    }
}

/// Manager-specific information (only present for report_type: manager)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManagerInfo {
    #[serde(default)]
    pub team_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportProfile {
    pub name: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub start_date: Option<NaiveDate>,
    #[serde(default)]
    pub level: Option<String>,
    #[serde(default = "default_meeting_frequency", alias = "cadence")]
    pub meeting_frequency: String,
    #[serde(default = "default_active")]
    pub active: bool,

    /// Type of report - IC or Manager (default: IC)
    #[serde(default)]
    pub report_type: ReportType,

    /// Manager-specific info (only for report_type: manager)
    #[serde(default)]
    pub manager_info: Option<ManagerInfo>,

    // Personal info
    #[serde(default)]
    pub birthday: Option<NaiveDate>,
    #[serde(default)]
    pub partner: Option<String>,
    #[serde(default)]
    pub children: Vec<String>,

    // Skills
    #[serde(default)]
    pub skills: Option<Skills>,
    #[serde(default)]
    pub skills_updated: Option<NaiveDate>,

    // Display color (auto-generated from name hash if not set)
    #[serde(default)]
    pub color: Option<String>,
}

fn default_meeting_frequency() -> String {
    "biweekly".to_string()
}

fn default_active() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Skills {
    #[serde(default)]
    pub technical: HashMap<String, String>,
    #[serde(default)]
    pub delivery: HashMap<String, String>,
    #[serde(default)]
    pub collaboration: HashMap<String, String>,
    #[serde(default)]
    pub leadership: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Report {
    pub slug: String,
    pub path: PathBuf,
    pub profile: ReportProfile,
    pub notes_content: String,
    /// For 2nd-level reports, the slug of their manager (your direct report)
    pub manager_slug: Option<String>,
    /// For managers, their 2nd-level reports
    pub team: Vec<Report>,
}

impl Report {
    pub fn new(slug: String, path: PathBuf, profile: ReportProfile, notes_content: String) -> Self {
        Self {
            slug,
            path,
            profile,
            notes_content,
            manager_slug: None,
            team: Vec::new(),
        }
    }

    pub fn new_with_manager(
        slug: String,
        path: PathBuf,
        profile: ReportProfile,
        notes_content: String,
        manager_slug: String,
    ) -> Self {
        Self {
            slug,
            path,
            profile,
            notes_content,
            manager_slug: Some(manager_slug),
            team: Vec::new(),
        }
    }

    pub fn meeting_frequency_days(&self) -> u32 {
        match self.profile.meeting_frequency.as_str() {
            "weekly" => 7,
            "biweekly" => 14,
            "monthly" => 30,
            _ => 14, // default to biweekly
        }
    }

    /// Check if this report is a manager
    pub fn is_manager(&self) -> bool {
        self.profile.report_type.is_manager()
    }

    /// Check if this is a 2nd-level report (has a manager in your org)
    pub fn is_second_level(&self) -> bool {
        self.manager_slug.is_some()
    }

    /// Get the team size (only meaningful for managers)
    pub fn team_size(&self) -> usize {
        self.team.len()
    }
}

/// Career level tracks - supports both IC (P-track) and Manager (M-track)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Level {
    // IC Track (Adventurers)
    P1, // Junior
    P2, // Mid-level
    P3, // Senior
    P4, // Staff
    P5, // Principal

    // Manager Track (Lieutenants)
    M1, // Team Lead
    M2, // Engineering Manager
    M3, // Senior Manager
    M4, // Director
    M5, // Senior Director / VP
}

impl Level {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "P1" => Some(Level::P1),
            "P2" => Some(Level::P2),
            "P3" => Some(Level::P3),
            "P4" => Some(Level::P4),
            "P5" => Some(Level::P5),
            "M1" => Some(Level::M1),
            "M2" => Some(Level::M2),
            "M3" => Some(Level::M3),
            "M4" => Some(Level::M4),
            "M5" => Some(Level::M5),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Level::P1 => "P1",
            Level::P2 => "P2",
            Level::P3 => "P3",
            Level::P4 => "P4",
            Level::P5 => "P5",
            Level::M1 => "M1",
            Level::M2 => "M2",
            Level::M3 => "M3",
            Level::M4 => "M4",
            Level::M5 => "M5",
        }
    }

    /// Check if this is a manager level (M-track)
    pub fn is_manager(&self) -> bool {
        matches!(
            self,
            Level::M1 | Level::M2 | Level::M3 | Level::M4 | Level::M5
        )
    }

    /// Check if this is an IC level (P-track)
    pub fn is_ic(&self) -> bool {
        !self.is_manager()
    }

    /// Get the track prefix (P or M)
    pub fn track(&self) -> &'static str {
        if self.is_manager() {
            "M"
        } else {
            "P"
        }
    }

    /// Get the numeric level (1-5)
    pub fn number(&self) -> u8 {
        match self {
            Level::P1 | Level::M1 => 1,
            Level::P2 | Level::M2 => 2,
            Level::P3 | Level::M3 => 3,
            Level::P4 | Level::M4 => 4,
            Level::P5 | Level::M5 => 5,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MeetingFrequency {
    Weekly,
    Biweekly,
    Monthly,
}

impl MeetingFrequency {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "weekly" => Some(MeetingFrequency::Weekly),
            "biweekly" => Some(MeetingFrequency::Biweekly),
            "monthly" => Some(MeetingFrequency::Monthly),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            MeetingFrequency::Weekly => "weekly",
            MeetingFrequency::Biweekly => "biweekly",
            MeetingFrequency::Monthly => "monthly",
        }
    }

    pub fn days(&self) -> u32 {
        match self {
            MeetingFrequency::Weekly => 7,
            MeetingFrequency::Biweekly => 14,
            MeetingFrequency::Monthly => 30,
        }
    }
}
