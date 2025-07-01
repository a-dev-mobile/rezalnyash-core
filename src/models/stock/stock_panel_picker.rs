//! Stock panel picker for cutting optimization
//!
//! This module provides the StockPanelPicker which manages the generation
//! and retrieval of stock solutions in a multi-threaded environment.

use crate::models::{TileDimensions, Task, stock::{StockSolution, StockSolutionGenerator}};
use crate::errors::{Result, StockError};
use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};
use std::time::Duration;

/// Minimum number of initial stock solutions to generate
const MIN_INIT_STOCK_SOLUTIONS_TO_GENERATE: usize = 10;

/// Minimum number of stock solutions to generate when all-fit solution exists
const MIN_STOCK_SOLUTIONS_TO_GENERATE_WITH_ALL_FIT_SOLUTION: usize = 100;

/// Stock panel picker for managing stock solution generation
///
/// The picker manages a background thread that generates stock solutions
/// and provides thread-safe access to retrieve solutions by index.
pub struct StockPanelPicker {
    /// Stock solution generator
    stock_solution_generator: StockSolutionGenerator,
    
    /// Reference to the task for status checking
    task: Arc<Mutex<Task>>,
    
    /// Generated stock solutions
    stock_solutions: Arc<Mutex<Vec<StockSolution>>>,
    
    /// Maximum retrieved index for tracking demand
    max_retrieved_idx: Arc<Mutex<usize>>,
    
    /// Handle to the background generation thread
    generation_thread: Option<JoinHandle<()>>,
    
    /// Channel for stopping the generation thread
    stop_sender: Option<mpsc::Sender<()>>,
}

impl StockPanelPicker {
    /// Creates a new StockPanelPicker with max length hint
    ///
    /// # Arguments
    /// * `tiles_to_fit` - Vector of tiles that need to be cut
    /// * `stock_tiles` - Vector of available stock tiles
    /// * `task` - Reference to the task for status checking
    /// * `max_length_hint` - Optional hint for maximum solution length
    ///
    /// # Returns
    /// Result containing the new picker or an error
    ///
    /// # Examples
    /// ```
    /// use std::sync::{Arc, Mutex};
    /// use rezalnyas_core::models::{TileDimensions, Task, stock::StockPanelPicker};
    ///
    /// let tiles_to_fit = vec![TileDimensions::simple(100, 200)];
    /// let stock_tiles = vec![TileDimensions::simple(300, 400)];
    /// let task = Arc::new(Mutex::new(Task::new("test".to_string())));
    /// let picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, Some(50)).unwrap();
    /// ```
    pub fn new(
        tiles_to_fit: Vec<TileDimensions>,
        stock_tiles: Vec<TileDimensions>,
        task: Arc<Mutex<Task>>,
        max_length_hint: Option<usize>,
    ) -> Result<Self> {
        // Create the stock solution generator
        let stock_solution_generator = StockSolutionGenerator::new(
            tiles_to_fit,
            stock_tiles,
            max_length_hint,
        )?;
        
        Ok(Self {
            stock_solution_generator,
            task,
            stock_solutions: Arc::new(Mutex::new(Vec::new())),
            max_retrieved_idx: Arc::new(Mutex::new(0)),
            generation_thread: None,
            stop_sender: None,
        })
    }

    /// Creates a new StockPanelPicker without max length hint
    ///
    /// # Arguments
    /// * `tiles_to_fit` - Vector of tiles that need to be cut
    /// * `stock_tiles` - Vector of available stock tiles
    /// * `task` - Reference to the task for status checking
    ///
    /// # Returns
    /// Result containing the new picker or an error
    pub fn new_without_hint(
        tiles_to_fit: Vec<TileDimensions>,
        stock_tiles: Vec<TileDimensions>,
        task: Arc<Mutex<Task>>,
    ) -> Result<Self> {
        Self::new(tiles_to_fit, stock_tiles, task, None)
    }

    /// Gets the required area from the generator
    ///
    /// # Returns
    /// Total required area as u64
    pub fn get_required_area(&self) -> u64 {
        self.stock_solution_generator.get_required_area()
    }

    /// Initializes the background stock solution generation thread
    ///
    /// # Returns
    /// Result indicating success or failure
    ///
    /// # Examples
    /// ```
    /// use std::sync::{Arc, Mutex};
    /// use rezalnyas_core::models::{TileDimensions, Task, stock::StockPanelPicker};
    ///
    /// let tiles_to_fit = vec![TileDimensions::simple(100, 200)];
    /// let stock_tiles = vec![TileDimensions::simple(300, 400)];
    /// let task = Arc::new(Mutex::new(Task::new("test".to_string())));
    /// let mut picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None).unwrap();
    /// picker.init().unwrap();
    /// ```
    pub fn init(&mut self) -> Result<()> {
        if self.generation_thread.is_some() {
            return Ok(()); // Already initialized
        }

        let (stop_tx, stop_rx) = mpsc::channel();
        self.stop_sender = Some(stop_tx);

        // We need to move the generator to the thread, so we'll replace it with a dummy
        // and move the real one to the thread
        let tiles_to_fit = vec![TileDimensions::simple(1, 1)]; // Dummy data
        let stock_tiles = vec![TileDimensions::simple(1, 1)]; // Dummy data
        let dummy_generator = StockSolutionGenerator::new_without_hint(tiles_to_fit, stock_tiles)?;
        
        // Replace our generator with the dummy and move the real one to the thread
        let generator = std::mem::replace(&mut self.stock_solution_generator, dummy_generator);
        
        let solutions = Arc::clone(&self.stock_solutions);
        let max_retrieved = Arc::clone(&self.max_retrieved_idx);
        let task = Arc::clone(&self.task);

        let handle = thread::spawn(move || {
            Self::generation_thread_worker(generator, solutions, max_retrieved, task, stop_rx);
        });

        self.generation_thread = Some(handle);
        Ok(())
    }

    /// Gets a stock solution by index
    ///
    /// This method will wait for the solution to be generated if it's not ready yet.
    ///
    /// # Arguments
    /// * `index` - The index of the solution to retrieve
    ///
    /// # Returns
    /// Result containing the stock solution or an error
    ///
    /// # Examples
    /// ```
    /// use std::sync::{Arc, Mutex};
    /// use rezalnyas_core::models::{TileDimensions, Task, stock::StockPanelPicker};
    ///
    /// let tiles_to_fit = vec![TileDimensions::simple(100, 200)];
    /// let stock_tiles = vec![TileDimensions::simple(300, 400)];
    /// let task = Arc::new(Mutex::new(Task::new("test".to_string())));
    /// let mut picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None).unwrap();
    /// picker.init().unwrap();
    /// 
    /// // This will wait for the first solution to be generated
    /// if let Ok(Some(solution)) = picker.get_stock_solution(0) {
    ///     println!("Got solution with {} tiles", solution.len());
    /// }
    /// ```
    pub fn get_stock_solution(&self, index: usize) -> Result<Option<StockSolution>> {
        if self.generation_thread.is_none() {
            return Err(StockError::StockPanelPickerNotInitialized.into());
        }

        // Wait for solution to be available
        let max_wait_iterations = 60; // 60 seconds max wait
        let mut wait_count = 0;

        loop {
            // Check if solution is available
            {
                let solutions = self.stock_solutions.lock().unwrap();
                if solutions.len() > index {
                    // Update max retrieved index
                    {
                        let mut max_idx = self.max_retrieved_idx.lock().unwrap();
                        *max_idx = (*max_idx).max(index);
                    }
                    return Ok(Some(solutions[index].clone()));
                }
            }

            // Check if generation thread is still alive
            if let Some(ref handle) = self.generation_thread {
                if handle.is_finished() {
                    // Thread finished, no more solutions will be generated
                    let solutions = self.stock_solutions.lock().unwrap();
                    if solutions.len() <= index {
                        return Ok(None); // No more solutions available
                    }
                }
            }

            // Check if task is still running
            {
                let task = self.task.lock().unwrap();
                if !task.is_running() {
                    return Err(StockError::StockGenerationInterrupted {
                        message: "Task is no longer running".to_string(),
                    }.into());
                }
            }

            wait_count += 1;
            if wait_count >= max_wait_iterations {
                return Err(StockError::StockGenerationInterrupted {
                    message: "Timeout waiting for stock solution generation".to_string(),
                }.into());
            }

            // Wait a bit before checking again
            thread::sleep(Duration::from_millis(1000));
        }
    }

    /// Stops the background generation thread
    pub fn stop(&mut self) {
        if let Some(sender) = self.stop_sender.take() {
            let _ = sender.send(());
        }

        if let Some(handle) = self.generation_thread.take() {
            let _ = handle.join();
        }
    }

    /// Gets the current number of generated solutions
    ///
    /// # Returns
    /// Number of solutions currently available
    pub fn get_solution_count(&self) -> usize {
        self.stock_solutions.lock().unwrap().len()
    }

    /// Checks if the generation thread is still active
    ///
    /// # Returns
    /// `true` if the thread is still generating solutions
    pub fn is_generating(&self) -> bool {
        if let Some(ref handle) = self.generation_thread {
            !handle.is_finished()
        } else {
            false
        }
    }

    /// Worker function for the background generation thread
    fn generation_thread_worker(
        mut generator: StockSolutionGenerator,
        solutions: Arc<Mutex<Vec<StockSolution>>>,
        max_retrieved: Arc<Mutex<usize>>,
        task: Arc<Mutex<Task>>,
        stop_receiver: mpsc::Receiver<()>,
    ) {
        let mut last_solution: Option<StockSolution> = None;

        loop {
            // Check for stop signal
            if stop_receiver.try_recv().is_ok() {
                break;
            }

            // Check if task is still running
            let task_running = {
                let task_guard = task.lock().unwrap();
                task_guard.is_running()
            };

            if !task_running {
                break;
            }

            // Check if we need to generate more solutions
            let should_generate = {
                let solutions_guard = solutions.lock().unwrap();
                let max_retrieved_guard = max_retrieved.lock().unwrap();
                
                *max_retrieved_guard >= solutions_guard.len().saturating_sub(1) ||
                solutions_guard.len() <= MIN_INIT_STOCK_SOLUTIONS_TO_GENERATE
            };

            if should_generate {
                // Generate next solution
                if let Some(new_solution) = generator.generate_stock_solution() {
                    {
                        let mut solutions_guard = solutions.lock().unwrap();
                        solutions_guard.push(new_solution.clone());

                        // If solution doesn't have unique panel size, add sorted variant
                        if !new_solution.has_unique_panel_size() {
                            let mut sorted_solution = new_solution.clone();
                            sorted_solution.sort_panels_desc();
                            solutions_guard.push(sorted_solution);
                        }
                    }
                    last_solution = Some(new_solution);
                } else {
                    // No more solutions available
                    break;
                }
            } else {
                // No need to generate, sleep a bit
                thread::sleep(Duration::from_millis(1000));
            }

            // Check termination conditions
            let should_terminate = {
                let solutions_guard = solutions.lock().unwrap();
                let task_guard = task.lock().unwrap();
                
                last_solution.is_none() ||
                !task_guard.is_running() ||
                (task_guard.has_solution_all_fit() && 
                 solutions_guard.len() >= MIN_STOCK_SOLUTIONS_TO_GENERATE_WITH_ALL_FIT_SOLUTION)
            };

            if should_terminate {
                break;
            }
        }
    }

    /// Sorts the current stock solutions by total area
    fn sort_stock_solutions(&self) {
        let mut solutions = self.stock_solutions.lock().unwrap();
        solutions.sort_by(|a, b| a.get_total_area().cmp(&b.get_total_area()));
    }
}

impl Drop for StockPanelPicker {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    fn create_test_task() -> Arc<Mutex<Task>> {
        let mut task = Task::new("test-task".to_string());
        task.set_running_status().unwrap();
        Arc::new(Mutex::new(task))
    }

    fn create_test_tiles() -> Vec<TileDimensions> {
        vec![
            TileDimensions::simple(100, 200),
            TileDimensions::simple(150, 250),
        ]
    }

    fn create_test_stock() -> Vec<TileDimensions> {
        vec![
            TileDimensions::simple(300, 400),
            TileDimensions::simple(350, 450),
            TileDimensions::simple(400, 500),
        ]
    }

    #[test]
    fn test_new_picker() {
        let tiles_to_fit = create_test_tiles();
        let stock_tiles = create_test_stock();
        let task = create_test_task();
        
        let picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, Some(50));
        assert!(picker.is_ok());
    }

    #[test]
    fn test_new_picker_without_hint() {
        let tiles_to_fit = create_test_tiles();
        let stock_tiles = create_test_stock();
        let task = create_test_task();
        
        let picker = StockPanelPicker::new_without_hint(tiles_to_fit, stock_tiles, task);
        assert!(picker.is_ok());
    }

    #[test]
    fn test_get_required_area() {
        let tiles_to_fit = vec![
            TileDimensions::simple(100, 200), // area: 20000
            TileDimensions::simple(150, 250), // area: 37500
        ];
        let stock_tiles = create_test_stock();
        let task = create_test_task();
        
        let picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None).unwrap();
        assert_eq!(picker.get_required_area(), 57500);
    }

    #[test]
    fn test_init() {
        let tiles_to_fit = create_test_tiles();
        let stock_tiles = create_test_stock();
        let task = create_test_task();
        
        let mut picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None).unwrap();
        let result = picker.init();
        assert!(result.is_ok());
        assert!(picker.is_generating());
    }

    #[test]
    fn test_get_stock_solution_not_initialized() {
        let tiles_to_fit = create_test_tiles();
        let stock_tiles = create_test_stock();
        let task = create_test_task();
        
        let picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None).unwrap();
        let result = picker.get_stock_solution(0);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_stock_solution() {
        let tiles_to_fit = vec![TileDimensions::simple(100, 200)];
        let stock_tiles = vec![TileDimensions::simple(300, 400)];
        let task = create_test_task();
        
        let mut picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None).unwrap();
        picker.init().unwrap();
        
        // Give some time for generation
        thread::sleep(Duration::from_millis(100));
        
        let result = picker.get_stock_solution(0);
        assert!(result.is_ok());
        
        if let Ok(Some(solution)) = result {
            assert!(!solution.is_empty());
        }
    }

    #[test]
    fn test_solution_count() {
        let tiles_to_fit = create_test_tiles();
        let stock_tiles = create_test_stock();
        let task = create_test_task();
        
        let mut picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None).unwrap();
        assert_eq!(picker.get_solution_count(), 0);
        
        picker.init().unwrap();
        
        // Give some time for generation
        thread::sleep(Duration::from_millis(100));
        
        // Should have generated at least one solution
        assert!(picker.get_solution_count() > 0);
    }

    #[test]
    fn test_stop() {
        let tiles_to_fit = create_test_tiles();
        let stock_tiles = create_test_stock();
        let task = create_test_task();
        
        let mut picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None).unwrap();
        picker.init().unwrap();
        
        assert!(picker.is_generating());
        
        picker.stop();
        
        // Give some time for thread to stop
        thread::sleep(Duration::from_millis(100));
        
        assert!(!picker.is_generating());
    }

    #[test]
    fn test_task_not_running() {
        let tiles_to_fit = create_test_tiles();
        let stock_tiles = create_test_stock();
        let task = Arc::new(Mutex::new(Task::new("test-task".to_string()))); // Not running
        
        let mut picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None).unwrap();
        picker.init().unwrap();
        
        let result = picker.get_stock_solution(0);
        assert!(result.is_err());
    }

    #[test]
    fn test_drop_stops_thread() {
        let tiles_to_fit = create_test_tiles();
        let stock_tiles = create_test_stock();
        let task = create_test_task();
        
        {
            let mut picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None).unwrap();
            picker.init().unwrap();
            assert!(picker.is_generating());
        } // picker is dropped here
        
        // Thread should be stopped after drop
        thread::sleep(Duration::from_millis(100));
    }

    #[test]
    fn test_sort_stock_solutions() {
        let tiles_to_fit = create_test_tiles();
        let stock_tiles = create_test_stock();
        let task = create_test_task();
        
        let mut picker = StockPanelPicker::new(tiles_to_fit, stock_tiles, task, None).unwrap();
        picker.init().unwrap();
        
        // Give some time for generation
        thread::sleep(Duration::from_millis(200));
        
        picker.sort_stock_solutions();
        
        // Verify solutions are sorted by area
        let solutions = picker.stock_solutions.lock().unwrap();
        if solutions.len() > 1 {
            for i in 1..solutions.len() {
                assert!(solutions[i-1].get_total_area() <= solutions[i].get_total_area());
            }
        }
    }
}
