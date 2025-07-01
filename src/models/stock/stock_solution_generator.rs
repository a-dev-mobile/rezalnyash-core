//! Stock solution generator for cutting optimization
//!
//! This module provides the StockSolutionGenerator which generates optimal
//! combinations of stock tiles to fulfill cutting requirements.

use crate::models::{TileDimensions, stock::StockSolution};
use crate::errors::{Result, StockError};
use std::collections::HashSet;

/// Maximum default length for stock solutions
const DEFAULT_MAX_STOCK_SOLUTION_LENGTH: usize = 1000;

/// Minimum number of stock solutions to generate initially
const MIN_INIT_STOCK_SOLUTIONS_TO_GENERATE: usize = 10;

/// Generates stock solutions for cutting optimization
///
/// The generator creates combinations of stock tiles that can fulfill
/// the required tile cutting needs, optimizing for area usage and
/// dimensional constraints.
#[derive(Debug)]
pub struct StockSolutionGenerator {
    /// Tiles that need to be cut from stock
    tiles_to_fit: Vec<TileDimensions>,
    
    /// Available stock tiles
    stock_tiles: Vec<TileDimensions>,
    
    /// Maximum length hint for stock solutions
    max_stock_solution_length_hint: Option<usize>,
    
    /// Previously returned stock tile indexes for iteration
    previous_returned_stock_tiles_indexes: Vec<usize>,
    
    /// Previous index to iterate from
    prev_index_to_iterate: usize,
    
    /// Stock solutions to exclude from generation
    stock_solutions_to_exclude: Vec<StockSolution>,
    
    /// Total required area for all tiles to fit
    required_area: u64,
    
    /// Maximum dimension required
    required_max_dimension: u32,
    
    /// Smallest tile area
    smallest_tile_area: u64,
    
    /// Pre-computed solution with all panels
    all_panel_stock_solution: Option<StockSolution>,
}

impl StockSolutionGenerator {
    /// Generates multiple stock solutions
    ///
    /// # Arguments
    /// * `max_length_hint` - Maximum length hint for solutions
    /// * `excluded_solutions` - Solutions to exclude from generation
    ///
    /// # Returns
    /// Result containing vector of generated solutions
    pub fn generate_multiple_solutions(
        &mut self,
        max_length_hint: usize,
        excluded_solutions: &[StockSolution],
    ) -> Result<Vec<StockSolution>> {
        let mut solutions = Vec::new();
        
        // Add excluded solutions to our internal exclusion list
        for excluded in excluded_solutions {
            if !self.stock_solutions_to_exclude.contains(excluded) {
                self.stock_solutions_to_exclude.push(excluded.clone());
            }
        }
        
        // Temporarily set the max length hint
        let original_hint = self.max_stock_solution_length_hint;
        self.max_stock_solution_length_hint = Some(max_length_hint);
        
        // Generate solutions until we can't generate more
        while let Some(solution) = self.generate_stock_solution() {
            solutions.push(solution);
            
            // Limit the number of solutions to prevent infinite loops
            if solutions.len() >= 10 {
                break;
            }
        }
        
        // Restore original hint
        self.max_stock_solution_length_hint = original_hint;
        
        Ok(solutions)
    }

    /// Creates a new StockSolutionGenerator
    ///
    /// # Arguments
    /// * `tiles_to_fit` - Vector of tiles that need to be cut
    /// * `stock_tiles` - Vector of available stock tiles
    /// * `max_length_hint` - Optional hint for maximum solution length
    ///
    /// # Returns
    /// Result containing the new generator or an error
    ///
    /// # Examples
    /// ```
    /// use rezalnyas_core::models::{TileDimensions, stock::StockSolutionGenerator};
    ///
    /// let tiles_to_fit = vec![TileDimensions::simple(100, 200)];
    /// let stock_tiles = vec![TileDimensions::simple(300, 400)];
    /// let generator = StockSolutionGenerator::new(tiles_to_fit, stock_tiles, Some(50)).unwrap();
    /// ```
    pub fn new(
        tiles_to_fit: Vec<TileDimensions>,
        stock_tiles: Vec<TileDimensions>,
        max_length_hint: Option<usize>,
    ) -> Result<Self> {
        if tiles_to_fit.is_empty() {
            return Err(StockError::StockNoTilesToFit.into());
        }
        
        if stock_tiles.is_empty() {
            return Err(StockError::StockNoStockTiles.into());
        }

        let mut generator = Self {
            tiles_to_fit,
            stock_tiles,
            max_stock_solution_length_hint: max_length_hint,
            previous_returned_stock_tiles_indexes: Vec::new(),
            prev_index_to_iterate: 0,
            stock_solutions_to_exclude: Vec::new(),
            required_area: 0,
            required_max_dimension: 0,
            smallest_tile_area: u64::MAX,
            all_panel_stock_solution: None,
        };

        generator.sort_stock_tiles_area_asc();
        generator.calc_required_area();
        generator.all_panel_stock_solution = Some(generator.gen_all_panel_stock_solution());

        Ok(generator)
    }

    /// Creates a new StockSolutionGenerator without max length hint
    ///
    /// # Arguments
    /// * `tiles_to_fit` - Vector of tiles that need to be cut
    /// * `stock_tiles` - Vector of available stock tiles
    ///
    /// # Returns
    /// Result containing the new generator or an error
    pub fn new_without_hint(
        tiles_to_fit: Vec<TileDimensions>,
        stock_tiles: Vec<TileDimensions>,
    ) -> Result<Self> {
        Self::new(tiles_to_fit, stock_tiles, None)
    }

    /// Gets the total required area for all tiles to fit
    ///
    /// # Returns
    /// Total required area as u64
    pub fn get_required_area(&self) -> u64 {
        self.required_area
    }

    /// Generates the next stock solution
    ///
    /// # Returns
    /// Option containing the next stock solution, or None if no more solutions
    ///
    /// # Examples
    /// ```
    /// use rezalnyas_core::models::{TileDimensions, stock::StockSolutionGenerator};
    ///
    /// let tiles_to_fit = vec![TileDimensions::simple(100, 200)];
    /// let stock_tiles = vec![TileDimensions::simple(300, 400)];
    /// let mut generator = StockSolutionGenerator::new(tiles_to_fit, stock_tiles, None).unwrap();
    /// 
    /// if let Some(solution) = generator.generate_stock_solution() {
    ///     println!("Generated solution with {} tiles", solution.len());
    /// }
    /// ```
    pub fn generate_stock_solution(&mut self) -> Option<StockSolution> {
        let min_panels_needed = self.calculate_min_panels_needed();
        let max_length = self.get_effective_max_length();

        // If all stock panels are unique, return the all-panel solution only if it fits max length
        if self.is_unique_stock_panel() {
            if let Some(ref all_panel_solution) = self.all_panel_stock_solution {
                if !self.is_excluded(all_panel_solution) && all_panel_solution.len() <= max_length {
                    self.stock_solutions_to_exclude.push(all_panel_solution.clone());
                    return Some(all_panel_solution.clone());
                }
            }
        }

        // If max length is at default and we haven't excluded all-panel solution, return it
        if max_length == DEFAULT_MAX_STOCK_SOLUTION_LENGTH {
            if let Some(ref all_panel_solution) = self.all_panel_stock_solution {
                if !self.is_excluded(all_panel_solution) {
                    self.stock_solutions_to_exclude.push(all_panel_solution.clone());
                    return Some(all_panel_solution.clone());
                }
            }
        }

        // Try to find solutions with increasing panel counts (Java: while loop)
        // Note: Java uses < stockTiles.size() and <= maxLength
        let mut panel_count = min_panels_needed;
        while panel_count < self.stock_tiles.len() && panel_count <= max_length {
            if let Some(solution) = self.get_candidate_stock_solution(panel_count) {
                self.stock_solutions_to_exclude.push(solution.clone());
                let mut sorted_solution = solution;
                sorted_solution.sort_panels_asc();
                return Some(sorted_solution);
            }
            panel_count += 1;
        }

        None
    }

    /// Calculates the required area and dimensions
    fn calc_required_area(&mut self) {
        self.required_area = 0;
        self.required_max_dimension = 0;
        self.smallest_tile_area = u64::MAX;

        for tile in &self.tiles_to_fit {
            self.required_area += tile.area();
            self.required_max_dimension = self.required_max_dimension.max(tile.max_dimension());
            self.smallest_tile_area = self.smallest_tile_area.min(tile.area());
        }
    }

    /// Sorts stock tiles by area in ascending order
    fn sort_stock_tiles_area_asc(&mut self) {
        self.stock_tiles.sort_by(|a, b| a.area().cmp(&b.area()));
    }

    /// Checks if a solution is in the exclusion list
    fn is_excluded(&self, solution: &StockSolution) -> bool {
        self.stock_solutions_to_exclude.iter().any(|excluded| excluded == solution)
    }

    /// Checks if a list of tile indexes represents an excluded solution
    fn is_excluded_by_indexes(&self, indexes: &[usize]) -> bool {
        if self.stock_solutions_to_exclude.is_empty() {
            return false;
        }

        let tiles: Vec<TileDimensions> = indexes
            .iter()
            .map(|&i| self.stock_tiles[i].clone())
            .collect();
        let solution = StockSolution::new(tiles);
        self.is_excluded(&solution)
    }

    /// Checks if all stock panels have the same ID (unique panel type)
    fn is_unique_stock_panel(&self) -> bool {
        if self.stock_tiles.is_empty() {
            return true;
        }

        let first_id = self.stock_tiles[0].id();
        self.stock_tiles.iter().all(|tile| tile.id() == first_id)
    }

    /// Gets the area of the biggest stock tile
    fn get_biggest_stock_tile_area(&self) -> u64 {
        self.stock_tiles
            .iter()
            .map(|tile| tile.area())
            .max()
            .unwrap_or(0)
    }

    /// Calculates the minimum number of panels needed
    fn calculate_min_panels_needed(&self) -> usize {
        let biggest_area = self.get_biggest_stock_tile_area();
        if biggest_area == 0 {
            return 1;
        }
        ((self.required_area as f64) / (biggest_area as f64)).ceil() as usize
    }

    /// Gets the effective maximum length for solutions
    fn get_effective_max_length(&self) -> usize {
        let min_needed = self.calculate_min_panels_needed();
        
        if let Some(hint) = self.max_stock_solution_length_hint {
            if hint >= min_needed {
                return hint;
            }
        }
        
        DEFAULT_MAX_STOCK_SOLUTION_LENGTH
    }

    /// Generates a solution containing all available panels
    fn gen_all_panel_stock_solution(&self) -> StockSolution {
        let mut solution = StockSolution::empty();
        let max_panels = DEFAULT_MAX_STOCK_SOLUTION_LENGTH.min(self.stock_tiles.len());
        
        // Add panels from largest to smallest (reverse order since sorted ascending)
        for i in 0..max_panels {
            let index = self.stock_tiles.len() - 1 - i;
            solution.add_stock_tile(self.stock_tiles[index].clone());
        }
        
        solution.sort_panels_asc();
        solution
    }

    /// Gets a candidate stock solution with the specified number of panels
    fn get_candidate_stock_solution(&mut self, panel_count: usize) -> Option<StockSolution> {
        let mut indexes = if self.previous_returned_stock_tiles_indexes.len() == panel_count {
            self.previous_returned_stock_tiles_indexes.clone()
        } else {
            (0..panel_count).collect()
        };

        let start_index = if self.previous_returned_stock_tiles_indexes.len() == panel_count {
            self.prev_index_to_iterate
        } else {
            0
        };

        self.iterate_solution(panel_count, &mut indexes, start_index)
    }

    /// Iterates through possible solutions following the Java algorithm exactly
    fn iterate_solution(
        &mut self,
        panel_count: usize,
        indexes: &mut Vec<usize>,
        iteration_index: usize,
    ) -> Option<StockSolution> {
        // Validate uniqueness for the current iteration level (like Java HashSet check)
        let mut seen = HashSet::new();
        for i in 0..iteration_index {
            if !seen.insert(indexes[i]) {
                return None;
            }
        }

        // If not at the last position, recurse (like Java)
        if iteration_index < panel_count - 1 {
            let mut current_height = 0;
            let mut current_width = 0;
            let mut i = 0;

            while i < self.stock_tiles.len() {
                let i6 = current_height;
                let i7 = current_width;
                let i8 = i;
                
                // Try recursion with current state
                if let Some(solution) = self.iterate_solution(panel_count, indexes, iteration_index + 1) {
                    return Some(solution);
                }

                i = i8;
                
                // Find next tile with different dimensions or sufficient area
                loop {
                    i += 1;
                    if i >= self.stock_tiles.len() {
                        break;
                    }
                    if self.stock_tiles[i].width() != i7 || 
                       self.stock_tiles[i].height() != i6 {
                        if self.stock_tiles[i].area() >= self.smallest_tile_area {
                            break;
                        }
                    }
                }

                if i < self.stock_tiles.len() {
                    current_width = self.stock_tiles[i].width();
                    current_height = self.stock_tiles[i].height();
                    
                    // Update indexes from current iteration level onwards (like Java)
                    let mut i9 = iteration_index;
                    let mut i5 = i;
                    while i9 < indexes.len() && i5 < self.stock_tiles.len() {
                        indexes[i9] = i5;
                        i9 += 1;
                        i5 += 1;
                    }
                } else {
                    current_width = i7;
                    current_height = i6;
                }
            }
        }

        // Main solution checking loop (like Java do-while)
        let mut next_unused_tile: Option<usize>;
        loop {
            // Calculate area like in Java: area -= tile.area() for each tile
            let mut area = self.required_area as i64;
            let mut has_required_dimension = false;
            
            for &index in indexes.iter() {
                let tile = &self.stock_tiles[index];
                area -= tile.area() as i64;
                if tile.max_dimension() >= self.required_max_dimension {
                    has_required_dimension = true;
                }
            }

            // Check if solution meets requirements (area <= 0 means we have enough area)
            if area <= 0 && 
               has_required_dimension && 
               self.is_valid_index_combination(indexes) && 
               !self.is_excluded_by_indexes(indexes) {
                
                let tiles: Vec<TileDimensions> = indexes.iter()
                    .map(|&i| self.stock_tiles[i].clone())
                    .collect();
                
                let solution = StockSolution::new(tiles);
                
                // Update state for next iteration (like Java)
                self.previous_returned_stock_tiles_indexes.clear();
                self.previous_returned_stock_tiles_indexes.extend_from_slice(indexes);
                self.prev_index_to_iterate = iteration_index;
                
                return Some(solution);
            }

            // Try next combination at current iteration level (like Java do-while condition)
            next_unused_tile = self.get_next_unused_stock_tile(indexes, iteration_index);
            if let Some(next_index) = next_unused_tile {
                indexes[iteration_index] = next_index;
            } else {
                break;
            }
        }

        None
    }
    /// Gets the next unused stock tile index following Java algorithm
    fn get_next_unused_stock_tile(&self, indexes: &[usize], position: usize) -> Option<usize> {
        let current_tile = &self.stock_tiles[indexes[position]];
        let mut i = indexes[position];
        
        loop {
            i += 1;
            if i >= self.stock_tiles.len() {
                return None;
            }
            if !indexes.contains(&i) {
                let candidate_tile = &self.stock_tiles[i];
                if candidate_tile.width() > current_tile.width() || 
                   candidate_tile.height() > current_tile.height() {
                    return Some(i);
                }
            }
        }
    }

    /// Validates that index combination has no duplicates
    fn is_valid_index_combination(&self, indexes: &[usize]) -> bool {
        let mut seen = HashSet::new();
        indexes.iter().all(|&index| seen.insert(index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_new_generator() {
        let tiles_to_fit = create_test_tiles();
        let stock_tiles = create_test_stock();
        
        let generator = StockSolutionGenerator::new(tiles_to_fit, stock_tiles, Some(50));
        assert!(generator.is_ok());
    }

    #[test]
    fn test_new_generator_empty_tiles() {
        let tiles_to_fit = vec![];
        let stock_tiles = create_test_stock();
        
        let result = StockSolutionGenerator::new(tiles_to_fit, stock_tiles, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_new_generator_empty_stock() {
        let tiles_to_fit = create_test_tiles();
        let stock_tiles = vec![];
        
        let result = StockSolutionGenerator::new(tiles_to_fit, stock_tiles, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_new_without_hint() {
        let tiles_to_fit = create_test_tiles();
        let stock_tiles = create_test_stock();
        
        let generator = StockSolutionGenerator::new_without_hint(tiles_to_fit, stock_tiles);
        assert!(generator.is_ok());
    }

    #[test]
    fn test_get_required_area() {
        let tiles_to_fit = vec![
            TileDimensions::simple(100, 200), // area: 20000
            TileDimensions::simple(150, 250), // area: 37500
        ];
        let stock_tiles = create_test_stock();
        
        let generator = StockSolutionGenerator::new(tiles_to_fit, stock_tiles, None).unwrap();
        assert_eq!(generator.get_required_area(), 57500);
    }

    #[test]
    fn test_generate_stock_solution() {
        let tiles_to_fit = vec![TileDimensions::simple(100, 200)];
        let stock_tiles = vec![TileDimensions::simple(300, 400)];
        
        let mut generator = StockSolutionGenerator::new(tiles_to_fit, stock_tiles, None).unwrap();
        let solution = generator.generate_stock_solution();
        
        assert!(solution.is_some());
        let solution = solution.unwrap();
        assert!(!solution.is_empty());
    }

    #[test]
    fn test_generate_multiple_solutions() {
        let tiles_to_fit = vec![TileDimensions::simple(100, 200)];
        let stock_tiles = vec![
            TileDimensions::simple(300, 400),
            TileDimensions::simple(350, 450),
            TileDimensions::simple(400, 500),
        ];
        
        let mut generator = StockSolutionGenerator::new(tiles_to_fit, stock_tiles, Some(2)).unwrap();
        
        let solution1 = generator.generate_stock_solution();
        println!("Solution1: {:?}", solution1);
        assert!(solution1.is_some());
        
        let solution2 = generator.generate_stock_solution();
        println!("Solution2: {:?}", solution2);
        println!("Previous indexes: {:?}", generator.previous_returned_stock_tiles_indexes);
        println!("Prev index to iterate: {}", generator.prev_index_to_iterate);
        println!("Excluded solutions: {}", generator.stock_solutions_to_exclude.len());
        assert!(solution2.is_some());
        
        // Solutions should be different
        assert_ne!(solution1.unwrap(), solution2.unwrap());
    }

    #[test]
    fn test_unique_stock_panel() {
        let tiles_to_fit = vec![TileDimensions::simple(100, 200)];
        let stock_tiles = vec![
            TileDimensions::new_with_defaults(1, 300, 400, "Wood".to_string(), 0, None),
            TileDimensions::new_with_defaults(1, 300, 400, "Wood".to_string(), 0, None),
        ];
        
        let mut generator = StockSolutionGenerator::new(tiles_to_fit, stock_tiles, None).unwrap();
        let solution = generator.generate_stock_solution();
        
        assert!(solution.is_some());
    }

    #[test]
    fn test_solution_exclusion() {
        let tiles_to_fit = vec![TileDimensions::simple(100, 200)];
        let stock_tiles = vec![TileDimensions::simple(300, 400)];
        
        let mut generator = StockSolutionGenerator::new(tiles_to_fit, stock_tiles, None).unwrap();
        
        // Generate first solution
        let solution1 = generator.generate_stock_solution();
        assert!(solution1.is_some());
        
        // Try to generate second solution - should be None since only one stock tile
        let solution2 = generator.generate_stock_solution();
        assert!(solution2.is_none());
    }

    #[test]
    fn test_area_calculation() {
        let tiles_to_fit = vec![
            TileDimensions::simple(100, 200), // area: 20000
            TileDimensions::simple(50, 100),  // area: 5000
        ];
        let stock_tiles = create_test_stock();
        
        let generator = StockSolutionGenerator::new(tiles_to_fit, stock_tiles, None).unwrap();
        assert_eq!(generator.required_area, 25000);
        assert_eq!(generator.smallest_tile_area, 5000);
        assert_eq!(generator.required_max_dimension, 200);
    }

    #[test]
    fn test_stock_tiles_sorting() {
        let tiles_to_fit = vec![TileDimensions::simple(100, 200)];
        let stock_tiles = vec![
            TileDimensions::simple(400, 500), // area: 200000
            TileDimensions::simple(300, 400), // area: 120000
            TileDimensions::simple(350, 450), // area: 157500
        ];
        
        let generator = StockSolutionGenerator::new(tiles_to_fit, stock_tiles, None).unwrap();
        
        // Should be sorted by area ascending
        let areas: Vec<u64> = generator.stock_tiles.iter().map(|t| t.area()).collect();
        assert_eq!(areas, vec![120000, 157500, 200000]);
    }

    #[test]
    fn test_min_panels_calculation() {
        let tiles_to_fit = vec![TileDimensions::simple(100, 200)]; // area: 20000
        let stock_tiles = vec![TileDimensions::simple(150, 200)];  // area: 30000
        
        let generator = StockSolutionGenerator::new(tiles_to_fit, stock_tiles, None).unwrap();
        let min_panels = generator.calculate_min_panels_needed();
        
        // 20000 / 30000 = 0.67, ceil = 1
        assert_eq!(min_panels, 1);
    }

    #[test]
    fn test_all_panel_solution_generation() {
        let tiles_to_fit = vec![TileDimensions::simple(100, 200)];
        let stock_tiles = vec![
            TileDimensions::simple(300, 400),
            TileDimensions::simple(350, 450),
        ];
        
        let generator = StockSolutionGenerator::new(tiles_to_fit, stock_tiles, None).unwrap();
        let all_panel_solution = generator.gen_all_panel_stock_solution();
        
        assert_eq!(all_panel_solution.len(), 2);
        
        // Should be sorted by area ascending
        let tiles = all_panel_solution.get_stock_tile_dimensions();
        assert!(tiles[0].area() <= tiles[1].area());
    }
}
