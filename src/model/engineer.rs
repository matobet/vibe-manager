use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineerProfile {
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
pub struct Engineer {
    pub slug: String,
    pub path: PathBuf,
    pub profile: EngineerProfile,
    pub notes_content: String,
}

impl Engineer {
    pub fn new(slug: String, path: PathBuf, profile: EngineerProfile, notes_content: String) -> Self {
        Self {
            slug,
            path,
            profile,
            notes_content,
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Level {
    P1,
    P2,
    P3,
    P4,
    P5,
}

impl Level {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "P1" => Some(Level::P1),
            "P2" => Some(Level::P2),
            "P3" => Some(Level::P3),
            "P4" => Some(Level::P4),
            "P5" => Some(Level::P5),
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
    pub fn from_str(s: &str) -> Option<Self> {
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
