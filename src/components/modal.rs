//! Modal dialog system

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::model::ReportType;
use crate::theme::{
    focused_block, style_header, style_muted, style_title, COLOR_PRIMARY, COLOR_SECONDARY,
};

/// Render a centered modal dialog
pub fn render_modal(frame: &mut Frame, area: Rect, width: u16, height: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((area.height.saturating_sub(height)) / 2),
            Constraint::Length(height),
            Constraint::Min(0),
        ])
        .split(area);

    let popup_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length((area.width.saturating_sub(width)) / 2),
            Constraint::Length(width),
            Constraint::Min(0),
        ])
        .split(popup_layout[1])[1];

    // Clear the area behind the modal
    frame.render_widget(Clear, popup_area);

    popup_area
}

// ═══════════════════════════════════════════════════════════════════════════════
// NEW REPORT MODAL - Enhanced UX with IC/Manager support
// ═══════════════════════════════════════════════════════════════════════════════

/// Field indices for the new report modal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NewReportField {
    ReportType = 0,
    Name = 1,
    Title = 2,
    Level = 3,
    Frequency = 4,
}

impl NewReportField {
    pub fn next(self) -> Self {
        match self {
            Self::ReportType => Self::Name,
            Self::Name => Self::Title,
            Self::Title => Self::Level,
            Self::Level => Self::Frequency,
            Self::Frequency => Self::ReportType, // Wrap around
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Self::ReportType => Self::Frequency,
            Self::Name => Self::ReportType,
            Self::Title => Self::Name,
            Self::Level => Self::Title,
            Self::Frequency => Self::Level,
        }
    }
}

/// State for the new report modal
#[derive(Debug, Clone)]
pub struct NewReportState {
    pub report_type: ReportType,
    pub name: String,
    pub title: String,          // Job title (required)
    pub level_index: usize,     // 0-4 for P1-P5 or M1-M5
    pub frequency_index: usize, // 0=weekly, 1=biweekly, 2=monthly
    pub current_field: NewReportField,
}

impl Default for NewReportState {
    fn default() -> Self {
        Self {
            report_type: ReportType::Individual,
            name: String::new(),
            title: String::new(),
            level_index: 2,                            // P3/M3 default
            frequency_index: 1,                        // biweekly default
            current_field: NewReportField::ReportType, // Start at type selector
        }
    }
}

impl NewReportState {
    pub fn level_str(&self) -> String {
        let prefix = if self.report_type.is_manager() {
            "M"
        } else {
            "P"
        };
        format!("{}{}", prefix, self.level_index + 1)
    }

    pub fn frequency_str(&self) -> &'static str {
        match self.frequency_index {
            0 => "weekly",
            2 => "monthly",
            _ => "biweekly",
        }
    }

    pub fn next_field(&mut self) {
        self.current_field = self.current_field.next();
    }

    pub fn prev_field(&mut self) {
        self.current_field = self.current_field.prev();
    }

    pub fn handle_left(&mut self) {
        match self.current_field {
            NewReportField::ReportType => {
                self.report_type = if self.report_type.is_manager() {
                    ReportType::Individual
                } else {
                    ReportType::Manager
                };
            }
            NewReportField::Level => {
                if self.level_index > 0 {
                    self.level_index -= 1;
                }
            }
            NewReportField::Frequency => {
                if self.frequency_index > 0 {
                    self.frequency_index -= 1;
                }
            }
            _ => {}
        }
    }

    pub fn handle_right(&mut self) {
        match self.current_field {
            NewReportField::ReportType => {
                self.report_type = if self.report_type.is_manager() {
                    ReportType::Individual
                } else {
                    ReportType::Manager
                };
            }
            NewReportField::Level => {
                if self.level_index < 4 {
                    self.level_index += 1;
                }
            }
            NewReportField::Frequency => {
                if self.frequency_index < 2 {
                    self.frequency_index += 1;
                }
            }
            _ => {}
        }
    }

    pub fn handle_char(&mut self, c: char) {
        match self.current_field {
            NewReportField::Name => self.name.push(c),
            NewReportField::Title => self.title.push(c),
            _ => {}
        }
    }

    pub fn handle_backspace(&mut self) {
        match self.current_field {
            NewReportField::Name => {
                self.name.pop();
            }
            NewReportField::Title => {
                self.title.pop();
            }
            _ => {}
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.name.trim().is_empty() && !self.title.trim().is_empty()
    }
}

/// New report modal with enhanced UX
pub struct NewReportModal<'a> {
    state: &'a NewReportState,
}

impl<'a> NewReportModal<'a> {
    pub fn new(state: &'a NewReportState) -> Self {
        Self { state }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        // Modal height: all fields single-line (6 rows + margins)
        let modal_area = render_modal(frame, area, 76, 16);

        let block = focused_block("Recruit New Report");
        let inner = block.inner(modal_area);
        frame.render_widget(block, modal_area);

        // Layout with preview on right
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(30), Constraint::Length(20)])
            .split(inner);

        // Left side: form fields
        self.render_form(frame, main_chunks[0]);

        // Right side: avatar preview
        self.render_preview(frame, main_chunks[1]);
    }

    fn render_form(&self, frame: &mut Frame, area: Rect) {
        let constraints = vec![
            Constraint::Length(2), // Report type
            Constraint::Length(2), // Name
            Constraint::Length(2), // Title
            Constraint::Length(2), // Level
            Constraint::Length(2), // Frequency
            Constraint::Length(2), // Help
        ];

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .margin(1)
            .split(area);

        // Report Type row
        self.render_type_row(frame, chunks[0]);

        // Name row
        self.render_name_row(frame, chunks[1]);

        // Title row
        self.render_title_row(frame, chunks[2]);

        // Level row
        self.render_level_row(frame, chunks[3]);

        // Frequency row
        self.render_frequency_row(frame, chunks[4]);

        // Help row
        self.render_help(frame, chunks[5]);
    }

    fn render_type_row(&self, frame: &mut Frame, area: Rect) {
        let is_active = self.state.current_field == NewReportField::ReportType;
        let is_manager = self.state.report_type.is_manager();

        // Focus indicator
        let focus = if is_active { "▸ " } else { "  " };
        let focus_style = Style::default().fg(COLOR_PRIMARY);

        // IC option styling
        let ic_selected = !is_manager;
        let ic_style = if ic_selected {
            Style::default()
                .fg(COLOR_SECONDARY)
                .add_modifier(Modifier::BOLD)
        } else {
            style_muted()
        };

        // Manager option styling
        let mgr_selected = is_manager;
        let mgr_style = if mgr_selected {
            Style::default()
                .fg(COLOR_SECONDARY)
                .add_modifier(Modifier::BOLD)
        } else {
            style_muted()
        };

        // Arrow styling - bright when focused
        let arrow_style = if is_active {
            Style::default()
                .fg(COLOR_PRIMARY)
                .add_modifier(Modifier::BOLD)
        } else {
            style_muted()
        };

        // Selection brackets
        let ic_l = if ic_selected { "[" } else { " " };
        let ic_r = if ic_selected { "]" } else { " " };
        let mgr_l = if mgr_selected { "[" } else { " " };
        let mgr_r = if mgr_selected { "]" } else { " " };

        let mut spans = vec![
            Span::styled(focus, focus_style),
            Span::styled("Type:  ", style_header()),
            Span::styled(ic_l, ic_style),
            Span::styled("IC", ic_style),
            Span::styled(ic_r, ic_style),
            Span::styled("  ◀──▶  ", arrow_style),
            Span::styled(mgr_l, mgr_style),
            Span::styled("Manager", mgr_style),
            Span::styled(mgr_r, mgr_style),
        ];

        // Show arrows hint when focused
        if is_active {
            spans.push(Span::styled(
                "  ◀▶",
                Style::default()
                    .fg(COLOR_PRIMARY)
                    .add_modifier(Modifier::BOLD),
            ));
        }

        let line = Line::from(spans);
        let para = Paragraph::new(line);
        frame.render_widget(para, area);
    }

    fn render_name_row(&self, frame: &mut Frame, area: Rect) {
        let is_active = self.state.current_field == NewReportField::Name;
        let style = if is_active {
            Style::default().fg(COLOR_SECONDARY)
        } else {
            Style::default()
        };

        // Focus indicator
        let focus = if is_active { "▸ " } else { "  " };
        let focus_style = Style::default().fg(COLOR_PRIMARY);

        let cursor = if is_active { "█" } else { "" };
        let hint = if self.state.name.is_empty() && is_active {
            "e.g. Alex Chen"
        } else {
            ""
        };

        let line = Line::from(vec![
            Span::styled(focus, focus_style),
            Span::styled("Name:  ", style_header()),
            Span::styled(&self.state.name, style),
            Span::styled(cursor, Style::default().fg(COLOR_PRIMARY)),
            Span::styled(hint, style_muted()),
        ]);

        let para = Paragraph::new(line);
        frame.render_widget(para, area);
    }

    fn render_title_row(&self, frame: &mut Frame, area: Rect) {
        let is_active = self.state.current_field == NewReportField::Title;
        let style = if is_active {
            Style::default().fg(COLOR_SECONDARY)
        } else {
            Style::default()
        };

        // Focus indicator
        let focus = if is_active { "▸ " } else { "  " };
        let focus_style = Style::default().fg(COLOR_PRIMARY);

        let cursor = if is_active { "█" } else { "" };
        let hint = if self.state.title.is_empty() && is_active {
            "e.g. Software Engineer"
        } else {
            ""
        };

        let line = Line::from(vec![
            Span::styled(focus, focus_style),
            Span::styled("Title: ", style_header()),
            Span::styled(&self.state.title, style),
            Span::styled(cursor, Style::default().fg(COLOR_PRIMARY)),
            Span::styled(hint, style_muted()),
        ]);

        let para = Paragraph::new(line);
        frame.render_widget(para, area);
    }

    fn render_level_row(&self, frame: &mut Frame, area: Rect) {
        let is_active = self.state.current_field == NewReportField::Level;
        let prefix = if self.state.report_type.is_manager() {
            "M"
        } else {
            "P"
        };

        // Focus indicator
        let focus = if is_active { "▸ " } else { "  " };
        let focus_style = Style::default().fg(COLOR_PRIMARY);

        let mut spans = vec![
            Span::styled(focus, focus_style),
            Span::styled("Level: ", style_header()),
        ];

        for i in 0..5 {
            let is_selected = i == self.state.level_index;
            let level_str = format!("{}{}", prefix, i + 1);

            let style = if is_selected {
                Style::default()
                    .fg(COLOR_SECONDARY)
                    .add_modifier(Modifier::BOLD)
            } else {
                style_muted()
            };

            let bracket_l = if is_selected { "[" } else { " " };
            let bracket_r = if is_selected { "]" } else { " " };

            spans.push(Span::styled(bracket_l, style));
            spans.push(Span::styled(level_str, style));
            spans.push(Span::styled(bracket_r, style));
        }

        // Show arrows hint when focused
        if is_active {
            spans.push(Span::styled(
                "  ◀▶",
                Style::default()
                    .fg(COLOR_PRIMARY)
                    .add_modifier(Modifier::BOLD),
            ));
        }

        let line = Line::from(spans);
        let para = Paragraph::new(line);
        frame.render_widget(para, area);
    }

    fn render_frequency_row(&self, frame: &mut Frame, area: Rect) {
        let is_active = self.state.current_field == NewReportField::Frequency;
        let frequencies = ["Weekly", "Biweekly", "Monthly"];

        // Focus indicator
        let focus = if is_active { "▸ " } else { "  " };
        let focus_style = Style::default().fg(COLOR_PRIMARY);

        let mut spans = vec![
            Span::styled(focus, focus_style),
            Span::styled("1:1s:  ", style_header()),
        ];

        for (i, freq) in frequencies.iter().enumerate() {
            let is_selected = i == self.state.frequency_index;

            let style = if is_selected {
                Style::default()
                    .fg(COLOR_SECONDARY)
                    .add_modifier(Modifier::BOLD)
            } else {
                style_muted()
            };

            let bracket_l = if is_selected { "[" } else { " " };
            let bracket_r = if is_selected { "]" } else { " " };

            spans.push(Span::styled(bracket_l, style));
            spans.push(Span::styled(*freq, style));
            spans.push(Span::styled(bracket_r, style));
            spans.push(Span::raw(" "));
        }

        // Show arrows hint when focused
        if is_active {
            spans.push(Span::styled(
                " ◀▶",
                Style::default()
                    .fg(COLOR_PRIMARY)
                    .add_modifier(Modifier::BOLD),
            ));
        }

        let line = Line::from(spans);
        let para = Paragraph::new(line);
        frame.render_widget(para, area);
    }

    fn render_help(&self, frame: &mut Frame, area: Rect) {
        // Compact help text - single spaces between items
        let help = Line::from(vec![
            Span::styled("Tab/↑↓", style_header()),
            Span::raw(" Field "),
            Span::styled("←→", style_header()),
            Span::raw(" Select "),
            Span::styled("Enter", style_header()),
            Span::raw(" Create "),
            Span::styled("Esc", style_header()),
            Span::raw(" Cancel"),
        ]);

        let para = Paragraph::new(help);
        frame.render_widget(para, area);
    }

    fn render_preview(&self, frame: &mut Frame, area: Rect) {
        // Create a preview card showing what the avatar will look like
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(style_muted())
            .title(" Preview ")
            .title_style(style_muted());

        let inner = block.inner(area);
        frame.render_widget(block, area);

        if self.state.name.is_empty() {
            let placeholder = Paragraph::new(vec![
                Line::from(""),
                Line::from(""),
                Line::from(Span::styled("Enter a name", style_muted())),
                Line::from(Span::styled("to preview", style_muted())),
            ])
            .alignment(Alignment::Center);
            frame.render_widget(placeholder, inner);
            return;
        }

        // Show avatar preview
        let level = self.state.level_str();
        let is_manager = self.state.report_type.is_manager();

        // Simplified avatar frames
        // Get avatar frame pieces based on level and type
        // Matches sprites.rs frame progression
        let (top, headband, face, bot) = if is_manager {
            match self.state.level_index {
                0 => ("╭─────╮", "│──◇──│", "│ •_• │", "╰─────╯"), // M1
                1 => ("┌─────┐", "│══◆══│", "│ •_• │", "└─────┘"), // M2
                2 => ("╔═════╗", "║══★══║", "║ •_• ║", "╚═════╝"), // M3
                3 => ("╔═════╗", "║═★═★═║", "║ •_• ║", "╚═════╝"), // M4
                _ => ("╔═════╗", "║★═★═★║", "║ •_• ║", "╚═════╝"), // M5
            }
        } else {
            match self.state.level_index {
                0 => ("╭─────╮", "", "│ •_• │", "╰─────╯"), // P1
                1 => ("┌─────┐", "", "│ •_• │", "└─────┘"), // P2
                2 => ("╔═════╗", "", "║ •_• ║", "╚═════╝"), // P3
                3 => ("╔══★══╗", "", "║ •_• ║", "╚═════╝"), // P4
                _ => ("╔═★═★═╗", "", "║ •_• ║", "╚═════╝"), // P5
            }
        };

        // Truncate name for preview
        let display_name: String = self.state.name.chars().take(12).collect();

        let mut lines = vec![
            Line::from(Span::styled(format!(" ★ {} ★ ", level), style_title())),
            Line::from(Span::styled(top, Style::default().fg(COLOR_PRIMARY))),
        ];

        // Add headband for managers
        if is_manager && !headband.is_empty() {
            lines.push(Line::from(Span::styled(
                headband,
                Style::default().fg(COLOR_PRIMARY),
            )));
        }

        // Add face
        lines.push(Line::from(Span::styled(
            face,
            Style::default().fg(COLOR_PRIMARY),
        )));

        // Add bottom border
        lines.push(Line::from(Span::styled(
            bot,
            Style::default().fg(COLOR_PRIMARY),
        )));

        // Add name
        lines.push(Line::from(Span::styled(
            display_name,
            Style::default()
                .fg(COLOR_SECONDARY)
                .add_modifier(Modifier::BOLD),
        )));

        let para = Paragraph::new(lines).alignment(Alignment::Center);
        frame.render_widget(para, inner);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// LEGACY SUPPORT - Keep old struct for backward compatibility during transition
// ═══════════════════════════════════════════════════════════════════════════════

/// Legacy new report modal (deprecated, use NewReportModal with NewReportState)
pub struct LegacyNewReportModal<'a> {
    fields: &'a [String],
    current_field: usize,
}

impl<'a> LegacyNewReportModal<'a> {
    pub fn new(fields: &'a [String], current_field: usize) -> Self {
        Self {
            fields,
            current_field,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let modal_area = render_modal(frame, area, 50, 14);

        let block = focused_block("New Report");
        let inner = block.inner(modal_area);
        frame.render_widget(block, modal_area);

        let field_labels = ["Name:", "Level:", "Meeting Frequency:"];
        let field_hints = ["e.g. Alex Chen", "P1-P5", "weekly/biweekly/monthly"];

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(2),
            ])
            .margin(1)
            .split(inner);

        for (i, (label, hint)) in field_labels.iter().zip(field_hints.iter()).enumerate() {
            let value = self.fields.get(i).map(|s| s.as_str()).unwrap_or("");
            let is_active = i == self.current_field;

            let style = if is_active {
                Style::default().fg(COLOR_SECONDARY)
            } else {
                Style::default()
            };

            let cursor = if is_active { "█" } else { "" };

            let lines = vec![
                Line::from(Span::styled(*label, style_header())),
                Line::from(vec![
                    Span::styled(value, style),
                    Span::styled(cursor, Style::default().fg(COLOR_PRIMARY)),
                    if value.is_empty() {
                        Span::styled(format!(" {}", hint), style_muted())
                    } else {
                        Span::raw("")
                    },
                ]),
            ];

            let para = Paragraph::new(lines);
            frame.render_widget(para, chunks[i]);
        }

        // Help text
        let help = Line::from(vec![
            Span::styled("Tab/Enter", style_header()),
            Span::raw(" Next field  "),
            Span::styled("Esc", style_header()),
            Span::raw(" Cancel"),
        ]);
        let help_para = Paragraph::new(help);
        frame.render_widget(help_para, chunks[3]);
    }
}

/// Help modal
pub struct HelpModal;

impl HelpModal {
    pub fn render(frame: &mut Frame, area: Rect) {
        let modal_area = render_modal(frame, area, 60, 20);

        let block = focused_block("Help");
        let inner = block.inner(modal_area);
        frame.render_widget(block, modal_area);

        let sections = vec![
            (
                "Party View",
                vec![
                    ("h/l or ←/→", "Select party member"),
                    ("j/k or ↑/↓", "Navigate grid"),
                    ("Enter/Space", "View member details"),
                    ("n", "Recruit new member"),
                    ("g/G", "Jump to first/last"),
                    ("r", "Refresh data"),
                    ("q", "Quit"),
                ],
            ),
            (
                "Member Details",
                vec![
                    ("n", "New 1-on-1 meeting"),
                    ("m", "Record mood observation"),
                    ("Enter", "View entry notes"),
                    ("Del", "Delete entry"),
                    ("Esc", "Back to party view"),
                ],
            ),
            (
                "Meeting Notes",
                vec![
                    ("←↑↓→", "Move cursor"),
                    ("Home/End", "Start/end of line"),
                    ("Del", "Delete character"),
                    ("Ctrl+S", "Save note"),
                    ("F1-F5", "Set mood (1-5)"),
                    ("Esc", "Back"),
                ],
            ),
        ];

        let mut lines = Vec::new();
        for (section, bindings) in sections {
            lines.push(Line::from(Span::styled(section, style_header())));
            for (key, desc) in bindings {
                lines.push(Line::from(vec![
                    Span::styled(
                        format!("  {:12}", key),
                        Style::default().fg(COLOR_SECONDARY),
                    ),
                    Span::raw(desc),
                ]));
            }
            lines.push(Line::from(""));
        }

        lines.push(Line::from(Span::styled(
            "Press ? or Esc to close",
            style_muted(),
        )));

        let para = Paragraph::new(lines);
        frame.render_widget(para, inner);
    }
}
