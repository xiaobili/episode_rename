//! Message system for Elm-style state management.
//!
//! This module defines the `Message` enum that represents all possible state
//! mutation events in the application. Messages are organized into domain groups
//! for better organization and ergonomics.
//!
//! # Architecture (D-01, D-02, D-09)
//!
//! - Each domain has its own enum file under `src/message/`
//! - The main `Message` enum aggregates all domain enums
//! - `From<T>` traits allow ergonomic conversion from domain enums to `Message`
//! - `Message::Error(ErrorInfo)` provides centralized error handling (D-09)
//!
//! # Example
//!
//! ```ignore
//! use openlist_tui::message::{Message, NavMsg};
//!
//! // Create a navigation message
//! let msg: Message = NavMsg::SelectNext.into();
//!
//! // Match on the message
//! match msg {
//!     Message::Navigation(nav) => { /* handle navigation */ },
//!     Message::Error(err) => { /* handle error centrally */ },
//!     _ => {}
//! }
//! ```

pub mod navigation;
pub mod auth;
pub mod rename;
pub mod ui;
pub mod async_msg;
pub mod error;

// Re-export domain enums for convenience
pub use navigation::NavMsg;
pub use auth::AuthMsg;
pub use rename::RenameMsg;
pub use ui::UiMsg;
pub use async_msg::AsyncMsg;
pub use error::ErrorMsg;

use crate::state::ErrorInfo;

/// The main Message enum for all state mutation events.
///
/// This enum aggregates all domain-specific messages into a single type
/// that can be processed by the `update()` function.
///
/// # Variants
///
/// - `Navigation` - File/directory list navigation events
/// - `Auth` - Authentication and login events
/// - `Rename` - File rename operation events
/// - `Ui` - UI state and screen transition events
/// - `Async` - Async task result events
/// - `Error` - Centralized error handling (D-09)
#[derive(Debug, Clone)]
pub enum Message {
    /// Navigation messages for file/directory list operations
    Navigation(NavMsg),

    /// Authentication messages for login/logout operations
    Auth(AuthMsg),

    /// Rename operation messages
    Rename(RenameMsg),

    /// UI state messages
    Ui(UiMsg),

    /// Async task result messages
    Async(AsyncMsg),

    /// Error message for centralized error handling (D-09)
    ///
    /// This variant allows errors to be routed through the message system,
    /// enabling centralized error handling and consistent UI behavior.
    Error(ErrorInfo),
}

// Implement From traits for ergonomic conversion from domain enums to Message

impl From<NavMsg> for Message {
    fn from(msg: NavMsg) -> Self {
        Message::Navigation(msg)
    }
}

impl From<AuthMsg> for Message {
    fn from(msg: AuthMsg) -> Self {
        Message::Auth(msg)
    }
}

impl From<RenameMsg> for Message {
    fn from(msg: RenameMsg) -> Self {
        Message::Rename(msg)
    }
}

impl From<UiMsg> for Message {
    fn from(msg: UiMsg) -> Self {
        Message::Ui(msg)
    }
}

impl From<AsyncMsg> for Message {
    fn from(msg: AsyncMsg) -> Self {
        Message::Async(msg)
    }
}

impl From<ErrorMsg> for Message {
    fn from(msg: ErrorMsg) -> Self {
        match msg {
            ErrorMsg::ShowError(err) => Message::Error(err),
            ErrorMsg::DismissError => Message::Ui(UiMsg::ClearError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nav_msg_conversion() {
        let msg: Message = NavMsg::SelectNext.into();
        assert!(matches!(msg, Message::Navigation(NavMsg::SelectNext)));
    }

    #[test]
    fn test_auth_msg_conversion() {
        let msg: Message = AuthMsg::StartLogin.into();
        assert!(matches!(msg, Message::Auth(AuthMsg::StartLogin)));
    }

    #[test]
    fn test_error_message_variant() {
        let error_info = ErrorInfo::new("Test error".to_string());
        let msg = Message::Error(error_info.clone());
        assert!(matches!(msg, Message::Error(ref e) if e.message == "Test error"));
    }

    #[test]
    fn test_error_msg_show_error_conversion() {
        let error_info = ErrorInfo::new("Test error".to_string());
        let msg: Message = ErrorMsg::ShowError(error_info.clone()).into();
        assert!(matches!(msg, Message::Error(ref e) if e.message == "Test error"));
    }

    #[test]
    fn test_error_msg_dismiss_conversion() {
        let msg: Message = ErrorMsg::DismissError.into();
        assert!(matches!(msg, Message::Ui(UiMsg::ClearError)));
    }
}
