//! Dashboard component - team overview with RPG-style avatars

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::model::{EngineerSummary, WorkspaceSummary};
use crate::theme::{
    mood_gauge, rpg_block, simple_block, style_header, style_muted,
    style_success, style_title, style_warning, COLOR_PRIMARY, COLOR_SECONDARY,
    COLOR_SUCCESS, ICON_HEART, ICON_PERSON, ICON_WARNING,
};

use super::AvatarGrid;

pub struct Dashboard<'a> {
    summaries: &'a [EngineerSummary],
    workspace_summary: &'a WorkspaceSummary,
    selected: usize,
}

impl<'a> Dashboard<'a> {
    pub fn new(
        summaries: &'a [EngineerSummary],
        workspace_summary: &'a WorkspaceSummary,
        selected: usize,
    ) -> Self {
        Self {
            summaries,
            workspace_summary,
            selected,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        // Layout: Title, Stats panel, Avatar grid
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Title
                Constraint::Length(4),  // Stats
                Constraint::Min(12),    // Avatar grid
            ])
            .split(area);

        self.render_title(frame, chunks[0]);
        self.render_stats(frame, chunks[1]);
        self.render_party(frame, chunks[2]);
    }

    fn render_title(&self, frame: &mut Frame, area: Rect) {
        let title_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(COLOR_PRIMARY))
            .title("═══════════════════════════════════════")
            .title_alignment(Alignment::Center);

        let title_text = vec![
            Line::from(vec![
                Span::styled("⚔ ", Style::default().fg(COLOR_SECONDARY)),
                Span::styled("VIBE MANAGER", style_title()),
                Span::styled(" ⚔", Style::default().fg(COLOR_SECONDARY)),
            ]),
        ];

        let inner = title_block.inner(area);
        frame.render_widget(title_block, area);
        let para = Paragraph::new(title_text).alignment(Alignment::Center);
        frame.render_widget(para, inner);
    }

    fn render_stats(&self, frame: &mut Frame, area: Rect) {
        let stats_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(34),
            ])
            .split(area);

        // Team size
        let team_text = vec![
            Line::from(vec![
                Span::styled(format!("{} ", ICON_PERSON), Style::default().fg(COLOR_PRIMARY)),
                Span::styled(
                    format!("{}", self.workspace_summary.active_count),
                    style_header(),
                ),
                Span::styled(
                    format!(" / {}", self.workspace_summary.team_size),
                    style_muted(),
                ),
            ]),
            Line::from(Span::styled("party members", style_muted())),
        ];
        let team_para = Paragraph::new(team_text).block(simple_block("Party"));
        frame.render_widget(team_para, stats_chunks[0]);

        // Overdue count
        let overdue_style = if self.workspace_summary.overdue_count > 0 {
            style_warning()
        } else {
            style_success()
        };
        let overdue_text = vec![
            Line::from(vec![
                Span::styled(
                    format!("{} ", if self.workspace_summary.overdue_count > 0 { ICON_WARNING } else { "★" }),
                    overdue_style,
                ),
                Span::styled(
                    format!("{}", self.workspace_summary.overdue_count),
                    overdue_style,
                ),
            ]),
            Line::from(Span::styled("need attention", style_muted())),
        ];
        let overdue_para = Paragraph::new(overdue_text).block(simple_block("Quests"));
        frame.render_widget(overdue_para, stats_chunks[1]);

        // Average mood
        let mood_text = match self.workspace_summary.average_mood {
            Some(mood) => {
                let rounded = mood.round() as u8;
                vec![
                    Line::from(vec![
                        Span::styled(format!("{} ", ICON_HEART), Style::default().fg(COLOR_SUCCESS)),
                        Span::raw(mood_gauge(rounded)),
                    ]),
                    Line::from(Span::styled(format!("{:.1} morale", mood), style_muted())),
                ]
            }
            None => {
                vec![
                    Line::from(Span::styled("Unknown", style_muted())),
                    Line::from(Span::styled("check morale", style_muted())),
                ]
            }
        };
        let mood_para = Paragraph::new(mood_text).block(simple_block("Morale"));
        frame.render_widget(mood_para, stats_chunks[2]);
    }

    fn render_party(&self, frame: &mut Frame, area: Rect) {
        // Outer block with RPG title
        let block = rpg_block("Your Party");
        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Render avatar grid
        let grid = AvatarGrid::new(self.summaries, self.selected);
        grid.render(frame, inner);
    }
}

/// Render empty state when no engineers exist
pub fn render_empty_state(frame: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
        ])
        .split(area);

    // Title
    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(COLOR_PRIMARY));

    let title_text = vec![
        Line::from(vec![
            Span::styled("⚔ ", Style::default().fg(COLOR_SECONDARY)),
            Span::styled("VIBE MANAGER", style_title()),
            Span::styled(" ⚔", Style::default().fg(COLOR_SECONDARY)),
        ]),
    ];

    let inner = title_block.inner(chunks[0]);
    frame.render_widget(title_block, chunks[0]);
    let para = Paragraph::new(title_text).alignment(Alignment::Center);
    frame.render_widget(para, inner);

    // Empty party message
    let empty_art = vec![
        Line::from(""),
        Line::from(Span::styled("       ╭───╮       ", style_muted())),
        Line::from(Span::styled("       │? ?│       ", style_muted())),
        Line::from(Span::styled("       │ ─ │       ", style_muted())),
        Line::from(Span::styled("       ╰───╯       ", style_muted())),
        Line::from(Span::styled("        /█\\        ", style_muted())),
        Line::from(Span::styled("        / \\        ", style_muted())),
        Line::from(""),
        Line::from(Span::styled("Your party is empty!", style_header())),
        Line::from(""),
        Line::from("Press 'n' to recruit your first party member"),
        Line::from(""),
        Line::from(Span::styled("Tips:", style_muted())),
        Line::from(Span::styled("  • Each member gets their own quest log", style_muted())),
        Line::from(Span::styled("  • Track morale with 1-on-1 meetings", style_muted())),
        Line::from(Span::styled("  • Press '?' for help", style_muted())),
    ];

    let para = Paragraph::new(empty_art)
        .block(rpg_block("Welcome, Manager!"))
        .alignment(Alignment::Center);

    frame.render_widget(para, chunks[1]);
}
