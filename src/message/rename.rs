//! Rename operation messages for all rename modes.
//!
//! These messages handle all state mutations related to file renaming,
//! including smart, manual, unified, regex, and single file rename modes.

use crate::state::RenameMode;

/// Rename operation messages.
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum RenameMsg {
    // === Mode Selection ===
    /// Open the rename mode selection popup (r key)
    OpenPopup,

    /// Close the rename popup without action (Esc)
    ClosePopup,

    /// Select a specific rename mode
    SelectMode(RenameMode),

    /// Select the next rename mode (down/j in popup)
    NextMode,

    /// Select the previous rename mode (up/k in popup)
    PreviousMode,

    // === Smart Rename ===
    /// Execute smart rename on selected files
    ExecuteSmartRename,

    // === Manual Rename ===
    /// Start manual rename mode for selected files
    StartManualRename,

    /// Submit the current manual rename and move to next file
    SubmitManualRename,

    /// Skip the current file in manual rename mode
    SkipManualRename,

    /// Cancel manual rename operation
    CancelManualRename,

    /// Append a character to the manual rename input
    InputManualRename(char),

    /// Delete the last character from the manual rename input
    DeleteManualRenameChar,

    // === Unified Rename ===
    /// Start unified naming mode
    StartUnifiedMode,

    /// Execute unified rename with current settings
    SubmitUnified,

    /// Cancel unified rename operation
    CancelUnified,

    /// Toggle focus between unified rename input fields
    ToggleUnifiedFocus,

    /// Append a character to the show name field
    InputUnifiedShowName(char),

    /// Append a character to the season field
    InputUnifiedSeason(char),

    /// Append a character to the start episode field
    InputUnifiedStartEpisode(char),

    /// Append a character to the pattern field
    InputUnifiedPattern(char),

    /// Delete the last character from the current unified input field
    DeleteUnifiedChar,

    /// Generate a preview of unified rename results
    GenerateUnifiedPreview,

    // === Regex Rename ===
    /// Start regex rename mode
    StartRegexMode,

    /// Submit regex pattern and generate preview (or execute if preview exists)
    SubmitRegex,

    /// Cancel regex rename operation
    CancelRegex,

    /// Toggle focus between find and replace fields
    ToggleRegexFocus,

    /// Append a character to the find pattern
    InputRegexFind(char),

    /// Append a character to the replace pattern
    InputRegexReplace(char),

    /// Delete the last character from the current regex input field
    DeleteRegexChar,

    /// Generate a preview of regex rename results
    GenerateRegexPreview,

    // === Single File Rename ===
    /// Start single file rename for the selected item
    StartSingleRename,

    /// Submit the single file rename
    SubmitSingleRename,

    /// Cancel single file rename
    CancelSingleRename,

    /// Append a character to the single rename input
    InputSingleRename(char),

    /// Delete the last character from the single rename input
    DeleteSingleRenameChar,
}
