//! Progress Tracker Tests
//!
//! Comprehensive unit tests for the ProgressTracker module, covering
//! progress calculation algorithms, validation, and edge cases.

#[cfg(test)]
mod tests {
    use super::super::progress_tracker::*;
    use crate::models::task::Task;
    use crate::enums::Status;
    use crate::models::calculation_response::CalculationResponse;
    use std::sync::Arc;
    use std::time::{SystemTime, UNIX_EPOCH, Duration};
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn init_logging_for_tests() {
        INIT.call_once(|| {
            use crate::logging::{init::init_logging, structs::LogConfig, enums::LogLevel};
            let config = LogConfig {
                level: LogLevel::Error, // Only show errors in tests
            };
            let _ = init_logging(config);
        });
    }

    /// Mock implementation of PermutationThreadSpawner for testing
    #[derive(Debug)]
    struct MockPermutationThreadSpawner {
        total_threads: i32,
    }

    impl MockPermutationThreadSpawner {
        fn new(total_threads: i32) -> Self {
            Self { total_threads }
        }

        fn set_total_threads(&mut self, total_threads: i32) {
            self.total_threads = total_threads;
        }
    }

    impl PermutationThreadSpawner for MockPermutationThreadSpawner {
        fn get_nbr_total_threads(&self) -> i32 {
            self.total_threads
        }
    }

    /// Helper function to create a task with elapsed time
    fn create_task_with_elapsed_time(id: &str, elapsed_ms: u64) -> Arc<Task> {
        let mut task = Task::new(id.to_string());
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        task.start_time = now - elapsed_ms;
        Arc::new(task)
    }

    /// Helper function to create a task with solution
    fn create_task_with_solution(id: &str) -> Arc<Task> {
        let mut task = Task::new(id.to_string());
        let mut solution = CalculationResponse::new();
        // Create a dummy panel to make has_solution() return true
        use crate::models::calculation_response::FinalTile;
        let dummy_panel = FinalTile::with_params(1, 100.0, 200.0);
        solution.panels = Some(vec![dummy_panel]); // Has solution with at least one panel
        solution.no_fit_panels = Vec::new(); // All fit
        task.solution = Some(solution);
        Arc::new(task)
    }

    #[test]
    fn test_new_progress_tracker() {
        let spawner = Arc::new(MockPermutationThreadSpawner::new(10));
        let task = Arc::new(Task::new("test-task".to_string()));
        let tracker = ProgressTracker::new(
            spawner,
            1000,
            task,
            "wood".to_string(),
        );

        assert_eq!(tracker.get_material(), "wood");
        assert_eq!(tracker.get_total_permutations(), 1000);
        assert_eq!(tracker.get_task().id, "test-task");
    }

    #[test]
    fn test_progress_calculation_without_solution_time_based() {
        let spawner = Arc::new(MockPermutationThreadSpawner::new(1)); // No completed threads
        let task = create_task_with_elapsed_time("test-task", 300_000); // 5 minutes
        let tracker = ProgressTracker::new(
            spawner,
            1000,
            task,
            "wood".to_string(),
        );

        let progress = tracker.get_progress_percentage();
        
        // Time progress: 300,000ms / 600,000ms * 100 = 50%
        // Thread progress: (1-1)/1000 * 100 = 0%
        // Should return max(50, 0) = 50%
        assert_eq!(progress, 50);
    }

    #[test]
    fn test_progress_calculation_without_solution_thread_based() {
        let spawner = Arc::new(MockPermutationThreadSpawner::new(50)); // 49 completed + 1 current
        let task = Arc::new(Task::new("test-task".to_string())); // New task, minimal elapsed time
        let tracker = ProgressTracker::new(
            spawner,
            100,
            task,
            "wood".to_string(),
        );

        let progress = tracker.get_progress_percentage();
        
        // Time progress: very small (new task)
        // Thread progress: (50-1)/100 * 100 = 49%
        // Should return max(~0, 49) = 49%
        assert_eq!(progress, 49);
    }

    #[test]
    fn test_progress_calculation_with_solution_time_based() {
        let spawner = Arc::new(MockPermutationThreadSpawner::new(1));
        let task = create_task_with_solution("test-task");
        // Manually set elapsed time by adjusting start_time
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        let mut task_mut = Arc::try_unwrap(task).unwrap();
        task_mut.start_time = now - 30_000; // 30 seconds ago
        let task = Arc::new(task_mut);
        
        let tracker = ProgressTracker::new(
            spawner,
            200,
            task,
            "wood".to_string(),
        );

        let progress = tracker.get_progress_percentage();
        
        // Time progress: 30,000ms / 60,000ms * 100 = 50%
        // Thread progress: (1-1)/150 * 100 = 0% (limited by MAX_PERMUTATIONS_WITH_SOLUTION)
        // Should return max(50, 0) = 50%
        assert_eq!(progress, 50);
    }

    #[test]
    fn test_progress_calculation_with_solution_thread_based() {
        let spawner = Arc::new(MockPermutationThreadSpawner::new(76)); // 75 completed + 1 current
        let task = create_task_with_solution("test-task");
        let tracker = ProgressTracker::new(
            spawner,
            200, // More than MAX_PERMUTATIONS_WITH_SOLUTION (150)
            task,
            "wood".to_string(),
        );

        let progress = tracker.get_progress_percentage();
        
        // Time progress: very small (new task)
        // Thread progress: (76-1)/150 * 100 = 50% (limited by MAX_PERMUTATIONS_WITH_SOLUTION)
        // Should return max(~0, 50) = 50%
        assert_eq!(progress, 50);
    }

    #[test]
    fn test_progress_capped_at_100_percent() {
        let spawner = Arc::new(MockPermutationThreadSpawner::new(200));
        let task = create_task_with_elapsed_time("test-task", 700_000); // 11+ minutes
        let tracker = ProgressTracker::new(
            spawner,
            10, // Small total permutations
            task,
            "wood".to_string(),
        );

        let progress = tracker.get_progress_percentage();
        
        // Both time and thread progress would exceed 100%, should be capped
        assert_eq!(progress, 100);
    }

    #[test]
    fn test_progress_with_zero_permutations() {
        let spawner = Arc::new(MockPermutationThreadSpawner::new(10));
        let task = Arc::new(Task::new("test-task".to_string()));
        let tracker = ProgressTracker::new(
            spawner,
            0, // Zero permutations
            task,
            "wood".to_string(),
        );

        let progress = tracker.get_progress_percentage();
        
        // Thread progress should be 0 when total_permutations is 0
        // Time progress should be very small for new task
        // Should return 0
        assert_eq!(progress, 0);
    }

    #[test]
    fn test_max_permutations_with_solution_limit() {
        let spawner = Arc::new(MockPermutationThreadSpawner::new(151)); // 150 completed + 1 current
        let task = create_task_with_solution("test-task");
        let tracker = ProgressTracker::new(
            spawner,
            1000, // Much more than MAX_PERMUTATIONS_WITH_SOLUTION
            task,
            "wood".to_string(),
        );

        let progress = tracker.get_progress_percentage();
        
        // Thread progress: (151-1)/150 * 100 = 100% (limited by MAX_PERMUTATIONS_WITH_SOLUTION)
        // Should be exactly 100%
        assert_eq!(progress, 100);
    }

    #[test]
    fn test_refresh_task_status_info() {
        init_logging_for_tests();
        let spawner = Arc::new(MockPermutationThreadSpawner::new(50));
        let task = Arc::new(Task::new("test-task".to_string()));
        let tracker = ProgressTracker::new(
            spawner,
            100,
            task,
            "wood".to_string(),
        );

        // This should not fail, even though we can't actually update the task
        // due to Arc<Task> being immutable
        let result = tracker.refresh_task_status_info();
        assert!(result.is_ok());
    }

    #[test]
    fn test_validation_valid_tracker() {
        init_logging_for_tests();
        let spawner = Arc::new(MockPermutationThreadSpawner::new(10));
        let task = Arc::new(Task::new("test-task".to_string()));
        let tracker = ProgressTracker::new(
            spawner,
            100,
            task,
            "wood".to_string(),
        );

        assert!(tracker.validate().is_ok());
    }

    #[test]
    fn test_validation_negative_permutations() {
        let spawner = Arc::new(MockPermutationThreadSpawner::new(10));
        let task = Arc::new(Task::new("test-task".to_string()));
        let tracker = ProgressTracker::new(
            spawner,
            -1, // Invalid
            task,
            "wood".to_string(),
        );

        assert!(tracker.validate().is_err());
    }

    #[test]
    fn test_validation_empty_material() {
        let spawner = Arc::new(MockPermutationThreadSpawner::new(10));
        let task = Arc::new(Task::new("test-task".to_string()));
        let tracker = ProgressTracker::new(
            spawner,
            100,
            task,
            "".to_string(), // Invalid
        );

        assert!(tracker.validate().is_err());
    }

    #[test]
    fn test_validation_non_running_task() {
        init_logging_for_tests();
        let spawner = Arc::new(MockPermutationThreadSpawner::new(10));
        let task = Arc::new(Task::new("test-task".to_string())); // Status is Idle
        let tracker = ProgressTracker::new(
            spawner,
            100,
            task,
            "wood".to_string(),
        );

        // Should still be valid, but will log a warning
        assert!(tracker.validate().is_ok());
    }

    #[test]
    fn test_getters() {
        let spawner = Arc::new(MockPermutationThreadSpawner::new(10));
        let task = Arc::new(Task::new("test-task".to_string()));
        let tracker = ProgressTracker::new(
            spawner.clone(),
            100,
            task.clone(),
            "wood".to_string(),
        );

        assert_eq!(tracker.get_material(), "wood");
        assert_eq!(tracker.get_total_permutations(), 100);
        assert_eq!(tracker.get_task().id, "test-task");
        assert_eq!(tracker.get_permutation_thread_spawner().get_nbr_total_threads(), 10);
    }

    #[test]
    fn test_display_format() {
        let spawner = Arc::new(MockPermutationThreadSpawner::new(10));
        let task = Arc::new(Task::new("test-task".to_string()));
        let tracker = ProgressTracker::new(
            spawner,
            100,
            task,
            "wood".to_string(),
        );

        let display_str = format!("{}", tracker);
        assert!(display_str.contains("ProgressTracker"));
        assert!(display_str.contains("wood"));
        assert!(display_str.contains("100"));
        assert!(display_str.contains("%"));
    }

    #[test]
    fn test_progress_calculation_edge_cases() {
        // Test with 1 thread (edge case for thread-1 calculation)
        let spawner = Arc::new(MockPermutationThreadSpawner::new(1));
        let task = Arc::new(Task::new("test-task".to_string()));
        let tracker = ProgressTracker::new(
            spawner,
            100,
            task,
            "wood".to_string(),
        );

        let progress = tracker.get_progress_percentage();
        // Thread progress: (1-1)/100 * 100 = 0%
        assert_eq!(progress, 0);
    }

    #[test]
    fn test_progress_calculation_exact_time_boundaries() {
        // Test exactly 1 minute for task with solution
        let spawner = Arc::new(MockPermutationThreadSpawner::new(1));
        let task = create_task_with_elapsed_time("test-task", 60_000); // Exactly 1 minute
        let mut task_mut = Arc::try_unwrap(task).unwrap();
        let mut solution = CalculationResponse::new();
        // Create a dummy panel to make has_solution() return true
        use crate::models::calculation_response::FinalTile;
        let dummy_panel = FinalTile::with_params(1, 100.0, 200.0);
        solution.panels = Some(vec![dummy_panel]);
        solution.no_fit_panels = Vec::new();
        task_mut.solution = Some(solution);
        let task = Arc::new(task_mut);
        
        let tracker = ProgressTracker::new(
            spawner,
            100,
            task,
            "wood".to_string(),
        );

        let progress = tracker.get_progress_percentage();
        // Time progress: 60,000ms / 60,000ms * 100 = 100%
        assert_eq!(progress, 100);

        // Test exactly 10 minutes for task without solution
        let spawner = Arc::new(MockPermutationThreadSpawner::new(1));
        let task = create_task_with_elapsed_time("test-task", 600_000); // Exactly 10 minutes
        let tracker = ProgressTracker::new(
            spawner,
            100,
            task,
            "wood".to_string(),
        );

        let progress = tracker.get_progress_percentage();
        // Time progress: 600,000ms / 600,000ms * 100 = 100%
        assert_eq!(progress, 100);
    }

    #[test]
    fn test_task_with_partial_solution() {
        // Test task that has solution but not all tiles fit
        let spawner = Arc::new(MockPermutationThreadSpawner::new(50));
        let task = Arc::new({
            let mut task = Task::new("test-task".to_string());
            let mut solution = CalculationResponse::new();
            // Create a dummy panel to make has_solution() return true
            use crate::models::calculation_response::{FinalTile, NoFitTile};
            let dummy_panel = FinalTile::with_params(1, 100.0, 200.0);
            solution.panels = Some(vec![dummy_panel]); // Has solution
            // Add a no-fit panel to make has_solution_all_fit() return false
            let no_fit_panel = NoFitTile::with_params(2, 50, 50, 1);
            solution.no_fit_panels = vec![no_fit_panel]; // Some tiles don't fit
            task.solution = Some(solution);
            task
        });
        
        let tracker = ProgressTracker::new(
            spawner,
            100,
            task,
            "wood".to_string(),
        );

        let progress = tracker.get_progress_percentage();
        
        // Should use the "without solution" algorithm since not all tiles fit
        // Thread progress: (50-1)/100 * 100 = 49%
        assert_eq!(progress, 49);
    }
}
