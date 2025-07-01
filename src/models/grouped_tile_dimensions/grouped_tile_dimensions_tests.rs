//! Tests for GroupedTileDimensions functionality
//! 
//! This module contains comprehensive unit tests for all GroupedTileDimensions methods,
//! ensuring functional equivalence with the original Java implementation.

#[cfg(test)]
mod tests {
    use super::super::grouped_tile_dimensions::GroupedTileDimensions;
    use crate::models::tile_dimensions::TileDimensions;
    use std::collections::HashMap;

    #[test]
    fn test_from_grouped_constructor() {
        let original = GroupedTileDimensions::new(100, 200, 5);
        let copied = GroupedTileDimensions::from_grouped(&original);

        assert_eq!(copied.width(), 100);
        assert_eq!(copied.height(), 200);
        assert_eq!(copied.group(), 5);
        assert_eq!(copied, original);
    }

    #[test]
    fn test_from_tile_dimensions_constructor() {
        let tile_dims = TileDimensions::new(
            1,
            150,
            250,
            "Wood".to_string(),
            1,
            Some("Test Label".to_string()),
            false,
        );
        let grouped = GroupedTileDimensions::from_tile_dimensions(tile_dims, 10);

        assert_eq!(grouped.id(), 1);
        assert_eq!(grouped.width(), 150);
        assert_eq!(grouped.height(), 250);
        assert_eq!(grouped.material(), "Wood");
        assert_eq!(grouped.orientation(), 1);
        assert_eq!(grouped.label(), Some("Test Label"));
        assert!(!grouped.is_rotated());
        assert_eq!(grouped.group(), 10);
    }

    #[test]
    fn test_simple_constructor() {
        let grouped = GroupedTileDimensions::new(75, 125, 3);

        assert_eq!(grouped.width(), 75);
        assert_eq!(grouped.height(), 125);
        assert_eq!(grouped.group(), 3);
        assert_eq!(grouped.id(), -1); // Default from TileDimensions::simple
    }

    #[test]
    fn test_full_params_constructor() {
        let grouped = GroupedTileDimensions::new_with_full_params(
            42,
            300,
            400,
            "Metal".to_string(),
            2,
            Some("Full Test".to_string()),
            true,
            7,
        );

        assert_eq!(grouped.id(), 42);
        assert_eq!(grouped.width(), 300);
        assert_eq!(grouped.height(), 400);
        assert_eq!(grouped.material(), "Metal");
        assert_eq!(grouped.orientation(), 2);
        assert_eq!(grouped.label(), Some("Full Test"));
        assert!(grouped.is_rotated());
        assert_eq!(grouped.group(), 7);
    }

    #[test]
    fn test_group_getter() {
        let grouped = GroupedTileDimensions::new(50, 60, 15);
        assert_eq!(grouped.group(), 15);
    }

    #[test]
    fn test_tile_dimensions_getter() {
        let tile_dims = TileDimensions::simple(80, 90);
        let grouped = GroupedTileDimensions::from_tile_dimensions(tile_dims.clone(), 20);
        
        assert_eq!(grouped.tile_dimensions(), &tile_dims);
    }

    #[test]
    fn test_delegated_methods() {
        let grouped = GroupedTileDimensions::new_with_full_params(
            99,
            120,
            180,
            "Plastic".to_string(),
            3,
            Some("Delegated Test".to_string()),
            false,
            25,
        );

        // Test all delegated methods
        assert_eq!(grouped.id(), 99);
        assert_eq!(grouped.width(), 120);
        assert_eq!(grouped.height(), 180);
        assert_eq!(grouped.material(), "Plastic");
        assert_eq!(grouped.orientation(), 3);
        assert_eq!(grouped.label(), Some("Delegated Test"));
        assert!(!grouped.is_rotated());
        assert_eq!(grouped.max_dimension(), 180);
        assert_eq!(grouped.area(), 21600); // 120 * 180
        assert!(!grouped.is_square());
        assert!(!grouped.is_horizontal()); // height > width
        assert_eq!(grouped.dimensions_to_string(), "120x180");
    }

    #[test]
    fn test_rotate_90() {
        let grouped = GroupedTileDimensions::new_with_full_params(
            1,
            100,
            200,
            "Wood".to_string(),
            1,
            Some("Rotate Test".to_string()),
            false,
            5,
        );

        let rotated = grouped.rotate_90();

        assert_eq!(rotated.width(), 200);
        assert_eq!(rotated.height(), 100);
        assert_eq!(rotated.orientation(), 2);
        assert!(rotated.is_rotated());
        assert_eq!(rotated.group(), 5); // Group should remain the same
        assert_eq!(rotated.id(), grouped.id());
        assert_eq!(rotated.material(), grouped.material());
    }

    #[test]
    fn test_has_same_dimensions() {
        let grouped1 = GroupedTileDimensions::new(100, 200, 1);
        let grouped2 = GroupedTileDimensions::new(100, 200, 2); // Different group
        let grouped3 = GroupedTileDimensions::new(200, 100, 1); // Rotated
        let grouped4 = GroupedTileDimensions::new(150, 200, 1); // Different dimensions

        assert!(grouped1.has_same_dimensions(&grouped2));
        assert!(grouped1.has_same_dimensions(&grouped3)); // Should handle rotation
        assert!(!grouped1.has_same_dimensions(&grouped4));
    }

    #[test]
    fn test_fits() {
        let large_grouped = GroupedTileDimensions::new(200, 300, 1);
        let small_grouped = GroupedTileDimensions::new(100, 150, 2);
        let rotated_small = GroupedTileDimensions::new(150, 100, 3);
        let too_large = GroupedTileDimensions::new(250, 350, 4);

        assert!(large_grouped.fits(&small_grouped));
        assert!(large_grouped.fits(&rotated_small));
        assert!(!large_grouped.fits(&too_large));
    }

    #[test]
    fn test_hash_code() {
        let grouped1 = GroupedTileDimensions::new(100, 200, 5);
        let grouped2 = GroupedTileDimensions::new(100, 200, 5);
        let grouped3 = GroupedTileDimensions::new(100, 200, 6); // Different group
        let grouped4 = GroupedTileDimensions::new(150, 200, 5); // Different dimensions

        assert_eq!(grouped1.hash_code(), grouped2.hash_code());
        assert_ne!(grouped1.hash_code(), grouped3.hash_code());
        assert_ne!(grouped1.hash_code(), grouped4.hash_code());
    }

    #[test]
    fn test_display_format() {
        let grouped = GroupedTileDimensions::new_with_full_params(
            42,
            100,
            200,
            "Wood".to_string(),
            1,
            Some("Display Test".to_string()),
            false,
            7,
        );

        let display_string = format!("{}", grouped);
        // Note: Keeping the original typo "gropup" for compatibility
        assert_eq!(display_string, "id=42, gropup=7[100x200]");
    }

    #[test]
    fn test_equality() {
        let grouped1 = GroupedTileDimensions::new_with_full_params(
            1,
            100,
            200,
            "Wood".to_string(),
            1,
            Some("Test".to_string()),
            false,
            5,
        );
        let grouped2 = GroupedTileDimensions::new_with_full_params(
            1,
            100,
            200,
            "Metal".to_string(), // Different material
            2,                   // Different orientation
            None,                // Different label
            true,                // Different rotation
            5,                   // Same group
        );
        let grouped3 = GroupedTileDimensions::new_with_full_params(
            1,
            100,
            200,
            "Wood".to_string(),
            1,
            Some("Test".to_string()),
            false,
            6, // Different group
        );
        let grouped4 = GroupedTileDimensions::new_with_full_params(
            2, // Different ID
            100,
            200,
            "Wood".to_string(),
            1,
            Some("Test".to_string()),
            false,
            5,
        );

        // Equality should be based on TileDimensions equality AND group
        assert_eq!(grouped1, grouped2); // Same ID, dimensions, and group
        assert_ne!(grouped1, grouped3); // Different group
        assert_ne!(grouped1, grouped4); // Different ID
    }

    #[test]
    fn test_hash_trait() {
        let grouped1 = GroupedTileDimensions::new(100, 200, 5);
        let grouped2 = GroupedTileDimensions::new(100, 200, 5);
        let grouped3 = GroupedTileDimensions::new(150, 200, 5);

        let mut map = HashMap::new();
        map.insert(grouped1.clone(), "value1");
        
        // Same dimensions and group should find the same entry
        assert_eq!(map.get(&grouped2), Some(&"value1"));
        assert_eq!(map.get(&grouped3), None);
    }

    #[test]
    fn test_clone() {
        let original = GroupedTileDimensions::new_with_full_params(
            1,
            100,
            200,
            "Wood".to_string(),
            1,
            Some("Clone Test".to_string()),
            false,
            5,
        );

        let cloned = original.clone();
        assert_eq!(original, cloned);
        assert_eq!(original.group(), cloned.group());
        assert_eq!(original.material(), cloned.material());
        assert_eq!(original.label(), cloned.label());
    }

    #[test]
    fn test_deref_trait() {
        let grouped = GroupedTileDimensions::new(200, 100, 5);
        
        // Test that we can call TileDimensions methods directly via Deref
        assert_eq!(grouped.area(), 20000);
        assert!(!grouped.is_square());
        assert!(grouped.is_horizontal()); // width (200) > height (100)
    }

    #[test]
    fn test_edge_cases() {
        // Test with zero dimensions
        let zero_grouped = GroupedTileDimensions::new(0, 0, 0);
        assert_eq!(zero_grouped.area(), 0);
        assert!(zero_grouped.is_square());

        // Test with negative group
        let negative_group = GroupedTileDimensions::new(50, 50, -1);
        assert_eq!(negative_group.group(), -1);

        // Test with large dimensions
        let large_grouped = GroupedTileDimensions::new(u32::MAX, u32::MAX, i32::MAX);
        assert_eq!(large_grouped.width(), u32::MAX);
        assert_eq!(large_grouped.height(), u32::MAX);
        assert_eq!(large_grouped.group(), i32::MAX);
    }

    #[test]
    fn test_functional_equivalence_with_java() {
        // Test the three Java constructors equivalents
        
        // Java: GroupedTileDimensions(GroupedTileDimensions groupedTileDimensions)
        let original = GroupedTileDimensions::new(100, 200, 5);
        let copy_constructed = GroupedTileDimensions::from_grouped(&original);
        assert_eq!(copy_constructed.width(), original.width());
        assert_eq!(copy_constructed.height(), original.height());
        assert_eq!(copy_constructed.group(), original.group());

        // Java: GroupedTileDimensions(TileDimensions tileDimensions, int i)
        let tile_dims = TileDimensions::simple(150, 250);
        let from_tile_dims = GroupedTileDimensions::from_tile_dimensions(tile_dims, 10);
        assert_eq!(from_tile_dims.width(), 150);
        assert_eq!(from_tile_dims.height(), 250);
        assert_eq!(from_tile_dims.group(), 10);

        // Java: GroupedTileDimensions(int i, int i2, int i3)
        let simple_constructed = GroupedTileDimensions::new(75, 125, 3);
        assert_eq!(simple_constructed.width(), 75);
        assert_eq!(simple_constructed.height(), 125);
        assert_eq!(simple_constructed.group(), 3);

        // Test getGroup() equivalent
        assert_eq!(simple_constructed.group(), 3);

        // Test toString() equivalent (with the original typo)
        let display_result = format!("{}", simple_constructed);
        assert!(display_result.contains("gropup=3"));
        assert!(display_result.contains("[75x125]"));

        // Test equals() equivalent
        let equal_grouped = GroupedTileDimensions::new(75, 125, 3);
        assert_eq!(simple_constructed, equal_grouped);

        // Test hashCode() equivalent
        assert_eq!(simple_constructed.hash_code(), equal_grouped.hash_code());
    }
}
