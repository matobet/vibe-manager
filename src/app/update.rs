//! Message handling and state updates
//!
//! This module implements the update function of the TEA pattern.
//! It processes messages and returns effects for the runtime to execute.

use anyhow::Result;

use super::{App, Effect, Msg, ViewMode};
use crate::model::{compute_report_summary, compute_workspace_summary, ManagerInfo};
use crate::storage;

impl App {
    /// Process a message and update state (TEA update function)
    ///
    /// Returns an Effect that the runtime should execute
    pub fn update(&mut self, msg: Msg) -> Result<Effect> {
        let effect = match msg {
            Msg::Quit => {
                self.should_quit = true;
                Effect::None
            }

            Msg::Back => {
                self.handle_back();
                Effect::None
            }

            Msg::ShowHelp => {
                self.view_mode = ViewMode::Help;
                Effect::None
            }

            Msg::HideHelp => {
                self.view_mode = ViewMode::Dashboard;
                Effect::None
            }

            Msg::SelectNext => {
                let max_len = self.current_list_len();
                if max_len > 0 {
                    self.selected_index = (self.selected_index + 1) % max_len;
                }
                Effect::None
            }

            Msg::SelectPrev => {
                let max_len = self.current_list_len();
                if max_len > 0 {
                    self.selected_index = if self.selected_index == 0 {
                        max_len - 1
                    } else {
                        self.selected_index - 1
                    };
                }
                Effect::None
            }

            Msg::SelectFirst => {
                self.selected_index = 0;
                Effect::None
            }

            Msg::SelectLast => {
                let max_len = self.current_list_len();
                if max_len > 0 {
                    self.selected_index = max_len - 1;
                }
                Effect::None
            }

            Msg::ViewReport => {
                if !self.reports.is_empty() {
                    self.selected_report_index = Some(self.selected_index);
                    self.selected_index = 0; // Reset for meeting navigation
                    self.view_mode = ViewMode::ReportDetail;
                }
                Effect::None
            }

            Msg::ViewMeeting(display_index) => {
                if let Some(actual_index) = self.meeting_display_to_entry_index(display_index) {
                    if let Some(report_idx) = self.selected_report_index {
                        self.selected_entry_index = Some(actual_index);
                        let entry = &self.entries_by_report[report_idx][actual_index];
                        self.editor_content = entry.content.clone();
                        self.editor_mood = entry.mood();
                        self.view_mode = ViewMode::NoteViewer;
                    }
                }
                Effect::None
            }

            Msg::NewMeeting => {
                if let Some(report_idx) = self.selected_report_index {
                    let report = &self.reports[report_idx];
                    match storage::create_meeting(&report.path, None) {
                        Ok(meeting) => {
                            self.editor_content = meeting.content.clone();
                            self.editor_mood = None;
                            self.entries_by_report[report_idx].push(meeting);
                            self.selected_entry_index =
                                Some(self.entries_by_report[report_idx].len() - 1);
                            self.view_mode = ViewMode::NoteViewer;
                            return Ok(Effect::SpawnEditor { is_new: true });
                        }
                        Err(e) => {
                            self.set_status(format!("Error: {}", e));
                        }
                    }
                }
                Effect::None
            }

            Msg::EditMeeting => Effect::SpawnEditor { is_new: false },

            Msg::EditMeetingFromList(display_index) => {
                if let Some(actual_index) = self.meeting_display_to_entry_index(display_index) {
                    if let Some(report_idx) = self.selected_report_index {
                        self.selected_entry_index = Some(actual_index);
                        let entry = &self.entries_by_report[report_idx][actual_index];
                        self.editor_content = entry.content.clone();
                        self.editor_mood = entry.mood();
                        // Don't change view mode - go straight to editor
                        return Ok(Effect::SpawnEditor { is_new: false });
                    }
                }
                Effect::None
            }

            Msg::UpdateMood(mood) => {
                self.editor_mood = Some(mood);
                // Save mood to disk immediately
                if let (Some(report_idx), Some(meet_idx)) =
                    (self.selected_report_index, self.selected_entry_index)
                {
                    let meeting = &mut self.entries_by_report[report_idx][meet_idx];
                    meeting.frontmatter.mood = Some(mood);
                    if let Err(e) = storage::save_entry(meeting) {
                        self.set_status(format!("Error saving mood: {}", e));
                    } else {
                        self.set_status("Mood updated");
                    }
                }
                Effect::None
            }

            Msg::ShowDeleteConfirm => {
                self.handle_show_delete_confirm();
                Effect::None
            }

            Msg::ConfirmDelete => {
                if let (Some(report_idx), Some(entry_idx)) =
                    (self.selected_report_index, self.selected_entry_index)
                {
                    match self.delete_entry(report_idx, entry_idx) {
                        Ok(()) => {
                            self.view_mode = ViewMode::ReportDetail;
                            self.set_status("Entry deleted");
                        }
                        Err(e) => {
                            self.set_status(format!("Error deleting entry: {}", e));
                        }
                    }
                }
                Effect::None
            }

            Msg::ShowNewReport => {
                self.new_report_state = crate::components::modal::NewReportState::default();
                self.view_mode = ViewMode::NewReportModal;
                Effect::None
            }

            Msg::CreateReport => {
                return self.handle_create_report();
            }

            Msg::CancelModal => {
                self.handle_cancel_modal();
                Effect::None
            }

            Msg::ModalLeft => {
                if self.view_mode == ViewMode::NewReportModal {
                    self.new_report_state.handle_left();
                }
                Effect::None
            }

            Msg::ModalRight => {
                if self.view_mode == ViewMode::NewReportModal {
                    self.new_report_state.handle_right();
                }
                Effect::None
            }

            Msg::ModalNextField => {
                if self.view_mode == ViewMode::NewReportModal {
                    self.new_report_state.next_field();
                }
                Effect::None
            }

            Msg::ModalPrevField => {
                if self.view_mode == ViewMode::NewReportModal {
                    self.new_report_state.prev_field();
                }
                Effect::None
            }

            // Entry input modal actions (mood observation)
            Msg::ShowEntryInput => {
                if self.selected_report_index.is_some() {
                    self.pending_entry_mood = None;
                    self.pending_entry_context = crate::model::Context::Standup;
                    self.pending_entry_notes.clear();
                    self.view_mode = ViewMode::EntryInputModal;
                }
                Effect::None
            }

            Msg::SetEntryMood(mood) => {
                if self.view_mode == ViewMode::EntryInputModal {
                    self.pending_entry_mood = Some(mood);
                }
                Effect::None
            }

            Msg::CycleEntryContext => {
                if self.view_mode == ViewMode::EntryInputModal {
                    self.pending_entry_context = self.pending_entry_context.next();
                }
                Effect::None
            }

            Msg::SaveEntry => {
                self.handle_save_entry();
                Effect::None
            }

            Msg::RefreshData => {
                self.load_data()?;
                Effect::None
            }

            Msg::Input(c) => {
                if self.view_mode == ViewMode::NewReportModal {
                    self.new_report_state.handle_char(c);
                } else if self.view_mode == ViewMode::EntryInputModal {
                    self.pending_entry_notes.push(c);
                }
                Effect::None
            }

            Msg::Backspace => {
                if self.view_mode == ViewMode::NewReportModal {
                    self.new_report_state.handle_backspace();
                } else if self.view_mode == ViewMode::EntryInputModal {
                    self.pending_entry_notes.pop();
                }
                Effect::None
            }

            Msg::Enter => {
                if self.view_mode == ViewMode::NewReportModal {
                    if self.new_report_state.is_valid() {
                        return self.update(Msg::CreateReport);
                    } else {
                        self.set_status("Name is required");
                    }
                }
                Effect::None
            }
        };

        Ok(effect)
    }

    /// Handle the Back message
    fn handle_back(&mut self) {
        match self.view_mode {
            ViewMode::ReportDetail => {
                // Restore report selection for dashboard navigation
                if let Some(report_idx) = self.selected_report_index {
                    self.selected_index = report_idx;
                }
                self.view_mode = ViewMode::Dashboard;
                self.selected_report_index = None;
            }
            ViewMode::NoteViewer => {
                self.view_mode = ViewMode::ReportDetail;
                self.selected_entry_index = None;
            }
            ViewMode::Help | ViewMode::NewReportModal => {
                self.view_mode = ViewMode::Dashboard;
            }
            ViewMode::EntryInputModal => {
                self.view_mode = ViewMode::ReportDetail;
                self.pending_entry_mood = None;
                self.pending_entry_notes.clear();
            }
            ViewMode::DeleteConfirmModal => {
                if self.delete_from_list {
                    self.selected_entry_index = None;
                    self.view_mode = ViewMode::ReportDetail;
                } else {
                    self.view_mode = ViewMode::NoteViewer;
                }
            }
            ViewMode::Dashboard => {}
        }
    }

    /// Handle ShowDeleteConfirm message
    fn handle_show_delete_confirm(&mut self) {
        // Handle delete from both NoteViewer and ReportDetail
        if self.view_mode == ViewMode::NoteViewer {
            // Already viewing a meeting - selected_entry_index already set
            self.delete_from_list = false;
            self.view_mode = ViewMode::DeleteConfirmModal;
        } else if self.view_mode == ViewMode::ReportDetail {
            // Deleting from the meeting list - map display index to entry index
            if let Some(actual_index) = self.meeting_display_to_entry_index(self.selected_index) {
                self.selected_entry_index = Some(actual_index);
                self.delete_from_list = true;
                self.view_mode = ViewMode::DeleteConfirmModal;
            }
        }
    }

    /// Handle CancelModal message
    fn handle_cancel_modal(&mut self) {
        match self.view_mode {
            ViewMode::NewReportModal => {
                self.view_mode = ViewMode::Dashboard;
            }
            ViewMode::EntryInputModal => {
                self.view_mode = ViewMode::ReportDetail;
                self.pending_entry_mood = None;
                self.pending_entry_notes.clear();
            }
            ViewMode::DeleteConfirmModal => {
                if self.delete_from_list {
                    self.selected_entry_index = None;
                    self.view_mode = ViewMode::ReportDetail;
                } else {
                    self.view_mode = ViewMode::NoteViewer;
                }
            }
            _ => {}
        }
    }

    /// Handle CreateReport message
    fn handle_create_report(&mut self) -> Result<Effect> {
        if !self.new_report_state.is_valid() {
            self.set_status("Name and Title are required");
            return Ok(Effect::None);
        }

        let name = self.new_report_state.name.clone();
        let title = self.new_report_state.title.clone();
        let level = self.new_report_state.level_str();
        let meeting_frequency = self.new_report_state.frequency_str().to_string();
        let report_type = self.new_report_state.report_type;

        let manager_info = if report_type.is_manager() {
            Some(ManagerInfo { team_name: None })
        } else {
            None
        };

        let profile = crate::model::ReportProfile {
            name: name.clone(),
            title: Some(title),
            start_date: Some(chrono::Local::now().date_naive()),
            level: Some(level),
            meeting_frequency,
            active: true,
            report_type,
            manager_info,
            birthday: None,
            partner: None,
            children: vec![],
            skills: None,
            skills_updated: None,
            color: None,
        };

        let type_label = if report_type.is_manager() {
            "manager"
        } else {
            "IC"
        };

        match storage::create_report(&self.workspace.path, &name, profile) {
            Ok(_) => {
                self.load_data()?;
                self.set_status(format!("Recruited {} ({})", name, type_label));
            }
            Err(e) => {
                self.set_status(format!("Error: {}", e));
            }
        }
        self.view_mode = ViewMode::Dashboard;
        Ok(Effect::None)
    }

    /// Handle SaveEntry message
    fn handle_save_entry(&mut self) {
        if let Some(report_idx) = self.selected_report_index {
            let report = &self.reports[report_idx];
            match storage::create_entry(
                &report.path,
                self.pending_entry_mood,
                Some(self.pending_entry_context),
                self.pending_entry_notes.clone(),
            ) {
                Ok(entry) => {
                    self.entries_by_report[report_idx].push(entry);
                    // Recompute summary
                    let report = &self.reports[report_idx];
                    let entries = &self.entries_by_report[report_idx];
                    self.summaries[report_idx] = compute_report_summary(
                        report,
                        entries,
                        self.workspace.config.settings.overdue_threshold_days,
                    );
                    self.workspace_summary = compute_workspace_summary(&self.summaries);
                    self.set_status("Observation recorded");
                }
                Err(e) => {
                    self.set_status(format!("Error: {}", e));
                }
            }
            self.pending_entry_mood = None;
            self.pending_entry_notes.clear();
            self.view_mode = ViewMode::ReportDetail;
        }
    }
}
