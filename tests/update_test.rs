// tests/update_test.rs
//! Tests for the update() function and state transitions.
//! These tests validate that messages produce correct state changes.

use openlist_tui::app::App;
use openlist_tui::message::{Message, NavMsg, AuthMsg, RenameMsg, UiMsg, AsyncMsg};
use openlist_tui::update::update;
use openlist_tui::state::{Screen, ErrorInfo};
use openlist_tui::api::types::FileItem;

fn create_test_app() -> App {
    App::new()
}

#[test]
fn test_update_navigation_select_next() {
    // Test: update(app, NavMsg::SelectNext.into()) increments selected_index
    let mut app = create_test_app();
    // Add files to the list so selection has an effect
    app.navigation.files.push(FileItem {
        name: "test1.mp4".to_string(),
        is_dir: false,
        size: Some(1000),
    });
    app.navigation.files.push(FileItem {
        name: "test2.mp4".to_string(),
        is_dir: false,
        size: Some(2000),
    });
    app.navigation.focus = openlist_tui::state::Focus::File;
    app.navigation.selected_index = 0;

    update(&mut app, NavMsg::SelectNext.into());
    assert_eq!(app.navigation.selected_index, 1);
}

#[test]
fn test_update_navigation_select_previous() {
    // Test: update(app, NavMsg::SelectPrevious.into()) decrements selected_index
    let mut app = create_test_app();
    app.navigation.files.push(FileItem {
        name: "test1.mp4".to_string(),
        is_dir: false,
        size: Some(1000),
    });
    app.navigation.files.push(FileItem {
        name: "test2.mp4".to_string(),
        is_dir: false,
        size: Some(2000),
    });
    app.navigation.focus = openlist_tui::state::Focus::File;
    app.navigation.selected_index = 1;

    update(&mut app, NavMsg::SelectPrevious.into());
    assert_eq!(app.navigation.selected_index, 0);
}

#[test]
fn test_update_auth_start_login() {
    // Test: update(app, AuthMsg::StartLogin.into()) sets screen to LoginScreen
    let mut app = create_test_app();
    update(&mut app, AuthMsg::StartLogin.into());
    assert!(matches!(app.ui.screen, Screen::LoginScreen));
}

#[test]
fn test_update_auth_cancel_login() {
    // Test: update(app, AuthMsg::CancelLogin.into()) clears login state
    let mut app = create_test_app();
    app.ui.screen = Screen::LoginScreen;
    app.auth.username_input = "testuser".to_string();
    app.auth.password_input = "testpass".to_string();

    update(&mut app, AuthMsg::CancelLogin.into());
    assert!(matches!(app.ui.screen, Screen::Normal));
    assert!(app.auth.username_input.is_empty());
    assert!(app.auth.password_input.is_empty());
}

#[test]
fn test_update_rename_open_popup() {
    // Test: update(app, RenameMsg::OpenPopup.into()) sets screen to RenameModeSelection
    let mut app = create_test_app();
    app.auth.is_authenticated = true; // Must be authenticated
    update(&mut app, RenameMsg::OpenPopup.into());
    assert!(matches!(app.ui.screen, Screen::RenameModeSelection));
}

#[test]
fn test_update_ui_clear_error() {
    // Test: update(app, UiMsg::ClearError.into()) restores previous screen
    let mut app = create_test_app();
    let error_info = ErrorInfo::new("Test".to_string());
    app.ui.screen = Screen::ErrorPopup {
        error: error_info,
        previous_screen: Box::new(Screen::Normal),
    };
    update(&mut app, UiMsg::ClearError.into());
    assert!(matches!(app.ui.screen, Screen::Normal));
}

#[test]
fn test_update_error_message_sets_popup() {
    // D-09, D-10: Test Message::Error sets ErrorPopup screen
    let mut app = create_test_app();
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
fn test_update_error_recovery_redirect_to_login() {
    // D-11: Test error recovery flows through update function
    // When token expired, ClearErrorAndRelogin should redirect to login
    let mut app = create_test_app();
    let error_info = ErrorInfo::new("Token expired".to_string());
    app.ui.screen = Screen::ErrorPopup {
        error: error_info,
        previous_screen: Box::new(Screen::Normal),
    };
    app.auth.is_token_expired = true;

    update(&mut app, UiMsg::ClearErrorAndRelogin.into());
    assert!(matches!(app.ui.screen, Screen::LoginScreen));
    assert!(!app.auth.is_token_expired);
}

#[test]
fn test_update_async_login_result_success() {
    // Test: AsyncMsg::LoginResult(Ok(token)) sets authenticated state
    let mut app = create_test_app();
    app.auth.username_input = "testuser".to_string();
    app.ui.screen = Screen::LoginScreen;

    update(&mut app, AsyncMsg::LoginResult(Ok("test-token".to_string())).into());

    assert!(app.auth.is_authenticated);
    assert_eq!(app.auth.current_user, Some("testuser".to_string()));
    assert!(matches!(app.ui.screen, Screen::Normal));
}

#[test]
fn test_update_async_login_result_failure() {
    // Test: AsyncMsg::LoginResult(Err(e)) shows error popup
    let mut app = create_test_app();
    app.auth.username_input = "testuser".to_string();
    app.ui.screen = Screen::LoginScreen;

    use openlist_tui::error::AppError;
    update(&mut app, AsyncMsg::LoginResult(Err(AppError::Auth("Invalid credentials".to_string()))).into());

    assert!(!app.auth.is_authenticated);
    assert!(matches!(app.ui.screen, Screen::ErrorPopup { .. }));
}

#[test]
fn test_update_preserves_invariant() {
    // Test: After any message, state invariants are preserved
    // - selected_index within bounds
    // - screen consistency
    // - loading state consistency
    let mut app = create_test_app();

    // Add files so selection has meaning
    app.navigation.files.push(FileItem {
        name: "test.mp4".to_string(),
        is_dir: false,
        size: Some(1000),
    });
    app.navigation.focus = openlist_tui::state::Focus::File;
    app.navigation.selected_index = 0;

    // Apply various messages
    update(&mut app, NavMsg::SelectNext.into());
    update(&mut app, NavMsg::SelectPrevious.into());
    update(&mut app, AuthMsg::StartLogin.into());
    update(&mut app, AuthMsg::CancelLogin.into());

    // Check invariants
    let total_files = app.navigation.files.len();
    assert!(
        app.navigation.selected_index < total_files || total_files == 0,
        "selected_index {} out of bounds for {} files",
        app.navigation.selected_index,
        total_files
    );

    // Loading state should be cleared (no pending async operations)
    assert!(app.ui.loading_message.is_none());
}

#[test]
fn test_update_navigation_toggle_focus() {
    // Test toggling focus between Directory and File lists
    let mut app = create_test_app();
    app.navigation.focus = openlist_tui::state::Focus::Directory;

    update(&mut app, NavMsg::ToggleFocus.into());
    assert!(matches!(app.navigation.focus, openlist_tui::state::Focus::File));

    update(&mut app, NavMsg::ToggleFocus.into());
    assert!(matches!(app.navigation.focus, openlist_tui::state::Focus::Directory));
}

#[test]
fn test_update_auth_input_username() {
    // Test username input handling
    let mut app = create_test_app();

    update(&mut app, AuthMsg::InputUsername('a').into());
    assert_eq!(app.auth.username_input, "a");

    update(&mut app, AuthMsg::InputUsername('b').into());
    assert_eq!(app.auth.username_input, "ab");
}

#[test]
fn test_update_auth_delete_username_char() {
    // Test username character deletion
    let mut app = create_test_app();
    app.auth.username_input = "test".to_string();

    update(&mut app, AuthMsg::DeleteUsernameChar.into());
    assert_eq!(app.auth.username_input, "tes");
}

#[test]
fn test_update_ui_set_screen() {
    // Test screen transition via UiMsg::SetScreen
    let mut app = create_test_app();

    update(&mut app, UiMsg::SetScreen(Screen::LoginScreen).into());
    assert!(matches!(app.ui.screen, Screen::LoginScreen));

    update(&mut app, UiMsg::SetScreen(Screen::Normal).into());
    assert!(matches!(app.ui.screen, Screen::Normal));
}

#[test]
fn test_update_ui_loading_state() {
    // Test loading state management
    let mut app = create_test_app();

    update(&mut app, UiMsg::StartLoading("Loading...".to_string()).into());
    assert_eq!(app.ui.loading_message, Some("Loading...".to_string()));

    update(&mut app, UiMsg::StopLoading.into());
    assert!(app.ui.loading_message.is_none());
}

#[test]
fn test_update_error_stops_loading() {
    // D-09: Error message should stop loading state
    let mut app = create_test_app();
    app.ui.loading_message = Some("Loading...".to_string());

    let error_info = ErrorInfo::new("Error occurred".to_string());
    update(&mut app, Message::Error(error_info));

    assert!(app.ui.loading_message.is_none());
    assert!(matches!(app.ui.screen, Screen::ErrorPopup { .. }));
}

#[test]
fn test_update_rename_mode_selection() {
    // Test rename mode selection
    let mut app = create_test_app();
    app.auth.is_authenticated = true;

    update(&mut app, RenameMsg::OpenPopup.into());
    assert!(matches!(app.ui.screen, Screen::RenameModeSelection));

    use openlist_tui::state::RenameMode;
    update(&mut app, RenameMsg::SelectMode(RenameMode::Manual).into());
    assert_eq!(app.rename.mode_selection.selected_mode, RenameMode::Manual);

    update(&mut app, RenameMsg::NextMode.into());
    assert_eq!(app.rename.mode_selection.selected_mode, RenameMode::Unified);

    update(&mut app, RenameMsg::PreviousMode.into());
    assert_eq!(app.rename.mode_selection.selected_mode, RenameMode::Manual);

    update(&mut app, RenameMsg::ClosePopup.into());
    assert!(matches!(app.ui.screen, Screen::Normal));
}

// ============================================================================
// Task 2: Additional update() function domain coverage tests
// ============================================================================

#[test]
fn test_update_rename_unified_input() {
    // Test: RenameMsg::InputUnifiedShowName updates unified.show_name
    let mut app = create_test_app();
    app.auth.is_authenticated = true;
    app.navigation.files.push(FileItem {
        name: "video1.mp4".to_string(),
        is_dir: false,
        size: Some(1000),
    });

    // Start unified mode
    update(&mut app, RenameMsg::StartUnifiedMode.into());
    assert!(matches!(app.ui.screen, Screen::UnifiedRename));

    // Input show name
    for c in "TestShow".chars() {
        update(&mut app, RenameMsg::InputUnifiedShowName(c).into());
    }
    assert_eq!(app.rename.unified.show_name, "TestShow");

    // Input season (default is "1", appending "2" makes it "12")
    update(&mut app, RenameMsg::InputUnifiedSeason('2').into());
    assert_eq!(app.rename.unified.season, "12");

    // Input start episode (default is "1", appending "5" makes it "15")
    update(&mut app, RenameMsg::InputUnifiedStartEpisode('5').into());
    assert_eq!(app.rename.unified.start_episode, "15");
}

#[test]
fn test_update_ui_start_stop_loading() {
    // Test: UiMsg::StartLoading sets loading_message, StopLoading clears it
    let mut app = create_test_app();

    // Start loading
    update(&mut app, UiMsg::StartLoading("Loading...".to_string()).into());
    assert_eq!(app.ui.loading_message, Some("Loading...".to_string()));
    assert!(app.ui.loading_progress.is_none());

    // Stop loading
    update(&mut app, UiMsg::StopLoading.into());
    assert!(app.ui.loading_message.is_none());
    assert!(app.ui.loading_progress.is_none());
}

#[test]
fn test_update_async_login_result_success_full() {
    // Test: AsyncMsg::LoginResult(Ok(token)) sets authenticated state
    let mut app = create_test_app();
    app.auth.username_input = "testuser".to_string();
    app.ui.screen = Screen::LoginScreen;

    update(&mut app, AsyncMsg::LoginResult(Ok("test-token".to_string())).into());

    // Per D-02: Verify authentication state
    assert!(app.auth.is_authenticated, "Should be authenticated after successful login");
    assert_eq!(app.auth.current_user, Some("testuser".to_string()));
    assert!(matches!(app.ui.screen, Screen::Normal), "Should return to Normal screen");

    // Verify token is saved
    assert_eq!(app.config.token, Some("test-token".to_string()));

    // Verify login inputs are cleared
    assert!(app.auth.username_input.is_empty());
    assert!(app.auth.password_input.is_empty());
}

#[test]
fn test_update_async_list_directory() {
    // Test: AsyncMsg::ListDirectoryResult populates navigation.directories and files
    let mut app = create_test_app();

    // Create test items - mix of directories and video files
    let items = vec![
        FileItem {
            name: "Movies".to_string(),
            is_dir: true,
            size: None,
        },
        FileItem {
            name: "TVShows".to_string(),
            is_dir: true,
            size: None,
        },
        FileItem {
            name: "video1.mp4".to_string(),
            is_dir: false,
            size: Some(1000),
        },
        FileItem {
            name: "video2.mkv".to_string(),
            is_dir: false,
            size: Some(2000),
        },
        FileItem {
            name: "readme.txt".to_string(), // Non-video file, should be filtered
            is_dir: false,
            size: Some(100),
        },
    ];

    update(&mut app, AsyncMsg::ListDirectoryResult(Ok(items)).into());

    // Verify directories contain only is_dir items
    assert_eq!(app.navigation.directories.len(), 2);
    assert!(app.navigation.directories.iter().all(|i| i.is_dir));

    // Verify files contain only video files (filtered by is_video_file)
    assert_eq!(app.navigation.files.len(), 2);
    assert!(app.navigation.files.iter().all(|i| !i.is_dir));
    assert!(app.navigation.files.iter().all(|i|
        i.name.ends_with(".mp4") || i.name.ends_with(".mkv")
    ));
}

#[test]
fn test_update_preserves_unrelated_state() {
    // Test: State changes in one domain don't affect unrelated state
    let mut app = create_test_app();

    // Set up some navigation state
    app.navigation.current_path = "/movies".to_string();
    app.navigation.selected_index = 3;

    // Apply auth message
    update(&mut app, AuthMsg::InputUsername('a').into());

    // Navigation state should be preserved
    assert_eq!(app.navigation.current_path, "/movies");
    assert_eq!(app.navigation.selected_index, 3);

    // Set up auth state
    app.auth.username_input = "testuser".to_string();
    app.auth.is_authenticated = true;

    // Apply navigation message
    app.navigation.files.push(FileItem {
        name: "test.mp4".to_string(),
        is_dir: false,
        size: Some(1000),
    });
    app.navigation.focus = openlist_tui::state::Focus::File;
    update(&mut app, NavMsg::SelectNext.into());

    // Auth state should be preserved
    assert_eq!(app.auth.username_input, "testuser");
    assert!(app.auth.is_authenticated);
}

#[test]
fn test_update_async_login_result_clears_inputs() {
    // Test: Successful login clears username and password inputs
    let mut app = create_test_app();
    app.auth.username_input = "testuser".to_string();
    app.auth.password_input = "testpass".to_string();
    app.ui.screen = Screen::LoginScreen;

    update(&mut app, AsyncMsg::LoginResult(Ok("token123".to_string())).into());

    assert!(app.auth.username_input.is_empty(), "Username should be cleared");
    assert!(app.auth.password_input.is_empty(), "Password should be cleared");
    assert!(matches!(app.ui.screen, Screen::Normal));
}

#[test]
fn test_update_rename_regex_input() {
    // Test: RenameMsg for regex mode updates regex state
    let mut app = create_test_app();
    app.auth.is_authenticated = true;
    app.navigation.files.push(FileItem {
        name: "video1.mp4".to_string(),
        is_dir: false,
        size: Some(1000),
    });

    // Start regex mode
    update(&mut app, RenameMsg::StartRegexMode.into());
    assert!(matches!(app.ui.screen, Screen::RegexRename));

    // Input regex find pattern
    for c in "video".chars() {
        update(&mut app, RenameMsg::InputRegexFind(c).into());
    }
    assert_eq!(app.rename.regex.find, "video");

    // Input regex replace pattern
    for c in "movie".chars() {
        update(&mut app, RenameMsg::InputRegexReplace(c).into());
    }
    assert_eq!(app.rename.regex.replace, "movie");
}
