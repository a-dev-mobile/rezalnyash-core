use serde::{Deserialize, Serialize};
use crate::errors::core_errors::CoreError;

/// Performance thresholds configuration for task and thread management
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    /// Maximum number of simultaneous tasks (default: 1)
    max_simultaneous_tasks: u32,
    /// Maximum number of simultaneous threads
    max_simultaneous_threads: u32,
    /// Thread check interval in milliseconds
    thread_check_interval: u64,
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            max_simultaneous_tasks: 1,
            max_simultaneous_threads: 0,
            thread_check_interval: 0,
        }
    }
}

impl PerformanceThresholds {
    /// Creates a new PerformanceThresholds with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new PerformanceThresholds with specified threads and interval
    pub fn with_config(max_simultaneous_threads: u32, thread_check_interval: u64) -> Self {
        Self {
            max_simultaneous_tasks: 1, // Default value from Java
            max_simultaneous_threads,
            thread_check_interval,
        }
    }

    /// Gets the thread check interval in milliseconds
    pub fn get_thread_check_interval(&self) -> u64 {
        self.thread_check_interval
    }

    /// Sets the thread check interval in milliseconds
    /// Returns an error if the interval is unreasonably large
    pub fn set_thread_check_interval(&mut self, interval: u64) -> Result<(), CoreError> {
        // Validate that interval is reasonable (less than 24 hours in milliseconds)
        const MAX_INTERVAL_MS: u64 = 24 * 60 * 60 * 1000; // 24 hours
        
        if interval > MAX_INTERVAL_MS {
            return Err(CoreError::InvalidInput {
                details: format!("Thread check interval {} ms exceeds maximum allowed {} ms", 
                               interval, MAX_INTERVAL_MS)
            });
        }

        self.thread_check_interval = interval;
        Ok(())
    }

    /// Gets the maximum number of simultaneous threads
    pub fn get_max_simultaneous_threads(&self) -> u32 {
        self.max_simultaneous_threads
    }

    /// Sets the maximum number of simultaneous threads
    /// Returns an error if the value is invalid
    pub fn set_max_simultaneous_threads(&mut self, threads: u32) -> Result<(), CoreError> {
        // Validate reasonable thread count (1-1000)
        if threads == 0 {
            return Err(CoreError::InvalidInput {
                details: "Maximum simultaneous threads must be greater than 0".to_string()
            });
        }

        if threads > 1000 {
            return Err(CoreError::InvalidInput {
                details: format!("Maximum simultaneous threads {} exceeds reasonable limit of 1000", threads)
            });
        }

        self.max_simultaneous_threads = threads;
        Ok(())
    }

    /// Gets the maximum number of simultaneous tasks
    pub fn get_max_simultaneous_tasks(&self) -> u32 {
        self.max_simultaneous_tasks
    }

    /// Sets the maximum number of simultaneous tasks
    /// Returns an error if the value is invalid
    pub fn set_max_simultaneous_tasks(&mut self, tasks: u32) -> Result<(), CoreError> {
        // Validate reasonable task count (1-10000)
        if tasks == 0 {
            return Err(CoreError::InvalidInput {
                details: "Maximum simultaneous tasks must be greater than 0".to_string()
            });
        }

        if tasks > 10000 {
            return Err(CoreError::InvalidInput {
                details: format!("Maximum simultaneous tasks {} exceeds reasonable limit of 10000", tasks)
            });
        }

        self.max_simultaneous_tasks = tasks;
        Ok(())
    }

    /// Builder pattern methods for fluent construction
    pub fn max_simultaneous_tasks(mut self, tasks: u32) -> Result<Self, CoreError> {
        self.set_max_simultaneous_tasks(tasks)?;
        Ok(self)
    }

    pub fn max_simultaneous_threads(mut self, threads: u32) -> Result<Self, CoreError> {
        self.set_max_simultaneous_threads(threads)?;
        Ok(self)
    }

    pub fn thread_check_interval(mut self, interval: u64) -> Result<Self, CoreError> {
        self.set_thread_check_interval(interval)?;
        Ok(self)
    }

    /// Validates that the configuration is internally consistent
    pub fn validate(&self) -> Result<(), CoreError> {
        if self.max_simultaneous_threads == 0 && self.thread_check_interval > 0 {
            return Err(CoreError::InvalidConfiguration {
                message: "Thread check interval is set but max simultaneous threads is 0".to_string()
            });
        }

        if self.max_simultaneous_threads > 0 && self.thread_check_interval == 0 {
            return Err(CoreError::InvalidConfiguration {
                message: "Max simultaneous threads is set but thread check interval is 0".to_string()
            });
        }

        Ok(())
    }

    /// Returns a summary of the performance thresholds for logging
    pub fn summary(&self) -> String {
        format!(
            "PerformanceThresholds(tasks: {}, threads: {}, interval: {}ms)",
            self.max_simultaneous_tasks,
            self.max_simultaneous_threads,
            self.thread_check_interval
        )
    }

    /// Checks if threading is enabled (both threads > 0 and interval > 0)
    pub fn is_threading_enabled(&self) -> bool {
        self.max_simultaneous_threads > 0 && self.thread_check_interval > 0
    }

    /// Gets the thread check interval as a Duration
    pub fn get_thread_check_duration(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.thread_check_interval)
    }

    /// Calculates the theoretical maximum concurrent operations
    pub fn max_concurrent_operations(&self) -> u32 {
        if self.is_threading_enabled() {
            self.max_simultaneous_tasks * self.max_simultaneous_threads
        } else {
            self.max_simultaneous_tasks
        }
    }
}
