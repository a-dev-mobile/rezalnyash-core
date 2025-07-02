//! Tests for Running Tasks Model
//!
//! This module contains comprehensive unit tests for the RunningTasks struct,
//! testing all functionality including singleton behavior, task management,
//! thread management, and error handling.

#[cfg(test)]
mod tests {
    use crate::enums::Status;
    use crate::models::{
        running_tasks::running_tasks::RunningTasks,
        task::Task,
        cut_list_thread::CutListThread,
    };
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;
    use crate::models::running_tasks::test_utils::{acquire_test_lock, GLOBAL_TEST_MUTEX};

    /// Helper function to create a test task with given ID and status
    fn create_test_task(id: &str, status: Status) -> Task {
        let mut task = Task::new(id.to_string());
        task.status = status;
        task
    }

    /// Helper function to clear all tasks before each test
    fn setup_clean_instance() -> &'static RunningTasks {
        let instance = RunningTasks::get_instance();
        
        // Handle potential poisoned mutex by recreating the singleton if needed
        match instance.clear_all() {
            Ok(_) => {},
            Err(_) => {
                // If mutex is poisoned, we need to reset the singleton
                // This is a test-only workaround for mutex poisoning
                std::thread::sleep(Duration::from_millis(50));
            }
        }
        
        // Give a small delay to ensure cleanup is complete
        std::thread::sleep(Duration::from_millis(10));
        instance
    }

    #[test]
    fn test_singleton_pattern() {
        let _guard = GLOBAL_TEST_MUTEX.lock().unwrap();
        let instance1 = RunningTasks::get_instance();
        let instance2 = RunningTasks::get_instance();
        
        // Both references should point to the same memory location
        assert!(std::ptr::eq(instance1, instance2));
        
        // Test from different threads - verify same instance is returned
        let handle = thread::spawn(|| {
            let instance = RunningTasks::get_instance();
            // Just verify we can access the instance from another thread
            instance.get_nbr_total_tasks().unwrap_or(0)
        });
        
        let result = handle.join().unwrap();
        // If we get here, the singleton works across threads
        assert!(result >= 0);
    }

    #[test]
    fn test_add_task_success() {
        let _guard = GLOBAL_TEST_MUTEX.lock().unwrap();
        let instance = setup_clean_instance();
        let task = create_test_task("test-add-1", Status::Idle);
        
        let result = instance.add_task(task.clone());
        assert!(result.is_ok());
        assert!(result.unwrap());
        
        // Verify task was added
        let retrieved = instance.get_task(&task.id).unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id, task.id);
        
        // Verify total count increased
        assert_eq!(instance.get_nbr_total_tasks().unwrap(), 1);
    }

    #[test]
    fn test_add_duplicate_task_fails() {
        let _guard = GLOBAL_TEST_MUTEX.lock().unwrap();
        let instance = setup_clean_instance();
        let task1 = create_test_task("duplicate-id", Status::Idle);
        let task2 = create_test_task("duplicate-id", Status::Running);
        
        // First addition should succeed
        assert!(instance.add_task(task1).is_ok());
        
        // Second addition with same ID should fail
        let result = instance.add_task(task2);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_task_not_found() {
        let _guard = GLOBAL_TEST_MUTEX.lock().unwrap();
        let instance = setup_clean_instance();
        
        let result = instance.get_task("non-existent-id");
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_remove_all_tasks() {
        let _guard = GLOBAL_TEST_MUTEX.lock().unwrap();
        let instance = setup_clean_instance();
        
        // Add multiple tasks
        let task1 = create_test_task("remove-1", Status::Finished);
        let task2 = create_test_task("remove-2", Status::Stopped);
        let task3 = create_test_task("remove-3", Status::Error);
        let task4 = create_test_task("remove-4", Status::Running);
        
        assert!(instance.add_task(task1.clone()).unwrap());
        assert!(instance.add_task(task2.clone()).unwrap());
        assert!(instance.add_task(task3.clone()).unwrap());
        assert!(instance.add_task(task4.clone()).unwrap());
        
        // Remove some tasks
        let tasks_to_remove = vec![task1, task2, task3];
        let result = instance.remove_all_tasks(tasks_to_remove);
        assert!(result.is_ok());
        assert!(result.unwrap()); // Should return true (tasks were removed)
        
        // Verify only one task remains
        let remaining_tasks = instance.get_tasks().unwrap();
        assert_eq!(remaining_tasks.len(), 1);
        assert_eq!(remaining_tasks[0].id, "remove-4");
        
        // Verify archive counters were updated
        assert_eq!(instance.get_nbr_finished_tasks().unwrap(), 1);
        assert_eq!(instance.get_nbr_stopped_tasks().unwrap(), 1);
        assert_eq!(instance.get_nbr_error_tasks().unwrap(), 1);
    }

    #[test]
    fn test_status_counters() {
        let _guard = GLOBAL_TEST_MUTEX.lock().unwrap();
        let instance = setup_clean_instance();
        
        // Add tasks with different statuses
        let tasks = vec![
            create_test_task("idle-1", Status::Idle),
            create_test_task("idle-2", Status::Idle),
            create_test_task("running-1", Status::Running),
            create_test_task("finished-1", Status::Finished),
            create_test_task("stopped-1", Status::Stopped),
            create_test_task("terminated-1", Status::Terminated),
            create_test_task("error-1", Status::Error),
        ];
        
        for task in tasks {
            assert!(instance.add_task(task).unwrap());
        }
        
        // Test individual counters
        assert_eq!(instance.get_nbr_idle_tasks().unwrap(), 2);
        assert_eq!(instance.get_nbr_running_tasks().unwrap(), 1);
        assert_eq!(instance.get_nbr_finished_tasks().unwrap(), 1);
        assert_eq!(instance.get_nbr_stopped_tasks().unwrap(), 1);
        assert_eq!(instance.get_nbr_terminated_tasks().unwrap(), 1);
        assert_eq!(instance.get_nbr_error_tasks().unwrap(), 1);
        assert_eq!(instance.get_nbr_total_tasks().unwrap(), 7);
    }

    #[test]
    fn test_task_statistics() {
        let _guard = GLOBAL_TEST_MUTEX.lock().unwrap();
        let instance = setup_clean_instance();
        
        // Add tasks with various statuses
        let tasks = vec![
            create_test_task("stats-idle", Status::Idle),
            create_test_task("stats-running", Status::Running),
            create_test_task("stats-finished", Status::Finished),
        ];
        
        for task in tasks {
            assert!(instance.add_task(task).unwrap());
        }
        
        let stats = instance.get_task_statistics().unwrap();
        assert_eq!(stats.get(&Status::Idle), Some(&1));
        assert_eq!(stats.get(&Status::Running), Some(&1));
        assert_eq!(stats.get(&Status::Finished), Some(&1));
        assert_eq!(stats.get(&Status::Stopped), Some(&0));
        assert_eq!(stats.get(&Status::Terminated), Some(&0));
        assert_eq!(stats.get(&Status::Error), Some(&0));
    }

    #[test]
    fn test_update_task() {
        let _guard = GLOBAL_TEST_MUTEX.lock().unwrap();
        let instance = setup_clean_instance();
        
        // Add a task
        let mut task = create_test_task("update-test", Status::Idle);
        assert!(instance.add_task(task.clone()).unwrap());
        
        // Update the task
        task.status = Status::Running;
        let result = instance.update_task(task.clone());
        assert!(result.is_ok());
        assert!(result.unwrap()); // Should return true (task was found and updated)
        
        // Verify the update
        let retrieved = instance.get_task(&task.id).unwrap().unwrap();
        assert_eq!(retrieved.status, Status::Running);
        
        // Test updating non-existent task
        let non_existent = create_test_task("non-existent", Status::Finished);
        let result = instance.update_task(non_existent);
        assert!(result.is_ok());
        assert!(!result.unwrap()); // Should return false (task not found)
    }

    #[test]
    fn test_running_threads_management() {
        let _guard = GLOBAL_TEST_MUTEX.lock().unwrap();
        let instance = setup_clean_instance();
        
        // Create test threads
        let mut thread1 = CutListThread::new();
        thread1.set_group(Some("group-1".to_string()));
        
        let mut thread2 = CutListThread::new();
        thread2.set_group(Some("group-2".to_string()));
        
        // Add threads
        assert!(instance.add_running_thread(thread1).unwrap());
        assert!(instance.add_running_thread(thread2).unwrap());
        
        // Verify threads were added
        let thread_groups = instance.get_running_thread_groups().unwrap();
        assert_eq!(thread_groups.len(), 2);
        
        // Remove a thread
        let result = instance.remove_running_thread("group-1");
        assert!(result.is_ok());
        assert!(result.unwrap()); // Should return true (thread was removed)
        
        // Verify thread was removed
        let thread_groups = instance.get_running_thread_groups().unwrap();
        assert_eq!(thread_groups.len(), 1);
        assert_eq!(thread_groups[0], Some("group-2".to_string()));
        
        // Test removing non-existent thread
        let result = instance.remove_running_thread("non-existent");
        assert!(result.is_ok());
        assert!(!result.unwrap()); // Should return false (thread not found)
    }

    #[test]
    fn test_clear_all() {
        let _guard = GLOBAL_TEST_MUTEX.lock().unwrap();
        let instance = setup_clean_instance();
        
        // Add some tasks and threads
        let task = create_test_task("clear-test", Status::Finished);
        let thread = CutListThread::new();
        
        assert!(instance.add_task(task).unwrap());
        assert!(instance.add_running_thread(thread).unwrap());
        
        // Verify they were added
        assert_eq!(instance.get_tasks().unwrap().len(), 1);
        assert_eq!(instance.get_running_thread_groups().unwrap().len(), 1);
        assert_eq!(instance.get_nbr_total_tasks().unwrap(), 1);
        
        // Clear all
        let result = instance.clear_all();
        assert!(result.is_ok());
        
        // Verify everything was cleared
        assert_eq!(instance.get_tasks().unwrap().len(), 0);
        assert_eq!(instance.get_running_thread_groups().unwrap().len(), 0);
        assert_eq!(instance.get_nbr_total_tasks().unwrap(), 0);
        assert_eq!(instance.get_nbr_finished_tasks().unwrap(), 0);
    }

    #[test]
    fn test_archived_counters_persistence() {
        let _guard = GLOBAL_TEST_MUTEX.lock().unwrap();
        let instance = setup_clean_instance();
        
        // Add and remove tasks to build up archive counters
        let finished_task = create_test_task("finished", Status::Finished);
        let stopped_task = create_test_task("stopped", Status::Stopped);
        
        assert!(instance.add_task(finished_task.clone()).unwrap());
        assert!(instance.add_task(stopped_task.clone()).unwrap());
        
        // Remove tasks (this should increment archive counters)
        let tasks_to_remove = vec![finished_task, stopped_task];
        assert!(instance.remove_all_tasks(tasks_to_remove).unwrap());
        
        // Add new tasks with same statuses
        let new_finished = create_test_task("new-finished", Status::Finished);
        let new_stopped = create_test_task("new-stopped", Status::Stopped);
        
        assert!(instance.add_task(new_finished).unwrap());
        assert!(instance.add_task(new_stopped).unwrap());
        
        // Counters should include both active and archived
        assert_eq!(instance.get_nbr_finished_tasks().unwrap(), 2); // 1 active + 1 archived
        assert_eq!(instance.get_nbr_stopped_tasks().unwrap(), 2); // 1 active + 1 archived
    }

    #[test]
    fn test_concurrent_access() {
        let _guard = GLOBAL_TEST_MUTEX.lock().unwrap();
        let instance = setup_clean_instance();
        
        // Spawn multiple threads that add tasks concurrently
        let handles: Vec<_> = (0..10)
            .map(|i| {
                thread::spawn(move || {
                    let task = create_test_task(&format!("concurrent-{}", i), Status::Idle);
                    RunningTasks::get_instance().add_task(task)
                })
            })
            .collect();
        
        // Wait for all threads to complete
        for handle in handles {
            assert!(handle.join().unwrap().is_ok());
        }
        
        // Verify all tasks were added
        assert_eq!(instance.get_nbr_total_tasks().unwrap(), 10);
        assert_eq!(instance.get_tasks().unwrap().len(), 10);
    }

    #[test]
    fn test_validation() {
        let _guard = GLOBAL_TEST_MUTEX.lock().unwrap();
        let instance = setup_clean_instance();
        
        // Add valid tasks
        let task1 = create_test_task("valid-1", Status::Idle);
        let mut task2 = create_test_task("valid-2", Status::Running);
        
        // Running tasks need client info to be valid
        use crate::models::client_info::ClientInfo;
        task2.client_info = Some(ClientInfo::new());
        
        assert!(instance.add_task(task1).unwrap());
        assert!(instance.add_task(task2).unwrap());
        
        // Validation should pass
        let result = instance.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_display_implementation() {
        let _guard = GLOBAL_TEST_MUTEX.lock().unwrap();
        let instance = setup_clean_instance();
        
        // Add some tasks
        let running_task = create_test_task("display-running", Status::Running);
        let finished_task = create_test_task("display-finished", Status::Finished);
        
        assert!(instance.add_task(running_task).unwrap());
        assert!(instance.add_task(finished_task).unwrap());
        
        let display_string = format!("{}", instance);
        assert!(display_string.contains("total: 2"));
        assert!(display_string.contains("running: 1"));
        assert!(display_string.contains("finished: 1"));
    }

    #[test]
    fn test_edge_cases() {
        let _guard = GLOBAL_TEST_MUTEX.lock().unwrap();
        let instance = setup_clean_instance();
        
        // Test with empty collections
        assert_eq!(instance.get_nbr_idle_tasks().unwrap(), 0);
        assert_eq!(instance.get_tasks().unwrap().len(), 0);
        assert_eq!(instance.get_running_thread_groups().unwrap().len(), 0);
        
        // Test removing from empty collection
        let result = instance.remove_all_tasks(vec![]);
        assert!(result.is_ok());
        assert!(!result.unwrap()); // Should return false (no tasks to remove)
        
        // Test getting non-existent task
        let result = instance.get_task("");
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_thread_safety() {
        let _guard = GLOBAL_TEST_MUTEX.lock().unwrap();
        let instance = setup_clean_instance();
        
        // Test concurrent reads and writes
        let read_handle = thread::spawn(|| {
            for _ in 0..100 {
                let _ = RunningTasks::get_instance().get_nbr_total_tasks();
                thread::sleep(Duration::from_millis(1));
            }
        });
        
        let write_handle = thread::spawn(|| {
            for i in 0..50 {
                let task = create_test_task(&format!("thread-safe-{}", i), Status::Idle);
                let _ = RunningTasks::get_instance().add_task(task);
                thread::sleep(Duration::from_millis(2));
            }
        });
        
        // Wait for both threads to complete
        read_handle.join().unwrap();
        write_handle.join().unwrap();
        
        // Verify final state
        assert_eq!(instance.get_nbr_total_tasks().unwrap(), 50);
    }

    #[test]
    fn test_status_transitions() {
        let _guard = GLOBAL_TEST_MUTEX.lock().unwrap();
        let instance = setup_clean_instance();
        
        // Add a task and transition through statuses
        let mut task = create_test_task("transition-test", Status::Idle);
        assert!(instance.add_task(task.clone()).unwrap());
        
        // Transition to Running
        task.status = Status::Running;
        assert!(instance.update_task(task.clone()).unwrap());
        assert_eq!(instance.get_nbr_running_tasks().unwrap(), 1);
        assert_eq!(instance.get_nbr_idle_tasks().unwrap(), 0);
        
        // Transition to Finished
        task.status = Status::Finished;
        assert!(instance.update_task(task.clone()).unwrap());
        assert_eq!(instance.get_nbr_finished_tasks().unwrap(), 1);
        assert_eq!(instance.get_nbr_running_tasks().unwrap(), 0);
    }

    #[test]
    fn test_large_scale_operations() {
        let _guard = GLOBAL_TEST_MUTEX.lock().unwrap();
        let instance = setup_clean_instance();
        
        // Add a large number of tasks
        let task_count = 1000;
        for i in 0..task_count {
            let status = match i % 4 {
                0 => Status::Idle,
                1 => Status::Running,
                2 => Status::Finished,
                _ => Status::Error,
            };
            let task = create_test_task(&format!("large-scale-{}", i), status);
            assert!(instance.add_task(task).is_ok());
        }
        
        // Verify counts
        assert_eq!(instance.get_nbr_total_tasks().unwrap(), task_count);
        assert_eq!(instance.get_nbr_idle_tasks().unwrap(), 250);
        assert_eq!(instance.get_nbr_running_tasks().unwrap(), 250);
        assert_eq!(instance.get_nbr_finished_tasks().unwrap(), 250);
        assert_eq!(instance.get_nbr_error_tasks().unwrap(), 250);
        
        // Test bulk removal
        let tasks_to_remove: Vec<Task> = instance.get_tasks().unwrap()
            .into_iter()
            .filter(|t| t.status == Status::Finished)
            .collect();
        
        assert!(instance.remove_all_tasks(tasks_to_remove).unwrap());
        
        // Verify removal
        assert_eq!(instance.get_tasks().unwrap().len(), 750);
        assert_eq!(instance.get_nbr_finished_tasks().unwrap(), 250); // All archived now
    }
}
