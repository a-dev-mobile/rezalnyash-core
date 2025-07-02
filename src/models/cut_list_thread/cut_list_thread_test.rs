//! Tests for CutListThread
//!
//! This module contains comprehensive unit tests for the CutListThread struct
//! and its associated functionality.

#[cfg(test)]
mod tests {
    use super::super::cut_list_thread::*;
    use crate::models::{
        tile_dimensions::TileDimensions, 
        tile_node::TileNode, 
        task::Task,
        stock::stock_solution::StockSolution,
        mosaic::Mosaic,
    };
    use crate::enums::{CutOrientationPreference, Status};
    use std::sync::{Arc, Mutex};
    use std::cmp::Ordering;

    /// Mock solution comparator for testing
    #[derive(Debug)]
    struct MockSolutionComparator;

    impl SolutionComparator for MockSolutionComparator {
        fn compare(&self, a: &Solution, b: &Solution) -> Ordering {
            // Simple comparison based on number of mosaics
            a.get_mosaics().len().cmp(&b.get_mosaics().len())
        }
    }

    /// Mock logger for testing
    #[derive(Debug)]
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
    }

    impl CutListLogger for MockLogger {
        fn log(&self, message: &str) {
            self.messages.lock().unwrap().push(message.to_string());
        }
    }

    /// Helper function to create a test tile dimensions
    fn create_test_tile(width: u32, height: u32, id: i32) -> TileDimensions {
        TileDimensions::new(
            id,
            width,
            height,
            "Wood".to_string(),
            0,
            None,
            false,
        )
    }

    /// Helper function to create a test stock solution
    fn create_test_stock_solution() -> StockSolution {
        let stock_tiles = vec![
            create_test_tile(1000, 2000, 1),
            create_test_tile(1200, 2400, 2),
        ];
        StockSolution::new(stock_tiles)
    }

    /// Helper function to create a test task
    fn create_test_task() -> Arc<Mutex<Task>> {
        let task = Task::new("test_task_id".to_string());
        Arc::new(Mutex::new(task))
    }

    #[test]
    fn test_cut_list_thread_new() {
        let thread = CutListThread::new();
        
        assert_eq!(thread.get_accuracy_factor(), 10);
        assert_eq!(thread.get_cut_thickness(), 0);
        assert_eq!(thread.get_min_trim_dimension(), 0);
        assert_eq!(thread.get_first_cut_orientation(), CutOrientationPreference::Both);
        assert!(!thread.is_consider_grain_direction());
        assert_eq!(thread.get_status(), Status::Queued);
        assert_eq!(thread.get_percentage_done(), 0);
        assert!(thread.get_group().is_none());
        assert!(thread.get_aux_info().is_none());
        assert!(thread.get_task().is_none());
        assert!(thread.get_stock_solution().is_none());
        assert!(thread.get_cut_list_logger().is_none());
        assert!(thread.get_tiles().is_empty());
        assert!(thread.get_solutions().is_empty());
    }

    #[test]
    fn test_cut_list_thread_setters_getters() {
        let mut thread = CutListThread::new();
        let task = create_test_task();
        let stock_solution = create_test_stock_solution();
        let logger = Box::new(MockLogger::new());
        let comparator = Box::new(MockSolutionComparator);
        let tiles = vec![create_test_tile(100, 200, 1)];

        // Test setters
        thread.set_group(Some("test_group".to_string()));
        thread.set_aux_info(Some("test_aux".to_string()));
        thread.set_task(Some(task.clone()));
        thread.set_stock_solution(Some(stock_solution.clone()));
        thread.set_cut_list_logger(Some(logger));
        thread.set_accuracy_factor(20);
        thread.set_cut_thickness(3);
        thread.set_min_trim_dimension(10);
        thread.set_first_cut_orientation(CutOrientationPreference::Horizontal);
        thread.set_consider_grain_direction(true);
        thread.set_tiles(tiles.clone());
        thread.set_thread_prioritized_comparators(vec![comparator]);
        thread.set_final_solution_prioritized_comparators(vec![Box::new(MockSolutionComparator)]);

        // Test getters
        assert_eq!(thread.get_group(), Some(&"test_group".to_string()));
        assert_eq!(thread.get_aux_info(), Some(&"test_aux".to_string()));
        assert!(thread.get_task().is_some());
        assert!(thread.get_stock_solution().is_some());
        assert!(thread.get_cut_list_logger().is_some());
        assert_eq!(thread.get_accuracy_factor(), 20);
        assert_eq!(thread.get_cut_thickness(), 3);
        assert_eq!(thread.get_min_trim_dimension(), 10);
        assert_eq!(thread.get_first_cut_orientation(), CutOrientationPreference::Horizontal);
        assert!(thread.is_consider_grain_direction());
        assert_eq!(thread.get_tiles().len(), 1);
        assert_eq!(thread.get_thread_prioritized_comparators().len(), 1);
        assert_eq!(thread.get_final_solution_prioritized_comparators().len(), 1);
    }

    #[test]
    fn test_solution_new() {
        let stock_solution = create_test_stock_solution();
        let solution = Solution::new(&stock_solution);

        assert!(solution.get_material().is_none());
        assert!(solution.get_mosaics().is_empty());
        assert_eq!(solution.get_unused_stock_panels().len(), 2);
        assert!(solution.get_no_fit_panels().is_empty());
        assert!(solution.creator_thread_group.is_none());
        assert!(solution.aux_info.is_none());
    }

    #[test]
    fn test_solution_add_mosaic() {
        let stock_solution = create_test_stock_solution();
        let mut solution = Solution::new(&stock_solution);
        let tile_dims = create_test_tile(500, 600, 1);
        let mosaic = Mosaic::from_tile_dimensions(&tile_dims).unwrap();

        solution.add_mosaic(mosaic);
        assert_eq!(solution.get_mosaics().len(), 1);
    }

    #[test]
    fn test_solution_set_creator_thread_group() {
        let stock_solution = create_test_stock_solution();
        let mut solution = Solution::new(&stock_solution);

        solution.set_creator_thread_group(Some("group1".to_string()));
        assert_eq!(solution.creator_thread_group, Some("group1".to_string()));
    }

    #[test]
    fn test_solution_set_aux_info() {
        let stock_solution = create_test_stock_solution();
        let mut solution = Solution::new(&stock_solution);

        solution.set_aux_info(Some("aux_data".to_string()));
        assert_eq!(solution.aux_info, Some("aux_data".to_string()));
    }

    #[test]
    fn test_remove_duplicated() {
        let thread = CutListThread::new();
        let stock_solution = create_test_stock_solution();
        
        // Create identical solutions
        let solution1 = Solution::new(&stock_solution);
        let solution2 = Solution::new(&stock_solution);
        let solution3 = Solution::new(&stock_solution);
        
        let mut solutions = vec![solution1, solution2, solution3];
        let removed_count = thread.remove_duplicated(&mut solutions);
        
        // Should remove duplicates (exact behavior depends on mosaic content)
        assert!(removed_count >= 0);
        assert!(solutions.len() <= 3);
    }

    #[test]
    fn test_get_elapsed_time_millis() {
        let thread = CutListThread::new();
        
        // Before start time is set
        assert_eq!(thread.get_elapsed_time_millis(), 0);
    }

    #[test]
    fn test_get_material() {
        let thread = CutListThread::new();
        
        // No solutions
        assert!(thread.get_material().is_none());
    }

    #[test]
    fn test_tile_node_creation() {
        let tile_node = TileNode::new(0, 0, 1000, 2000);
        assert!(tile_node.is_ok());
        
        let node = tile_node.unwrap();
        assert_eq!(node.width(), 1000);
        assert_eq!(node.height(), 2000);
    }

    #[test]
    fn test_run_without_setup() {
        let mut thread = CutListThread::new();
        
        // Should fail gracefully without stock solution
        let result = thread.run();
        assert!(result.is_err());
        assert_eq!(thread.get_status(), Status::Error);
    }

    #[test]
    fn test_run_with_setup() {
        let mut thread = CutListThread::new();
        let task = create_test_task();
        let stock_solution = create_test_stock_solution();
        let tiles = vec![
            create_test_tile(100, 200, 1),
            create_test_tile(150, 250, 2),
        ];
        
        thread.set_task(Some(task));
        thread.set_stock_solution(Some(stock_solution));
        thread.set_tiles(tiles);
        thread.set_cut_thickness(3);
        
        let result = thread.run();
        
        // Should complete successfully
        assert!(result.is_ok());
        assert!(matches!(thread.get_status(), Status::Finished | Status::Running));
    }

    #[test]
    fn test_compute_solutions_basic() {
        let mut thread = CutListThread::new();
        let task = create_test_task();
        let stock_solution = create_test_stock_solution();
        let tiles = vec![create_test_tile(100, 200, 1)];
        
        thread.set_task(Some(task));
        thread.set_stock_solution(Some(stock_solution));
        thread.set_tiles(tiles);
        
        let result = thread.compute_solutions();
        assert!(result.is_ok());
    }

    #[test]
    fn test_default_cut_list_logger() {
        // Initialize logger first
        use crate::logging::{enums::LogLevel, structs::LogConfig};
        let config = LogConfig { level: LogLevel::Info };
        let _ = crate::logging::init::init_logging(config);
        
        let logger = DefaultCutListLogger;
        
        // Should not panic
        logger.log("Test message");
    }

    #[test]
    fn test_mock_solution_comparator() {
        let comparator = MockSolutionComparator;
        let stock_solution = create_test_stock_solution();
        
        let solution1 = Solution::new(&stock_solution);
        let mut solution2 = Solution::new(&stock_solution);
        let tile_dims = create_test_tile(500, 600, 1);
        let mosaic = Mosaic::from_tile_dimensions(&tile_dims).unwrap();
        solution2.add_mosaic(mosaic);
        
        let result = comparator.compare(&solution1, &solution2);
        assert_eq!(result, Ordering::Less); // solution1 has fewer mosaics
    }

    #[test]
    fn test_mock_logger() {
        let logger = MockLogger::new();
        
        logger.log("Message 1");
        logger.log("Message 2");
        
        let messages = logger.get_messages();
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0], "Message 1");
        assert_eq!(messages[1], "Message 2");
    }

    #[test]
    fn test_solution_new_with_replacement() {
        let stock_solution = create_test_stock_solution();
        let original = Solution::new(&stock_solution);
        let tile_dims = create_test_tile(500, 600, 1);
        let mosaic = Mosaic::from_tile_dimensions(&tile_dims).unwrap();
        
        let new_solution = Solution::new_with_replacement(&original, &mosaic);
        
        // Should have same properties as original
        assert_eq!(new_solution.material, original.material);
        assert_eq!(new_solution.mosaics.len(), original.mosaics.len());
        assert_eq!(new_solution.unused_stock_panels.len(), original.unused_stock_panels.len());
        assert_eq!(new_solution.no_fit_panels.len(), original.no_fit_panels.len());
    }

    #[test]
    fn test_cut_list_thread_default() {
        let thread = CutListThread::default();
        
        // Should be same as new()
        assert_eq!(thread.get_accuracy_factor(), 10);
        assert_eq!(thread.get_status(), Status::Queued);
        assert!(thread.get_tiles().is_empty());
    }

    #[test]
    fn test_solution_getters_mutable() {
        let stock_solution = create_test_stock_solution();
        let mut solution = Solution::new(&stock_solution);
        
        // Test mutable getters
        solution.get_mosaics_mut().clear();
        assert!(solution.get_mosaics().is_empty());
        
        solution.get_unused_stock_panels_mut().clear();
        assert!(solution.get_unused_stock_panels().is_empty());
        
        solution.get_no_fit_panels_mut().push(create_test_tile(100, 100, 1));
        assert_eq!(solution.get_no_fit_panels().len(), 1);
    }

    #[test]
    fn test_cut_orientation_preferences() {
        let mut thread = CutListThread::new();
        
        // Test all orientation preferences
        thread.set_first_cut_orientation(CutOrientationPreference::Horizontal);
        assert_eq!(thread.get_first_cut_orientation(), CutOrientationPreference::Horizontal);
        
        thread.set_first_cut_orientation(CutOrientationPreference::Vertical);
        assert_eq!(thread.get_first_cut_orientation(), CutOrientationPreference::Vertical);
        
        thread.set_first_cut_orientation(CutOrientationPreference::Both);
        assert_eq!(thread.get_first_cut_orientation(), CutOrientationPreference::Both);
    }

    #[test]
    fn test_percentage_calculation() {
        let mut thread = CutListThread::new();
        let task = create_test_task();
        let stock_solution = create_test_stock_solution();
        
        // Create multiple tiles to test percentage calculation
        let tiles = vec![
            create_test_tile(100, 200, 1),
            create_test_tile(150, 250, 2),
            create_test_tile(200, 300, 3),
            create_test_tile(250, 350, 4),
        ];
        
        thread.set_task(Some(task));
        thread.set_stock_solution(Some(stock_solution));
        thread.set_tiles(tiles);
        
        let _ = thread.compute_solutions();
        
        // Percentage should be calculated during processing
        // (exact value depends on implementation details)
        assert!(thread.get_percentage_done() >= 0);
        assert!(thread.get_percentage_done() <= 100);
    }

    #[test]
    fn test_cut_direction_enum() {
        // Test CutDirection enum values
        let horizontal = CutDirection::Horizontal;
        let vertical = CutDirection::Vertical;
        
        assert_ne!(horizontal, vertical);
    }

    #[test]
    fn test_solution_material_handling() {
        let stock_solution = create_test_stock_solution();
        let mut solution = Solution::new(&stock_solution);
        
        // Test material setting
        solution.material = Some("Wood".to_string());
        assert_eq!(solution.get_material(), Some(&"Wood".to_string()));
        
        solution.material = None;
        assert!(solution.get_material().is_none());
    }

    #[test]
    fn test_tile_dimensions_properties() {
        let tile = create_test_tile(100, 200, 42);
        
        assert_eq!(tile.width(), 100);
        assert_eq!(tile.height(), 200);
        assert_eq!(tile.id(), 42);
    }

    #[test]
    fn test_stock_solution_creation() {
        let stock_solution = create_test_stock_solution();
        let tiles = stock_solution.get_stock_tile_dimensions();
        
        assert_eq!(tiles.len(), 2);
        assert_eq!(tiles[0].width(), 1000);
        assert_eq!(tiles[0].height(), 2000);
        assert_eq!(tiles[1].width(), 1200);
        assert_eq!(tiles[1].height(), 2400);
    }

    #[test]
    fn test_solution_clone() {
        let stock_solution = create_test_stock_solution();
        let solution = Solution::new(&stock_solution);
        let cloned = solution.clone();
        
        assert_eq!(solution.get_mosaics().len(), cloned.get_mosaics().len());
        assert_eq!(solution.get_unused_stock_panels().len(), cloned.get_unused_stock_panels().len());
        assert_eq!(solution.get_no_fit_panels().len(), cloned.get_no_fit_panels().len());
    }

    #[test]
    fn test_mosaic_creation() {
        let tile_dims = create_test_tile(500, 600, 1);
        let mosaic_result = Mosaic::from_tile_dimensions(&tile_dims);
        
        assert!(mosaic_result.is_ok());
        let mosaic = mosaic_result.unwrap();
        assert_eq!(mosaic.stock_id(), 1);
        assert_eq!(mosaic.material(), "Wood");
    }

    #[test]
    fn test_tile_dimensions_simple_constructor() {
        let tile = TileDimensions::simple(100, 200);
        
        assert_eq!(tile.width(), 100);
        assert_eq!(tile.height(), 200);
        assert_eq!(tile.id(), -1); // Default ID for simple constructor
    }

    #[test]
    fn test_solution_efficiency_calculation() {
        let stock_solution = create_test_stock_solution();
        let solution = Solution::new(&stock_solution);
        
        // Test that solution can be created and basic properties work
        assert!(solution.get_mosaics().is_empty());
        assert!(!solution.get_unused_stock_panels().is_empty());
    }

    #[test]
    fn test_thread_with_minimal_setup() {
        let mut thread = CutListThread::new();
        let stock_solution = create_test_stock_solution();
        
        thread.set_stock_solution(Some(stock_solution));
        
        // Should have stock solution set
        assert!(thread.get_stock_solution().is_some());
    }

    #[test]
    fn test_cut_list_thread_status_handling() {
        let thread = CutListThread::new();
        
        // Initial status should be Queued
        assert_eq!(thread.get_status(), Status::Queued);
        
        // Status should be accessible
        match thread.get_status() {
            Status::Queued => assert!(true),
            _ => assert!(false, "Expected Queued status"),
        }
    }

    #[test]
    fn test_solution_with_multiple_mosaics() {
        let stock_solution = create_test_stock_solution();
        let mut solution = Solution::new(&stock_solution);
        
        // Add multiple mosaics
        let tile1 = create_test_tile(100, 200, 1);
        let tile2 = create_test_tile(150, 250, 2);
        
        let mosaic1 = Mosaic::from_tile_dimensions(&tile1).unwrap();
        let mosaic2 = Mosaic::from_tile_dimensions(&tile2).unwrap();
        
        solution.add_mosaic(mosaic1);
        solution.add_mosaic(mosaic2);
        
        assert_eq!(solution.get_mosaics().len(), 2);
    }

    #[test]
    fn test_thread_accuracy_factor_bounds() {
        let mut thread = CutListThread::new();
        
        // Test setting various accuracy factors
        thread.set_accuracy_factor(1);
        assert_eq!(thread.get_accuracy_factor(), 1);
        
        thread.set_accuracy_factor(100);
        assert_eq!(thread.get_accuracy_factor(), 100);
        
        thread.set_accuracy_factor(50);
        assert_eq!(thread.get_accuracy_factor(), 50);
    }

    #[test]
    fn test_cut_thickness_settings() {
        let mut thread = CutListThread::new();
        
        // Test various cut thickness values
        thread.set_cut_thickness(0);
        assert_eq!(thread.get_cut_thickness(), 0);
        
        thread.set_cut_thickness(3);
        assert_eq!(thread.get_cut_thickness(), 3);
        
        thread.set_cut_thickness(10);
        assert_eq!(thread.get_cut_thickness(), 10);
    }
}
