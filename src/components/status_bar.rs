//! Status bar component

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::app::ViewMode;
use crate::theme::{style_muted, COLOR_PRIMARY, COLOR_SECONDARY};

pub struct StatusBar<'a> {
    view_mode: ViewMode,
    context: &'a str,
    message: Option<&'a str>,
}

impl<'a> StatusBar<'a> {
    pub fn new(view_mode: ViewMode, context: &'a str, message: Option<&'a str>) -> Self {
        Self {
            view_mode,
            context,
            message,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(20), Constraint::Length(45)])
            .split(area);

        // Left side: mode and context
        let mode_str = match self.view_mode {
            ViewMode::Dashboard => "DASHBOARD",
            ViewMode::ReportDetail | ViewMode::EntryInputModal => "REPORT",
            ViewMode::NoteViewer | ViewMode::DeleteConfirmModal => "NOTE",
            ViewMode::NewReportModal => "NEW REPORT",
            ViewMode::Help => "HELP",
        };

        let left_content = if let Some(msg) = self.message {
            Line::from(vec![
                Span::styled(
                    format!(" {} ", mode_str),
                    Style::default()
                        .fg(COLOR_PRIMARY)
                        .bg(ratatui::style::Color::DarkGray),
                ),
                Span::raw(" "),
                Span::styled(msg, Style::default().fg(COLOR_SECONDARY)),
            ])
        } else {
            Line::from(vec![
                Span::styled(
                    format!(" {} ", mode_str),
                    Style::default()
                        .fg(COLOR_PRIMARY)
                        .bg(ratatui::style::Color::DarkGray),
                ),
                Span::raw(" "),
                Span::raw(self.context),
            ])
        };

        let left = Paragraph::new(left_content);
        frame.render_widget(left, chunks[0]);

        // Right side: keybindings hint
        let hints = match self.view_mode {
            ViewMode::Dashboard => "h/l:nav  Enter:view  n:new  ?:help  q:quit",
            ViewMode::ReportDetail => "e:edit  n:new  m:mood  Del:delete  Enter:view  Bksp:back",
            ViewMode::NoteViewer | ViewMode::DeleteConfirmModal => {
                "e:edit  Del:delete  F1-F5:mood  Bksp:back"
            }
            ViewMode::EntryInputModal => "1-5:mood  Tab:context  Enter:save  Esc:cancel",
            ViewMode::Help => "?/Esc:close",
            _ => "Esc:cancel  Enter:confirm",
        };

        let right = Paragraph::new(Line::from(Span::styled(hints, style_muted())))
            .alignment(ratatui::layout::Alignment::Right);
        frame.render_widget(right, chunks[1]);
    }
}
