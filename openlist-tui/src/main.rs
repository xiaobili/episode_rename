mod api;
mod app;
mod config;
mod error;
mod models;
mod task;
mod ui;
mod validate;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use crate::app::App;
use crate::config::Config;
use crate::app::Focus;
use crate::app::UnifiedFocus;
use crate::app::RegexFocus;
use crate::task::{TaskResult, PendingTask};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()?;
    let mut app = App::with_config(config);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal, &mut app).await;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    result
}

async fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        // Advance spinner frame for loading animation
        if app.pending_task.is_loading() {
            app.pending_task.advance_spinner();
        }

        terminal.draw(|f| ui::render::render(f, app))?;

        // Check for completed async tasks
        if let Ok(task_result) = app.task_channel.rx.try_recv() {
            match task_result {
                TaskResult::ListDirectory(_id, result) => {
                    match result {
                        Ok(_) => {
                            // Directory loaded successfully - will be handled by caller
                        }
                        Err(e) => {
                            app.handle_api_error_from_app_error(e);
                        }
                    }
                    // Stop loading state
                    app.pending_task = PendingTask::Idle;
                    app.stop_loading();
                }
                TaskResult::BatchRename(_id, result) => {
                    match result {
                        Ok(_) => {
                            // Batch rename successful - reload directory
                            if let Err(e) = app.load_directory_contents().await {
                                app.handle_api_error(e);
                            }
                        }
                        Err(e) => {
                            app.handle_api_error_from_app_error(e);
                        }
                    }
                    // Stop loading state
                    app.pending_task = PendingTask::Idle;
                    app.stop_loading();
                }
                TaskResult::Login(_id, result) => {
                    match result {
                        Ok(token) => {
                            // Login successful
                            app.is_authenticated = true;
                            app.current_user = Some(app.username_input.clone());
                            app.client = crate::api::client::OpenListClient::new(
                                app.config.base_url.clone(),
                                Some(token.clone())
                            );
                            // Save token to config
                            app.config.token = Some(token);
                            app.config.username = Some(app.username_input.clone());
                            let _ = app.config.save();
                            // Close login screen
                            app.show_login_screen = false;
                            app.clear_login();
                            // Load root directory contents
                            if let Err(e) = app.load_directory_contents().await {
                                app.handle_api_error(e);
                            }
                        }
                        Err(e) => {
                            app.handle_api_error_from_app_error(e);
                        }
                    }
                    app.is_logging_in = false;
                    app.pending_task = PendingTask::Idle;
                    app.stop_loading();
                }
            }
        }

        // Check if manual rename finished and execute batch rename
        if app.manual_rename_finished {
            // Take the results and clear the flag
            let results = app.take_manual_rename_results();
            app.manual_rename_finished = false;

            if !results.is_empty() {
                // Build rename objects for batch rename API
                use crate::api::types::RenameObject;
                let renames: Vec<RenameObject> = results.iter()
                    .filter(|(_, _, confirmed)| *confirmed)
                    .map(|(src_name, new_name, _)| RenameObject {
                        src_name: src_name.clone(),
                        new_name: new_name.clone(),
                    })
                    .collect();

                if !renames.is_empty() {
                    // Set loading state with progress
                    app.start_loading("正在批量重命名...".to_string());
                    app.update_progress(0, renames.len());
                    app.pending_task = PendingTask::Renaming {
                        id: 1,
                        total: renames.len(),
                        completed: 0,
                        message: "正在批量重命名...".to_string(),
                        spinner_frame: 0,
                    };

                    // Execute batch rename
                    match app.client.batch_rename(&app.current_path, renames).await {
                        Ok(_) => {
                            // Success - reload directory to show updated names
                            if let Err(e) = app.load_directory_contents().await {
                                app.handle_api_error(e);
                            }
                        }
                        Err(e) => {
                            // Error - use centralized error handler
                            app.handle_api_error_from_app_error(e);
                        }
                    }
                    // Stop loading
                    app.stop_loading();
                    app.pending_task = PendingTask::Idle;
                } else {
                    // No renames to execute, just reload directory
                    if let Err(e) = app.load_directory_contents().await {
                        app.handle_api_error(e);
                    }
                }
            } else {
                // No results (all skipped), just reload directory
                if let Err(e) = app.load_directory_contents().await {
                    app.handle_api_error(e);
                }
            }
        }

        // Check if unified rename finished and execute batch rename
        if app.unified_rename_finished {
            // Take the results and clear the flag
            let results = app.take_unified_rename_results();
            app.unified_rename_finished = false;

            if !results.is_empty() {
                // Build rename objects for batch rename API
                use crate::api::types::RenameObject;
                let renames: Vec<RenameObject> = results.iter()
                    .filter(|(_, _, confirmed)| *confirmed)
                    .map(|(src_name, new_name, _)| RenameObject {
                        src_name: src_name.clone(),
                        new_name: new_name.clone(),
                    })
                    .collect();

                if !renames.is_empty() {
                    // Set loading state with progress
                    app.start_loading("正在批量重命名...".to_string());
                    app.update_progress(0, renames.len());
                    app.pending_task = PendingTask::Renaming {
                        id: 2,
                        total: renames.len(),
                        completed: 0,
                        message: "正在批量重命名...".to_string(),
                        spinner_frame: 0,
                    };

                    // Execute batch rename
                    match app.client.batch_rename(&app.current_path, renames).await {
                        Ok(_) => {
                            // Success - reload directory to show updated names
                            if let Err(e) = app.load_directory_contents().await {
                                app.handle_api_error(e);
                            }
                        }
                        Err(e) => {
                            // Error - use centralized error handler
                            app.handle_api_error_from_app_error(e);
                        }
                    }
                    // Stop loading
                    app.stop_loading();
                    app.pending_task = PendingTask::Idle;
                } else {
                    // No renames to execute, just reload directory
                    if let Err(e) = app.load_directory_contents().await {
                        app.handle_api_error(e);
                    }
                }
            } else {
                // No results, just reload directory
                if let Err(e) = app.load_directory_contents().await {
                    app.handle_api_error(e);
                }
            }
        }

        // Check if regex rename finished and execute batch rename
        if app.regex_rename_finished {
            // Take the results and clear the flag
            let results = app.take_regex_rename_results();
            app.regex_rename_finished = false;

            if !results.is_empty() {
                // Build rename objects for batch rename API
                use crate::api::types::RenameObject;
                let renames: Vec<RenameObject> = results.iter()
                    .filter(|(_, _, confirmed)| *confirmed)
                    .map(|(src_name, new_name, _)| RenameObject {
                        src_name: src_name.clone(),
                        new_name: new_name.clone(),
                    })
                    .collect();

                if !renames.is_empty() {
                    // Set loading state with progress
                    app.start_loading("正在批量重命名...".to_string());
                    app.update_progress(0, renames.len());
                    app.pending_task = PendingTask::Renaming {
                        id: 3,
                        total: renames.len(),
                        completed: 0,
                        message: "正在批量重命名...".to_string(),
                        spinner_frame: 0,
                    };

                    // Execute batch rename
                    match app.client.batch_rename(&app.current_path, renames).await {
                        Ok(_) => {
                            // Success - reload directory to show updated names
                            if let Err(e) = app.load_directory_contents().await {
                                app.handle_api_error(e);
                            }
                        }
                        Err(e) => {
                            // Error - use centralized error handler
                            app.handle_api_error_from_app_error(e);
                        }
                    }
                    // Stop loading
                    app.stop_loading();
                    app.pending_task = PendingTask::Idle;
                } else {
                    // No renames to execute, just reload directory
                    if let Err(e) = app.load_directory_contents().await {
                        app.handle_api_error(e);
                    }
                }
            } else {
                // No results, just reload directory
                if let Err(e) = app.load_directory_contents().await {
                    app.handle_api_error(e);
                }
            }
        }

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                // Handle error popup - highest priority
                if app.show_error_popup {
                    match key.code {
                        KeyCode::Enter => {
                            if app.is_token_expired {
                                // Token expired - redirect to login
                                app.clear_error_and_prepare_relogin();
                            } else if app.error_message.as_ref().map_or(false, |m| m.contains("网络")) {
                                // Network error - retry last operation (for future implementation)
                                app.clear_error();
                            } else {
                                // Other errors - just close
                                app.clear_error();
                            }
                        }
                        KeyCode::Esc => {
                            app.clear_error();
                        }
                        _ => {}
                    }
                    continue;
                }

                // Handle login screen input
                if app.show_login_screen {
                    if app.pending_task.is_loading() {
                        // Wait for login to complete, only allow Esc to cancel
                        if key.code == KeyCode::Esc {
                            app.pending_task = PendingTask::Idle;
                            app.stop_loading();
                            app.is_logging_in = false;
                            app.clear_login();
                        }
                    } else {
                        match key.code {
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::Char('c')
                                if key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                            {
                                return Ok(())
                            }
                            KeyCode::Esc => {
                                app.clear_login();
                            }
                            KeyCode::Enter => {
                                // Submit login
                                if app.username_input.is_empty() || app.password_input.is_empty() {
                                    app.error_message = Some("用户名和密码不能为空".to_string());
                                    app.show_error_popup = true;
                                    app.show_login_screen = false;
                                } else {
                                    // Start loading state and perform async login
                                    app.is_logging_in = true;
                                    app.start_loading("正在登录...".to_string());
                                    app.pending_task = PendingTask::Loading {
                                        id: 0,
                                        message: "正在登录...".to_string(),
                                        spinner_frame: 0,
                                    };

                                    // Clone data for async task
                                    let username = app.username_input.clone();
                                    let password = app.password_input.clone();
                                    let base_url = app.config.base_url.clone();
                                    let tx = app.task_channel.tx.clone();

                                    // Spawn async task
                                    tokio::spawn(async move {
                                        let client = crate::api::client::OpenListClient::new(base_url, None);
                                        let result = client.login(&username, &password).await;
                                        let _ = tx.send(TaskResult::Login(0, result));
                                    });
                                }
                            }
                            KeyCode::Backspace => {
                                match app.login_focus {
                                    crate::app::LoginFocus::Username => {
                                        app.delete_last_username_char();
                                    }
                                    crate::app::LoginFocus::Password => {
                                        app.delete_last_password_char();
                                    }
                                }
                            }
                            KeyCode::Tab => {
                                // Toggle between username and password input
                                app.toggle_login_focus();
                            }
                            KeyCode::Char(c) => {
                                // Text input for username/password based on focus
                                match app.login_focus {
                                    crate::app::LoginFocus::Username => {
                                        if app.username_input.len() < 50 {
                                            app.append_to_username(c);
                                        }
                                    }
                                    crate::app::LoginFocus::Password => {
                                        if app.password_input.len() < 50 {
                                            app.append_to_password(c);
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    continue;
                }

                // Handle rename popup input
                if app.show_rename_mode_popup {
                    match key.code {
                        KeyCode::Esc => {
                            app.close_rename_popup();
                        }
                        KeyCode::Enter => {
                            // Handle different rename modes
                            match app.selected_rename_mode {
                                crate::app::RenameMode::Manual => {
                                    // Close mode popup and start manual rename
                                    app.close_rename_popup();
                                    app.start_manual_rename();
                                }
                                crate::app::RenameMode::Unified => {
                                    // Close mode popup and start unified naming
                                    app.close_rename_popup();
                                    app.start_unified_mode();
                                }
                                crate::app::RenameMode::Regex => {
                                    // Close mode popup and start regex rename
                                    app.close_rename_popup();
                                    app.start_regex_mode();
                                }
                                _ => {
                                    // For other modes (Smart), just close popup
                                    // Smart rename would execute here in future implementation
                                    app.close_rename_popup();
                                }
                            }
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            app.select_previous_rename_mode();
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            app.select_next_rename_mode();
                        }
                        _ => {}
                    }
                    continue;
                }

                // Handle manual rename popup input
                if app.show_manual_rename_popup {
                    match key.code {
                        KeyCode::Esc => {
                            // Cancel manual rename
                            app.cancel_manual_rename();
                        }
                        KeyCode::Enter => {
                            // Submit current filename and go to next
                            app.submit_manual_rename();
                        }
                        KeyCode::Char('s') | KeyCode::Char('S') => {
                            // Skip current file
                            app.skip_manual_rename();
                        }
                        KeyCode::Backspace => {
                            app.delete_last_manual_rename_char();
                        }
                        KeyCode::Char(c) => {
                            // Text input for new filename
                            if app.manual_rename_input.len() < 200 {
                                app.manual_rename_input.push(c);
                            }
                        }
                        _ => {}
                    }
                    continue;
                }

                // Handle unified naming popup input
                if app.show_unified_input {
                    match key.code {
                        KeyCode::Esc => {
                            // Cancel unified naming
                            app.cancel_unified();
                        }
                        KeyCode::Enter => {
                            // Validate inputs first
                            match app.validate_unified_inputs() {
                                Ok(_) => {
                                    // Execute unified rename
                                    app.execute_unified_rename();
                                }
                                Err(e) => {
                                    // Show validation error
                                    app.error_message = Some(e);
                                    app.show_error_popup = true;
                                    app.show_unified_input = false;
                                }
                            }
                        }
                        KeyCode::Tab => {
                            // Switch between input fields
                            app.toggle_unified_focus();
                        }
                        KeyCode::Backspace => {
                            // Delete last character from current field
                            match app.unified_focus {
                                UnifiedFocus::ShowName => {
                                    app.unified_show_name.pop();
                                }
                                UnifiedFocus::Season => {
                                    app.unified_season.pop();
                                }
                                UnifiedFocus::StartEpisode => {
                                    app.unified_start_episode.pop();
                                }
                                UnifiedFocus::Pattern => {
                                    app.unified_pattern.pop();
                                }
                            }
                            // Update preview after editing
                            app.generate_unified_preview();
                        }
                        KeyCode::Char(c) => {
                            // Text input for current field
                            match app.unified_focus {
                                UnifiedFocus::ShowName => {
                                    if app.unified_show_name.len() < 100 {
                                        app.unified_show_name.push(c);
                                        app.generate_unified_preview();
                                    }
                                }
                                UnifiedFocus::Season => {
                                    // Only allow digits
                                    if c.is_ascii_digit() && app.unified_season.len() < 3 {
                                        app.unified_season.push(c);
                                        app.generate_unified_preview();
                                    }
                                }
                                UnifiedFocus::StartEpisode => {
                                    // Only allow digits
                                    if c.is_ascii_digit() && app.unified_start_episode.len() < 4 {
                                        app.unified_start_episode.push(c);
                                        app.generate_unified_preview();
                                    }
                                }
                                UnifiedFocus::Pattern => {
                                    if app.unified_pattern.len() < 100 {
                                        app.unified_pattern.push(c);
                                        app.generate_unified_preview();
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                    continue;
                }

                // Handle regex rename popup input
                if app.show_regex_input {
                    match key.code {
                        KeyCode::Esc => {
                            // Cancel regex rename
                            app.cancel_regex();
                        }
                        KeyCode::Enter => {
                            if app.has_regex_preview() {
                                // Preview already generated - execute rename
                                app.execute_regex_rename();
                            } else {
                                // Generate preview (validates regex)
                                app.submit_regex();
                            }
                        }
                        KeyCode::Tab => {
                            // Switch between find/replace fields
                            app.toggle_regex_focus();
                        }
                        KeyCode::Backspace => {
                            // Delete last character from current field
                            match app.regex_focus {
                                RegexFocus::Find => {
                                    app.regex_find.pop();
                                }
                                RegexFocus::Replace => {
                                    app.regex_replace.pop();
                                }
                            }
                            // Clear preview when editing
                            app.regex_preview.clear();
                            app.regex_error = None;
                        }
                        KeyCode::Char(c) => {
                            // Text input for current field
                            match app.regex_focus {
                                RegexFocus::Find => {
                                    if app.regex_find.len() < 100 {
                                        app.regex_find.push(c);
                                        // Clear preview when editing
                                        app.regex_preview.clear();
                                        app.regex_error = None;
                                    }
                                }
                                RegexFocus::Replace => {
                                    if app.regex_replace.len() < 100 {
                                        app.regex_replace.push(c);
                                        // Clear preview when editing
                                        app.regex_preview.clear();
                                        app.regex_error = None;
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                    continue;
                }

                // Handle single file rename popup input
                if app.show_single_rename {
                    match key.code {
                        KeyCode::Esc => {
                            // Cancel single rename
                            app.cancel_single_rename();
                        }
                        KeyCode::Enter => {
                            // Submit single rename - execute immediately via API
                            if app.single_rename_target.is_some() {
                                let new_name = app.single_rename_input.clone();
                                let target = app.single_rename_target.clone();
                                let current_path = app.current_path.clone();

                                // Build full path for the file
                                let full_path = if current_path == "/" {
                                    format!("/{}", target.as_ref().unwrap().name)
                                } else {
                                    format!("{}/{}", current_path, target.as_ref().unwrap().name)
                                };

                                // Execute the rename API call
                                match app.client.rename_single(&full_path, &new_name).await {
                                    Ok(_) => {
                                        // Success - close dialog and reload directory
                                        app.cancel_single_rename();
                                        if let Err(e) = app.load_directory_contents().await {
                                            app.handle_api_error(e);
                                        }
                                    }
                                    Err(e) => {
                                        // Error - use centralized error handler
                                        app.handle_api_error_from_app_error(e);
                                        app.show_single_rename = false;
                                    }
                                }
                            }
                        }
                        KeyCode::Backspace => {
                            app.delete_last_single_rename_char();
                        }
                        KeyCode::Char(c) => {
                            // Text input for new filename
                            if app.single_rename_input.len() < 200 {
                                app.single_rename_input.push(c);
                            }
                        }
                        _ => {}
                    }
                    continue;
                }

                // Normal mode key bindings
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('c')
                        if key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                    {
                        return Ok(())
                    }
                    KeyCode::Char('l') => {
                        // Open login screen
                        app.start_login();
                    }
                    KeyCode::Char('r') => {
                        // Open rename popup
                        app.open_rename_popup();
                    }
                    KeyCode::Char('N') => {
                        // Single file rename (Shift+n)
                        app.start_single_rename();
                    }
                    KeyCode::Up | KeyCode::Char('k') => app.select_previous(),
                    KeyCode::Down | KeyCode::Char('j') => app.select_next(),
                    KeyCode::Tab => app.toggle_focus(),
                    KeyCode::Enter => {
                        // Enter directory on Enter key
                        if app.focus == Focus::Directory {
                            // Check if parent directory option is selected (index 0 when not at root)
                            if app.current_path != "/" && !app.current_path.is_empty() && app.selected_index == 0 {
                                // Go to parent directory
                                app.go_parent();
                                // Load parent directory contents
                                if let Err(e) = app.load_directory_contents().await {
                                    app.handle_api_error(e);
                                }
                            } else if !app.directories.is_empty() {
                                // Calculate the actual directory index (accounting for parent option)
                                let dir_index = if app.current_path != "/" && !app.current_path.is_empty() {
                                    app.selected_index.saturating_sub(1)
                                } else {
                                    app.selected_index
                                };
                                // Clone the directory name to avoid borrow checker issues
                                let dir_name = app.directories.get(dir_index).map(|d| d.name.clone());
                                if let Some(dir_name) = dir_name {
                                    app.enter_directory(&dir_name);
                                    // Load directory contents
                                    if let Err(e) = app.load_directory_contents().await {
                                        app.handle_api_error(e);
                                    }
                                }
                            }
                        }
                    }
                    KeyCode::Left | KeyCode::Char('h') => {
                        // Go to parent directory on Left arrow or 'h' key
                        app.go_parent();
                        // Load parent directory contents
                        if let Err(e) = app.load_directory_contents().await {
                            app.handle_api_error(e);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
