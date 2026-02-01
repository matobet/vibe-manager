use chrono::Local;
use ratatui::style::Color;

use super::{JournalEntry, Report, ReportType};
use crate::utils::report_color;

#[derive(Debug, Clone)]
pub struct ReportSummary {
    pub name: String,
    pub level: String,
    pub meeting_frequency: String,
    pub active: bool,
    pub days_since_meeting: Option<i64>,
    pub is_overdue: bool,
    pub mood_trend: Option<MoodTrend>,
    pub recent_mood: Option<u8>,
    /// Display color for the report (derived from name hash or explicit color)
    pub color: Color,
    /// Urgency score for sorting (higher = needs more attention)
    pub urgency_score: i32,
    /// Report type (IC or Manager)
    pub report_type: ReportType,
    /// For managers: team metrics
    pub team_metrics: Option<TeamMetrics>,
}

/// Team metrics computed for managers
#[derive(Debug, Clone)]
pub struct TeamMetrics {
    /// Number of 2nd-level reports
    pub team_size: usize,
    /// Average mood across the team
    pub team_average_mood: Option<f32>,
    /// Team mood trend direction
    pub team_mood_trend: Option<MoodTrend>,
    /// Number of team members overdue for meetings
    pub team_overdue_count: usize,
    /// Composite team health score (0-100)
    pub team_health_score: u8,
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
            MoodTrend::Rising => "↗",
            MoodTrend::Falling => "↘",
            MoodTrend::Stable => " ",
        }
    }
}

pub fn compute_report_summary(
    report: &Report,
    entries: &[JournalEntry],
    overdue_threshold: u32,
) -> ReportSummary {
    let today = Local::now().date_naive();

    // For overdue calculation, only count "meetings" (entries with content)
    let meetings: Vec<&JournalEntry> = entries.iter().filter(|e| e.is_meeting()).collect();

    // Find last meeting date
    let last_meeting_date = meetings.iter().map(|m| m.date()).max();

    // Calculate days since last meeting
    let days_since_meeting = last_meeting_date.map(|d| (today - d).num_days());

    // Calculate if overdue
    let frequency_days = report.meeting_frequency_days() as i64;
    let is_overdue = days_since_meeting
        .map(|days| days > frequency_days + overdue_threshold as i64)
        .unwrap_or(true); // No meetings = overdue

    // Calculate mood trend from last 5 entries (any type, not just meetings)
    let recent_moods: Vec<u8> = entries
        .iter()
        .rev() // Most recent first
        .filter_map(|e| e.mood())
        .take(5)
        .collect();

    let recent_mood = recent_moods.first().copied();
    let mood_trend = calculate_mood_trend(&recent_moods);

    let color = report_color(report.profile.color.as_deref(), &report.profile.name);

    let urgency_score = calculate_urgency_score(
        days_since_meeting,
        frequency_days,
        overdue_threshold as i64,
        recent_mood,
        mood_trend,
    );

    ReportSummary {
        name: report.profile.name.clone(),
        level: report
            .profile
            .level
            .clone()
            .unwrap_or_else(|| "-".to_string()),
        meeting_frequency: report.profile.meeting_frequency.clone(),
        active: report.profile.active,
        days_since_meeting,
        is_overdue,
        mood_trend,
        recent_mood,
        color,
        urgency_score,
        report_type: report.profile.report_type,
        team_metrics: None, // Set separately for managers
    }
}

/// Compute team metrics for a manager from their 2nd-level report summaries
pub fn compute_team_metrics(team_summaries: &[ReportSummary]) -> TeamMetrics {
    let team_size = team_summaries.len();
    let active_summaries: Vec<_> = team_summaries.iter().filter(|s| s.active).collect();

    // Team average mood
    let moods: Vec<f32> = active_summaries
        .iter()
        .filter_map(|s| s.recent_mood)
        .map(|m| m as f32)
        .collect();

    let team_average_mood = if moods.is_empty() {
        None
    } else {
        Some(moods.iter().sum::<f32>() / moods.len() as f32)
    };

    // Team mood trend (aggregate from individual trends)
    let team_mood_trend = aggregate_mood_trends(team_summaries);

    // Count overdue
    let team_overdue_count = active_summaries.iter().filter(|s| s.is_overdue).count();

    // Calculate team health score (0-100)
    let team_health_score = calculate_team_health_score(
        team_size,
        team_average_mood,
        team_overdue_count,
        team_summaries,
    );

    TeamMetrics {
        team_size,
        team_average_mood,
        team_mood_trend,
        team_overdue_count,
        team_health_score,
    }
}

/// Aggregate individual mood trends into a team trend
fn aggregate_mood_trends(summaries: &[ReportSummary]) -> Option<MoodTrend> {
    let mut rising = 0;
    let mut falling = 0;

    for summary in summaries.iter().filter(|s| s.active) {
        match summary.mood_trend {
            Some(MoodTrend::Rising) => rising += 1,
            Some(MoodTrend::Falling) => falling += 1,
            _ => {}
        }
    }

    if rising > falling * 2 {
        Some(MoodTrend::Rising)
    } else if falling > rising * 2 {
        Some(MoodTrend::Falling)
    } else if rising > 0 || falling > 0 {
        Some(MoodTrend::Stable)
    } else {
        None
    }
}

/// Calculate a composite team health score (0-100)
/// Higher = healthier team
fn calculate_team_health_score(
    team_size: usize,
    avg_mood: Option<f32>,
    overdue_count: usize,
    summaries: &[ReportSummary],
) -> u8 {
    if team_size == 0 {
        return 0;
    }

    let mut score: f32 = 100.0;

    // Deduct for low average mood (max -30 points)
    if let Some(mood) = avg_mood {
        // mood is 1-5, ideal is 4+
        let mood_penalty = (4.0 - mood).max(0.0) * 10.0;
        score -= mood_penalty;
    } else {
        // No mood data - slight penalty
        score -= 10.0;
    }

    // Deduct for overdue meetings (max -40 points)
    let overdue_ratio = overdue_count as f32 / team_size as f32;
    score -= overdue_ratio * 40.0;

    // Deduct for falling moods (max -20 points)
    let falling_count = summaries
        .iter()
        .filter(|s| s.active && s.mood_trend == Some(MoodTrend::Falling))
        .count();
    let falling_ratio = falling_count as f32 / team_size as f32;
    score -= falling_ratio * 20.0;

    // Ensure bounds
    score.clamp(0.0, 100.0) as u8
}

/// Calculate urgency score for sorting reports by attention needed.
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
    /// Total count including 2nd-level reports
    pub total_report_count: usize,
}

pub fn compute_workspace_summary(summaries: &[ReportSummary]) -> WorkspaceSummary {
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
        total_report_count: team_size, // Will be updated to include 2nd-level reports
    }
}

/// Extended workspace summary including 2nd-level reports
pub fn compute_extended_workspace_summary(
    direct_summaries: &[ReportSummary],
    second_level_count: usize,
) -> WorkspaceSummary {
    let mut summary = compute_workspace_summary(direct_summaries);
    summary.total_report_count = direct_summaries.len() + second_level_count;
    summary
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

    #[test]
    fn test_team_health_score_healthy() {
        let summaries = vec![
            create_test_summary(Some(4), Some(MoodTrend::Stable), false),
            create_test_summary(Some(5), Some(MoodTrend::Rising), false),
            create_test_summary(Some(4), Some(MoodTrend::Stable), false),
        ];
        let metrics = compute_team_metrics(&summaries);
        assert!(metrics.team_health_score >= 90);
    }

    #[test]
    fn test_team_health_score_unhealthy() {
        let summaries = vec![
            create_test_summary(Some(2), Some(MoodTrend::Falling), true),
            create_test_summary(Some(1), Some(MoodTrend::Falling), true),
            create_test_summary(Some(3), Some(MoodTrend::Falling), true),
        ];
        let metrics = compute_team_metrics(&summaries);
        assert!(metrics.team_health_score < 50);
    }

    fn create_test_summary(
        mood: Option<u8>,
        trend: Option<MoodTrend>,
        overdue: bool,
    ) -> ReportSummary {
        ReportSummary {
            name: "Test".to_string(),
            level: "P3".to_string(),
            meeting_frequency: "biweekly".to_string(),
            active: true,
            days_since_meeting: Some(7),
            is_overdue: overdue,
            mood_trend: trend,
            recent_mood: mood,
            color: Color::White,
            urgency_score: 0,
            report_type: ReportType::Individual,
            team_metrics: None,
        }
    }
}
