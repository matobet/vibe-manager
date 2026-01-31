pub mod computed;
pub mod engineer;
pub mod meeting;
pub mod workspace;

pub use computed::*;
pub use engineer::*;
pub use meeting::*;
pub use workspace::*;

// Re-export JournalEntry types with clearer names
pub use meeting::{Context, JournalEntry, JournalEntryFrontmatter};
