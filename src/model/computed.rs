use chrono::Local;
use ratatui::style::Color;

use super::{Engineer, Meeting};
use crate::utils::engineer_color;

#[derive(Debug, Clone)]
pub struct EngineerSummary {
    pub name: String,
    pub level: String,
    pub meeting_frequency: String,
    pub active: bool,
    pub days_since_meeting: Option<i64>,
    pub is_overdue: bool,
    pub mood_trend: Option<MoodTrend>,
    pub recent_mood: Option<u8>,
    /// Display color for the engineer (derived from name hash or explicit color)
    pub color: Color,
    /// Urgency score for sorting (higher = needs more attention)
    pub urgency_score: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoodTrend {
    Rising,
    Stable,
    Falling,
}

impl MoodTrend {
    pub fn as_str(&self) -> &'static str {
        match self {
            MoodTrend::Rising => "↑",
            MoodTrend::Stable => "→",
            MoodTrend::Falling => "↓",
        }
    }
}

pub fn compute_engineer_summary(engineer: &Engineer, meetings: &[Meeting], overdue_threshold: u32) -> EngineerSummary {
    let today = Local::now().date_naive();

    // Find last meeting date
    let last_meeting_date = meetings.iter().map(|m| m.date).max();

    // Calculate days since last meeting
    let days_since_meeting = last_meeting_date.map(|d| (today - d).num_days());

    // Calculate if overdue
    let frequency_days = engineer.meeting_frequency_days() as i64;
    let is_overdue = days_since_meeting
        .map(|days| days > frequency_days + overdue_threshold as i64)
        .unwrap_or(true); // No meetings = overdue

    // Calculate mood trend from last 3 meetings
    let recent_moods: Vec<u8> = meetings
        .iter()
        .rev() // Most recent first
        .take(3)
        .filter_map(|m| m.mood())
        .collect();

    let recent_mood = recent_moods.first().copied();
    let mood_trend = calculate_mood_trend(&recent_moods);

    let color = engineer_color(
        engineer.profile.color.as_deref(),
        &engineer.profile.name,
    );

    let urgency_score = calculate_urgency_score(
        days_since_meeting,
        frequency_days,
        overdue_threshold as i64,
        recent_mood,
        mood_trend,
    );

    EngineerSummary {
        name: engineer.profile.name.clone(),
        level: engineer.profile.level.clone().unwrap_or_else(|| "-".to_string()),
        meeting_frequency: engineer.profile.meeting_frequency.clone(),
        active: engineer.profile.active,
        days_since_meeting,
        is_overdue,
        mood_trend,
        recent_mood,
        color,
        urgency_score,
    }
}

/// Calculate urgency score for sorting engineers by attention needed.
/// Higher score = more urgent attention required.
///
/// Scoring factors:
/// - Never had a meeting: +100 (highest priority)
/// - Days overdue past meeting frequency: +10 per day overdue
/// - Approaching due date: +5 if within 2 days of frequency
/// - Low mood (1-2): +20
/// - Falling mood trend: +15
/// - Unknown mood (no recent data): +10
fn calculate_urgency_score(
    days_since: Option<i64>,
    frequency_days: i64,
    overdue_threshold: i64,
    mood: Option<u8>,
    trend: Option<MoodTrend>,
) -> i32 {
    let mut score: i32 = 0;

    // Meeting urgency
    match days_since {
        None => {
            // Never had a meeting - highest priority
            score += 100;
        }
        Some(days) => {
            let days_until_due = frequency_days - days;
            let days_overdue = days - frequency_days - overdue_threshold;

            if days_overdue > 0 {
                // Overdue: +10 per day, capped at 80
                score += (days_overdue * 10).min(80) as i32;
            } else if days_until_due <= 2 {
                // Approaching due date
                score += 5;
            }
        }
    }

    // Mood urgency
    match mood {
        None => {
            // No mood data - needs check-in
            score += 10;
        }
        Some(m) if m <= 2 => {
            // Low mood - concerning
            score += 20;
        }
        _ => {}
    }

    // Trend urgency
    if trend == Some(MoodTrend::Falling) {
        score += 15;
    }

    score
}

fn calculate_mood_trend(moods: &[u8]) -> Option<MoodTrend> {
    if moods.len() < 2 {
        return None;
    }

    let newest = moods[0] as i32;
    let oldest = moods[moods.len() - 1] as i32;
    let diff = newest - oldest;

    Some(if diff > 0 {
        MoodTrend::Rising
    } else if diff < 0 {
        MoodTrend::Falling
    } else {
        MoodTrend::Stable
    })
}

#[derive(Debug, Clone)]
pub struct WorkspaceSummary {
    pub team_size: usize,
    pub active_count: usize,
    pub overdue_count: usize,
    pub average_mood: Option<f32>,
}

pub fn compute_workspace_summary(summaries: &[EngineerSummary]) -> WorkspaceSummary {
    let active_summaries: Vec<_> = summaries.iter().filter(|s| s.active).collect();

    let team_size = summaries.len();
    let active_count = active_summaries.len();
    let overdue_count = active_summaries.iter().filter(|s| s.is_overdue).count();

    let moods: Vec<f32> = active_summaries
        .iter()
        .filter_map(|s| s.recent_mood)
        .map(|m| m as f32)
        .collect();

    let average_mood = if moods.is_empty() {
        None
    } else {
        Some(moods.iter().sum::<f32>() / moods.len() as f32)
    };

    WorkspaceSummary {
        team_size,
        active_count,
        overdue_count,
        average_mood,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mood_trend_rising() {
        let moods = vec![5, 4, 3]; // newest to oldest
        assert_eq!(calculate_mood_trend(&moods), Some(MoodTrend::Rising));
    }

    #[test]
    fn test_mood_trend_falling() {
        let moods = vec![2, 3, 4]; // newest to oldest
        assert_eq!(calculate_mood_trend(&moods), Some(MoodTrend::Falling));
    }

    #[test]
    fn test_mood_trend_stable() {
        let moods = vec![3, 3, 3];
        assert_eq!(calculate_mood_trend(&moods), Some(MoodTrend::Stable));
    }

    #[test]
    fn test_mood_trend_insufficient_data() {
        let moods = vec![3];
        assert_eq!(calculate_mood_trend(&moods), None);
    }

    #[test]
    fn test_urgency_never_met() {
        // Never had a meeting = highest urgency
        let score = calculate_urgency_score(None, 14, 3, None, None);
        assert_eq!(score, 110); // 100 (never met) + 10 (no mood data)
    }

    #[test]
    fn test_urgency_overdue() {
        // 5 days overdue (20 days since, 14 day frequency, 3 day threshold)
        // days_overdue = 20 - 14 - 3 = 3
        let score = calculate_urgency_score(Some(20), 14, 3, Some(3), Some(MoodTrend::Stable));
        assert_eq!(score, 30); // 3 days * 10 = 30
    }

    #[test]
    fn test_urgency_low_mood_falling() {
        // On schedule but low mood and falling
        let score = calculate_urgency_score(Some(7), 14, 3, Some(2), Some(MoodTrend::Falling));
        assert_eq!(score, 35); // 20 (low mood) + 15 (falling)
    }

    #[test]
    fn test_urgency_all_good() {
        // Recently met, good mood, stable
        let score = calculate_urgency_score(Some(3), 14, 3, Some(4), Some(MoodTrend::Stable));
        assert_eq!(score, 0);
    }

    #[test]
    fn test_urgency_approaching_due() {
        // 12 days since meeting, 14 day frequency = 2 days until due
        let score = calculate_urgency_score(Some(12), 14, 3, Some(3), Some(MoodTrend::Stable));
        assert_eq!(score, 5); // approaching due date
    }
}
