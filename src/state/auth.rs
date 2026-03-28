#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoginFocus {
    Username,
    Password,
}

pub struct AuthState {
    pub is_authenticated: bool,
    pub current_user: Option<String>,
    pub is_token_expired: bool,
    pub auto_relogin_pending: bool,
    pub username_input: String,
    pub password_input: String,
    pub login_focus: LoginFocus,
}

impl Default for AuthState {
    fn default() -> Self {
        Self {
            is_authenticated: false,
            current_user: None,
            is_token_expired: false,
            auto_relogin_pending: false,
            username_input: String::new(),
            password_input: String::new(),
            login_focus: LoginFocus::Username,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_state_default() {
        let auth = AuthState::default();
        assert!(!auth.is_authenticated);
        assert!(auth.username_input.is_empty());
        assert!(auth.password_input.is_empty());
    }

    #[test]
    fn test_auth_state_fields_accessible() {
        let mut auth = AuthState::default();
        auth.username_input = "admin".to_string();
        auth.is_authenticated = true;
        assert_eq!(auth.username_input, "admin");
        assert!(auth.is_authenticated);
    }
}
