#[derive(Debug, Clone, PartialEq)]
pub struct ErrorInfo {
    pub message: String,
    pub error_type: Option<String>,
    pub error_code: Option<i32>,
}

impl ErrorInfo {
    pub fn new(message: String) -> Self {
        Self {
            message,
            error_type: None,
            error_code: None,
        }
    }

    #[allow(dead_code)]
    pub fn with_type(message: String, error_type: String) -> Self {
        Self {
            message,
            error_type: Some(error_type),
            error_code: None,
        }
    }

    pub fn with_code(message: String, error_type: Option<String>, error_code: Option<i32>) -> Self {
        Self {
            message,
            error_type,
            error_code,
        }
    }
}

impl Default for ErrorInfo {
    fn default() -> Self {
        Self {
            message: String::new(),
            error_type: None,
            error_code: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Normal,
    LoginScreen,
    RenameModeSelection,
    ManualRename,
    UnifiedRename,
    RegexRename,
    SingleRename,
    ErrorPopup {
        error: ErrorInfo,
        previous_screen: Box<Screen>,
    },
}

impl Default for Screen {
    fn default() -> Self {
        Screen::Normal
    }
}

impl Screen {
    #[allow(dead_code)]
    pub fn is_input_mode(&self) -> bool {
        matches!(
            self,
            Screen::LoginScreen
                | Screen::ManualRename
                | Screen::UnifiedRename
                | Screen::RegexRename
                | Screen::SingleRename
        )
    }
}

#[allow(dead_code)]
pub struct UIState {
    pub screen: Screen,
    pub window_width: u16,
    pub window_height: u16,
    pub loading_message: Option<String>,
    pub loading_progress: Option<(usize, usize)>,
    pub loading_spinner_frame: usize,
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            screen: Screen::Normal,
            window_width: 80,
            window_height: 24,
            loading_message: None,
            loading_progress: None,
            loading_spinner_frame: 0,
        }
    }
}
