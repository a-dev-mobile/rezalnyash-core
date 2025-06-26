use std::fmt;

/// Main application error type that encompasses all possible errors
#[derive(Debug)]
pub enum AppError {
    // Core errors - Basic application errors and external library errors
    InvalidConfiguration {
        message: String,
    },
    InvalidInput {
        details: String,
    },
    Io(std::io::Error),
    Json(serde_json::Error),
    ParseFloat(std::num::ParseFloatError),
    Internal {
        message: String,
    },

    // Task management errors - Task lifecycle, execution, and state management
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

    // Computation errors - Optimization, solution computation, and algorithm errors
    OptimizationFailed {
        reason: String,
    },
    ComputationGeneral {
        message: String,
    },
    SolutionComputation {
        message: String,
    },
    SolutionComparison {
        message: String,
    },
    NodeCopy {
        message: String,
    },
    CandidateSearch {
        message: String,
    },

    // Service errors - Service-level errors and resource management
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

    // Stock errors - Stock solution and panel picker errors
    StockNoStockTiles,
    StockNoTilesToFit,
    StockComputationLimitExceeded,
    StockPanelPickerNotInitialized,
    StockGenerationInterrupted {
        message: String,
    },
    StockNoMoreSolutions,
    StockPanelPickerThread {
        message: String,
    },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Core errors
            Self::InvalidConfiguration { message } => {
                write!(f, "Invalid configuration: {}", message)
            }
            Self::InvalidInput { details } => write!(f, "Invalid input data: {}", details),
            Self::Io(err) => write!(f, "IO error: {}", err),
            Self::Json(err) => write!(f, "JSON parsing error: {}", err),
            Self::ParseFloat(err) => write!(f, "Parse float error: {}", err),
            Self::Internal { message } => write!(f, "Internal error: {}", message),

            // Task errors
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

            // Computation errors
            Self::OptimizationFailed { reason } => write!(f, "Optimization failed: {}", reason),
            Self::ComputationGeneral { message } => write!(f, "Computation error: {}", message),
            Self::SolutionComputation { message } => {
                write!(f, "Error during solution computation: {}", message)
            }
            Self::SolutionComparison { message } => {
                write!(f, "Error during solution comparison: {}", message)
            }
            Self::NodeCopy { message } => write!(f, "Node copying error: {}", message),
            Self::CandidateSearch { message } => write!(f, "Candidate search error: {}", message),

            // Service errors
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

            // Stock errors
            Self::StockNoStockTiles => write!(f, "No stock tiles provided"),
            Self::StockNoTilesToFit => write!(f, "No tiles to fit provided"),
            Self::StockComputationLimitExceeded => {
                write!(f, "Stock solution computation exceeded reasonable limits")
            }
            Self::StockPanelPickerNotInitialized => {
                write!(f, "Stock panel picker thread not initialized")
            }
            Self::StockGenerationInterrupted { message } => {
                write!(f, "Stock solution generation interrupted: {}", message)
            }
            Self::StockNoMoreSolutions => write!(f, "No more stock solutions available"),
            Self::StockPanelPickerThread { message } => {
                write!(f, "Stock panel picker thread error: {}", message)
            }
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::Json(err) => Some(err),
            Self::ParseFloat(err) => Some(err),
            _ => None,
        }
    }
}

// Automatic conversions from external error types
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}

impl From<std::num::ParseFloatError> for AppError {
    fn from(err: std::num::ParseFloatError) -> Self {
        Self::ParseFloat(err)
    }
}

/// Result type alias for the application
pub type Result<T> = std::result::Result<T, AppError>;

impl AppError {
    /// Returns true if this error indicates a temporary condition that might be retried
    pub fn is_retryable(&self) -> bool {
        match self {
            // Core errors
            Self::Io(_) => true,

            // Task errors
            Self::TaskTimeout | Self::TaskExecution(_) | Self::TaskThreadSync { .. } => true,

            // Service errors
            Self::ServiceResourceUnavailable { .. }
            | Self::ServiceLockFailed { .. }
            | Self::ServiceMaxTasksReached => true,

            // Stock errors
            Self::StockGenerationInterrupted { .. } | Self::StockPanelPickerThread { .. } => true,

            _ => false,
        }
    }

    /// Returns true if this error indicates a client error (4xx equivalent)
    pub fn is_client_error(&self) -> bool {
        match self {
            // Core errors
            Self::InvalidConfiguration { .. } | Self::InvalidInput { .. } | Self::Json(_) => true,

            // Task errors
            Self::TaskNotFound { .. }
            | Self::TaskInvalidId { .. }
            | Self::TaskInvalidState { .. }
            | Self::TaskInvalidStatusTransition { .. }
            | Self::TaskMissingClientInfo
            | Self::TaskMaterialMismatch { .. } => true,

            // Computation errors
            Self::OptimizationFailed { .. } => true,

            // Service errors
            Self::ServiceTaskAlreadyExists { .. }
            | Self::ServiceClientAlreadyHasTask { .. }
            | Self::ServiceInvalidClientId { .. }
            | Self::ServicePermissionDenied { .. } => true,

            // Stock errors
            Self::StockNoStockTiles | Self::StockNoTilesToFit => true,

            _ => false,
        }
    }

    /// Returns true if this error indicates a server error (5xx equivalent)
    pub fn is_server_error(&self) -> bool {
        match self {
            // Core errors
            Self::Internal { .. } | Self::Io(_) => true,

            // Task errors
            Self::TaskExecution(_)
            | Self::TaskThreadTerminated
            | Self::TaskThreadSync { .. }
            | Self::TaskThreadError { .. } => true,

            // Computation errors
            Self::ComputationGeneral { .. }
            | Self::SolutionComputation { .. }
            | Self::SolutionComparison { .. }
            | Self::NodeCopy { .. }
            | Self::CandidateSearch { .. } => true,

            // Service errors
            Self::ServiceShuttingDown
            | Self::ServiceLockFailed { .. }
            | Self::ServiceResourceUnavailable { .. }
            | Self::ServiceMaxTasksReached => true,

            // Stock errors
            Self::StockComputationLimitExceeded
            | Self::StockPanelPickerNotInitialized
            | Self::StockGenerationInterrupted { .. }
            | Self::StockNoMoreSolutions
            | Self::StockPanelPickerThread { .. } => true,

            _ => false,
        }
    }
}
