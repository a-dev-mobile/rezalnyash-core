use std::fmt;

/// Service errors - Service-level errors and resource management
#[derive(Debug)]
pub enum ServiceError {
    ServiceTaskAlreadyExists {
        task_id: String,
    },
    ServiceClientAlreadyHasTask {
        client_id: String,
        existing_task_id: String,
    },
    ServiceInvalidClientId {
        client_id: String,
    },
    ServiceShuttingDown,
    ServiceMaxTasksReached,
    ServiceLockFailed {
        resource: String,
    },
    ServiceResourceUnavailable {
        resource: String,
    },
    ServicePermissionDenied {
        operation: String,
    },
    ServiceThreadSync {
        message: String,
    },
    ServiceThreadError {
        details: String,
    },
    ServiceInitializationError {
        message: String,
    },
    ServiceLockError {
        message: String,
    },
    ServiceValidationError {
        message: String,
    },
        ThreadPoolError {
        message: String,
    },
    ServiceNotInitialized,
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ServiceTaskAlreadyExists { task_id } => {
                write!(f, "Task already exists: {}", task_id)
            }
            Self::ServiceClientAlreadyHasTask {
                client_id,
                existing_task_id,
            } => {
                write!(
                    f,
                    "Client {} already has a running task: {}",
                    client_id, existing_task_id
                )
            }
            Self::ServiceInvalidClientId { client_id } => {
                write!(f, "Invalid client ID: {}", client_id)
            }
            Self::ServiceShuttingDown => write!(f, "Service is shutting down"),
            Self::ServiceMaxTasksReached => write!(f, "Maximum number of concurrent tasks reached"),
            Self::ServiceLockFailed { resource } => {
                write!(f, "Lock acquisition failed: {}", resource)
            }
            Self::ServiceResourceUnavailable { resource } => {
                write!(f, "Resource not available: {}", resource)
            }
            Self::ServicePermissionDenied { operation } => {
                write!(f, "Permission denied for operation: {}", operation)
            }
            Self::ServiceThreadSync { message } => {
                write!(f, "Thread synchronization error: {}", message)
            }
            Self::ServiceThreadError { details } => write!(f, "Thread error: {}", details),
            Self::ServiceInitializationError { message } => {
                write!(f, "Service initialization error: {}", message)
            }
            Self::ServiceLockError { message } => {
                write!(f, "Service lock error: {}", message)
            }
            Self::ServiceValidationError { message } => {
                write!(f, "Service validation error: {}", message)
            }
            Self::ThreadPoolError { message } => {
                write!(f, "Thread pool error: {}", message)
            }
            Self::ServiceNotInitialized => write!(f, "Service not initialized"),
        }
    }
}

impl std::error::Error for ServiceError {}
