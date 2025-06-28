use serde::{Deserialize, Serialize};

/// Status of a task or operation
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


