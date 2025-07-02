//! Progress Tracker Model
//!
//! This module provides the ProgressTracker struct which monitors and calculates
//! progress percentages for tasks based on elapsed time and thread completion.
//! It's a Rust conversion of the Java ProgressTracker class with improved
//! error handling and type safety.

use crate::errors::{Result, TaskError};
use crate::models::task::Task;
use crate::{log_debug, log_warn};
use std::sync::Arc;

/// Maximum permutations threshold for tasks with solutions
const MAX_PERMUTATIONS_WITH_SOLUTION: i32 = 150;

/// Trait for permutation thread spawner functionality
pub trait PermutationThreadSpawner: Send + Sync + std::fmt::Debug {
    /// Gets the total number of threads
    fn get_nbr_total_threads(&self) -> i32;
}

/// Progress tracker for monitoring task completion
///
/// The ProgressTracker calculates progress percentages based on elapsed time
/// and thread completion ratios, with different algorithms for tasks with
/// and without solutions.
#[derive(Debug)]
pub struct ProgressTracker {
    /// Reference to the permutation thread spawner
    permutation_thread_spawner: Arc<dyn PermutationThreadSpawner>,
    
    /// Total number of permutations to process
    total_permutations: i32,
    
    /// Task being tracked
    task: Arc<Task>,
    
    /// Material being processed
    material: String,
}

impl ProgressTracker {
    /// Creates a new ProgressTracker
    ///
    /// # Arguments
    /// * `permutation_thread_spawner` - The thread spawner managing permutations
    /// * `total_permutations` - Total number of permutations to process
    /// * `task` - The task being tracked
    /// * `material` - The material being processed
    ///
    /// # Returns
    /// A new ProgressTracker instance
    ///
    /// # Examples
    /// ```
    /// use std::sync::Arc;
    /// use rezalnyash_core::models::{
    ///     watch_dog::progress_tracker::ProgressTracker,
    ///     task::Task,
    /// };
    ///
    /// // This example requires implementing PermutationThreadSpawner
    /// // let spawner = Arc::new(MySpawner::new());
    /// // let task = Arc::new(Task::new("task-001".to_string()));
    /// // let tracker = ProgressTracker::new(spawner, 1000, task, "wood".to_string());
    /// ```
    pub fn new(
        permutation_thread_spawner: Arc<dyn PermutationThreadSpawner>,
        total_permutations: i32,
        task: Arc<Task>,
        material: String,
    ) -> Self {
        Self {
            permutation_thread_spawner,
            total_permutations,
            task,
            material,
        }
    }

    /// Refreshes the task status information by calculating current progress
    ///
    /// This method implements the same logic as the Java version:
    /// - For tasks with solutions: uses 1-minute time scale and limited permutations
    /// - For tasks without solutions: uses 10-minute time scale and full permutations
    ///
    /// # Returns
    /// `Ok(())` if successful, `Err(TaskError)` if the task update fails
    pub fn refresh_task_status_info(&self) -> Result<()> {
        let progress_percentage = if self.task.has_solution_all_fit() {
            self.calculate_progress_with_solution()
        } else {
            self.calculate_progress_without_solution()
        };

        log_debug!(
            "Progress for material '{}': {}%", 
            self.material, 
            progress_percentage
        );

        // Note: In the original Java, this would call task.setMaterialPercentageDone()
        // Since we have an Arc<Task> (immutable reference), we would need to either:
        // 1. Use interior mutability (Mutex/RwLock) in the Task struct
        // 2. Return the percentage and let the caller update the task
        // 3. Use a callback mechanism
        //
        // For this conversion, we'll return the percentage and document that
        // the caller should update the task accordingly.
        
        // In a real implementation, you might want to use a callback or
        // modify the Task to use interior mutability for progress tracking
        log_warn!(
            "Progress calculated as {}% for material '{}' - caller should update task",
            progress_percentage,
            self.material
        );

        Ok(())
    }

    /// Gets the calculated progress percentage for the current material
    ///
    /// # Returns
    /// Progress percentage (0-100)
    pub fn get_progress_percentage(&self) -> i32 {
        if self.task.has_solution_all_fit() {
            self.calculate_progress_with_solution()
        } else {
            self.calculate_progress_without_solution()
        }
    }

    /// Calculates progress for tasks that have a solution where all tiles fit
    ///
    /// Uses a 1-minute time scale (60,000ms) and limits permutations to MAX_PERMUTATIONS_WITH_SOLUTION
    ///
    /// # Returns
    /// Progress percentage (0-100)
    fn calculate_progress_with_solution(&self) -> i32 {
        let elapsed_time = self.task.get_elapsed_time();
        let total_threads = self.permutation_thread_spawner.get_nbr_total_threads();
        
        // Time-based progress (1 minute = 100%)
        let time_progress = ((elapsed_time as f64 / 60_000.0) * 100.0) as i32;
        
        // Thread-based progress with limited permutations
        let limited_permutations = std::cmp::min(MAX_PERMUTATIONS_WITH_SOLUTION, self.total_permutations);
        let thread_progress = if limited_permutations > 0 {
            (((total_threads - 1) as f64 / limited_permutations as f64) * 100.0) as i32
        } else {
            0
        };
        
        // Return the maximum of time and thread progress, capped at 100%
        std::cmp::min(std::cmp::max(time_progress, thread_progress), 100)
    }

    /// Calculates progress for tasks without a complete solution
    ///
    /// Uses a 10-minute time scale (600,000ms) and full permutation count
    ///
    /// # Returns
    /// Progress percentage (0-100)
    fn calculate_progress_without_solution(&self) -> i32 {
        let elapsed_time = self.task.get_elapsed_time();
        let total_threads = self.permutation_thread_spawner.get_nbr_total_threads();
        
        // Time-based progress (10 minutes = 100%)
        let time_progress = ((elapsed_time as f64 / 600_000.0) * 100.0) as i32;
        
        // Thread-based progress with full permutations
        let thread_progress = if self.total_permutations > 0 {
            (((total_threads - 1) as f64 / self.total_permutations as f64) * 100.0) as i32
        } else {
            0
        };
        
        // Return the maximum of time and thread progress, capped at 100%
        std::cmp::min(std::cmp::max(time_progress, thread_progress), 100)
    }

    /// Gets the material being tracked
    pub fn get_material(&self) -> &str {
        &self.material
    }

    /// Gets the total permutations count
    pub fn get_total_permutations(&self) -> i32 {
        self.total_permutations
    }

    /// Gets the task being tracked
    pub fn get_task(&self) -> Arc<Task> {
        self.task.clone()
    }

    /// Gets the permutation thread spawner
    pub fn get_permutation_thread_spawner(&self) -> Arc<dyn PermutationThreadSpawner> {
        self.permutation_thread_spawner.clone()
    }

    /// Validates the progress tracker configuration
    ///
    /// # Returns
    /// `Ok(())` if valid, `Err(TaskError)` if invalid
    pub fn validate(&self) -> Result<()> {
        if self.total_permutations < 0 {
            return Err(TaskError::TaskInvalidState {
                current_state: format!("Invalid total_permutations: {}", self.total_permutations),
            }.into());
        }

        if self.material.is_empty() {
            return Err(TaskError::TaskInvalidState {
                current_state: "Material cannot be empty".to_string(),
            }.into());
        }

        // Validate that the task is in a trackable state
        if !self.task.is_running() {
            log_warn!("Progress tracker created for non-running task: {}", self.task.id);
        }

        Ok(())
    }
}

impl std::fmt::Display for ProgressTracker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ProgressTracker {{ material: {}, progress: {}%, total_permutations: {} }}",
            self.material,
            self.get_progress_percentage(),
            self.total_permutations
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::task::Task;
    use crate::enums::Status;
    use std::sync::Arc;

    /// Mock implementation of PermutationThreadSpawner for testing
    #[derive(Debug)]
    struct MockPermutationThreadSpawner {
        total_threads: i32,
    }

    impl MockPermutationThreadSpawner {
        fn new(total_threads: i32) -> Self {
            Self { total_threads }
        }
    }

    impl PermutationThreadSpawner for MockPermutationThreadSpawner {
        fn get_nbr_total_threads(&self) -> i32 {
            self.total_threads
        }
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
    fn test_progress_calculation_without_solution() {
        let spawner = Arc::new(MockPermutationThreadSpawner::new(50)); // 49 completed + 1 current
        let task = Arc::new(Task::new("test-task".to_string()));
        let tracker = ProgressTracker::new(
            spawner,
            100,
            task,
            "wood".to_string(),
        );

        let progress = tracker.calculate_progress_without_solution();
        
        // Thread progress: (50-1)/100 * 100 = 49%
        // Time progress will be very small for a new task
        // Should return the maximum, which is thread progress
        assert_eq!(progress, 49);
    }

    #[test]
    fn test_progress_calculation_with_solution() {
        let spawner = Arc::new(MockPermutationThreadSpawner::new(50));
        let task = Arc::new(Task::new("test-task".to_string()));
        let tracker = ProgressTracker::new(
            spawner,
            200, // More than MAX_PERMUTATIONS_WITH_SOLUTION
            task,
            "wood".to_string(),
        );

        let progress = tracker.calculate_progress_with_solution();
        
        // Thread progress: (50-1)/150 * 100 = 32% (limited by MAX_PERMUTATIONS_WITH_SOLUTION)
        // Time progress will be very small for a new task
        assert_eq!(progress, 32);
    }

    #[test]
    fn test_progress_capped_at_100() {
        let spawner = Arc::new(MockPermutationThreadSpawner::new(200));
        let task = Arc::new(Task::new("test-task".to_string()));
        let tracker = ProgressTracker::new(
            spawner,
            10, // Small total permutations
            task,
            "wood".to_string(),
        );

        let progress = tracker.calculate_progress_without_solution();
        
        // Thread progress would be (200-1)/10 * 100 = 1990%, but should be capped at 100%
        assert_eq!(progress, 100);
    }

    #[test]
    fn test_validation() {
        let spawner = Arc::new(MockPermutationThreadSpawner::new(10));
        let task = Arc::new(Task::new("test-task".to_string()));
        
        // Valid tracker
        let tracker = ProgressTracker::new(
            spawner.clone(),
            100,
            task.clone(),
            "wood".to_string(),
        );
        assert!(tracker.validate().is_ok());

        // Invalid total permutations
        let invalid_tracker = ProgressTracker::new(
            spawner.clone(),
            -1,
            task.clone(),
            "wood".to_string(),
        );
        assert!(invalid_tracker.validate().is_err());

        // Empty material
        let empty_material_tracker = ProgressTracker::new(
            spawner,
            100,
            task,
            "".to_string(),
        );
        assert!(empty_material_tracker.validate().is_err());
    }

    #[test]
    fn test_display() {
        let spawner = Arc::new(MockPermutationThreadSpawner::new(10));
        let task = Arc::new(Task::new("test-task".to_string()));
        let tracker = ProgressTracker::new(
            spawner,
            100,
            task,
            "wood".to_string(),
        );

        let display_str = format!("{}", tracker);
        assert!(display_str.contains("wood"));
        assert!(display_str.contains("100"));
    }
}
