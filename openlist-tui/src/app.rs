use crate::api::client::OpenListClient;
use crate::api::types::FileItem;
use crate::config::Config;
use crate::task::{TaskChannel, PendingTask};

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
}

impl Default for App {
    fn default() -> Self {
        Self {
            client: OpenListClient::new("http://192.168.1.1:5244".into(), None),
            is_authenticated: false,
            current_user: None,
            current_path: "/".into(),
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
}
