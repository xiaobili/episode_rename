use crate::task::{TaskChannel, PendingTask};

pub struct AsyncState {
    pub task_channel: TaskChannel,
    pub pending_task: PendingTask,
}

impl Default for AsyncState {
    fn default() -> Self {
        Self {
            task_channel: TaskChannel::new(),
            pending_task: PendingTask::Idle,
        }
    }
}
