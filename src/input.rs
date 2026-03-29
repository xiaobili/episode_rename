//! Input handling module for key event to message conversion.
//!
//! Per D-13, all key event processing is extracted from main.rs
//! to enable independent testing and cleaner event loop.

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::app::App;
use crate::message::{AsyncMsg, AuthMsg, Message, NavMsg, RenameMsg, UiMsg};
use crate::state::{LoginFocus, RegexFocus, Screen, UnifiedFocus};
use crate::task::TaskResult;

/// Convert a key event to a Message based on current app state.
/// Returns None if the key should be ignored or handled specially (quit).
pub fn key_to_message(app: &App, key: KeyEvent) -> Option<Message> {
    // Only process key press events
    if key.kind != KeyEventKind::Press {
        return None;
    }

    // Global quit - handle specially, not as message
    if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
        return None; // Signal to quit
    }

    // Error popup takes precedence (D-09, D-10)
    if matches!(app.ui.screen, Screen::ErrorPopup { .. }) {
        return error_popup_to_message(app, key);
    }

    // Route by screen (D-13)
    match app.ui.screen {
        Screen::LoginScreen => login_screen_to_message(app, key),
        Screen::RenameModeSelection => rename_mode_to_message(app, key),
        Screen::ManualRename => manual_rename_to_message(app, key),
        Screen::UnifiedRename => unified_rename_to_message(app, key),
        Screen::RegexRename => regex_rename_to_message(app, key),
        Screen::SingleRename => single_rename_to_message(app, key),
        Screen::FolderRename => folder_rename_to_message(app, key),
        Screen::Normal => normal_mode_to_message(app, key),
        Screen::ErrorPopup { .. } => None,
    }
}

fn error_popup_to_message(app: &App, key: KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Enter => {
            if app.auth.is_token_expired {
                Some(UiMsg::ClearErrorAndRelogin.into())
            } else {
                Some(UiMsg::ClearError.into())
            }
        }
        KeyCode::Esc => Some(UiMsg::ClearError.into()),
        _ => None,
    }
}

fn login_screen_to_message(app: &App, key: KeyEvent) -> Option<Message> {
    // If loading, only allow escape
    if app.async_state.pending_task.is_loading() {
        return match key.code {
            KeyCode::Esc => {
                // Cancel loading - handle specially
                None // Will be handled in main loop
            }
            _ => None,
        };
    }

    match key.code {
        KeyCode::Esc => Some(AuthMsg::CancelLogin.into()),
        KeyCode::Enter => {
            // Validate and submit - handle specially for async spawn
            None // Will be handled in main loop
        }
        KeyCode::Tab => Some(AuthMsg::ToggleLoginFocus.into()),
        KeyCode::Backspace => match app.auth.login_focus {
            LoginFocus::Username => Some(AuthMsg::DeleteUsernameChar.into()),
            LoginFocus::Password => Some(AuthMsg::DeletePasswordChar.into()),
        },
        KeyCode::Char(c) => match app.auth.login_focus {
            LoginFocus::Username => {
                if app.auth.username_input.len() < 50 {
                    Some(AuthMsg::InputUsername(c).into())
                } else {
                    None
                }
            }
            LoginFocus::Password => {
                if app.auth.password_input.len() < 50 {
                    Some(AuthMsg::InputPassword(c).into())
                } else {
                    None
                }
            }
        },
        _ => None,
    }
}

fn rename_mode_to_message(_app: &App, key: KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Esc => Some(RenameMsg::ClosePopup.into()),
        KeyCode::Enter => {
            // Handle specially for mode-specific actions
            None
        }
        KeyCode::Up | KeyCode::Char('k') => Some(RenameMsg::PreviousMode.into()),
        KeyCode::Down | KeyCode::Char('j') => Some(RenameMsg::NextMode.into()),
        _ => None,
    }
}

fn manual_rename_to_message(app: &App, key: KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Esc => Some(RenameMsg::CancelManualRename.into()),
        KeyCode::Enter => Some(RenameMsg::SubmitManualRename.into()),
        KeyCode::Char('s') | KeyCode::Char('S') => Some(RenameMsg::SkipManualRename.into()),
        KeyCode::Backspace => Some(RenameMsg::DeleteManualRenameChar.into()),
        KeyCode::Char(c) => {
            if app.rename.manual.input.len() < 200 {
                Some(RenameMsg::InputManualRename(c).into())
            } else {
                None
            }
        }
        _ => None,
    }
}

fn unified_rename_to_message(app: &App, key: KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Esc => Some(RenameMsg::CancelUnified.into()),
        KeyCode::Enter => {
            // Handle specially for validation
            None
        }
        KeyCode::Tab => Some(RenameMsg::ToggleUnifiedFocus.into()),
        KeyCode::Backspace => Some(RenameMsg::DeleteUnifiedChar.into()),
        KeyCode::Char(c) => match app.rename.unified.focus {
            UnifiedFocus::ShowName => {
                if app.rename.unified.show_name.len() < 100 {
                    Some(RenameMsg::InputUnifiedShowName(c).into())
                } else {
                    None
                }
            }
            UnifiedFocus::Season => {
                if c.is_ascii_digit() && app.rename.unified.season.len() < 3 {
                    Some(RenameMsg::InputUnifiedSeason(c).into())
                } else {
                    None
                }
            }
            UnifiedFocus::StartEpisode => {
                if c.is_ascii_digit() && app.rename.unified.start_episode.len() < 4 {
                    Some(RenameMsg::InputUnifiedStartEpisode(c).into())
                } else {
                    None
                }
            }
            UnifiedFocus::Pattern => {
                if app.rename.unified.pattern.len() < 100 {
                    Some(RenameMsg::InputUnifiedPattern(c).into())
                } else {
                    None
                }
            }
        },
        _ => None,
    }
}

fn regex_rename_to_message(app: &App, key: KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Esc => Some(RenameMsg::CancelRegex.into()),
        KeyCode::Enter => {
            if app.has_regex_preview() {
                Some(RenameMsg::SubmitRegex.into())
            } else {
                Some(RenameMsg::GenerateRegexPreview.into())
            }
        }
        KeyCode::Tab => Some(RenameMsg::ToggleRegexFocus.into()),
        KeyCode::Backspace => Some(RenameMsg::DeleteRegexChar.into()),
        KeyCode::Char(c) => match app.rename.regex.focus {
            RegexFocus::Find => {
                if app.rename.regex.find.len() < 100 {
                    Some(RenameMsg::InputRegexFind(c).into())
                } else {
                    None
                }
            }
            RegexFocus::Replace => {
                if app.rename.regex.replace.len() < 100 {
                    Some(RenameMsg::InputRegexReplace(c).into())
                } else {
                    None
                }
            }
        },
        _ => None,
    }
}

fn single_rename_to_message(app: &App, key: KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Esc => Some(RenameMsg::CancelSingleRename.into()),
        KeyCode::Enter => {
            // Handle specially for async API call
            None
        }
        KeyCode::Backspace => Some(RenameMsg::DeleteSingleRenameChar.into()),
        KeyCode::Char(c) => {
            if app.rename.single.input.len() < 200 {
                Some(RenameMsg::InputSingleRename(c).into())
            } else {
                None
            }
        }
        _ => None,
    }
}

fn folder_rename_to_message(app: &App, key: KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Esc => Some(RenameMsg::CancelFolderRename.into()),
        KeyCode::Enter => {
            // Handle specially for async API call
            None
        }
        KeyCode::Backspace => Some(RenameMsg::DeleteFolderRenameChar.into()),
        KeyCode::Char(c) => {
            if app.rename.folder.input.len() < 200 {
                Some(RenameMsg::InputFolderRename(c).into())
            } else {
                None
            }
        }
        _ => None,
    }
}

fn normal_mode_to_message(app: &App, key: KeyEvent) -> Option<Message> {
    if app.async_state.pending_task.is_loading() {
        return None;
    }

    match key.code {
        KeyCode::Char('q') => None, // Quit handled specially
        KeyCode::Char('l') => Some(AuthMsg::StartLogin.into()),
        KeyCode::Char('r') => Some(RenameMsg::OpenPopup.into()),
        KeyCode::Char('N') => Some(RenameMsg::StartSingleRename.into()),
        KeyCode::Char('F') if key.modifiers.contains(KeyModifiers::SHIFT) => {
            Some(RenameMsg::StartFolderRename.into())
        }
        KeyCode::Up | KeyCode::Char('k') => Some(NavMsg::SelectPrevious.into()),
        KeyCode::Down | KeyCode::Char('j') => Some(NavMsg::SelectNext.into()),
        KeyCode::Tab => Some(NavMsg::ToggleFocus.into()),
        KeyCode::Enter => {
            // Handle specially for directory navigation + async load
            None
        }
        KeyCode::Left | KeyCode::Char('h') => {
            // Handle specially for async directory reload (see tasks.rs::handle_special_keys)
            None
        }
        _ => None,
    }
}

/// Convert TaskResult to Message per D-08
pub fn task_result_to_message(result: TaskResult) -> Message {
    match result {
        TaskResult::Login(_, result) => AsyncMsg::LoginResult(result).into(),
        TaskResult::AutoLogin(_, result) => AsyncMsg::AutoLoginResult(result).into(),
        TaskResult::ListDirectory(_, result) => AsyncMsg::ListDirectoryResult(result).into(),
        TaskResult::BatchRename(_, result) => AsyncMsg::BatchRenameResult(result).into(),
    }
}

/// Check if a key event should trigger quit
pub fn should_quit(key: KeyEvent) -> bool {
    (key.code == KeyCode::Char('q') && key.kind == KeyEventKind::Press)
        || (key.code == KeyCode::Char('c')
            && key.modifiers.contains(KeyModifiers::CONTROL)
            && key.kind == KeyEventKind::Press)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

    #[test]
    fn test_should_quit_q_key() {
        let key = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
        assert!(should_quit(key));
    }

    #[test]
    fn test_should_quit_ctrl_c() {
        let key = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        assert!(should_quit(key));
    }

    #[test]
    fn test_should_not_quit_other_keys() {
        let key = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE);
        assert!(!should_quit(key));
    }

    #[test]
    fn test_key_to_message_ctrl_c_returns_none() {
        let app = App::default();
        let key = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        assert!(key_to_message(&app, key).is_none());
    }
}
