//! Task Report Model
//!
//! This module defines the TaskReport struct which represents a progress report
//! for task execution, including thread management, completion status, and timing information.

use crate::enums::Status;
use crate::errors::{Result, TaskError};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Represents a comprehensive progress report for task execution
///
/// TaskReport provides detailed information about task execution progress,
/// including thread management statistics, completion percentage, timing data,
/// and current status. This is typically used for monitoring and reporting
/// task execution state to clients or management systems.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskReport {
    /// Unique identifier for the task
    task_id: String,
    
    /// Client identifier associated with this task
    client_id: String,
    
    /// Current status of the task execution
    status: String,
    
    /// Number of currently running threads
    nbr_running_threads: i32,
    
    /// Number of threads waiting in the queue
    nbr_queued_threads: i32,
    
    /// Number of threads that have completed execution
    nbr_completed_threads: i32,
    
    /// Total number of panels being processed
    nbr_panels: i32,
    
    /// Completion percentage (0-100)
    percentage_done: i32,
    
    /// Elapsed time since task start (formatted string)
    elapsed_time: String,
}

impl TaskReport {
    /// Creates a new TaskReport with the specified task and client IDs
    ///
    /// # Arguments
    /// * `task_id` - Unique identifier for the task
    /// * `client_id` - Client identifier associated with this task
    ///
    /// # Returns
    /// A new TaskReport with default values
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::models::task::TaskReport;
    ///
    /// let report = TaskReport::new("task-001".to_string(), "client-123".to_string());
    /// assert_eq!(report.get_task_id(), "task-001");
    /// assert_eq!(report.get_client_id(), "client-123");
    /// assert_eq!(report.get_percentage_done(), 0);
    /// ```
    pub fn new(task_id: String, client_id: String) -> Self {
        Self {
            task_id,
            client_id,
            status: Status::Idle.as_str().to_string(),
            nbr_running_threads: 0,
            nbr_queued_threads: 0,
            nbr_completed_threads: 0,
            nbr_panels: 0,
            percentage_done: 0,
            elapsed_time: "0ms".to_string(),
        }
    }

    /// Creates a TaskReport from an existing Task
    ///
    /// # Arguments
    /// * `task` - The task to create a report from
    ///
    /// # Returns
    /// A TaskReport populated with data from the task
    pub fn from_task(task: &crate::models::task::Task) -> Self {
        let elapsed_ms = task.get_elapsed_time();
        let elapsed_time = Self::format_elapsed_time(elapsed_ms);
        
        Self {
            task_id: task.id.clone(),
            client_id: task.client_info
                .as_ref()
                .and_then(|ci| ci.id.clone())
                .unwrap_or_else(|| "unknown".to_string()),
            status: task.status.as_str().to_string(),
            nbr_running_threads: task.get_nbr_running_threads(),
            nbr_queued_threads: task.get_nbr_queued_threads(),
            nbr_completed_threads: task.get_nbr_finished_threads(),
            nbr_panels: task.solution
                .as_ref()
                .and_then(|s| s.panels.as_ref())
                .map(|panels| panels.len() as i32)
                .unwrap_or(0),
            percentage_done: task.get_percentage_done(),
            elapsed_time,
        }
    }

    /// Gets the task ID
    ///
    /// # Returns
    /// Reference to the task ID string
    pub fn get_task_id(&self) -> &str {
        &self.task_id
    }

    /// Sets the task ID
    ///
    /// # Arguments
    /// * `task_id` - The new task ID
    ///
    /// # Returns
    /// `Ok(())` if successful, `Err(TaskError)` if the ID is invalid
    pub fn set_task_id(&mut self, task_id: String) -> Result<()> {
        if task_id.trim().is_empty() {
            return Err(TaskError::TaskInvalidId { task_id }.into());
        }
        self.task_id = task_id;
        Ok(())
    }

    /// Gets the client ID
    ///
    /// # Returns
    /// Reference to the client ID string
    pub fn get_client_id(&self) -> &str {
        &self.client_id
    }

    /// Sets the client ID
    ///
    /// # Arguments
    /// * `client_id` - The new client ID
    pub fn set_client_id(&mut self, client_id: String) {
        self.client_id = client_id;
    }

    /// Gets the current status
    ///
    /// # Returns
    /// Reference to the status string
    pub fn get_status(&self) -> &str {
        &self.status
    }

    /// Sets the status
    ///
    /// # Arguments
    /// * `status` - The new status string
    pub fn set_status(&mut self, status: String) {
        self.status = status;
    }

    /// Sets the status from a Status enum
    ///
    /// # Arguments
    /// * `status` - The Status enum value
    pub fn set_status_enum(&mut self, status: Status) {
        self.status = status.as_str().to_string();
    }

    /// Gets the number of running threads
    ///
    /// # Returns
    /// Number of currently running threads
    pub fn get_nbr_running_threads(&self) -> i32 {
        self.nbr_running_threads
    }

    /// Sets the number of running threads
    ///
    /// # Arguments
    /// * `count` - The number of running threads
    ///
    /// # Returns
    /// `Ok(())` if successful, `Err(TaskError)` if count is negative
    pub fn set_nbr_running_threads(&mut self, count: i32) -> Result<()> {
        if count < 0 {
            return Err(TaskError::TaskInvalidState {
                current_state: format!("Negative running thread count: {}", count),
            }.into());
        }
        self.nbr_running_threads = count;
        Ok(())
    }

    /// Gets the number of queued threads
    ///
    /// # Returns
    /// Number of threads waiting in the queue
    pub fn get_nbr_queued_threads(&self) -> i32 {
        self.nbr_queued_threads
    }

    /// Sets the number of queued threads
    ///
    /// # Arguments
    /// * `count` - The number of queued threads
    ///
    /// # Returns
    /// `Ok(())` if successful, `Err(TaskError)` if count is negative
    pub fn set_nbr_queued_threads(&mut self, count: i32) -> Result<()> {
        if count < 0 {
            return Err(TaskError::TaskInvalidState {
                current_state: format!("Negative queued thread count: {}", count),
            }.into());
        }
        self.nbr_queued_threads = count;
        Ok(())
    }

    /// Gets the number of completed threads
    ///
    /// # Returns
    /// Number of threads that have completed execution
    pub fn get_nbr_completed_threads(&self) -> i32 {
        self.nbr_completed_threads
    }

    /// Sets the number of completed threads
    ///
    /// # Arguments
    /// * `count` - The number of completed threads
    ///
    /// # Returns
    /// `Ok(())` if successful, `Err(TaskError)` if count is negative
    pub fn set_nbr_completed_threads(&mut self, count: i32) -> Result<()> {
        if count < 0 {
            return Err(TaskError::TaskInvalidState {
                current_state: format!("Negative completed thread count: {}", count),
            }.into());
        }
        self.nbr_completed_threads = count;
        Ok(())
    }

    /// Gets the number of panels
    ///
    /// # Returns
    /// Total number of panels being processed
    pub fn get_nbr_panels(&self) -> i32 {
        self.nbr_panels
    }

    /// Sets the number of panels
    ///
    /// # Arguments
    /// * `count` - The number of panels
    ///
    /// # Returns
    /// `Ok(())` if successful, `Err(TaskError)` if count is negative
    pub fn set_nbr_panels(&mut self, count: i32) -> Result<()> {
        if count < 0 {
            return Err(TaskError::TaskInvalidState {
                current_state: format!("Negative panel count: {}", count),
            }.into());
        }
        self.nbr_panels = count;
        Ok(())
    }

    /// Gets the completion percentage
    ///
    /// # Returns
    /// Completion percentage (0-100)
    pub fn get_percentage_done(&self) -> i32 {
        self.percentage_done
    }

    /// Sets the completion percentage
    ///
    /// # Arguments
    /// * `percentage` - The completion percentage (0-100)
    ///
    /// # Returns
    /// `Ok(())` if successful, `Err(TaskError)` if percentage is out of range
    pub fn set_percentage_done(&mut self, percentage: i32) -> Result<()> {
        if !(0..=100).contains(&percentage) {
            return Err(TaskError::TaskInvalidState {
                current_state: format!("Invalid percentage: {} (must be 0-100)", percentage),
            }.into());
        }
        self.percentage_done = percentage;
        Ok(())
    }

    /// Gets the elapsed time string
    ///
    /// # Returns
    /// Reference to the formatted elapsed time string
    pub fn get_elapsed_time(&self) -> &str {
        &self.elapsed_time
    }

    /// Sets the elapsed time string
    ///
    /// # Arguments
    /// * `elapsed_time` - The formatted elapsed time string
    pub fn set_elapsed_time(&mut self, elapsed_time: String) {
        self.elapsed_time = elapsed_time;
    }

    /// Sets the elapsed time from milliseconds
    ///
    /// # Arguments
    /// * `elapsed_ms` - Elapsed time in milliseconds
    pub fn set_elapsed_time_ms(&mut self, elapsed_ms: u64) {
        self.elapsed_time = Self::format_elapsed_time(elapsed_ms);
    }

    /// Gets the total number of threads (running + queued + completed)
    ///
    /// # Returns
    /// Total thread count
    pub fn get_total_threads(&self) -> i32 {
        self.nbr_running_threads + self.nbr_queued_threads + self.nbr_completed_threads
    }

    /// Checks if the task is currently active (has running or queued threads)
    ///
    /// # Returns
    /// `true` if the task has active threads, `false` otherwise
    pub fn is_active(&self) -> bool {
        self.nbr_running_threads > 0 || self.nbr_queued_threads > 0
    }

    /// Checks if the task is completed (100% done)
    ///
    /// # Returns
    /// `true` if the task is 100% complete, `false` otherwise
    pub fn is_completed(&self) -> bool {
        self.percentage_done >= 100
    }

    /// Updates the report with current thread counts and percentage
    ///
    /// # Arguments
    /// * `running` - Number of running threads
    /// * `queued` - Number of queued threads
    /// * `completed` - Number of completed threads
    /// * `percentage` - Completion percentage
    ///
    /// # Returns
    /// `Ok(())` if successful, `Err(TaskError)` if any values are invalid
    pub fn update_progress(
        &mut self,
        running: i32,
        queued: i32,
        completed: i32,
        percentage: i32,
    ) -> Result<()> {
        self.set_nbr_running_threads(running)?;
        self.set_nbr_queued_threads(queued)?;
        self.set_nbr_completed_threads(completed)?;
        self.set_percentage_done(percentage)?;
        Ok(())
    }

    /// Validates the task report for consistency
    ///
    /// # Returns
    /// `Ok(())` if valid, `Err(TaskError)` if inconsistent
    pub fn validate(&self) -> Result<()> {
        // Check that task ID is not empty
        if self.task_id.trim().is_empty() {
            return Err(TaskError::TaskInvalidId {
                task_id: self.task_id.clone(),
            }.into());
        }

        // Check that thread counts are non-negative
        if self.nbr_running_threads < 0 {
            return Err(TaskError::TaskInvalidState {
                current_state: format!("Negative running threads: {}", self.nbr_running_threads),
            }.into());
        }

        if self.nbr_queued_threads < 0 {
            return Err(TaskError::TaskInvalidState {
                current_state: format!("Negative queued threads: {}", self.nbr_queued_threads),
            }.into());
        }

        if self.nbr_completed_threads < 0 {
            return Err(TaskError::TaskInvalidState {
                current_state: format!("Negative completed threads: {}", self.nbr_completed_threads),
            }.into());
        }

        // Check that panel count is non-negative
        if self.nbr_panels < 0 {
            return Err(TaskError::TaskInvalidState {
                current_state: format!("Negative panel count: {}", self.nbr_panels),
            }.into());
        }

        // Check that percentage is in valid range
        if !(0..=100).contains(&self.percentage_done) {
            return Err(TaskError::TaskInvalidState {
                current_state: format!("Invalid percentage: {}", self.percentage_done),
            }.into());
        }

        Ok(())
    }

    /// Formats elapsed time from milliseconds to a human-readable string
    ///
    /// # Arguments
    /// * `elapsed_ms` - Elapsed time in milliseconds
    ///
    /// # Returns
    /// Formatted time string (e.g., "1h 23m 45s", "2m 30s", "500ms")
    fn format_elapsed_time(elapsed_ms: u64) -> String {
        if elapsed_ms < 1000 {
            format!("{}ms", elapsed_ms)
        } else if elapsed_ms < 60_000 {
            let seconds = elapsed_ms / 1000;
            let ms = elapsed_ms % 1000;
            if ms == 0 {
                format!("{}s", seconds)
            } else {
                format!("{}.{:03}s", seconds, ms)
            }
        } else if elapsed_ms < 3_600_000 {
            let minutes = elapsed_ms / 60_000;
            let seconds = (elapsed_ms % 60_000) / 1000;
            if seconds == 0 {
                format!("{}m", minutes)
            } else {
                format!("{}m {}s", minutes, seconds)
            }
        } else {
            let hours = elapsed_ms / 3_600_000;
            let minutes = (elapsed_ms % 3_600_000) / 60_000;
            let seconds = (elapsed_ms % 60_000) / 1000;
            
            if minutes == 0 && seconds == 0 {
                format!("{}h", hours)
            } else if seconds == 0 {
                format!("{}h {}m", hours, minutes)
            } else {
                format!("{}h {}m {}s", hours, minutes, seconds)
            }
        }
    }

    /// Parses elapsed time string back to milliseconds
    ///
    /// # Arguments
    /// * `time_str` - Formatted time string
    ///
    /// # Returns
    /// `Ok(milliseconds)` if successful, `Err(TaskError)` if parsing fails
    pub fn parse_elapsed_time(time_str: &str) -> Result<u64> {
        if time_str.ends_with("ms") {
            let ms_str = &time_str[..time_str.len() - 2];
            ms_str.parse::<u64>()
                .map_err(|_| TaskError::TaskInvalidState {
                    current_state: format!("Invalid time format: {}", time_str),
                }.into())
        } else if time_str.ends_with('s') && !time_str.contains('m') && !time_str.contains('h') {
            let s_str = &time_str[..time_str.len() - 1];
            if let Ok(seconds) = s_str.parse::<f64>() {
                Ok((seconds * 1000.0) as u64)
            } else {
                Err(TaskError::TaskInvalidState {
                    current_state: format!("Invalid time format: {}", time_str),
                }.into())
            }
        } else {
            // For complex formats like "1h 23m 45s", we'd need more parsing logic
            // For now, return an error for unsupported formats
            Err(TaskError::TaskInvalidState {
                current_state: format!("Unsupported time format: {}", time_str),
            }.into())
        }
    }
}

impl Default for TaskReport {
    fn default() -> Self {
        Self::new("default-task".to_string(), "default-client".to_string())
    }
}

impl fmt::Display for TaskReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TaskReport {{ task: {}, client: {}, status: {}, progress: {}%, threads: {}/{}/{}, panels: {}, elapsed: {} }}",
            self.task_id,
            self.client_id,
            self.status,
            self.percentage_done,
            self.nbr_running_threads,
            self.nbr_queued_threads,
            self.nbr_completed_threads,
            self.nbr_panels,
            self.elapsed_time
        )
    }
}
