use openlist_tui::app::{App, Screen};
use openlist_tui::state::ErrorInfo;
use openlist_tui::error::AppError;
use openlist_tui::config::Config;
use openlist_tui::update::*;

#[test]
fn test_initial_error_state() {
    let app = App::new();
    assert!(!matches!(app.ui.screen, Screen::ErrorPopup { .. }));
    assert!(!app.auth.is_token_expired);
    assert!(!app.auth.auto_relogin_pending);
}

#[test]
fn test_handle_token_expired_error() {
    let mut app = App::new();
    app.auth.is_authenticated = true;
    app.auth.current_user = Some("test_user".to_string());

    // Simulate token expired error
    handle_api_error_from_app_error(&mut app, AppError::TokenExpired);

    assert!(matches!(app.ui.screen, Screen::ErrorPopup { .. }));
    assert!(app.auth.is_token_expired);
    assert!(app.auth.auto_relogin_pending);
    assert!(!app.auth.is_authenticated);
}

#[test]
fn test_handle_auth_error() {
    let mut app = App::new();

    // Simulate auth error
    handle_api_error_from_app_error(&mut app, AppError::Auth("Invalid credentials".to_string()));

    assert!(matches!(app.ui.screen, Screen::ErrorPopup { .. }));
    // Note: Auth errors no longer set is_token_expired (see app.rs implementation)
    // Check error message contains "认证失败"
    if let Screen::ErrorPopup { error, .. } = &app.ui.screen {
        assert!(error.message.contains("认证失败"));
    } else {
        panic!("Expected ErrorPopup screen");
    }
}

#[test]
fn test_handle_network_error() {
    let mut app = App::new();

    // Simulate network error
    handle_api_error_from_app_error(&mut app, AppError::Network("connection refused".to_string()));

    assert!(matches!(app.ui.screen, Screen::ErrorPopup { .. }));
    assert!(!app.auth.is_token_expired);
    if let Screen::ErrorPopup { error, .. } = &app.ui.screen {
        assert!(error.message.contains("网络错误"));
    } else {
        panic!("Expected ErrorPopup screen");
    }
}

#[test]
fn test_handle_not_found_error() {
    let mut app = App::new();

    // Simulate not found error
    handle_api_error_from_app_error(&mut app, AppError::NotFound("/some/path".to_string()));

    assert!(matches!(app.ui.screen, Screen::ErrorPopup { .. }));
    assert!(!app.auth.is_token_expired);
    if let Screen::ErrorPopup { error, .. } = &app.ui.screen {
        assert!(error.message.contains("路径不存在"));
    } else {
        panic!("Expected ErrorPopup screen");
    }
}

#[test]
fn test_handle_api_error_generic() {
    let mut app = App::new();

    // Simulate generic API error
    handle_api_error_from_app_error(&mut app, AppError::ApiError("Server error".to_string()));

    assert!(matches!(app.ui.screen, Screen::ErrorPopup { .. }));
    assert!(!app.auth.is_token_expired);
    if let Screen::ErrorPopup { error, .. } = &app.ui.screen {
        assert!(error.message.contains("API 错误"));
    } else {
        panic!("Expected ErrorPopup screen");
    }
}

#[test]
fn test_clear_error() {
    let mut app = App::new();

    // Set up error state
    handle_api_error_from_app_error(&mut app, AppError::ApiError("Test error".to_string()));
    assert!(matches!(app.ui.screen, Screen::ErrorPopup { .. }));

    // Clear error
    clear_error(&mut app);

    assert!(!matches!(app.ui.screen, Screen::ErrorPopup { .. }));
}

#[test]
fn test_clear_error_and_prepare_relogin() {
    let mut app = App::new();

    // Set up token expired state
    handle_api_error_from_app_error(&mut app, AppError::TokenExpired);
    assert!(app.auth.is_token_expired);
    assert!(app.auth.auto_relogin_pending);

    // Clear and prepare for re-login
    clear_error_and_prepare_relogin(&mut app);

    assert!(!matches!(app.ui.screen, Screen::ErrorPopup { .. }));
    assert!(!app.auth.is_token_expired);
    assert!(!app.auth.auto_relogin_pending);
    assert!(matches!(app.ui.screen, Screen::LoginScreen));
}

#[test]
fn test_app_error_is_unauthorized() {
    assert!(AppError::TokenExpired.is_unauthorized());
    assert!(!AppError::Auth("test".to_string()).is_unauthorized());
    assert!(!AppError::Network("connection refused".to_string()).is_unauthorized());
}

#[test]
fn test_app_error_is_network_error() {
    assert!(AppError::Network("connection refused".to_string()).is_network_error());
    assert!(!AppError::TokenExpired.is_network_error());
    assert!(!AppError::Auth("test".to_string()).is_network_error());
}

#[test]
fn test_app_error_error_type() {
    assert_eq!(AppError::Network("connection refused".to_string()).error_type(), "网络错误");
    assert_eq!(AppError::Auth("test".to_string()).error_type(), "认证错误");
    assert_eq!(AppError::TokenExpired.error_type(), "Token 过期");
    assert_eq!(AppError::NotFound("path".to_string()).error_type(), "资源不存在");
    assert_eq!(AppError::ApiError("error".to_string()).error_type(), "API 错误");
}

#[test]
fn test_app_error_error_code() {
    assert_eq!(AppError::TokenExpired.error_code(), Some(401));
    assert_eq!(AppError::NotFound("path".to_string()).error_code(), Some(404));
    assert_eq!(AppError::Auth("test".to_string()).error_code(), None);
    assert_eq!(AppError::ApiError("error".to_string()).error_code(), None);
}

#[test]
fn test_error_popup_shows_on_api_error() {
    let mut app = App::new();

    // Any API error should show error popup
    handle_api_error_from_app_error(&mut app, AppError::ApiError("Test".to_string()));
    assert!(matches!(app.ui.screen, Screen::ErrorPopup { .. }));

    app.ui.screen = Screen::Normal;
    handle_api_error_from_app_error(&mut app, AppError::TokenExpired);
    assert!(matches!(app.ui.screen, Screen::ErrorPopup { .. }));
}

#[test]
fn test_token_expired_redirects_to_login() {
    let mut app = App::new();

    // Trigger token expired
    handle_api_error_from_app_error(&mut app, AppError::TokenExpired);
    assert!(app.auth.is_token_expired);

    // Simulate user pressing Enter to go to login
    clear_error_and_prepare_relogin(&mut app);

    assert!(matches!(app.ui.screen, Screen::LoginScreen));
    assert!(!matches!(app.ui.screen, Screen::ErrorPopup { .. }));
}

#[test]
fn test_error_state_preserved_across_config() {
    let config = Config::default();
    let app = App::with_config(config);

    // New app should have clean error state
    assert!(!matches!(app.ui.screen, Screen::ErrorPopup { .. }));
    assert!(!app.auth.is_token_expired);
    assert!(!app.auth.auto_relogin_pending);
}

// Test error detection in different scenarios
#[test]
fn test_error_detection_scenarios() {
    let mut app = App::new();

    // Scenario 1: Network error during login
    handle_api_error_from_app_error(&mut app, AppError::Network("timeout".to_string()));
    assert!(!app.auth.is_token_expired);
    if let Screen::ErrorPopup { error, .. } = &app.ui.screen {
        assert!(error.message.contains("网络错误"));
    }
    clear_error(&mut app);

    // Scenario 2: Token expired during API call
    handle_api_error_from_app_error(&mut app, AppError::TokenExpired);
    assert!(app.auth.is_token_expired);
    assert!(app.auth.auto_relogin_pending);
    clear_error(&mut app);

    // Scenario 3: Invalid credentials
    handle_api_error_from_app_error(&mut app, AppError::Auth("Invalid password".to_string()));
    // Auth errors do not set is_token_expired anymore
    clear_error(&mut app);
}

// Test from_boxed_error conversion
#[test]
fn test_from_boxed_error_token_expired() {
    let err: Box<dyn std::error::Error> = Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "401 Unauthorized"
    ));
    let app_err = AppError::from_boxed_error(err);
    assert!(matches!(app_err, AppError::TokenExpired));
}

#[test]
fn test_from_boxed_error_network() {
    let err: Box<dyn std::error::Error> = Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "connection timeout"
    ));
    let app_err = AppError::from_boxed_error(err);
    assert!(matches!(app_err, AppError::Network(_)));
}

#[test]
fn test_from_boxed_error_default() {
    let err: Box<dyn std::error::Error> = Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "some random error"
    ));
    let app_err = AppError::from_boxed_error(err);
    assert!(matches!(app_err, AppError::ApiError(_)));
}
