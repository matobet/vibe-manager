//! Repository pattern for storage layer
//!
//! Encapsulates storage operations into domain-oriented repository types
//! for cleaner, more maintainable code.

mod entry;
mod report;
mod workspace;

pub use entry::EntryRepository;
pub use report::ReportRepository;
pub use workspace::WorkspaceRepository;
