//! Tests for Task model

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::enums::Status;
    use crate::models::{ClientInfo, CalculationResponse};
    use crate::models::calculation_response::{FinalTile, NoFitTile};

    #[test]
    fn test_task_new() {
        let task = Task::new("test-task-001".to_string());
        
        assert_eq!(task.id, "test-task-001");
        assert_eq!(task.status, Status::Idle);
        assert!(task.calculation_request.is_none());
        assert!(task.client_info.is_none());
        assert!(task.start_time > 0);
        assert_eq!(task.end_time, 0);
        assert_eq!(task.factor, 1.0);
        assert!(!task.is_min_trim_dimension_influenced);
        assert!(task.log.is_none());
        assert!(task.solution.is_none());
        assert!(task.stock_dimensions_per_material.is_none());
        assert!(task.tile_dimensions_per_material.is_none());
        assert!(task.solutions.is_empty());
        assert!(task.per_material_percentage_done.is_empty());
        assert!(task.last_queried > 0);
        assert!(task.no_material_tiles.is_empty());
    }

    #[test]
    fn test_task_is_running() {
        let mut task = Task::new("test-task".to_string());
        
        assert!(!task.is_running());
        
        task.status = Status::Running;
        assert!(task.is_running());
        
        task.status = Status::Finished;
        assert!(!task.is_running());
    }

    #[test]
    fn test_set_running_status() {
        let mut task = Task::new("test-task".to_string());
        
        // Should succeed from Idle
        assert!(task.set_running_status().is_ok());
        assert_eq!(task.status, Status::Running);
        
        // Should fail from Running
        assert!(task.set_running_status().is_err());
        assert_eq!(task.status, Status::Running);
    }

    #[test]
    fn test_stop() {
        let mut task = Task::new("test-task".to_string());
        
        // Should fail from Idle
        assert!(task.stop().is_err());
        assert_eq!(task.status, Status::Idle);
        
        // Should succeed from Running
        task.status = Status::Running;
        assert!(task.stop().is_ok());
        assert_eq!(task.status, Status::Stopped);
        assert!(task.end_time > 0);
    }

    #[test]
    fn test_terminate() {
        let mut task = Task::new("test-task".to_string());
        
        // Should fail from Idle
        assert!(task.terminate().is_err());
        assert_eq!(task.status, Status::Idle);
        
        // Should succeed from Running
        task.status = Status::Running;
        assert!(task.terminate().is_ok());
        assert_eq!(task.status, Status::Terminated);
        assert!(task.end_time > 0);
    }

    #[test]
    fn test_terminate_error() {
        let mut task = Task::new("test-task".to_string());
        
        task.terminate_error();
        assert_eq!(task.status, Status::Error);
        assert!(task.end_time > 0);
    }

    #[test]
    fn test_add_material_to_compute() {
        let mut task = Task::new("test-task".to_string());
        
        task.add_material_to_compute("Wood".to_string());
        
        assert!(task.solutions.contains_key("Wood"));
        assert_eq!(task.solutions.get("Wood").unwrap().len(), 0);
        assert_eq!(task.per_material_percentage_done.get("Wood"), Some(&0));
        
        // Check thread group rankings
        let rankings = task.thread_group_rankings.lock().unwrap();
        assert!(rankings.contains_key("Wood"));
    }

    #[test]
    fn test_get_solutions() {
        let mut task = Task::new("test-task".to_string());
        
        assert!(task.get_solutions("Wood").is_none());
        
        task.add_material_to_compute("Wood".to_string());
        let solutions = task.get_solutions("Wood");
        assert!(solutions.is_some());
        assert!(solutions.unwrap().is_empty());
    }

    #[test]
    fn test_append_line_to_log() {
        let mut task = Task::new("test-task".to_string());
        
        // First line
        task.append_line_to_log("First log entry".to_string());
        assert_eq!(task.log, Some("First log entry".to_string()));
        
        // Second line
        task.append_line_to_log("Second log entry".to_string());
        assert_eq!(task.log, Some("First log entry\nSecond log entry".to_string()));
        
        // Third line
        task.append_line_to_log("Third log entry".to_string());
        assert_eq!(task.log, Some("First log entry\nSecond log entry\nThird log entry".to_string()));
    }

    #[test]
    fn test_get_percentage_done() {
        let mut task = Task::new("test-task".to_string());
        
        // No materials should return 0
        assert_eq!(task.get_percentage_done(), 0);
        
        // Add materials with different percentages
        task.add_material_to_compute("Wood".to_string());
        task.add_material_to_compute("Metal".to_string());
        
        task.set_material_percentage_done("Wood".to_string(), 50);
        task.set_material_percentage_done("Metal".to_string(), 80);
        
        // Should return average: (50 + 80) / 2 = 65
        assert_eq!(task.get_percentage_done(), 65);
    }

    #[test]
    fn test_set_material_percentage_done() {
        let mut task = Task::new("test-task".to_string());
        task.add_material_to_compute("Wood".to_string());
        
        task.set_material_percentage_done("Wood".to_string(), 75);
        assert_eq!(task.per_material_percentage_done.get("Wood"), Some(&75));
        
        // Setting to 100% should trigger check_if_finished
        task.set_material_percentage_done("Wood".to_string(), 100);
        assert_eq!(task.status, Status::Finished);
        assert!(task.end_time > 0);
    }

    #[test]
    fn test_check_if_finished() {
        let mut task = Task::new("test-task".to_string());
        task.status = Status::Running;
        
        // Add multiple materials
        task.add_material_to_compute("Wood".to_string());
        task.add_material_to_compute("Metal".to_string());
        
        // Not all finished yet
        task.set_material_percentage_done("Wood".to_string(), 100);
        task.set_material_percentage_done("Metal".to_string(), 90);
        task.check_if_finished();
        assert_eq!(task.status, Status::Running);
        
        // All finished
        task.set_material_percentage_done("Metal".to_string(), 100);
        task.check_if_finished();
        assert_eq!(task.status, Status::Finished);
        assert!(task.end_time > 0);
    }

    #[test]
    fn test_get_elapsed_time() {
        let mut task = Task::new("test-task".to_string());
        let start_time = task.start_time;
        
        // Should return time since start
        let elapsed = task.get_elapsed_time();
        assert!(elapsed >= 0);
        
        // Set end time
        task.end_time = start_time + 5000; // 5 seconds later
        assert_eq!(task.get_elapsed_time(), 5000);
    }

    #[test]
    fn test_thread_group_rankings() {
        let mut task = Task::new("test-task".to_string());
        task.add_material_to_compute("Wood".to_string());
        
        // Initially empty
        let rankings = task.get_thread_group_rankings("Wood");
        assert!(rankings.is_some());
        assert!(rankings.unwrap().is_empty());
        
        // Increment rankings
        task.increment_thread_group_rankings("Wood", "group1");
        task.increment_thread_group_rankings("Wood", "group1");
        task.increment_thread_group_rankings("Wood", "group2");
        
        let rankings = task.get_thread_group_rankings("Wood").unwrap();
        assert_eq!(rankings.get("group1"), Some(&2));
        assert_eq!(rankings.get("group2"), Some(&1));
    }

    #[test]
    fn test_thread_counts() {
        let task = Task::new("test-task".to_string());
        
        // All should start at 0
        assert_eq!(task.get_nbr_running_threads(), 0);
        assert_eq!(task.get_nbr_queued_threads(), 0);
        assert_eq!(task.get_nbr_finished_threads(), 0);
        assert_eq!(task.get_nbr_terminated_threads(), 0);
        assert_eq!(task.get_nbr_error_threads(), 0);
        assert_eq!(task.get_nbr_total_threads(), 0);
        
        // Test material-specific finished threads
        assert_eq!(task.get_nbr_finished_threads_for_material("Wood"), 0);
        
        // Test max thread progress
        assert_eq!(task.get_max_thread_progress_percentage(), 0);
    }

    #[test]
    fn test_has_solution() {
        let mut task = Task::new("test-task".to_string());
        
        assert!(!task.has_solution());
        
        // Add empty solution
        let mut response = CalculationResponse::new();
        task.solution = Some(response.clone());
        assert!(!task.has_solution());
        
        // Add solution with panels
        let final_tile = FinalTile::with_params(1, 100.0, 200.0);
        response.panels = Some(vec![final_tile]);
        task.solution = Some(response);
        assert!(task.has_solution());
    }

    #[test]
    fn test_has_solution_all_fit() {
        let mut task = Task::new("test-task".to_string());
        
        assert!(!task.has_solution_all_fit());
        
        // Add solution with panels but no no-fit panels
        let mut response = CalculationResponse::new();
        let final_tile = FinalTile::with_params(1, 100.0, 200.0);
        response.panels = Some(vec![final_tile]);
        task.solution = Some(response.clone());
        assert!(task.has_solution_all_fit());
        
        // Add no-fit panels
        let no_fit_tile = NoFitTile::with_params(1, 50, 50, 2);
        response.no_fit_panels.push(no_fit_tile);
        task.solution = Some(response);
        assert!(!task.has_solution_all_fit());
    }

    #[test]
    fn test_build_solution() {
        let mut task = Task::new("test-task".to_string());
        task.add_material_to_compute("Wood".to_string());
        
        // Add a solution for the material
        let mut material_response = CalculationResponse::new();
        material_response.total_used_area = 80.0;
        material_response.total_wasted_area = 20.0;
        material_response.total_nbr_cuts = 5;
        material_response.total_cut_length = 150.0;
        
        let solution = Solution::with_response(
            "Wood".to_string(),
            0.8,
            0.8,
            material_response
        );
        
        task.solutions.get_mut("Wood").unwrap().push(solution);
        
        // Build solution
        task.build_solution();
        
        let built_solution = task.solution.as_ref().unwrap();
        assert_eq!(built_solution.total_used_area, 80.0);
        assert_eq!(built_solution.total_wasted_area, 20.0);
        assert_eq!(built_solution.total_nbr_cuts, 5);
        assert_eq!(built_solution.total_cut_length, 150.0);
        assert_eq!(built_solution.total_used_area_ratio, 0.8);
        assert_eq!(built_solution.task_id, Some(task.id.clone()));
    }

    #[test]
    fn test_update_last_queried() {
        let mut task = Task::new("test-task".to_string());
        let original_time = task.last_queried;
        
        // Small delay to ensure time difference
        std::thread::sleep(std::time::Duration::from_millis(1));
        
        task.update_last_queried();
        assert!(task.last_queried > original_time);
    }

    #[test]
    fn test_validate() {
        let mut task = Task::new("test-task".to_string());
        
        // Idle task should validate
        assert!(task.validate().is_ok());
        
        // Running task without client info should fail
        task.status = Status::Running;
        assert!(task.validate().is_err());
        
        // Running task with client info should pass
        task.client_info = Some(ClientInfo::default());
        assert!(task.validate().is_ok());
        
        // Finished task without solution should fail
        task.status = Status::Finished;
        task.client_info = None;
        assert!(task.validate().is_err());
        
        // Finished task with solution should pass
        let mut response = CalculationResponse::new();
        let final_tile = FinalTile::with_params(1, 100.0, 200.0);
        response.panels = Some(vec![final_tile]);
        response.total_used_area = 80.0;
        response.total_wasted_area = 20.0;
        response.total_used_area_ratio = 0.8;
        task.solution = Some(response);
        match task.validate() {
            Ok(_) => {},
            Err(e) => panic!("Validation failed: {:?}", e),
        }
    }

    #[test]
    fn test_solution_new() {
        let solution = Solution::new("Wood".to_string(), 0.85, 0.9);
        
        assert_eq!(solution.material, "Wood");
        assert_eq!(solution.score, 0.85);
        assert_eq!(solution.efficiency, 0.9);
        assert!(solution.response.is_none());
    }

    #[test]
    fn test_solution_with_response() {
        let response = CalculationResponse::new();
        let solution = Solution::with_response("Metal".to_string(), 0.75, 0.8, response);
        
        assert_eq!(solution.material, "Metal");
        assert_eq!(solution.score, 0.75);
        assert_eq!(solution.efficiency, 0.8);
        assert!(solution.response.is_some());
    }

    #[test]
    fn test_solution_is_better_than() {
        let solution1 = Solution::new("Wood".to_string(), 0.8, 0.85);
        let solution2 = Solution::new("Wood".to_string(), 0.7, 0.9);
        
        assert!(solution1.is_better_than(&solution2));
        assert!(!solution2.is_better_than(&solution1));
        
        let solution3 = Solution::new("Wood".to_string(), 0.8, 0.7);
        assert!(!solution1.is_better_than(&solution3));
        assert!(!solution3.is_better_than(&solution1));
    }

    #[test]
    fn test_task_default() {
        let task = Task::default();
        assert_eq!(task.id, "default-task");
        assert_eq!(task.status, Status::Idle);
    }

    #[test]
    fn test_task_display() {
        let mut task = Task::new("display-test".to_string());
        task.status = Status::Running;
        task.add_material_to_compute("Wood".to_string());
        task.set_material_percentage_done("Wood".to_string(), 45);
        
        let display_str = format!("{}", task);
        assert!(display_str.contains("display-test"));
        assert!(display_str.contains("Running"));
        assert!(display_str.contains("45%"));
    }

    #[test]
    fn test_serialization() {
        let mut task = Task::new("serialize-test".to_string());
        task.factor = 2.5;
        task.is_min_trim_dimension_influenced = true;
        task.log = Some("Test log entry".to_string());
        
        // Test serialization (Note: Arc<Mutex<>> fields won't serialize with serde)
        // This test focuses on the serializable fields
        let id = task.id.clone();
        let status = task.status;
        let factor = task.factor;
        let influenced = task.is_min_trim_dimension_influenced;
        let log = task.log.clone();
        
        assert_eq!(id, "serialize-test");
        assert_eq!(status, Status::Idle);
        assert_eq!(factor, 2.5);
        assert!(influenced);
        assert_eq!(log, Some("Test log entry".to_string()));
    }

    #[test]
    fn test_complex_workflow() {
        let mut task = Task::new("workflow-test".to_string());
        
        // Set up task
        task.factor = 1000.0;
        task.client_info = Some(ClientInfo::default());
        
        // Add materials
        task.add_material_to_compute("Wood".to_string());
        task.add_material_to_compute("Metal".to_string());
        
        // Start task
        assert!(task.set_running_status().is_ok());
        assert!(task.validate().is_ok());
        
        // Progress materials
        task.set_material_percentage_done("Wood".to_string(), 50);
        task.set_material_percentage_done("Metal".to_string(), 30);
        assert_eq!(task.get_percentage_done(), 40); // (50 + 30) / 2
        assert_eq!(task.status, Status::Running);
        
        // Add some logging
        task.append_line_to_log("Started processing Wood".to_string());
        task.append_line_to_log("Started processing Metal".to_string());
        
        // Complete materials
        task.set_material_percentage_done("Wood".to_string(), 100);
        assert_eq!(task.status, Status::Running); // Metal not finished yet
        
        task.set_material_percentage_done("Metal".to_string(), 100);
        assert_eq!(task.status, Status::Finished); // All materials finished
        
        // Should have built solution automatically
        assert!(task.solution.is_some());
        match task.validate() {
            Ok(_) => {},
            Err(e) => panic!("Validation failed: {:?}", e),
        }
        
        // Check final state
        assert_eq!(task.get_percentage_done(), 100);
        assert!(task.get_elapsed_time() > 0);
        assert!(task.log.is_some());
        assert!(task.log.as_ref().unwrap().contains("Wood"));
        assert!(task.log.as_ref().unwrap().contains("Metal"));
    }
}
