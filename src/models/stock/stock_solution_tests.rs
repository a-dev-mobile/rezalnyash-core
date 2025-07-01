//! Comprehensive tests for StockSolution
//!
//! This module contains extensive unit tests for the StockSolution struct,
//! covering all methods, edge cases, and performance scenarios.

#[cfg(test)]
mod tests {
    use crate::models::{TileDimensions, stock::StockSolution};
    use std::collections::HashMap;

    // Helper functions for creating test data
    fn create_small_tile() -> TileDimensions {
        TileDimensions::simple(100, 200)
    }

    fn create_medium_tile() -> TileDimensions {
        TileDimensions::simple(150, 300)
    }

    fn create_large_tile() -> TileDimensions {
        TileDimensions::simple(200, 400)
    }

    fn create_square_tile() -> TileDimensions {
        TileDimensions::simple(150, 150)
    }

    fn create_test_tiles() -> Vec<TileDimensions> {
        vec![
            create_small_tile(),
            create_medium_tile(),
            create_large_tile(),
        ]
    }

    // Constructor tests
    #[test]
    fn test_new_with_tiles() {
        let tiles = create_test_tiles();
        let solution = StockSolution::new(tiles.clone());
        
        assert_eq!(solution.get_stock_tile_dimensions().len(), 3);
        assert_eq!(solution.get_stock_tile_dimensions(), &tiles);
    }

    #[test]
    fn test_new_with_empty_vector() {
        let solution = StockSolution::new(vec![]);
        
        assert!(solution.is_empty());
        assert_eq!(solution.len(), 0);
    }

    #[test]
    fn test_from_tiles() {
        let tiles = create_test_tiles();
        let solution = StockSolution::from_tiles(&tiles);
        
        assert_eq!(solution.len(), 3);
        assert_eq!(solution.get_stock_tile_dimensions(), &tiles);
    }

    #[test]
    fn test_from_tiles_empty_slice() {
        let solution = StockSolution::from_tiles(&[]);
        
        assert!(solution.is_empty());
    }

    #[test]
    fn test_empty_constructor() {
        let solution = StockSolution::empty();
        
        assert!(solution.is_empty());
        assert_eq!(solution.len(), 0);
        assert_eq!(solution.get_total_area(), 0);
    }

    #[test]
    fn test_default_constructor() {
        let solution = StockSolution::default();
        
        assert!(solution.is_empty());
        assert_eq!(solution.len(), 0);
    }

    // Modification tests
    #[test]
    fn test_add_stock_tile() {
        let mut solution = StockSolution::empty();
        
        assert_eq!(solution.len(), 0);
        
        solution.add_stock_tile(create_small_tile());
        assert_eq!(solution.len(), 1);
        assert!(!solution.is_empty());
        
        solution.add_stock_tile(create_medium_tile());
        assert_eq!(solution.len(), 2);
    }

    #[test]
    fn test_add_multiple_tiles() {
        let mut solution = StockSolution::empty();
        let tiles = create_test_tiles();
        
        for tile in tiles {
            solution.add_stock_tile(tile);
        }
        
        assert_eq!(solution.len(), 3);
    }

    #[test]
    fn test_set_stock_tile_dimensions() {
        let mut solution = StockSolution::new(vec![create_small_tile()]);
        assert_eq!(solution.len(), 1);
        
        let new_tiles = create_test_tiles();
        solution.set_stock_tile_dimensions(new_tiles.clone());
        
        assert_eq!(solution.len(), 3);
        assert_eq!(solution.get_stock_tile_dimensions(), &new_tiles);
    }

    #[test]
    fn test_set_empty_tiles() {
        let mut solution = StockSolution::new(create_test_tiles());
        assert!(!solution.is_empty());
        
        solution.set_stock_tile_dimensions(vec![]);
        assert!(solution.is_empty());
    }

    // Sorting tests
    #[test]
    fn test_sort_panels_asc() {
        let mut solution = StockSolution::new(vec![
            create_large_tile(),  // area: 80000
            create_small_tile(),  // area: 20000
            create_medium_tile(), // area: 45000
        ]);
        
        solution.sort_panels_asc();
        let tiles = solution.get_stock_tile_dimensions();
        
        assert_eq!(tiles[0].area(), 20000);  // small
        assert_eq!(tiles[1].area(), 45000);  // medium
        assert_eq!(tiles[2].area(), 80000);  // large
        
        // Verify sorting is stable
        for i in 1..tiles.len() {
            assert!(tiles[i-1].area() <= tiles[i].area());
        }
    }

    #[test]
    fn test_sort_panels_desc() {
        let mut solution = StockSolution::new(vec![
            create_small_tile(),  // area: 20000
            create_large_tile(),  // area: 80000
            create_medium_tile(), // area: 45000
        ]);
        
        solution.sort_panels_desc();
        let tiles = solution.get_stock_tile_dimensions();
        
        assert_eq!(tiles[0].area(), 80000);  // large
        assert_eq!(tiles[1].area(), 45000);  // medium
        assert_eq!(tiles[2].area(), 20000);  // small
        
        // Verify sorting is stable
        for i in 1..tiles.len() {
            assert!(tiles[i-1].area() >= tiles[i].area());
        }
    }

    #[test]
    fn test_sort_empty_solution() {
        let mut solution = StockSolution::empty();
        
        // Should not panic
        solution.sort_panels_asc();
        solution.sort_panels_desc();
        
        assert!(solution.is_empty());
    }

    #[test]
    fn test_sort_single_tile() {
        let mut solution = StockSolution::new(vec![create_small_tile()]);
        
        solution.sort_panels_asc();
        assert_eq!(solution.len(), 1);
        
        solution.sort_panels_desc();
        assert_eq!(solution.len(), 1);
    }

    #[test]
    fn test_sort_equal_areas() {
        let tile1 = TileDimensions::simple(100, 200); // area: 20000
        let tile2 = TileDimensions::simple(200, 100); // area: 20000
        let tile3 = TileDimensions::simple(80, 250);  // area: 20000
        
        let mut solution = StockSolution::new(vec![tile1, tile2, tile3]);
        
        solution.sort_panels_asc();
        let tiles = solution.get_stock_tile_dimensions();
        
        // All should have same area
        for tile in tiles {
            assert_eq!(tile.area(), 20000);
        }
    }

    // Unique panel size tests
    #[test]
    fn test_has_unique_panel_size_true() {
        let solution = StockSolution::new(vec![
            TileDimensions::simple(100, 200),
            TileDimensions::simple(100, 200),
            TileDimensions::simple(200, 100), // Rotated version
        ]);
        
        assert!(solution.has_unique_panel_size());
    }

    #[test]
    fn test_has_unique_panel_size_false() {
        let solution = StockSolution::new(vec![
            TileDimensions::simple(100, 200),
            TileDimensions::simple(150, 300), // Different dimensions
        ]);
        
        assert!(!solution.has_unique_panel_size());
    }

    #[test]
    fn test_has_unique_panel_size_empty() {
        let solution = StockSolution::empty();
        assert!(solution.has_unique_panel_size());
    }

    #[test]
    fn test_has_unique_panel_size_single() {
        let solution = StockSolution::new(vec![create_small_tile()]);
        assert!(solution.has_unique_panel_size());
    }

    #[test]
    fn test_has_unique_panel_size_complex() {
        // Test with multiple tiles of same dimensions
        let solution = StockSolution::new(vec![
            TileDimensions::simple(100, 200),
            TileDimensions::simple(200, 100), // Rotated
            TileDimensions::simple(100, 200), // Same as first
            TileDimensions::simple(200, 100), // Rotated again
        ]);
        
        assert!(solution.has_unique_panel_size());
    }

    // Area calculation tests
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
    fn test_get_total_area_empty() {
        let solution = StockSolution::empty();
        assert_eq!(solution.get_total_area(), 0);
    }

    #[test]
    fn test_get_total_area_single() {
        let solution = StockSolution::new(vec![TileDimensions::simple(100, 200)]);
        assert_eq!(solution.get_total_area(), 20000);
    }

    #[test]
    fn test_get_total_area_large_numbers() {
        let solution = StockSolution::new(vec![
            TileDimensions::simple(1000, 2000), // area: 2,000,000
            TileDimensions::simple(1500, 3000), // area: 4,500,000
        ]);
        
        assert_eq!(solution.get_total_area(), 6_500_000);
    }

    // String representation tests
    #[test]
    fn test_to_string_grouped() {
        let solution = StockSolution::new(vec![
            TileDimensions::simple(100, 200),
            TileDimensions::simple(100, 200), // Duplicate
            TileDimensions::simple(150, 300),
            TileDimensions::simple(100, 200), // Another duplicate
        ]);
        
        let grouped = solution.to_string_grouped();
        
        assert!(grouped.contains("100x200*3"));
        assert!(grouped.contains("150x300*1"));
    }

    #[test]
    fn test_to_string_grouped_empty() {
        let solution = StockSolution::empty();
        let grouped = solution.to_string_grouped();
        
        assert_eq!(grouped, "");
    }

    #[test]
    fn test_to_string_grouped_single() {
        let solution = StockSolution::new(vec![TileDimensions::simple(100, 200)]);
        let grouped = solution.to_string_grouped();
        
        assert_eq!(grouped, "100x200*1");
    }

    #[test]
    fn test_to_string_grouped_all_unique() {
        let solution = StockSolution::new(vec![
            TileDimensions::simple(100, 200),
            TileDimensions::simple(150, 300),
            TileDimensions::simple(200, 400),
        ]);
        
        let grouped = solution.to_string_grouped();
        
        assert!(grouped.contains("100x200*1"));
        assert!(grouped.contains("150x300*1"));
        assert!(grouped.contains("200x400*1"));
    }

    #[test]
    fn test_to_string_grouped_consistency() {
        let solution = StockSolution::new(vec![
            TileDimensions::simple(200, 400),
            TileDimensions::simple(100, 200),
            TileDimensions::simple(150, 300),
        ]);
        
        let grouped1 = solution.to_string_grouped();
        let grouped2 = solution.to_string_grouped();
        
        // Should be consistent
        assert_eq!(grouped1, grouped2);
        
        // Should be sorted for predictable output
        let parts: Vec<&str> = grouped1.split(' ').collect();
        let mut sorted_parts = parts.clone();
        sorted_parts.sort();
        assert_eq!(parts, sorted_parts);
    }

    // Display trait tests
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
    fn test_display_empty() {
        let solution = StockSolution::empty();
        let display_string = format!("{}", solution);
        assert_eq!(display_string, "");
    }

    #[test]
    fn test_display_single() {
        let solution = StockSolution::new(vec![TileDimensions::simple(100, 200)]);
        let display_string = format!("{}", solution);
        assert_eq!(display_string, "[100x200]");
    }

    // Equality tests
    #[test]
    fn test_equality_same_order() {
        let solution1 = StockSolution::new(vec![
            TileDimensions::simple(100, 200),
            TileDimensions::simple(150, 300),
        ]);
        
        let solution2 = StockSolution::new(vec![
            TileDimensions::simple(100, 200),
            TileDimensions::simple(150, 300),
        ]);
        
        assert_eq!(solution1, solution2);
    }

    #[test]
    fn test_equality_different_order() {
        let solution1 = StockSolution::new(vec![
            TileDimensions::simple(100, 200),
            TileDimensions::simple(150, 300),
        ]);
        
        let solution2 = StockSolution::new(vec![
            TileDimensions::simple(150, 300),
            TileDimensions::simple(100, 200), // Different order
        ]);
        
        assert_eq!(solution1, solution2);
    }

    #[test]
    fn test_equality_with_rotated_tiles() {
        let solution1 = StockSolution::new(vec![
            TileDimensions::simple(100, 200),
        ]);
        
        let solution2 = StockSolution::new(vec![
            TileDimensions::simple(200, 100), // Rotated version
        ]);
        
        assert_eq!(solution1, solution2);
    }

    #[test]
    fn test_inequality_different_tiles() {
        let solution1 = StockSolution::new(vec![
            TileDimensions::simple(100, 200),
            TileDimensions::simple(150, 300),
        ]);
        
        let solution2 = StockSolution::new(vec![
            TileDimensions::simple(100, 200),
            TileDimensions::simple(200, 400), // Different tile
        ]);
        
        assert_ne!(solution1, solution2);
    }

    #[test]
    fn test_inequality_different_counts() {
        let solution1 = StockSolution::new(vec![
            TileDimensions::simple(100, 200),
        ]);
        
        let solution2 = StockSolution::new(vec![
            TileDimensions::simple(100, 200),
            TileDimensions::simple(100, 200), // Extra tile
        ]);
        
        assert_ne!(solution1, solution2);
    }

    #[test]
    fn test_equality_empty_solutions() {
        let solution1 = StockSolution::empty();
        let solution2 = StockSolution::empty();
        
        assert_eq!(solution1, solution2);
    }

    #[test]
    fn test_equality_complex_case() {
        let solution1 = StockSolution::new(vec![
            TileDimensions::simple(100, 200),
            TileDimensions::simple(150, 300),
            TileDimensions::simple(100, 200), // Duplicate
            TileDimensions::simple(200, 100), // Rotated version of first
        ]);
        
        let solution2 = StockSolution::new(vec![
            TileDimensions::simple(200, 100), // Rotated version
            TileDimensions::simple(300, 150), // Rotated version
            TileDimensions::simple(200, 100), // Another rotated
            TileDimensions::simple(100, 200), // Original
        ]);
        
        assert_eq!(solution1, solution2);
    }

    // Hash tests
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
    fn test_hash_with_rotated_tiles() {
        use std::collections::HashMap;
        
        let solution1 = StockSolution::new(vec![
            TileDimensions::simple(100, 200),
        ]);
        
        let solution2 = StockSolution::new(vec![
            TileDimensions::simple(200, 100), // Rotated
        ]);
        
        let mut map = HashMap::new();
        map.insert(solution1.clone(), "value1");
        
        // Should find the same entry since rotated tiles are considered equal
        assert_eq!(map.get(&solution2), Some(&"value1"));
    }

    #[test]
    fn test_hash_different_solutions() {
        use std::collections::HashMap;
        
        let solution1 = StockSolution::new(vec![
            TileDimensions::simple(100, 200),
        ]);
        
        let solution2 = StockSolution::new(vec![
            TileDimensions::simple(150, 300), // Different tile
        ]);
        
        let mut map = HashMap::new();
        map.insert(solution1.clone(), "value1");
        
        // Should not find entry for different solution
        assert_eq!(map.get(&solution2), None);
    }

    // Size and state tests
    #[test]
    fn test_is_empty() {
        let empty_solution = StockSolution::empty();
        assert!(empty_solution.is_empty());
        
        let non_empty_solution = StockSolution::new(vec![create_small_tile()]);
        assert!(!non_empty_solution.is_empty());
    }

    #[test]
    fn test_len() {
        let solution = StockSolution::new(create_test_tiles());
        assert_eq!(solution.len(), 3);
        
        let empty_solution = StockSolution::empty();
        assert_eq!(empty_solution.len(), 0);
    }

    // Performance and edge case tests
    #[test]
    fn test_large_solution() {
        let mut tiles = Vec::new();
        for i in 0..1000 {
            tiles.push(TileDimensions::simple(100 + i, 200 + i));
        }
        
        let solution = StockSolution::new(tiles);
        assert_eq!(solution.len(), 1000);
        assert!(!solution.is_empty());
        
        // Should handle large solutions efficiently
        let total_area = solution.get_total_area();
        assert!(total_area > 0);
    }

    #[test]
    fn test_zero_dimension_tiles() {
        let solution = StockSolution::new(vec![
            TileDimensions::simple(0, 100),
            TileDimensions::simple(100, 0),
            TileDimensions::simple(0, 0),
        ]);
        
        assert_eq!(solution.len(), 3);
        assert_eq!(solution.get_total_area(), 0);
    }

    #[test]
    fn test_very_large_dimensions() {
        let solution = StockSolution::new(vec![
            TileDimensions::simple(u32::MAX, 1),
            TileDimensions::simple(1, u32::MAX),
        ]);
        
        assert_eq!(solution.len(), 2);
        // Should handle large numbers without overflow
        let total_area = solution.get_total_area();
        assert!(total_area > 0);
    }

    #[test]
    fn test_clone() {
        let original = StockSolution::new(create_test_tiles());
        let cloned = original.clone();
        
        assert_eq!(original, cloned);
        assert_eq!(original.len(), cloned.len());
        assert_eq!(original.get_total_area(), cloned.get_total_area());
    }

    #[test]
    fn test_clone_independence() {
        let mut original = StockSolution::new(vec![create_small_tile()]);
        let mut cloned = original.clone();
        
        // Modify original
        original.add_stock_tile(create_medium_tile());
        
        // Clone should be unchanged
        assert_eq!(original.len(), 2);
        assert_eq!(cloned.len(), 1);
        assert_ne!(original, cloned);
    }

    // Integration tests
    #[test]
    fn test_workflow_create_modify_sort() {
        let mut solution = StockSolution::empty();
        
        // Add tiles
        solution.add_stock_tile(create_large_tile());
        solution.add_stock_tile(create_small_tile());
        solution.add_stock_tile(create_medium_tile());
        
        assert_eq!(solution.len(), 3);
        
        // Sort ascending
        solution.sort_panels_asc();
        let tiles = solution.get_stock_tile_dimensions();
        assert!(tiles[0].area() <= tiles[1].area());
        assert!(tiles[1].area() <= tiles[2].area());
        
        // Check total area
        let expected_area = create_large_tile().area() + 
                           create_small_tile().area() + 
                           create_medium_tile().area();
        assert_eq!(solution.get_total_area(), expected_area);
    }

    #[test]
    fn test_workflow_equality_and_hashing() {
        let solution1 = StockSolution::new(vec![
            create_small_tile(),
            create_medium_tile(),
        ]);
        
        let mut solution2 = StockSolution::empty();
        solution2.add_stock_tile(create_medium_tile());
        solution2.add_stock_tile(create_small_tile()); // Different order
        
        // Should be equal despite different construction
        assert_eq!(solution1, solution2);
        
        // Should work in hash maps
        let mut map = HashMap::new();
        map.insert(solution1, "test_value");
        assert_eq!(map.get(&solution2), Some(&"test_value"));
    }

    #[test]
    fn test_workflow_string_representations() {
        let solution = StockSolution::new(vec![
            TileDimensions::simple(100, 200),
            TileDimensions::simple(100, 200), // Duplicate
            TileDimensions::simple(150, 300),
        ]);
        
        // Test display format
        let display = format!("{}", solution);
        assert!(display.contains("[100x200]"));
        assert!(display.contains("[150x300]"));
        
        // Test grouped format
        let grouped = solution.to_string_grouped();
        assert!(grouped.contains("100x200*2"));
        assert!(grouped.contains("150x300*1"));
    }
}
