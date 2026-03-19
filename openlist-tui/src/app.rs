use crate::api::client::OpenListClient;
use crate::api::types::FileItem;
use crate::config::Config;
use crate::task::{TaskChannel, PendingTask};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoginFocus {
    Username,
    Password,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Focus {
    Directory,
    File,
    Input,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppMode {
    Normal,
    RenameSelection,
    Input,
}

pub struct App {
    pub client: OpenListClient,
    pub is_authenticated: bool,
    pub current_user: Option<String>,
    pub current_path: String,
    pub path_history: Vec<String>,
    pub directories: Vec<FileItem>,
    pub files: Vec<FileItem>,
    pub selected_index: usize,
    pub focus: Focus,
    pub mode: AppMode,
    pub show_rename_popup: bool,
    pub show_error_popup: bool,
    pub error_message: Option<String>,
    pub config: Config,
    pub task_channel: TaskChannel,
    pub pending_task: PendingTask,
    pub window_width: u16,
    pub window_height: u16,
    // Login state
    pub username_input: String,
    pub password_input: String,
    pub show_login_screen: bool,
    pub is_logging_in: bool,
    pub login_focus: LoginFocus,
}

impl Default for App {
    fn default() -> Self {
        Self {
            client: OpenListClient::new("http://192.168.1.1:5244".into(), None),
            is_authenticated: false,
            current_user: None,
            current_path: "/".into(),
            path_history: vec![],
            directories: vec![],
            files: vec![],
            selected_index: 0,
            focus: Focus::Directory,
            mode: AppMode::Normal,
            show_rename_popup: false,
            show_error_popup: false,
            error_message: None,
            config: Config::default(),
            task_channel: TaskChannel::new(),
            pending_task: PendingTask::Idle,
            window_width: 80,
            window_height: 24,
            username_input: String::new(),
            password_input: String::new(),
            show_login_screen: false,
            is_logging_in: false,
            login_focus: LoginFocus::Username,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_config(config: Config) -> Self {
        let base_url = config.base_url.clone();
        let token = config.token.clone();
        Self {
            client: OpenListClient::new(base_url, token),
            config,
            ..Self::default()
        }
    }

    pub fn select_next(&mut self) {
        let total = match self.focus {
            Focus::Directory => self.directories.len(),
            Focus::File => self.files.len(),
            Focus::Input => 0,
        };
        if total > 0 {
            self.selected_index = (self.selected_index + 1) % total;
        }
    }

    pub fn select_previous(&mut self) {
        let total = match self.focus {
            Focus::Directory => self.directories.len(),
            Focus::File => self.files.len(),
            Focus::Input => 0,
        };
        if total > 0 {
            self.selected_index = if self.selected_index == 0 {
                total - 1
            } else {
                self.selected_index - 1
            };
        }
    }

    pub fn toggle_focus(&mut self) {
        self.focus = match self.focus {
            Focus::Directory => Focus::File,
            Focus::File => Focus::Directory,
            Focus::Input => Focus::Directory,
        };
    }

    // Login methods
    pub fn start_login(&mut self) {
        self.show_login_screen = true;
        self.login_focus = LoginFocus::Username;
    }

    pub fn clear_login(&mut self) {
        self.show_login_screen = false;
        self.is_logging_in = false;
        self.username_input.clear();
        self.password_input.clear();
        self.login_focus = LoginFocus::Username;
    }

    pub fn submit_login(&mut self) {
        // Actual login logic is handled in main.rs event loop
        // This method just sets the flag to indicate login attempt
        self.is_logging_in = true;
    }

    pub fn toggle_login_focus(&mut self) {
        self.login_focus = match self.login_focus {
            LoginFocus::Username => LoginFocus::Password,
            LoginFocus::Password => LoginFocus::Username,
        };
    }

    pub fn set_login_focus_password(&mut self) {
        self.login_focus = LoginFocus::Password;
    }

    pub fn append_to_username(&mut self, ch: char) {
        self.username_input.push(ch);
    }

    pub fn append_to_password(&mut self, ch: char) {
        self.password_input.push(ch);
    }

    pub fn delete_last_username_char(&mut self) {
        self.username_input.pop();
    }

    pub fn delete_last_password_char(&mut self) {
        self.password_input.pop();
    }

    // Navigation methods
    pub fn enter_directory(&mut self, dir_name: &str) {
        // Save current path to history
        self.path_history.push(self.current_path.clone());

        // Build new path
        if self.current_path == "/" {
            self.current_path = format!("/{}", dir_name);
        } else {
            self.current_path = format!("{}/{}", self.current_path, dir_name);
        }

        // Reset selection when entering directory
        self.selected_index = 0;
        self.focus = Focus::Directory;
    }

    pub fn go_parent(&mut self) {
        if self.current_path == "/" || self.current_path.is_empty() {
            return;
        }

        // Save current path to history before navigating
        self.path_history.push(self.current_path.clone());

        // Remove last component from path
        // Split by '/' and filter out empty strings to get actual path components
        let parts: Vec<&str> = self.current_path.split('/').filter(|s| !s.is_empty()).collect();

        if parts.len() <= 1 {
            self.current_path = "/".to_string();
        } else {
            // Rebuild path without the last component
            self.current_path = format!("/{}", parts[..parts.len() - 1].join("/"));
        }

        // Reset selection when navigating
        self.selected_index = 0;
        self.focus = Focus::Directory;
    }

    pub async fn load_directory_contents(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let items = self.client.list_directory(&self.current_path).await?;

        // Separate directories and files
        self.directories.clear();
        self.files.clear();

        for item in items {
            if item.is_dir {
                self.directories.push(item);
            } else {
                // Filter to show only video files
                if is_video_file(&item.name) {
                    self.files.push(item);
                }
            }
        }

        Ok(())
    }
}

// Helper function to check if a file is a video file
fn is_video_file(filename: &str) -> bool {
    let video_extensions = [
        "mp4", "mkv", "avi", "mov", "wmv", "flv", "webm",
        "m4v", "mpeg", "mpg", "3gp", "rmvb", "rm",
    ];

    filename.rsplit('.').next().map(|ext| {
        video_extensions.iter().any(|&v| v.eq_ignore_ascii_case(ext))
    }).unwrap_or(false)
}
