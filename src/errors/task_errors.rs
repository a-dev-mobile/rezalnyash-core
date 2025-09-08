use std::fmt;

/// Task management errors - Task lifecycle, execution, and state management
#[derive(Debug)]
pub enum TaskError {
    TaskNotFound {
        id: String,
    },
    TaskInvalidId {
        task_id: String,
    },
    TaskExecution(String), // Simplified from tokio::task::JoinError
    TaskCancelled,
    TaskTimeout,
    TaskInvalidState {
        current_state: String,
    },
    TaskInvalidStatusTransition {
        from: String,
        to: String,
    },
    TaskMissingClientInfo,
    TaskThreadTerminated,
    TaskThreadSync {
        message: String,
    },
    TaskThreadError {
        details: String,
    },
    TaskMaterialMismatch {
        tile_material: String,
        mosaic_material: String,
    },
    TaskLockError {
        operation: String,
    },
    TaskAlreadyExists {
        task_id: String,
    },
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TaskNotFound { id } => write!(f, "Task not found: {}", id),
            Self::TaskInvalidId { task_id } => write!(f, "Invalid task ID: {}", task_id),
            Self::TaskExecution(details) => write!(f, "Task execution error: {}", details),
            Self::TaskCancelled => write!(f, "Task was cancelled"),
            Self::TaskTimeout => write!(f, "Task timeout exceeded"),
            Self::TaskInvalidState { current_state } => {
                write!(f, "Task is in invalid state: {}", current_state)
            }
            Self::TaskInvalidStatusTransition { from, to } => {
                write!(f, "Invalid status transition from {} to {}", from, to)
            }
            Self::TaskMissingClientInfo => write!(f, "Cannot start thread without user info"),
            Self::TaskThreadTerminated => write!(f, "Thread was terminated during execution"),
            Self::TaskThreadSync { message } => {
                write!(f, "Thread synchronization error: {}", message)
            }
            Self::TaskThreadError { details } => write!(f, "Thread error: {}", details),
            Self::TaskMaterialMismatch {
                tile_material,
                mosaic_material,
            } => {
                write!(
                    f,
                    "Material mismatch: tile[{}] mosaic[{}]",
                    tile_material, mosaic_material
                )
            }
            Self::TaskLockError { operation } => {
                write!(f, "Task lock error during operation: {}", operation)
            }
            Self::TaskAlreadyExists { task_id } => {
                write!(f, "Task already exists with ID: {}", task_id)
            }
        }
    }
}

impl std::error::Error for TaskError {}
