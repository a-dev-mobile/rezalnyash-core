//! Watch Dog Model
//!
//! This module provides the WatchDog struct which monitors and manages
//! running tasks, cleaning up finished tasks and enforcing timeouts.
//! It's a Rust conversion of the Java WatchDog class with improved
//! error handling and thread safety.

use crate::enums::Status;
use crate::errors::{Result, TaskError};
use crate::models::{
    running_tasks::running_tasks::RunningTasks,
    task::{Task, TaskReport},
};
use crate::{log_debug, log_error, log_info, log_warn, log_trace};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::sync::mpsc::{self, Receiver, Sender};

/// Time constants in milliseconds
const FINISHED_TASK_TTL: u64 = 60_000;           // 1 minute
pub const LONG_RUNNING_TASK_TTL: u64 = 600_000;  // 10 minutes
pub const LONG_RUNNING_TASK_WITH_SOLUTION_TTL: u64 = 60_000; // 1 minute
const ORPHAN_TASK_TTL: u64 = 60_000;             // 1 minute
const RUNNING_INTERVAL: u64 = 5_000;             // 5 seconds
const TASK_ERROR_THREAD_THRESHOLD: i32 = 100;

/// Trait for cut list logger functionality
pub trait CutListLogger: Send + Sync + std::fmt::Debug {
    /// Log an error message
    fn error(&self, message: &str);
    
    /// Log an error message with exception details
    fn error_with_exception(&self, message: &str, error: &dyn std::error::Error);
    
    /// Log a warning message with client and task context
    fn warn(&self, client_id: &str, task_id: &str, message: &str);
    
    /// Log an error message with client and task context
    fn error_with_context(&self, client_id: &str, task_id: &str, message: &str);
    
    /// Log task execution details
    fn log_execution(&self, task: &Task);
}

/// Default implementation of CutListLogger
#[derive(Debug, Clone)]
pub struct DefaultCutListLogger;

impl CutListLogger for DefaultCutListLogger {
    fn error(&self, message: &str) {
        log_error!("{}", message);
    }
    
    fn error_with_exception(&self, message: &str, error: &dyn std::error::Error) {
        log_error!("{}: {}", message, error);
    }
    
    fn warn(&self, client_id: &str, task_id: &str, message: &str) {
        log_warn!("Client[{}] Task[{}]: {}", client_id, task_id, message);
    }
    
    fn error_with_context(&self, client_id: &str, task_id: &str, message: &str) {
        log_error!("Client[{}] Task[{}]: {}", client_id, task_id, message);
    }
    
    fn log_execution(&self, task: &Task) {
        log_debug!("Logging execution for task: {}", task.id);
    }
}

/// Trait for cut list optimizer service functionality
pub trait CutListOptimizerService: Send + Sync + std::fmt::Debug {
    /// Terminate a task by ID
    /// 
    /// # Arguments
    /// * `task_id` - The ID of the task to terminate
    /// 
    /// # Returns
    /// 0 on success, non-zero error code on failure
    fn terminate_task(&self, task_id: &str) -> i32;
}

/// Default implementation of CutListOptimizerService
#[derive(Debug, Clone)]
pub struct DefaultCutListOptimizerService;

impl CutListOptimizerService for DefaultCutListOptimizerService {
    fn terminate_task(&self, task_id: &str) -> i32 {
        log_info!("Terminating task: {}", task_id);
        // In a real implementation, this would interact with the task execution system
        0 // Success
    }
}

/// Thread pool executor trait for managing task execution
pub trait ThreadPoolExecutor: Send + Sync + std::fmt::Debug {
    /// Get the number of active threads
    fn get_active_count(&self) -> i32;
    
    /// Get the total pool size
    fn get_pool_size(&self) -> i32;
    
    /// Get the number of queued tasks
    fn get_queue_size(&self) -> i32;
    
    /// Get the number of completed tasks
    fn get_completed_task_count(&self) -> u64;
}

/// Default implementation of ThreadPoolExecutor
#[derive(Debug)]
pub struct DefaultThreadPoolExecutor {
    active_count: Arc<Mutex<i32>>,
    pool_size: i32,
    queue_size: Arc<Mutex<i32>>,
    completed_count: Arc<Mutex<u64>>,
}

impl DefaultThreadPoolExecutor {
    pub fn new(pool_size: i32) -> Self {
        Self {
            active_count: Arc::new(Mutex::new(0)),
            pool_size,
            queue_size: Arc::new(Mutex::new(0)),
            completed_count: Arc::new(Mutex::new(0)),
        }
    }
}

impl ThreadPoolExecutor for DefaultThreadPoolExecutor {
    fn get_active_count(&self) -> i32 {
        *self.active_count.lock().unwrap_or_else(|_| {
            log_error!("Failed to acquire active_count lock");
            std::process::exit(1);
        })
    }
    
    fn get_pool_size(&self) -> i32 {
        self.pool_size
    }
    
    fn get_queue_size(&self) -> i32 {
        *self.queue_size.lock().unwrap_or_else(|_| {
            log_error!("Failed to acquire queue_size lock");
            std::process::exit(1);
        })
    }
    
    fn get_completed_task_count(&self) -> u64 {
        *self.completed_count.lock().unwrap_or_else(|_| {
            log_error!("Failed to acquire completed_count lock");
            std::process::exit(1);
        })
    }
}

/// Control messages for the WatchDog
#[derive(Debug, Clone)]
pub enum WatchDogControl {
    /// Stop the watch dog
    Stop,
    /// Pause monitoring
    Pause,
    /// Resume monitoring
    Resume,
}

/// Watch Dog for monitoring and managing running tasks
///
/// The WatchDog monitors task execution, enforces timeouts, cleans up
/// finished tasks, and provides comprehensive task lifecycle management.
/// This is a thread-safe Rust conversion of the Java WatchDog class.
#[derive(Debug)]
pub struct WatchDog {
    /// Reference to the running tasks singleton
    running_tasks: &'static RunningTasks,
    
    /// Thread pool executor for task management
    task_executor: Option<Arc<dyn ThreadPoolExecutor>>,
    
    /// Cut list optimizer service
    cut_list_optimizer_service: Option<Arc<dyn CutListOptimizerService>>,
    
    /// Cut list logger
    cut_list_logger: Arc<dyn CutListLogger>,
    
    /// Task reports from the last monitoring cycle
    task_reports: Arc<Mutex<Vec<TaskReport>>>,
    
    /// Control channel sender
    control_sender: Option<Sender<WatchDogControl>>,
    
    /// Whether the watch dog is currently running
    is_running: Arc<Mutex<bool>>,
    
    /// Whether the watch dog is paused
    is_paused: Arc<Mutex<bool>>,
}

impl WatchDog {
    /// Creates a new WatchDog instance
    ///
    /// # Returns
    /// A new WatchDog with default dependencies
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::models::watch_dog::WatchDog;
    ///
    /// let watch_dog = WatchDog::new();
    /// ```
    pub fn new() -> Self {
        Self {
            running_tasks: RunningTasks::get_instance(),
            task_executor: None,
            cut_list_optimizer_service: None,
            cut_list_logger: Arc::new(DefaultCutListLogger),
            task_reports: Arc::new(Mutex::new(Vec::new())),
            control_sender: None,
            is_running: Arc::new(Mutex::new(false)),
            is_paused: Arc::new(Mutex::new(false)),
        }
    }

    /// Creates a new WatchDog with custom dependencies
    ///
    /// # Arguments
    /// * `task_executor` - Thread pool executor for task management
    /// * `cut_list_optimizer_service` - Service for task optimization
    /// * `cut_list_logger` - Logger for cut list operations
    ///
    /// # Returns
    /// A new WatchDog with the specified dependencies
    pub fn with_dependencies(
        task_executor: Arc<dyn ThreadPoolExecutor>,
        cut_list_optimizer_service: Arc<dyn CutListOptimizerService>,
        cut_list_logger: Arc<dyn CutListLogger>,
    ) -> Self {
        Self {
            running_tasks: RunningTasks::get_instance(),
            task_executor: Some(task_executor),
            cut_list_optimizer_service: Some(cut_list_optimizer_service),
            cut_list_logger,
            task_reports: Arc::new(Mutex::new(Vec::new())),
            control_sender: None,
            is_running: Arc::new(Mutex::new(false)),
            is_paused: Arc::new(Mutex::new(false)),
        }
    }

    // Getters and setters
    pub fn get_running_tasks(&self) -> &'static RunningTasks {
        self.running_tasks
    }

    pub fn get_task_executor(&self) -> Option<Arc<dyn ThreadPoolExecutor>> {
        self.task_executor.clone()
    }

    pub fn set_task_executor(&mut self, task_executor: Arc<dyn ThreadPoolExecutor>) {
        self.task_executor = Some(task_executor);
    }

    pub fn get_cut_list_optimizer_service(&self) -> Option<Arc<dyn CutListOptimizerService>> {
        self.cut_list_optimizer_service.clone()
    }

    pub fn set_cut_list_optimizer_service(&mut self, service: Arc<dyn CutListOptimizerService>) {
        self.cut_list_optimizer_service = Some(service);
    }

    pub fn get_cut_list_logger(&self) -> Arc<dyn CutListLogger> {
        self.cut_list_logger.clone()
    }

    pub fn set_cut_list_logger(&mut self, logger: Arc<dyn CutListLogger>) {
        self.cut_list_logger = logger;
    }

    pub fn get_task_reports(&self) -> Result<Vec<TaskReport>> {
        self.task_reports
            .lock()
            .map(|guard| guard.clone())
            .map_err(|_| TaskError::TaskLockError {
                operation: "get_task_reports".to_string(),
            }.into())
    }

    /// Checks if the watch dog is currently running
    pub fn is_running(&self) -> bool {
        *self.is_running.lock().unwrap_or_else(|_| {
            log_error!("Failed to acquire is_running lock");
            std::process::exit(1);
        })
    }

    /// Checks if the watch dog is currently paused
    pub fn is_paused(&self) -> bool {
        *self.is_paused.lock().unwrap_or_else(|_| {
            log_error!("Failed to acquire is_paused lock");
            std::process::exit(1);
        })
    }

    /// Starts the watch dog in a separate thread
    ///
    /// # Returns
    /// `Ok(Sender<WatchDogControl>)` for sending control messages, or `Err` if already running
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::models::watch_dog::{WatchDog, WatchDogControl};
    ///
    /// let mut watch_dog = WatchDog::new();
    /// match watch_dog.start() {
    ///     Ok(control_sender) => {
    ///         // Watch dog started successfully
    ///         // Use control_sender to send control messages
    ///     },
    ///     Err(e) => eprintln!("Failed to start watch dog: {}", e),
    /// }
    /// ```
    pub fn start(&mut self) -> Result<Sender<WatchDogControl>> {
        if self.is_running() {
            return Err(TaskError::TaskInvalidState {
                current_state: "WatchDog is already running".to_string(),
            }.into());
        }

        let (control_sender, control_receiver) = mpsc::channel();
        self.control_sender = Some(control_sender.clone());

        // Clone necessary data for the thread
        let running_tasks = self.running_tasks;
        let task_executor = self.task_executor.clone();
        let cut_list_optimizer_service = self.cut_list_optimizer_service.clone();
        let cut_list_logger = self.cut_list_logger.clone();
        let task_reports = self.task_reports.clone();
        let is_running = self.is_running.clone();
        let is_paused = self.is_paused.clone();

        // Set running state
        *is_running.lock().unwrap() = true;

        // Spawn the monitoring thread
        thread::spawn(move || {
            Self::run_monitoring_loop(
                running_tasks,
                task_executor,
                cut_list_optimizer_service,
                cut_list_logger,
                task_reports,
                is_running,
                is_paused,
                control_receiver,
            );
        });

        log_info!("WatchDog started successfully");
        Ok(control_sender)
    }

    /// Stops the watch dog
    ///
    /// # Returns
    /// `Ok(())` if stopped successfully, `Err` if not running or failed to stop
    pub fn stop(&mut self) -> Result<()> {
        if !self.is_running() {
            return Err(TaskError::TaskInvalidState {
                current_state: "WatchDog is not running".to_string(),
            }.into());
        }

        if let Some(sender) = &self.control_sender {
            sender.send(WatchDogControl::Stop)
                .map_err(|_| TaskError::TaskThreadError {
                    details: "Failed to send stop signal".to_string(),
                })?;
        }

        // Wait a bit for the thread to stop
        thread::sleep(Duration::from_millis(100));

        log_info!("WatchDog stopped successfully");
        Ok(())
    }

    /// Pauses the watch dog monitoring
    pub fn pause(&self) -> Result<()> {
        if let Some(sender) = &self.control_sender {
            sender.send(WatchDogControl::Pause)
                .map_err(|_| TaskError::TaskThreadError {
                    details: "Failed to send pause signal".to_string(),
                })?;
        }
        Ok(())
    }

    /// Resumes the watch dog monitoring
    pub fn resume(&self) -> Result<()> {
        if let Some(sender) = &self.control_sender {
            sender.send(WatchDogControl::Resume)
                .map_err(|_| TaskError::TaskThreadError {
                    details: "Failed to send resume signal".to_string(),
                })?;
        }
        Ok(())
    }

    /// Main monitoring loop (runs in a separate thread)
    fn run_monitoring_loop(
        running_tasks: &'static RunningTasks,
        task_executor: Option<Arc<dyn ThreadPoolExecutor>>,
        cut_list_optimizer_service: Option<Arc<dyn CutListOptimizerService>>,
        cut_list_logger: Arc<dyn CutListLogger>,
        task_reports: Arc<Mutex<Vec<TaskReport>>>,
        is_running: Arc<Mutex<bool>>,
        is_paused: Arc<Mutex<bool>>,
        control_receiver: Receiver<WatchDogControl>,
    ) {
        log_info!("WatchDog monitoring loop started");

        loop {
            // Check for control messages (non-blocking)
            if let Ok(control_msg) = control_receiver.try_recv() {
                match control_msg {
                    WatchDogControl::Stop => {
                        log_info!("WatchDog received stop signal");
                        *is_running.lock().unwrap() = false;
                        break;
                    }
                    WatchDogControl::Pause => {
                        log_info!("WatchDog paused");
                        *is_paused.lock().unwrap() = true;
                    }
                    WatchDogControl::Resume => {
                        log_info!("WatchDog resumed");
                        *is_paused.lock().unwrap() = false;
                    }
                }
            }

            // Skip monitoring if paused
            if *is_paused.lock().unwrap() {
                thread::sleep(Duration::from_millis(RUNNING_INTERVAL));
                continue;
            }

            // Perform monitoring cycle
            if let Err(e) = Self::monitoring_cycle(
                running_tasks,
                &task_executor,
                &cut_list_optimizer_service,
                &cut_list_logger,
                &task_reports,
            ) {
                cut_list_logger.error_with_exception("Error during monitoring cycle", &e);
            }

            // Clean up finished tasks
            Self::clean_finished_tasks(
                running_tasks,
                &cut_list_optimizer_service,
                &cut_list_logger,
            );

            // Sleep before next cycle
            thread::sleep(Duration::from_millis(RUNNING_INTERVAL));
        }

        log_info!("WatchDog monitoring loop ended");
    }

    /// Performs one monitoring cycle
    fn monitoring_cycle(
        running_tasks: &RunningTasks,
        task_executor: &Option<Arc<dyn ThreadPoolExecutor>>,
        cut_list_optimizer_service: &Option<Arc<dyn CutListOptimizerService>>,
        cut_list_logger: &Arc<dyn CutListLogger>,
        task_reports: &Arc<Mutex<Vec<TaskReport>>>,
    ) -> Result<()> {
        // Log thread pool status
        if let Some(executor) = task_executor {
            let tasks = running_tasks.get_tasks()?;
            let total_tasks = running_tasks.get_nbr_total_tasks()?;
            
            log_debug!(
                "Tasks: Active[{}] Total[{}] - Threads: Active[{}/{}] Queued[{}] Completed[{}]",
                tasks.len(),
                total_tasks,
                executor.get_active_count(),
                executor.get_pool_size(),
                executor.get_queue_size(),
                executor.get_completed_task_count()
            );
        }

        // Process each task
        let tasks = running_tasks.get_tasks()?;
        let mut reports = Vec::new();

        for task in &tasks {
            match Self::process_task_monitoring(task, cut_list_optimizer_service, cut_list_logger) {
                Ok(report) => {
                    if let Some(report) = report {
                        reports.push(report);
                    }
                }
                Err(e) => {
                    cut_list_logger.error_with_exception("Error while monitoring task", &e);
                }
            }
        }

        // Update task reports
        if let Ok(mut task_reports_guard) = task_reports.lock() {
            task_reports_guard.clear();
            task_reports_guard.extend(reports);
        }

        Ok(())
    }

    /// Processes monitoring for a single task
    fn process_task_monitoring(
        task: &Task,
        cut_list_optimizer_service: &Option<Arc<dyn CutListOptimizerService>>,
        cut_list_logger: &Arc<dyn CutListLogger>,
    ) -> Result<Option<TaskReport>> {
        log_debug!("Watching task {}", task.id);

        // Create task report using from_task method
        let task_report = TaskReport::from_task(task);

        // Log task status
        log_info!("{}", task_report);

        // Check for error threshold
        if task.is_running() 
            && task.get_nbr_error_threads() > TASK_ERROR_THREAD_THRESHOLD 
            && task.get_nbr_error_threads() == task.get_nbr_total_threads() {
            
            cut_list_logger.error("Error thread threshold reached");
            // Note: In the original Java, task.terminateError() was called here
            // In Rust, we would need to modify the task through the RunningTasks singleton
            // For now, we'll just log the condition
        }

        // Log task execution
        cut_list_logger.log_execution(task);

        // Check for long-running tasks with solutions
        if task.status == Status::Running 
            && task.get_elapsed_time() > LONG_RUNNING_TASK_WITH_SOLUTION_TTL 
            && task.has_solution_all_fit() {
            
            if let Some(client_info) = &task.client_info {
                let client_id = client_info.id.as_deref().unwrap_or("unknown");
                cut_list_logger.warn(
                    client_id,
                    &task.id,
                    "Task with solution has been running for more than 1m and will be terminated"
                );
            }

            if let Some(service) = cut_list_optimizer_service {
                if service.terminate_task(&task.id) != 0 {
                    if let Some(client_info) = &task.client_info {
                        let client_id = client_info.id.as_deref().unwrap_or("unknown");
                        cut_list_logger.error_with_context(
                            client_id,
                            &task.id,
                            "Unable to terminate task"
                        );
                    }
                }
            }
        }

        // Check for long-running tasks (10+ minutes)
        if task.status == Status::Running && task.get_elapsed_time() > LONG_RUNNING_TASK_TTL {
            if let Some(client_info) = &task.client_info {
                let client_id = client_info.id.as_deref().unwrap_or("unknown");
                cut_list_logger.warn(
                    client_id,
                    &task.id,
                    "Task has been running for more than 10m and will be terminated"
                );
            }

            if let Some(service) = cut_list_optimizer_service {
                if service.terminate_task(&task.id) != 0 {
                    if let Some(client_info) = &task.client_info {
                        let client_id = client_info.id.as_deref().unwrap_or("unknown");
                        cut_list_logger.error_with_context(
                            client_id,
                            &task.id,
                            "Unable to terminate task"
                        );
                    }
                }
            }
        }

        // Check for orphaned tasks (not queried for 1+ minute)
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
            
        if task.status == Status::Running && now.saturating_sub(task.last_queried) > ORPHAN_TASK_TTL {
            if let Some(client_info) = &task.client_info {
                let client_id = client_info.id.as_deref().unwrap_or("unknown");
                cut_list_logger.warn(
                    client_id,
                    &task.id,
                    "Task status was not queried for more than 1m and will be terminated"
                );
            }

            if let Some(service) = cut_list_optimizer_service {
                if service.terminate_task(&task.id) != 0 {
                    if let Some(client_info) = &task.client_info {
                        let client_id = client_info.id.as_deref().unwrap_or("unknown");
                        cut_list_logger.error_with_context(
                            client_id,
                            &task.id,
                            "Unable to terminate task"
                        );
                    }
                }
            }
        }

        Ok(Some(task_report))
    }

    /// Cleans up finished tasks and orphaned threads
    fn clean_finished_tasks(
        running_tasks: &RunningTasks,
        cut_list_optimizer_service: &Option<Arc<dyn CutListOptimizerService>>,
        cut_list_logger: &Arc<dyn CutListLogger>,
    ) {
        log_debug!("Cleaning finished tasks");

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        // Get tasks to remove
        let tasks_to_remove = match running_tasks.get_tasks() {
            Ok(tasks) => {
                tasks.into_iter()
                    .filter(|task| {
                        let is_finished_status = matches!(
                            task.status,
                            Status::Finished | Status::Stopped | Status::Terminated | Status::Error
                        );
                        
                        is_finished_status && 
                        task.end_time > 0 && 
                        now.saturating_sub(task.end_time) > FINISHED_TASK_TTL
                    })
                    .collect::<Vec<_>>()
            }
            Err(e) => {
                cut_list_logger.error_with_exception("Failed to get tasks for cleanup", &e);
                return;
            }
        };

        // Remove finished tasks
        if !tasks_to_remove.is_empty() {
            match running_tasks.remove_all_tasks(tasks_to_remove.clone()) {
                Ok(_) => {
                    log_trace!("Cleared {} tasks", tasks_to_remove.len());
                }
                Err(e) => {
                    cut_list_logger.error_with_exception("Failed to remove finished tasks", &e);
                }
            }
        }

        // Clean up orphaned threads
        // Note: This would require access to running threads, which isn't directly available
        // In the original Java implementation. For now, we'll skip this part.
        // In a real implementation, you would need to track and clean up orphaned threads.
    }

    /// Formats elapsed time in milliseconds to a human-readable string
    pub fn format_elapsed_time(elapsed_ms: u64) -> String {
        if elapsed_ms < 1000 {
            format!("{}ms", elapsed_ms)
        } else if elapsed_ms < 60_000 {
            format!("{:.1}s", elapsed_ms as f64 / 1000.0)
        } else if elapsed_ms < 3_600_000 {
            let minutes = elapsed_ms / 60_000;
            let seconds = (elapsed_ms % 60_000) / 1000;
            format!("{}m {}s", minutes, seconds)
        } else {
            let hours = elapsed_ms / 3_600_000;
            let minutes = (elapsed_ms % 3_600_000) / 60_000;
            format!("{}h {}m", hours, minutes)
        }
    }

    /// Gets the current monitoring statistics
    ///
    /// # Returns
    /// A tuple containing (active_tasks, total_tasks, task_reports)
    pub fn get_monitoring_stats(&self) -> Result<(usize, u64, Vec<TaskReport>)> {
        let tasks = self.running_tasks.get_tasks()?;
        let total_tasks = self.running_tasks.get_nbr_total_tasks()?;
        let task_reports = self.get_task_reports()?;
        
        Ok((tasks.len(), total_tasks, task_reports))
    }

    /// Validates the WatchDog configuration
    ///
    /// # Returns
    /// `Ok(())` if valid, `Err` with validation errors
    pub fn validate(&self) -> Result<()> {
        if self.task_executor.is_none() {
            log_warn!("WatchDog has no task executor configured");
        }

        if self.cut_list_optimizer_service.is_none() {
            log_warn!("WatchDog has no cut list optimizer service configured");
        }

        // Validate running tasks
        self.running_tasks.validate()?;

        Ok(())
    }
}

impl Default for WatchDog {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for WatchDog {
    fn drop(&mut self) {
        if self.is_running() {
            let _ = self.stop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::task::Task;
    use crate::enums::Status;
    use std::sync::Mutex;

    // Global mutex to ensure tests run sequentially since they share singleton state
    static TEST_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn test_new_watch_dog() {
        let _guard = TEST_MUTEX.lock().unwrap();
        let watch_dog = WatchDog::new();
        
        assert!(!watch_dog.is_running());
        assert!(!watch_dog.is_paused());
        assert!(watch_dog.task_executor.is_none());
        assert!(watch_dog.cut_list_optimizer_service.is_none());
    }

    #[test]
    fn test_with_dependencies() {
        let _guard = TEST_MUTEX.lock().unwrap();
        let task_executor = Arc::new(DefaultThreadPoolExecutor::new(4));
        let optimizer_service = Arc::new(DefaultCutListOptimizerService);
        let logger = Arc::new(DefaultCutListLogger);
        
        let watch_dog = WatchDog::with_dependencies(
            task_executor.clone(),
            optimizer_service.clone(),
            logger.clone(),
        );
        
        assert!(watch_dog.task_executor.is_some());
        assert!(watch_dog.cut_list_optimizer_service.is_some());
    }

    #[test]
    fn test_format_elapsed_time() {
        assert_eq!(WatchDog::format_elapsed_time(500), "500ms");
        assert_eq!(WatchDog::format_elapsed_time(1500), "1.5s");
        assert_eq!(WatchDog::format_elapsed_time(65000), "1m 5s");
        assert_eq!(WatchDog::format_elapsed_time(3665000), "1h 1m");
    }

    #[test]
    fn test_task_reports() {
        let _guard = TEST_MUTEX.lock().unwrap();
        let watch_dog = WatchDog::new();
        
        let reports = watch_dog.get_task_reports().unwrap();
        assert!(reports.is_empty());
    }
}
