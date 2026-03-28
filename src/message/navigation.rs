//! Navigation messages for directory and file list navigation.
//!
//! These messages handle all state mutations related to moving through
//! the file system and selecting items.

/// Navigation messages for file/directory list operations.
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum NavMsg {
    /// Move selection to the next item (down/j key)
    SelectNext,

    /// Move selection to the previous item (up/k key)
    SelectPrevious,

    /// Toggle focus between directory and file lists (Tab key)
    ToggleFocus,

    /// Enter a directory by name
    EnterDirectory(String),

    /// Navigate to the parent directory (h/left arrow)
    GoParent,

    /// Request to load directory contents asynchronously
    LoadDirectory(String),
}
