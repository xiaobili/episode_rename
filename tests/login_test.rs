use openlist_tui::app::{App, LoginFocus, Screen};
use openlist_tui::config::Config;
use openlist_tui::update::*;

#[test]
fn test_initial_login_state() {
    let app = App::new();
    assert!(!matches!(app.ui.screen, Screen::LoginScreen));
    assert_eq!(app.auth.username_input, "");
    assert_eq!(app.auth.password_input, "");
    assert_eq!(app.auth.login_focus, LoginFocus::Username);
}

#[test]
fn test_start_login_shows_screen() {
    let mut app = App::new();
    start_login(&mut app);
    assert!(matches!(app.ui.screen, Screen::LoginScreen));
    assert_eq!(app.auth.login_focus, LoginFocus::Username);
}

#[test]
fn test_clear_login_hides_screen() {
    let mut app = App::new();
    start_login(&mut app);
    assert!(matches!(app.ui.screen, Screen::LoginScreen));
    clear_login(&mut app);
    assert!(!matches!(app.ui.screen, Screen::LoginScreen));
}

#[test]
fn test_clear_login_resets_inputs() {
    let mut app = App::new();
    start_login(&mut app);
    app.auth.username_input = "test_user".to_string();
    app.auth.password_input = "secret".to_string();
    clear_login(&mut app);
    assert_eq!(app.auth.username_input, "");
    assert_eq!(app.auth.password_input, "");
}

#[test]
fn test_username_input_append() {
    let mut app = App::new();
    start_login(&mut app);
    append_to_username(&mut app, 'j');
    append_to_username(&mut app, 'o');
    append_to_username(&mut app, 'h');
    append_to_username(&mut app, 'n');
    assert_eq!(app.auth.username_input, "john");
}

#[test]
fn test_password_input_append() {
    let mut app = App::new();
    start_login(&mut app);
    append_to_password(&mut app, 'p');
    append_to_password(&mut app, 'a');
    append_to_password(&mut app, 's');
    append_to_password(&mut app, 's');
    assert_eq!(app.auth.password_input, "pass");
}

#[test]
fn test_delete_from_username() {
    let mut app = App::new();
    start_login(&mut app);
    app.auth.username_input = "john".to_string();
    delete_last_username_char(&mut app);
    assert_eq!(app.auth.username_input, "joh");
    delete_last_username_char(&mut app);
    assert_eq!(app.auth.username_input, "jo");
}

#[test]
fn test_delete_from_password() {
    let mut app = App::new();
    start_login(&mut app);
    app.auth.password_input = "pass".to_string();
    delete_last_password_char(&mut app);
    assert_eq!(app.auth.password_input, "pas");
}

#[test]
fn test_delete_empty_username() {
    let mut app = App::new();
    start_login(&mut app);
    delete_last_username_char(&mut app);
    assert_eq!(app.auth.username_input, "");
}

#[test]
fn test_delete_empty_password() {
    let mut app = App::new();
    start_login(&mut app);
    delete_last_password_char(&mut app);
    assert_eq!(app.auth.password_input, "");
}

#[test]
fn test_with_config_login_state() {
    let config = Config::default();
    let app = App::with_config(config);
    assert!(!matches!(app.ui.screen, Screen::LoginScreen));
}

#[test]
fn test_toggle_login_focus() {
    let mut app = App::new();
    start_login(&mut app);
    assert_eq!(app.auth.login_focus, LoginFocus::Username);
    toggle_login_focus(&mut app);
    assert_eq!(app.auth.login_focus, LoginFocus::Password);
    toggle_login_focus(&mut app);
    assert_eq!(app.auth.login_focus, LoginFocus::Username);
}

#[test]
fn test_clear_login_resets_focus() {
    let mut app = App::new();
    start_login(&mut app);
    toggle_login_focus(&mut app);
    assert_eq!(app.auth.login_focus, LoginFocus::Password);
    clear_login(&mut app);
    assert_eq!(app.auth.login_focus, LoginFocus::Username);
}
