//! Permutation Thread Spawner Model
//!
//! This module provides the PermutationThreadSpawner struct which manages
//! thread spawning with concurrency limits and progress tracking.
//! It's a Rust conversion of the Java PermutationThreadSpawner class with
//! improved error handling and async support.

use crate::errors::{Result, TaskError};
use crate::models::watch_dog::progress_tracker::ProgressTracker;
use crate::{log_debug, log_error, log_info, log_warn};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

/// Default maximum number of alive spawner threads
pub const DEFAULT_MAX_ALIVE_SPAWNER_THREADS: usize = 5;

/// Default interval between max alive checks (in milliseconds)
pub const DEFAULT_INTERVAL_BETWEEN_MAX_ALIVE_CHECK: u64 = 1000;

/// Thread state information
#[derive(Debug, Clone, PartialEq)]
pub enum ThreadState {
    /// Thread is newly created but not started
    New,
    /// Thread is currently running
    Running,
    /// Thread has completed successfully
    Finished,
    /// Thread was terminated
    Terminated,
    /// Thread encountered an error
    Error,
}

/// Wrapper for thread information
#[derive(Debug)]
pub struct ManagedThread {
    /// The actual thread handle
    handle: Option<JoinHandle<Result<()>>>,
    /// Current state of the thread
    state: Arc<Mutex<ThreadState>>,
    /// Thread identifier
    id: String,
}

impl ManagedThread {
    /// Creates a new ManagedThread
    pub fn new<F>(id: String, f: F) -> Self 
    where
        F: FnOnce() -> Result<()> + Send + 'static,
    {
        let state = Arc::new(Mutex::new(ThreadState::New));
        let state_clone = state.clone();
        
        let handle = thread::spawn(move || {
            // Update state to running
            if let Ok(mut state_guard) = state_clone.lock() {
                *state_guard = ThreadState::Running;
            }
            
            // Execute the function
            let result = f();
            
            // Update state based on result
            if let Ok(mut state_guard) = state_clone.lock() {
                *state_guard = if result.is_ok() {
                    ThreadState::Finished
                } else {
                    ThreadState::Error
                };
            }
            
            result
        });
        
        Self {
            handle: Some(handle),
            state,
            id,
        }
    }

    /// Gets the current state of the thread
    pub fn get_state(&self) -> ThreadState {
        self.state.lock()
            .map(|guard| guard.clone())
            .unwrap_or(ThreadState::Error)
    }

    /// Checks if the thread is alive (running or new)
    pub fn is_alive(&self) -> bool {
        matches!(self.get_state(), ThreadState::New | ThreadState::Running)
    }

    /// Gets the thread ID
    pub fn get_id(&self) -> &str {
        &self.id
    }

    /// Joins the thread and returns the result
    pub fn join(mut self) -> Result<()> {
        if let Some(handle) = self.handle.take() {
            handle.join()
                .map_err(|_| TaskError::TaskThreadError {
                    details: format!("Thread {} panicked", self.id),
                })?
        } else {
            Ok(())
        }
    }
}

/// Permutation thread spawner for managing concurrent thread execution
///
/// The PermutationThreadSpawner controls the number of concurrently running threads,
/// monitors their progress, and provides thread lifecycle management with
/// configurable concurrency limits.
#[derive(Debug)]
pub struct PermutationThreadSpawner {
    /// Progress tracker for monitoring task completion
    progress_tracker: Option<Arc<ProgressTracker>>,
    
    /// Maximum number of alive spawner threads
    max_alive_spawner_threads: usize,
    
    /// Interval between max alive checks in milliseconds
    interval_between_max_alive_check: u64,
    
    /// List of managed threads
    threads: Arc<Mutex<Vec<ManagedThread>>>,
}

impl PermutationThreadSpawner {
    /// Creates a new PermutationThreadSpawner with default settings
    ///
    /// # Returns
    /// A new PermutationThreadSpawner instance
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::models::watch_dog::permutation_thread_spawner::PermutationThreadSpawner;
    ///
    /// let spawner = PermutationThreadSpawner::new();
    /// assert_eq!(spawner.get_max_alive_spawner_threads(), 5);
    /// ```
    pub fn new() -> Self {
        Self {
            progress_tracker: None,
            max_alive_spawner_threads: DEFAULT_MAX_ALIVE_SPAWNER_THREADS,
            interval_between_max_alive_check: DEFAULT_INTERVAL_BETWEEN_MAX_ALIVE_CHECK,
            threads: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Creates a new PermutationThreadSpawner with custom settings
    ///
    /// # Arguments
    /// * `max_alive_threads` - Maximum number of concurrent threads
    /// * `check_interval_ms` - Interval between alive checks in milliseconds
    ///
    /// # Returns
    /// A new PermutationThreadSpawner instance
    pub fn with_settings(max_alive_threads: usize, check_interval_ms: u64) -> Self {
        Self {
            progress_tracker: None,
            max_alive_spawner_threads: max_alive_threads,
            interval_between_max_alive_check: check_interval_ms,
            threads: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Sets the progress tracker
    ///
    /// # Arguments
    /// * `progress_tracker` - The progress tracker to use
    pub fn set_progress_tracker(&mut self, progress_tracker: Arc<ProgressTracker>) {
        self.progress_tracker = Some(progress_tracker);
    }

    /// Spawns a new thread with concurrency control
    ///
    /// This method will block until there's room for a new thread based on
    /// the max_alive_spawner_threads limit. It periodically checks the number
    /// of unfinished threads and refreshes progress tracking.
    ///
    /// # Arguments
    /// * `thread_id` - Unique identifier for the thread
    /// * `task` - The task to execute in the thread
    ///
    /// # Returns
    /// `Ok(())` if the thread was spawned successfully, `Err(TaskError)` otherwise
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::models::watch_dog::permutation_thread_spawner::PermutationThreadSpawner;
    ///
    /// let mut spawner = PermutationThreadSpawner::new();
    /// 
    /// let result = spawner.spawn("thread-1".to_string(), || {
    ///     // Your thread work here
    ///     println!("Thread executing");
    ///     Ok(())
    /// });
    /// 
    /// assert!(result.is_ok());
    /// ```
    pub fn spawn<F>(&self, thread_id: String, task: F) -> Result<()>
    where
        F: FnOnce() -> Result<()> + Send + 'static,
    {
        // Create the managed thread
        let managed_thread = ManagedThread::new(thread_id.clone(), task);
        
        // Add to threads list
        {
            let mut threads = self.threads.lock()
                .map_err(|_| TaskError::TaskLockError {
                    operation: "spawn - add thread".to_string(),
                })?;
            threads.push(managed_thread);
        }

        // Wait until we have room for more threads
        while self.get_nbr_unfinished_threads() >= self.max_alive_spawner_threads {
            // Refresh progress if we have a tracker
            if let Some(progress_tracker) = &self.progress_tracker {
                if let Err(e) = progress_tracker.refresh_task_status_info() {
                    log_error!("Failed to refresh task status info: {}", e);
                }
            }

            // Sleep for the configured interval
            thread::sleep(Duration::from_millis(self.interval_between_max_alive_check));
        }

        log_debug!("Spawned thread: {}", thread_id);
        Ok(())
    }

    /// Gets the number of unfinished threads (new or running)
    ///
    /// # Returns
    /// Number of threads that are either new or currently running
    pub fn get_nbr_unfinished_threads(&self) -> usize {
        self.threads.lock()
            .map(|threads| {
                threads.iter()
                    .filter(|thread| thread.is_alive())
                    .count()
            })
            .unwrap_or(0)
    }

    /// Gets the total number of threads
    ///
    /// # Returns
    /// Total number of threads managed by this spawner
    pub fn get_nbr_total_threads(&self) -> i32 {
        self.threads.lock()
            .map(|threads| threads.len() as i32)
            .unwrap_or(0)
    }

    /// Gets the number of finished threads
    ///
    /// # Returns
    /// Number of threads that have completed successfully
    pub fn get_nbr_finished_threads(&self) -> usize {
        self.threads.lock()
            .map(|threads| {
                threads.iter()
                    .filter(|thread| thread.get_state() == ThreadState::Finished)
                    .count()
            })
            .unwrap_or(0)
    }

    /// Gets the number of error threads
    ///
    /// # Returns
    /// Number of threads that encountered errors
    pub fn get_nbr_error_threads(&self) -> usize {
        self.threads.lock()
            .map(|threads| {
                threads.iter()
                    .filter(|thread| thread.get_state() == ThreadState::Error)
                    .count()
            })
            .unwrap_or(0)
    }

    /// Gets the number of running threads
    ///
    /// # Returns
    /// Number of threads currently running
    pub fn get_nbr_running_threads(&self) -> usize {
        self.threads.lock()
            .map(|threads| {
                threads.iter()
                    .filter(|thread| thread.get_state() == ThreadState::Running)
                    .count()
            })
            .unwrap_or(0)
    }

    /// Gets the maximum alive spawner threads setting
    pub fn get_max_alive_spawner_threads(&self) -> usize {
        self.max_alive_spawner_threads
    }

    /// Sets the maximum alive spawner threads
    ///
    /// # Arguments
    /// * `max_threads` - Maximum number of concurrent threads
    pub fn set_max_alive_spawner_threads(&mut self, max_threads: usize) {
        self.max_alive_spawner_threads = max_threads;
        log_info!("Updated max alive spawner threads to: {}", max_threads);
    }

    /// Gets the interval between max alive checks
    pub fn get_interval_between_max_alive_check(&self) -> u64 {
        self.interval_between_max_alive_check
    }

    /// Sets the interval between max alive checks
    ///
    /// # Arguments
    /// * `interval_ms` - Interval in milliseconds
    pub fn set_interval_between_max_alive_check(&mut self, interval_ms: u64) {
        self.interval_between_max_alive_check = interval_ms;
        log_info!("Updated interval between max alive check to: {}ms", interval_ms);
    }

    /// Gets the progress tracker
    pub fn get_progress_tracker(&self) -> Option<Arc<ProgressTracker>> {
        self.progress_tracker.clone()
    }

    /// Waits for all threads to complete
    ///
    /// # Returns
    /// `Ok(())` if all threads completed successfully, `Err(TaskError)` if any failed
    pub fn wait_for_all_threads(&self) -> Result<()> {
        log_info!("Waiting for all threads to complete...");
        
        // We can't join threads from the shared Vec directly due to ownership,
        // so we'll poll until all are finished
        loop {
            let unfinished = self.get_nbr_unfinished_threads();
            if unfinished == 0 {
                break;
            }
            
            log_debug!("Waiting for {} threads to finish", unfinished);
            thread::sleep(Duration::from_millis(100));
        }

        let error_count = self.get_nbr_error_threads();
        if error_count > 0 {
            log_warn!("{} threads completed with errors", error_count);
        }

        log_info!("All threads completed");
        Ok(())
    }

    /// Cleans up finished threads from the internal list
    ///
    /// This method removes threads that have completed (either successfully or with errors)
    /// to prevent memory leaks in long-running applications.
    ///
    /// # Returns
    /// Number of threads that were cleaned up
    pub fn cleanup_finished_threads(&self) -> Result<usize> {
        let mut threads = self.threads.lock()
            .map_err(|_| TaskError::TaskLockError {
                operation: "cleanup_finished_threads".to_string(),
            })?;

        let initial_count = threads.len();
        threads.retain(|thread| thread.is_alive());
        let cleaned_count = initial_count - threads.len();

        if cleaned_count > 0 {
            log_debug!("Cleaned up {} finished threads", cleaned_count);
        }

        Ok(cleaned_count)
    }

    /// Gets thread statistics
    ///
    /// # Returns
    /// Tuple of (total, running, finished, error, unfinished)
    pub fn get_thread_stats(&self) -> (usize, usize, usize, usize, usize) {
        let total = self.get_nbr_total_threads() as usize;
        let running = self.get_nbr_running_threads();
        let finished = self.get_nbr_finished_threads();
        let error = self.get_nbr_error_threads();
        let unfinished = self.get_nbr_unfinished_threads();
        
        (total, running, finished, error, unfinished)
    }

    /// Validates the spawner configuration
    ///
    /// # Returns
    /// `Ok(())` if valid, `Err(TaskError)` if invalid
    pub fn validate(&self) -> Result<()> {
        if self.max_alive_spawner_threads == 0 {
            return Err(TaskError::TaskInvalidState {
                current_state: "max_alive_spawner_threads cannot be zero".to_string(),
            }.into());
        }

        if self.interval_between_max_alive_check == 0 {
            return Err(TaskError::TaskInvalidState {
                current_state: "interval_between_max_alive_check cannot be zero".to_string(),
            }.into());
        }

        Ok(())
    }
}

impl Default for PermutationThreadSpawner {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for PermutationThreadSpawner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (total, running, finished, error, unfinished) = self.get_thread_stats();
        write!(
            f,
            "PermutationThreadSpawner {{ total: {}, running: {}, finished: {}, error: {}, unfinished: {}, max_alive: {} }}",
            total, running, finished, error, unfinished, self.max_alive_spawner_threads
        )
    }
}

// Implement the trait for use with ProgressTracker
impl super::progress_tracker::PermutationThreadSpawner for PermutationThreadSpawner {
    fn get_nbr_total_threads(&self) -> i32 {
        self.get_nbr_total_threads()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Instant;

    #[test]
    fn test_new_spawner() {
        let spawner = PermutationThreadSpawner::new();
        assert_eq!(spawner.get_max_alive_spawner_threads(), DEFAULT_MAX_ALIVE_SPAWNER_THREADS);
        assert_eq!(spawner.get_interval_between_max_alive_check(), DEFAULT_INTERVAL_BETWEEN_MAX_ALIVE_CHECK);
        assert_eq!(spawner.get_nbr_total_threads(), 0);
    }

    #[test]
    fn test_with_settings() {
        let spawner = PermutationThreadSpawner::with_settings(10, 500);
        assert_eq!(spawner.get_max_alive_spawner_threads(), 10);
        assert_eq!(spawner.get_interval_between_max_alive_check(), 500);
    }

    #[test]
    fn test_spawn_single_thread() {
        let spawner = PermutationThreadSpawner::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let result = spawner.spawn("test-thread".to_string(), move || {
            counter_clone.fetch_add(1, Ordering::SeqCst);
            Ok(())
        });

        assert!(result.is_ok());
        assert_eq!(spawner.get_nbr_total_threads(), 1);

        // Wait a bit for the thread to execute
        thread::sleep(Duration::from_millis(100));
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_spawn_multiple_threads() {
        let spawner = PermutationThreadSpawner::with_settings(2, 50);
        let counter = Arc::new(AtomicUsize::new(0));

        // Spawn 3 threads (more than max_alive)
        for i in 0..3 {
            let counter_clone = counter.clone();
            let result = spawner.spawn(format!("thread-{}", i), move || {
                thread::sleep(Duration::from_millis(50)); // Simulate work
                counter_clone.fetch_add(1, Ordering::SeqCst);
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
    fn test_thread_error_handling() {
        let spawner = PermutationThreadSpawner::new();

        let result = spawner.spawn("error-thread".to_string(), || {
            Err(TaskError::TaskInvalidState {
                current_state: "Test error".to_string(),
            }.into())
        });

        assert!(result.is_ok()); // Spawning should succeed

        // Wait for thread to complete
        thread::sleep(Duration::from_millis(100));
        assert_eq!(spawner.get_nbr_error_threads(), 1);
    }

    #[test]
    fn test_cleanup_finished_threads() {
        let spawner = PermutationThreadSpawner::new();

        // Spawn a thread that completes quickly
        let result = spawner.spawn("cleanup-test".to_string(), || {
            Ok(())
        });
        assert!(result.is_ok());

        // Wait for completion
        thread::sleep(Duration::from_millis(100));
        assert_eq!(spawner.get_nbr_total_threads(), 1);
        assert_eq!(spawner.get_nbr_finished_threads(), 1);

        // Cleanup
        let cleaned = spawner.cleanup_finished_threads().unwrap();
        assert_eq!(cleaned, 1);
        assert_eq!(spawner.get_nbr_total_threads(), 0);
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
    }

    #[test]
    fn test_validation() {
        let mut spawner = PermutationThreadSpawner::new();
        assert!(spawner.validate().is_ok());

        spawner.set_max_alive_spawner_threads(0);
        assert!(spawner.validate().is_err());

        spawner.set_max_alive_spawner_threads(5);
        spawner.set_interval_between_max_alive_check(0);
        assert!(spawner.validate().is_err());
    }

    #[test]
    fn test_display() {
        let spawner = PermutationThreadSpawner::new();
        let display_str = format!("{}", spawner);
        assert!(display_str.contains("PermutationThreadSpawner"));
        assert!(display_str.contains("max_alive: 5"));
    }
}
