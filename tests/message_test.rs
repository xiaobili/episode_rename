// tests/message_test.rs
//! Tests for Message enum and domain message types.
//! These tests validate message creation and variant handling.

use openlist_tui::message::{AsyncMsg, AuthMsg, ErrorMsg, Message, NavMsg, RenameMsg, UiMsg};
use openlist_tui::state::ErrorInfo;
use openlist_tui::state::RenameMode;
use openlist_tui::state::Screen;

#[test]
fn test_nav_select_next_message() {
    // Test that NavMsg::SelectNext can be created and converted to Message
    let msg: Message = NavMsg::SelectNext.into();
    assert!(matches!(msg, Message::Navigation(NavMsg::SelectNext)));
}

#[test]
fn test_nav_select_previous_message() {
    // Test that NavMsg::SelectPrevious can be created and converted to Message
    let msg: Message = NavMsg::SelectPrevious.into();
    assert!(matches!(msg, Message::Navigation(NavMsg::SelectPrevious)));
}

#[test]
fn test_auth_start_login_message() {
    // Test that AuthMsg::StartLogin sets login screen state
    let msg: Message = AuthMsg::StartLogin.into();
    assert!(matches!(msg, Message::Auth(AuthMsg::StartLogin)));
}

#[test]
fn test_auth_input_username_message() {
    // Test AuthMsg::InputUsername('a') message creation
    let msg: Message = AuthMsg::InputUsername('a').into();
    assert!(matches!(msg, Message::Auth(AuthMsg::InputUsername('a'))));
}

#[test]
fn test_rename_open_popup_message() {
    // Test RenameMsg::OpenPopup message creation
    let msg: Message = RenameMsg::OpenPopup.into();
    assert!(matches!(msg, Message::Rename(RenameMsg::OpenPopup)));
}

#[test]
fn test_ui_set_screen_message() {
    // Test UiMsg::SetScreen(Screen::LoginScreen) message creation
    let msg: Message = UiMsg::SetScreen(Screen::LoginScreen).into();
    assert!(matches!(
        msg,
        Message::Ui(UiMsg::SetScreen(Screen::LoginScreen))
    ));
}

#[test]
fn test_async_login_result_message() {
    // Test AsyncMsg::LoginResult(Ok("token".to_string())) message creation
    let msg: Message = AsyncMsg::LoginResult(Ok("token".to_string())).into();
    assert!(matches!(msg, Message::Async(AsyncMsg::LoginResult(Ok(ref s))) if s == "token"));
}

#[test]
fn test_error_message_variant() {
    // D-09: Test Message::Error(ErrorInfo) variant for centralized error handling
    let error_info = ErrorInfo::new("Test error".to_string());
    let msg = Message::Error(error_info.clone());
    assert!(matches!(msg, Message::Error(ref e) if e.message == "Test error"));
}

#[test]
fn test_from_navmsg_to_message() {
    // Test that From<NavMsg> for Message allows .into() conversion
    let nav_msg = NavMsg::SelectNext;
    let msg: Message = nav_msg.into();
    assert!(matches!(msg, Message::Navigation(_)));
}

#[test]
fn test_from_authmsg_to_message() {
    // Test that From<AuthMsg> for Message allows .into() conversion
    let auth_msg = AuthMsg::SubmitLogin;
    let msg: Message = auth_msg.into();
    assert!(matches!(msg, Message::Auth(_)));
}

#[test]
fn test_from_rename_to_message() {
    // Test that From<RenameMsg> for Message allows .into() conversion
    let rename_msg = RenameMsg::SelectMode(RenameMode::Smart);
    let msg: Message = rename_msg.into();
    assert!(matches!(msg, Message::Rename(_)));
}

#[test]
fn test_from_uimsg_to_message() {
    // Test that From<UiMsg> for Message allows .into() conversion
    let ui_msg = UiMsg::StartLoading("Loading...".to_string());
    let msg: Message = ui_msg.into();
    assert!(matches!(msg, Message::Ui(_)));
}

#[test]
fn test_from_asyncmsg_to_message() {
    // Test that From<AsyncMsg> for Message allows .into() conversion
    let async_msg = AsyncMsg::BatchRenameResult(Ok(()));
    let msg: Message = async_msg.into();
    assert!(matches!(msg, Message::Async(_)));
}

#[test]
fn test_error_msg_show_error_conversion() {
    // Test ErrorMsg::ShowError conversion to Message::Error
    let error_info = ErrorInfo::with_code(
        "Not found".to_string(),
        Some("NotFound".to_string()),
        Some(404),
    );
    let msg: Message = ErrorMsg::ShowError(error_info).into();
    assert!(matches!(msg, Message::Error(ref e) if e.message == "Not found"));
}

#[test]
fn test_error_msg_dismiss_conversion() {
    // Test ErrorMsg::DismissError conversion to UiMsg::ClearError
    let msg: Message = ErrorMsg::DismissError.into();
    assert!(matches!(msg, Message::Ui(UiMsg::ClearError)));
}

#[test]
fn test_rename_mode_selection_message() {
    // Test RenameMsg with RenameMode variants
    let msg_smart: Message = RenameMsg::SelectMode(RenameMode::Smart).into();
    assert!(matches!(
        msg_smart,
        Message::Rename(RenameMsg::SelectMode(RenameMode::Smart))
    ));

    let msg_manual: Message = RenameMsg::SelectMode(RenameMode::Manual).into();
    assert!(matches!(
        msg_manual,
        Message::Rename(RenameMsg::SelectMode(RenameMode::Manual))
    ));

    let msg_unified: Message = RenameMsg::SelectMode(RenameMode::Unified).into();
    assert!(matches!(
        msg_unified,
        Message::Rename(RenameMsg::SelectMode(RenameMode::Unified))
    ));

    let msg_regex: Message = RenameMsg::SelectMode(RenameMode::Regex).into();
    assert!(matches!(
        msg_regex,
        Message::Rename(RenameMsg::SelectMode(RenameMode::Regex))
    ));
}

#[test]
fn test_nav_enter_directory_message() {
    // Test NavMsg::EnterDirectory with path
    let msg: Message = NavMsg::EnterDirectory("videos".to_string()).into();
    assert!(matches!(msg, Message::Navigation(NavMsg::EnterDirectory(ref s)) if s == "videos"));
}

#[test]
fn test_ui_progress_messages() {
    // Test UI progress-related messages
    let start_msg: Message = UiMsg::StartLoading("Loading directory...".to_string()).into();
    assert!(
        matches!(start_msg, Message::Ui(UiMsg::StartLoading(ref s)) if s == "Loading directory...")
    );

    let stop_msg: Message = UiMsg::StopLoading.into();
    assert!(matches!(stop_msg, Message::Ui(UiMsg::StopLoading)));

    let progress_msg: Message = UiMsg::UpdateProgress(5, 10).into();
    assert!(matches!(
        progress_msg,
        Message::Ui(UiMsg::UpdateProgress(5, 10))
    ));

    let spinner_msg: Message = UiMsg::AdvanceSpinner.into();
    assert!(matches!(spinner_msg, Message::Ui(UiMsg::AdvanceSpinner)));
}

#[test]
fn test_auth_login_flow_messages() {
    // Test full login flow message sequence
    let start: Message = AuthMsg::StartLogin.into();
    assert!(matches!(start, Message::Auth(AuthMsg::StartLogin)));

    let input_user: Message = AuthMsg::InputUsername('a').into();
    assert!(matches!(
        input_user,
        Message::Auth(AuthMsg::InputUsername('a'))
    ));

    let toggle: Message = AuthMsg::ToggleLoginFocus.into();
    assert!(matches!(toggle, Message::Auth(AuthMsg::ToggleLoginFocus)));

    let input_pass: Message = AuthMsg::InputPassword('b').into();
    assert!(matches!(
        input_pass,
        Message::Auth(AuthMsg::InputPassword('b'))
    ));

    let submit: Message = AuthMsg::SubmitLogin.into();
    assert!(matches!(submit, Message::Auth(AuthMsg::SubmitLogin)));

    let cancel: Message = AuthMsg::CancelLogin.into();
    assert!(matches!(cancel, Message::Auth(AuthMsg::CancelLogin)));
}
