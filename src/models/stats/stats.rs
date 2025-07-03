//! Stats Model
//!
//! This module defines the Stats struct which represents comprehensive statistics
//! for task and thread management, including counts of various task states and
//! associated task reports.

use crate::errors::{Result, TaskError};
use crate::models::task::TaskReport;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents comprehensive statistics for task and thread management
///
/// Stats provides detailed information about task execution statistics,
/// including counts of tasks in various states (idle, running, finished, etc.),
/// thread management statistics, and a collection of detailed task reports.
/// This is typically used for monitoring and reporting overall system state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    /// Number of tasks currently in idle state
    nbr_idle_tasks: u64,
    
    /// Number of tasks currently running
    nbr_running_tasks: u64,
    
    /// Number of tasks that have finished successfully
    nbr_finished_tasks: u64,
    
    /// Number of tasks that have been stopped
    nbr_stopped_tasks: u64,
    
    /// Number of tasks that have been terminated
    nbr_terminated_tasks: u64,
    
    /// Number of tasks that have encountered errors
    nbr_error_tasks: u64,
    
    /// Number of threads currently running
    nbr_running_threads: u32,
    
    /// Number of threads waiting in the queue
    nbr_queued_threads: u32,
    
    /// Number of threads that have finished execution
    nbr_finished_threads: u64,
    
    /// Collection of detailed task reports
    task_reports: Vec<TaskReport>,
}

impl Stats {
    /// Creates a new Stats instance with all counters initialized to zero
    ///
    /// # Returns
    /// A new Stats instance with default values
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::models::task::Stats;
    ///
    /// let stats = Stats::new();
    /// assert_eq!(stats.get_nbr_idle_tasks(), 0);
    /// assert_eq!(stats.get_nbr_running_tasks(), 0);
    /// assert!(stats.get_task_reports().is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            nbr_idle_tasks: 0,
            nbr_running_tasks: 0,
            nbr_finished_tasks: 0,
            nbr_stopped_tasks: 0,
            nbr_terminated_tasks: 0,
            nbr_error_tasks: 0,
            nbr_running_threads: 0,
            nbr_queued_threads: 0,
            nbr_finished_threads: 0,
            task_reports: Vec::new(),
        }
    }

    /// Gets the number of idle tasks
    ///
    /// # Returns
    /// Number of tasks currently in idle state
    pub fn get_nbr_idle_tasks(&self) -> u64 {
        self.nbr_idle_tasks
    }

    /// Sets the number of idle tasks
    ///
    /// # Arguments
    /// * `count` - The number of idle tasks
    pub fn set_nbr_idle_tasks(&mut self, count: u64) {
        self.nbr_idle_tasks = count;
    }

    /// Gets the number of running tasks
    ///
    /// # Returns
    /// Number of tasks currently running
    pub fn get_nbr_running_tasks(&self) -> u64 {
        self.nbr_running_tasks
    }

    /// Sets the number of running tasks
    ///
    /// # Arguments
    /// * `count` - The number of running tasks
    pub fn set_nbr_running_tasks(&mut self, count: u64) {
        self.nbr_running_tasks = count;
    }

    /// Gets the number of finished tasks
    ///
    /// # Returns
    /// Number of tasks that have finished successfully
    pub fn get_nbr_finished_tasks(&self) -> u64 {
        self.nbr_finished_tasks
    }

    /// Sets the number of finished tasks
    ///
    /// # Arguments
    /// * `count` - The number of finished tasks
    pub fn set_nbr_finished_tasks(&mut self, count: u64) {
        self.nbr_finished_tasks = count;
    }

    /// Gets the number of stopped tasks
    ///
    /// # Returns
    /// Number of tasks that have been stopped
    pub fn get_nbr_stopped_tasks(&self) -> u64 {
        self.nbr_stopped_tasks
    }

    /// Sets the number of stopped tasks
    ///
    /// # Arguments
    /// * `count` - The number of stopped tasks
    pub fn set_nbr_stopped_tasks(&mut self, count: u64) {
        self.nbr_stopped_tasks = count;
    }

    /// Gets the number of terminated tasks
    ///
    /// # Returns
    /// Number of tasks that have been terminated
    pub fn get_nbr_terminated_tasks(&self) -> u64 {
        self.nbr_terminated_tasks
    }

    /// Sets the number of terminated tasks
    ///
    /// # Arguments
    /// * `count` - The number of terminated tasks
    pub fn set_nbr_terminated_tasks(&mut self, count: u64) {
        self.nbr_terminated_tasks = count;
    }

    /// Gets the number of error tasks
    ///
    /// # Returns
    /// Number of tasks that have encountered errors
    pub fn get_nbr_error_tasks(&self) -> u64 {
        self.nbr_error_tasks
    }

    /// Sets the number of error tasks
    ///
    /// # Arguments
    /// * `count` - The number of error tasks
    pub fn set_nbr_error_tasks(&mut self, count: u64) {
        self.nbr_error_tasks = count;
    }

    /// Gets the number of running threads
    ///
    /// # Returns
    /// Number of threads currently running
    pub fn get_nbr_running_threads(&self) -> u32 {
        self.nbr_running_threads
    }

    /// Sets the number of running threads
    ///
    /// # Arguments
    /// * `count` - The number of running threads
    pub fn set_nbr_running_threads(&mut self, count: u32) {
        self.nbr_running_threads = count;
    }

    /// Gets the number of queued threads
    ///
    /// # Returns
    /// Number of threads waiting in the queue
    pub fn get_nbr_queued_threads(&self) -> u32 {
        self.nbr_queued_threads
    }

    /// Sets the number of queued threads
    ///
    /// # Arguments
    /// * `count` - The number of queued threads
    pub fn set_nbr_queued_threads(&mut self, count: u32) {
        self.nbr_queued_threads = count;
    }

    /// Gets the number of finished threads
    ///
    /// # Returns
    /// Number of threads that have finished execution
    pub fn get_nbr_finished_threads(&self) -> u64 {
        self.nbr_finished_threads
    }

    /// Sets the number of finished threads
    ///
    /// # Arguments
    /// * `count` - The number of finished threads
    pub fn set_nbr_finished_threads(&mut self, count: u64) {
        self.nbr_finished_threads = count;
    }

    /// Gets a reference to the task reports
    ///
    /// # Returns
    /// Reference to the vector of task reports
    pub fn get_task_reports(&self) -> &Vec<TaskReport> {
        &self.task_reports
    }

    /// Gets a mutable reference to the task reports
    ///
    /// # Returns
    /// Mutable reference to the vector of task reports
    pub fn get_task_reports_mut(&mut self) -> &mut Vec<TaskReport> {
        &mut self.task_reports
    }

    /// Sets the task reports
    ///
    /// # Arguments
    /// * `reports` - Vector of task reports
    pub fn set_task_reports(&mut self, reports: Vec<TaskReport>) {
        self.task_reports = reports;
    }

    /// Adds a task report to the collection
    ///
    /// # Arguments
    /// * `report` - The task report to add
    pub fn add_task_report(&mut self, report: TaskReport) {
        self.task_reports.push(report);
    }

    /// Removes a task report by task ID
    ///
    /// # Arguments
    /// * `task_id` - The ID of the task report to remove
    ///
    /// # Returns
    /// `Some(TaskReport)` if found and removed, `None` if not found
    pub fn remove_task_report(&mut self, task_id: &str) -> Option<TaskReport> {
        if let Some(pos) = self.task_reports.iter().position(|r| r.get_task_id() == task_id) {
            Some(self.task_reports.remove(pos))
        } else {
            None
        }
    }

    /// Finds a task report by task ID
    ///
    /// # Arguments
    /// * `task_id` - The ID of the task report to find
    ///
    /// # Returns
    /// `Some(&TaskReport)` if found, `None` if not found
    pub fn find_task_report(&self, task_id: &str) -> Option<&TaskReport> {
        self.task_reports.iter().find(|r| r.get_task_id() == task_id)
    }

    /// Finds a mutable task report by task ID
    ///
    /// # Arguments
    /// * `task_id` - The ID of the task report to find
    ///
    /// # Returns
    /// `Some(&mut TaskReport)` if found, `None` if not found
    pub fn find_task_report_mut(&mut self, task_id: &str) -> Option<&mut TaskReport> {
        self.task_reports.iter_mut().find(|r| r.get_task_id() == task_id)
    }

    /// Gets the total number of tasks across all states
    ///
    /// # Returns
    /// Total number of tasks
    pub fn get_total_tasks(&self) -> u64 {
        self.nbr_idle_tasks
            + self.nbr_running_tasks
            + self.nbr_finished_tasks
            + self.nbr_stopped_tasks
            + self.nbr_terminated_tasks
            + self.nbr_error_tasks
    }

    /// Gets the total number of threads across all states
    ///
    /// # Returns
    /// Total number of threads
    pub fn get_total_threads(&self) -> u64 {
        self.nbr_running_threads as u64 + self.nbr_queued_threads as u64 + self.nbr_finished_threads
    }

    /// Gets the number of active tasks (idle + running)
    ///
    /// # Returns
    /// Number of active tasks
    pub fn get_active_tasks(&self) -> u64 {
        self.nbr_idle_tasks + self.nbr_running_tasks
    }

    /// Gets the number of completed tasks (finished + stopped + terminated + error)
    ///
    /// # Returns
    /// Number of completed tasks
    pub fn get_completed_tasks(&self) -> u64 {
        self.nbr_finished_tasks + self.nbr_stopped_tasks + self.nbr_terminated_tasks + self.nbr_error_tasks
    }

    /// Checks if there are any active tasks or threads
    ///
    /// # Returns
    /// `true` if there are active tasks or threads, `false` otherwise
    pub fn has_activity(&self) -> bool {
        self.get_active_tasks() > 0 || self.nbr_running_threads > 0 || self.nbr_queued_threads > 0
    }

    /// Updates all task counts at once
    ///
    /// # Arguments
    /// * `idle` - Number of idle tasks
    /// * `running` - Number of running tasks
    /// * `finished` - Number of finished tasks
    /// * `stopped` - Number of stopped tasks
    /// * `terminated` - Number of terminated tasks
    /// * `error` - Number of error tasks
    pub fn update_task_counts(
        &mut self,
        idle: u64,
        running: u64,
        finished: u64,
        stopped: u64,
        terminated: u64,
        error: u64,
    ) {
        self.nbr_idle_tasks = idle;
        self.nbr_running_tasks = running;
        self.nbr_finished_tasks = finished;
        self.nbr_stopped_tasks = stopped;
        self.nbr_terminated_tasks = terminated;
        self.nbr_error_tasks = error;
    }

    /// Updates all thread counts at once
    ///
    /// # Arguments
    /// * `running` - Number of running threads
    /// * `queued` - Number of queued threads
    /// * `finished` - Number of finished threads
    pub fn update_thread_counts(&mut self, running: u32, queued: u32, finished: u64) {
        self.nbr_running_threads = running;
        self.nbr_queued_threads = queued;
        self.nbr_finished_threads = finished;
    }

    /// Clears all statistics and task reports
    pub fn clear(&mut self) {
        self.nbr_idle_tasks = 0;
        self.nbr_running_tasks = 0;
        self.nbr_finished_tasks = 0;
        self.nbr_stopped_tasks = 0;
        self.nbr_terminated_tasks = 0;
        self.nbr_error_tasks = 0;
        self.nbr_running_threads = 0;
        self.nbr_queued_threads = 0;
        self.nbr_finished_threads = 0;
        self.task_reports.clear();
    }

    /// Validates the statistics for consistency
    ///
    /// # Returns
    /// `Ok(())` if valid, `Err(TaskError)` if inconsistent
    pub fn validate(&self) -> Result<()> {
        // Validate that task report count doesn't exceed total tasks
        let total_tasks = self.get_total_tasks();
        if self.task_reports.len() as u64 > total_tasks && total_tasks > 0 {
            return Err(TaskError::TaskInvalidState {
                current_state: format!(
                    "Task report count ({}) exceeds total task count ({})",
                    self.task_reports.len(),
                    total_tasks
                ),
            }.into());
        }

        // Validate each task report
        for report in &self.task_reports {
            report.validate()?;
        }

        Ok(())
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Stats {{ tasks: {}/{}/{}/{}/{}/{} (idle/running/finished/stopped/terminated/error), threads: {}/{}/{} (running/queued/finished), reports: {} }}",
            self.nbr_idle_tasks,
            self.nbr_running_tasks,
            self.nbr_finished_tasks,
            self.nbr_stopped_tasks,
            self.nbr_terminated_tasks,
            self.nbr_error_tasks,
            self.nbr_running_threads,
            self.nbr_queued_threads,
            self.nbr_finished_threads,
            self.task_reports.len()
        )
    }
}
