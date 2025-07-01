use serde::{Deserialize, Serialize};
use std::fmt;

/// Status of a task or operation
/// 
/// Represents the various states a task can be in during its lifecycle.
/// This enum provides methods for status transitions and validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Status {
    /// Task is idle and waiting to be queued
    Idle,
    /// Task is queued and waiting to be processed
    Queued,
    /// Task is currently running
    Running,
    /// Task has completed successfully
    Finished,
    /// Task was stopped by user request
    Stopped,
    /// Task was terminated forcefully
    Terminated,
    /// Task encountered an error
    Error,
}

impl Status {
    /// Returns true if the status represents an active state (queued or running)
    pub fn is_active(&self) -> bool {
        matches!(self, Status::Queued | Status::Running)
    }

    /// Returns true if the status represents a completed state (finished, stopped, terminated, or error)
    pub fn is_completed(&self) -> bool {
        matches!(self, Status::Finished | Status::Stopped | Status::Terminated | Status::Error)
    }

    /// Returns true if the status represents a successful completion
    pub fn is_successful(&self) -> bool {
        matches!(self, Status::Finished)
    }

    /// Returns true if the status represents a failure state
    pub fn is_failed(&self) -> bool {
        matches!(self, Status::Error)
    }

    /// Returns true if the status can transition to the given target status
    pub fn can_transition_to(&self, target: Status) -> bool {
        match (self, target) {
            // From Idle
            (Status::Idle, Status::Queued) => true,
            (Status::Idle, Status::Error) => true,
            
            // From Queued
            (Status::Queued, Status::Running) => true,
            (Status::Queued, Status::Stopped) => true,
            (Status::Queued, Status::Terminated) => true,
            (Status::Queued, Status::Error) => true,
            
            // From Running
            (Status::Running, Status::Finished) => true,
            (Status::Running, Status::Stopped) => true,
            (Status::Running, Status::Terminated) => true,
            (Status::Running, Status::Error) => true,
            
            // Terminal states cannot transition to anything except error
            (Status::Finished | Status::Stopped | Status::Terminated, Status::Error) => true,
            
            // Error can transition back to idle for retry
            (Status::Error, Status::Idle) => true,
            (Status::Error, Status::Queued) => true,
            
            // Same status is always valid
            (a, b) if *a == b => true,
            
            // All other transitions are invalid
            _ => false,
        }
    }

    /// Attempts to transition to the target status, returning an error if invalid
    pub fn transition_to(&self, target: Status) -> Result<Status, StatusTransitionError> {
        if self.can_transition_to(target) {
            Ok(target)
        } else {
            Err(StatusTransitionError::InvalidTransition {
                from: *self,
                to: target,
            })
        }
    }

    /// Returns all valid next statuses from the current status
    pub fn valid_next_statuses(&self) -> Vec<Status> {
        let all_statuses = [
            Status::Idle,
            Status::Queued,
            Status::Running,
            Status::Finished,
            Status::Stopped,
            Status::Terminated,
            Status::Error,
        ];

        all_statuses
            .iter()
            .filter(|&status| self.can_transition_to(*status) && *status != *self)
            .copied()
            .collect()
    }

    /// Returns a human-readable description of the status
    pub fn description(&self) -> &'static str {
        match self {
            Status::Idle => "Task is idle and ready to be queued",
            Status::Queued => "Task is queued and waiting for processing",
            Status::Running => "Task is currently being processed",
            Status::Finished => "Task completed successfully",
            Status::Stopped => "Task was stopped by user request",
            Status::Terminated => "Task was forcefully terminated",
            Status::Error => "Task encountered an error during processing",
        }
    }

    /// Returns the priority level for status ordering (lower number = higher priority)
    pub fn priority(&self) -> u8 {
        match self {
            Status::Error => 0,        // Highest priority
            Status::Running => 1,
            Status::Queued => 2,
            Status::Stopped => 3,
            Status::Terminated => 4,
            Status::Finished => 5,
            Status::Idle => 6,         // Lowest priority
        }
    }

    /// Returns all possible status values
    pub fn all() -> [Status; 7] {
        [
            Status::Idle,
            Status::Queued,
            Status::Running,
            Status::Finished,
            Status::Stopped,
            Status::Terminated,
            Status::Error,
        ]
    }

    /// Parses a status from a string representation
    pub fn from_str(s: &str) -> Result<Status, StatusParseError> {
        match s.to_lowercase().as_str() {
            "idle" => Ok(Status::Idle),
            "queued" => Ok(Status::Queued),
            "running" => Ok(Status::Running),
            "finished" => Ok(Status::Finished),
            "stopped" => Ok(Status::Stopped),
            "terminated" => Ok(Status::Terminated),
            "error" => Ok(Status::Error),
            _ => Err(StatusParseError::UnknownStatus(s.to_string())),
        }
    }

    /// Returns the status as a lowercase string
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Idle => "idle",
            Status::Queued => "queued",
            Status::Running => "running",
            Status::Finished => "finished",
            Status::Stopped => "stopped",
            Status::Terminated => "terminated",
            Status::Error => "error",
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::Idle
    }
}

impl PartialOrd for Status {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Status {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority().cmp(&other.priority())
    }
}

/// Errors that can occur during status transitions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatusTransitionError {
    /// Invalid transition from one status to another
    InvalidTransition { from: Status, to: Status },
}

impl fmt::Display for StatusTransitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatusTransitionError::InvalidTransition { from, to } => {
                write!(f, "Invalid status transition from {} to {}", from, to)
            }
        }
    }
}

impl std::error::Error for StatusTransitionError {}

/// Errors that can occur when parsing status from string
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatusParseError {
    /// Unknown status string
    UnknownStatus(String),
}

impl fmt::Display for StatusParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatusParseError::UnknownStatus(status) => {
                write!(f, "Unknown status: '{}'", status)
            }
        }
    }
}

impl std::error::Error for StatusParseError {}
