//! Central state update function for Elm-style message-driven architecture.
//!
//! This module implements the `update()` function per D-03, D-04, D-05 which:
//! - Takes a mutable reference to App state and a Message
//! - Returns nothing (in-place mutation per D-04)
//! - Delegates to domain-specific handlers for each Message variant
//!
//! # Architecture
//!
//! The update function is the heart of the Elm architecture:
//! 1. All state mutations flow through this single function
//! 2. Messages represent intent to change state
//! 3. Each domain has its own handler for better organization
//!
//! # Example
//!
//! ```ignore
//! use openlist_tui::message::{Message, NavMsg};
//! use openlist_tui::update::update;
//!
//! let mut app = App::new();
//! update(&mut app, NavMsg::SelectNext.into());
//! // app.navigation.selected_index is now updated
//! ```

use crate::api::client::OpenListClient;
use crate::app::App;
use crate::message::{AsyncMsg, AuthMsg, Message, NavMsg, RenameMsg, UiMsg};
use crate::state::{ErrorInfo, Screen};
use crate::task::PendingTask;

/// Central state update function per D-03, D-04.
///
/// Processes a message and mutates state in-place.
/// This is the single entry point for all state mutations in the Elm architecture.
///
/// # Arguments
///
/// * `state` - Mutable reference to the application state
/// * `msg` - The message to process
///
/// # Example
///
/// ```ignore
/// let mut app = App::new();
/// update(&mut app, Message::Navigation(NavMsg::SelectNext));
/// ```
pub fn update(state: &mut App, msg: Message) {
    match msg {
        Message::Navigation(nav_msg) => update_navigation(state, nav_msg),
        Message::Auth(auth_msg) => update_auth(state, auth_msg),
        Message::Rename(rename_msg) => update_rename(state, rename_msg),
        Message::Ui(ui_msg) => update_ui(state, ui_msg),
        Message::Async(async_msg) => update_async(state, async_msg),
        Message::Error(error_info) => update_error(state, error_info), // D-09
    }
}

/// Handle navigation messages for file/directory list operations.
fn update_navigation(state: &mut App, msg: NavMsg) {
    match msg {
        NavMsg::SelectNext => select_next(state),
        NavMsg::SelectPrevious => select_previous(state),
        NavMsg::ToggleFocus => toggle_focus(state),
        NavMsg::EnterDirectory(dir_name) => enter_directory(state, &dir_name),
        NavMsg::GoParent => go_parent(state),
        NavMsg::LoadDirectory(_path) => {
            // Async operation - main.rs will handle spawning the task
            // This message signals intent to load a directory
        }
    }
}

/// Handle authentication messages for login/logout operations.
fn update_auth(state: &mut App, msg: AuthMsg) {
    match msg {
        AuthMsg::StartLogin => start_login(state),
        AuthMsg::CancelLogin => clear_login(state),
        AuthMsg::ToggleLoginFocus => toggle_login_focus(state),
        AuthMsg::InputUsername(c) => append_to_username(state, c),
        AuthMsg::InputPassword(c) => append_to_password(state, c),
        AuthMsg::DeleteUsernameChar => delete_last_username_char(state),
        AuthMsg::DeletePasswordChar => delete_last_password_char(state),
        AuthMsg::SubmitLogin => {
            // Async operation - main.rs will handle spawning the login task
        }
        AuthMsg::Logout => {
            state.auth.is_authenticated = false;
            state.auth.current_user = None;
            state.config.token = None;
            let _ = state.config.save();
            state.ui.screen = Screen::LoginScreen;
        }
    }
}

/// Handle rename operation messages for all rename modes.
fn update_rename(state: &mut App, msg: RenameMsg) {
    match msg {
        // Mode selection
        RenameMsg::OpenPopup => open_rename_popup(state),
        RenameMsg::ClosePopup => close_rename_popup(state),
        RenameMsg::SelectMode(mode) => select_rename_mode(state, mode),
        RenameMsg::NextMode => select_next_rename_mode(state),
        RenameMsg::PreviousMode => select_previous_rename_mode(state),

        // Smart rename
        RenameMsg::ExecuteSmartRename => execute_smart_rename(state),

        // Manual rename
        RenameMsg::StartManualRename => start_manual_rename(state),
        RenameMsg::SubmitManualRename => submit_manual_rename(state),
        RenameMsg::SkipManualRename => skip_manual_rename(state),
        RenameMsg::CancelManualRename => cancel_manual_rename(state),
        RenameMsg::InputManualRename(c) => state.rename.manual.input.push(c),
        RenameMsg::DeleteManualRenameChar => { state.rename.manual.input.pop(); }

        // Unified rename
        RenameMsg::StartUnifiedMode => start_unified_mode(state),
        RenameMsg::SubmitUnified => {
            if let Err(e) = validate_unified_inputs(state) {
                let error_info = ErrorInfo::new(e);
                update_error(state, error_info);
            } else {
                submit_unified(state);
            }
        }
        RenameMsg::CancelUnified => cancel_unified(state),
        RenameMsg::ToggleUnifiedFocus => toggle_unified_focus(state),
        RenameMsg::InputUnifiedShowName(c) => {
            state.rename.unified.show_name.push(c);
            generate_unified_preview(state);
        }
        RenameMsg::InputUnifiedSeason(c) => {
            if c.is_ascii_digit() {
                state.rename.unified.season.push(c);
                generate_unified_preview(state);
            }
        }
        RenameMsg::InputUnifiedStartEpisode(c) => {
            if c.is_ascii_digit() {
                state.rename.unified.start_episode.push(c);
                generate_unified_preview(state);
            }
        }
        RenameMsg::InputUnifiedPattern(c) => {
            state.rename.unified.pattern.push(c);
            generate_unified_preview(state);
        }
        RenameMsg::DeleteUnifiedChar => {
            delete_unified_char(state);
            generate_unified_preview(state);
        }
        RenameMsg::GenerateUnifiedPreview => generate_unified_preview(state),

        // Regex rename
        RenameMsg::StartRegexMode => start_regex_mode(state),
        RenameMsg::SubmitRegex => submit_regex(state),
        RenameMsg::CancelRegex => cancel_regex(state),
        RenameMsg::ToggleRegexFocus => toggle_regex_focus(state),
        RenameMsg::InputRegexFind(c) => {
            state.rename.regex.find.push(c);
            state.rename.regex.preview.clear();
        }
        RenameMsg::InputRegexReplace(c) => {
            state.rename.regex.replace.push(c);
            state.rename.regex.preview.clear();
        }
        RenameMsg::DeleteRegexChar => {
            delete_regex_char(state);
            state.rename.regex.preview.clear();
        }
        RenameMsg::GenerateRegexPreview => generate_regex_preview(state),

        // Single file rename
        RenameMsg::StartSingleRename => start_single_rename(state),
        RenameMsg::SubmitSingleRename => submit_single_rename(state),
        RenameMsg::CancelSingleRename => cancel_single_rename(state),
        RenameMsg::InputSingleRename(c) => state.rename.single.input.push(c),
        RenameMsg::DeleteSingleRenameChar => { state.rename.single.input.pop(); }
    }
}

/// Handle UI state messages for screen transitions and visual state.
fn update_ui(state: &mut App, msg: UiMsg) {
    match msg {
        UiMsg::SetScreen(screen) => state.ui.screen = screen,
        UiMsg::ClearError => clear_error(state),
        UiMsg::ClearErrorAndRelogin => clear_error_and_prepare_relogin(state),
        UiMsg::StartLoading(message) => start_loading(state, message),
        UiMsg::StopLoading => stop_loading(state),
        UiMsg::AdvanceSpinner => advance_spinner(state),
        UiMsg::UpdateProgress(completed, total) => update_progress(state, completed, total),
    }
}

/// Handle async task result messages per D-06, D-08.
///
/// These messages represent the results of asynchronous operations,
/// following the Elm-style Cmd pattern for async-to-sync communication.
fn update_async(state: &mut App, msg: AsyncMsg) {
    match msg {
        AsyncMsg::LoginResult(result) => {
            state.async_state.pending_task = PendingTask::Idle;
            stop_loading(state);
            match result {
                Ok(token) => {
                    state.auth.is_authenticated = true;
                    state.auth.current_user = Some(state.auth.username_input.clone());
                    state.client = OpenListClient::new(state.config.base_url.clone(), Some(token.clone()));
                    state.config.token = Some(token);
                    state.config.username = Some(state.auth.username_input.clone());
                    let _ = state.config.save();
                    clear_login(state);
                }
                Err(e) => handle_api_error_from_app_error(state, e),
            }
        }
        AsyncMsg::AutoLoginResult(result) => {
            state.async_state.pending_task = PendingTask::Idle;
            stop_loading(state);
            match result {
                Ok(user_info) => {
                    state.auth.is_authenticated = true;
                    state.auth.current_user = Some(
                        user_info.nick.clone().unwrap_or(user_info.username)
                    );
                    state.client = OpenListClient::new(
                        state.config.base_url.clone(),
                        state.config.token.clone(),
                    );
                }
                Err(e) => {
                    state.config.token = None;
                    state.config.username = None;
                    let _ = state.config.save();
                    state.ui.screen = Screen::LoginScreen;
                    handle_api_error_from_app_error(state, e);
                }
            }
        }
        AsyncMsg::ListDirectoryResult(result) => {
            state.async_state.pending_task = PendingTask::Idle;
            stop_loading(state);
            match result {
                Ok(items) => {
                    state.navigation.directories.clear();
                    state.navigation.files.clear();
                    for item in items {
                        if item.is_dir {
                            state.navigation.directories.push(item);
                        } else if is_video_file(&item.name) {
                            state.navigation.files.push(item);
                        }
                    }
                }
                Err(e) => handle_api_error_from_app_error(state, e),
            }
        }
        AsyncMsg::BatchRenameResult(result) => {
            state.async_state.pending_task = PendingTask::Idle;
            stop_loading(state);
            match result {
                Ok(()) => {
                    // Directory reload will be handled by main.rs
                    // Just clear any rename state
                    state.rename.smart.pending = false;
                }
                Err(e) => handle_api_error_from_app_error(state, e),
            }
        }
    }
}

/// Handle Message::Error variant for centralized error handling.
///
/// Per D-09: Dedicated Error variant for centralized error handling.
/// Per D-10: Error message sets Screen::ErrorPopup with error context.
/// Per D-11: Error recovery flows through update function.
fn update_error(state: &mut App, error_info: ErrorInfo) {
    let previous_screen = std::mem::replace(&mut state.ui.screen, Screen::Normal);
    state.ui.screen = Screen::ErrorPopup {
        error: error_info,
        previous_screen: Box::new(previous_screen),
    };
    stop_loading(state);
    state.async_state.pending_task = PendingTask::Idle;
}

// ============================================================================
// Navigation Handler Functions (extracted from App impl)
// ============================================================================

use crate::state::Focus;

/// Select the next item in the current list.
pub fn select_next(state: &mut App) {
    let total = match state.navigation.focus {
        Focus::Directory => {
            state.navigation.directories.len() +
                if state.navigation.current_path != "/" && !state.navigation.current_path.is_empty() { 1 } else { 0 }
        },
        Focus::File => state.navigation.files.len(),
        Focus::Input => 0,
    };
    if total > 0 {
        state.navigation.selected_index = (state.navigation.selected_index + 1) % total;
    }
}

/// Select the previous item in the current list.
pub fn select_previous(state: &mut App) {
    let total = match state.navigation.focus {
        Focus::Directory => {
            state.navigation.directories.len() +
                if state.navigation.current_path != "/" && !state.navigation.current_path.is_empty() { 1 } else { 0 }
        },
        Focus::File => state.navigation.files.len(),
        Focus::Input => 0,
    };
    if total > 0 {
        state.navigation.selected_index = if state.navigation.selected_index == 0 {
            total - 1
        } else {
            state.navigation.selected_index - 1
        };
    }
}

/// Toggle focus between directory and file list.
pub fn toggle_focus(state: &mut App) {
    state.navigation.focus = match state.navigation.focus {
        Focus::Directory => Focus::File,
        Focus::File => Focus::Directory,
        Focus::Input => Focus::Directory,
    };
}

/// Enter a directory by name.
pub fn enter_directory(state: &mut App, dir_name: &str) {
    state.navigation.path_history.push(state.navigation.current_path.clone());
    if state.navigation.current_path == "/" {
        state.navigation.current_path = format!("/{}", dir_name);
    } else {
        state.navigation.current_path = format!("{}/{}", state.navigation.current_path, dir_name);
    }
    state.navigation.selected_index = 0;
    state.navigation.focus = Focus::Directory;
}

/// Navigate to parent directory.
pub fn go_parent(state: &mut App) {
    if state.navigation.current_path == "/" || state.navigation.current_path.is_empty() {
        return;
    }

    // Store the current directory name for selection after navigation
    let parts: Vec<&str> = state.navigation.current_path.split('/').filter(|s| !s.is_empty()).collect();
    if let Some(child_name) = parts.last() {
        state.navigation.pending_select_dir = Some(child_name.to_string());
    }

    state.navigation.path_history.push(state.navigation.current_path.clone());
    if parts.len() <= 1 {
        state.navigation.current_path = "/".to_string();
    } else {
        state.navigation.current_path = format!("/{}", parts[..parts.len() - 1].join("/"));
    }
    // Note: selected_index will be set in load_directory_contents based on pending_select_dir
    state.navigation.selected_index = 0;
    state.navigation.focus = Focus::Directory;
}

/// Load directory contents from the API.
pub async fn load_directory_contents(state: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let items = state.client.list_directory(&state.navigation.current_path).await?;
    state.navigation.directories.clear();
    state.navigation.files.clear();
    for item in items {
        if item.is_dir {
            state.navigation.directories.push(item);
        } else if is_video_file(&item.name) {
            state.navigation.files.push(item);
        }
    }

    // Select the directory we came from, if any
    if let Some(child_name) = state.navigation.pending_select_dir.take() {
        // Find the child directory in the list
        // Index 0 is ".." if not at root, so actual dirs start at 1
        if let Some(idx) = state.navigation.directories.iter().position(|d| d.name == child_name) {
            // +1 because index 0 is ".." when not at root
            let has_parent_entry = state.navigation.current_path != "/" && !state.navigation.current_path.is_empty();
            state.navigation.selected_index = if has_parent_entry { idx + 1 } else { idx };
        } else {
            state.navigation.selected_index = 0;
        }
    }

    Ok(())
}

// ============================================================================
// Auth Handler Functions (extracted from App impl)
// ============================================================================

use crate::state::LoginFocus;

/// Start the login flow by showing the login screen.
pub fn start_login(state: &mut App) {
    state.ui.screen = Screen::LoginScreen;
    state.auth.login_focus = LoginFocus::Username;
}

/// Clear login state and return to normal screen.
pub fn clear_login(state: &mut App) {
    state.ui.screen = Screen::Normal;
    state.auth.username_input.clear();
    state.auth.password_input.clear();
    state.auth.login_focus = LoginFocus::Username;
}

/// Toggle focus between username and password input fields.
pub fn toggle_login_focus(state: &mut App) {
    state.auth.login_focus = match state.auth.login_focus {
        LoginFocus::Username => LoginFocus::Password,
        LoginFocus::Password => LoginFocus::Username,
    };
}

/// Append a character to the username input.
pub fn append_to_username(state: &mut App, ch: char) {
    state.auth.username_input.push(ch);
}

/// Append a character to the password input.
pub fn append_to_password(state: &mut App, ch: char) {
    state.auth.password_input.push(ch);
}

/// Delete the last character from the username input.
pub fn delete_last_username_char(state: &mut App) {
    state.auth.username_input.pop();
}

/// Delete the last character from the password input.
pub fn delete_last_password_char(state: &mut App) {
    state.auth.password_input.pop();
}

// ============================================================================
// Error Handler Functions (extracted from App impl)
// ============================================================================

/// Handle API errors from AppError type.
pub fn handle_api_error_from_app_error(state: &mut App, error: crate::error::AppError) {
    use crate::error::AppError;

    let error_type = Some(error.error_type().to_string());
    let error_code = error.error_code();

    match &error {
        AppError::TokenExpired => {
            state.auth.is_token_expired = true;
            state.auth.auto_relogin_pending = true;
            state.auth.is_authenticated = false;
            let error_info = ErrorInfo::with_code(
                "Token 已过期，请重新登录".to_string(),
                error_type,
                error_code,
            );
            state.ui.screen = Screen::ErrorPopup {
                error: error_info,
                previous_screen: Box::new(Screen::Normal),
            };
        }
        AppError::Auth(msg) => {
            state.auth.is_token_expired = false;
            let error_info = ErrorInfo::with_code(
                format!("认证失败：{}", msg),
                error_type,
                error_code,
            );
            state.ui.screen = Screen::ErrorPopup {
                error: error_info,
                previous_screen: Box::new(Screen::Normal),
            };
        }
        AppError::Network(e) => {
            state.auth.is_token_expired = false;
            let error_info = ErrorInfo::with_code(
                format!("网络错误：{}", e),
                error_type,
                error_code,
            );
            state.ui.screen = Screen::ErrorPopup {
                error: error_info,
                previous_screen: Box::new(Screen::Normal),
            };
        }
        AppError::NotFound(path) => {
            state.auth.is_token_expired = false;
            let error_info = ErrorInfo::with_code(
                format!("路径不存在：{}", path),
                error_type,
                error_code,
            );
            state.ui.screen = Screen::ErrorPopup {
                error: error_info,
                previous_screen: Box::new(Screen::Normal),
            };
        }
        AppError::ApiError(msg) => {
            state.auth.is_token_expired = false;
            let error_info = ErrorInfo::with_code(
                format!("API 错误：{}", msg),
                error_type,
                error_code,
            );
            state.ui.screen = Screen::ErrorPopup {
                error: error_info,
                previous_screen: Box::new(Screen::Normal),
            };
        }
        _ => {
            state.auth.is_token_expired = false;
            let error_info = ErrorInfo::with_code(
                format!("{}", error),
                error_type,
                error_code,
            );
            state.ui.screen = Screen::ErrorPopup {
                error: error_info,
                previous_screen: Box::new(Screen::Normal),
            };
        }
    }
}

/// Handle API errors from boxed dyn Error.
pub fn handle_api_error(state: &mut App, error: Box<dyn std::error::Error + 'static>) {
    use crate::error::AppError;
    let app_error = AppError::from_boxed_error(error);
    handle_api_error_from_app_error(state, app_error);
}

/// Clear error state and return to previous screen.
pub fn clear_error(state: &mut App) {
    let previous_screen = match &state.ui.screen {
        Screen::ErrorPopup { previous_screen, .. } => (**previous_screen).clone(),
        _ => Screen::Normal,
    };
    state.ui.screen = previous_screen;
}

/// Clear error state and prepare for re-login.
pub fn clear_error_and_prepare_relogin(state: &mut App) {
    let _previous_screen = match &state.ui.screen {
        Screen::ErrorPopup { previous_screen, .. } => (**previous_screen).clone(),
        _ => Screen::Normal,
    };
    state.ui.screen = Screen::LoginScreen;
    state.auth.is_token_expired = false;
    state.auth.auto_relogin_pending = false;
}

// ============================================================================
// UI Handler Functions (extracted from App impl)
// ============================================================================

/// Start loading animation with a message.
pub fn start_loading(state: &mut App, message: String) {
    state.ui.loading_message = Some(message);
    state.ui.loading_progress = None;
    state.ui.loading_spinner_frame = 0;
}

/// Update loading progress.
pub fn update_progress(state: &mut App, completed: usize, total: usize) {
    state.ui.loading_progress = Some((completed, total));
}

/// Stop loading animation.
pub fn stop_loading(state: &mut App) {
    state.ui.loading_message = None;
    state.ui.loading_progress = None;
    state.ui.loading_spinner_frame = 0;
}

/// Advance spinner animation frame.
pub fn advance_spinner(state: &mut App) {
    state.ui.loading_spinner_frame = (state.ui.loading_spinner_frame + 1) % 10;
}

/// Get the current spinner character.
#[allow(dead_code)]
pub fn get_spinner_char(state: &App) -> char {
    const SPINNER_CHARS: [char; 10] = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
    SPINNER_CHARS[state.ui.loading_spinner_frame % 10]
}

// ============================================================================
// Rename Handler Functions (extracted from App impl)
// ============================================================================

use crate::state::{RenameMode, UnifiedFocus, RegexFocus};

// Mode selection
pub fn open_rename_popup(state: &mut App) {
    state.ui.screen = Screen::RenameModeSelection;
    state.rename.mode_selection.selected_mode = RenameMode::Smart;
    generate_rename_preview(state);
}

pub fn close_rename_popup(state: &mut App) {
    state.ui.screen = Screen::Normal;
    state.rename.mode_selection.preview.clear();
}

pub fn select_rename_mode(state: &mut App, mode: RenameMode) {
    state.rename.mode_selection.selected_mode = mode;
    generate_rename_preview(state);
}

pub fn select_next_rename_mode(state: &mut App) {
    let modes = RenameMode::all();
    let current_idx = modes.iter().position(|&m| m == state.rename.mode_selection.selected_mode).unwrap_or(0);
    let next_idx = (current_idx + 1) % modes.len();
    state.rename.mode_selection.selected_mode = modes[next_idx];
    generate_rename_preview(state);
}

pub fn select_previous_rename_mode(state: &mut App) {
    let modes = RenameMode::all();
    let current_idx = modes.iter().position(|&m| m == state.rename.mode_selection.selected_mode).unwrap_or(0);
    let prev_idx = if current_idx == 0 { modes.len() - 1 } else { current_idx - 1 };
    state.rename.mode_selection.selected_mode = modes[prev_idx];
    generate_rename_preview(state);
}

pub fn generate_rename_preview(state: &mut App) {
    use crate::models::episode::EpisodeParser;

    state.rename.mode_selection.preview.clear();
    let parser = EpisodeParser::new();

    let selected_file = match state.navigation.focus {
        Focus::File => state.navigation.files.get(state.navigation.selected_index),
        _ => None,
    };

    if let Some(file) = selected_file {
        if let Some(episode_info) = parser.parse(&file.name) {
            let new_name = match state.rename.mode_selection.selected_mode {
                RenameMode::Smart => {
                    parser.generate_name(&episode_info, "{title}.S{season}E{episode}",
                        &file.name.rsplit('.').next().map(|e| format!(".{}", e)).unwrap_or_default())
                }
                RenameMode::Manual => file.name.clone(),
                RenameMode::Unified => format!("{}_{:02}", episode_info.title, episode_info.episode),
                RenameMode::Regex => file.name.clone(),
            };
            state.rename.mode_selection.preview.push(format!("{} -> {}", file.name, new_name));
        } else {
            state.rename.mode_selection.preview.push(format!("{} (无法识别)", file.name));
        }
    } else if state.navigation.files.is_empty() {
        state.rename.mode_selection.preview.push("没有可重命名的文件".to_string());
    } else {
        for file in &state.navigation.files {
            if let Some(episode_info) = parser.parse(&file.name) {
                let ext = file.name.rsplit('.').next().map(|e| format!(".{}", e)).unwrap_or_default();
                let new_name = parser.generate_name(&episode_info, "{title}.S{season}E{episode}", &ext);
                state.rename.mode_selection.preview.push(format!("{} -> {}", file.name, new_name));
            } else {
                state.rename.mode_selection.preview.push(format!("{} (无法识别)", file.name));
            }
        }
    }
}

// Smart rename
pub fn execute_smart_rename(state: &mut App) {
    use crate::models::episode::EpisodeParser;

    state.rename.smart.results.clear();
    let parser = EpisodeParser::new();

    for file in &state.navigation.files {
        if let Some(episode_info) = parser.parse(&file.name) {
            let ext = file.name.rsplit('.').next().map(|e| format!(".{}", e)).unwrap_or_default();
            let new_name = parser.generate_name(&episode_info, "{title}.S{season}E{episode}", &ext);
            if new_name != file.name {
                state.rename.smart.results.push((file.name.clone(), new_name, true));
            }
        }
    }

    state.rename.smart.pending = true;
}

// Manual rename
pub fn start_manual_rename(state: &mut App) {
    if state.navigation.files.is_empty() {
        return;
    }
    state.rename.manual.files_to_rename = (0..state.navigation.files.len()).collect();
    state.rename.manual.index = 0;
    state.rename.manual.results.clear();

    if let Some(file) = state.navigation.files.get(state.rename.manual.index) {
        state.rename.manual.input = file.name.clone();
    }
    state.ui.screen = Screen::ManualRename;
}

pub fn submit_manual_rename(state: &mut App) {
    if let Some(file) = state.navigation.files.get(state.rename.manual.index) {
        let old_name = file.name.clone();
        let new_name = state.rename.manual.input.clone();
        if !new_name.is_empty() && new_name != old_name {
            state.rename.manual.results.push((old_name, new_name, true));
        }
    }
    next_manual_rename(state);
}

pub fn skip_manual_rename(state: &mut App) {
    next_manual_rename(state);
}

pub fn next_manual_rename(state: &mut App) {
    state.rename.manual.index += 1;

    if state.rename.manual.index >= state.navigation.files.len() {
        finish_manual_rename(state);
    } else {
        if let Some(file) = state.navigation.files.get(state.rename.manual.index) {
            state.rename.manual.input = file.name.clone();
        }
    }
}

fn finish_manual_rename(state: &mut App) {
    state.ui.screen = Screen::Normal;
    state.rename.manual.input.clear();
    state.rename.manual.files_to_rename.clear();
    state.rename.manual.index = 0;
    state.rename.manual.finished = true;
}

pub fn cancel_manual_rename(state: &mut App) {
    state.ui.screen = Screen::Normal;
    state.rename.manual.input.clear();
    state.rename.manual.files_to_rename.clear();
    state.rename.manual.index = 0;
    state.rename.manual.results.clear();
    state.rename.manual.finished = false;
}

#[allow(dead_code)]
pub fn delete_last_manual_rename_char(state: &mut App) {
    state.rename.manual.input.pop();
}

#[allow(dead_code)]
pub fn get_current_manual_rename_file(state: &App) -> Option<&crate::api::types::FileItem> {
    state.navigation.files.get(state.rename.manual.index)
}

#[allow(dead_code)]
pub fn get_manual_rename_progress(state: &App) -> (usize, usize) {
    // Return 1-indexed progress (file 1 of 4, file 2 of 4, etc.)
    (state.rename.manual.index + 1, state.navigation.files.len())
}

// Unified rename
pub fn start_unified_mode(state: &mut App) {
    if state.navigation.files.is_empty() {
        return;
    }
    state.ui.screen = Screen::UnifiedRename;
    state.rename.unified.focus = UnifiedFocus::ShowName;
    state.rename.unified.show_name.clear();
    state.rename.unified.season = "1".to_string();
    state.rename.unified.start_episode = "1".to_string();
    state.rename.unified.pattern = "{title}.S{season}E{episode}".to_string();
    state.rename.unified.preview.clear();
    state.rename.unified.results.clear();
    state.rename.unified.finished = false;
    generate_unified_preview(state);
}

pub fn submit_unified(state: &mut App) {
    state.rename.unified.finished = true;
    state.ui.screen = Screen::Normal;
}

pub fn cancel_unified(state: &mut App) {
    state.ui.screen = Screen::Normal;
    state.rename.unified.show_name.clear();
    state.rename.unified.season.clear();
    state.rename.unified.start_episode.clear();
    state.rename.unified.pattern = "{title}.S{season}E{episode}".to_string();
    state.rename.unified.focus = UnifiedFocus::ShowName;
    state.rename.unified.preview.clear();
    state.rename.unified.results.clear();
    state.rename.unified.finished = false;
}

pub fn toggle_unified_focus(state: &mut App) {
    state.rename.unified.focus = match state.rename.unified.focus {
        UnifiedFocus::ShowName => UnifiedFocus::Season,
        UnifiedFocus::Season => UnifiedFocus::StartEpisode,
        UnifiedFocus::StartEpisode => UnifiedFocus::Pattern,
        UnifiedFocus::Pattern => UnifiedFocus::ShowName,
    };
}

pub fn generate_unified_preview(state: &mut App) {
    state.rename.unified.preview.clear();

    let show_name = if state.rename.unified.show_name.is_empty() {
        "Show".to_string()
    } else {
        state.rename.unified.show_name.clone()
    };

    let season: u32 = state.rename.unified.season.parse().unwrap_or(1);
    let start_episode: u32 = state.rename.unified.start_episode.parse().unwrap_or(1);

    for (i, file) in state.navigation.files.iter().take(5).enumerate() {
        let episode = start_episode + i as u32;
        let ext = file.name.rsplit('.').next().map(|e| format!(".{}", e)).unwrap_or_default();

        let s = format!("{:02}", season);
        let e = format!("{:02}", episode);
        let new_name = format!(
            "{}{}",
            state.rename.unified.pattern
                .replace("{title}", &show_name)
                .replace("{season}", &s)
                .replace("{episode}", &e),
            ext
        );

        state.rename.unified.preview.push(format!("{} -> {}", file.name, new_name));
    }

    if state.navigation.files.len() > 5 {
        state.rename.unified.preview.push(format!("... 还有 {} 个文件", state.navigation.files.len() - 5));
    }
}

pub fn validate_unified_inputs(state: &App) -> Result<(), String> {
    if state.rename.unified.show_name.is_empty() {
        return Err("剧集名称不能为空".to_string());
    }
    if state.rename.unified.season.is_empty() {
        return Err("季数不能为空".to_string());
    }
    if state.rename.unified.season.parse::<u32>().is_err() {
        return Err("季数必须是数字".to_string());
    }
    if state.rename.unified.start_episode.is_empty() {
        return Err("起始集数不能为空".to_string());
    }
    if state.rename.unified.start_episode.parse::<u32>().is_err() {
        return Err("起始集数必须是数字".to_string());
    }
    Ok(())
}

pub fn execute_unified_rename(state: &mut App) -> Vec<(String, String, bool)> {
    state.rename.unified.results.clear();

    let show_name = state.rename.unified.show_name.clone();
    let season: u32 = state.rename.unified.season.parse().unwrap_or(1);
    let start_episode: u32 = state.rename.unified.start_episode.parse().unwrap_or(1);

    for (i, file) in state.navigation.files.iter().enumerate() {
        let episode = start_episode + i as u32;
        let ext = file.name.rsplit('.').next().map(|e| format!(".{}", e)).unwrap_or_default();

        let s = format!("{:02}", season);
        let e = format!("{:02}", episode);
        let new_name = format!(
            "{}{}",
            state.rename.unified.pattern
                .replace("{title}", &show_name)
                .replace("{season}", &s)
                .replace("{episode}", &e),
            ext
        );

        if new_name != file.name {
            state.rename.unified.results.push((file.name.clone(), new_name, true));
        }
    }

    state.rename.unified.finished = true;
    state.ui.screen = Screen::Normal;

    state.rename.unified.results.clone()
}

// Regex rename
pub fn start_regex_mode(state: &mut App) {
    if state.navigation.files.is_empty() {
        return;
    }
    state.ui.screen = Screen::RegexRename;
    state.rename.regex.focus = RegexFocus::Find;
    state.rename.regex.find.clear();
    state.rename.regex.replace.clear();
    state.rename.regex.preview.clear();
    state.rename.regex.results.clear();
    state.rename.regex.finished = false;
    state.rename.regex.error = None;
}

pub fn cancel_regex(state: &mut App) {
    state.ui.screen = Screen::Normal;
    state.rename.regex.find.clear();
    state.rename.regex.replace.clear();
    state.rename.regex.focus = RegexFocus::Find;
    state.rename.regex.preview.clear();
    state.rename.regex.results.clear();
    state.rename.regex.finished = false;
    state.rename.regex.error = None;
}

pub fn toggle_regex_focus(state: &mut App) {
    state.rename.regex.focus = match state.rename.regex.focus {
        RegexFocus::Find => RegexFocus::Replace,
        RegexFocus::Replace => RegexFocus::Find,
    };
}

pub fn submit_regex(state: &mut App) {
    match regex::Regex::new(&state.rename.regex.find) {
        Ok(_) => {
            generate_regex_preview(state);
            state.rename.regex.error = None;
        }
        Err(e) => {
            state.rename.regex.error = Some(format!("正则表达式无效：{}", e));
        }
    }
}

pub fn generate_regex_preview(state: &mut App) {
    state.rename.regex.preview.clear();

    if let Ok(re) = regex::Regex::new(&state.rename.regex.find) {
        for file in &state.navigation.files {
            let new_name = re.replace_all(&file.name, &state.rename.regex.replace).to_string();
            if new_name != file.name {
                state.rename.regex.preview.push((file.name.clone(), new_name));
            }
        }
    }
}

pub fn execute_regex_rename(state: &mut App) -> Vec<(String, String, bool)> {
    state.rename.regex.results.clear();

    if let Ok(re) = regex::Regex::new(&state.rename.regex.find) {
        for file in &state.navigation.files {
            let new_name = re.replace_all(&file.name, &state.rename.regex.replace).to_string();
            if new_name != file.name {
                state.rename.regex.results.push((file.name.clone(), new_name, true));
            }
        }
    }

    state.rename.regex.finished = true;
    state.ui.screen = Screen::Normal;

    state.rename.regex.results.clone()
}

// Single file rename
pub fn start_single_rename(state: &mut App) {
    if state.navigation.focus != Focus::File {
        return;
    }

    let selected_file = state.navigation.files.get(state.navigation.selected_index);
    if let Some(file) = selected_file {
        state.rename.single.target = Some(file.clone());
        state.rename.single.input = file.name.clone();
        state.ui.screen = Screen::SingleRename;
    }
}

pub fn submit_single_rename(state: &mut App) {
    if state.rename.single.target.is_some() && !state.rename.single.input.is_empty() {
        state.ui.screen = Screen::Normal;
    }
}

pub fn cancel_single_rename(state: &mut App) {
    state.ui.screen = Screen::Normal;
    state.rename.single.input.clear();
    state.rename.single.target = None;
}

#[allow(dead_code)]
pub fn delete_last_single_rename_char(state: &mut App) {
    state.rename.single.input.pop();
}

// ============================================================================
// Helper functions
// ============================================================================

/// Check if a file is a video file based on extension.
#[allow(dead_code)]
pub fn is_video_file(filename: &str) -> bool {
    let video_extensions = [
        "mp4", "mkv", "avi", "mov", "wmv", "flv", "webm",
        "m4v", "mpeg", "mpg", "3gp", "rmvb", "rm",
    ];

    filename.rsplit('.').next().map(|ext| {
        video_extensions.iter().any(|&v| v.eq_ignore_ascii_case(ext))
    }).unwrap_or(false)
}

/// Delete a character from the current unified rename input field.
fn delete_unified_char(state: &mut App) {
    use crate::state::UnifiedFocus;
    match state.rename.unified.focus {
        UnifiedFocus::ShowName => { state.rename.unified.show_name.pop(); }
        UnifiedFocus::Season => { state.rename.unified.season.pop(); }
        UnifiedFocus::StartEpisode => { state.rename.unified.start_episode.pop(); }
        UnifiedFocus::Pattern => { state.rename.unified.pattern.pop(); }
    }
}

/// Delete a character from the current regex input field.
fn delete_regex_char(state: &mut App) {
    use crate::state::RegexFocus;
    match state.rename.regex.focus {
        RegexFocus::Find => { state.rename.regex.find.pop(); }
        RegexFocus::Replace => { state.rename.regex.replace.pop(); }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::NavMsg;
    use crate::state::Screen;

    #[test]
    fn test_update_function_signature() {
        // Verify update function exists with correct signature
        // fn update(state: &mut App, msg: Message)
        // This test verifies the function compiles and can be called
        let mut app = App::new();
        update(&mut app, NavMsg::SelectNext.into());
    }

    #[test]
    fn test_update_returns_unit() {
        // D-04: Test that update() returns () (in-place mutation)
        let mut app = App::new();
        let result: () = update(&mut app, NavMsg::SelectNext.into());
        let _ = result; // Verify it's unit type
    }

    #[test]
    fn test_update_delegates_to_handlers() {
        // Test that update() delegates to domain-specific handlers
        let mut app = App::new();
        assert_eq!(app.navigation.selected_index, 0);
        update(&mut app, NavMsg::SelectNext.into());
        // selected_index should change (if there are items)
    }

    #[test]
    fn test_update_navigation_select_next() {
        let mut app = App::new();
        // Add a file to the list so selection has an effect
        app.navigation.files.push(crate::api::types::FileItem {
            name: "test.mp4".to_string(),
            is_dir: false,
            size: Some(1000),
        });
        app.navigation.focus = crate::state::Focus::File;
        app.navigation.selected_index = 0;

        update(&mut app, NavMsg::SelectNext.into());
        // With one item, selection wraps back to 0
        assert_eq!(app.navigation.selected_index, 0);
    }

    #[test]
    fn test_update_auth_start_login() {
        let mut app = App::new();
        update(&mut app, AuthMsg::StartLogin.into());
        assert!(matches!(app.ui.screen, Screen::LoginScreen));
    }

    #[test]
    fn test_update_error_message_sets_popup() {
        // D-09, D-10: Test Message::Error sets ErrorPopup screen
        let mut app = App::new();
        let error_info = ErrorInfo::with_code(
            "API Error".to_string(),
            Some("NetworkError".to_string()),
            Some(500),
        );
        update(&mut app, Message::Error(error_info.clone()));
        assert!(matches!(app.ui.screen, Screen::ErrorPopup { .. }));
        if let Screen::ErrorPopup { error, .. } = &app.ui.screen {
            assert_eq!(error.message, error_info.message);
        }
    }

    #[test]
    fn test_update_ui_clear_error() {
        let mut app = App::new();
        let error_info = ErrorInfo::new("Test".to_string());
        app.ui.screen = Screen::ErrorPopup {
            error: error_info,
            previous_screen: Box::new(Screen::Normal),
        };
        update(&mut app, UiMsg::ClearError.into());
        assert!(matches!(app.ui.screen, Screen::Normal));
    }

    #[test]
    fn test_update_ui_set_screen() {
        let mut app = App::new();
        update(&mut app, UiMsg::SetScreen(Screen::LoginScreen).into());
        assert!(matches!(app.ui.screen, Screen::LoginScreen));
    }
}
