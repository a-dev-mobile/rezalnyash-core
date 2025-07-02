//! Permutation Thread Spawner Tests
//!
//! Comprehensive unit tests for the PermutationThreadSpawner module, covering
//! thread management, concurrency control, progress tracking, and error handling.

#[cfg(test)]
mod tests {
    use super::super::permutation_thread_spawner::*;
    use super::super::progress_tracker::{ProgressTracker, PermutationThreadSpawner as ProgressTrackerTrait};
    use crate::models::task::Task;
    use crate::errors::TaskError;
    use std::sync::{Arc, Mutex};
    use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
    use std::time::{Duration, Instant};
    use std::thread;

    /// Mock progress tracker for testing
    #[derive(Debug)]
    struct MockProgressTracker {
        refresh_count: Arc<AtomicUsize>,
        should_fail: Arc<AtomicBool>,
    }

    impl MockProgressTracker {
        fn new() -> Self {
            Self {
                refresh_count: Arc::new(AtomicUsize::new(0)),
                should_fail: Arc::new(AtomicBool::new(false)),
            }
        }

        fn get_refresh_count(&self) -> usize {
            self.refresh_count.load(Ordering::SeqCst)
        }

        fn set_should_fail(&self, should_fail: bool) {
            self.should_fail.store(should_fail, Ordering::SeqCst);
        }

        fn refresh_task_status_info(&self) -> crate::errors::Result<()> {
            self.refresh_count.fetch_add(1, Ordering::SeqCst);
            if self.should_fail.load(Ordering::SeqCst) {
                Err(TaskError::TaskInvalidState {
                    current_state: "Mock failure".to_string(),
                }.into())
            } else {
                Ok(())
            }
        }
    }

    #[test]
    fn test_new_spawner() {
        let spawner = PermutationThreadSpawner::new();
        assert_eq!(spawner.get_max_alive_spawner_threads(), DEFAULT_MAX_ALIVE_SPAWNER_THREADS);
        assert_eq!(spawner.get_interval_between_max_alive_check(), DEFAULT_INTERVAL_BETWEEN_MAX_ALIVE_CHECK);
        assert_eq!(spawner.get_nbr_total_threads(), 0);
        assert_eq!(spawner.get_nbr_unfinished_threads(), 0);
    }

    #[test]
    fn test_with_settings() {
        let spawner = PermutationThreadSpawner::with_settings(10, 500);
        assert_eq!(spawner.get_max_alive_spawner_threads(), 10);
        assert_eq!(spawner.get_interval_between_max_alive_check(), 500);
        assert_eq!(spawner.get_nbr_total_threads(), 0);
    }

    #[test]
    fn test_spawn_single_thread() {
        let spawner = PermutationThreadSpawner::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let result = spawner.spawn("test-thread".to_string(), move || {
            counter_clone.fetch_add(1, Ordering::SeqCst);
            thread::sleep(Duration::from_millis(10)); // Brief work simulation
            Ok(())
        });

        assert!(result.is_ok());
        assert_eq!(spawner.get_nbr_total_threads(), 1);

        // Wait for the thread to complete
        let start = Instant::now();
        while spawner.get_nbr_unfinished_threads() > 0 && start.elapsed() < Duration::from_secs(5) {
            thread::sleep(Duration::from_millis(10));
        }

        assert_eq!(counter.load(Ordering::SeqCst), 1);
        assert_eq!(spawner.get_nbr_finished_threads(), 1);
        assert_eq!(spawner.get_nbr_unfinished_threads(), 0);
    }

    #[test]
    fn test_spawn_multiple_threads_within_limit() {
        let spawner = PermutationThreadSpawner::with_settings(3, 50);
        let counter = Arc::new(AtomicUsize::new(0));

        // Spawn 3 threads (equal to max_alive)
        for i in 0..3 {
            let counter_clone = counter.clone();
            let result = spawner.spawn(format!("thread-{}", i), move || {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                thread::sleep(Duration::from_millis(50)); // Simulate work
                Ok(())
            });
            assert!(result.is_ok());
        }

        assert_eq!(spawner.get_nbr_total_threads(), 3);

        // Wait for all threads to complete
        let start = Instant::now();
        while spawner.get_nbr_unfinished_threads() > 0 && start.elapsed() < Duration::from_secs(5) {
            thread::sleep(Duration::from_millis(10));
        }

        assert_eq!(counter.load(Ordering::SeqCst), 3);
        assert_eq!(spawner.get_nbr_finished_threads(), 3);
    }

    #[test]
    fn test_spawn_threads_exceeding_limit() {
        let spawner = PermutationThreadSpawner::with_settings(2, 50);
        let counter = Arc::new(AtomicUsize::new(0));
        let start_time = Instant::now();

        // Spawn 4 threads (more than max_alive of 2)
        for i in 0..4 {
            let counter_clone = counter.clone();
            let result = spawner.spawn(format!("thread-{}", i), move || {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                thread::sleep(Duration::from_millis(100)); // Longer work to test concurrency control
                Ok(())
            });
            assert!(result.is_ok());
        }

        assert_eq!(spawner.get_nbr_total_threads(), 4);

        // The spawning should have taken some time due to concurrency control
        let spawn_duration = start_time.elapsed();
        assert!(spawn_duration >= Duration::from_millis(100)); // Should have waited

        // Wait for all threads to complete
        let start = Instant::now();
        while spawner.get_nbr_unfinished_threads() > 0 && start.elapsed() < Duration::from_secs(10) {
            thread::sleep(Duration::from_millis(10));
        }

        assert_eq!(counter.load(Ordering::SeqCst), 4);
        assert_eq!(spawner.get_nbr_finished_threads(), 4);
    }

    #[test]
    fn test_thread_error_handling() {
        let spawner = PermutationThreadSpawner::new();

        let result = spawner.spawn("error-thread".to_string(), || {
            Err(TaskError::TaskInvalidState {
                current_state: "Test error".to_string(),
            }.into())
        });

        assert!(result.is_ok()); // Spawning should succeed

        // Wait for thread to complete
        let start = Instant::now();
        while spawner.get_nbr_unfinished_threads() > 0 && start.elapsed() < Duration::from_secs(5) {
            thread::sleep(Duration::from_millis(10));
        }

        assert_eq!(spawner.get_nbr_error_threads(), 1);
        assert_eq!(spawner.get_nbr_finished_threads(), 0);
        assert_eq!(spawner.get_nbr_total_threads(), 1);
    }

    #[test]
    fn test_mixed_success_and_error_threads() {
        let spawner = PermutationThreadSpawner::new();

        // Spawn successful thread
        let result1 = spawner.spawn("success-thread".to_string(), || {
            thread::sleep(Duration::from_millis(10));
            Ok(())
        });
        assert!(result1.is_ok());

        // Spawn error thread
        let result2 = spawner.spawn("error-thread".to_string(), || {
            Err(TaskError::TaskInvalidState {
                current_state: "Test error".to_string(),
            }.into())
        });
        assert!(result2.is_ok());

        // Wait for threads to complete
        let start = Instant::now();
        while spawner.get_nbr_unfinished_threads() > 0 && start.elapsed() < Duration::from_secs(5) {
            thread::sleep(Duration::from_millis(10));
        }

        assert_eq!(spawner.get_nbr_total_threads(), 2);
        assert_eq!(spawner.get_nbr_finished_threads(), 1);
        assert_eq!(spawner.get_nbr_error_threads(), 1);
        assert_eq!(spawner.get_nbr_unfinished_threads(), 0);
    }

    #[test]
    fn test_cleanup_finished_threads() {
        let spawner = PermutationThreadSpawner::new();

        // Spawn threads that complete quickly
        for i in 0..3 {
            let result = spawner.spawn(format!("cleanup-test-{}", i), || {
                Ok(())
            });
            assert!(result.is_ok());
        }

        // Wait for completion
        let start = Instant::now();
        while spawner.get_nbr_unfinished_threads() > 0 && start.elapsed() < Duration::from_secs(5) {
            thread::sleep(Duration::from_millis(10));
        }

        assert_eq!(spawner.get_nbr_total_threads(), 3);
        assert_eq!(spawner.get_nbr_finished_threads(), 3);

        // Cleanup
        let cleaned = spawner.cleanup_finished_threads().unwrap();
        assert_eq!(cleaned, 3);
        assert_eq!(spawner.get_nbr_total_threads(), 0);
        assert_eq!(spawner.get_nbr_finished_threads(), 0);
    }

    #[test]
    fn test_wait_for_all_threads() {
        let spawner = PermutationThreadSpawner::new();
        let counter = Arc::new(AtomicUsize::new(0));

        // Spawn multiple threads
        for i in 0..3 {
            let counter_clone = counter.clone();
            let result = spawner.spawn(format!("wait-test-{}", i), move || {
                thread::sleep(Duration::from_millis(50));
                counter_clone.fetch_add(1, Ordering::SeqCst);
                Ok(())
            });
            assert!(result.is_ok());
        }

        // Wait for all to complete
        let result = spawner.wait_for_all_threads();
        assert!(result.is_ok());

        assert_eq!(counter.load(Ordering::SeqCst), 3);
        assert_eq!(spawner.get_nbr_unfinished_threads(), 0);
        assert_eq!(spawner.get_nbr_finished_threads(), 3);
    }

    #[test]
    fn test_thread_stats() {
        let spawner = PermutationThreadSpawner::new();
        let (total, running, finished, error, unfinished) = spawner.get_thread_stats();
        
        assert_eq!(total, 0);
        assert_eq!(running, 0);
        assert_eq!(finished, 0);
        assert_eq!(error, 0);
        assert_eq!(unfinished, 0);

        // Spawn a long-running thread to test stats while running
        let barrier = Arc::new(AtomicBool::new(false));
        let barrier_clone = barrier.clone();
        
        let result = spawner.spawn("stats-test".to_string(), move || {
            while !barrier_clone.load(Ordering::SeqCst) {
                thread::sleep(Duration::from_millis(10));
            }
            Ok(())
        });
        assert!(result.is_ok());

        // Give thread time to start
        thread::sleep(Duration::from_millis(50));

        let (total, running, finished, error, unfinished) = spawner.get_thread_stats();
        assert_eq!(total, 1);
        assert_eq!(running, 1);
        assert_eq!(finished, 0);
        assert_eq!(error, 0);
        assert_eq!(unfinished, 1);

        // Release the thread
        barrier.store(true, Ordering::SeqCst);

        // Wait for completion
        let start = Instant::now();
        while spawner.get_nbr_unfinished_threads() > 0 && start.elapsed() < Duration::from_secs(5) {
            thread::sleep(Duration::from_millis(10));
        }

        let (total, running, finished, error, unfinished) = spawner.get_thread_stats();
        assert_eq!(total, 1);
        assert_eq!(running, 0);
        assert_eq!(finished, 1);
        assert_eq!(error, 0);
        assert_eq!(unfinished, 0);
    }

    #[test]
    fn test_setters_and_getters() {
        let mut spawner = PermutationThreadSpawner::new();

        // Test max alive threads
        spawner.set_max_alive_spawner_threads(10);
        assert_eq!(spawner.get_max_alive_spawner_threads(), 10);

        // Test interval
        spawner.set_interval_between_max_alive_check(2000);
        assert_eq!(spawner.get_interval_between_max_alive_check(), 2000);

        // Test progress tracker
        assert!(spawner.get_progress_tracker().is_none());
    }

    #[test]
    fn test_validation() {
        let mut spawner = PermutationThreadSpawner::new();
        assert!(spawner.validate().is_ok());

        // Test invalid max alive threads
        spawner.set_max_alive_spawner_threads(0);
        assert!(spawner.validate().is_err());

        // Reset and test invalid interval
        spawner.set_max_alive_spawner_threads(5);
        spawner.set_interval_between_max_alive_check(0);
        assert!(spawner.validate().is_err());
    }

    #[test]
    fn test_display_format() {
        let spawner = PermutationThreadSpawner::new();
        let display_str = format!("{}", spawner);
        
        assert!(display_str.contains("PermutationThreadSpawner"));
        assert!(display_str.contains("total: 0"));
        assert!(display_str.contains("max_alive: 5"));
    }

    #[test]
    fn test_default_implementation() {
        let spawner = PermutationThreadSpawner::default();
        assert_eq!(spawner.get_max_alive_spawner_threads(), DEFAULT_MAX_ALIVE_SPAWNER_THREADS);
        assert_eq!(spawner.get_interval_between_max_alive_check(), DEFAULT_INTERVAL_BETWEEN_MAX_ALIVE_CHECK);
    }

    #[test]
    fn test_progress_tracker_trait_implementation() {
        let spawner = PermutationThreadSpawner::new();
        
        // Test that it implements the trait correctly
        let trait_ref: &dyn ProgressTrackerTrait = &spawner;
        assert_eq!(trait_ref.get_nbr_total_threads(), 0);

        // Spawn a thread and test again
        let result = spawner.spawn("trait-test".to_string(), || Ok(()));
        assert!(result.is_ok());
        assert_eq!(trait_ref.get_nbr_total_threads(), 1);
    }

    #[test]
    fn test_managed_thread_states() {
        let spawner = PermutationThreadSpawner::new();
        let state_tracker = Arc::new(Mutex::new(Vec::new()));
        let state_tracker_clone = state_tracker.clone();

        let result = spawner.spawn("state-test".to_string(), move || {
            // Record that we're running
            state_tracker_clone.lock().unwrap().push("running");
            thread::sleep(Duration::from_millis(50));
            state_tracker_clone.lock().unwrap().push("finished");
            Ok(())
        });

        assert!(result.is_ok());

        // Initially should be running
        thread::sleep(Duration::from_millis(25));
        assert_eq!(spawner.get_nbr_running_threads(), 1);

        // Wait for completion
        let start = Instant::now();
        while spawner.get_nbr_unfinished_threads() > 0 && start.elapsed() < Duration::from_secs(5) {
            thread::sleep(Duration::from_millis(10));
        }

        assert_eq!(spawner.get_nbr_finished_threads(), 1);
        assert_eq!(spawner.get_nbr_running_threads(), 0);

        let states = state_tracker.lock().unwrap();
        assert_eq!(states.len(), 2);
        assert_eq!(states[0], "running");
        assert_eq!(states[1], "finished");
    }

    #[test]
    fn test_concurrent_spawning() {
        let spawner = Arc::new(PermutationThreadSpawner::with_settings(2, 100));
        let counter = Arc::new(AtomicUsize::new(0));
        let mut handles = Vec::new();

        // Spawn multiple threads concurrently from different threads
        for i in 0..4 {
            let spawner_clone = spawner.clone();
            let counter_clone = counter.clone();
            
            let handle = thread::spawn(move || {
                let result = spawner_clone.spawn(format!("concurrent-{}", i), move || {
                    counter_clone.fetch_add(1, Ordering::SeqCst);
                    thread::sleep(Duration::from_millis(50));
                    Ok(())
                });
                result
            });
            handles.push(handle);
        }

        // Wait for all spawning threads to complete
        for handle in handles {
            let result = handle.join().unwrap();
            assert!(result.is_ok());
        }

        assert_eq!(spawner.get_nbr_total_threads(), 4);

        // Wait for all worker threads to complete
        let start = Instant::now();
        while spawner.get_nbr_unfinished_threads() > 0 && start.elapsed() < Duration::from_secs(10) {
            thread::sleep(Duration::from_millis(10));
        }

        assert_eq!(counter.load(Ordering::SeqCst), 4);
        assert_eq!(spawner.get_nbr_finished_threads(), 4);
    }

    #[test]
    fn test_thread_state_transitions() {
        let spawner = PermutationThreadSpawner::new();
        
        // Test successful thread state transitions
        let result = spawner.spawn("transition-test".to_string(), || {
            thread::sleep(Duration::from_millis(10));
            Ok(())
        });
        assert!(result.is_ok());

        // Should start as running
        thread::sleep(Duration::from_millis(5));
        let (_, running, _, _, _) = spawner.get_thread_stats();
        assert!(running > 0);

        // Wait for completion and check final state
        let start = Instant::now();
        while spawner.get_nbr_unfinished_threads() > 0 && start.elapsed() < Duration::from_secs(5) {
            thread::sleep(Duration::from_millis(10));
        }

        let (_, running, finished, error, _) = spawner.get_thread_stats();
        assert_eq!(running, 0);
        assert_eq!(finished, 1);
        assert_eq!(error, 0);
    }

    #[test]
    fn test_large_number_of_threads() {
        let spawner = PermutationThreadSpawner::with_settings(3, 50);
        let counter = Arc::new(AtomicUsize::new(0));
        let thread_count = 10;

        // Spawn many threads
        for i in 0..thread_count {
            let counter_clone = counter.clone();
            let result = spawner.spawn(format!("large-test-{}", i), move || {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                thread::sleep(Duration::from_millis(10));
                Ok(())
            });
            assert!(result.is_ok());
        }

        assert_eq!(spawner.get_nbr_total_threads(), thread_count);

        // Wait for all to complete
        let result = spawner.wait_for_all_threads();
        assert!(result.is_ok());

        assert_eq!(counter.load(Ordering::SeqCst), thread_count as usize);
        assert_eq!(spawner.get_nbr_finished_threads(), thread_count as usize);
        assert_eq!(spawner.get_nbr_unfinished_threads(), 0);
    }
}
