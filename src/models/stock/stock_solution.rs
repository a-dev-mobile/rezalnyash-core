//! Stock solution model for cutting optimization
//!
//! This module provides the StockSolution struct which represents a collection
//! of stock tiles that can be used to fulfill cutting requirements.

use crate::models::TileDimensions;
use std::collections::HashMap;
use std::fmt;

/// Represents a solution consisting of stock tiles for cutting optimization
///
/// A stock solution contains a collection of stock tiles that can be used
/// to cut the required tiles. It provides functionality for sorting, comparison,
/// and area calculations.
#[derive(Debug, Clone)]
pub struct StockSolution {
    /// Collection of stock tile dimensions
    stock_tile_dimensions: Vec<TileDimensions>,
}

impl StockSolution {
    /// Creates a new StockSolution from a vector of tile dimensions
    ///
    /// # Arguments
    /// * `stock_tiles` - Vector of stock tile dimensions
    ///
    /// # Examples
    /// ```
    /// use rezalnyas_core::models::{TileDimensions, stock::StockSolution};
    ///
    /// let tiles = vec![
    ///     TileDimensions::simple(100, 200),
    ///     TileDimensions::simple(150, 300),
    /// ];
    /// let solution = StockSolution::new(tiles);
    /// assert_eq!(solution.get_stock_tile_dimensions().len(), 2);
    /// ```
    pub fn new(stock_tiles: Vec<TileDimensions>) -> Self {
        Self {
            stock_tile_dimensions: stock_tiles,
        }
    }

    /// Creates a new StockSolution from individual tile dimensions
    ///
    /// # Arguments
    /// * `tiles` - Variable number of tile dimensions
    ///
    /// # Examples
    /// ```
    /// use rezalnyas_core::models::{TileDimensions, stock::StockSolution};
    ///
    /// let tile1 = TileDimensions::simple(100, 200);
    /// let tile2 = TileDimensions::simple(150, 300);
    /// let solution = StockSolution::from_tiles(&[tile1, tile2]);
    /// assert_eq!(solution.get_stock_tile_dimensions().len(), 2);
    /// ```
    pub fn from_tiles(tiles: &[TileDimensions]) -> Self {
        Self {
            stock_tile_dimensions: tiles.to_vec(),
        }
    }

    /// Creates an empty StockSolution
    ///
    /// # Examples
    /// ```
    /// use rezalnyas_core::models::stock::StockSolution;
    ///
    /// let solution = StockSolution::empty();
    /// assert_eq!(solution.get_stock_tile_dimensions().len(), 0);
    /// ```
    pub fn empty() -> Self {
        Self {
            stock_tile_dimensions: Vec::new(),
        }
    }

    /// Adds a stock tile to the solution
    ///
    /// # Arguments
    /// * `tile_dimensions` - The tile dimensions to add
    ///
    /// # Examples
    /// ```
    /// use rezalnyas_core::models::{TileDimensions, stock::StockSolution};
    ///
    /// let mut solution = StockSolution::empty();
    /// solution.add_stock_tile(TileDimensions::simple(100, 200));
    /// assert_eq!(solution.get_stock_tile_dimensions().len(), 1);
    /// ```
    pub fn add_stock_tile(&mut self, tile_dimensions: TileDimensions) {
        self.stock_tile_dimensions.push(tile_dimensions);
    }

    /// Gets a reference to the stock tile dimensions
    ///
    /// # Returns
    /// Reference to the vector of stock tile dimensions
    pub fn get_stock_tile_dimensions(&self) -> &Vec<TileDimensions> {
        &self.stock_tile_dimensions
    }

    /// Sets the stock tile dimensions
    ///
    /// # Arguments
    /// * `stock_tiles` - New vector of stock tile dimensions
    pub fn set_stock_tile_dimensions(&mut self, stock_tiles: Vec<TileDimensions>) {
        self.stock_tile_dimensions = stock_tiles;
    }

    /// Sorts panels in ascending order by area
    ///
    /// # Examples
    /// ```
    /// use rezalnyas_core::models::{TileDimensions, stock::StockSolution};
    ///
    /// let mut solution = StockSolution::new(vec![
    ///     TileDimensions::simple(200, 300), // area: 60000
    ///     TileDimensions::simple(100, 200), // area: 20000
    /// ]);
    /// solution.sort_panels_asc();
    /// let tiles = solution.get_stock_tile_dimensions();
    /// assert!(tiles[0].area() < tiles[1].area());
    /// ```
    pub fn sort_panels_asc(&mut self) {
        self.stock_tile_dimensions.sort_by(|a, b| a.area().cmp(&b.area()));
    }

    /// Sorts panels in descending order by area
    ///
    /// # Examples
    /// ```
    /// use rezalnyas_core::models::{TileDimensions, stock::StockSolution};
    ///
    /// let mut solution = StockSolution::new(vec![
    ///     TileDimensions::simple(100, 200), // area: 20000
    ///     TileDimensions::simple(200, 300), // area: 60000
    /// ]);
    /// solution.sort_panels_desc();
    /// let tiles = solution.get_stock_tile_dimensions();
    /// assert!(tiles[0].area() > tiles[1].area());
    /// ```
    pub fn sort_panels_desc(&mut self) {
        self.stock_tile_dimensions.sort_by(|a, b| b.area().cmp(&a.area()));
    }

    /// Checks if all panels have the same dimensions
    ///
    /// # Returns
    /// `true` if all panels have unique (same) dimensions, `false` otherwise
    ///
    /// # Examples
    /// ```
    /// use rezalnyas_core::models::{TileDimensions, stock::StockSolution};
    ///
    /// let solution_unique = StockSolution::new(vec![
    ///     TileDimensions::simple(100, 200),
    ///     TileDimensions::simple(100, 200),
    /// ]);
    /// assert!(solution_unique.has_unique_panel_size());
    ///
    /// let solution_mixed = StockSolution::new(vec![
    ///     TileDimensions::simple(100, 200),
    ///     TileDimensions::simple(150, 300),
    /// ]);
    /// assert!(!solution_mixed.has_unique_panel_size());
    /// ```
    pub fn has_unique_panel_size(&self) -> bool {
        if self.stock_tile_dimensions.is_empty() {
            return true;
        }

        let first = &self.stock_tile_dimensions[0];
        self.stock_tile_dimensions
            .iter()
            .skip(1)
            .all(|tile| tile.has_same_dimensions(first))
    }

    /// Calculates the total area of all stock tiles
    ///
    /// # Returns
    /// Total area as u64
    ///
    /// # Examples
    /// ```
    /// use rezalnyas_core::models::{TileDimensions, stock::StockSolution};
    ///
    /// let solution = StockSolution::new(vec![
    ///     TileDimensions::simple(100, 200), // area: 20000
    ///     TileDimensions::simple(150, 300), // area: 45000
    /// ]);
    /// assert_eq!(solution.get_total_area(), 65000);
    /// ```
    pub fn get_total_area(&self) -> u64 {
        self.stock_tile_dimensions
            .iter()
            .map(|tile| tile.area())
            .sum()
    }

    /// Returns a string representation showing grouped dimensions
    ///
    /// Groups tiles by dimensions and shows count for each unique size.
    ///
    /// # Returns
    /// String in format "100x200*2 150x300*1"
    ///
    /// # Examples
    /// ```
    /// use rezalnyas_core::models::{TileDimensions, stock::StockSolution};
    ///
    /// let solution = StockSolution::new(vec![
    ///     TileDimensions::simple(100, 200),
    ///     TileDimensions::simple(100, 200),
    ///     TileDimensions::simple(150, 300),
    /// ]);
    /// let grouped = solution.to_string_grouped();
    /// assert!(grouped.contains("100x200*2"));
    /// assert!(grouped.contains("150x300*1"));
    /// ```
    pub fn to_string_grouped(&self) -> String {
        let mut dimension_counts: HashMap<String, u32> = HashMap::new();

        for tile in &self.stock_tile_dimensions {
            let dimension_key = format!("{}x{}", tile.width(), tile.height());
            *dimension_counts.entry(dimension_key).or_insert(0) += 1;
        }

        let mut result_parts: Vec<String> = dimension_counts
            .iter()
            .map(|(dimensions, count)| format!("{}*{}", dimensions, count))
            .collect();

        result_parts.sort(); // For consistent output
        result_parts.join(" ")
    }

    /// Checks if the solution is empty
    ///
    /// # Returns
    /// `true` if no stock tiles are present
    pub fn is_empty(&self) -> bool {
        self.stock_tile_dimensions.is_empty()
    }

    /// Gets the number of stock tiles in the solution
    ///
    /// # Returns
    /// Number of stock tiles
    pub fn len(&self) -> usize {
        self.stock_tile_dimensions.len()
    }
}

impl PartialEq for StockSolution {
    /// Checks equality based on tile dimensions, ignoring order
    ///
    /// Two solutions are equal if they contain the same tiles (by dimensions),
    /// regardless of order.
    fn eq(&self, other: &Self) -> bool {
        if self.stock_tile_dimensions.len() != other.stock_tile_dimensions.len() {
            return false;
        }

        // Create a copy of other's tiles to mark as used
        let mut other_tiles = other.stock_tile_dimensions.clone();

        for self_tile in &self.stock_tile_dimensions {
            let mut found = false;
            for (i, other_tile) in other_tiles.iter().enumerate() {
                if self_tile.has_same_dimensions(other_tile) {
                    other_tiles.remove(i);
                    found = true;
                    break;
                }
            }
            if !found {
                return false;
            }
        }

        true
    }
}

impl Eq for StockSolution {}

impl std::hash::Hash for StockSolution {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Hash based on sorted dimension strings for consistent hashing
        let mut dimension_strings: Vec<String> = self
            .stock_tile_dimensions
            .iter()
            .map(|tile| format!("{}x{}", tile.width(), tile.height()))
            .collect();
        dimension_strings.sort();
        dimension_strings.hash(state);
    }
}

impl fmt::Display for StockSolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tile_strings: Vec<String> = self
            .stock_tile_dimensions
            .iter()
            .map(|tile| format!("[{}x{}]", tile.width(), tile.height()))
            .collect();
        write!(f, "{}", tile_strings.join(""))
    }
}

impl Default for StockSolution {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stock_solution() {
        let tiles = vec![
            TileDimensions::simple(100, 200),
            TileDimensions::simple(150, 300),
        ];
        let solution = StockSolution::new(tiles);
        assert_eq!(solution.get_stock_tile_dimensions().len(), 2);
    }

    #[test]
    fn test_from_tiles() {
        let tile1 = TileDimensions::simple(100, 200);
        let tile2 = TileDimensions::simple(150, 300);
        let solution = StockSolution::from_tiles(&[tile1, tile2]);
        assert_eq!(solution.get_stock_tile_dimensions().len(), 2);
    }

    #[test]
    fn test_empty_solution() {
        let solution = StockSolution::empty();
        assert!(solution.is_empty());
        assert_eq!(solution.len(), 0);
    }

    #[test]
    fn test_add_stock_tile() {
        let mut solution = StockSolution::empty();
        solution.add_stock_tile(TileDimensions::simple(100, 200));
        assert_eq!(solution.len(), 1);
        assert!(!solution.is_empty());
    }

    #[test]
    fn test_sort_panels_asc() {
        let mut solution = StockSolution::new(vec![
            TileDimensions::simple(200, 300), // area: 60000
            TileDimensions::simple(100, 200), // area: 20000
            TileDimensions::simple(150, 250), // area: 37500
        ]);

        solution.sort_panels_asc();
        let tiles = solution.get_stock_tile_dimensions();

        assert_eq!(tiles[0].area(), 20000);
        assert_eq!(tiles[1].area(), 37500);
        assert_eq!(tiles[2].area(), 60000);
    }

    #[test]
    fn test_sort_panels_desc() {
        let mut solution = StockSolution::new(vec![
            TileDimensions::simple(100, 200), // area: 20000
            TileDimensions::simple(200, 300), // area: 60000
            TileDimensions::simple(150, 250), // area: 37500
        ]);

        solution.sort_panels_desc();
        let tiles = solution.get_stock_tile_dimensions();

        assert_eq!(tiles[0].area(), 60000);
        assert_eq!(tiles[1].area(), 37500);
        assert_eq!(tiles[2].area(), 20000);
    }

    #[test]
    fn test_has_unique_panel_size() {
        // Test with same dimensions
        let solution_unique = StockSolution::new(vec![
            TileDimensions::simple(100, 200),
            TileDimensions::simple(100, 200),
            TileDimensions::simple(200, 100), // Rotated version should be considered same
        ]);
        assert!(solution_unique.has_unique_panel_size());

        // Test with different dimensions
        let solution_mixed = StockSolution::new(vec![
            TileDimensions::simple(100, 200),
            TileDimensions::simple(150, 300),
        ]);
        assert!(!solution_mixed.has_unique_panel_size());

        // Test empty solution
        let solution_empty = StockSolution::empty();
        assert!(solution_empty.has_unique_panel_size());
    }

    #[test]
    fn test_get_total_area() {
        let solution = StockSolution::new(vec![
            TileDimensions::simple(100, 200), // area: 20000
            TileDimensions::simple(150, 300), // area: 45000
            TileDimensions::simple(50, 100),  // area: 5000
        ]);
        assert_eq!(solution.get_total_area(), 70000);
    }

    #[test]
    fn test_to_string_grouped() {
        let solution = StockSolution::new(vec![
            TileDimensions::simple(100, 200),
            TileDimensions::simple(100, 200),
            TileDimensions::simple(150, 300),
        ]);

        let grouped = solution.to_string_grouped();
        assert!(grouped.contains("100x200*2"));
        assert!(grouped.contains("150x300*1"));
    }

    #[test]
    fn test_equality() {
        let solution1 = StockSolution::new(vec![
            TileDimensions::simple(100, 200),
            TileDimensions::simple(150, 300),
        ]);

        let solution2 = StockSolution::new(vec![
            TileDimensions::simple(150, 300),
            TileDimensions::simple(100, 200), // Different order
        ]);

        let solution3 = StockSolution::new(vec![
            TileDimensions::simple(100, 200),
            TileDimensions::simple(200, 400), // Different tile
        ]);

        assert_eq!(solution1, solution2); // Same tiles, different order
        assert_ne!(solution1, solution3); // Different tiles
    }

    #[test]
    fn test_equality_with_rotated_tiles() {
        let solution1 = StockSolution::new(vec![
            TileDimensions::simple(100, 200),
        ]);

        let solution2 = StockSolution::new(vec![
            TileDimensions::simple(200, 100), // Rotated version
        ]);

        assert_eq!(solution1, solution2); // Should be equal due to has_same_dimensions
    }

    #[test]
    fn test_display() {
        let solution = StockSolution::new(vec![
            TileDimensions::simple(100, 200),
            TileDimensions::simple(150, 300),
        ]);

        let display_string = format!("{}", solution);
        assert_eq!(display_string, "[100x200][150x300]");
    }

    #[test]
    fn test_hash_consistency() {
        use std::collections::HashMap;

        let solution1 = StockSolution::new(vec![
            TileDimensions::simple(100, 200),
            TileDimensions::simple(150, 300),
        ]);

        let solution2 = StockSolution::new(vec![
            TileDimensions::simple(150, 300),
            TileDimensions::simple(100, 200), // Different order
        ]);

        let mut map = HashMap::new();
        map.insert(solution1.clone(), "value1");

        // Should find the same entry due to consistent hashing
        assert_eq!(map.get(&solution2), Some(&"value1"));
    }

    #[test]
    fn test_set_stock_tile_dimensions() {
        let mut solution = StockSolution::new(vec![
            TileDimensions::simple(100, 200),
        ]);

        let new_tiles = vec![
            TileDimensions::simple(150, 300),
            TileDimensions::simple(200, 400),
        ];

        solution.set_stock_tile_dimensions(new_tiles);
        assert_eq!(solution.len(), 2);
        assert_eq!(solution.get_stock_tile_dimensions()[0].width(), 150);
    }

    #[test]
    fn test_default() {
        let solution = StockSolution::default();
        assert!(solution.is_empty());
    }
}
