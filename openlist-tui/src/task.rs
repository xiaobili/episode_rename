use tokio::sync::mpsc;
use crate::api::types::FileItem;
use crate::error::AppError;

pub type TaskId = u32;

pub enum TaskResult {
    ListDirectory(TaskId, Result<Vec<FileItem>, AppError>),
    BatchRename(TaskId, Result<(), AppError>),
    Login(TaskId, Result<String, AppError>),
}

pub enum PendingTask {
    Idle,
    Loading { id: TaskId },
    Renaming { id: TaskId, total: usize, completed: usize },
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
