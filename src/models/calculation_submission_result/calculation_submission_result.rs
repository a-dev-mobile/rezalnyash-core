//! Calculation Submission Result Model
//!
//! This module provides a complete Rust conversion of the Java CalculationSubmissionResult class,
//! maintaining functional equivalence while using idiomatic Rust patterns.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents the result of a calculation submission
///
/// This is a direct conversion of the Java CalculationSubmissionResult class,
/// maintaining all fields and functionality while using Rust idioms.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CalculationSubmissionResult {
    /// Status code indicating the result of the submission
    pub status_code: String,
    
    /// Optional task identifier assigned to the submission
    pub task_id: Option<String>,
}

impl CalculationSubmissionResult {
    /// Creates a new CalculationSubmissionResult with both status code and task ID
    ///
    /// # Arguments
    /// * `status_code` - The status code for the submission
    /// * `task_id` - The task identifier for the submission
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::models::CalculationSubmissionResult;
    /// 
    /// let result = CalculationSubmissionResult::new("SUCCESS".to_string(), Some("task_123".to_string()));
    /// assert_eq!(result.status_code(), "SUCCESS");
    /// assert_eq!(result.task_id(), Some("task_123"));
    /// ```
    pub fn new(status_code: String, task_id: Option<String>) -> Self {
        Self {
            status_code,
            task_id,
        }
    }

    /// Creates a new CalculationSubmissionResult with only a status code
    ///
    /// # Arguments
    /// * `status_code` - The status code for the submission
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::models::CalculationSubmissionResult;
    /// 
    /// let result = CalculationSubmissionResult::with_status_code("ERROR".to_string());
    /// assert_eq!(result.status_code(), "ERROR");
    /// assert_eq!(result.task_id(), None);
    /// ```
    pub fn with_status_code(status_code: String) -> Self {
        Self {
            status_code,
            task_id: None,
        }
    }

    /// Gets the status code
    ///
    /// # Returns
    /// A reference to the status code string
    pub fn status_code(&self) -> &str {
        &self.status_code
    }

    /// Sets the status code
    ///
    /// # Arguments
    /// * `status_code` - The new status code
    pub fn set_status_code(&mut self, status_code: String) {
        self.status_code = status_code;
    }

    /// Gets the task ID
    ///
    /// # Returns
    /// An Option containing the task ID if present
    pub fn task_id(&self) -> Option<&str> {
        self.task_id.as_deref()
    }

    /// Sets the task ID
    ///
    /// # Arguments
    /// * `task_id` - The new task ID (can be None)
    pub fn set_task_id(&mut self, task_id: Option<String>) {
        self.task_id = task_id;
    }

    /// Checks if the submission was successful based on status code
    ///
    /// # Returns
    /// true if status code indicates success, false otherwise
    pub fn is_success(&self) -> bool {
        self.status_code.to_uppercase() == "SUCCESS" || 
        self.status_code.to_uppercase() == "OK" ||
        self.status_code.starts_with("2") // HTTP 2xx status codes
    }

    /// Checks if the submission has an error based on status code
    ///
    /// # Returns
    /// true if status code indicates an error, false otherwise
    pub fn is_error(&self) -> bool {
        self.status_code.to_uppercase() == "ERROR" ||
        self.status_code.starts_with("4") || // HTTP 4xx status codes
        self.status_code.starts_with("5")    // HTTP 5xx status codes
    }

    /// Checks if a task ID is present
    ///
    /// # Returns
    /// true if task_id is Some, false if None
    pub fn has_task_id(&self) -> bool {
        self.task_id.is_some()
    }

    /// Consumes the result and returns the task ID if present
    ///
    /// # Returns
    /// The task ID string if present, None otherwise
    pub fn into_task_id(self) -> Option<String> {
        self.task_id
    }

    /// Creates a builder for constructing CalculationSubmissionResult instances
    ///
    /// # Returns
    /// A new CalculationSubmissionResultBuilder
    pub fn builder() -> CalculationSubmissionResultBuilder {
        CalculationSubmissionResultBuilder::new()
    }
}

/// Builder pattern for CalculationSubmissionResult
#[derive(Debug, Default)]
pub struct CalculationSubmissionResultBuilder {
    status_code: Option<String>,
    task_id: Option<String>,
}

impl CalculationSubmissionResultBuilder {
    /// Creates a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the status code
    pub fn status_code<S: Into<String>>(mut self, status_code: S) -> Self {
        self.status_code = Some(status_code.into());
        self
    }

    /// Sets the task ID
    pub fn task_id<S: Into<String>>(mut self, task_id: S) -> Self {
        self.task_id = Some(task_id.into());
        self
    }

    /// Sets the task ID as None
    pub fn no_task_id(mut self) -> Self {
        self.task_id = None;
        self
    }

    /// Builds the CalculationSubmissionResult
    ///
    /// # Returns
    /// Result containing the built CalculationSubmissionResult or an error if status_code is missing
    pub fn build(self) -> Result<CalculationSubmissionResult, &'static str> {
        let status_code = self.status_code.ok_or("status_code is required")?;
        
        Ok(CalculationSubmissionResult {
            status_code,
            task_id: self.task_id,
        })
    }
}

impl fmt::Display for CalculationSubmissionResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.task_id {
            Some(task_id) => write!(f, "CalculationSubmissionResult {{ status: {}, task_id: {} }}", 
                                  self.status_code, task_id),
            None => write!(f, "CalculationSubmissionResult {{ status: {} }}", self.status_code),
        }
    }
}

impl Default for CalculationSubmissionResult {
    fn default() -> Self {
        Self {
            status_code: "UNKNOWN".to_string(),
            task_id: None,
        }
    }
}

// Convenience constructors for common status codes
impl CalculationSubmissionResult {
    /// Creates a successful result with a task ID
    pub fn success(task_id: String) -> Self {
        Self::new("SUCCESS".to_string(), Some(task_id))
    }

    /// Creates a successful result without a task ID
    pub fn success_no_task() -> Self {
        Self::with_status_code("SUCCESS".to_string())
    }

    /// Creates an error result with a specific error code
    pub fn error(error_code: String) -> Self {
        Self::with_status_code(error_code)
    }

    /// Creates a pending result with a task ID
    pub fn pending(task_id: String) -> Self {
        Self::new("PENDING".to_string(), Some(task_id))
    }

    /// Creates a rejected result
    pub fn rejected() -> Self {
        Self::with_status_code("REJECTED".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_both_parameters() {
        let result = CalculationSubmissionResult::new(
            "SUCCESS".to_string(), 
            Some("task_123".to_string())
        );
        
        assert_eq!(result.status_code(), "SUCCESS");
        assert_eq!(result.task_id(), Some("task_123"));
        assert!(result.has_task_id());
    }

    #[test]
    fn test_new_with_status_code_only() {
        let result = CalculationSubmissionResult::with_status_code("ERROR".to_string());
        
        assert_eq!(result.status_code(), "ERROR");
        assert_eq!(result.task_id(), None);
        assert!(!result.has_task_id());
    }

    #[test]
    fn test_setters() {
        let mut result = CalculationSubmissionResult::default();
        
        result.set_status_code("PENDING".to_string());
        result.set_task_id(Some("new_task".to_string()));
        
        assert_eq!(result.status_code(), "PENDING");
        assert_eq!(result.task_id(), Some("new_task"));
    }

    #[test]
    fn test_is_success() {
        assert!(CalculationSubmissionResult::with_status_code("SUCCESS".to_string()).is_success());
        assert!(CalculationSubmissionResult::with_status_code("OK".to_string()).is_success());
        assert!(CalculationSubmissionResult::with_status_code("200".to_string()).is_success());
        assert!(CalculationSubmissionResult::with_status_code("201".to_string()).is_success());
        assert!(!CalculationSubmissionResult::with_status_code("ERROR".to_string()).is_success());
        assert!(!CalculationSubmissionResult::with_status_code("404".to_string()).is_success());
    }

    #[test]
    fn test_is_error() {
        assert!(CalculationSubmissionResult::with_status_code("ERROR".to_string()).is_error());
        assert!(CalculationSubmissionResult::with_status_code("400".to_string()).is_error());
        assert!(CalculationSubmissionResult::with_status_code("404".to_string()).is_error());
        assert!(CalculationSubmissionResult::with_status_code("500".to_string()).is_error());
        assert!(!CalculationSubmissionResult::with_status_code("SUCCESS".to_string()).is_error());
        assert!(!CalculationSubmissionResult::with_status_code("200".to_string()).is_error());
    }

    #[test]
    fn test_into_task_id() {
        let result = CalculationSubmissionResult::new(
            "SUCCESS".to_string(), 
            Some("task_456".to_string())
        );
        
        let task_id = result.into_task_id();
        assert_eq!(task_id, Some("task_456".to_string()));
    }

    #[test]
    fn test_builder_pattern() {
        let result = CalculationSubmissionResult::builder()
            .status_code("PROCESSING")
            .task_id("builder_task")
            .build()
            .unwrap();
        
        assert_eq!(result.status_code(), "PROCESSING");
        assert_eq!(result.task_id(), Some("builder_task"));
    }

    #[test]
    fn test_builder_without_task_id() {
        let result = CalculationSubmissionResult::builder()
            .status_code("ERROR")
            .no_task_id()
            .build()
            .unwrap();
        
        assert_eq!(result.status_code(), "ERROR");
        assert_eq!(result.task_id(), None);
    }

    #[test]
    fn test_builder_missing_status_code() {
        let result = CalculationSubmissionResult::builder()
            .task_id("some_task")
            .build();
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "status_code is required");
    }

    #[test]
    fn test_convenience_constructors() {
        let success = CalculationSubmissionResult::success("task_success".to_string());
        assert_eq!(success.status_code(), "SUCCESS");
        assert_eq!(success.task_id(), Some("task_success"));
        assert!(success.is_success());

        let success_no_task = CalculationSubmissionResult::success_no_task();
        assert_eq!(success_no_task.status_code(), "SUCCESS");
        assert_eq!(success_no_task.task_id(), None);
        assert!(success_no_task.is_success());

        let error = CalculationSubmissionResult::error("VALIDATION_ERROR".to_string());
        assert_eq!(error.status_code(), "VALIDATION_ERROR");
        assert_eq!(error.task_id(), None);

        let pending = CalculationSubmissionResult::pending("task_pending".to_string());
        assert_eq!(pending.status_code(), "PENDING");
        assert_eq!(pending.task_id(), Some("task_pending"));

        let rejected = CalculationSubmissionResult::rejected();
        assert_eq!(rejected.status_code(), "REJECTED");
        assert_eq!(rejected.task_id(), None);
    }

    #[test]
    fn test_display() {
        let with_task = CalculationSubmissionResult::new(
            "SUCCESS".to_string(), 
            Some("task_123".to_string())
        );
        assert_eq!(
            format!("{}", with_task),
            "CalculationSubmissionResult { status: SUCCESS, task_id: task_123 }"
        );

        let without_task = CalculationSubmissionResult::with_status_code("ERROR".to_string());
        assert_eq!(
            format!("{}", without_task),
            "CalculationSubmissionResult { status: ERROR }"
        );
    }

    #[test]
    fn test_default() {
        let default = CalculationSubmissionResult::default();
        assert_eq!(default.status_code(), "UNKNOWN");
        assert_eq!(default.task_id(), None);
    }

    #[test]
    fn test_clone_and_equality() {
        let original = CalculationSubmissionResult::new(
            "SUCCESS".to_string(), 
            Some("task_clone".to_string())
        );
        let cloned = original.clone();
        
        assert_eq!(original, cloned);
        assert_eq!(original.status_code(), cloned.status_code());
        assert_eq!(original.task_id(), cloned.task_id());
    }

    #[test]
    fn test_serialization() {
        let result = CalculationSubmissionResult::new(
            "SUCCESS".to_string(), 
            Some("task_serialize".to_string())
        );
        
        // Test that it can be serialized and deserialized
        let json = serde_json::to_string(&result).unwrap();
        let deserialized: CalculationSubmissionResult = serde_json::from_str(&json).unwrap();
        
        assert_eq!(result, deserialized);
    }
}
