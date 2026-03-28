use crate::api::types::FileItem;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RenameMode {
    Smart,
    Manual,
    Unified,
    Regex,
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
        [
            RenameMode::Smart,
            RenameMode::Manual,
            RenameMode::Unified,
            RenameMode::Regex,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnifiedFocus {
    ShowName,
    Season,
    StartEpisode,
    Pattern,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegexFocus {
    Find,
    Replace,
}

pub struct RenameModeSelectionState {
    pub selected_mode: RenameMode,
    pub preview: Vec<String>,
}

impl Default for RenameModeSelectionState {
    fn default() -> Self {
        Self {
            selected_mode: RenameMode::Smart,
            preview: vec![],
        }
    }
}

pub struct SmartRenameState {
    pub results: Vec<(String, String, bool)>,
    pub pending: bool,
}

impl Default for SmartRenameState {
    fn default() -> Self {
        Self {
            results: vec![],
            pending: false,
        }
    }
}

pub struct ManualRenameState {
    pub input: String,
    pub index: usize,
    pub files_to_rename: Vec<usize>,
    pub results: Vec<(String, String, bool)>,
    pub finished: bool,
}

impl Default for ManualRenameState {
    fn default() -> Self {
        Self {
            input: String::new(),
            index: 0,
            files_to_rename: vec![],
            results: vec![],
            finished: false,
        }
    }
}

pub struct UnifiedRenameState {
    pub show_name: String,
    pub season: String,
    pub start_episode: String,
    pub pattern: String,
    pub focus: UnifiedFocus,
    pub preview: Vec<String>,
    pub results: Vec<(String, String, bool)>,
    pub finished: bool,
}

impl Default for UnifiedRenameState {
    fn default() -> Self {
        Self {
            show_name: String::new(),
            season: "1".to_string(),
            start_episode: "1".to_string(),
            pattern: "{title}.S{season}E{episode}".to_string(),
            focus: UnifiedFocus::ShowName,
            preview: vec![],
            results: vec![],
            finished: false,
        }
    }
}

pub struct RegexRenameState {
    pub find: String,
    pub replace: String,
    pub focus: RegexFocus,
    pub preview: Vec<(String, String)>,
    pub results: Vec<(String, String, bool)>,
    pub finished: bool,
    pub error: Option<String>,
}

impl Default for RegexRenameState {
    fn default() -> Self {
        Self {
            find: String::new(),
            replace: String::new(),
            focus: RegexFocus::Find,
            preview: vec![],
            results: vec![],
            finished: false,
            error: None,
        }
    }
}

pub struct SingleRenameState {
    pub input: String,
    pub target: Option<FileItem>,
}

impl Default for SingleRenameState {
    fn default() -> Self {
        Self {
            input: String::new(),
            target: None,
        }
    }
}

pub struct RenameState {
    pub mode_selection: RenameModeSelectionState,
    pub smart: SmartRenameState,
    pub manual: ManualRenameState,
    pub unified: UnifiedRenameState,
    pub regex: RegexRenameState,
    pub single: SingleRenameState,
}

impl Default for RenameState {
    fn default() -> Self {
        Self {
            mode_selection: RenameModeSelectionState::default(),
            smart: SmartRenameState::default(),
            manual: ManualRenameState::default(),
            unified: UnifiedRenameState::default(),
            regex: RegexRenameState::default(),
            single: SingleRenameState::default(),
        }
    }
}
