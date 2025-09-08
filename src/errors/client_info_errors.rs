use std::fmt;

/// Client information validation errors
#[derive(Debug, Clone, PartialEq)]
pub enum ClientInfoError {
    InvalidEmail {
        email: String,
    },
    InvalidCountryCode {
        code: String,
    },
    InvalidScreenDimensions {
        message: String,
    },
    InvalidIpAddress {
        ip: String,
    },
    ValidationError {
        message: String,
    },
}

impl fmt::Display for ClientInfoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidEmail { email } => write!(f, "Invalid email format: {}", email),
            Self::InvalidCountryCode { code } => write!(f, "Invalid country ISO code: {}", code),
            Self::InvalidScreenDimensions { message } => write!(f, "Invalid screen dimensions: {}", message),
            Self::InvalidIpAddress { ip } => write!(f, "Invalid IP address: {}", ip),
            Self::ValidationError { message } => write!(f, "Client info validation error: {}", message),
        }
    }
}

impl std::error::Error for ClientInfoError {}
