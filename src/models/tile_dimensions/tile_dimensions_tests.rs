//! Integration tests for TileDimensions
//! 
//! These tests verify the complete functionality of the TileDimensions struct
//! including edge cases, performance characteristics, and integration scenarios.

use super::TileDimensions;
use std::collections::{HashMap, HashSet};

#[test]
fn test_constructor_variants() {
    // Test full constructor
    let full_tile = TileDimensions::new(
        42,
        150,
        200,
        "Ceramic".to_string(),
        2,
        Some("Kitchen Tile".to_string()),
        true,
    );
    
    assert_eq!(full_tile.id(), 42);
    assert_eq!(full_tile.width(), 150);
    assert_eq!(full_tile.height(), 200);
    assert_eq!(full_tile.material(), "Ceramic");
    assert_eq!(full_tile.orientation(), 2);
    assert_eq!(full_tile.label(), Some("Kitchen Tile"));
    assert!(full_tile.is_rotated());

    // Test constructor with defaults
    let default_tile = TileDimensions::new_with_defaults(
        10,
        100,
        150,
        "Wood".to_string(),
        1,
        None,
    );
    
    assert!(!default_tile.is_rotated());
    assert_eq!(default_tile.label(), None);

    // Test simple constructor
    let simple_tile = TileDimensions::simple(50, 75);
    assert_eq!(simple_tile.id(), -1);
    assert_eq!(simple_tile.material(), "DEFAULT_MATERIAL");
    assert_eq!(simple_tile.orientation(), 0);
}

#[test]
fn test_area_calculations() {
    let small_tile = TileDimensions::simple(10, 20);
    assert_eq!(small_tile.area(), 200);

    let large_tile = TileDimensions::simple(1000, 2000);
    assert_eq!(large_tile.area(), 2_000_000);

    // Test edge case with maximum u32 values
    let max_tile = TileDimensions::simple(u32::MAX, 1);
    assert_eq!(max_tile.area(), u32::MAX as u64);
}

#[test]
fn test_rotation_operations() {
    let original = TileDimensions::new(
        1,
        100,
        200,
        "Wood".to_string(),
        1,
        Some("Test".to_string()),
        false,
    );

    let rotated = original.rotate_90();
    
    // Verify dimensions are swapped
    assert_eq!(rotated.width(), original.height());
    assert_eq!(rotated.height(), original.width());
    
    // Verify orientation change
    assert_eq!(rotated.orientation(), 2);
    assert!(rotated.is_rotated());
    
    // Verify other properties are preserved
    assert_eq!(rotated.id(), original.id());
    assert_eq!(rotated.material(), original.material());
    assert_eq!(rotated.label(), original.label());

    // Test rotation with different orientation
    let orientation_2 = TileDimensions::new(
        2,
        150,
        100,
        "Metal".to_string(),
        2,
        None,
        false,
    );
    
    let rotated_2 = orientation_2.rotate_90();
    assert_eq!(rotated_2.orientation(), 1);
}

#[test]
fn test_geometric_properties() {
    // Test square detection
    let square = TileDimensions::simple(100, 100);
    let rectangle = TileDimensions::simple(100, 200);
    
    assert!(square.is_square());
    assert!(!rectangle.is_square());
    
    // Test horizontal detection
    let horizontal = TileDimensions::simple(200, 100);
    let vertical = TileDimensions::simple(100, 200);
    
    assert!(horizontal.is_horizontal());
    assert!(!vertical.is_horizontal());
    assert!(!square.is_horizontal()); // Square is not horizontal
    
    // Test max dimension
    assert_eq!(horizontal.max_dimension(), 200);
    assert_eq!(vertical.max_dimension(), 200);
    assert_eq!(square.max_dimension(), 100);
}

#[test]
fn test_dimension_comparison() {
    let tile1 = TileDimensions::simple(100, 200);
    let tile2 = TileDimensions::simple(100, 200); // Same dimensions
    let tile3 = TileDimensions::simple(200, 100); // Rotated dimensions
    let tile4 = TileDimensions::simple(150, 200); // Different dimensions
    
    // Test same dimensions
    assert!(tile1.has_same_dimensions(&tile2));
    assert!(tile1.has_same_dimensions(&tile3)); // Should handle rotation
    assert!(!tile1.has_same_dimensions(&tile4));
    
    // Test reflexivity
    assert!(tile1.has_same_dimensions(&tile1));
}

#[test]
fn test_fitting_logic() {
    let container = TileDimensions::simple(200, 300);
    
    // Test tiles that fit normally
    let fits_normal = TileDimensions::simple(100, 150);
    assert!(container.fits(&fits_normal));
    
    // Test tiles that fit when rotated
    let fits_rotated = TileDimensions::simple(250, 100);
    assert!(container.fits(&fits_rotated));
    
    // Test tiles that don't fit
    let too_large = TileDimensions::simple(250, 350);
    assert!(!container.fits(&too_large));
    
    // Test exact fit
    let exact_fit = TileDimensions::simple(200, 300);
    assert!(container.fits(&exact_fit));
    
    // Test exact fit rotated
    let exact_fit_rotated = TileDimensions::simple(300, 200);
    assert!(container.fits(&exact_fit_rotated));
}

#[test]
fn test_string_representations() {
    let tile = TileDimensions::new(
        42,
        150,
        200,
        "Ceramic".to_string(),
        1,
        Some("Kitchen".to_string()),
        false,
    );
    
    // Test Display implementation
    assert_eq!(format!("{}", tile), "id=42[150x200]");
    
    // Test dimensions string
    assert_eq!(tile.dimensions_to_string(), "150x200");
    
    // Test with negative ID
    let simple_tile = TileDimensions::simple(100, 50);
    assert_eq!(format!("{}", simple_tile), "id=-1[100x50]");
}

#[test]
fn test_equality_and_hashing() {
    let tile1 = TileDimensions::new(
        1,
        100,
        200,
        "Wood".to_string(),
        1,
        Some("Test".to_string()),
        false,
    );
    
    let tile2 = TileDimensions::new(
        1,
        100,
        200,
        "Metal".to_string(), // Different material
        2,                   // Different orientation
        None,                // Different label
        true,                // Different rotation
    );
    
    let tile3 = TileDimensions::new(
        2, // Different ID
        100,
        200,
        "Wood".to_string(),
        1,
        Some("Test".to_string()),
        false,
    );
    
    // Equality should be based only on id, width, and height
    assert_eq!(tile1, tile2);
    assert_ne!(tile1, tile3);
    
    // Test hash consistency
    let mut set = HashSet::new();
    set.insert(tile1.clone());
    assert!(set.contains(&tile2)); // Should find tile2 since it's equal to tile1
    assert!(!set.contains(&tile3)); // Should not find tile3
}

#[test]
fn test_hash_map_usage() {
    let mut tile_map: HashMap<TileDimensions, String> = HashMap::new();
    
    let tile1 = TileDimensions::simple(100, 200);
    let tile2 = TileDimensions::simple(100, 200); // Same dimensions, same ID
    let tile3 = TileDimensions::simple(150, 200); // Different dimensions
    
    tile_map.insert(tile1.clone(), "First tile".to_string());
    
    // Should find the same entry for tile2 (equal to tile1)
    assert_eq!(tile_map.get(&tile2), Some(&"First tile".to_string()));
    
    // Should not find entry for tile3
    assert_eq!(tile_map.get(&tile3), None);
}

#[test]
fn test_dimensions_based_hash() {
    let tile1 = TileDimensions::new(
        1,
        100,
        200,
        "Wood".to_string(),
        1,
        Some("Test".to_string()),
        false,
    );
    
    let tile2 = TileDimensions::new(
        999, // Different ID
        100,
        200,
        "Metal".to_string(),
        2,
        None,
        true,
    );
    
    let tile3 = TileDimensions::simple(150, 200);
    
    // Same dimensions should have same hash
    assert_eq!(
        tile1.dimensions_based_hash_code(),
        tile2.dimensions_based_hash_code()
    );
    
    // Different dimensions should have different hash
    assert_ne!(
        tile1.dimensions_based_hash_code(),
        tile3.dimensions_based_hash_code()
    );
}

#[test]
fn test_clone_functionality() {
    let original = TileDimensions::new(
        42,
        150,
        200,
        "Ceramic".to_string(),
        2,
        Some("Kitchen Tile".to_string()),
        true,
    );
    
    let cloned = original.clone();
    
    // Verify all fields are equal
    assert_eq!(original, cloned);
    assert_eq!(original.material(), cloned.material());
    assert_eq!(original.label(), cloned.label());
    assert_eq!(original.orientation(), cloned.orientation());
    assert_eq!(original.is_rotated(), cloned.is_rotated());
    
    // Verify they are separate instances
    assert_eq!(original.id(), cloned.id());
}

#[test]
fn test_edge_cases() {
    // Test with zero dimensions (should be allowed)
    let zero_width = TileDimensions::simple(0, 100);
    assert_eq!(zero_width.area(), 0);
    assert_eq!(zero_width.max_dimension(), 100);
    assert!(!zero_width.is_square());
    
    let zero_height = TileDimensions::simple(100, 0);
    assert_eq!(zero_height.area(), 0);
    assert_eq!(zero_height.max_dimension(), 100);
    assert!(!zero_height.is_square());
    
    // Test with very large dimensions
    let large_tile = TileDimensions::simple(u32::MAX - 1, u32::MAX - 1);
    assert_eq!(large_tile.area(), (u32::MAX as u64 - 1) * (u32::MAX as u64 - 1));
    assert!(large_tile.is_square());
}

#[test]
fn test_performance_characteristics() {
    // Test that operations are efficient with many tiles
    let mut tiles = Vec::new();
    
    // Create 1000 tiles
    for i in 0..1000 {
        tiles.push(TileDimensions::simple(i % 100 + 1, i % 50 + 1));
    }
    
    // Test bulk operations
    let total_area: u64 = tiles.iter().map(|t| t.area()).sum();
    assert!(total_area > 0);
    
    let max_dimensions: Vec<u32> = tiles.iter().map(|t| t.max_dimension()).collect();
    assert_eq!(max_dimensions.len(), 1000);
    
    // Test fitting operations
    let container = TileDimensions::simple(200, 200);
    let fitting_count = tiles.iter().filter(|t| container.fits(t)).count();
    assert!(fitting_count > 0);
}

#[test]
fn test_material_and_label_handling() {
    // Test with empty material
    let empty_material = TileDimensions::new(
        1,
        100,
        200,
        String::new(),
        0,
        None,
        false,
    );
    assert_eq!(empty_material.material(), "");
    
    // Test with very long material name
    let long_material = "A".repeat(1000);
    let long_material_tile = TileDimensions::new(
        2,
        100,
        200,
        long_material.clone(),
        0,
        None,
        false,
    );
    assert_eq!(long_material_tile.material(), &long_material);
    
    // Test with very long label
    let long_label = "B".repeat(1000);
    let long_label_tile = TileDimensions::new(
        3,
        100,
        200,
        "Wood".to_string(),
        0,
        Some(long_label.clone()),
        false,
    );
    assert_eq!(long_label_tile.label(), Some(long_label.as_str()));
}

#[test]
fn test_orientation_values() {
    // Test various orientation values
    for orientation in 0..10 {
        let tile = TileDimensions::new(
            1,
            100,
            200,
            "Test".to_string(),
            orientation,
            None,
            false,
        );
        assert_eq!(tile.orientation(), orientation);
        
        // Test rotation behavior
        let rotated = tile.rotate_90();
        let expected_orientation = if orientation == 1 { 2 } else { 1 };
        assert_eq!(rotated.orientation(), expected_orientation);
    }
}
