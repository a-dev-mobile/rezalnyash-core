use std::fmt;

/// Core application errors - Basic application errors and external library errors
#[derive(Debug)]
pub enum CoreError {
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
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration { message } => {
                write!(f, "Invalid configuration: {}", message)
            }
            Self::InvalidInput { details } => write!(f, "Invalid input data: {}", details),
            Self::Io(err) => write!(f, "IO error: {}", err),
            Self::Json(err) => write!(f, "JSON parsing error: {}", err),
            Self::ParseFloat(err) => write!(f, "Parse float error: {}", err),
            Self::Internal { message } => write!(f, "Internal error: {}", message),
        }
    }
}

impl std::error::Error for CoreError {
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
impl From<std::io::Error> for CoreError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<serde_json::Error> for CoreError {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}

impl From<std::num::ParseFloatError> for CoreError {
    fn from(err: std::num::ParseFloatError) -> Self {
        Self::ParseFloat(err)
    }
}
