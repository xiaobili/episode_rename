//! Async task result messages.
//!
//! These messages represent the results of asynchronous operations,
//! following the Elm-style Cmd pattern for async-to-sync communication.

use crate::api::types::{FileItem, UserInfo};
use crate::error::AppError;

/// Async task result messages.
#[derive(Debug, Clone)]
pub enum AsyncMsg {
    /// Result of a login operation
    LoginResult(Result<String, AppError>),

    /// Result of an auto-login (token verification) operation
    AutoLoginResult(Result<UserInfo, AppError>),

    /// Result of a directory listing operation
    ListDirectoryResult(Result<Vec<FileItem>, AppError>),

    /// Result of a batch rename operation
    BatchRenameResult(Result<(), AppError>),
}
