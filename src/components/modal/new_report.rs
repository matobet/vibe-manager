//! New report modal
//!
//! Modal dialog for creating a new report (team member) with IC/Manager support.

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::model::ReportType;
use crate::theme::{
    focused_block, selection_style, style_header, style_muted, style_title, COLOR_PRIMARY,
    COLOR_SECONDARY,
};

/// Field indices for the new report modal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NewReportField {
    /// Report type selector (IC/Manager)
    ReportType = 0,
    /// Name input field
    Name = 1,
    /// Job title input field
    Title = 2,
    /// Level selector (P1-P5 or M1-M5)
    Level = 3,
    /// Meeting frequency selector
    Frequency = 4,
}

impl NewReportField {
    /// Move to the next field (with wrap-around)
    pub fn next(self) -> Self {
        match self {
            Self::ReportType => Self::Name,
            Self::Name => Self::Title,
            Self::Title => Self::Level,
            Self::Level => Self::Frequency,
            Self::Frequency => Self::ReportType,
        }
    }

    /// Move to the previous field (with wrap-around)
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

/// State for the new report modal form
#[derive(Debug, Clone)]
pub struct NewReportState {
    /// Report type (IC or Manager)
    pub report_type: ReportType,
    /// Report name
    pub name: String,
    /// Job title (required)
    pub title: String,
    /// Level index (0-4 for P1-P5 or M1-M5)
    pub level_index: usize,
    /// Frequency index (0=weekly, 1=biweekly, 2=monthly)
    pub frequency_index: usize,
    /// Currently focused field
    pub current_field: NewReportField,
}

impl Default for NewReportState {
    fn default() -> Self {
        Self {
            report_type: ReportType::Individual,
            name: String::new(),
            title: String::new(),
            level_index: 2,     // P3/M3 default
            frequency_index: 1, // biweekly default
            current_field: NewReportField::ReportType,
        }
    }
}

impl NewReportState {
    /// Get the level string (e.g., "P3" or "M2")
    pub fn level_str(&self) -> String {
        let prefix = if self.report_type.is_manager() {
            "M"
        } else {
            "P"
        };
        format!("{}{}", prefix, self.level_index + 1)
    }

    /// Get the frequency string
    pub fn frequency_str(&self) -> &'static str {
        match self.frequency_index {
            0 => "weekly",
            2 => "monthly",
            _ => "biweekly",
        }
    }

    /// Move to the next field
    pub fn next_field(&mut self) {
        self.current_field = self.current_field.next();
    }

    /// Move to the previous field
    pub fn prev_field(&mut self) {
        self.current_field = self.current_field.prev();
    }

    /// Handle left arrow key
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

    /// Handle right arrow key
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

    /// Handle character input
    pub fn handle_char(&mut self, c: char) {
        match self.current_field {
            NewReportField::Name => self.name.push(c),
            NewReportField::Title => self.title.push(c),
            _ => {}
        }
    }

    /// Handle backspace
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

    /// Check if the form is valid for submission
    pub fn is_valid(&self) -> bool {
        !self.name.trim().is_empty() && !self.title.trim().is_empty()
    }
}

/// New report modal with enhanced UX
pub struct NewReportModal<'a> {
    state: &'a NewReportState,
}

impl<'a> NewReportModal<'a> {
    /// Create a new modal with the given state
    pub fn new(state: &'a NewReportState) -> Self {
        Self { state }
    }

    /// Render the modal
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let modal_area = super::render_modal(frame, area, 76, 16);

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

        self.render_type_row(frame, chunks[0]);
        self.render_text_input_row(
            frame,
            chunks[1],
            "Name",
            &self.state.name,
            "e.g. Alex Chen",
            NewReportField::Name,
        );
        self.render_text_input_row(
            frame,
            chunks[2],
            "Title",
            &self.state.title,
            "e.g. Software Engineer",
            NewReportField::Title,
        );
        self.render_level_row(frame, chunks[3]);
        self.render_frequency_row(frame, chunks[4]);
        self.render_help(frame, chunks[5]);
    }

    fn render_type_row(&self, frame: &mut Frame, area: Rect) {
        let is_active = self.state.current_field == NewReportField::ReportType;
        let is_manager = self.state.report_type.is_manager();

        let focus = if is_active { "▸ " } else { "  " };
        let focus_style = Style::default().fg(COLOR_PRIMARY);

        let ic_selected = !is_manager;
        let ic_style = selection_style(ic_selected);
        let mgr_style = selection_style(is_manager);

        let arrow_style = if is_active {
            Style::default()
                .fg(COLOR_PRIMARY)
                .add_modifier(Modifier::BOLD)
        } else {
            style_muted()
        };

        let ic_l = if ic_selected { "[" } else { " " };
        let ic_r = if ic_selected { "]" } else { " " };
        let mgr_l = if is_manager { "[" } else { " " };
        let mgr_r = if is_manager { "]" } else { " " };

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

        if is_active {
            spans.push(Span::styled(
                "  ◀▶",
                Style::default()
                    .fg(COLOR_PRIMARY)
                    .add_modifier(Modifier::BOLD),
            ));
        }

        let line = Line::from(spans);
        frame.render_widget(Paragraph::new(line), area);
    }

    fn render_text_input_row(
        &self,
        frame: &mut Frame,
        area: Rect,
        label: &str,
        value: &str,
        hint: &str,
        field: NewReportField,
    ) {
        let is_active = self.state.current_field == field;
        let style = if is_active {
            Style::default().fg(COLOR_SECONDARY)
        } else {
            Style::default()
        };

        let focus = if is_active { "▸ " } else { "  " };
        let focus_style = Style::default().fg(COLOR_PRIMARY);
        let cursor = if is_active { "█" } else { "" };
        let display_hint = if value.is_empty() && is_active {
            hint
        } else {
            ""
        };

        // Pad label to 6 characters for alignment
        let padded_label = format!("{:6} ", label);

        let line = Line::from(vec![
            Span::styled(focus, focus_style),
            Span::styled(padded_label, style_header()),
            Span::styled(value, style),
            Span::styled(cursor, Style::default().fg(COLOR_PRIMARY)),
            Span::styled(display_hint, style_muted()),
        ]);

        frame.render_widget(Paragraph::new(line), area);
    }

    fn render_level_row(&self, frame: &mut Frame, area: Rect) {
        let is_active = self.state.current_field == NewReportField::Level;
        let prefix = if self.state.report_type.is_manager() {
            "M"
        } else {
            "P"
        };

        let focus = if is_active { "▸ " } else { "  " };
        let focus_style = Style::default().fg(COLOR_PRIMARY);

        let mut spans = vec![
            Span::styled(focus, focus_style),
            Span::styled("Level: ", style_header()),
        ];

        for i in 0..5 {
            let is_selected = i == self.state.level_index;
            let level_str = format!("{}{}", prefix, i + 1);
            let style = selection_style(is_selected);

            let bracket_l = if is_selected { "[" } else { " " };
            let bracket_r = if is_selected { "]" } else { " " };

            spans.push(Span::styled(bracket_l, style));
            spans.push(Span::styled(level_str, style));
            spans.push(Span::styled(bracket_r, style));
        }

        if is_active {
            spans.push(Span::styled(
                "  ◀▶",
                Style::default()
                    .fg(COLOR_PRIMARY)
                    .add_modifier(Modifier::BOLD),
            ));
        }

        let line = Line::from(spans);
        frame.render_widget(Paragraph::new(line), area);
    }

    fn render_frequency_row(&self, frame: &mut Frame, area: Rect) {
        let is_active = self.state.current_field == NewReportField::Frequency;
        let frequencies = ["Weekly", "Biweekly", "Monthly"];

        let focus = if is_active { "▸ " } else { "  " };
        let focus_style = Style::default().fg(COLOR_PRIMARY);

        let mut spans = vec![
            Span::styled(focus, focus_style),
            Span::styled("1:1s:  ", style_header()),
        ];

        for (i, freq) in frequencies.iter().enumerate() {
            let is_selected = i == self.state.frequency_index;
            let style = selection_style(is_selected);

            let bracket_l = if is_selected { "[" } else { " " };
            let bracket_r = if is_selected { "]" } else { " " };

            spans.push(Span::styled(bracket_l, style));
            spans.push(Span::styled(*freq, style));
            spans.push(Span::styled(bracket_r, style));
            spans.push(Span::raw(" "));
        }

        if is_active {
            spans.push(Span::styled(
                " ◀▶",
                Style::default()
                    .fg(COLOR_PRIMARY)
                    .add_modifier(Modifier::BOLD),
            ));
        }

        let line = Line::from(spans);
        frame.render_widget(Paragraph::new(line), area);
    }

    fn render_help(&self, frame: &mut Frame, area: Rect) {
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

        frame.render_widget(Paragraph::new(help), area);
    }

    fn render_preview(&self, frame: &mut Frame, area: Rect) {
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

        let (top, headband, face, bot) = if is_manager {
            match self.state.level_index {
                0 => ("╭─────╮", "│──◇──│", "│ •_• │", "╰─────╯"),
                1 => ("┌─────┐", "│══◆══│", "│ •_• │", "└─────┘"),
                2 => ("╔═════╗", "║══★══║", "║ •_• ║", "╚═════╝"),
                3 => ("╔═════╗", "║═★═★═║", "║ •_• ║", "╚═════╝"),
                _ => ("╔═════╗", "║★═★═★║", "║ •_• ║", "╚═════╝"),
            }
        } else {
            match self.state.level_index {
                0 => ("╭─────╮", "", "│ •_• │", "╰─────╯"),
                1 => ("┌─────┐", "", "│ •_• │", "└─────┘"),
                2 => ("╔═════╗", "", "║ •_• ║", "╚═════╝"),
                3 => ("╔══★══╗", "", "║ •_• ║", "╚═════╝"),
                _ => ("╔═★═★═╗", "", "║ •_• ║", "╚═════╝"),
            }
        };

        let display_name: String = self.state.name.chars().take(12).collect();

        let mut lines = vec![
            Line::from(Span::styled(format!(" ★ {} ★ ", level), style_title())),
            Line::from(Span::styled(top, Style::default().fg(COLOR_PRIMARY))),
        ];

        if is_manager && !headband.is_empty() {
            lines.push(Line::from(Span::styled(
                headband,
                Style::default().fg(COLOR_PRIMARY),
            )));
        }

        lines.push(Line::from(Span::styled(
            face,
            Style::default().fg(COLOR_PRIMARY),
        )));
        lines.push(Line::from(Span::styled(
            bot,
            Style::default().fg(COLOR_PRIMARY),
        )));
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
