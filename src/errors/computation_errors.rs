use std::fmt;

/// Computation errors - Optimization, solution computation, and algorithm errors
#[derive(Debug)]
pub enum ComputationError {
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
}

impl fmt::Display for ComputationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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
        }
    }
}

impl std::error::Error for ComputationError {}
