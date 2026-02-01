//! Vibe Manager - A TUI for engineering managers
//!
//! Vibe Manager is a terminal-based application for tracking 1-on-1 meetings,
//! team health, and career progress using an 8-bit RPG aesthetic.
//!
//! ## Architecture
//!
//! The application follows The Elm Architecture (TEA):
//! - **Model**: [`app::App`] holds all application state
//! - **Update**: [`app::App::update`] processes messages and returns effects
//! - **View**: The `views` module renders based on [`app::ViewMode`]
//!
//! ## Modules
//!
//! - [`app`] - Application state and TEA runtime
//! - [`components`] - Reusable UI widgets
//! - [`editor`] - External editor integration
//! - [`model`] - Data structures (Report, JournalEntry, Workspace)
//! - [`storage`] - File I/O and workspace loading
//! - [`theme`] - 8-bit color palette and styling
//! - [`utils`] - Utility functions
//! - [`views`] - Full-screen layouts

pub mod app;
pub mod components;
pub mod editor;
pub mod model;
pub mod storage;
pub mod theme;
pub mod utils;
pub mod views;
