#[cfg(test)]
mod tests {
    use crate::models::performance_thresholds::PerformanceThresholds;
    use crate::errors::core_errors::CoreError;

    #[test]
    fn test_new_performance_thresholds() {
        let thresholds = PerformanceThresholds::new();
        assert_eq!(thresholds.get_max_simultaneous_tasks(), 1);
        assert_eq!(thresholds.get_max_simultaneous_threads(), 0);
        assert_eq!(thresholds.get_thread_check_interval(), 0);
    }

    #[test]
    fn test_default() {
        let thresholds = PerformanceThresholds::default();
        assert_eq!(thresholds.get_max_simultaneous_tasks(), 1);
        assert_eq!(thresholds.get_max_simultaneous_threads(), 0);
        assert_eq!(thresholds.get_thread_check_interval(), 0);
    }

    #[test]
    fn test_with_config() {
        let thresholds = PerformanceThresholds::with_config(4, 1000);
        assert_eq!(thresholds.get_max_simultaneous_tasks(), 1);
        assert_eq!(thresholds.get_max_simultaneous_threads(), 4);
        assert_eq!(thresholds.get_thread_check_interval(), 1000);
    }

    #[test]
    fn test_set_thread_check_interval_valid() {
        let mut thresholds = PerformanceThresholds::new();
        let result = thresholds.set_thread_check_interval(5000);
        assert!(result.is_ok());
        assert_eq!(thresholds.get_thread_check_interval(), 5000);
    }

    #[test]
    fn test_set_thread_check_interval_too_large() {
        let mut thresholds = PerformanceThresholds::new();
        let result = thresholds.set_thread_check_interval(25 * 60 * 60 * 1000); // 25 hours
        assert!(result.is_err());
        assert_eq!(thresholds.get_thread_check_interval(), 0); // Should remain unchanged
        
        match result {
            Err(CoreError::InvalidInput { details }) => {
                assert!(details.contains("exceeds maximum allowed"));
            }
            _ => panic!("Expected InvalidInput error"),
        }
    }

    #[test]
    fn test_set_max_simultaneous_threads_valid() {
        let mut thresholds = PerformanceThresholds::new();
        let result = thresholds.set_max_simultaneous_threads(8);
        assert!(result.is_ok());
        assert_eq!(thresholds.get_max_simultaneous_threads(), 8);
    }

    #[test]
    fn test_set_max_simultaneous_threads_zero() {
        let mut thresholds = PerformanceThresholds::new();
        let result = thresholds.set_max_simultaneous_threads(0);
        assert!(result.is_err());
        assert_eq!(thresholds.get_max_simultaneous_threads(), 0); // Should remain unchanged
        
        match result {
            Err(CoreError::InvalidInput { details }) => {
                assert!(details.contains("must be greater than 0"));
            }
            _ => panic!("Expected InvalidInput error"),
        }
    }

    #[test]
    fn test_set_max_simultaneous_threads_too_large() {
        let mut thresholds = PerformanceThresholds::new();
        let result = thresholds.set_max_simultaneous_threads(1001);
        assert!(result.is_err());
        assert_eq!(thresholds.get_max_simultaneous_threads(), 0); // Should remain unchanged
        
        match result {
            Err(CoreError::InvalidInput { details }) => {
                assert!(details.contains("exceeds reasonable limit"));
            }
            _ => panic!("Expected InvalidInput error"),
        }
    }

    #[test]
    fn test_set_max_simultaneous_tasks_valid() {
        let mut thresholds = PerformanceThresholds::new();
        let result = thresholds.set_max_simultaneous_tasks(100);
        assert!(result.is_ok());
        assert_eq!(thresholds.get_max_simultaneous_tasks(), 100);
    }

    #[test]
    fn test_set_max_simultaneous_tasks_zero() {
        let mut thresholds = PerformanceThresholds::new();
        let result = thresholds.set_max_simultaneous_tasks(0);
        assert!(result.is_err());
        assert_eq!(thresholds.get_max_simultaneous_tasks(), 1); // Should remain unchanged
        
        match result {
            Err(CoreError::InvalidInput { details }) => {
                assert!(details.contains("must be greater than 0"));
            }
            _ => panic!("Expected InvalidInput error"),
        }
    }

    #[test]
    fn test_set_max_simultaneous_tasks_too_large() {
        let mut thresholds = PerformanceThresholds::new();
        let result = thresholds.set_max_simultaneous_tasks(10001);
        assert!(result.is_err());
        assert_eq!(thresholds.get_max_simultaneous_tasks(), 1); // Should remain unchanged
        
        match result {
            Err(CoreError::InvalidInput { details }) => {
                assert!(details.contains("exceeds reasonable limit"));
            }
            _ => panic!("Expected InvalidInput error"),
        }
    }

    #[test]
    fn test_builder_pattern_valid() {
        let result = PerformanceThresholds::new()
            .max_simultaneous_tasks(50)
            .and_then(|t| t.max_simultaneous_threads(4))
            .and_then(|t| t.thread_check_interval(2000));

        assert!(result.is_ok());
        let thresholds = result.unwrap();
        assert_eq!(thresholds.get_max_simultaneous_tasks(), 50);
        assert_eq!(thresholds.get_max_simultaneous_threads(), 4);
        assert_eq!(thresholds.get_thread_check_interval(), 2000);
    }

    #[test]
    fn test_builder_pattern_invalid() {
        let result = PerformanceThresholds::new()
            .max_simultaneous_tasks(0); // Invalid

        assert!(result.is_err());
    }

    #[test]
    fn test_validate_consistent_config() {
        let thresholds = PerformanceThresholds::with_config(4, 1000);
        let result = thresholds.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_inconsistent_threads_without_interval() {
        let mut thresholds = PerformanceThresholds::new();
        thresholds.set_max_simultaneous_threads(4).unwrap();
        // thread_check_interval remains 0
        
        let result = thresholds.validate();
        assert!(result.is_err());
        
        match result {
            Err(CoreError::InvalidConfiguration { message }) => {
                assert!(message.contains("thread check interval is 0"));
            }
            _ => panic!("Expected InvalidConfiguration error"),
        }
    }

    #[test]
    fn test_validate_inconsistent_interval_without_threads() {
        let mut thresholds = PerformanceThresholds::new();
        thresholds.set_thread_check_interval(1000).unwrap();
        // max_simultaneous_threads remains 0
        
        let result = thresholds.validate();
        assert!(result.is_err());
        
        match result {
            Err(CoreError::InvalidConfiguration { message }) => {
                assert!(message.contains("max simultaneous threads is 0"));
            }
            _ => panic!("Expected InvalidConfiguration error"),
        }
    }

    #[test]
    fn test_validate_both_zero_is_valid() {
        let thresholds = PerformanceThresholds::new(); // Both threads and interval are 0
        let result = thresholds.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_summary() {
        let thresholds = PerformanceThresholds::with_config(8, 5000);
        let summary = thresholds.summary();
        assert_eq!(summary, "PerformanceThresholds(tasks: 1, threads: 8, interval: 5000ms)");
    }

    #[test]
    fn test_is_threading_enabled() {
        let thresholds_disabled = PerformanceThresholds::new();
        assert!(!thresholds_disabled.is_threading_enabled());

        let thresholds_enabled = PerformanceThresholds::with_config(4, 1000);
        assert!(thresholds_enabled.is_threading_enabled());

        let mut thresholds_partial = PerformanceThresholds::new();
        thresholds_partial.set_max_simultaneous_threads(4).unwrap();
        assert!(!thresholds_partial.is_threading_enabled()); // Missing interval

        let mut thresholds_partial2 = PerformanceThresholds::new();
        thresholds_partial2.set_thread_check_interval(1000).unwrap();
        assert!(!thresholds_partial2.is_threading_enabled()); // Missing threads
    }

    #[test]
    fn test_get_thread_check_duration() {
        let thresholds = PerformanceThresholds::with_config(4, 2500);
        let duration = thresholds.get_thread_check_duration();
        assert_eq!(duration, std::time::Duration::from_millis(2500));
    }

    #[test]
    fn test_max_concurrent_operations() {
        // Threading disabled
        let thresholds_no_threading = PerformanceThresholds::new();
        assert_eq!(thresholds_no_threading.max_concurrent_operations(), 1);

        // Threading enabled
        let mut thresholds_with_threading = PerformanceThresholds::with_config(4, 1000);
        thresholds_with_threading.set_max_simultaneous_tasks(10).unwrap();
        assert_eq!(thresholds_with_threading.max_concurrent_operations(), 40); // 10 * 4

        // Only tasks set
        let mut thresholds_tasks_only = PerformanceThresholds::new();
        thresholds_tasks_only.set_max_simultaneous_tasks(5).unwrap();
        assert_eq!(thresholds_tasks_only.max_concurrent_operations(), 5);
    }

    #[test]
    fn test_clone_and_equality() {
        let thresholds1 = PerformanceThresholds::with_config(4, 1000);
        let thresholds2 = thresholds1.clone();
        assert_eq!(thresholds1, thresholds2);

        let thresholds3 = PerformanceThresholds::with_config(8, 1000);
        assert_ne!(thresholds1, thresholds3);
    }

    #[test]
    fn test_serde_serialization() {
        let thresholds = PerformanceThresholds::with_config(4, 2000);
        
        // Test serialization
        let json = serde_json::to_string(&thresholds).expect("Failed to serialize");
        assert!(json.contains("\"max_simultaneous_threads\":4"));
        assert!(json.contains("\"thread_check_interval\":2000"));

        // Test deserialization
        let deserialized: PerformanceThresholds = serde_json::from_str(&json)
            .expect("Failed to deserialize");
        assert_eq!(thresholds, deserialized);
    }

    #[test]
    fn test_boundary_values() {
        let mut thresholds = PerformanceThresholds::new();

        // Test minimum valid values
        assert!(thresholds.set_max_simultaneous_tasks(1).is_ok());
        assert!(thresholds.set_max_simultaneous_threads(1).is_ok());
        assert!(thresholds.set_thread_check_interval(1).is_ok());

        // Test maximum valid values
        assert!(thresholds.set_max_simultaneous_tasks(10000).is_ok());
        assert!(thresholds.set_max_simultaneous_threads(1000).is_ok());
        assert!(thresholds.set_thread_check_interval(24 * 60 * 60 * 1000).is_ok()); // 24 hours

        // Test just over the limits
        assert!(thresholds.set_max_simultaneous_tasks(10001).is_err());
        assert!(thresholds.set_max_simultaneous_threads(1001).is_err());
        assert!(thresholds.set_thread_check_interval(24 * 60 * 60 * 1000 + 1).is_err());
    }

    #[test]
    fn test_error_messages() {
        let mut thresholds = PerformanceThresholds::new();

        // Test specific error messages
        let result = thresholds.set_max_simultaneous_tasks(0);
        assert!(result.is_err());
        if let Err(CoreError::InvalidInput { details }) = result {
            assert_eq!(details, "Maximum simultaneous tasks must be greater than 0");
        }

        let result = thresholds.set_max_simultaneous_threads(0);
        assert!(result.is_err());
        if let Err(CoreError::InvalidInput { details }) = result {
            assert_eq!(details, "Maximum simultaneous threads must be greater than 0");
        }

        let result = thresholds.set_max_simultaneous_tasks(10001);
        assert!(result.is_err());
        if let Err(CoreError::InvalidInput { details }) = result {
            assert_eq!(details, "Maximum simultaneous tasks 10001 exceeds reasonable limit of 10000");
        }

        let result = thresholds.set_max_simultaneous_threads(1001);
        assert!(result.is_err());
        if let Err(CoreError::InvalidInput { details }) = result {
            assert_eq!(details, "Maximum simultaneous threads 1001 exceeds reasonable limit of 1000");
        }
    }

    #[test]
    fn test_java_compatibility() {
        // Test that we maintain the same behavior as the original Java class
        
        // Default constructor behavior
        let thresholds = PerformanceThresholds::new();
        assert_eq!(thresholds.get_max_simultaneous_tasks(), 1); // Java default
        assert_eq!(thresholds.get_max_simultaneous_threads(), 0); // Java default
        assert_eq!(thresholds.get_thread_check_interval(), 0); // Java default

        // Parameterized constructor behavior
        let thresholds = PerformanceThresholds::with_config(5, 3000);
        assert_eq!(thresholds.get_max_simultaneous_tasks(), 1); // Java default maintained
        assert_eq!(thresholds.get_max_simultaneous_threads(), 5);
        assert_eq!(thresholds.get_thread_check_interval(), 3000);

        // Setter behavior (but with validation in Rust)
        let mut thresholds = PerformanceThresholds::new();
        assert!(thresholds.set_max_simultaneous_tasks(42).is_ok());
        assert_eq!(thresholds.get_max_simultaneous_tasks(), 42);
        
        assert!(thresholds.set_max_simultaneous_threads(8).is_ok());
        assert_eq!(thresholds.get_max_simultaneous_threads(), 8);
        
        assert!(thresholds.set_thread_check_interval(1500).is_ok());
        assert_eq!(thresholds.get_thread_check_interval(), 1500);
    }

    #[test]
    fn test_comprehensive_workflow() {
        // Test a complete workflow that might be used in practice
        let mut thresholds = PerformanceThresholds::new();
        
        // Configure for a multi-threaded environment
        thresholds.set_max_simultaneous_tasks(20).unwrap();
        thresholds.set_max_simultaneous_threads(4).unwrap();
        thresholds.set_thread_check_interval(1000).unwrap();
        
        // Validate configuration
        assert!(thresholds.validate().is_ok());
        
        // Check that threading is properly enabled
        assert!(thresholds.is_threading_enabled());
        
        // Verify calculations
        assert_eq!(thresholds.max_concurrent_operations(), 80); // 20 * 4
        
        // Check duration conversion
        assert_eq!(thresholds.get_thread_check_duration(), std::time::Duration::from_secs(1));
        
        // Verify summary
        let summary = thresholds.summary();
        assert!(summary.contains("tasks: 20"));
        assert!(summary.contains("threads: 4"));
        assert!(summary.contains("interval: 1000ms"));
    }
}
