use openlist_tui::app::{App, LoginFocus};
use openlist_tui::config::Config;

#[test]
fn test_initial_login_state() {
    let app = App::new();
    assert!(!app.show_login_screen);
    assert!(!app.is_logging_in);
    assert_eq!(app.username_input, "");
    assert_eq!(app.password_input, "");
    assert_eq!(app.login_focus, LoginFocus::Username);
}

#[test]
fn test_start_login_shows_screen() {
    let mut app = App::new();
    app.start_login();
    assert!(app.show_login_screen);
    assert_eq!(app.login_focus, LoginFocus::Username);
}

#[test]
fn test_clear_login_hides_screen() {
    let mut app = App::new();
    app.start_login();
    assert!(app.show_login_screen);
    app.clear_login();
    assert!(!app.show_login_screen);
}

#[test]
fn test_clear_login_resets_inputs() {
    let mut app = App::new();
    app.start_login();
    app.username_input = "test_user".to_string();
    app.password_input = "secret".to_string();
    app.clear_login();
    assert_eq!(app.username_input, "");
    assert_eq!(app.password_input, "");
}

#[test]
fn test_username_input_append() {
    let mut app = App::new();
    app.start_login();
    app.append_to_username('j');
    app.append_to_username('o');
    app.append_to_username('h');
    app.append_to_username('n');
    assert_eq!(app.username_input, "john");
}

#[test]
fn test_password_input_append() {
    let mut app = App::new();
    app.start_login();
    app.append_to_password('p');
    app.append_to_password('a');
    app.append_to_password('s');
    app.append_to_password('s');
    assert_eq!(app.password_input, "pass");
}

#[test]
fn test_delete_from_username() {
    let mut app = App::new();
    app.start_login();
    app.username_input = "john".to_string();
    app.delete_last_username_char();
    assert_eq!(app.username_input, "joh");
    app.delete_last_username_char();
    assert_eq!(app.username_input, "jo");
}

#[test]
fn test_delete_from_password() {
    let mut app = App::new();
    app.start_login();
    app.password_input = "pass".to_string();
    app.delete_last_password_char();
    assert_eq!(app.password_input, "pas");
}

#[test]
fn test_delete_empty_username() {
    let mut app = App::new();
    app.start_login();
    app.delete_last_username_char();
    assert_eq!(app.username_input, "");
}

#[test]
fn test_delete_empty_password() {
    let mut app = App::new();
    app.start_login();
    app.delete_last_password_char();
    assert_eq!(app.password_input, "");
}

#[test]
fn test_with_config_login_state() {
    let config = Config::default();
    let app = App::with_config(config);
    assert!(!app.show_login_screen);
    assert!(!app.is_logging_in);
}

#[test]
fn test_toggle_login_focus() {
    let mut app = App::new();
    app.start_login();
    assert_eq!(app.login_focus, LoginFocus::Username);
    app.toggle_login_focus();
    assert_eq!(app.login_focus, LoginFocus::Password);
    app.toggle_login_focus();
    assert_eq!(app.login_focus, LoginFocus::Username);
}

#[test]
fn test_clear_login_resets_focus() {
    let mut app = App::new();
    app.start_login();
    app.toggle_login_focus();
    assert_eq!(app.login_focus, LoginFocus::Password);
    app.clear_login();
    assert_eq!(app.login_focus, LoginFocus::Username);
}

#[test]
fn test_submit_login_sets_logging_in() {
    let mut app = App::new();
    app.start_login();
    app.username_input = "user".to_string();
    app.password_input = "pass".to_string();
    app.submit_login();
    assert!(app.is_logging_in);
}
