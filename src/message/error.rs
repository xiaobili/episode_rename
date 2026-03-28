//! Error-related messages for centralized error handling.
//!
//! These messages handle error display and dismissal, implementing D-09
//! for centralized error handling through the message system.

use crate::state::ErrorInfo;

/// Error handling messages.
///
/// Note: The main `Message::Error(ErrorInfo)` variant is defined in mod.rs
/// as a direct variant for centralized error handling per D-09.
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum ErrorMsg {
    /// Display an error popup with the given error information
    ShowError(ErrorInfo),

    /// Dismiss the current error and return to the previous screen
    DismissError,
}
