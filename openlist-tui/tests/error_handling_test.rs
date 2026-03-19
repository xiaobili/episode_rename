use openlist_tui::app::App;
use openlist_tui::error::AppError;
use openlist_tui::config::Config;

#[test]
fn test_initial_error_state() {
    let app = App::new();
    assert!(!app.show_error_popup);
    assert_eq!(app.error_message, None);
    assert_eq!(app.error_type, None);
    assert_eq!(app.error_code, None);
    assert!(!app.is_token_expired);
    assert!(!app.auto_relogin_pending);
}

#[test]
fn test_handle_token_expired_error() {
    let mut app = App::new();
    app.is_authenticated = true;
    app.current_user = Some("test_user".to_string());

    // Simulate token expired error
    app.handle_api_error_from_app_error(AppError::TokenExpired);

    assert!(app.show_error_popup);
    assert!(app.is_token_expired);
    assert!(app.auto_relogin_pending);
    assert!(!app.is_authenticated);
    assert_eq!(app.error_message, Some("Token 已过期，请重新登录".to_string()));
    assert_eq!(app.error_type, Some("Token 过期".to_string()));
    assert_eq!(app.error_code, Some(401));
}

#[test]
fn test_handle_auth_error() {
    let mut app = App::new();

    // Simulate auth error
    app.handle_api_error_from_app_error(AppError::Auth("Invalid credentials".to_string()));

    assert!(app.show_error_popup);
    assert!(app.is_token_expired);
    assert!(app.error_message.unwrap().contains("认证失败"));
    assert_eq!(app.error_type, Some("认证错误".to_string()));
}

#[test]
fn test_handle_network_error() {
    let mut app = App::new();

    // Simulate network error
    app.handle_api_error_from_app_error(AppError::Network("connection refused".to_string()));

    assert!(app.show_error_popup);
    assert!(!app.is_token_expired);
    assert!(app.error_message.unwrap().contains("网络错误"));
    assert_eq!(app.error_type, Some("网络错误".to_string()));
}

#[test]
fn test_handle_not_found_error() {
    let mut app = App::new();

    // Simulate not found error
    app.handle_api_error_from_app_error(AppError::NotFound("/some/path".to_string()));

    assert!(app.show_error_popup);
    assert!(!app.is_token_expired);
    assert!(app.error_message.unwrap().contains("路径不存在"));
    assert_eq!(app.error_type, Some("资源不存在".to_string()));
}

#[test]
fn test_handle_api_error_generic() {
    let mut app = App::new();

    // Simulate generic API error
    app.handle_api_error_from_app_error(AppError::ApiError("Server error".to_string()));

    assert!(app.show_error_popup);
    assert!(!app.is_token_expired);
    assert!(app.error_message.unwrap().contains("API 错误"));
    assert_eq!(app.error_type, Some("API 错误".to_string()));
}

#[test]
fn test_clear_error() {
    let mut app = App::new();

    // Set up error state
    app.handle_api_error_from_app_error(AppError::ApiError("Test error".to_string()));
    assert!(app.show_error_popup);

    // Clear error
    app.clear_error();

    assert!(!app.show_error_popup);
    assert_eq!(app.error_message, None);
    assert_eq!(app.error_type, None);
    assert_eq!(app.error_code, None);
}

#[test]
fn test_clear_error_and_prepare_relogin() {
    let mut app = App::new();

    // Set up token expired state
    app.handle_api_error_from_app_error(AppError::TokenExpired);
    assert!(app.is_token_expired);
    assert!(app.auto_relogin_pending);

    // Clear and prepare for re-login
    app.clear_error_and_prepare_relogin();

    assert!(!app.show_error_popup);
    assert!(!app.is_token_expired);
    assert!(!app.auto_relogin_pending);
    assert!(app.show_login_screen);
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
    app.handle_api_error_from_app_error(AppError::ApiError("Test".to_string()));
    assert!(app.show_error_popup);

    app.show_error_popup = false;
    app.handle_api_error_from_app_error(AppError::TokenExpired);
    assert!(app.show_error_popup);
}

#[test]
fn test_token_expired_redirects_to_login() {
    let mut app = App::new();

    // Trigger token expired
    app.handle_api_error_from_app_error(AppError::TokenExpired);
    assert!(app.is_token_expired);

    // Simulate user pressing Enter to go to login
    app.clear_error_and_prepare_relogin();

    assert!(app.show_login_screen);
    assert!(!app.show_error_popup);
}

#[test]
fn test_error_state_preserved_across_config() {
    let config = Config::default();
    let app = App::with_config(config);

    // New app should have clean error state
    assert!(!app.show_error_popup);
    assert!(!app.is_token_expired);
    assert!(!app.auto_relogin_pending);
}

// Test error detection in different scenarios
#[test]
fn test_error_detection_scenarios() {
    let mut app = App::new();

    // Scenario 1: Network error during login
    app.handle_api_error_from_app_error(AppError::Network("timeout".to_string()));
    assert!(!app.is_token_expired);
    assert!(app.error_message.as_ref().unwrap().contains("网络错误"));
    app.clear_error();

    // Scenario 2: Token expired during API call
    app.handle_api_error_from_app_error(AppError::TokenExpired);
    assert!(app.is_token_expired);
    assert!(app.auto_relogin_pending);
    app.clear_error();

    // Scenario 3: Invalid credentials
    app.handle_api_error_from_app_error(AppError::Auth("Invalid password".to_string()));
    assert!(app.is_token_expired); // Auth errors also trigger re-login
    app.clear_error();
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
