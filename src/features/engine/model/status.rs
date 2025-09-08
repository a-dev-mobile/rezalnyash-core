use serde::{Deserialize, Serialize};

/// Статусы задач
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    Idle,
    Queued,
    Running,
    Finished,
    Stopped,
    Terminated,
    Error,
}