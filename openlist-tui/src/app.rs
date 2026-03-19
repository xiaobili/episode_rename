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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RenameMode {
    Smart,      // 智能重命名
    Manual,     // 手动重命名
    Unified,    // 统一命名
    Regex,      // 正则替换
}

impl RenameMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            RenameMode::Smart => "智能重命名",
            RenameMode::Manual => "手动重命名",
            RenameMode::Unified => "统一命名",
            RenameMode::Regex => "正则替换",
        }
    }

    pub fn all() -> [RenameMode; 4] {
        [RenameMode::Smart, RenameMode::Manual, RenameMode::Unified, RenameMode::Regex]
    }
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
    // Rename state
    pub show_rename_mode_popup: bool,
    pub selected_rename_mode: RenameMode,
    pub rename_preview: Vec<String>,
    pub rename_pattern_input: String,
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
            show_rename_mode_popup: false,
            selected_rename_mode: RenameMode::Smart,
            rename_preview: vec![],
            rename_pattern_input: String::new(),
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

    // Rename mode methods
    pub fn open_rename_popup(&mut self) {
        self.show_rename_mode_popup = true;
        self.selected_rename_mode = RenameMode::Smart;
        self.generate_rename_preview();
    }

    pub fn close_rename_popup(&mut self) {
        self.show_rename_mode_popup = false;
        self.rename_preview.clear();
    }

    pub fn select_rename_mode(&mut self, mode: RenameMode) {
        self.selected_rename_mode = mode;
        self.generate_rename_preview();
    }

    pub fn select_next_rename_mode(&mut self) {
        let modes = RenameMode::all();
        let current_idx = modes.iter().position(|&m| m == self.selected_rename_mode).unwrap_or(0);
        let next_idx = (current_idx + 1) % modes.len();
        self.selected_rename_mode = modes[next_idx];
        self.generate_rename_preview();
    }

    pub fn select_previous_rename_mode(&mut self) {
        let modes = RenameMode::all();
        let current_idx = modes.iter().position(|&m| m == self.selected_rename_mode).unwrap_or(0);
        let prev_idx = if current_idx == 0 { modes.len() - 1 } else { current_idx - 1 };
        self.selected_rename_mode = modes[prev_idx];
        self.generate_rename_preview();
    }

    pub fn generate_rename_preview(&mut self) {
        use crate::models::episode::EpisodeParser;

        self.rename_preview.clear();
        let parser = EpisodeParser::new();

        // Get selected file(s) - for now, use the currently selected file
        let selected_file = match self.focus {
            Focus::File => self.files.get(self.selected_index),
            _ => None,
        };

        if let Some(file) = selected_file {
            if let Some(episode_info) = parser.parse(&file.name) {
                // Generate new filename based on selected mode
                let new_name = match self.selected_rename_mode {
                    RenameMode::Smart => {
                        parser.generate_name(&episode_info, "{title}.S{season}E{episode}",
                            &file.name.rsplit('.').next().map(|e| format!(".{}", e)).unwrap_or_default())
                    }
                    RenameMode::Manual => file.name.clone(), // Manual mode - user will input
                    RenameMode::Unified => format!("{}_{:02}", episode_info.title, episode_info.episode),
                    RenameMode::Regex => file.name.clone(), // Regex mode - user will input pattern
                };
                self.rename_preview.push(format!("{} -> {}", file.name, new_name));
            } else {
                self.rename_preview.push(format!("{} (无法识别)", file.name));
            }
        } else if self.files.is_empty() {
            self.rename_preview.push("没有可重命名的文件".to_string());
        } else {
            // Preview all files in current directory
            for file in &self.files {
                if let Some(episode_info) = parser.parse(&file.name) {
                    let ext = file.name.rsplit('.').next().map(|e| format!(".{}", e)).unwrap_or_default();
                    let new_name = parser.generate_name(&episode_info, "{title}.S{season}E{episode}", &ext);
                    self.rename_preview.push(format!("{} -> {}", file.name, new_name));
                } else {
                    self.rename_preview.push(format!("{} (无法识别)", file.name));
                }
            }
        }
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
