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
pub enum UnifiedFocus {
    ShowName,
    Season,
    StartEpisode,
    Pattern,
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegexFocus {
    Find,
    Replace,
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
    pub error_type: Option<String>,
    pub error_code: Option<i32>,
    pub is_token_expired: bool,
    pub auto_relogin_pending: bool,
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
    // Manual rename state
    pub show_manual_rename_popup: bool,
    pub manual_rename_input: String,
    pub manual_rename_index: usize,
    pub files_to_rename: Vec<usize>,
    pub manual_rename_results: Vec<(String, String, bool)>, // (old_name, new_name, confirmed)
    pub manual_rename_finished: bool,
    // Unified naming state
    pub show_unified_input: bool,
    pub unified_show_name: String,
    pub unified_season: String,
    pub unified_start_episode: String,
    pub unified_pattern: String,
    pub unified_focus: UnifiedFocus,
    pub unified_preview: Vec<String>,
    pub unified_rename_results: Vec<(String, String, bool)>, // (old_name, new_name, confirmed)
    pub unified_rename_finished: bool,
    // Regex rename state
    pub show_regex_input: bool,
    pub regex_find: String,
    pub regex_replace: String,
    pub regex_focus: RegexFocus,
    pub regex_preview: Vec<(String, String)>, // (original, new)
    pub regex_rename_results: Vec<(String, String, bool)>, // (old_name, new_name, confirmed)
    pub regex_rename_finished: bool,
    pub regex_error: Option<String>,
    // Single file rename state
    pub show_single_rename: bool,
    pub single_rename_input: String,
    pub single_rename_target: Option<FileItem>,
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
            error_type: None,
            error_code: None,
            is_token_expired: false,
            auto_relogin_pending: false,
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
            // Manual rename state
            show_manual_rename_popup: false,
            manual_rename_input: String::new(),
            manual_rename_index: 0,
            files_to_rename: vec![],
            manual_rename_results: vec![],
            manual_rename_finished: false,
            // Unified naming state
            show_unified_input: false,
            unified_show_name: String::new(),
            unified_season: String::new(),
            unified_start_episode: String::new(),
            unified_pattern: "{title}.S{season}E{episode}".to_string(),
            unified_focus: UnifiedFocus::ShowName,
            unified_preview: vec![],
            unified_rename_results: vec![],
            unified_rename_finished: false,
            // Regex rename state
            show_regex_input: false,
            regex_find: String::new(),
            regex_replace: String::new(),
            regex_focus: RegexFocus::Find,
            regex_preview: vec![],
            regex_rename_results: vec![],
            regex_rename_finished: false,
            regex_error: None,
            // Single file rename state
            show_single_rename: false,
            single_rename_input: String::new(),
            single_rename_target: None,
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

    // Manual rename methods
    pub fn start_manual_rename(&mut self) {
        if self.files.is_empty() {
            return;
        }
        // Collect all file indices to rename
        self.files_to_rename = (0..self.files.len()).collect();
        self.manual_rename_index = 0;
        self.manual_rename_results.clear();

        // Pre-populate input with current filename
        if let Some(file) = self.files.get(self.manual_rename_index) {
            self.manual_rename_input = file.name.clone();
        }
        self.show_manual_rename_popup = true;
    }

    pub fn submit_manual_rename(&mut self) {
        // Save the rename result
        if let Some(file) = self.files.get(self.manual_rename_index) {
            let old_name = file.name.clone();
            let new_name = self.manual_rename_input.clone();
            if !new_name.is_empty() && new_name != old_name {
                self.manual_rename_results.push((old_name, new_name, true));
            }
        }

        // Move to next file
        self.next_manual_rename();
    }

    pub fn skip_manual_rename(&mut self) {
        // Skip current file (don't add to results)
        self.next_manual_rename();
    }

    pub fn next_manual_rename(&mut self) {
        self.manual_rename_index += 1;

        if self.manual_rename_index >= self.files.len() {
            // All files processed, close popup and execute batch rename
            self.finish_manual_rename();
        } else {
            // Load next filename into input
            if let Some(file) = self.files.get(self.manual_rename_index) {
                self.manual_rename_input = file.name.clone();
            }
        }
    }

    pub fn finish_manual_rename(&mut self) {
        self.show_manual_rename_popup = false;
        self.manual_rename_input.clear();
        self.files_to_rename.clear();
        self.manual_rename_index = 0;
        self.manual_rename_finished = true;
        // Note: manual_rename_results contains the confirmed renames to execute
        // Results are cleared after batch rename execution in main.rs
    }

    pub fn cancel_manual_rename(&mut self) {
        self.show_manual_rename_popup = false;
        self.manual_rename_input.clear();
        self.files_to_rename.clear();
        self.manual_rename_index = 0;
        self.manual_rename_results.clear();
        self.manual_rename_finished = false;
    }

    pub fn delete_last_manual_rename_char(&mut self) {
        self.manual_rename_input.pop();
    }

    pub fn get_manual_rename_progress(&self) -> (usize, usize) {
        (self.manual_rename_index + 1, self.files.len())
    }

    pub fn get_current_manual_rename_file(&self) -> Option<&FileItem> {
        self.files.get(self.manual_rename_index)
    }

    pub fn get_manual_rename_results(&self) -> &[(String, String, bool)] {
        &self.manual_rename_results
    }

    pub fn take_manual_rename_results(&mut self) -> Vec<(String, String, bool)> {
        std::mem::take(&mut self.manual_rename_results)
    }

    // Unified naming methods
    pub fn start_unified_mode(&mut self) {
        if self.files.is_empty() {
            return;
        }
        self.show_unified_input = true;
        self.unified_focus = UnifiedFocus::ShowName;
        self.unified_show_name.clear();
        self.unified_season = "1".to_string();
        self.unified_start_episode = "1".to_string();
        self.unified_pattern = "{title}.S{season}E{episode}".to_string();
        self.unified_preview.clear();
        self.unified_rename_results.clear();
        self.unified_rename_finished = false;
        self.generate_unified_preview();
    }

    pub fn submit_unified(&mut self) {
        // Validate and execute unified rename
        self.unified_rename_finished = true;
        self.show_unified_input = false;
        // Note: unified_rename_results contains the renames to execute
        // Results are cleared after batch rename execution in main.rs
    }

    pub fn cancel_unified(&mut self) {
        self.show_unified_input = false;
        self.unified_show_name.clear();
        self.unified_season.clear();
        self.unified_start_episode.clear();
        self.unified_pattern = "{title}.S{season}E{episode}".to_string();
        self.unified_focus = UnifiedFocus::ShowName;
        self.unified_preview.clear();
        self.unified_rename_results.clear();
        self.unified_rename_finished = false;
    }

    pub fn toggle_unified_focus(&mut self) {
        self.unified_focus = match self.unified_focus {
            UnifiedFocus::ShowName => UnifiedFocus::Season,
            UnifiedFocus::Season => UnifiedFocus::StartEpisode,
            UnifiedFocus::StartEpisode => UnifiedFocus::Pattern,
            UnifiedFocus::Pattern => UnifiedFocus::ShowName,
        };
    }

    pub fn generate_unified_preview(&mut self) {
        self.unified_preview.clear();

        let show_name = if self.unified_show_name.is_empty() {
            "Show".to_string()
        } else {
            self.unified_show_name.clone()
        };

        let season: u32 = self.unified_season.parse().unwrap_or(1);
        let start_episode: u32 = self.unified_start_episode.parse().unwrap_or(1);

        // Generate preview for first few files
        for (i, file) in self.files.iter().take(5).enumerate() {
            let episode = start_episode + i as u32;
            let ext = file.name.rsplit('.').next().map(|e| format!(".{}", e)).unwrap_or_default();

            // Generate name using pattern
            let s = format!("{:02}", season);
            let e = format!("{:02}", episode);
            let new_name = format!(
                "{}{}",
                self.unified_pattern
                    .replace("{title}", &show_name)
                    .replace("{season}", &s)
                    .replace("{episode}", &e),
                ext
            );

            self.unified_preview.push(format!("{} -> {}", file.name, new_name));
        }

        if self.files.len() > 5 {
            self.unified_preview.push(format!("... 还有 {} 个文件", self.files.len() - 5));
        }
    }

    pub fn validate_unified_inputs(&self) -> Result<(), String> {
        if self.unified_show_name.is_empty() {
            return Err("剧集名称不能为空".to_string());
        }

        if self.unified_season.is_empty() {
            return Err("季数不能为空".to_string());
        }

        if self.unified_season.parse::<u32>().is_err() {
            return Err("季数必须是数字".to_string());
        }

        if self.unified_start_episode.is_empty() {
            return Err("起始集数不能为空".to_string());
        }

        if self.unified_start_episode.parse::<u32>().is_err() {
            return Err("起始集数必须是数字".to_string());
        }

        Ok(())
    }

    pub fn execute_unified_rename(&mut self) -> Vec<(String, String, bool)> {
        self.unified_rename_results.clear();

        let show_name = self.unified_show_name.clone();
        let season: u32 = self.unified_season.parse().unwrap_or(1);
        let start_episode: u32 = self.unified_start_episode.parse().unwrap_or(1);

        for (i, file) in self.files.iter().enumerate() {
            let episode = start_episode + i as u32;
            let ext = file.name.rsplit('.').next().map(|e| format!(".{}", e)).unwrap_or_default();

            // Generate name using pattern
            let s = format!("{:02}", season);
            let e = format!("{:02}", episode);
            let new_name = format!(
                "{}{}",
                self.unified_pattern
                    .replace("{title}", &show_name)
                    .replace("{season}", &s)
                    .replace("{episode}", &e),
                ext
            );

            if new_name != file.name {
                self.unified_rename_results.push((file.name.clone(), new_name, true));
            }
        }

        self.unified_rename_finished = true;
        self.show_unified_input = false;

        self.unified_rename_results.clone()
    }

    pub fn get_unified_preview(&self) -> &[String] {
        &self.unified_preview
    }

    pub fn take_unified_rename_results(&mut self) -> Vec<(String, String, bool)> {
        std::mem::take(&mut self.unified_rename_results)
    }

    // Regex rename methods
    pub fn start_regex_mode(&mut self) {
        if self.files.is_empty() {
            return;
        }
        self.show_regex_input = true;
        self.regex_focus = RegexFocus::Find;
        self.regex_find.clear();
        self.regex_replace.clear();
        self.regex_preview.clear();
        self.regex_rename_results.clear();
        self.regex_rename_finished = false;
        self.regex_error = None;
    }

    pub fn cancel_regex(&mut self) {
        self.show_regex_input = false;
        self.regex_find.clear();
        self.regex_replace.clear();
        self.regex_focus = RegexFocus::Find;
        self.regex_preview.clear();
        self.regex_rename_results.clear();
        self.regex_rename_finished = false;
        self.regex_error = None;
    }

    pub fn toggle_regex_focus(&mut self) {
        self.regex_focus = match self.regex_focus {
            RegexFocus::Find => RegexFocus::Replace,
            RegexFocus::Replace => RegexFocus::Find,
        };
    }

    pub fn submit_regex(&mut self) {
        // Validate regex pattern first
        match regex::Regex::new(&self.regex_find) {
            Ok(_) => {
                // Valid regex - generate preview
                self.generate_regex_preview();
                self.regex_error = None;
            }
            Err(e) => {
                // Invalid regex - show error
                self.regex_error = Some(format!("正则表达式无效：{}", e));
            }
        }
    }

    pub fn generate_regex_preview(&mut self) {
        self.regex_preview.clear();

        if let Ok(re) = regex::Regex::new(&self.regex_find) {
            for file in &self.files {
                let new_name = re.replace_all(&file.name, &self.regex_replace).to_string();
                if new_name != file.name {
                    self.regex_preview.push((file.name.clone(), new_name));
                }
            }
        }
    }

    pub fn execute_regex_rename(&mut self) -> Vec<(String, String, bool)> {
        self.regex_rename_results.clear();

        if let Ok(re) = regex::Regex::new(&self.regex_find) {
            for file in &self.files {
                let new_name = re.replace_all(&file.name, &self.regex_replace).to_string();
                if new_name != file.name {
                    self.regex_rename_results.push((file.name.clone(), new_name, true));
                }
            }
        }

        self.regex_rename_finished = true;
        self.show_regex_input = false;

        self.regex_rename_results.clone()
    }

    pub fn get_regex_preview(&self) -> &[(String, String)] {
        &self.regex_preview
    }

    pub fn take_regex_rename_results(&mut self) -> Vec<(String, String, bool)> {
        std::mem::take(&mut self.regex_rename_results)
    }

    pub fn has_regex_preview(&self) -> bool {
        !self.regex_preview.is_empty() && self.regex_error.is_none()
    }

    // Single file rename methods
    pub fn start_single_rename(&mut self) {
        // Get selected file from current focus (file list)
        if self.focus != Focus::File {
            return;
        }

        let selected_file = self.files.get(self.selected_index);
        if let Some(file) = selected_file {
            self.single_rename_target = Some(file.clone());
            self.single_rename_input = file.name.clone();
            self.show_single_rename = true;
        }
    }

    pub fn submit_single_rename(&mut self) {
        // Single rename is executed immediately via API
        // The actual API call is handled in main.rs
        // Here we just set a flag to indicate submission
        if self.single_rename_target.is_some() && !self.single_rename_input.is_empty() {
            // Close the dialog - main.rs will handle the API call
            self.show_single_rename = false;
        }
    }

    pub fn cancel_single_rename(&mut self) {
        self.show_single_rename = false;
        self.single_rename_input.clear();
        self.single_rename_target = None;
    }

    pub fn delete_last_single_rename_char(&mut self) {
        self.single_rename_input.pop();
    }

    pub fn get_single_rename_target(&self) -> Option<&FileItem> {
        self.single_rename_target.as_ref()
    }

    /// Handle API errors, detecting token expiration and triggering re-login flow
    pub fn handle_api_error_from_app_error(&mut self, error: crate::error::AppError) {
        use crate::error::AppError;

        // Store error type and code for display
        self.error_type = Some(error.error_type().to_string());
        self.error_code = error.error_code();

        match &error {
            AppError::TokenExpired => {
                // Token expired - set flags and prepare for re-login
                self.is_token_expired = true;
                self.auto_relogin_pending = true;
                self.is_authenticated = false;
                self.error_message = Some("Token 已过期，请重新登录".to_string());
                self.show_error_popup = true;
            }
            AppError::Auth(msg) => {
                // Authentication error - may be token related
                self.is_token_expired = true;
                self.error_message = Some(format!("认证失败：{}", msg));
                self.show_error_popup = true;
            }
            AppError::Network(e) => {
                // Network error - offer retry option
                self.is_token_expired = false;
                self.error_message = Some(format!("网络错误：{}", e));
                self.show_error_popup = true;
            }
            AppError::NotFound(path) => {
                self.is_token_expired = false;
                self.error_message = Some(format!("路径不存在：{}", path));
                self.show_error_popup = true;
            }
            AppError::ApiError(msg) => {
                self.is_token_expired = false;
                self.error_message = Some(format!("API 错误：{}", msg));
                self.show_error_popup = true;
            }
            _ => {
                // Other errors
                self.is_token_expired = false;
                self.error_message = Some(format!("{}", error));
                self.show_error_popup = true;
            }
        }
    }

    /// Handle API errors from boxed dyn Error (for load_directory_contents)
    pub fn handle_api_error(&mut self, error: Box<dyn std::error::Error + 'static>) {
        use crate::error::AppError;

        // Convert the error to AppError
        let app_error = AppError::from_boxed_error(error);

        // Delegate to the main handler
        self.handle_api_error_from_app_error(app_error);
    }

    /// Clear error state and prepare for re-login
    pub fn clear_error_and_prepare_relogin(&mut self) {
        self.show_error_popup = false;
        self.error_message = None;
        self.error_type = None;
        self.error_code = None;
        self.is_token_expired = false;
        self.auto_relogin_pending = false;
        self.show_login_screen = true;
    }

    /// Clear error state without re-login
    pub fn clear_error(&mut self) {
        self.show_error_popup = false;
        self.error_message = None;
        self.error_type = None;
        self.error_code = None;
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
