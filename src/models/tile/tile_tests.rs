//! Comprehensive tests for the Tile model
//! 
//! This module contains extensive unit tests for all Tile functionality,
//! including edge cases, error conditions, and performance characteristics.

use crate::models::{Tile, TileDimensions};
use crate::errors::CoreError;

#[cfg(test)]
mod tile_tests {
    use super::*;
    use std::collections::HashMap;

    // Constructor tests
    #[test]
    fn test_from_tile_dimensions_success() {
        let tile_dims = TileDimensions::simple(150, 250);
        let tile = Tile::from_tile_dimensions(&tile_dims).unwrap();
        
        assert_eq!(tile.x1(), 0);
        assert_eq!(tile.x2(), 150);
        assert_eq!(tile.y1(), 0);
        assert_eq!(tile.y2(), 250);
        assert_eq!(tile.width(), 150);
        assert_eq!(tile.height(), 250);
        assert_eq!(tile.area(), 37_500);
    }

    #[test]
    fn test_from_tile_dimensions_large_values() {
        // Test with maximum safe u32 values
        let tile_dims = TileDimensions::simple(u32::MAX / 2, u32::MAX / 2);
        let result = Tile::from_tile_dimensions(&tile_dims);
        assert!(result.is_ok());
        
        let tile = result.unwrap();
        assert_eq!(tile.width(), (u32::MAX / 2) as i32);
        assert_eq!(tile.height(), (u32::MAX / 2) as i32);
    }

    #[test]
    fn test_new_valid_coordinates_positive() {
        let tile = Tile::new(5, 25, 10, 30).unwrap();
        
        assert_eq!(tile.x1(), 5);
        assert_eq!(tile.x2(), 25);
        assert_eq!(tile.y1(), 10);
        assert_eq!(tile.y2(), 30);
        assert_eq!(tile.width(), 20);
        assert_eq!(tile.height(), 20);
    }

    #[test]
    fn test_new_valid_coordinates_negative() {
        let tile = Tile::new(-50, -10, -30, -5).unwrap();
        
        assert_eq!(tile.x1(), -50);
        assert_eq!(tile.x2(), -10);
        assert_eq!(tile.y1(), -30);
        assert_eq!(tile.y2(), -5);
        assert_eq!(tile.width(), 40);
        assert_eq!(tile.height(), 25);
    }

    #[test]
    fn test_new_invalid_x_coordinates() {
        // x2 == x1
        let result = Tile::new(10, 10, 5, 15);
        assert!(result.is_err());
        if let Err(CoreError::InvalidInput { details }) = result {
            assert!(details.contains("x2 (10) must be greater than x1 (10)"));
        }

        // x2 < x1
        let result = Tile::new(20, 15, 5, 15);
        assert!(result.is_err());
        if let Err(CoreError::InvalidInput { details }) = result {
            assert!(details.contains("x2 (15) must be greater than x1 (20)"));
        }
    }

    #[test]
    fn test_new_invalid_y_coordinates() {
        // y2 == y1
        let result = Tile::new(5, 15, 10, 10);
        assert!(result.is_err());
        if let Err(CoreError::InvalidInput { details }) = result {
            assert!(details.contains("y2 (10) must be greater than y1 (10)"));
        }

        // y2 < y1
        let result = Tile::new(5, 15, 20, 15);
        assert!(result.is_err());
        if let Err(CoreError::InvalidInput { details }) = result {
            assert!(details.contains("y2 (15) must be greater than y1 (20)"));
        }
    }

    #[test]
    fn test_new_unchecked() {
        let tile = Tile::new_unchecked(0, 100, 0, 200);
        
        assert_eq!(tile.x1(), 0);
        assert_eq!(tile.x2(), 100);
        assert_eq!(tile.y1(), 0);
        assert_eq!(tile.y2(), 200);
        assert_eq!(tile.width(), 100);
        assert_eq!(tile.height(), 200);
    }

    // Getter tests
    #[test]
    fn test_coordinate_getters() {
        let tile = Tile::new_unchecked(15, 85, 25, 75);
        
        assert_eq!(tile.x1(), 15);
        assert_eq!(tile.x2(), 85);
        assert_eq!(tile.y1(), 25);
        assert_eq!(tile.y2(), 75);
    }

    // Dimension calculation tests
    #[test]
    fn test_width_calculation() {
        let tile1 = Tile::new_unchecked(0, 100, 0, 50);
        let tile2 = Tile::new_unchecked(-20, 30, 0, 50);
        
        assert_eq!(tile1.width(), 100);
        assert_eq!(tile2.width(), 50);
    }

    #[test]
    fn test_height_calculation() {
        let tile1 = Tile::new_unchecked(0, 50, 0, 100);
        let tile2 = Tile::new_unchecked(0, 50, -25, 25);
        
        assert_eq!(tile1.height(), 100);
        assert_eq!(tile2.height(), 50);
    }

    #[test]
    fn test_area_calculation_small() {
        let tile = Tile::new_unchecked(0, 10, 0, 20);
        assert_eq!(tile.area(), 200);
    }

    #[test]
    fn test_area_calculation_large() {
        // Test area that would overflow i32 but fits in u64
        let tile = Tile::new_unchecked(0, 100_000, 0, 100_000);
        assert_eq!(tile.area(), 10_000_000_000_u64);
    }

    #[test]
    fn test_area_calculation_maximum() {
        // Test with maximum i32 values
        let max_coord = i32::MAX;
        let tile = Tile::new_unchecked(0, max_coord, 0, max_coord);
        let expected_area = (max_coord as u64) * (max_coord as u64);
        assert_eq!(tile.area(), expected_area);
    }

    #[test]
    fn test_max_side() {
        let horizontal_tile = Tile::new_unchecked(0, 150, 0, 100);
        let vertical_tile = Tile::new_unchecked(0, 100, 0, 150);
        let square_tile = Tile::new_unchecked(0, 125, 0, 125);
        
        assert_eq!(horizontal_tile.max_side(), 150);
        assert_eq!(vertical_tile.max_side(), 150);
        assert_eq!(square_tile.max_side(), 125);
    }

    // Orientation tests
    #[test]
    fn test_is_horizontal() {
        let horizontal_tile = Tile::new_unchecked(0, 200, 0, 100);
        let vertical_tile = Tile::new_unchecked(0, 100, 0, 200);
        let square_tile = Tile::new_unchecked(0, 150, 0, 150);
        
        assert!(horizontal_tile.is_horizontal());
        assert!(!vertical_tile.is_horizontal());
        assert!(!square_tile.is_horizontal());
    }

    #[test]
    fn test_is_vertical() {
        let horizontal_tile = Tile::new_unchecked(0, 200, 0, 100);
        let vertical_tile = Tile::new_unchecked(0, 100, 0, 200);
        let square_tile = Tile::new_unchecked(0, 150, 0, 150);
        
        assert!(!horizontal_tile.is_vertical());
        assert!(vertical_tile.is_vertical());
        assert!(!square_tile.is_vertical());
    }

    #[test]
    fn test_is_square() {
        let horizontal_tile = Tile::new_unchecked(0, 200, 0, 100);
        let vertical_tile = Tile::new_unchecked(0, 100, 0, 200);
        let square_tile = Tile::new_unchecked(0, 150, 0, 150);
        
        assert!(!horizontal_tile.is_square());
        assert!(!vertical_tile.is_square());
        assert!(square_tile.is_square());
    }

    // Translation tests
    #[test]
    fn test_translate_positive_offset() {
        let tile = Tile::new_unchecked(10, 50, 20, 60);
        let translated = tile.translate(15, 25).unwrap();
        
        assert_eq!(translated.x1(), 25);
        assert_eq!(translated.x2(), 65);
        assert_eq!(translated.y1(), 45);
        assert_eq!(translated.y2(), 85);
        
        // Verify dimensions remain the same
        assert_eq!(translated.width(), tile.width());
        assert_eq!(translated.height(), tile.height());
        assert_eq!(translated.area(), tile.area());
    }

    #[test]
    fn test_translate_negative_offset() {
        let tile = Tile::new_unchecked(50, 100, 60, 120);
        let translated = tile.translate(-30, -40).unwrap();
        
        assert_eq!(translated.x1(), 20);
        assert_eq!(translated.x2(), 70);
        assert_eq!(translated.y1(), 20);
        assert_eq!(translated.y2(), 80);
        
        // Verify dimensions remain the same
        assert_eq!(translated.width(), tile.width());
        assert_eq!(translated.height(), tile.height());
    }

    #[test]
    fn test_translate_zero_offset() {
        let tile = Tile::new_unchecked(10, 50, 20, 60);
        let translated = tile.translate(0, 0).unwrap();
        
        assert_eq!(tile, translated);
    }

    #[test]
    fn test_translate_overflow_x1() {
        let tile = Tile::new_unchecked(i32::MAX - 5, i32::MAX, 0, 10);
        let result = tile.translate(10, 0);
        
        assert!(result.is_err());
        if let Err(CoreError::InvalidInput { details }) = result {
            assert!(details.contains("x1 overflow"));
        }
    }

    #[test]
    fn test_translate_overflow_x2() {
        let tile = Tile::new_unchecked(0, i32::MAX - 5, 0, 10);
        let result = tile.translate(10, 0);
        
        assert!(result.is_err());
        if let Err(CoreError::InvalidInput { details }) = result {
            assert!(details.contains("x2 overflow"));
        }
    }

    #[test]
    fn test_translate_overflow_y1() {
        let tile = Tile::new_unchecked(0, 10, i32::MAX - 5, i32::MAX);
        let result = tile.translate(0, 10);
        
        assert!(result.is_err());
        if let Err(CoreError::InvalidInput { details }) = result {
            assert!(details.contains("y1 overflow"));
        }
    }

    #[test]
    fn test_translate_overflow_y2() {
        let tile = Tile::new_unchecked(0, 10, 0, i32::MAX - 5);
        let result = tile.translate(0, 10);
        
        assert!(result.is_err());
        if let Err(CoreError::InvalidInput { details }) = result {
            assert!(details.contains("y2 overflow"));
        }
    }

    #[test]
    fn test_translate_underflow() {
        let tile = Tile::new_unchecked(i32::MIN + 5, i32::MIN + 10, 0, 10);
        let result = tile.translate(-10, 0);
        
        assert!(result.is_err());
    }

    // Point containment tests
    #[test]
    fn test_contains_point_inside() {
        let tile = Tile::new_unchecked(10, 50, 20, 60);
        
        assert!(tile.contains_point(30, 40));
        assert!(tile.contains_point(25, 35));
        assert!(tile.contains_point(45, 55));
    }

    #[test]
    fn test_contains_point_on_boundary() {
        let tile = Tile::new_unchecked(10, 50, 20, 60);
        
        // Corner points
        assert!(tile.contains_point(10, 20));
        assert!(tile.contains_point(50, 20));
        assert!(tile.contains_point(10, 60));
        assert!(tile.contains_point(50, 60));
        
        // Edge points
        assert!(tile.contains_point(30, 20));
        assert!(tile.contains_point(30, 60));
        assert!(tile.contains_point(10, 40));
        assert!(tile.contains_point(50, 40));
    }

    #[test]
    fn test_contains_point_outside() {
        let tile = Tile::new_unchecked(10, 50, 20, 60);
        
        // Outside in all directions
        assert!(!tile.contains_point(5, 40));   // Left
        assert!(!tile.contains_point(55, 40));  // Right
        assert!(!tile.contains_point(30, 15));  // Above
        assert!(!tile.contains_point(30, 65));  // Below
        
        // Diagonal outside
        assert!(!tile.contains_point(5, 15));
        assert!(!tile.contains_point(55, 65));
    }

    #[test]
    fn test_contains_point_negative_coordinates() {
        let tile = Tile::new_unchecked(-50, -10, -30, -5);
        
        assert!(tile.contains_point(-30, -15));
        assert!(tile.contains_point(-50, -30)); // Boundary
        assert!(!tile.contains_point(-60, -15));
        assert!(!tile.contains_point(-30, -35));
    }

    // Overlap tests
    #[test]
    fn test_overlaps_with_clear_overlap() {
        let tile1 = Tile::new_unchecked(10, 50, 20, 60);
        let tile2 = Tile::new_unchecked(30, 70, 40, 80);
        
        assert!(tile1.overlaps_with(&tile2));
        assert!(tile2.overlaps_with(&tile1)); // Symmetric
    }

    #[test]
    fn test_overlaps_with_no_overlap() {
        let tile1 = Tile::new_unchecked(10, 30, 20, 40);
        let tile2 = Tile::new_unchecked(50, 70, 60, 80);
        
        assert!(!tile1.overlaps_with(&tile2));
        assert!(!tile2.overlaps_with(&tile1)); // Symmetric
    }

    #[test]
    fn test_overlaps_with_edge_touching() {
        let tile1 = Tile::new_unchecked(10, 30, 20, 40);
        let tile2 = Tile::new_unchecked(30, 50, 20, 40); // Touching on right edge
        let tile3 = Tile::new_unchecked(10, 30, 40, 60); // Touching on bottom edge
        
        assert!(!tile1.overlaps_with(&tile2)); // Edge touching is not overlap
        assert!(!tile1.overlaps_with(&tile3)); // Edge touching is not overlap
    }

    #[test]
    fn test_overlaps_with_contained() {
        let large_tile = Tile::new_unchecked(10, 100, 20, 80);
        let small_tile = Tile::new_unchecked(30, 70, 40, 60);
        
        assert!(large_tile.overlaps_with(&small_tile));
        assert!(small_tile.overlaps_with(&large_tile)); // Symmetric
    }

    #[test]
    fn test_overlaps_with_partial_overlap() {
        let tile1 = Tile::new_unchecked(10, 50, 20, 60);
        let tile2 = Tile::new_unchecked(40, 80, 50, 90); // Partial overlap
        
        assert!(tile1.overlaps_with(&tile2));
        assert!(tile2.overlaps_with(&tile1)); // Symmetric
    }

    #[test]
    fn test_overlaps_with_same_tile() {
        let tile = Tile::new_unchecked(10, 50, 20, 60);
        
        assert!(tile.overlaps_with(&tile));
    }

    // Equality and hashing tests
    #[test]
    fn test_equality_same_coordinates() {
        let tile1 = Tile::new_unchecked(10, 50, 20, 60);
        let tile2 = Tile::new_unchecked(10, 50, 20, 60);
        
        assert_eq!(tile1, tile2);
    }

    #[test]
    fn test_equality_different_coordinates() {
        let tile1 = Tile::new_unchecked(10, 50, 20, 60);
        let tile2 = Tile::new_unchecked(10, 50, 20, 61);
        let tile3 = Tile::new_unchecked(11, 50, 20, 60);
        
        assert_ne!(tile1, tile2);
        assert_ne!(tile1, tile3);
    }

    #[test]
    fn test_hash_consistency() {
        let tile1 = Tile::new_unchecked(10, 50, 20, 60);
        let tile2 = Tile::new_unchecked(10, 50, 20, 60);
        let tile3 = Tile::new_unchecked(10, 50, 20, 61);
        
        let mut map = HashMap::new();
        map.insert(tile1.clone(), "value1");
        
        // Same coordinates should find the same entry
        assert_eq!(map.get(&tile2), Some(&"value1"));
        assert_eq!(map.get(&tile3), None);
    }

    // Display tests
    #[test]
    fn test_display_positive_coordinates() {
        let tile = Tile::new_unchecked(10, 50, 20, 80);
        let display_str = format!("{}", tile);
        assert_eq!(display_str, "Tile[(10,20) to (50,80), 40x60]");
    }

    #[test]
    fn test_display_negative_coordinates() {
        let tile = Tile::new_unchecked(-50, -10, -30, -5);
        let display_str = format!("{}", tile);
        assert_eq!(display_str, "Tile[(-50,-30) to (-10,-5), 40x25]");
    }

    #[test]
    fn test_display_mixed_coordinates() {
        let tile = Tile::new_unchecked(-20, 30, -10, 40);
        let display_str = format!("{}", tile);
        assert_eq!(display_str, "Tile[(-20,-10) to (30,40), 50x50]");
    }

    // Clone tests
    #[test]
    fn test_clone_independence() {
        let original = Tile::new_unchecked(10, 50, 20, 80);
        let cloned = original.clone();
        
        assert_eq!(original, cloned);
        
        // Verify they are independent (though Tile is immutable, this tests the clone)
        assert_eq!(original.x1(), cloned.x1());
        assert_eq!(original.x2(), cloned.x2());
        assert_eq!(original.y1(), cloned.y1());
        assert_eq!(original.y2(), cloned.y2());
    }

    // Edge case and stress tests
    #[test]
    fn test_minimum_size_tile() {
        let tile = Tile::new_unchecked(0, 1, 0, 1);
        
        assert_eq!(tile.width(), 1);
        assert_eq!(tile.height(), 1);
        assert_eq!(tile.area(), 1);
        assert!(tile.is_square());
    }

    #[test]
    fn test_maximum_coordinate_tile() {
        let tile = Tile::new_unchecked(0, i32::MAX, 0, i32::MAX);
        
        assert_eq!(tile.width(), i32::MAX);
        assert_eq!(tile.height(), i32::MAX);
        assert_eq!(tile.area(), (i32::MAX as u64) * (i32::MAX as u64));
        assert!(tile.is_square());
    }

    #[test]
    fn test_very_wide_tile() {
        let tile = Tile::new_unchecked(0, 1000000, 0, 1);
        
        assert_eq!(tile.width(), 1000000);
        assert_eq!(tile.height(), 1);
        assert_eq!(tile.area(), 1000000);
        assert!(tile.is_horizontal());
        assert!(!tile.is_vertical());
        assert!(!tile.is_square());
    }

    #[test]
    fn test_very_tall_tile() {
        let tile = Tile::new_unchecked(0, 1, 0, 1000000);
        
        assert_eq!(tile.width(), 1);
        assert_eq!(tile.height(), 1000000);
        assert_eq!(tile.area(), 1000000);
        assert!(!tile.is_horizontal());
        assert!(tile.is_vertical());
        assert!(!tile.is_square());
    }

    // Integration tests with TileDimensions
    #[test]
    fn test_integration_with_tile_dimensions() {
        let tile_dims = TileDimensions::new(
            1,
            100,
            200,
            "Wood".to_string(),
            1,
            Some("Test Tile".to_string()),
            false,
        );
        
        let tile = Tile::from_tile_dimensions(&tile_dims).unwrap();
        
        assert_eq!(tile.width() as u32, tile_dims.width());
        assert_eq!(tile.height() as u32, tile_dims.height());
        assert_eq!(tile.area(), tile_dims.area());
        assert_eq!(tile.is_horizontal(), tile_dims.is_horizontal());
    }

    #[test]
    fn test_integration_rotated_tile_dimensions() {
        let original_dims = TileDimensions::simple(150, 100);
        let rotated_dims = original_dims.rotate_90();
        
        let original_tile = Tile::from_tile_dimensions(&original_dims).unwrap();
        let rotated_tile = Tile::from_tile_dimensions(&rotated_dims).unwrap();
        
        assert_eq!(original_tile.width(), rotated_tile.height());
        assert_eq!(original_tile.height(), rotated_tile.width());
        assert_eq!(original_tile.area(), rotated_tile.area());
        assert_ne!(original_tile.is_horizontal(), rotated_tile.is_horizontal());
    }
}
