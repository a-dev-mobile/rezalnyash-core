//! Unit tests for CalculationSubmissionResult
//!
//! This module contains comprehensive tests for the CalculationSubmissionResult struct,
//! ensuring all functionality works correctly and maintains compatibility with the Java version.

use super::*;
use serde_json;

#[cfg(test)]
mod calculation_submission_result_tests {
    use super::*;

    #[test]
    fn test_new_constructor_with_both_parameters() {
        let status = "SUCCESS".to_string();
        let task_id = "task_12345".to_string();
        
        let result = CalculationSubmissionResult::new(status.clone(), Some(task_id.clone()));
        
        assert_eq!(result.status_code(), "SUCCESS");
        assert_eq!(result.task_id(), Some("task_12345"));
        assert!(result.has_task_id());
        assert!(result.is_success());
        assert!(!result.is_error());
    }

    #[test]
    fn test_new_constructor_with_none_task_id() {
        let status = "ERROR".to_string();
        
        let result = CalculationSubmissionResult::new(status, None);
        
        assert_eq!(result.status_code(), "ERROR");
        assert_eq!(result.task_id(), None);
        assert!(!result.has_task_id());
        assert!(!result.is_success());
        assert!(result.is_error());
    }

    #[test]
    fn test_with_status_code_constructor() {
        let status = "PENDING".to_string();
        
        let result = CalculationSubmissionResult::with_status_code(status);
        
        assert_eq!(result.status_code(), "PENDING");
        assert_eq!(result.task_id(), None);
        assert!(!result.has_task_id());
        assert!(!result.is_success());
        assert!(!result.is_error());
    }

    #[test]
    fn test_getters_and_setters() {
        let mut result = CalculationSubmissionResult::default();
        
        // Test initial state
        assert_eq!(result.status_code(), "UNKNOWN");
        assert_eq!(result.task_id(), None);
        
        // Test setters
        result.set_status_code("PROCESSING".to_string());
        result.set_task_id(Some("new_task_id".to_string()));
        
        // Test getters after setting
        assert_eq!(result.status_code(), "PROCESSING");
        assert_eq!(result.task_id(), Some("new_task_id"));
        assert!(result.has_task_id());
        
        // Test setting task_id to None
        result.set_task_id(None);
        assert_eq!(result.task_id(), None);
        assert!(!result.has_task_id());
    }

    #[test]
    fn test_is_success_various_status_codes() {
        // Test success cases
        assert!(CalculationSubmissionResult::with_status_code("SUCCESS".to_string()).is_success());
        assert!(CalculationSubmissionResult::with_status_code("success".to_string()).is_success());
        assert!(CalculationSubmissionResult::with_status_code("OK".to_string()).is_success());
        assert!(CalculationSubmissionResult::with_status_code("ok".to_string()).is_success());
        assert!(CalculationSubmissionResult::with_status_code("200".to_string()).is_success());
        assert!(CalculationSubmissionResult::with_status_code("201".to_string()).is_success());
        assert!(CalculationSubmissionResult::with_status_code("202".to_string()).is_success());
        assert!(CalculationSubmissionResult::with_status_code("299".to_string()).is_success());
        
        // Test non-success cases
        assert!(!CalculationSubmissionResult::with_status_code("ERROR".to_string()).is_success());
        assert!(!CalculationSubmissionResult::with_status_code("PENDING".to_string()).is_success());
        assert!(!CalculationSubmissionResult::with_status_code("400".to_string()).is_success());
        assert!(!CalculationSubmissionResult::with_status_code("500".to_string()).is_success());
        assert!(!CalculationSubmissionResult::with_status_code("100".to_string()).is_success());
        assert!(!CalculationSubmissionResult::with_status_code("300".to_string()).is_success());
    }

    #[test]
    fn test_is_error_various_status_codes() {
        // Test error cases
        assert!(CalculationSubmissionResult::with_status_code("ERROR".to_string()).is_error());
        assert!(CalculationSubmissionResult::with_status_code("error".to_string()).is_error());
        assert!(CalculationSubmissionResult::with_status_code("400".to_string()).is_error());
        assert!(CalculationSubmissionResult::with_status_code("404".to_string()).is_error());
        assert!(CalculationSubmissionResult::with_status_code("499".to_string()).is_error());
        assert!(CalculationSubmissionResult::with_status_code("500".to_string()).is_error());
        assert!(CalculationSubmissionResult::with_status_code("503".to_string()).is_error());
        assert!(CalculationSubmissionResult::with_status_code("599".to_string()).is_error());
        
        // Test non-error cases
        assert!(!CalculationSubmissionResult::with_status_code("SUCCESS".to_string()).is_error());
        assert!(!CalculationSubmissionResult::with_status_code("OK".to_string()).is_error());
        assert!(!CalculationSubmissionResult::with_status_code("200".to_string()).is_error());
        assert!(!CalculationSubmissionResult::with_status_code("PENDING".to_string()).is_error());
        assert!(!CalculationSubmissionResult::with_status_code("100".to_string()).is_error());
        assert!(!CalculationSubmissionResult::with_status_code("300".to_string()).is_error());
    }

    #[test]
    fn test_into_task_id() {
        let result_with_task = CalculationSubmissionResult::new(
            "SUCCESS".to_string(), 
            Some("consumed_task".to_string())
        );
        
        let task_id = result_with_task.into_task_id();
        assert_eq!(task_id, Some("consumed_task".to_string()));
        
        let result_without_task = CalculationSubmissionResult::with_status_code("ERROR".to_string());
        let no_task_id = result_without_task.into_task_id();
        assert_eq!(no_task_id, None);
    }

    #[test]
    fn test_builder_pattern_complete() {
        let result = CalculationSubmissionResult::builder()
            .status_code("PROCESSING")
            .task_id("builder_task_123")
            .build()
            .unwrap();
        
        assert_eq!(result.status_code(), "PROCESSING");
        assert_eq!(result.task_id(), Some("builder_task_123"));
        assert!(result.has_task_id());
    }

    #[test]
    fn test_builder_pattern_status_only() {
        let result = CalculationSubmissionResult::builder()
            .status_code("ERROR")
            .no_task_id()
            .build()
            .unwrap();
        
        assert_eq!(result.status_code(), "ERROR");
        assert_eq!(result.task_id(), None);
        assert!(!result.has_task_id());
    }

    #[test]
    fn test_builder_pattern_missing_status_code() {
        let result = CalculationSubmissionResult::builder()
            .task_id("orphan_task")
            .build();
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "status_code is required");
    }

    #[test]
    fn test_builder_pattern_string_conversion() {
        let result = CalculationSubmissionResult::builder()
            .status_code("SUCCESS") // &str
            .task_id("test_task") // &str
            .build()
            .unwrap();
        
        assert_eq!(result.status_code(), "SUCCESS");
        assert_eq!(result.task_id(), Some("test_task"));
    }

    #[test]
    fn test_convenience_constructors() {
        // Test success with task
        let success = CalculationSubmissionResult::success("success_task".to_string());
        assert_eq!(success.status_code(), "SUCCESS");
        assert_eq!(success.task_id(), Some("success_task"));
        assert!(success.is_success());
        assert!(!success.is_error());
        assert!(success.has_task_id());

        // Test success without task
        let success_no_task = CalculationSubmissionResult::success_no_task();
        assert_eq!(success_no_task.status_code(), "SUCCESS");
        assert_eq!(success_no_task.task_id(), None);
        assert!(success_no_task.is_success());
        assert!(!success_no_task.is_error());
        assert!(!success_no_task.has_task_id());

        // Test error
        let error = CalculationSubmissionResult::error("VALIDATION_FAILED".to_string());
        assert_eq!(error.status_code(), "VALIDATION_FAILED");
        assert_eq!(error.task_id(), None);
        assert!(!error.is_success());
        assert!(!error.has_task_id());

        // Test pending
        let pending = CalculationSubmissionResult::pending("pending_task".to_string());
        assert_eq!(pending.status_code(), "PENDING");
        assert_eq!(pending.task_id(), Some("pending_task"));
        assert!(!pending.is_success());
        assert!(!pending.is_error());
        assert!(pending.has_task_id());

        // Test rejected
        let rejected = CalculationSubmissionResult::rejected();
        assert_eq!(rejected.status_code(), "REJECTED");
        assert_eq!(rejected.task_id(), None);
        assert!(!rejected.is_success());
        assert!(!rejected.is_error());
        assert!(!rejected.has_task_id());
    }

    #[test]
    fn test_display_formatting() {
        let with_task = CalculationSubmissionResult::new(
            "SUCCESS".to_string(), 
            Some("display_task".to_string())
        );
        let display_with_task = format!("{}", with_task);
        assert_eq!(display_with_task, "CalculationSubmissionResult { status: SUCCESS, task_id: display_task }");

        let without_task = CalculationSubmissionResult::with_status_code("ERROR".to_string());
        let display_without_task = format!("{}", without_task);
        assert_eq!(display_without_task, "CalculationSubmissionResult { status: ERROR }");
    }

    #[test]
    fn test_default_implementation() {
        let default = CalculationSubmissionResult::default();
        
        assert_eq!(default.status_code(), "UNKNOWN");
        assert_eq!(default.task_id(), None);
        assert!(!default.has_task_id());
        assert!(!default.is_success());
        assert!(!default.is_error());
    }

    #[test]
    fn test_clone_and_equality() {
        let original = CalculationSubmissionResult::new(
            "SUCCESS".to_string(), 
            Some("clone_test".to_string())
        );
        let cloned = original.clone();
        
        // Test equality
        assert_eq!(original, cloned);
        
        // Test that they have the same values
        assert_eq!(original.status_code(), cloned.status_code());
        assert_eq!(original.task_id(), cloned.task_id());
        assert_eq!(original.has_task_id(), cloned.has_task_id());
        assert_eq!(original.is_success(), cloned.is_success());
        assert_eq!(original.is_error(), cloned.is_error());
    }

    #[test]
    fn test_hash_consistency() {
        use std::collections::HashMap;
        
        let result1 = CalculationSubmissionResult::new(
            "SUCCESS".to_string(), 
            Some("hash_test".to_string())
        );
        let result2 = CalculationSubmissionResult::new(
            "SUCCESS".to_string(), 
            Some("hash_test".to_string())
        );
        
        // Equal objects should have the same hash
        let mut map = HashMap::new();
        map.insert(result1.clone(), "value1");
        map.insert(result2.clone(), "value2");
        
        // Should only have one entry since they're equal
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&result1), Some(&"value2"));
    }

    #[test]
    fn test_serialization_deserialization() {
        let original = CalculationSubmissionResult::new(
            "SUCCESS".to_string(), 
            Some("serialize_test".to_string())
        );
        
        // Serialize to JSON
        let json = serde_json::to_string(&original).expect("Failed to serialize");
        
        // Deserialize from JSON
        let deserialized: CalculationSubmissionResult = serde_json::from_str(&json)
            .expect("Failed to deserialize");
        
        // Should be equal
        assert_eq!(original, deserialized);
        assert_eq!(original.status_code(), deserialized.status_code());
        assert_eq!(original.task_id(), deserialized.task_id());
    }

    #[test]
    fn test_serialization_without_task_id() {
        let original = CalculationSubmissionResult::with_status_code("ERROR".to_string());
        
        // Serialize to JSON
        let json = serde_json::to_string(&original).expect("Failed to serialize");
        
        // Deserialize from JSON
        let deserialized: CalculationSubmissionResult = serde_json::from_str(&json)
            .expect("Failed to deserialize");
        
        // Should be equal
        assert_eq!(original, deserialized);
        assert_eq!(original.status_code(), deserialized.status_code());
        assert_eq!(original.task_id(), deserialized.task_id());
        assert_eq!(original.task_id(), None);
    }

    #[test]
    fn test_json_structure() {
        let result = CalculationSubmissionResult::new(
            "SUCCESS".to_string(), 
            Some("json_test".to_string())
        );
        
        let json = serde_json::to_string(&result).expect("Failed to serialize");
        let json_value: serde_json::Value = serde_json::from_str(&json)
            .expect("Failed to parse JSON");
        
        // Check JSON structure
        assert_eq!(json_value["status_code"], "SUCCESS");
        assert_eq!(json_value["task_id"], "json_test");
    }

    #[test]
    fn test_json_structure_without_task_id() {
        let result = CalculationSubmissionResult::with_status_code("ERROR".to_string());
        
        let json = serde_json::to_string(&result).expect("Failed to serialize");
        let json_value: serde_json::Value = serde_json::from_str(&json)
            .expect("Failed to parse JSON");
        
        // Check JSON structure
        assert_eq!(json_value["status_code"], "ERROR");
        assert_eq!(json_value["task_id"], serde_json::Value::Null);
    }

    #[test]
    fn test_edge_cases() {
        // Empty strings
        let empty_status = CalculationSubmissionResult::with_status_code("".to_string());
        assert_eq!(empty_status.status_code(), "");
        assert!(!empty_status.is_success());
        assert!(!empty_status.is_error());

        let empty_task = CalculationSubmissionResult::new("SUCCESS".to_string(), Some("".to_string()));
        assert_eq!(empty_task.task_id(), Some(""));
        assert!(empty_task.has_task_id());

        // Very long strings
        let long_status = "A".repeat(1000);
        let long_task = "B".repeat(1000);
        let long_result = CalculationSubmissionResult::new(long_status.clone(), Some(long_task.clone()));
        assert_eq!(long_result.status_code(), long_status);
        assert_eq!(long_result.task_id(), Some(long_task.as_str()));
    }

    #[test]
    fn test_builder_reuse() {
        let builder = CalculationSubmissionResult::builder();
        
        // Build first result
        let result1 = builder
            .status_code("SUCCESS")
            .task_id("task1")
            .build()
            .unwrap();
        
        // Create new builder for second result
        let result2 = CalculationSubmissionResult::builder()
            .status_code("ERROR")
            .no_task_id()
            .build()
            .unwrap();
        
        assert_eq!(result1.status_code(), "SUCCESS");
        assert_eq!(result1.task_id(), Some("task1"));
        assert_eq!(result2.status_code(), "ERROR");
        assert_eq!(result2.task_id(), None);
    }

    #[test]
    fn test_debug_formatting() {
        let result = CalculationSubmissionResult::new(
            "DEBUG_TEST".to_string(), 
            Some("debug_task".to_string())
        );
        
        let debug_output = format!("{:?}", result);
        assert!(debug_output.contains("DEBUG_TEST"));
        assert!(debug_output.contains("debug_task"));
        assert!(debug_output.contains("CalculationSubmissionResult"));
    }

    #[test]
    fn test_mutability() {
        let mut result = CalculationSubmissionResult::default();
        
        // Test multiple mutations
        result.set_status_code("STEP1".to_string());
        assert_eq!(result.status_code(), "STEP1");
        
        result.set_task_id(Some("task_step1".to_string()));
        assert_eq!(result.task_id(), Some("task_step1"));
        
        result.set_status_code("STEP2".to_string());
        assert_eq!(result.status_code(), "STEP2");
        
        result.set_task_id(Some("task_step2".to_string()));
        assert_eq!(result.task_id(), Some("task_step2"));
        
        result.set_task_id(None);
        assert_eq!(result.task_id(), None);
    }

    #[test]
    fn test_java_compatibility_scenarios() {
        // Test scenarios that mirror Java usage patterns
        
        // Java: new CalculationSubmissionResult("SUCCESS", "task123")
        let java_style1 = CalculationSubmissionResult::new(
            "SUCCESS".to_string(), 
            Some("task123".to_string())
        );
        assert_eq!(java_style1.status_code(), "SUCCESS");
        assert_eq!(java_style1.task_id(), Some("task123"));
        
        // Java: new CalculationSubmissionResult("ERROR")
        let java_style2 = CalculationSubmissionResult::with_status_code("ERROR".to_string());
        assert_eq!(java_style2.status_code(), "ERROR");
        assert_eq!(java_style2.task_id(), None);
        
        // Java: result.setStatusCode("UPDATED")
        let mut java_style3 = java_style1.clone();
        java_style3.set_status_code("UPDATED".to_string());
        assert_eq!(java_style3.status_code(), "UPDATED");
        
        // Java: result.setTaskId("new_task")
        java_style3.set_task_id(Some("new_task".to_string()));
        assert_eq!(java_style3.task_id(), Some("new_task"));
    }
}
