//! Async task handling module for spawning and processing background operations.
//!
//! Per D-13, D-14, all async task spawning is extracted from main.rs.

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{backend::Backend, Terminal};

use crate::api::types::RenameObject;
use crate::app::App;
use crate::message::{AuthMsg, Message, NavMsg, RenameMsg, UiMsg};
use crate::state::{ErrorInfo, Focus, RenameMode, Screen};
use crate::task::PendingTask;
use crate::update::update;

/// Start auto-login if token exists in config.
pub fn start_auto_login(app: &mut App) {
    let has_token = app
        .config
        .token
        .as_ref()
        .map(|t| !t.is_empty())
        .unwrap_or(false);
    if !has_token {
        return;
    }

    let base_url = app.config.base_url.clone();
    let token = app.config.token.clone().unwrap();
    let tx = app.async_state.task_channel.tx.clone();

    update(app, UiMsg::StartLoading("自动登录中...".to_string()).into());
    app.async_state.pending_task = PendingTask::Loading {
        id: 5,
        message: "自动登录中...".to_string(),
        spinner_frame: 0,
    };

    tokio::spawn(async move {
        let client = crate::api::client::OpenListClient::new(base_url, Some(token));
        let result = client.get_current_user().await;
        let _ = tx.send(crate::task::TaskResult::AutoLogin(5, result));
    });
}

/// Process pending rename flags and spawn async batch rename tasks.
///
/// This function checks for finished rename operations and spawns
/// the async API calls to execute the renames.
pub async fn process_pending_renames(app: &mut App) {
    // Handle smart rename pending
    if app.rename.smart.pending {
        let results = app.take_smart_rename_results();
        app.rename.smart.pending = false;

        if !results.is_empty() {
            let renames = collect_renames(&results);
            if !renames.is_empty() {
                spawn_batch_rename(app, renames, 4, "智能重命名...").await;
            } else {
                app.rename.smart.pending = false;
            }
        }
    }

    // Handle manual rename finished
    if app.rename.manual.finished {
        let results = app.take_manual_rename_results();
        app.rename.manual.finished = false;

        if !results.is_empty() {
            let renames = collect_renames(&results);
            if !renames.is_empty() {
                spawn_batch_rename(app, renames, 1, "正在批量重命名...").await;
            }
        }
    }

    // Handle unified rename finished
    if app.rename.unified.finished {
        let results = app.take_unified_rename_results();
        app.rename.unified.finished = false;

        if !results.is_empty() {
            let renames = collect_renames(&results);
            if !renames.is_empty() {
                spawn_batch_rename(app, renames, 2, "正在批量重命名...").await;
            }
        }
    }

    // Handle regex rename finished
    if app.rename.regex.finished {
        let results = app.take_regex_rename_results();
        app.rename.regex.finished = false;

        if !results.is_empty() {
            let renames = collect_renames(&results);
            if !renames.is_empty() {
                spawn_batch_rename(app, renames, 3, "正在批量重命名...").await;
            }
        }
    }
}

/// Collect confirmed renames from results.
fn collect_renames(results: &[(String, String, bool)]) -> Vec<RenameObject> {
    results
        .iter()
        .filter(|(_, _, confirmed)| *confirmed)
        .map(|(src_name, new_name, _)| RenameObject {
            src_name: src_name.clone(),
            new_name: new_name.clone(),
        })
        .collect()
}

/// Spawn a batch rename async task.
pub async fn spawn_batch_rename(app: &mut App, renames: Vec<RenameObject>, id: u32, message: &str) {
    app.async_state.pending_task = PendingTask::Loading {
        id,
        message: message.to_string(),
        spinner_frame: 0,
    };

    let tx = app.async_state.task_channel.tx.clone();
    let client = app.client.clone();
    let current_path = app.navigation.current_path.clone();

    tokio::spawn(async move {
        let result = client.batch_rename(&current_path, renames).await;
        let _ = tx.send(crate::task::TaskResult::BatchRename(id, result));
    });
}

/// Handle special keys that need async spawning or complex logic.
///
/// These keys don't map cleanly to simple messages because they require:
/// - Input validation before state change
/// - Async task spawning
/// - Complex multi-step logic
pub async fn handle_special_keys<B: Backend>(
    _terminal: &mut Terminal<B>,
    app: &mut App,
    key: crossterm::event::KeyEvent,
) -> Result<()> {
    // Only process Press events
    if key.kind != KeyEventKind::Press {
        return Ok(());
    }

    match app.ui.screen {
        // Login screen Enter - submit login
        Screen::LoginScreen => {
            if key.code == KeyCode::Enter && !app.async_state.pending_task.is_loading() {
                // Validate inputs
                if app.auth.username_input.is_empty() || app.auth.password_input.is_empty() {
                    let error = ErrorInfo::new("用户名和密码不能为空".to_string());
                    update(app, Message::Error(error));
                } else {
                    // Start loading and spawn login task
                    update(app, UiMsg::StartLoading("正在登录...".to_string()).into());
                    app.async_state.pending_task = PendingTask::Loading {
                        id: 0,
                        message: "正在登录...".to_string(),
                        spinner_frame: 0,
                    };

                    let username = app.auth.username_input.clone();
                    let password = app.auth.password_input.clone();
                    let base_url = app.config.base_url.clone();
                    let tx = app.async_state.task_channel.tx.clone();

                    tokio::spawn(async move {
                        let client = crate::api::client::OpenListClient::new(base_url, None);
                        let result = client.login(&username, &password).await;
                        let _ = tx.send(crate::task::TaskResult::Login(0, result));
                    });
                }
            }
            // Escape during loading - cancel
            if key.code == KeyCode::Esc && app.async_state.pending_task.is_loading() {
                app.async_state.pending_task = PendingTask::Idle;
                update(app, UiMsg::StopLoading.into());
                update(app, AuthMsg::CancelLogin.into());
            }
        }

        // Rename mode selection Enter - execute selected mode
        Screen::RenameModeSelection => {
            if key.code == KeyCode::Enter {
                let selected_mode = app.rename.mode_selection.selected_mode;
                // Close popup first
                update(app, RenameMsg::ClosePopup.into());
                // Start the selected mode
                match selected_mode {
                    RenameMode::Smart => update(app, RenameMsg::ExecuteSmartRename.into()),
                    RenameMode::Manual => update(app, RenameMsg::StartManualRename.into()),
                    RenameMode::Unified => update(app, RenameMsg::StartUnifiedMode.into()),
                    RenameMode::Regex => update(app, RenameMsg::StartRegexMode.into()),
                }
            }
        }

        // Unified rename Enter - validate and execute
        Screen::UnifiedRename => {
            if key.code == KeyCode::Enter {
                match crate::update::validate_unified_inputs(app) {
                    Ok(()) => {
                        crate::update::execute_unified_rename(app);
                    }
                    Err(e) => {
                        let error = ErrorInfo::new(e);
                        update(app, Message::Error(error));
                    }
                }
            }
        }

        // Regex rename Enter - handle preview/execute
        Screen::RegexRename => {
            if key.code == KeyCode::Enter {
                if app.has_regex_preview() {
                    // Execute the rename
                    crate::update::execute_regex_rename(app);
                }
                // If no preview, the message dispatch handles GenerateRegexPreview
            }
        }

        // Single rename Enter - execute async API call
        Screen::SingleRename => {
            if key.code == KeyCode::Enter {
                if let Some(target) = &app.rename.single.target {
                    let new_name = app.rename.single.input.clone();
                    let current_path = app.navigation.current_path.clone();

                    let full_path = if current_path == "/" {
                        format!("/{}", target.name)
                    } else {
                        format!("{}/{}", current_path, target.name)
                    };

                    match app.client.rename_single(&full_path, &new_name).await {
                        Ok(()) => {
                            update(app, RenameMsg::CancelSingleRename.into());
                            if let Err(e) = app.load_directory_contents().await {
                                app.handle_api_error(e);
                            }
                        }
                        Err(e) => {
                            update(app, Message::Error(ErrorInfo::new(e.to_string())));
                        }
                    }
                }
            }
        }

        // Folder rename Enter - execute async API call
        Screen::FolderRename => {
            if key.code == KeyCode::Enter {
                if let Some(target) = &app.rename.folder.target {
                    let new_name = app.rename.folder.input.trim().to_string();

                    // Validate before API call
                    let validation_error = crate::validate::validate_folder_name(
                        &new_name,
                        &app.navigation.directories,
                    );

                    if let Some(error) = validation_error {
                        app.rename.folder.validation_error = Some(error);
                    } else {
                        let current_path = app.navigation.current_path.clone();

                        let full_path = if current_path == "/" {
                            format!("/{}", target.name)
                        } else {
                            format!("{}/{}", current_path, target.name)
                        };

                        // Set loading animation before API call
                        app.async_state.pending_task = PendingTask::Loading {
                            id: 6, // Unique ID for folder rename
                            message: "正在重命名文件夹...".to_string(),
                            spinner_frame: 0,
                        };

                        match app.client.rename_single(&full_path, &new_name).await {
                            Ok(()) => {
                                // Reset loading state
                                app.async_state.pending_task = PendingTask::Idle;
                                // Per D-17: close popup and refresh silently
                                update(app, RenameMsg::CancelFolderRename.into());
                                if let Err(e) = app.load_directory_contents().await {
                                    app.handle_api_error(e);
                                }
                            }
                            Err(e) => {
                                // Reset loading state
                                app.async_state.pending_task = PendingTask::Idle;
                                // Per D-15: API errors shown via error popup
                                update(app, Message::Error(ErrorInfo::new(e.to_string())));
                            }
                        }
                    }
                }
            }
        }

        // Normal mode Enter - directory navigation
        Screen::Normal if !app.async_state.pending_task.is_loading() => {
            if key.code == KeyCode::Enter && app.navigation.focus == Focus::Directory {
                let current_path = app.navigation.current_path.clone();
                let selected_index = app.navigation.selected_index;

                // Check if parent directory option is selected
                if current_path != "/" && !current_path.is_empty() && selected_index == 0 {
                    // Go to parent directory
                    update(app, NavMsg::GoParent.into());
                    if let Err(e) = app.load_directory_contents().await {
                        app.handle_api_error(e);
                    }
                } else if !app.navigation.directories.is_empty() {
                    // Calculate the actual directory index
                    let dir_index = if current_path != "/" && !current_path.is_empty() {
                        selected_index.saturating_sub(1)
                    } else {
                        selected_index
                    };

                    if let Some(dir_name) = app
                        .navigation
                        .directories
                        .get(dir_index)
                        .map(|d| d.name.clone())
                    {
                        update(app, NavMsg::EnterDirectory(dir_name).into());
                        if let Err(e) = app.load_directory_contents().await {
                            app.handle_api_error(e);
                        }
                    }
                }
            }

            // Left/h - go to parent directory
            if (key.code == KeyCode::Left || key.code == KeyCode::Char('h'))
                && app.navigation.focus == Focus::Directory
            {
                update(app, NavMsg::GoParent.into());
                if let Err(e) = app.load_directory_contents().await {
                    app.handle_api_error(e);
                }
            }
        }

        _ => {}
    }

    Ok(())
}
