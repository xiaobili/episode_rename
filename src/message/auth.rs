//! Authentication messages for login/logout operations.
//!
//! These messages handle all state mutations related to user authentication,
//! including the login form and session management.

/// Authentication messages for login/logout operations.
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum AuthMsg {
    /// Start login process (l key in normal mode)
    StartLogin,

    /// Submit login credentials (Enter in login screen)
    SubmitLogin,

    /// Cancel login and return to previous screen (Esc in login screen)
    CancelLogin,

    /// Toggle focus between username and password fields (Tab in login screen)
    ToggleLoginFocus,

    /// Append a character to the username input
    InputUsername(char),

    /// Append a character to the password input
    InputPassword(char),

    /// Delete the last character from the username input
    DeleteUsernameChar,

    /// Delete the last character from the password input
    DeletePasswordChar,

    /// Log out the current user
    Logout,
}
