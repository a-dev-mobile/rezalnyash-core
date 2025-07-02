//! Tests for the WatchDog module
//!
//! This module contains comprehensive tests for the WatchDog functionality,
//! including monitoring, task management, and thread safety.

use super::*;
use crate::models::task::Task;
use crate::models::client_info::ClientInfo;
use crate::models::calculation_request::{CalculationRequest, Panel};
use crate::models::running_tasks::running_tasks::RunningTasks;
use crate::enums::Status;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Global mutex to ensure tests run sequentially since they share singleton state
static TEST_MUTEX: Mutex<()> = Mutex::new(());

/// Mock logger for testing
#[derive(Debug, Clone)]
struct MockLogger {
    messages: Arc<Mutex<Vec<String>>>,
}

impl MockLogger {
    fn new() -> Self {
        Self {
            messages: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn get_messages(&self) -> Vec<String> {
        self.messages.lock().unwrap().clone()
    }

    fn clear_messages(&self) {
        self.messages.lock().unwrap().clear();
    }
}

impl CutListLogger for MockLogger {
    fn error(&self, message: &str) {
        self.messages.lock().unwrap().push(format!("ERROR: {}", message));
    }
    
    fn error_with_exception(&self, message: &str, error: &dyn std::error::Error) {
        self.messages.lock().unwrap().push(format!("ERROR: {}: {}", message, error));
    }
    
    fn warn(&self, client_id: &str, task_id: &str, message: &str) {
        self.messages.lock().unwrap().push(format!("WARN: Client[{}] Task[{}]: {}", client_id, task_id, message));
    }
    
    fn error_with_context(&self, client_id: &str, task_id: &str, message: &str) {
        self.messages.lock().unwrap().push(format!("ERROR: Client[{}] Task[{}]: {}", client_id, task_id, message));
    }
    
    fn log_execution(&self, task: &Task) {
        self.messages.lock().unwrap().push(format!("EXECUTION: Task[{}]", task.id));
    }
}

/// Mock optimizer service for testing
#[derive(Debug, Clone)]
struct MockOptimizerService {
    terminate_results: Arc<Mutex<std::collections::HashMap<String, i32>>>,
}

impl MockOptimizerService {
    fn new() -> Self {
        Self {
            terminate_results: Arc::new(Mutex::new(std::collections::HashMap::new())),
        }
    }

    fn set_terminate_result(&self, task_id: &str, result: i32) {
        self.terminate_results.lock().unwrap().insert(task_id.to_string(), result);
    }
}

impl CutListOptimizerService for MockOptimizerService {
    fn terminate_task(&self, task_id: &str) -> i32 {
        self.terminate_results
            .lock()
            .unwrap()
            .get(task_id)
            .copied()
            .unwrap_or(0)
    }
}

/// Mock thread pool executor for testing
#[derive(Debug)]
struct MockThreadPoolExecutor {
    active_count: i32,
    pool_size: i32,
    queue_size: i32,
    completed_count: u64,
}

impl MockThreadPoolExecutor {
    fn new(active_count: i32, pool_size: i32, queue_size: i32, completed_count: u64) -> Self {
        Self {
            active_count,
            pool_size,
            queue_size,
            completed_count,
        }
    }
}

impl ThreadPoolExecutor for MockThreadPoolExecutor {
    fn get_active_count(&self) -> i32 {
        self.active_count
    }
    
    fn get_pool_size(&self) -> i32 {
        self.pool_size
    }
    
    fn get_queue_size(&self) -> i32 {
        self.queue_size
    }
    
    fn get_completed_task_count(&self) -> u64 {
        self.completed_count
    }
}

fn create_test_task(id: &str, status: Status) -> Task {
    let mut task = Task::new(id.to_string());
    task.status = status;
    
    // Set up client info
    let mut client_info = ClientInfo::new();
    client_info.id = Some("client-123".to_string());
    task.client_info = Some(client_info);
    
    // Set up calculation request with panels
    let panels = vec![
        Panel::simple(1, "100.0".to_string(), "200.0".to_string(), 1),
        Panel::simple(2, "150.0".to_string(), "250.0".to_string(), 2),
        Panel::simple(3, "200.0".to_string(), "300.0".to_string(), 1),
    ];
    let calc_request = CalculationRequest::new().with_panels(panels);
    task.calculation_request = Some(calc_request);
    
    task
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_watch_dog() {
        let _guard = TEST_MUTEX.lock().unwrap();
        let watch_dog = WatchDog::new();
        
        assert!(!watch_dog.is_running());
        assert!(!watch_dog.is_paused());
        assert!(watch_dog.get_task_executor().is_none());
        assert!(watch_dog.get_cut_list_optimizer_service().is_none());
    }

    #[test]
    fn test_with_dependencies() {
        let _guard = TEST_MUTEX.lock().unwrap();
        let task_executor = Arc::new(MockThreadPoolExecutor::new(2, 4, 1, 10));
        let optimizer_service = Arc::new(MockOptimizerService::new());
        let logger = Arc::new(MockLogger::new());
        
        let watch_dog = WatchDog::with_dependencies(
            task_executor.clone(),
            optimizer_service.clone(),
            logger.clone(),
        );
        
        assert!(watch_dog.get_task_executor().is_some());
        assert!(watch_dog.get_cut_list_optimizer_service().is_some());
    }

    #[test]
    fn test_getters_and_setters() {
        let _guard = TEST_MUTEX.lock().unwrap();
        let mut watch_dog = WatchDog::new();
        
        // Test task executor
        let task_executor = Arc::new(MockThreadPoolExecutor::new(2, 4, 1, 10));
        watch_dog.set_task_executor(task_executor.clone());
        assert!(watch_dog.get_task_executor().is_some());
        
        // Test optimizer service
        let optimizer_service = Arc::new(MockOptimizerService::new());
        watch_dog.set_cut_list_optimizer_service(optimizer_service.clone());
        assert!(watch_dog.get_cut_list_optimizer_service().is_some());
        
        // Test logger
        let logger = Arc::new(MockLogger::new());
        watch_dog.set_cut_list_logger(logger.clone());
        // Logger is always set, so we can't test for None
    }

    #[test]
    fn test_task_reports() {
        let _guard = TEST_MUTEX.lock().unwrap();
        let watch_dog = WatchDog::new();
        
        let reports = watch_dog.get_task_reports().unwrap();
        assert!(reports.is_empty());
    }

    #[test]
    fn test_start_and_stop() {
        let _guard = TEST_MUTEX.lock().unwrap();
        let mut watch_dog = WatchDog::new();
        
        // Initially not running
        assert!(!watch_dog.is_running());
        
        // Start the watch dog
        let _control_sender = watch_dog.start().unwrap();
        assert!(watch_dog.is_running());
        
        // Try to start again (should fail)
        let result = watch_dog.start();
        assert!(result.is_err());
        
        // Stop the watch dog
        let stop_result = watch_dog.stop();
        assert!(stop_result.is_ok());
        
        // Give it time to stop
        thread::sleep(Duration::from_millis(200));
        
        // Try to stop again (should fail)
        let result = watch_dog.stop();
        assert!(result.is_err());
    }

    #[test]
    fn test_pause_and_resume() {
        let _guard = TEST_MUTEX.lock().unwrap();
        let mut watch_dog = WatchDog::new();
        
        // Start the watch dog
        let _control_sender = watch_dog.start().unwrap();
        
        // Initially not paused
        assert!(!watch_dog.is_paused());
        
        // Pause
        let pause_result = watch_dog.pause();
        assert!(pause_result.is_ok());
        
        // Give it time to process the pause signal
        thread::sleep(Duration::from_millis(100));
        assert!(watch_dog.is_paused());
        
        // Resume
        let resume_result = watch_dog.resume();
        assert!(resume_result.is_ok());
        
        // Give it time to process the resume signal
        thread::sleep(Duration::from_millis(100));
        assert!(!watch_dog.is_paused());
        
        // Stop
        let _stop_result = watch_dog.stop();
    }

    #[test]
    fn test_format_elapsed_time() {
        let _guard = TEST_MUTEX.lock().unwrap();
        
        assert_eq!(WatchDog::format_elapsed_time(500), "500ms");
        assert_eq!(WatchDog::format_elapsed_time(1500), "1.5s");
        assert_eq!(WatchDog::format_elapsed_time(65000), "1m 5s");
        assert_eq!(WatchDog::format_elapsed_time(3665000), "1h 1m");
    }

    #[test]
    fn test_monitoring_stats() {
        let _guard = TEST_MUTEX.lock().unwrap();
        let watch_dog = WatchDog::new();
        
        let stats = watch_dog.get_monitoring_stats().unwrap();
        assert_eq!(stats.0, 0); // active tasks
        assert_eq!(stats.1, 0); // total tasks
        assert_eq!(stats.2.len(), 0); // task reports
    }

    #[test]
    fn test_validate() {
        let _guard = TEST_MUTEX.lock().unwrap();
        let watch_dog = WatchDog::new();
        
        // Should validate successfully even without dependencies
        let result = watch_dog.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_default_implementations() {
        let _guard = TEST_MUTEX.lock().unwrap();
        
        // Test DefaultCutListLogger
        let logger = DefaultCutListLogger;
        logger.error("test error");
        logger.error_with_exception("test error", &std::io::Error::new(std::io::ErrorKind::Other, "test"));
        logger.warn("client1", "task1", "test warning");
        logger.error_with_context("client1", "task1", "test error");
        
        let task = create_test_task("test-task", Status::Running);
        logger.log_execution(&task);
        
        // Test DefaultCutListOptimizerService
        let optimizer = DefaultCutListOptimizerService;
        let result = optimizer.terminate_task("test-task");
        assert_eq!(result, 0);
        
        // Test DefaultThreadPoolExecutor
        let executor = DefaultThreadPoolExecutor::new(4);
        assert_eq!(executor.get_pool_size(), 4);
        assert_eq!(executor.get_active_count(), 0);
        assert_eq!(executor.get_queue_size(), 0);
        assert_eq!(executor.get_completed_task_count(), 0);
    }

    #[test]
    fn test_thread_safety() {
        let _guard = TEST_MUTEX.lock().unwrap();
        let watch_dog = Arc::new(Mutex::new(WatchDog::new()));
        let mut handles = vec![];
        
        // Spawn multiple threads to test concurrent access
        for _i in 0..5 {
            let watch_dog_clone = Arc::clone(&watch_dog);
            let handle = thread::spawn(move || {
                let wd = watch_dog_clone.lock().unwrap();
                // Just test that we can acquire the lock and call methods
                let _ = wd.is_running();
                let _ = wd.is_paused();
                let _ = wd.get_task_reports();
            });
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_drop_behavior() {
        let _guard = TEST_MUTEX.lock().unwrap();
        let mut watch_dog = WatchDog::new();
        
        // Start the watch dog
        let _control_sender = watch_dog.start().unwrap();
        assert!(watch_dog.is_running());
        
        // Drop should stop the watch dog
        drop(watch_dog);
        
        // Give it time to stop
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_mock_logger() {
        let _guard = TEST_MUTEX.lock().unwrap();
        let logger = MockLogger::new();
        
        logger.error("test error");
        logger.warn("client1", "task1", "test warning");
        
        let messages = logger.get_messages();
        assert_eq!(messages.len(), 2);
        assert!(messages[0].contains("ERROR: test error"));
        assert!(messages[1].contains("WARN: Client[client1] Task[task1]: test warning"));
        
        logger.clear_messages();
        let messages = logger.get_messages();
        assert!(messages.is_empty());
    }

    #[test]
    fn test_mock_optimizer_service() {
        let _guard = TEST_MUTEX.lock().unwrap();
        let optimizer = MockOptimizerService::new();
        
        // Default result should be 0
        assert_eq!(optimizer.terminate_task("test-task"), 0);
        
        // Set custom result
        optimizer.set_terminate_result("test-task", 1);
        assert_eq!(optimizer.terminate_task("test-task"), 1);
        
        // Unknown task should return 0
        assert_eq!(optimizer.terminate_task("unknown-task"), 0);
    }

    #[test]
    fn test_control_messages() {
        let _guard = TEST_MUTEX.lock().unwrap();
        let mut watch_dog = WatchDog::new();
        
        // Start the watch dog
        let control_sender = watch_dog.start().unwrap();
        assert!(watch_dog.is_running());
        
        // Send pause message
        control_sender.send(WatchDogControl::Pause).unwrap();
        thread::sleep(Duration::from_millis(100));
        assert!(watch_dog.is_paused());
        
        // Send resume message
        control_sender.send(WatchDogControl::Resume).unwrap();
        thread::sleep(Duration::from_millis(100));
        assert!(!watch_dog.is_paused());
        
        // Send stop message
        control_sender.send(WatchDogControl::Stop).unwrap();
        thread::sleep(Duration::from_millis(200));
        assert!(!watch_dog.is_running());
    }

    #[test]
    fn test_basic_functionality() {
        let _guard = TEST_MUTEX.lock().unwrap();
        let running_tasks = RunningTasks::get_instance();
        
        // Clear any existing tasks
        let _ = running_tasks.clear_all();
        
        // Create a watch dog with dependencies
        let task_executor = Arc::new(MockThreadPoolExecutor::new(2, 4, 1, 10));
        let optimizer_service = Arc::new(MockOptimizerService::new());
        let logger = Arc::new(MockLogger::new());
        
        let mut watch_dog = WatchDog::with_dependencies(
            task_executor,
            optimizer_service,
            logger.clone(),
        );
        
        // Add a test task
        let task = create_test_task("basic-test", Status::Running);
        let _ = running_tasks.add_task(task);
        
        // Start the watch dog
        let _control_sender = watch_dog.start().unwrap();
        
        // Let it run for a short time
        thread::sleep(Duration::from_millis(500));
        
        // Check that it's monitoring
        assert!(watch_dog.is_running());
        assert!(!watch_dog.is_paused());
        
        // Stop the watch dog
        let _ = watch_dog.stop();
        
        // Clean up
        let _ = running_tasks.clear_all();
    }
}
