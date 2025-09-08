use std::fmt;

use super::{
    core_errors::CoreError,
    task_errors::TaskError,
    computation_errors::ComputationError,
    service_errors::ServiceError,
    stock_errors::StockError,
};
/// Result type alias for the application
pub type Result<T> = std::result::Result<T, AppError>;


/// Main application error type that encompasses all possible errors
#[derive(Debug)]
pub enum AppError {
    // Core errors - Basic application errors and external library errors
    Core(CoreError),

    // Task management errors - Task lifecycle, execution, and state management
    Task(TaskError),

    // Computation errors - Optimization, solution computation, and algorithm errors
    Computation(ComputationError),

    // Service errors - Service-level errors and resource management
    Service(ServiceError),

    // Stock errors - Stock solution and panel picker errors
    Stock(StockError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Core(err) => write!(f, "{}", err),
            Self::Task(err) => write!(f, "{}", err),
            Self::Computation(err) => write!(f, "{}", err),
            Self::Service(err) => write!(f, "{}", err),
            Self::Stock(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Core(err) => Some(err),
            Self::Task(err) => Some(err),
            Self::Computation(err) => Some(err),
            Self::Service(err) => Some(err),
            Self::Stock(err) => Some(err),
        }
    }
}

// Automatic conversions from specific error types
impl From<CoreError> for AppError {
    fn from(err: CoreError) -> Self {
        Self::Core(err)
    }
}

impl From<TaskError> for AppError {
    fn from(err: TaskError) -> Self {
        Self::Task(err)
    }
}

impl From<ComputationError> for AppError {
    fn from(err: ComputationError) -> Self {
        Self::Computation(err)
    }
}

impl From<ServiceError> for AppError {
    fn from(err: ServiceError) -> Self {
        Self::Service(err)
    }
}

impl From<StockError> for AppError {
    fn from(err: StockError) -> Self {
        Self::Stock(err)
    }
}

// Automatic conversions from external error types
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self::Core(CoreError::Io(err))
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        Self::Core(CoreError::Json(err))
    }
}

impl From<std::num::ParseFloatError> for AppError {
    fn from(err: std::num::ParseFloatError) -> Self {
        Self::Core(CoreError::ParseFloat(err))
    }
}


