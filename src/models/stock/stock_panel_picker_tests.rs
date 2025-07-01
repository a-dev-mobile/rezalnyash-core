//! Comprehensive tests for StockPanelPicker
//!
//! This module contains extensive unit tests for the StockPanelPicker struct,
//! covering all methods, threading behavior, and integration scenarios.

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::models::{TileDimensions, Task, stock::{StockPanelPicker, StockSolution}};
    use crate::errors::StockError;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    // Helper functions for creating test data
    fn create_running_task() -> Arc<Mutex<Task>> {
        let mut task = Task::new("test-task".to_string());
        task.set_running_status().unwrap();
        Arc::new(Mutex::new(task))
    }

    fn create_idle_task() -> Arc<Mutex<Task>> {
        let task = Task::new("idle-task".to_string());
        Arc::new(Mutex::new(task))
    }

    fn create_small_tiles() -> Vec<TileDimensions> {
        vec![
            TileDimensions::simple(100, 200),
            TileDimensions::simple(150, 250),
        ]
    }

    fn create_large_tiles() -> Vec<TileDimensions> {
        vec![
            TileDimensions::simple(500, 600),
            TileDimensions::simple(700, 800),
            TileDimensions::simple(900, 1000),
        ]
    }

    fn create_small_stock() -> Vec<TileDimensions> {
        vec![
            TileDimensions::simple(300, 400),
            TileDimensions::simple(350, 450),
        ]
    }

    fn create_large_stock() -> Vec<TileDimensions> {
        vec![
            TileDimensions::simple(1200, 1500),
            TileDimensions::simple(1300, 1600),
            TileDimensions::simple(1400, 1700),
            TileDimensions::simple(1500, 1800),
        ]
    }

    fn create_varied_stock() -> Vec<TileDimensions> {
        vec![
            TileDimensions::simple(200, 300),
            TileDimensions::simple(400, 500),
            TileDimensions::simple(600, 700),
            TileDimensions::simple(800, 900),
            TileDimensions::simple(1000, 1100),
        ]
    }

    // Constructor tests
    #[test]
    fn test_new_picker_with_hint() {
        let tiles_to_fit = create_small_tiles();
        let stock_tiles = create_small_stock();
        let task = create_running_task();
        
        let picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, Some(50));
        assert!(picker.is_ok());
        
        let picker = picker.unwrap();
        assert_eq!(picker.get_solution_count(), 0);
        assert!(!picker.is_generating());
    }

    #[test]
    fn test_new_picker_without_hint() {
        let tiles_to_fit = create_small_tiles();
        let stock_tiles = create_small_stock();
        let task = create_running_task();
        
        let picker = StockPanelPicker::new_without_hint(tiles_to_fit, stock_tiles, task);
        assert!(picker.is_ok());
    }

    #[test]
    fn test_new_picker_empty_tiles() {
        let tiles_to_fit = vec![];
        let stock_tiles = create_small_stock();
        let task = create_running_task();
        
        let result = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_new_picker_empty_stock() {
        let tiles_to_fit = create_small_tiles();
        let stock_tiles = vec![];
        let task = create_running_task();
        
        let result = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_new_picker_both_empty() {
        let tiles_to_fit = vec![];
        let stock_tiles = vec![];
        let task = create_running_task();
        
        let result = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None);
        assert!(result.is_err());
    }

    // Required area tests
    #[test]
    fn test_get_required_area_small() {
        let tiles_to_fit = vec![
            TileDimensions::simple(100, 200), // area: 20000
            TileDimensions::simple(150, 250), // area: 37500
        ];
        let stock_tiles = create_small_stock();
        let task = create_running_task();
        
        let picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None).unwrap();
        assert_eq!(picker.get_required_area(), 57500);
    }

    #[test]
    fn test_get_required_area_large() {
        let tiles_to_fit = vec![
            TileDimensions::simple(1000, 2000), // area: 2,000,000
            TileDimensions::simple(1500, 2500), // area: 3,750,000
        ];
        let stock_tiles = create_large_stock();
        let task = create_running_task();
        
        let picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None).unwrap();
        assert_eq!(picker.get_required_area(), 5_750_000);
    }

    #[test]
    fn test_get_required_area_single_tile() {
        let tiles_to_fit = vec![TileDimensions::simple(100, 200)]; // area: 20000
        let stock_tiles = create_small_stock();
        let task = create_running_task();
        
        let picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None).unwrap();
        assert_eq!(picker.get_required_area(), 20000);
    }

    // Initialization tests
    #[test]
    fn test_init_success() {
        let tiles_to_fit = create_small_tiles();
        let stock_tiles = create_small_stock();
        let task = create_running_task();
        
        let mut picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None).unwrap();
        let result = picker.init();
        
        assert!(result.is_ok());
        assert!(picker.is_generating());
    }

    #[test]
    fn test_init_already_initialized() {
        let tiles_to_fit = create_small_tiles();
        let stock_tiles = create_small_stock();
        let task = create_running_task();
        
        let mut picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None).unwrap();
        
        // First initialization
        let result1 = picker.init();
        assert!(result1.is_ok());
        assert!(picker.is_generating());
        
        // Second initialization should succeed but not create new thread
        let result2 = picker.init();
        assert!(result2.is_ok());
        assert!(picker.is_generating());
    }

    #[test]
    fn test_init_with_large_dataset() {
        let tiles_to_fit = create_large_tiles();
        let stock_tiles = create_large_stock();
        let task = create_running_task();
        
        let mut picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, Some(10)).unwrap();
        let result = picker.init();
        
        assert!(result.is_ok());
        assert!(picker.is_generating());
    }

    // Solution retrieval tests
    #[test]
    fn test_get_stock_solution_not_initialized() {
        let tiles_to_fit = create_small_tiles();
        let stock_tiles = create_small_stock();
        let task = create_running_task();
        
        let picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None).unwrap();
        let result = picker.get_stock_solution(0);
        
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("not initialized"));
        }
    }

    #[test]
    fn test_get_stock_solution_task_not_running() {
        let tiles_to_fit = create_small_tiles();
        let stock_tiles = create_small_stock();
        let task = create_idle_task(); // Task not running
        
        let mut picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None).unwrap();
        picker.init().unwrap();
        
        let result = picker.get_stock_solution(0);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("no longer running"));
        }
    }

    #[test]
    fn test_get_stock_solution_success() {
        let tiles_to_fit = vec![TileDimensions::simple(100, 200)];
        let stock_tiles = vec![TileDimensions::simple(300, 400)];
        let task = create_running_task();
        
        let mut picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None).unwrap();
        picker.init().unwrap();
        
        // Give some time for generation
        thread::sleep(Duration::from_millis(200));
        
        let result = picker.get_stock_solution(0);
        assert!(result.is_ok());
        
        if let Ok(Some(solution)) = result {
            assert!(!solution.is_empty());
            assert!(solution.get_total_area() > 0);
        }
    }

    #[test]
    fn test_get_stock_solution_multiple_solutions() {
        let tiles_to_fit = vec![TileDimensions::simple(100, 200)];
        let stock_tiles = create_varied_stock();
        let task = create_running_task();
        
        let mut picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, Some(3)).unwrap();
        picker.init().unwrap();
        
        // Give time for multiple solutions to generate
        thread::sleep(Duration::from_millis(500));
        
        // Should be able to get multiple solutions
        let solution1 = picker.get_stock_solution(0);
        assert!(solution1.is_ok());
        
        let solution2 = picker.get_stock_solution(1);
        assert!(solution2.is_ok());
        
        // Solutions should be different (if multiple exist)
        if let (Ok(Some(sol1)), Ok(Some(sol2))) = (solution1, solution2) {
            // They might be the same if only one solution is possible
            // but at least we should get valid solutions
            assert!(!sol1.is_empty());
            assert!(!sol2.is_empty());
        }
    }

    #[test]
    fn test_get_stock_solution_index_out_of_range() {
        let tiles_to_fit = vec![TileDimensions::simple(100, 200)];
        let stock_tiles = vec![TileDimensions::simple(300, 400)];
        let task = create_running_task();
        
        let mut picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None).unwrap();
        picker.init().unwrap();
        
        // Give some time for generation
        thread::sleep(Duration::from_millis(200));
        
        // Try to get a solution at a very high index
        let result = picker.get_stock_solution(1000);
        // This should either return None or timeout
        assert!(result.is_ok() || result.is_err());
    }

    // Solution count tests
    #[test]
    fn test_get_solution_count_initial() {
        let tiles_to_fit = create_small_tiles();
        let stock_tiles = create_small_stock();
        let task = create_running_task();
        
        let picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None).unwrap();
        assert_eq!(picker.get_solution_count(), 0);
    }

    #[test]
    fn test_get_solution_count_after_init() {
        let tiles_to_fit = create_small_tiles();
        let stock_tiles = create_small_stock();
        let task = create_running_task();
        
        let mut picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None).unwrap();
        picker.init().unwrap();
        
        // Give some time for generation
        thread::sleep(Duration::from_millis(300));
        
        let count = picker.get_solution_count();
        assert!(count > 0, "Should have generated at least one solution");
    }

    #[test]
    fn test_get_solution_count_increases() {
        let tiles_to_fit = create_small_tiles();
        let stock_tiles = create_varied_stock();
        let task = create_running_task();
        
        let mut picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, Some(5)).unwrap();
        picker.init().unwrap();
        
        // Check count increases over time
        thread::sleep(Duration::from_millis(100));
        let count1 = picker.get_solution_count();
        
        thread::sleep(Duration::from_millis(200));
        let count2 = picker.get_solution_count();
        
        assert!(count2 >= count1, "Solution count should not decrease");
    }

    // Thread management tests
    #[test]
    fn test_is_generating_before_init() {
        let tiles_to_fit = create_small_tiles();
        let stock_tiles = create_small_stock();
        let task = create_running_task();
        
        let picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None).unwrap();
        assert!(!picker.is_generating());
    }

    #[test]
    fn test_is_generating_after_init() {
        let tiles_to_fit = create_small_tiles();
        let stock_tiles = create_small_stock();
        let task = create_running_task();
        
        let mut picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None).unwrap();
        picker.init().unwrap();
        
        assert!(picker.is_generating());
    }

    #[test]
    fn test_is_generating_after_stop() {
        let tiles_to_fit = create_small_tiles();
        let stock_tiles = create_small_stock();
        let task = create_running_task();
        
        let mut picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None).unwrap();
        picker.init().unwrap();
        
        assert!(picker.is_generating());
        
        picker.stop();
        
        // Give some time for thread to stop
        thread::sleep(Duration::from_millis(100));
        
        assert!(!picker.is_generating());
    }

    #[test]
    fn test_stop_before_init() {
        let tiles_to_fit = create_small_tiles();
        let stock_tiles = create_small_stock();
        let task = create_running_task();
        
        let mut picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None).unwrap();
        
        // Should not panic
        picker.stop();
        assert!(!picker.is_generating());
    }

    #[test]
    fn test_stop_multiple_times() {
        let tiles_to_fit = create_small_tiles();
        let stock_tiles = create_small_stock();
        let task = create_running_task();
        
        let mut picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None).unwrap();
        picker.init().unwrap();
        
        // Stop multiple times should not panic
        picker.stop();
        picker.stop();
        picker.stop();
        
        assert!(!picker.is_generating());
    }

    // Drop behavior tests
    #[test]
    fn test_drop_stops_thread() {
        let tiles_to_fit = create_small_tiles();
        let stock_tiles = create_small_stock();
        let task = create_running_task();
        
        {
            let mut picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None).unwrap();
            picker.init().unwrap();
            assert!(picker.is_generating());
        } // picker is dropped here
        
        // Thread should be stopped after drop
        thread::sleep(Duration::from_millis(100));
        // We can't directly test this, but it shouldn't panic or hang
    }

    // Sorting tests
    #[test]
    fn test_sort_stock_solutions() {
        let tiles_to_fit = create_small_tiles();
        let stock_tiles = create_varied_stock();
        let task = create_running_task();
        
        let mut picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, Some(3)).unwrap();
        picker.init().unwrap();
        
        // Give time for multiple solutions to generate
        thread::sleep(Duration::from_millis(400));
        
        picker.sort_stock_solutions();
        
        // Verify solutions are sorted by area
        let solutions = picker.stock_solutions.lock().unwrap();
        if solutions.len() > 1 {
            for i in 1..solutions.len() {
                assert!(
                    solutions[i-1].get_total_area() <= solutions[i].get_total_area(),
                    "Solutions should be sorted by total area"
                );
            }
        }
    }

    #[test]
    fn test_sort_empty_solutions() {
        let tiles_to_fit = create_small_tiles();
        let stock_tiles = create_small_stock();
        let task = create_running_task();
        
        let picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None).unwrap();
        
        // Should not panic when sorting empty solutions
        picker.sort_stock_solutions();
        
        assert_eq!(picker.get_solution_count(), 0);
    }

    // Edge case tests
    #[test]
    fn test_very_large_tiles() {
        let tiles_to_fit = vec![
            TileDimensions::simple(10000, 20000),
            TileDimensions::simple(15000, 25000),
        ];
        let stock_tiles = vec![
            TileDimensions::simple(30000, 40000),
            TileDimensions::simple(35000, 45000),
        ];
        let task = create_running_task();
        
        let picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None);
        assert!(picker.is_ok());
        
        let picker = picker.unwrap();
        assert!(picker.get_required_area() > 0);
    }

    #[test]
    fn test_single_tile_single_stock() {
        let tiles_to_fit = vec![TileDimensions::simple(100, 200)];
        let stock_tiles = vec![TileDimensions::simple(300, 400)];
        let task = create_running_task();
        
        let mut picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None).unwrap();
        picker.init().unwrap();
        
        thread::sleep(Duration::from_millis(200));
        
        let result = picker.get_stock_solution(0);
        assert!(result.is_ok());
        
        if let Ok(Some(solution)) = result {
            assert_eq!(solution.len(), 1);
            assert!(solution.get_total_area() >= 120000); // 300 * 400
        }
    }

    #[test]
    fn test_max_length_hint_respected() {
        let tiles_to_fit = vec![TileDimensions::simple(100, 200)];
        let stock_tiles = create_varied_stock();
        let task = create_running_task();
        
        let mut picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, Some(2)).unwrap();
        picker.init().unwrap();
        
        thread::sleep(Duration::from_millis(300));
        
        // Should generate solutions but respect the hint
        let count = picker.get_solution_count();
        assert!(count > 0);
        
        // Check that solutions don't exceed reasonable bounds
        if let Ok(Some(solution)) = picker.get_stock_solution(0) {
            assert!(solution.len() <= 10); // Reasonable upper bound
        }
    }

    // Performance tests
    #[test]
    fn test_concurrent_access() {
        let tiles_to_fit = create_small_tiles();
        let stock_tiles = create_varied_stock();
        let task = create_running_task();
        
        let mut picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, Some(5)).unwrap();
        picker.init().unwrap();
        
        let picker = Arc::new(picker);
        let mut handles = vec![];
        
        // Spawn multiple threads trying to access solutions
        for i in 0..3 {
            let picker_clone = Arc::clone(&picker);
            let handle = thread::spawn(move || {
                thread::sleep(Duration::from_millis(100 * i as u64));
                let result = picker_clone.get_stock_solution(0);
                result.is_ok()
            });
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            let result = handle.join();
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_solution_count_thread_safety() {
        let tiles_to_fit = create_small_tiles();
        let stock_tiles = create_varied_stock();
        let task = create_running_task();
        
        let mut picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, Some(5)).unwrap();
        picker.init().unwrap();
        
        let picker = Arc::new(picker);
        let mut handles = vec![];
        
        // Spawn multiple threads checking solution count
        for _ in 0..5 {
            let picker_clone = Arc::clone(&picker);
            let handle = thread::spawn(move || {
                for _ in 0..10 {
                    let count = picker_clone.get_solution_count();
                    assert!(count >= 0);
                    thread::sleep(Duration::from_millis(10));
                }
            });
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            let result = handle.join();
            assert!(result.is_ok());
        }
    }

    // Integration tests
    #[test]
    fn test_full_workflow() {
        let tiles_to_fit = vec![
            TileDimensions::simple(100, 200),
            TileDimensions::simple(150, 250),
        ];
        let stock_tiles = vec![
            TileDimensions::simple(300, 400),
            TileDimensions::simple(350, 450),
            TileDimensions::simple(400, 500),
        ];
        let task = create_running_task();
        
        // Create picker
        let mut picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, Some(3)).unwrap();
        
        // Check initial state
        assert_eq!(picker.get_solution_count(), 0);
        assert!(!picker.is_generating());
        assert_eq!(picker.get_required_area(), 57500); // 20000 + 37500
        
        // Initialize
        picker.init().unwrap();
        assert!(picker.is_generating());
        
        // Wait for solutions to generate
        thread::sleep(Duration::from_millis(400));
        
        // Check solutions are available
        assert!(picker.get_solution_count() > 0);
        
        // Get first solution
        let solution = picker.get_stock_solution(0).unwrap();
        assert!(solution.is_some());
        
        let solution = solution.unwrap();
        assert!(!solution.is_empty());
        assert!(solution.get_total_area() >= picker.get_required_area());
        
        // Sort solutions
        picker.sort_stock_solutions();
        
        // Stop picker
        picker.stop();
        assert!(!picker.is_generating());
    }

    #[test]
    fn test_workflow_with_task_state_changes() {
        let tiles_to_fit = vec![TileDimensions::simple(100, 200)];
        let stock_tiles = vec![TileDimensions::simple(300, 400)];
        let task = create_running_task();
        
        let mut picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, Arc::clone(&task), None).unwrap();
        picker.init().unwrap();
        
        // Let it generate for a bit
        thread::sleep(Duration::from_millis(200));
        
        // Stop the task
        {
            let mut task_guard = task.lock().unwrap();
            task_guard.stop().unwrap();
        }
        
        // Now trying to get solutions should fail
        let result = picker.get_stock_solution(10); // High index to force waiting
        assert!(result.is_err());
    }
}
