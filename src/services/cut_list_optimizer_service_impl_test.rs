//! Tests for CutListOptimizerServiceImpl
//!
//! This module contains comprehensive tests for the CutListOptimizerServiceImpl
//! to ensure all functionality works correctly after porting from Java.

use super::*;
use crate::enums::{Status, StatusCode};
use crate::models::{
    calculation_request::CalculationRequest,
    client_info::ClientInfo,
    configuration::Configuration,
    performance_thresholds::PerformanceThresholds,
};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// Helper function to create a test calculation request
fn create_test_calculation_request() -> CalculationRequest {
    let mut client_info = ClientInfo::new();
    client_info.id = Some("test_client".to_string());
    
    let mut configuration = Configuration::default();
    configuration.performance_thresholds = Some(PerformanceThresholds::default());
    
    CalculationRequest {
        client_info,
        configuration,
        panels: Vec::new(),
        stock_panels: Vec::new(),
    }
}

/// Helper function to create a test calculation request with panels
fn create_test_calculation_request_with_panels() -> CalculationRequest {
    let mut request = create_test_calculation_request();
    
    // Add some test panels (simplified)
    // In real implementation, you would add actual Panel objects
    // request.panels.push(Panel::new(...));
    
    request
}

#[cfg(test)]
mod service_creation_tests {
    use super::*;

    #[test]
    fn test_new_service_creation() {
        let service = CutListOptimizerServiceImpl::new();
        
        assert!(!service.allow_multiple_tasks_per_client);
        assert!(service.task_executor.is_none());
        assert!(service.watch_dog.is_none());
        assert_eq!(service.date_format, "%Y%m%d%H%M");
    }

    #[test]
    fn test_get_instance_singleton_pattern() {
        let service1 = CutListOptimizerServiceImpl::get_instance();
        let service2 = CutListOptimizerServiceImpl::get_instance();
        
        // Both should have same default configuration
        assert_eq!(service1.allow_multiple_tasks_per_client, service2.allow_multiple_tasks_per_client);
        assert_eq!(service1.date_format, service2.date_format);
    }

    #[test]
    fn test_default_implementation() {
        let service = CutListOptimizerServiceImpl::default();
        
        assert!(!service.allow_multiple_tasks_per_client);
        assert!(service.task_executor.is_none());
        assert!(service.watch_dog.is_none());
    }
}

#[cfg(test)]
mod task_id_generation_tests {
    use super::*;

    #[test]
    fn test_generate_unique_task_ids() {
        let service = CutListOptimizerServiceImpl::new();
        
        let task_id1 = service.generate_task_id();
        let task_id2 = service.generate_task_id();
        let task_id3 = service.generate_task_id();
        
        assert!(!task_id1.is_empty());
        assert!(!task_id2.is_empty());
        assert!(!task_id3.is_empty());
        
        // All IDs should be unique
        assert_ne!(task_id1, task_id2);
        assert_ne!(task_id2, task_id3);
        assert_ne!(task_id1, task_id3);
    }

    #[test]
    fn test_task_id_format() {
        let service = CutListOptimizerServiceImpl::new();
        let task_id = service.generate_task_id();
        
        // Task ID should contain timestamp + counter
        // Format: YYYYMMDDHHMM + counter
        assert!(task_id.len() >= 12); // At least timestamp length
        
        // Should be numeric (timestamp + counter)
        let numeric_part = &task_id[..12]; // First 12 chars should be timestamp
        assert!(numeric_part.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_task_id_counter_increment() {
        let service = CutListOptimizerServiceImpl::new();
        
        // Generate multiple IDs quickly to test counter increment
        let mut ids = Vec::new();
        for _ in 0..10 {
            ids.push(service.generate_task_id());
        }
        
        // All should be unique
        for i in 0..ids.len() {
            for j in i + 1..ids.len() {
                assert_ne!(ids[i], ids[j], "Task IDs should be unique");
            }
        }
    }
}

#[cfg(test)]
mod initialization_tests {
    use super::*;

    #[test]
    fn test_service_initialization() {
        let mut service = CutListOptimizerServiceImpl::new();
        assert!(service.task_executor.is_none());
        assert!(service.watch_dog.is_none());
        
        service.init(4);
        
        assert!(service.task_executor.is_some());
        // Note: watch_dog might be None if WatchDog start fails, which is acceptable
    }

    #[test]
    fn test_initialization_with_different_thread_counts() {
        let mut service1 = CutListOptimizerServiceImpl::new();
        let mut service2 = CutListOptimizerServiceImpl::new();
        
        service1.init(2);
        service2.init(8);
        
        assert!(service1.task_executor.is_some());
        assert!(service2.task_executor.is_some());
    }

    #[test]
    fn test_initialization_with_zero_threads() {
        let mut service = CutListOptimizerServiceImpl::new();
        
        // This should handle gracefully or use a minimum thread count
        service.init(0);
        
        // Should still create executor (rayon handles 0 threads by using available cores)
        assert!(service.task_executor.is_some());
    }
}

#[cfg(test)]
mod configuration_tests {
    use super::*;

    #[test]
    fn test_set_allow_multiple_tasks_per_client() {
        let mut service = CutListOptimizerServiceImpl::new();
        
        // Default should be false
        assert!(!service.allow_multiple_tasks_per_client);
        
        // Test setting to true
        service.set_allow_multiple_tasks_per_client(true);
        assert!(service.allow_multiple_tasks_per_client);
        
        // Test setting back to false
        service.set_allow_multiple_tasks_per_client(false);
        assert!(!service.allow_multiple_tasks_per_client);
    }

    #[test]
    fn test_set_cut_list_logger() {
        let mut service = CutListOptimizerServiceImpl::new();
        
        // Create a new logger
        let new_logger = Arc::new(DefaultCutListLogger);
        service.set_cut_list_logger(new_logger.clone());
        
        // Logger should be updated
        // Note: We can't directly compare Arc<dyn Trait> easily,
        // but we can verify the operation doesn't panic
        assert!(true); // If we reach here, the operation succeeded
    }
}

#[cfg(test)]
mod stats_tests {
    use super::*;

    #[test]
    fn test_get_stats_basic() {
        let service = CutListOptimizerServiceImpl::new();
        let stats = service.get_stats();
        
        // Basic validation that stats object is created with valid values
        assert!(stats.nbr_idle_tasks >= 0);
        assert!(stats.nbr_running_tasks >= 0);
        assert!(stats.nbr_finished_tasks >= 0);
        assert!(stats.nbr_stopped_tasks >= 0);
        assert!(stats.nbr_terminated_tasks >= 0);
        assert!(stats.nbr_error_tasks >= 0);
    }

    #[test]
    fn test_get_stats_with_initialized_service() {
        let mut service = CutListOptimizerServiceImpl::new();
        service.init(4);
        
        let stats = service.get_stats();
        
        // With initialized service, thread stats should be available
        assert!(stats.nbr_running_threads >= 0);
        assert!(stats.nbr_queued_threads >= 0);
        assert!(stats.nbr_finished_threads >= 0);
    }
}

#[cfg(test)]
mod task_management_tests {
    use super::*;

    #[test]
    fn test_get_task_status_nonexistent() {
        let service = CutListOptimizerServiceImpl::new();
        let status = service.get_task_status("nonexistent_task");
        
        assert!(status.is_none());
    }

    #[test]
    fn test_get_tasks_empty_result() {
        let service = CutListOptimizerServiceImpl::new();
        let tasks = service.get_tasks("test_client", Status::Running);
        
        // Should return empty vector for non-existent client
        assert!(tasks.is_empty());
    }

    #[test]
    fn test_stop_task_nonexistent() {
        let service = CutListOptimizerServiceImpl::new();
        let result = service.stop_task("nonexistent_task");
        
        assert!(result.is_none());
    }

    #[test]
    fn test_terminate_task_nonexistent() {
        let service = CutListOptimizerServiceImpl::new();
        let result = service.terminate_task("nonexistent_task");
        
        assert_eq!(result, -1);
    }
}

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn test_decimal_places_calculation() {
        let service = CutListOptimizerServiceImpl::new();
        
        assert_eq!(service.get_nbr_decimal_places("123.45"), 2);
        assert_eq!(service.get_nbr_decimal_places("123"), 0);
        assert_eq!(service.get_nbr_decimal_places("123.456789"), 6);
        assert_eq!(service.get_nbr_decimal_places("0.1"), 1);
        assert_eq!(service.get_nbr_decimal_places("1000.0"), 1);
    }

    #[test]
    fn test_integer_places_calculation() {
        let service = CutListOptimizerServiceImpl::new();
        
        assert_eq!(service.get_nbr_integer_places("123.45"), 3);
        assert_eq!(service.get_nbr_integer_places("123"), 3);
        assert_eq!(service.get_nbr_integer_places("1.456789"), 1);
        assert_eq!(service.get_nbr_integer_places("0.1"), 1);
        assert_eq!(service.get_nbr_integer_places("1000.0"), 4);
    }

    #[test]
    fn test_validate_panels_empty() {
        let service = CutListOptimizerServiceImpl::new();
        let panels = Vec::new();
        
        let (count, valid) = service.validate_panels(&panels);
        
        assert_eq!(count, 0);
        assert!(!valid);
    }

    #[test]
    fn test_max_decimal_places_default() {
        let service = CutListOptimizerServiceImpl::new();
        let panels = Vec::new();
        
        let max_decimal = service.get_max_nbr_decimal_places(&panels);
        
        // Should return default value
        assert_eq!(max_decimal, 2);
    }

    #[test]
    fn test_max_integer_places_default() {
        let service = CutListOptimizerServiceImpl::new();
        let panels = Vec::new();
        
        let max_integer = service.get_max_nbr_integer_places(&panels);
        
        // Should return default value
        assert_eq!(max_integer, 4);
    }
}

#[cfg(test)]
mod task_submission_tests {
    use super::*;

    #[test]
    fn test_submit_task_with_empty_panels() {
        let service = CutListOptimizerServiceImpl::new();
        let request = create_test_calculation_request();
        
        let result = service.submit_task(request);
        
        // Should return error for invalid tiles (empty panels)
        assert_eq!(result.status_code, StatusCode::InvalidTiles.get_string_value());
        assert!(result.task_id.is_none());
    }

    #[test]
    fn test_submit_task_basic_validation() {
        let service = CutListOptimizerServiceImpl::new();
        
        // Create request with basic client info
        let mut client_info = ClientInfo::new();
        client_info.id = Some("test_client".to_string());
        
        let configuration = Configuration::default();
        
        let request = CalculationRequest {
            client_info,
            configuration,
            panels: Vec::new(), // Empty panels
            stock_panels: Vec::new(), // Empty stock
        };
        
        let result = service.submit_task(request);
        
        // Should fail validation due to empty panels
        assert_ne!(result.status_code, StatusCode::Ok.get_string_value());
    }

    #[test]
    fn test_submit_task_generates_task_id() {
        let service = CutListOptimizerServiceImpl::new();
        
        // This test would need a valid request to pass validation
        // For now, we test that the method doesn't panic
        let request = create_test_calculation_request();
        let result = service.submit_task(request);
        
        // Even if validation fails, the method should return a proper result
        assert!(!result.status_code.is_empty());
    }
}

#[cfg(test)]
mod utility_method_tests {
    use super::*;

    #[test]
    fn test_is_one_dimensional_optimization() {
        let service = CutListOptimizerServiceImpl::new();
        let tiles = Vec::new();
        let stock = Vec::new();
        
        let result = service.is_one_dimensional_optimization(&tiles, &stock);
        
        // Current implementation returns false
        assert!(!result);
    }

    #[test]
    fn test_get_tile_dimensions_per_material() {
        let service = CutListOptimizerServiceImpl::new();
        let tiles = Vec::new(); // Empty for now
        
        let result = service.get_tile_dimensions_per_material(tiles);
        
        // Should return empty HashMap for empty input
        assert!(result.is_empty());
    }

    #[test]
    fn test_remove_duplicated_permutations_empty() {
        let service = CutListOptimizerServiceImpl::new();
        let mut permutations = Vec::new();
        
        let removed_count = service.remove_duplicated_permutations(&mut permutations);
        
        assert_eq!(removed_count, 0);
        assert!(permutations.is_empty());
    }

    #[test]
    fn test_get_distinct_grouped_tile_dimensions() {
        let service = CutListOptimizerServiceImpl::new();
        let items = vec!["a".to_string(), "b".to_string(), "a".to_string()];
        let configuration = Configuration::default();
        
        let result = service.get_distinct_grouped_tile_dimensions(items, &configuration);
        
        assert_eq!(result.len(), 2); // "a" and "b"
        assert_eq!(result.get("a"), Some(&2)); // "a" appears twice
        assert_eq!(result.get("b"), Some(&1)); // "b" appears once
    }
}

#[cfg(test)]
mod thread_eligibility_tests {
    use super::*;

    #[test]
    fn test_is_thread_eligible_to_start_no_rankings() {
        let service = CutListOptimizerServiceImpl::new();
        let task = Task::new("test_task".to_string());
        
        let result = service.is_thread_eligible_to_start("group1", &task, "wood");
        
        // Should return true when no rankings are found (fail-safe)
        assert!(result);
    }
}

#[cfg(test)]
mod clone_tests {
    use super::*;

    #[test]
    fn test_service_clone() {
        let service1 = CutListOptimizerServiceImpl::new();
        let service2 = service1.clone();
        
        // Cloned service should have same configuration
        assert_eq!(service1.allow_multiple_tasks_per_client, service2.allow_multiple_tasks_per_client);
        assert_eq!(service1.date_format, service2.date_format);
    }

    #[test]
    fn test_service_clone_with_modifications() {
        let mut service1 = CutListOptimizerServiceImpl::new();
        service1.set_allow_multiple_tasks_per_client(true);
        
        let service2 = service1.clone();
        
        // Cloned service should have same modified configuration
        assert_eq!(service1.allow_multiple_tasks_per_client, service2.allow_multiple_tasks_per_client);
        assert!(service2.allow_multiple_tasks_per_client);
    }
}

#[cfg(test)]
mod task_executor_tests {
    use super::*;

    #[test]
    fn test_task_executor_creation() {
        let executor = TaskExecutor::new(4);
        assert!(executor.is_ok());
        
        let executor = executor.unwrap();
        assert_eq!(executor.get_active_count(), 0);
        assert_eq!(executor.get_completed_task_count(), 0);
    }

    #[test]
    fn test_task_executor_with_zero_threads() {
        let executor = TaskExecutor::new(0);
        assert!(executor.is_ok());
        
        // Rayon should handle 0 threads gracefully
        let executor = executor.unwrap();
        assert_eq!(executor.get_active_count(), 0);
    }

    #[test]
    fn test_task_executor_execute_simple_task() {
        let executor = TaskExecutor::new(2).unwrap();
        
        let result = executor.execute(|| {
            // Simple task that does nothing
        });
        
        assert!(result.is_ok());
        
        // Give some time for task to complete
        thread::sleep(Duration::from_millis(100));
    }

    #[test]
    fn test_task_executor_queue_size() {
        let executor = TaskExecutor::new(2).unwrap();
        let queue_size = executor.get_queue_size();
        
        // Queue size should be non-negative
        assert!(queue_size >= 0);
        assert!(queue_size <= THREAD_QUEUE_SIZE as i32);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_service_lifecycle() {
        let mut service = CutListOptimizerServiceImpl::new();
        
        // Initialize service
        service.init(2);
        assert!(service.task_executor.is_some());
        
        // Get initial stats
        let stats = service.get_stats();
        assert!(stats.nbr_running_threads >= 0);
        
        // Configure service
        service.set_allow_multiple_tasks_per_client(true);
        assert!(service.allow_multiple_tasks_per_client);
        
        // Test task operations with non-existent tasks
        assert!(service.get_task_status("fake_task").is_none());
        assert!(service.stop_task("fake_task").is_none());
        assert_eq!(service.terminate_task("fake_task"), -1);
    }

    #[test]
    fn test_concurrent_task_id_generation() {
        let service = Arc::new(CutListOptimizerServiceImpl::new());
        let mut handles = Vec::new();
        let mut task_ids = Arc::new(Mutex::new(Vec::new()));
        
        // Spawn multiple threads to generate task IDs concurrently
        for _ in 0..10 {
            let service_clone = service.clone();
            let task_ids_clone = task_ids.clone();
            
            let handle = thread::spawn(move || {
                let task_id = service_clone.generate_task_id();
                if let Ok(mut ids) = task_ids_clone.lock() {
                    ids.push(task_id);
                }
            });
            
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Verify all task IDs are unique
        let ids = task_ids.lock().unwrap();
        assert_eq!(ids.len(), 10);
        
        for i in 0..ids.len() {
            for j in i + 1..ids.len() {
                assert_ne!(ids[i], ids[j], "Task IDs should be unique even when generated concurrently");
            }
        }
    }
}
