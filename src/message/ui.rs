//! UI state messages for screen transitions and visual state.
//!
//! These messages handle all state mutations related to the UI layer,
//! including screen transitions, loading states, and error display.

use crate::state::Screen;

/// UI state messages.
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum UiMsg {
    /// Transition to a different screen
    SetScreen(Screen),

    /// Clear the current error and return to previous screen (Esc in error popup)
    ClearError,

    /// Clear error and redirect to login (Enter when token expired)
    ClearErrorAndRelogin,

    /// Start loading state with a message
    StartLoading(String),

    /// Stop loading state
    StopLoading,

    /// Advance the loading spinner animation frame
    AdvanceSpinner,

    /// Update the progress indicator (completed, total)
    UpdateProgress(usize, usize),
}
