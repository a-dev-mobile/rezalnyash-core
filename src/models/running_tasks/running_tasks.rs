//! Running Tasks Model
//!
//! This module provides the RunningTasks struct which manages a collection of tasks
//! and running threads in a thread-safe manner. It implements the singleton pattern
//! using Rust's lazy_static and provides comprehensive task lifecycle management.

use crate::enums::Status;
use crate::errors::{Result, TaskError};
use crate::models::{
    cut_list_thread::CutListThread,
    task::Task,
};
use std::sync::{Arc, Mutex, OnceLock};
use std::collections::HashMap;

/// Thread-safe singleton for managing running tasks and threads
///
/// This struct provides centralized management of computational tasks and their
/// associated threads, including lifecycle tracking, status monitoring, and
/// archival statistics.
#[derive(Debug)]
pub struct RunningTasks {
    /// List of active tasks
    tasks: Arc<Mutex<Vec<Task>>>,
    /// List of running cut list threads
    running_threads: Arc<Mutex<Vec<CutListThread>>>,
    /// Total number of tasks created
    nbr_total_tasks: Arc<Mutex<u64>>,
    /// Number of archived finished tasks
    nbr_archived_finished_tasks: Arc<Mutex<u64>>,
    /// Number of archived stopped tasks
    nbr_archived_stopped_tasks: Arc<Mutex<u64>>,
    /// Number of archived terminated tasks
    nbr_archived_terminated_tasks: Arc<Mutex<u64>>,
    /// Number of archived error tasks
    nbr_archived_error_tasks: Arc<Mutex<u64>>,
}

impl RunningTasks {
    /// Creates a new RunningTasks instance
    ///
    /// # Returns
    /// A new RunningTasks with empty collections and zero counters
    fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(Vec::new())),
            running_threads: Arc::new(Mutex::new(Vec::new())),
            nbr_total_tasks: Arc::new(Mutex::new(0)),
            nbr_archived_finished_tasks: Arc::new(Mutex::new(0)),
            nbr_archived_stopped_tasks: Arc::new(Mutex::new(0)),
            nbr_archived_terminated_tasks: Arc::new(Mutex::new(0)),
            nbr_archived_error_tasks: Arc::new(Mutex::new(0)),
        }
    }

    /// Gets the singleton instance of RunningTasks
    ///
    /// # Returns
    /// Reference to the singleton RunningTasks instance
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::models::running_tasks::RunningTasks;
    ///
    /// let instance = RunningTasks::get_instance();
    /// let total_tasks = instance.get_nbr_total_tasks().unwrap_or(0);
    /// ```
    pub fn get_instance() -> &'static RunningTasks {
        static INSTANCE: OnceLock<RunningTasks> = OnceLock::new();
        INSTANCE.get_or_init(RunningTasks::new)
    }

    /// Gets the total number of tasks created
    ///
    /// # Returns
    /// `Ok(u64)` with the total task count, or `Err` if lock acquisition fails
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::models::running_tasks::RunningTasks;
    ///
    /// let instance = RunningTasks::get_instance();
    /// match instance.get_nbr_total_tasks() {
    ///     Ok(count) => println!("Total tasks: {}", count),
    ///     Err(e) => eprintln!("Failed to get task count: {}", e),
    /// }
    /// ```
    pub fn get_nbr_total_tasks(&self) -> Result<u64> {
        self.nbr_total_tasks
            .lock()
            .map(|guard| *guard)
            .map_err(|_| TaskError::TaskLockError {
                operation: "get_nbr_total_tasks".to_string(),
            }.into())
    }

    /// Gets an immutable copy of all tasks
    ///
    /// # Returns
    /// `Ok(Vec<Task>)` with a copy of all tasks, or `Err` if lock acquisition fails
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::models::running_tasks::RunningTasks;
    ///
    /// let instance = RunningTasks::get_instance();
    /// match instance.get_tasks() {
    ///     Ok(tasks) => println!("Found {} tasks", tasks.len()),
    ///     Err(e) => eprintln!("Failed to get tasks: {}", e),
    /// }
    /// ```
    pub fn get_tasks(&self) -> Result<Vec<Task>> {
        self.tasks
            .lock()
            .map(|guard| guard.clone())
            .map_err(|_| TaskError::TaskLockError {
                operation: "get_tasks".to_string(),
            }.into())
    }

    /// Removes all specified tasks from the active list and updates archive counters
    ///
    /// # Arguments
    /// * `tasks_to_remove` - Vector of tasks to remove
    ///
    /// # Returns
    /// `Ok(bool)` indicating if any tasks were removed, or `Err` if operation fails
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::models::running_tasks::RunningTasks;
    /// use rezalnyash_core::models::task::Task;
    /// use rezalnyash_core::enums::Status;
    ///
    /// let instance = RunningTasks::get_instance();
    /// let mut task = Task::new("test-task".to_string());
    /// task.status = Status::Finished;
    /// 
    /// match instance.remove_all_tasks(vec![task]) {
    ///     Ok(removed) => println!("Tasks removed: {}", removed),
    ///     Err(e) => eprintln!("Failed to remove tasks: {}", e),
    /// }
    /// ```
    pub fn remove_all_tasks(&self, tasks_to_remove: Vec<Task>) -> Result<bool> {
        let mut tasks_guard = self.tasks
            .lock()
            .map_err(|_| TaskError::TaskLockError {
                operation: "remove_all_tasks".to_string(),
            })?;

        let mut finished_guard = self.nbr_archived_finished_tasks
            .lock()
            .map_err(|_| TaskError::TaskLockError {
                operation: "remove_all_tasks_finished".to_string(),
            })?;

        let mut stopped_guard = self.nbr_archived_stopped_tasks
            .lock()
            .map_err(|_| TaskError::TaskLockError {
                operation: "remove_all_tasks_stopped".to_string(),
            })?;

        let mut terminated_guard = self.nbr_archived_terminated_tasks
            .lock()
            .map_err(|_| TaskError::TaskLockError {
                operation: "remove_all_tasks_terminated".to_string(),
            })?;

        let mut error_guard = self.nbr_archived_error_tasks
            .lock()
            .map_err(|_| TaskError::TaskLockError {
                operation: "remove_all_tasks_error".to_string(),
            })?;

        let mut any_removed = false;

        // Update archive counters based on task status
        for task in &tasks_to_remove {
            match task.status {
                Status::Finished => *finished_guard += 1,
                Status::Stopped => *stopped_guard += 1,
                Status::Terminated => *terminated_guard += 1,
                Status::Error => *error_guard += 1,
                _ => {} // Other statuses don't increment archive counters
            }
        }

        // Remove tasks from the active list
        let original_len = tasks_guard.len();
        tasks_guard.retain(|task| {
            !tasks_to_remove.iter().any(|remove_task| remove_task.id == task.id)
        });
        
        any_removed = tasks_guard.len() < original_len;

        Ok(any_removed)
    }

    /// Adds a new task to the active list
    ///
    /// # Arguments
    /// * `task` - The task to add
    ///
    /// # Returns
    /// `Ok(bool)` indicating success, or `Err` if operation fails
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::models::running_tasks::RunningTasks;
    /// use rezalnyash_core::models::task::Task;
    ///
    /// let instance = RunningTasks::get_instance();
    /// let task = Task::new("new-task".to_string());
    /// 
    /// match instance.add_task(task) {
    ///     Ok(added) => println!("Task added: {}", added),
    ///     Err(e) => eprintln!("Failed to add task: {}", e),
    /// }
    /// ```
    pub fn add_task(&self, task: Task) -> Result<bool> {
        let mut tasks_guard = self.tasks
            .lock()
            .map_err(|_| TaskError::TaskLockError {
                operation: "add_task".to_string(),
            })?;

        let mut total_guard = self.nbr_total_tasks
            .lock()
            .map_err(|_| TaskError::TaskLockError {
                operation: "add_task_total".to_string(),
            })?;

        // Check for duplicate task IDs
        if tasks_guard.iter().any(|existing_task| existing_task.id == task.id) {
            return Err(TaskError::TaskAlreadyExists {
                task_id: task.id,
            }.into());
        }

        tasks_guard.push(task);
        *total_guard += 1;

        Ok(true)
    }

    /// Finds a task by its ID
    ///
    /// # Arguments
    /// * `task_id` - The ID of the task to find
    ///
    /// # Returns
    /// `Ok(Option<Task>)` with the task if found, or `Err` if operation fails
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::models::running_tasks::RunningTasks;
    ///
    /// let instance = RunningTasks::get_instance();
    /// match instance.get_task("task-123") {
    ///     Ok(Some(task)) => println!("Found task: {}", task.id),
    ///     Ok(None) => println!("Task not found"),
    ///     Err(e) => eprintln!("Failed to get task: {}", e),
    /// }
    /// ```
    pub fn get_task(&self, task_id: &str) -> Result<Option<Task>> {
        let tasks_guard = self.tasks
            .lock()
            .map_err(|_| TaskError::TaskLockError {
                operation: "get_task".to_string(),
            })?;

        Ok(tasks_guard
            .iter()
            .find(|task| task.id == task_id)
            .cloned())
    }

    /// Gets the count of running threads
    ///
    /// # Returns
    /// `Ok(usize)` with the number of running threads, or `Err` if operation fails
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::models::running_tasks::RunningTasks;
    ///
    /// let instance = RunningTasks::get_instance();
    /// match instance.get_running_threads_count() {
    ///     Ok(count) => println!("Found {} running threads", count),
    ///     Err(e) => eprintln!("Failed to get thread count: {}", e),
    /// }
    /// ```
    pub fn get_running_threads_count(&self) -> Result<usize> {
        self.running_threads
            .lock()
            .map(|guard| guard.len())
            .map_err(|_| TaskError::TaskLockError {
                operation: "get_running_threads_count".to_string(),
            }.into())
    }

    /// Gets information about running threads (group names only)
    ///
    /// # Returns
    /// `Ok(Vec<Option<String>>)` with thread group names, or `Err` if operation fails
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::models::running_tasks::RunningTasks;
    ///
    /// let instance = RunningTasks::get_instance();
    /// match instance.get_running_thread_groups() {
    ///     Ok(groups) => println!("Thread groups: {:?}", groups),
    ///     Err(e) => eprintln!("Failed to get thread groups: {}", e),
    /// }
    /// ```
    pub fn get_running_thread_groups(&self) -> Result<Vec<Option<String>>> {
        self.running_threads
            .lock()
            .map(|guard| guard.iter().map(|thread| thread.get_group().cloned()).collect())
            .map_err(|_| TaskError::TaskLockError {
                operation: "get_running_thread_groups".to_string(),
            }.into())
    }

    /// Adds a running thread to the collection
    ///
    /// # Arguments
    /// * `thread` - The thread to add
    ///
    /// # Returns
    /// `Ok(bool)` indicating success, or `Err` if operation fails
    pub fn add_running_thread(&self, thread: CutListThread) -> Result<bool> {
        let mut threads_guard = self.running_threads
            .lock()
            .map_err(|_| TaskError::TaskLockError {
                operation: "add_running_thread".to_string(),
            })?;

        threads_guard.push(thread);
        Ok(true)
    }

    /// Removes a running thread from the collection
    ///
    /// # Arguments
    /// * `thread_id` - The ID of the thread to remove (using group as identifier)
    ///
    /// # Returns
    /// `Ok(bool)` indicating if a thread was removed, or `Err` if operation fails
    pub fn remove_running_thread(&self, thread_id: &str) -> Result<bool> {
        let mut threads_guard = self.running_threads
            .lock()
            .map_err(|_| TaskError::TaskLockError {
                operation: "remove_running_thread".to_string(),
            })?;

        let original_len = threads_guard.len();
        threads_guard.retain(|thread| {
            thread.get_group().map_or(true, |group| group != thread_id)
        });

        Ok(threads_guard.len() < original_len)
    }

    /// Gets the number of idle tasks
    ///
    /// # Returns
    /// `Ok(u64)` with the count of idle tasks, or `Err` if operation fails
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::models::running_tasks::RunningTasks;
    ///
    /// let instance = RunningTasks::get_instance();
    /// match instance.get_nbr_idle_tasks() {
    ///     Ok(count) => println!("Idle tasks: {}", count),
    ///     Err(e) => eprintln!("Failed to get idle task count: {}", e),
    /// }
    /// ```
    pub fn get_nbr_idle_tasks(&self) -> Result<u64> {
        let tasks_guard = self.tasks
            .lock()
            .map_err(|_| TaskError::TaskLockError {
                operation: "get_nbr_idle_tasks".to_string(),
            })?;

        let count = tasks_guard
            .iter()
            .filter(|task| task.status == Status::Idle)
            .count() as u64;

        Ok(count)
    }

    /// Gets the number of running tasks
    ///
    /// # Returns
    /// `Ok(u64)` with the count of running tasks, or `Err` if operation fails
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::models::running_tasks::RunningTasks;
    ///
    /// let instance = RunningTasks::get_instance();
    /// match instance.get_nbr_running_tasks() {
    ///     Ok(count) => println!("Running tasks: {}", count),
    ///     Err(e) => eprintln!("Failed to get running task count: {}", e),
    /// }
    /// ```
    pub fn get_nbr_running_tasks(&self) -> Result<u64> {
        let tasks_guard = self.tasks
            .lock()
            .map_err(|_| TaskError::TaskLockError {
                operation: "get_nbr_running_tasks".to_string(),
            })?;

        let count = tasks_guard
            .iter()
            .filter(|task| task.status == Status::Running)
            .count() as u64;

        Ok(count)
    }

    /// Gets the total number of finished tasks (active + archived)
    ///
    /// # Returns
    /// `Ok(u64)` with the total count of finished tasks, or `Err` if operation fails
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::models::running_tasks::RunningTasks;
    ///
    /// let instance = RunningTasks::get_instance();
    /// match instance.get_nbr_finished_tasks() {
    ///     Ok(count) => println!("Finished tasks: {}", count),
    ///     Err(e) => eprintln!("Failed to get finished task count: {}", e),
    /// }
    /// ```
    pub fn get_nbr_finished_tasks(&self) -> Result<u64> {
        let tasks_guard = self.tasks
            .lock()
            .map_err(|_| TaskError::TaskLockError {
                operation: "get_nbr_finished_tasks".to_string(),
            })?;

        let archived_guard = self.nbr_archived_finished_tasks
            .lock()
            .map_err(|_| TaskError::TaskLockError {
                operation: "get_nbr_finished_tasks_archived".to_string(),
            })?;

        let active_count = tasks_guard
            .iter()
            .filter(|task| task.status == Status::Finished)
            .count() as u64;

        Ok(active_count + *archived_guard)
    }

    /// Gets the total number of stopped tasks (active + archived)
    ///
    /// # Returns
    /// `Ok(u64)` with the total count of stopped tasks, or `Err` if operation fails
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::models::running_tasks::RunningTasks;
    ///
    /// let instance = RunningTasks::get_instance();
    /// match instance.get_nbr_stopped_tasks() {
    ///     Ok(count) => println!("Stopped tasks: {}", count),
    ///     Err(e) => eprintln!("Failed to get stopped task count: {}", e),
    /// }
    /// ```
    pub fn get_nbr_stopped_tasks(&self) -> Result<u64> {
        let tasks_guard = self.tasks
            .lock()
            .map_err(|_| TaskError::TaskLockError {
                operation: "get_nbr_stopped_tasks".to_string(),
            })?;

        let archived_guard = self.nbr_archived_stopped_tasks
            .lock()
            .map_err(|_| TaskError::TaskLockError {
                operation: "get_nbr_stopped_tasks_archived".to_string(),
            })?;

        let active_count = tasks_guard
            .iter()
            .filter(|task| task.status == Status::Stopped)
            .count() as u64;

        Ok(active_count + *archived_guard)
    }

    /// Gets the total number of terminated tasks (active + archived)
    ///
    /// # Returns
    /// `Ok(u64)` with the total count of terminated tasks, or `Err` if operation fails
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::models::running_tasks::RunningTasks;
    ///
    /// let instance = RunningTasks::get_instance();
    /// match instance.get_nbr_terminated_tasks() {
    ///     Ok(count) => println!("Terminated tasks: {}", count),
    ///     Err(e) => eprintln!("Failed to get terminated task count: {}", e),
    /// }
    /// ```
    pub fn get_nbr_terminated_tasks(&self) -> Result<u64> {
        let tasks_guard = self.tasks
            .lock()
            .map_err(|_| TaskError::TaskLockError {
                operation: "get_nbr_terminated_tasks".to_string(),
            })?;

        let archived_guard = self.nbr_archived_terminated_tasks
            .lock()
            .map_err(|_| TaskError::TaskLockError {
                operation: "get_nbr_terminated_tasks_archived".to_string(),
            })?;

        let active_count = tasks_guard
            .iter()
            .filter(|task| task.status == Status::Terminated)
            .count() as u64;

        Ok(active_count + *archived_guard)
    }

    /// Gets the total number of error tasks (active + archived)
    ///
    /// # Returns
    /// `Ok(u64)` with the total count of error tasks, or `Err` if operation fails
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::models::running_tasks::RunningTasks;
    ///
    /// let instance = RunningTasks::get_instance();
    /// match instance.get_nbr_error_tasks() {
    ///     Ok(count) => println!("Error tasks: {}", count),
    ///     Err(e) => eprintln!("Failed to get error task count: {}", e),
    /// }
    /// ```
    pub fn get_nbr_error_tasks(&self) -> Result<u64> {
        let tasks_guard = self.tasks
            .lock()
            .map_err(|_| TaskError::TaskLockError {
                operation: "get_nbr_error_tasks".to_string(),
            })?;

        let archived_guard = self.nbr_archived_error_tasks
            .lock()
            .map_err(|_| TaskError::TaskLockError {
                operation: "get_nbr_error_tasks_archived".to_string(),
            })?;

        let active_count = tasks_guard
            .iter()
            .filter(|task| task.status == Status::Error)
            .count() as u64;

        Ok(active_count + *archived_guard)
    }

    /// Gets task statistics grouped by status
    ///
    /// # Returns
    /// `Ok(HashMap<Status, u64>)` with counts for each status, or `Err` if operation fails
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::models::running_tasks::RunningTasks;
    ///
    /// let instance = RunningTasks::get_instance();
    /// match instance.get_task_statistics() {
    ///     Ok(stats) => {
    ///         for (status, count) in stats {
    ///             println!("{:?}: {}", status, count);
    ///         }
    ///     },
    ///     Err(e) => eprintln!("Failed to get statistics: {}", e),
    /// }
    /// ```
    pub fn get_task_statistics(&self) -> Result<HashMap<Status, u64>> {
        let mut stats = HashMap::new();

        stats.insert(Status::Idle, self.get_nbr_idle_tasks()?);
        stats.insert(Status::Running, self.get_nbr_running_tasks()?);
        stats.insert(Status::Finished, self.get_nbr_finished_tasks()?);
        stats.insert(Status::Stopped, self.get_nbr_stopped_tasks()?);
        stats.insert(Status::Terminated, self.get_nbr_terminated_tasks()?);
        stats.insert(Status::Error, self.get_nbr_error_tasks()?);

        Ok(stats)
    }

    /// Clears all tasks and resets counters
    ///
    /// # Returns
    /// `Ok(())` if successful, or `Err` if operation fails
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::models::running_tasks::RunningTasks;
    ///
    /// let instance = RunningTasks::get_instance();
    /// match instance.clear_all() {
    ///     Ok(_) => println!("All tasks cleared"),
    ///     Err(e) => eprintln!("Failed to clear tasks: {}", e),
    /// }
    /// ```
    pub fn clear_all(&self) -> Result<()> {
        let mut tasks_guard = self.tasks
            .lock()
            .map_err(|_| TaskError::TaskLockError {
                operation: "clear_all_tasks".to_string(),
            })?;

        let mut threads_guard = self.running_threads
            .lock()
            .map_err(|_| TaskError::TaskLockError {
                operation: "clear_all_threads".to_string(),
            })?;

        let mut total_guard = self.nbr_total_tasks
            .lock()
            .map_err(|_| TaskError::TaskLockError {
                operation: "clear_all_total".to_string(),
            })?;

        let mut finished_guard = self.nbr_archived_finished_tasks
            .lock()
            .map_err(|_| TaskError::TaskLockError {
                operation: "clear_all_finished".to_string(),
            })?;

        let mut stopped_guard = self.nbr_archived_stopped_tasks
            .lock()
            .map_err(|_| TaskError::TaskLockError {
                operation: "clear_all_stopped".to_string(),
            })?;

        let mut terminated_guard = self.nbr_archived_terminated_tasks
            .lock()
            .map_err(|_| TaskError::TaskLockError {
                operation: "clear_all_terminated".to_string(),
            })?;

        let mut error_guard = self.nbr_archived_error_tasks
            .lock()
            .map_err(|_| TaskError::TaskLockError {
                operation: "clear_all_error".to_string(),
            })?;

        tasks_guard.clear();
        threads_guard.clear();
        *total_guard = 0;
        *finished_guard = 0;
        *stopped_guard = 0;
        *terminated_guard = 0;
        *error_guard = 0;

        Ok(())
    }

    /// Updates a task in the collection
    ///
    /// # Arguments
    /// * `updated_task` - The task with updated information
    ///
    /// # Returns
    /// `Ok(bool)` indicating if the task was found and updated, or `Err` if operation fails
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::models::running_tasks::RunningTasks;
    /// use rezalnyash_core::models::task::Task;
    /// use rezalnyash_core::enums::Status;
    ///
    /// let instance = RunningTasks::get_instance();
    /// let mut task = Task::new("existing-task".to_string());
    /// task.status = Status::Running;
    /// 
    /// match instance.update_task(task) {
    ///     Ok(updated) => println!("Task updated: {}", updated),
    ///     Err(e) => eprintln!("Failed to update task: {}", e),
    /// }
    /// ```
    pub fn update_task(&self, updated_task: Task) -> Result<bool> {
        let mut tasks_guard = self.tasks
            .lock()
            .map_err(|_| TaskError::TaskLockError {
                operation: "update_task".to_string(),
            })?;

        if let Some(existing_task) = tasks_guard
            .iter_mut()
            .find(|task| task.id == updated_task.id)
        {
            *existing_task = updated_task;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Validates the consistency of the RunningTasks state
    ///
    /// # Returns
    /// `Ok(())` if valid, or `Err` with validation errors
    pub fn validate(&self) -> Result<()> {
        let tasks = self.get_tasks()?;
        
        // Validate each task
        for task in &tasks {
            task.validate()?;
        }

        // Check for duplicate task IDs
        let mut seen_ids = std::collections::HashSet::new();
        for task in &tasks {
            if !seen_ids.insert(&task.id) {
                return Err(TaskError::TaskAlreadyExists {
                    task_id: task.id.clone(),
                }.into());
            }
        }

        Ok(())
    }
}

// Thread-safe implementation
unsafe impl Send for RunningTasks {}
unsafe impl Sync for RunningTasks {}

impl std::fmt::Display for RunningTasks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let total_tasks = self.get_nbr_total_tasks().unwrap_or(0);
        let running_tasks = self.get_nbr_running_tasks().unwrap_or(0);
        let finished_tasks = self.get_nbr_finished_tasks().unwrap_or(0);
        
        write!(
            f,
            "RunningTasks {{ total: {}, running: {}, finished: {} }}",
            total_tasks, running_tasks, finished_tasks
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::Status;
    use std::sync::Mutex;

    // Global mutex to ensure tests run sequentially since they share singleton state
    static TEST_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn test_singleton_instance() {
        let _guard = TEST_MUTEX.lock().unwrap();
        let instance1 = RunningTasks::get_instance();
        let instance2 = RunningTasks::get_instance();
        
        // Both should point to the same instance
        assert!(std::ptr::eq(instance1, instance2));
    }

    #[test]
    fn test_add_and_get_task() {
        let _guard = TEST_MUTEX.lock().unwrap();
        let instance = RunningTasks::get_instance();
        let task = Task::new("test-task-1".to_string());
        let task_id = task.id.clone();
        
        // Clear any existing tasks first
        let _ = instance.clear_all();
        
        // Add task
        assert!(instance.add_task(task).unwrap());
        
        // Get task
        let retrieved_task = instance.get_task(&task_id).unwrap();
        assert!(retrieved_task.is_some());
        assert_eq!(retrieved_task.unwrap().id, task_id);
    }

    #[test]
    fn test_task_counters() {
        let _guard = TEST_MUTEX.lock().unwrap();
        let instance = RunningTasks::get_instance();
        let _ = instance.clear_all();
        
        // Add tasks with different statuses
        let mut task1 = Task::new("task-1".to_string());
        task1.status = Status::Running;
        
        let mut task2 = Task::new("task-2".to_string());
        task2.status = Status::Finished;
        
        let mut task3 = Task::new("task-3".to_string());
        task3.status = Status::Error;
        
        assert!(instance.add_task(task1).unwrap());
        assert!(instance.add_task(task2).unwrap());
        assert!(instance.add_task(task3).unwrap());
        
        // Check counters
        assert_eq!(instance.get_nbr_total_tasks().unwrap(), 3);
        assert_eq!(instance.get_nbr_running_tasks().unwrap(), 1);
        assert_eq!(instance.get_nbr_finished_tasks().unwrap(), 1);
        assert_eq!(instance.get_nbr_error_tasks().unwrap(), 1);
    }

    #[test]
    fn test_remove_tasks() {
        let _guard = TEST_MUTEX.lock().unwrap();
        let instance = RunningTasks::get_instance();
        let _ = instance.clear_all();
        
        // Add tasks
        let mut task1 = Task::new("remove-task-1".to_string());
        task1.status = Status::Finished;
        
        let mut task2 = Task::new("remove-task-2".to_string());
        task2.status = Status::Stopped;
        
        assert!(instance.add_task(task1.clone()).unwrap());
        assert!(instance.add_task(task2.clone()).unwrap());
        
        // Remove tasks
        let tasks_to_remove = vec![task1, task2];
        assert!(instance.remove_all_tasks(tasks_to_remove).unwrap());
        
        // Check that tasks were removed and archived counters updated
        assert_eq!(instance.get_tasks().unwrap().len(), 0);
        assert_eq!(instance.get_nbr_finished_tasks().unwrap(), 1); // 1 archived
        assert_eq!(instance.get_nbr_stopped_tasks().unwrap(), 1); // 1 archived
    }

    #[test]
    fn test_task_statistics() {
        let _guard = TEST_MUTEX.lock().unwrap();
        let instance = RunningTasks::get_instance();
        let _ = instance.clear_all();
        
        // Add various tasks
        let mut task1 = Task::new("stats-task-1".to_string());
        task1.status = Status::Running;
        
        let mut task2 = Task::new("stats-task-2".to_string());
        task2.status = Status::Idle;
        
        assert!(instance.add_task(task1).unwrap());
        assert!(instance.add_task(task2).unwrap());
        
        let stats = instance.get_task_statistics().unwrap();
        assert_eq!(stats.get(&Status::Running), Some(&1));
        assert_eq!(stats.get(&Status::Idle), Some(&1));
        assert_eq!(stats.get(&Status::Finished), Some(&0));
    }
}
