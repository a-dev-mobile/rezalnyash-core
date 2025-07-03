//! Cut List Optimizer Service Trait
//!
//! This module provides the main service interface for the cut list optimizer.
//! It's a Rust port of the Java CutListOptimizerService interface with all
//! methods and functionality preserved.

use crate::enums::{Status, StatusCode};
use crate::errors::Result;
use crate::models::stats::stats::Stats;
use crate::models::task_status_response::task_status_response::TaskStatusResponse;
use crate::models::{
    calculation_request::CalculationRequest,
    calculation_submission_result::CalculationSubmissionResult,

};
use crate::models::watch_dog::CutListLogger;
use std::sync::Arc;

/// Main service interface for cut list optimization
///
/// This trait defines all the operations available for managing and executing
/// cut list optimization tasks. It provides methods for task submission,
/// monitoring, control, and statistics gathering.
pub trait CutListOptimizerService: Send + Sync + std::fmt::Debug {
    /// Get current service statistics
    ///
    /// Returns comprehensive statistics about the service including:
    /// - Number of tasks in different states (idle, running, finished, etc.)
    /// - Thread pool statistics (active, queued, completed threads)
    /// - Task reports with detailed information
    ///
    /// # Returns
    /// `Stats` object containing current service statistics
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::services::CutListOptimizerService;
    ///
    /// fn print_stats(service: &dyn CutListOptimizerService) {
    ///     let stats = service.get_stats();
    ///     println!("Running tasks: {}", stats.nbr_running_tasks);
    ///     println!("Finished tasks: {}", stats.nbr_finished_tasks);
    /// }
    /// ```
    fn get_stats(&self) -> Stats;

    /// Get status of a specific task
    ///
    /// Retrieves detailed status information for a task including:
    /// - Current status (Running, Finished, Error, etc.)
    /// - Progress percentage
    /// - Current best solution (if any)
    /// - Initialization percentage
    ///
    /// # Arguments
    /// * `task_id` - The unique identifier of the task
    ///
    /// # Returns
    /// `Some(TaskStatusResponse)` if task exists, `None` if task not found
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::services::CutListOptimizerService;
    ///
    /// fn check_task_status(service: &dyn CutListOptimizerService, task_id: &str) {
    ///     if let Some(status) = service.get_task_status(task_id) {
    ///         println!("Task {} is {}", task_id, status.status);
    ///         println!("Progress: {}%", status.percentage_done);
    ///     } else {
    ///         println!("Task {} not found", task_id);
    ///     }
    /// }
    /// ```
    fn get_task_status(&self, task_id: &str) -> Option<TaskStatusResponse>;

    /// Get list of task IDs for a client with specific status
    ///
    /// Retrieves all task IDs belonging to a specific client that have
    /// the specified status. Useful for monitoring client's tasks.
    ///
    /// # Arguments
    /// * `client_id` - The client identifier
    /// * `status` - The task status to filter by
    ///
    /// # Returns
    /// Vector of task IDs matching the criteria
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::services::CutListOptimizerService;
    /// use rezalnyash_core::enums::Status;
    ///
    /// fn get_running_tasks(service: &dyn CutListOptimizerService, client_id: &str) -> Vec<String> {
    ///     service.get_tasks(client_id, Status::Running)
    /// }
    /// ```
    fn get_tasks(&self, client_id: &str, status: Status) -> Vec<String>;

    /// Initialize the service with specified thread pool size
    ///
    /// Sets up the service with the given number of worker threads.
    /// Must be called before submitting any tasks.
    ///
    /// # Arguments
    /// * `thread_pool_size` - Number of threads in the thread pool
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::services::CutListOptimizerService;
    ///
    /// fn setup_service(service: &mut dyn CutListOptimizerService) {
    ///     service.init(8); // Initialize with 8 worker threads
    /// }
    /// ```
    fn init(&mut self, thread_pool_size: i32);

    /// Set whether multiple tasks per client are allowed
    ///
    /// Controls whether a single client can have multiple concurrent tasks.
    /// When disabled, new task submissions will be rejected if the client
    /// already has running tasks.
    ///
    /// # Arguments
    /// * `allow` - `true` to allow multiple tasks per client, `false` to restrict
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::services::CutListOptimizerService;
    ///
    /// fn configure_service(service: &mut dyn CutListOptimizerService) {
    ///     service.set_allow_multiple_tasks_per_client(false); // One task per client
    /// }
    /// ```
    fn set_allow_multiple_tasks_per_client(&mut self, allow: bool);

    /// Set the cut list logger
    ///
    /// Configures the logger used for cut list operations and debugging.
    /// The logger will receive all optimization-related log messages.
    ///
    /// # Arguments
    /// * `logger` - The logger implementation to use
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::services::CutListOptimizerService;
    /// use rezalnyash_core::models::watch_dog::{CutListLogger, DefaultCutListLogger};
    /// use std::sync::Arc;
    ///
    /// fn setup_logging(service: &mut dyn CutListOptimizerService) {
    ///     let logger: Arc<dyn CutListLogger> = Arc::new(DefaultCutListLogger);
    ///     service.set_cut_list_logger(logger);
    /// }
    /// ```
    fn set_cut_list_logger(&mut self, logger: Arc<dyn CutListLogger>);

    /// Stop a running task
    ///
    /// Gracefully stops a running task, allowing it to finish its current
    /// operation and save any partial results. The task status will change
    /// to Stopped.
    ///
    /// # Arguments
    /// * `task_id` - The unique identifier of the task to stop
    ///
    /// # Returns
    /// `Some(TaskStatusResponse)` with final status if task exists, `None` if task not found
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::services::CutListOptimizerService;
    ///
    /// fn stop_task_if_running(service: &dyn CutListOptimizerService, task_id: &str) {
    ///     if let Some(status) = service.stop_task(task_id) {
    ///         println!("Task {} stopped, final status: {}", task_id, status.status);
    ///     } else {
    ///         println!("Task {} not found", task_id);
    ///     }
    /// }
    /// ```
    fn stop_task(&self, task_id: &str) -> Option<TaskStatusResponse>;

    /// Submit a new optimization task
    ///
    /// Submits a new cut list optimization task for processing. The task
    /// will be validated and queued for execution if valid.
    ///
    /// # Arguments
    /// * `request` - The calculation request containing panels, stock, and configuration
    ///
    /// # Returns
    /// `CalculationSubmissionResult` indicating success/failure and task ID if successful
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::services::CutListOptimizerService;
    /// use rezalnyash_core::models::calculation_request::CalculationRequest;
    /// use rezalnyash_core::enums::StatusCode;
    ///
    /// fn submit_optimization(service: &dyn CutListOptimizerService, request: CalculationRequest) {
    ///     let result = service.submit_task(request);
    ///     if result.status_code == StatusCode::Ok.get_string_value() {
    ///         if let Some(task_id) = result.task_id {
    ///             println!("Task submitted successfully: {}", task_id);
    ///         }
    ///     } else {
    ///         println!("Task submission failed: {}", result.status_code);
    ///     }
    /// }
    /// ```
    fn submit_task(&self, request: CalculationRequest) -> CalculationSubmissionResult;

    /// Terminate a task immediately
    ///
    /// Forcefully terminates a task without waiting for graceful shutdown.
    /// This should be used as a last resort when stop_task doesn't work.
    ///
    /// # Arguments
    /// * `task_id` - The unique identifier of the task to terminate
    ///
    /// # Returns
    /// 0 on success, non-zero error code on failure
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::services::CutListOptimizerService;
    ///
    /// fn force_terminate_task(service: &dyn CutListOptimizerService, task_id: &str) {
    ///     let result = service.terminate_task(task_id);
    ///     if result == 0 {
    ///         println!("Task {} terminated successfully", task_id);
    ///     } else {
    ///         println!("Failed to terminate task {}: error code {}", task_id, result);
    ///     }
    /// }
    /// ```
    fn terminate_task(&self, task_id: &str) -> i32;
}

/// Status codes for operation results
///
/// These codes indicate the result of various service operations,
/// particularly task submission results.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceStatusCode {
    /// Operation completed successfully
    Ok = 0,
    /// Invalid tiles in the request
    InvalidTiles = 1,
    /// Invalid stock tiles in the request
    InvalidStockTiles = 2,
    /// Task already running for this client
    TaskAlreadyRunning = 3,
    /// Server is unavailable
    ServerUnavailable = 4,
    /// Too many panels in the request
    TooManyPanels = 5,
    /// Too many stock panels in the request
    TooManyStockPanels = 6,
}

impl ServiceStatusCode {
    /// Get the numeric value of the status code
    pub fn get_value(self) -> i32 {
        self as i32
    }

    /// Get the string representation of the status code
    pub fn get_string_value(self) -> String {
        (self as i32).to_string()
    }
}

impl From<ServiceStatusCode> for StatusCode {
    fn from(service_code: ServiceStatusCode) -> Self {
        match service_code {
            ServiceStatusCode::Ok => StatusCode::Ok,
            ServiceStatusCode::InvalidTiles => StatusCode::InvalidTiles,
            ServiceStatusCode::InvalidStockTiles => StatusCode::InvalidStockTiles,
            ServiceStatusCode::TaskAlreadyRunning => StatusCode::TaskAlreadyRunning,
            ServiceStatusCode::ServerUnavailable => StatusCode::ServerUnavailable,
            ServiceStatusCode::TooManyPanels => StatusCode::TooManyPanels,
            ServiceStatusCode::TooManyStockPanels => StatusCode::TooManyStockPanels,
        }
    }
}

impl From<StatusCode> for ServiceStatusCode {
    fn from(status_code: StatusCode) -> Self {
        match status_code {
            StatusCode::Ok => ServiceStatusCode::Ok,
            StatusCode::InvalidTiles => ServiceStatusCode::InvalidTiles,
            StatusCode::InvalidStockTiles => ServiceStatusCode::InvalidStockTiles,
            StatusCode::TaskAlreadyRunning => ServiceStatusCode::TaskAlreadyRunning,
            StatusCode::ServerUnavailable => ServiceStatusCode::ServerUnavailable,
            StatusCode::TooManyPanels => ServiceStatusCode::TooManyPanels,
            StatusCode::TooManyStockPanels => ServiceStatusCode::TooManyStockPanels,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_status_code_values() {
        assert_eq!(ServiceStatusCode::Ok.get_value(), 0);
        assert_eq!(ServiceStatusCode::InvalidTiles.get_value(), 1);
        assert_eq!(ServiceStatusCode::InvalidStockTiles.get_value(), 2);
        assert_eq!(ServiceStatusCode::TaskAlreadyRunning.get_value(), 3);
        assert_eq!(ServiceStatusCode::ServerUnavailable.get_value(), 4);
        assert_eq!(ServiceStatusCode::TooManyPanels.get_value(), 5);
        assert_eq!(ServiceStatusCode::TooManyStockPanels.get_value(), 6);
    }

    #[test]
    fn test_service_status_code_string_values() {
        assert_eq!(ServiceStatusCode::Ok.get_string_value(), "0");
        assert_eq!(ServiceStatusCode::InvalidTiles.get_string_value(), "1");
        assert_eq!(ServiceStatusCode::InvalidStockTiles.get_string_value(), "2");
        assert_eq!(ServiceStatusCode::TaskAlreadyRunning.get_string_value(), "3");
        assert_eq!(ServiceStatusCode::ServerUnavailable.get_string_value(), "4");
        assert_eq!(ServiceStatusCode::TooManyPanels.get_string_value(), "5");
        assert_eq!(ServiceStatusCode::TooManyStockPanels.get_string_value(), "6");
    }

    #[test]
    fn test_status_code_conversion() {
        let service_code = ServiceStatusCode::Ok;
        let status_code: StatusCode = service_code.into();
        let back_to_service: ServiceStatusCode = status_code.into();
        
        assert_eq!(service_code, back_to_service);
    }
}
