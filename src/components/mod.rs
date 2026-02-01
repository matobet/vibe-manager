//! UI components
//!
//! Reusable UI widgets and modal dialogs for the TUI.

pub mod avatar;
pub mod dashboard;
pub mod delete_modal;
pub mod entry_modal;
pub mod modal;
pub mod mood_chart;
pub mod note_viewer;
pub mod report_detail;
pub mod status_bar;

// Avatar and card components
pub use avatar::{AvatarCard, AvatarGrid};

// Dashboard components
pub use dashboard::{render_empty_state, render_vibe_manager_title, Dashboard};

// Modal components
pub use delete_modal::DeleteConfirmModal;
pub use entry_modal::EntryInputModal;
pub use modal::{render_modal, HelpModal, NewReportField, NewReportModal, NewReportState};

// Other components
pub use mood_chart::render_mood_chart_with_axis;
pub use note_viewer::NoteViewer;
pub use report_detail::ReportDetail;
pub use status_bar::StatusBar;
