use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::{log_debug, models::tile_dimensions::TileDimensions, models::task::Task};

/// Represents a solution using specific stock panels
#[derive(Debug, Clone)]
pub struct StockSolution {
    pub stock_tile_dimensions: Vec<TileDimensions>,
    pub total_area: u64,
}

impl StockSolution {
    pub fn new(stock_tiles: Vec<TileDimensions>) -> Self {
        let total_area = stock_tiles.iter().map(|tile| tile.width * tile.height).sum();
        Self {
            stock_tile_dimensions: stock_tiles,
            total_area,
        }
    }

    pub fn get_total_area(&self) -> u64 {
        self.total_area
    }

    pub fn get_stock_tile_dimensions(&self) -> &[TileDimensions] {
        &self.stock_tile_dimensions
    }

    /// Check if all panels have unique sizes
    pub fn has_unique_panel_size(&self) -> bool {
        for i in 0..self.stock_tile_dimensions.len() {
            for j in i + 1..self.stock_tile_dimensions.len() {
                let tile1 = &self.stock_tile_dimensions[i];
                let tile2 = &self.stock_tile_dimensions[j];
                if tile1.width == tile2.width && tile1.height == tile2.height {
                    return false;
                }
            }
        }
        true
    }

    /// Sort panels in descending order by area
    pub fn sort_panels_desc(&mut self) {
        self.stock_tile_dimensions.sort_by(|a, b| {
            let area_a = a.width * a.height;
            let area_b = b.width * b.height;
            area_b.cmp(&area_a)
        });
    }
}

impl std::fmt::Display for StockSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let panels_info: String = self.stock_tile_dimensions
            .iter()
            .map(|tile| format!("{}x{}", tile.width, tile.height))
            .collect::<Vec<_>>()
            .join(",");
        write!(f, "{}", panels_info)
    }
}

/// Generates stock solutions by combining available stock panels
pub struct StockSolutionGenerator {
    tiles: Vec<TileDimensions>,
    stock_tiles: Vec<TileDimensions>,
    max_stock_units: Option<usize>,
    required_area: u64,
    current_combination: Vec<usize>,
    finished: bool,
}

impl StockSolutionGenerator {
    pub fn new(
        tiles: Vec<TileDimensions>,
        stock_tiles: Vec<TileDimensions>,
        max_stock_units: Option<usize>,
    ) -> Self {
        let required_area = tiles.iter().map(|tile| tile.width * tile.height).sum();
        let stock_tiles_len = stock_tiles.len();
        
        Self {
            tiles,
            stock_tiles,
            max_stock_units,
            required_area,
            current_combination: vec![0; stock_tiles_len],
            finished: false,
        }
    }

    pub fn get_required_area(&self) -> u64 {
        self.required_area
    }

    /// Generate next stock solution
    pub fn generate_stock_solution(&mut self) -> Option<StockSolution> {
        if self.finished {
            return None;
        }

        loop {
            // Generate current solution
            let mut stock_solution_tiles = Vec::new();
            let mut total_area = 0u64;
            
            for (i, &count) in self.current_combination.iter().enumerate() {
                for _ in 0..count {
                    stock_solution_tiles.push(self.stock_tiles[i].clone());
                    total_area += self.stock_tiles[i].width * self.stock_tiles[i].height;
                }
            }

            // Check if this combination provides enough area
            let has_enough_area = total_area >= self.required_area;
            let is_valid_combination = !stock_solution_tiles.is_empty();

            // Advance to next combination
            self.advance_combination();

            if has_enough_area && is_valid_combination {
                return Some(StockSolution::new(stock_solution_tiles));
            }

            if self.finished {
                break;
            }
        }

        None
    }

    /// Advance to the next combination (like counting in mixed radix)
    fn advance_combination(&mut self) {
        let max_units_per_type = if let Some(max) = self.max_stock_units {
            max
        } else {
            // Calculate reasonable maximum based on required area
            let min_stock_area = self.stock_tiles
                .iter()
                .map(|tile| tile.width * tile.height)
                .min()
                .unwrap_or(1);
            
            ((self.required_area / min_stock_area) + 1) as usize
        };

        let mut carry = 1;
        for i in 0..self.current_combination.len() {
            self.current_combination[i] += carry;
            if self.current_combination[i] <= max_units_per_type {
                carry = 0;
                break;
            } else {
                self.current_combination[i] = 0;
                carry = 1;
            }
        }

        if carry == 1 {
            self.finished = true;
        }
    }
}

/// Main stock panel picker that manages stock solution generation in a separate thread
pub struct StockPanelPicker {
    stock_solutions: Arc<Mutex<Vec<StockSolution>>>,
    max_retrieved_idx: Arc<Mutex<usize>>,
    generator_thread: Option<thread::JoinHandle<()>>,
    task: Arc<Mutex<Task>>,
}

impl StockPanelPicker {
    const MIN_INIT_STOCK_SOLUTIONS_TO_GENERATE: usize = 10;
    const MIN_STOCK_SOLUTIONS_TO_GENERATE_WITH_ALL_FIT_SOLUTION: usize = 100;

    pub fn new(
        tiles: Vec<TileDimensions>,
        stock_tiles: Vec<TileDimensions>,
        task: Task,
        max_stock_units: Option<usize>,
    ) -> Self {
        Self {
            stock_solutions: Arc::new(Mutex::new(Vec::new())),
            max_retrieved_idx: Arc::new(Mutex::new(0)),
            generator_thread: None,
            task: Arc::new(Mutex::new(task)),
        }
    }

    /// Initialize the stock panel picker and start generation thread
    pub fn init(&mut self, tiles: Vec<TileDimensions>, stock_tiles: Vec<TileDimensions>, max_stock_units: Option<usize>) {
        let stock_solutions = Arc::clone(&self.stock_solutions);
        let max_retrieved_idx = Arc::clone(&self.max_retrieved_idx);
        let task = Arc::clone(&self.task);
        
        let handle = thread::spawn(move || {
            let mut generator = StockSolutionGenerator::new(tiles, stock_tiles, max_stock_units);
            let mut last_generated_solution: Option<StockSolution> = None;

            loop {
                // Check if we need to generate more solutions
                let should_generate = {
                    let solutions = stock_solutions.lock().unwrap();
                    let max_idx = *max_retrieved_idx.lock().unwrap();
                    
                    max_idx >= solutions.len().saturating_sub(1) || 
                    solutions.len() <= Self::MIN_INIT_STOCK_SOLUTIONS_TO_GENERATE
                };

                if should_generate {
                    if let Some(mut stock_solution) = generator.generate_stock_solution() {
                        let mut solutions = stock_solutions.lock().unwrap();
                        
                        log_debug!(
                            "Added idx[{}] [{}] area[{}][{}] to stack",
                            solutions.len(),
                            stock_solution.stock_tile_dimensions.len(),
                            stock_solution.total_area,
                            stock_solution
                        );

                        solutions.push(stock_solution.clone());

                        // If panels are not unique, add a sorted version
                        if !stock_solution.has_unique_panel_size() {
                            stock_solution.sort_panels_desc();
                            solutions.push(stock_solution.clone());
                        }

                        last_generated_solution = Some(stock_solution);
                    } else {
                        last_generated_solution = None;
                    }
                } else {
                    let (max_idx, solutions_len) = {
                        let solutions = stock_solutions.lock().unwrap();
                        let max_idx = *max_retrieved_idx.lock().unwrap();
                        (max_idx, solutions.len())
                    };
                    
                    log_debug!(
                        "No need to generate new candidate stock solution: maxRetrievedIdx[{}] stockSolutions[{}]",
                        max_idx,
                        solutions_len
                    );
                }

                // Sleep if we have enough solutions
                if stock_solutions.lock().unwrap().len() > Self::MIN_INIT_STOCK_SOLUTIONS_TO_GENERATE {
                    thread::sleep(Duration::from_millis(1000));
                }

                // Check exit conditions
                let should_exit = {
                    let task_guard = task.lock().unwrap();
                    let solutions_count = stock_solutions.lock().unwrap().len();
                    
                    last_generated_solution.is_none() ||
                    !task_guard.is_running() ||
                    (task_guard.has_solution_all_fit() && solutions_count >= Self::MIN_STOCK_SOLUTIONS_TO_GENERATE_WITH_ALL_FIT_SOLUTION)
                };

                if should_exit {
                    break;
                }
            }

            // Log exit reason
            let solutions_count = stock_solutions.lock().unwrap().len();
            if last_generated_solution.is_none() {
                log_debug!(
                    "Finishing stock picker thread: nbrGeneratedStockSolutions[{}] - There are no more available stock solutions",
                    solutions_count
                );
            } else if !task.lock().unwrap().is_running() {
                log_debug!(
                    "Finishing stock picker thread: nbrGeneratedStockSolutions[{}] - Task has no longer running status",
                    solutions_count
                );
            } else if task.lock().unwrap().has_solution_all_fit() {
                log_debug!(
                    "Finishing stock picker thread: nbrGeneratedStockSolutions[{}] - Task has already an all fit solution",
                    solutions_count
                );
            }
        });

        self.generator_thread = Some(handle);
    }

    /// Get stock solution by index, waiting if necessary
    pub fn get_stock_solution(&self, index: usize) -> Option<StockSolution> {
        if self.generator_thread.is_none() {
            panic!("StockPanelPickerThread not initialized");
        }

        // Wait for solution to be available
        loop {
            {
                let solutions = self.stock_solutions.lock().unwrap();
                if solutions.len() > index {
                    // Update max retrieved index
                    let mut max_idx = self.max_retrieved_idx.lock().unwrap();
                    *max_idx = (*max_idx).max(index);
                    
                    return Some(solutions[index].clone());
                }
            }

            // Check if generator thread is still alive
            if let Some(ref handle) = self.generator_thread {
                if handle.is_finished() {
                    log_debug!("No more possible stock solutions");
                    return None;
                }
            }

            log_debug!("Waiting for stock solution generation: idx[{}]", index);
            thread::sleep(Duration::from_millis(1000));
        }
    }

    /// Get the required area from generator
    pub fn get_required_area(&self) -> u64 {
        // This would need to be stored when creating the generator
        // For now, return 0 as placeholder
        0
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::orientation::Orientation;

    fn create_test_tile(id: u8, width: u64, height: u64) -> TileDimensions {
        TileDimensions {
            id,
            width,
            height,
            orientation: Orientation::Default,
            is_rotated: false,
        }
    }

    #[test]
    fn test_stock_solution_creation() {
        let stock_tiles = vec![
            create_test_tile(1, 400, 300),
            create_test_tile(2, 200, 150),
        ];

        let solution = StockSolution::new(stock_tiles.clone());
        assert_eq!(solution.get_total_area(), 400 * 300 + 200 * 150);
        assert_eq!(solution.get_stock_tile_dimensions().len(), 2);
    }

    #[test]
    fn test_stock_solution_unique_panels() {
        let stock_tiles_unique = vec![
            create_test_tile(1, 400, 300),
            create_test_tile(2, 200, 150),
        ];

        let stock_tiles_duplicate = vec![
            create_test_tile(1, 400, 300),
            create_test_tile(2, 400, 300),
        ];

        let solution_unique = StockSolution::new(stock_tiles_unique);
        let solution_duplicate = StockSolution::new(stock_tiles_duplicate);

        assert!(solution_unique.has_unique_panel_size());
        assert!(!solution_duplicate.has_unique_panel_size());
    }

    #[test]
    fn test_stock_solution_generator() {
        let tiles = vec![
            create_test_tile(1, 100, 50),
            create_test_tile(2, 150, 75),
        ];

        let stock_tiles = vec![
            create_test_tile(10, 200, 100),
        ];

        let mut generator = StockSolutionGenerator::new(tiles, stock_tiles, Some(3));
        
        let solution1 = generator.generate_stock_solution();
        assert!(solution1.is_some());
        
        let solution2 = generator.generate_stock_solution();
        assert!(solution2.is_some());
        
        // Solutions should be different
        if let (Some(s1), Some(s2)) = (&solution1, &solution2) {
            assert_ne!(s1.stock_tile_dimensions.len(), s2.stock_tile_dimensions.len());
        }
    }
}
