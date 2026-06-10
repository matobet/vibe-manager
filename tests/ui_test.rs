//! Snapshot tests for dashboard card rendering
//!
//! Uses ratatui's TestBackend (renders into an in-memory buffer) with insta
//! snapshots. All summaries are constructed literals with fixed values —
//! never loaded from fixtures, whose dates drift against `Local::now()`.

use ratatui::style::Color;
use ratatui::{backend::TestBackend, Terminal};

use vibe_manager::components::{AvatarGrid, DoorwayCard, DOORWAY_CARD_HEIGHT};
use vibe_manager::model::{MoodTrend, OutlierInfo, ReportSummary, ReportType, TeamMetrics};

/// A manager summary with a troubled squad (named worst outlier + overflow)
fn manager_with_outliers() -> ReportSummary {
    ReportSummary {
        name: "Jordan Lee".to_string(),
        title: Some("Eng Manager".to_string()),
        level: "M2".to_string(),
        meeting_frequency: "weekly".to_string(),
        active: true,
        days_since_meeting: Some(3),
        is_overdue: false,
        mood_trend: Some(MoodTrend::Stable),
        recent_mood: Some(4),
        color: Color::White,
        urgency_score: 25,
        report_type: ReportType::Manager,
        team_metrics: Some(TeamMetrics {
            team_size: 4,
            team_average_mood: Some(3.2),
            team_mood_trend: Some(MoodTrend::Falling),
            team_overdue_count: 1,
            team_health_score: 76,
            outliers: vec![
                OutlierInfo {
                    name: "Sam Taylor".to_string(),
                    urgency_score: 50,
                    mood_trend: Some(MoodTrend::Falling),
                    recent_mood: Some(2),
                    is_overdue: true,
                    days_since_meeting: Some(42),
                },
                OutlierInfo {
                    name: "Kim Diaz".to_string(),
                    urgency_score: 30,
                    mood_trend: None,
                    recent_mood: Some(2),
                    is_overdue: false,
                    days_since_meeting: Some(10),
                },
                OutlierInfo {
                    name: "Pat Lopez".to_string(),
                    urgency_score: 20,
                    mood_trend: None,
                    recent_mood: None,
                    is_overdue: true,
                    days_since_meeting: Some(35),
                },
            ],
            next_in_rotation: Some("Sam Taylor".to_string()),
        }),
    }
}

/// A manager summary with an all-healthy squad
fn manager_all_well() -> ReportSummary {
    let mut summary = manager_with_outliers();
    summary.name = "Chris Wong".to_string();
    summary.team_metrics = Some(TeamMetrics {
        team_size: 5,
        team_average_mood: Some(4.4),
        team_mood_trend: Some(MoodTrend::Stable),
        team_overdue_count: 0,
        team_health_score: 90,
        outliers: vec![],
        next_in_rotation: Some("Ana Perez".to_string()),
    });
    summary
}

/// A manager with no team/ directory yet
fn manager_no_team() -> ReportSummary {
    let mut summary = manager_with_outliers();
    summary.name = "Sasha Novak".to_string();
    summary.title = Some("Team Lead".to_string());
    summary.level = "M1".to_string();
    summary.team_metrics = None;
    summary
}

/// An overdue manager: zZ on the relationship line, normal sprite
fn manager_overdue() -> ReportSummary {
    let mut summary = manager_with_outliers();
    summary.name = "Robin Vance".to_string();
    summary.is_overdue = true;
    summary.days_since_meeting = Some(56);
    summary
}

fn render_doorway(summary: &ReportSummary, is_selected: bool, width: u16) -> Terminal<TestBackend> {
    let backend = TestBackend::new(width, DOORWAY_CARD_HEIGHT);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|frame| {
            DoorwayCard::new(summary, is_selected).render(frame, frame.area());
        })
        .unwrap();
    terminal
}

#[test]
fn doorway_card_selected_shows_hint() {
    let summary = manager_with_outliers();
    let terminal = render_doorway(&summary, true, 60);
    insta::assert_snapshot!(terminal.backend());
}

#[test]
fn doorway_card_unselected_blank_hint_line() {
    let summary = manager_with_outliers();
    let terminal = render_doorway(&summary, false, 60);
    insta::assert_snapshot!(terminal.backend());
}

#[test]
fn doorway_card_healthy_squad() {
    let summary = manager_all_well();
    let terminal = render_doorway(&summary, false, 60);
    insta::assert_snapshot!(terminal.backend());
}

#[test]
fn doorway_card_no_team() {
    let summary = manager_no_team();
    let terminal = render_doorway(&summary, false, 60);
    insta::assert_snapshot!(terminal.backend());
}

#[test]
fn doorway_card_overdue_manager() {
    let summary = manager_overdue();
    let terminal = render_doorway(&summary, false, 60);
    insta::assert_snapshot!(terminal.backend());
}

/// Mood 2 face uses the fullwidth `︵` glyph — its compensation must hold
/// inside the doorway sprite column
#[test]
fn doorway_card_worried_manager_sprite_alignment() {
    let mut summary = manager_with_outliers();
    summary.name = "Mor Gan".to_string();
    summary.recent_mood = Some(2);
    let terminal = render_doorway(&summary, false, 60);
    insta::assert_snapshot!(terminal.backend());
}

/// An IC summary matching the existing AvatarCard shape
fn ic_summary(name: &str, level: &str) -> ReportSummary {
    ReportSummary {
        name: name.to_string(),
        title: None,
        level: level.to_string(),
        meeting_frequency: "biweekly".to_string(),
        active: true,
        days_since_meeting: Some(5),
        is_overdue: false,
        mood_trend: Some(MoodTrend::Stable),
        recent_mood: Some(4),
        color: Color::White,
        urgency_score: 0,
        report_type: ReportType::Individual,
        team_metrics: None,
    }
}

/// Heterogeneous grid: managers get full-width doorway rows, consecutive ICs
/// chunk into the unchanged 18×9 card grid, in one urgency-sorted sequence
#[test]
fn dashboard_grid_mixes_doorway_and_ic_rows() {
    let summaries = vec![
        manager_with_outliers(),
        ic_summary("Alex Chen", "P3"),
        ic_summary("Sam Reyes", "P2"),
        manager_all_well(),
        ic_summary("Jonas", "P1"),
    ];

    let backend = TestBackend::new(60, 28);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|frame| {
            AvatarGrid::new(&summaries, 0).render(frame, frame.area());
        })
        .unwrap();
    insta::assert_snapshot!(terminal.backend());
}

/// DOOR-03: selected and unselected renders are both 5 rows and differ only
/// in the door-hint line (content row index 3)
#[test]
fn doorway_card_height_invariant() {
    let summary = manager_with_outliers();
    let selected = render_doorway(&summary, true, 60);
    let unselected = render_doorway(&summary, false, 60);

    let selected_buffer = selected.backend().buffer();
    let unselected_buffer = unselected.backend().buffer();
    assert_eq!(selected_buffer.area.height, DOORWAY_CARD_HEIGHT);
    assert_eq!(unselected_buffer.area.height, DOORWAY_CARD_HEIGHT);

    // The sprite column (x < 9) holds the sprite's bottom frame on row 3, so
    // the invariant is over the text column: only its hint line may differ.
    const TEXT_COL_START: u16 = 9;
    let row_text = |buffer: &ratatui::buffer::Buffer, y: u16, from_x: u16| -> String {
        (from_x..buffer.area.width)
            .map(|x| buffer[(x, y)].symbol())
            .collect()
    };
    for y in 0..DOORWAY_CARD_HEIGHT {
        // Sprite column text never changes with selection (only its color does)
        assert_eq!(
            row_text(selected_buffer, y, 0)
                .chars()
                .take(9)
                .collect::<String>(),
            row_text(unselected_buffer, y, 0)
                .chars()
                .take(9)
                .collect::<String>(),
            "sprite column row {} changed with selection",
            y
        );

        let selected_text_col = row_text(selected_buffer, y, TEXT_COL_START);
        let unselected_text_col = row_text(unselected_buffer, y, TEXT_COL_START);
        if y == 3 {
            assert!(
                selected_text_col.contains("▸ Space to visit squad"),
                "selected hint line missing: {:?}",
                selected_text_col
            );
            assert_eq!(
                unselected_text_col.trim(),
                "",
                "unselected hint line not blank"
            );
        } else {
            // Styling may differ with selection, but the text must not
            assert_eq!(
                selected_text_col, unselected_text_col,
                "non-hint text row {} changed with selection",
                y
            );
        }
    }
}
