//! Unit tests for the Stats model
//!
//! This module contains comprehensive tests for the Stats struct,
//! covering all methods, edge cases, and validation scenarios.

#[cfg(test)]
mod tests {
    use super::super::stats::Stats;
    use crate::models::task::TaskReport;
    use crate::errors::TaskError;

    /// Helper function to create a sample task report
    fn create_sample_task_report(task_id: &str, client_id: &str) -> TaskReport {
        TaskReport::new(task_id.to_string(), client_id.to_string())
    }

    /// Helper function to create a stats instance with sample data
    fn create_sample_stats() -> Stats {
        let mut stats = Stats::new();
        stats.set_nbr_idle_tasks(5);
        stats.set_nbr_running_tasks(3);
        stats.set_nbr_finished_tasks(10);
        stats.set_nbr_stopped_tasks(2);
        stats.set_nbr_terminated_tasks(1);
        stats.set_nbr_error_tasks(1);
        stats.set_nbr_running_threads(4);
        stats.set_nbr_queued_threads(2);
        stats.set_nbr_finished_threads(15);
        stats
    }

    #[test]
    fn test_new_stats() {
        let stats = Stats::new();
        
        assert_eq!(stats.get_nbr_idle_tasks(), 0);
        assert_eq!(stats.get_nbr_running_tasks(), 0);
        assert_eq!(stats.get_nbr_finished_tasks(), 0);
        assert_eq!(stats.get_nbr_stopped_tasks(), 0);
        assert_eq!(stats.get_nbr_terminated_tasks(), 0);
        assert_eq!(stats.get_nbr_error_tasks(), 0);
        assert_eq!(stats.get_nbr_running_threads(), 0);
        assert_eq!(stats.get_nbr_queued_threads(), 0);
        assert_eq!(stats.get_nbr_finished_threads(), 0);
        assert!(stats.get_task_reports().is_empty());
    }

    #[test]
    fn test_default_stats() {
        let stats = Stats::default();
        let new_stats = Stats::new();
        
        assert_eq!(stats, new_stats);
    }

    #[test]
    fn test_idle_tasks_getters_setters() {
        let mut stats = Stats::new();
        
        assert_eq!(stats.get_nbr_idle_tasks(), 0);
        
        stats.set_nbr_idle_tasks(42);
        assert_eq!(stats.get_nbr_idle_tasks(), 42);
        
        stats.set_nbr_idle_tasks(0);
        assert_eq!(stats.get_nbr_idle_tasks(), 0);
    }

    #[test]
    fn test_running_tasks_getters_setters() {
        let mut stats = Stats::new();
        
        assert_eq!(stats.get_nbr_running_tasks(), 0);
        
        stats.set_nbr_running_tasks(15);
        assert_eq!(stats.get_nbr_running_tasks(), 15);
    }

    #[test]
    fn test_finished_tasks_getters_setters() {
        let mut stats = Stats::new();
        
        assert_eq!(stats.get_nbr_finished_tasks(), 0);
        
        stats.set_nbr_finished_tasks(100);
        assert_eq!(stats.get_nbr_finished_tasks(), 100);
    }

    #[test]
    fn test_stopped_tasks_getters_setters() {
        let mut stats = Stats::new();
        
        assert_eq!(stats.get_nbr_stopped_tasks(), 0);
        
        stats.set_nbr_stopped_tasks(5);
        assert_eq!(stats.get_nbr_stopped_tasks(), 5);
    }

    #[test]
    fn test_terminated_tasks_getters_setters() {
        let mut stats = Stats::new();
        
        assert_eq!(stats.get_nbr_terminated_tasks(), 0);
        
        stats.set_nbr_terminated_tasks(3);
        assert_eq!(stats.get_nbr_terminated_tasks(), 3);
    }

    #[test]
    fn test_error_tasks_getters_setters() {
        let mut stats = Stats::new();
        
        assert_eq!(stats.get_nbr_error_tasks(), 0);
        
        stats.set_nbr_error_tasks(7);
        assert_eq!(stats.get_nbr_error_tasks(), 7);
    }

    #[test]
    fn test_running_threads_getters_setters() {
        let mut stats = Stats::new();
        
        assert_eq!(stats.get_nbr_running_threads(), 0);
        
        stats.set_nbr_running_threads(8);
        assert_eq!(stats.get_nbr_running_threads(), 8);
    }

    #[test]
    fn test_queued_threads_getters_setters() {
        let mut stats = Stats::new();
        
        assert_eq!(stats.get_nbr_queued_threads(), 0);
        
        stats.set_nbr_queued_threads(12);
        assert_eq!(stats.get_nbr_queued_threads(), 12);
    }

    #[test]
    fn test_finished_threads_getters_setters() {
        let mut stats = Stats::new();
        
        assert_eq!(stats.get_nbr_finished_threads(), 0);
        
        stats.set_nbr_finished_threads(50);
        assert_eq!(stats.get_nbr_finished_threads(), 50);
    }

    #[test]
    fn test_task_reports_getters_setters() {
        let mut stats = Stats::new();
        
        assert!(stats.get_task_reports().is_empty());
        
        let reports = vec![
            create_sample_task_report("task1", "client1"),
            create_sample_task_report("task2", "client2"),
        ];
        
        stats.set_task_reports(reports.clone());
        assert_eq!(stats.get_task_reports().len(), 2);
        assert_eq!(stats.get_task_reports()[0].get_task_id(), "task1");
        assert_eq!(stats.get_task_reports()[1].get_task_id(), "task2");
    }

    #[test]
    fn test_add_task_report() {
        let mut stats = Stats::new();
        
        let report1 = create_sample_task_report("task1", "client1");
        let report2 = create_sample_task_report("task2", "client2");
        
        stats.add_task_report(report1);
        assert_eq!(stats.get_task_reports().len(), 1);
        assert_eq!(stats.get_task_reports()[0].get_task_id(), "task1");
        
        stats.add_task_report(report2);
        assert_eq!(stats.get_task_reports().len(), 2);
        assert_eq!(stats.get_task_reports()[1].get_task_id(), "task2");
    }

    #[test]
    fn test_remove_task_report() {
        let mut stats = Stats::new();
        
        let report1 = create_sample_task_report("task1", "client1");
        let report2 = create_sample_task_report("task2", "client2");
        
        stats.add_task_report(report1);
        stats.add_task_report(report2);
        assert_eq!(stats.get_task_reports().len(), 2);
        
        // Remove existing task
        let removed = stats.remove_task_report("task1");
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().get_task_id(), "task1");
        assert_eq!(stats.get_task_reports().len(), 1);
        assert_eq!(stats.get_task_reports()[0].get_task_id(), "task2");
        
        // Try to remove non-existing task
        let not_found = stats.remove_task_report("task3");
        assert!(not_found.is_none());
        assert_eq!(stats.get_task_reports().len(), 1);
    }

    #[test]
    fn test_find_task_report() {
        let mut stats = Stats::new();
        
        let report1 = create_sample_task_report("task1", "client1");
        let report2 = create_sample_task_report("task2", "client2");
        
        stats.add_task_report(report1);
        stats.add_task_report(report2);
        
        // Find existing task
        let found = stats.find_task_report("task1");
        assert!(found.is_some());
        assert_eq!(found.unwrap().get_task_id(), "task1");
        
        // Try to find non-existing task
        let not_found = stats.find_task_report("task3");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_find_task_report_mut() {
        let mut stats = Stats::new();
        
        let report = create_sample_task_report("task1", "client1");
        stats.add_task_report(report);
        
        // Find and modify existing task
        let found = stats.find_task_report_mut("task1");
        assert!(found.is_some());
        
        let task_report = found.unwrap();
        task_report.set_client_id("new_client".to_string());
        
        // Verify the change
        let found_again = stats.find_task_report("task1");
        assert_eq!(found_again.unwrap().get_client_id(), "new_client");
    }

    #[test]
    fn test_get_total_tasks() {
        let stats = create_sample_stats();
        
        // 5 + 3 + 10 + 2 + 1 + 1 = 22
        assert_eq!(stats.get_total_tasks(), 22);
    }

    #[test]
    fn test_get_total_threads() {
        let stats = create_sample_stats();
        
        // 4 + 2 + 15 = 21
        assert_eq!(stats.get_total_threads(), 21);
    }

    #[test]
    fn test_get_active_tasks() {
        let stats = create_sample_stats();
        
        // 5 + 3 = 8
        assert_eq!(stats.get_active_tasks(), 8);
    }

    #[test]
    fn test_get_completed_tasks() {
        let stats = create_sample_stats();
        
        // 10 + 2 + 1 + 1 = 14
        assert_eq!(stats.get_completed_tasks(), 14);
    }

    #[test]
    fn test_has_activity() {
        let mut stats = Stats::new();
        
        // No activity initially
        assert!(!stats.has_activity());
        
        // Add idle tasks
        stats.set_nbr_idle_tasks(1);
        assert!(stats.has_activity());
        
        stats.set_nbr_idle_tasks(0);
        assert!(!stats.has_activity());
        
        // Add running tasks
        stats.set_nbr_running_tasks(1);
        assert!(stats.has_activity());
        
        stats.set_nbr_running_tasks(0);
        assert!(!stats.has_activity());
        
        // Add running threads
        stats.set_nbr_running_threads(1);
        assert!(stats.has_activity());
        
        stats.set_nbr_running_threads(0);
        assert!(!stats.has_activity());
        
        // Add queued threads
        stats.set_nbr_queued_threads(1);
        assert!(stats.has_activity());
    }

    #[test]
    fn test_update_task_counts() {
        let mut stats = Stats::new();
        
        stats.update_task_counts(1, 2, 3, 4, 5, 6);
        
        assert_eq!(stats.get_nbr_idle_tasks(), 1);
        assert_eq!(stats.get_nbr_running_tasks(), 2);
        assert_eq!(stats.get_nbr_finished_tasks(), 3);
        assert_eq!(stats.get_nbr_stopped_tasks(), 4);
        assert_eq!(stats.get_nbr_terminated_tasks(), 5);
        assert_eq!(stats.get_nbr_error_tasks(), 6);
    }

    #[test]
    fn test_update_thread_counts() {
        let mut stats = Stats::new();
        
        stats.update_thread_counts(10, 20, 30);
        
        assert_eq!(stats.get_nbr_running_threads(), 10);
        assert_eq!(stats.get_nbr_queued_threads(), 20);
        assert_eq!(stats.get_nbr_finished_threads(), 30);
    }

    #[test]
    fn test_clear() {
        let mut stats = create_sample_stats();
        stats.add_task_report(create_sample_task_report("task1", "client1"));
        
        // Verify stats has data
        assert!(stats.get_total_tasks() > 0);
        assert!(stats.get_total_threads() > 0);
        assert!(!stats.get_task_reports().is_empty());
        
        stats.clear();
        
        // Verify all data is cleared
        assert_eq!(stats.get_nbr_idle_tasks(), 0);
        assert_eq!(stats.get_nbr_running_tasks(), 0);
        assert_eq!(stats.get_nbr_finished_tasks(), 0);
        assert_eq!(stats.get_nbr_stopped_tasks(), 0);
        assert_eq!(stats.get_nbr_terminated_tasks(), 0);
        assert_eq!(stats.get_nbr_error_tasks(), 0);
        assert_eq!(stats.get_nbr_running_threads(), 0);
        assert_eq!(stats.get_nbr_queued_threads(), 0);
        assert_eq!(stats.get_nbr_finished_threads(), 0);
        assert!(stats.get_task_reports().is_empty());
    }

    #[test]
    fn test_validate_success() {
        let mut stats = create_sample_stats();
        
        // Add valid task reports
        stats.add_task_report(create_sample_task_report("task1", "client1"));
        stats.add_task_report(create_sample_task_report("task2", "client2"));
        
        assert!(stats.validate().is_ok());
    }

    #[test]
    fn test_validate_too_many_reports() {
        let mut stats = Stats::new();
        stats.set_nbr_idle_tasks(1); // Total tasks = 1
        
        // Add more reports than total tasks
        stats.add_task_report(create_sample_task_report("task1", "client1"));
        stats.add_task_report(create_sample_task_report("task2", "client2"));
        
        let result = stats.validate();
        assert!(result.is_err());
        
        if let Err(e) = result {
            match e {
                crate::errors::AppError::Task(TaskError::TaskInvalidState { current_state }) => {
                    assert!(current_state.contains("Task report count"));
                    assert!(current_state.contains("exceeds total task count"));
                }
                _ => panic!("Expected TaskInvalidState error, got: {:?}", e),
            }
        }
    }

    #[test]
    fn test_validate_with_zero_tasks() {
        let mut stats = Stats::new();
        
        // Add reports when total tasks is 0 - this should be allowed
        stats.add_task_report(create_sample_task_report("task1", "client1"));
        
        assert!(stats.validate().is_ok());
    }

    #[test]
    fn test_validate_invalid_task_report() {
        let mut stats = create_sample_stats();
        
        // Create an invalid task report
        let mut invalid_report = create_sample_task_report("", "client1"); // Empty task ID
        stats.add_task_report(invalid_report);
        
        let result = stats.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_display() {
        let stats = create_sample_stats();
        let display_str = format!("{}", stats);
        
        assert!(display_str.contains("Stats"));
        assert!(display_str.contains("5/3/10/2/1/1")); // task counts
        assert!(display_str.contains("4/2/15")); // thread counts
        assert!(display_str.contains("reports: 0")); // no reports added
    }

    #[test]
    fn test_clone() {
        let mut original = create_sample_stats();
        original.add_task_report(create_sample_task_report("task1", "client1"));
        
        let cloned = original.clone();
        
        assert_eq!(original, cloned);
        assert_eq!(original.get_total_tasks(), cloned.get_total_tasks());
        assert_eq!(original.get_task_reports().len(), cloned.get_task_reports().len());
    }

    #[test]
    fn test_partial_eq() {
        let stats1 = create_sample_stats();
        let stats2 = create_sample_stats();
        let mut stats3 = create_sample_stats();
        stats3.set_nbr_idle_tasks(999);
        
        assert_eq!(stats1, stats2);
        assert_ne!(stats1, stats3);
    }

    #[test]
    fn test_debug() {
        let stats = create_sample_stats();
        let debug_str = format!("{:?}", stats);
        
        assert!(debug_str.contains("Stats"));
        assert!(debug_str.contains("nbr_idle_tasks"));
    }

    #[test]
    fn test_serialize_deserialize() {
        let original = create_sample_stats();
        
        // Test serialization
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        assert!(!serialized.is_empty());
        
        // Test deserialization
        let deserialized: Stats = serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_large_numbers() {
        let mut stats = Stats::new();
        
        // Test with large numbers
        stats.set_nbr_idle_tasks(u64::MAX);
        stats.set_nbr_running_threads(u32::MAX);
        
        assert_eq!(stats.get_nbr_idle_tasks(), u64::MAX);
        assert_eq!(stats.get_nbr_running_threads(), u32::MAX);
    }

    #[test]
    fn test_mutable_access() {
        let mut stats = Stats::new();
        
        // Test mutable access to task reports
        stats.add_task_report(create_sample_task_report("task1", "client1"));
        
        {
            let reports_mut = stats.get_task_reports_mut();
            reports_mut.push(create_sample_task_report("task2", "client2"));
        }
        
        assert_eq!(stats.get_task_reports().len(), 2);
    }

    #[test]
    fn test_edge_cases() {
        let mut stats = Stats::new();
        
        // Test removing from empty list
        assert!(stats.remove_task_report("nonexistent").is_none());
        
        // Test finding in empty list
        assert!(stats.find_task_report("nonexistent").is_none());
        assert!(stats.find_task_report_mut("nonexistent").is_none());
        
        // Test with empty task reports vector
        assert_eq!(stats.get_total_tasks(), 0);
        assert_eq!(stats.get_total_threads(), 0);
        assert!(!stats.has_activity());
    }
}
