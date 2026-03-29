use crate::api::types::FileItem;
use crate::error::AppError;
use tokio::sync::mpsc;

pub type TaskId = u32;

#[allow(dead_code)]
pub enum TaskResult {
    ListDirectory(TaskId, Result<Vec<FileItem>, AppError>),
    BatchRename(TaskId, Result<(), AppError>),
    Login(TaskId, Result<String, AppError>),
    AutoLogin(TaskId, Result<crate::api::types::UserInfo, AppError>),
}

#[allow(dead_code)]
pub enum PendingTask {
    Idle,
    Loading {
        id: TaskId,
        message: String,
        spinner_frame: usize,
    },
    Renaming {
        id: TaskId,
        total: usize,
        completed: usize,
        message: String,
        spinner_frame: usize,
    },
}

impl PendingTask {
    pub fn advance_spinner(&mut self) {
        match self {
            PendingTask::Loading { spinner_frame, .. } => {
                *spinner_frame = (*spinner_frame + 1) % 10;
            }
            PendingTask::Renaming { spinner_frame, .. } => {
                *spinner_frame = (*spinner_frame + 1) % 10;
            }
            _ => {}
        }
    }

    pub fn get_spinner_char(&self) -> char {
        const SPINNER_CHARS: [char; 10] = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
        let frame = match self {
            PendingTask::Loading { spinner_frame, .. } => *spinner_frame,
            PendingTask::Renaming { spinner_frame, .. } => *spinner_frame,
            _ => 0,
        };
        SPINNER_CHARS[frame % 10]
    }

    pub fn is_loading(&self) -> bool {
        matches!(
            self,
            PendingTask::Loading { .. } | PendingTask::Renaming { .. }
        )
    }

    pub fn get_progress(&self) -> Option<(usize, usize)> {
        match self {
            PendingTask::Renaming {
                completed, total, ..
            } => Some((*completed, *total)),
            _ => None,
        }
    }

    pub fn get_message(&self) -> Option<&str> {
        match self {
            PendingTask::Loading { message, .. } => Some(message),
            PendingTask::Renaming { message, .. } => Some(message),
            _ => None,
        }
    }
}

pub struct TaskChannel {
    pub tx: mpsc::UnboundedSender<TaskResult>,
    pub rx: mpsc::UnboundedReceiver<TaskResult>,
}

impl TaskChannel {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self { tx, rx }
    }
}

impl Default for TaskChannel {
    fn default() -> Self {
        Self::new()
    }
}
