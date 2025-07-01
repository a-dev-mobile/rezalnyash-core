#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::cut::{Cut, CutBuilder};

    fn create_test_cut() -> Cut {
        Cut::new(
            10, 20, 50, 60,  // coordinates
            100, 80,         // original dimensions
            true,            // horizontal cut
            30,              // cut coordinate
            1,               // original tile ID
            2, 3             // child tile IDs
        )
    }

    #[test]
    fn test_new_cut() {
        let cut = create_test_cut();
        
        assert_eq!(cut.x1(), 10);
        assert_eq!(cut.y1(), 20);
        assert_eq!(cut.x2(), 50);
        assert_eq!(cut.y2(), 60);
        assert_eq!(cut.original_width(), 100);
        assert_eq!(cut.original_height(), 80);
        assert!(cut.is_horizontal());
        assert_eq!(cut.cut_coord(), 30);
        assert_eq!(cut.original_tile_id(), 1);
        assert_eq!(cut.child1_tile_id(), 2);
        assert_eq!(cut.child2_tile_id(), 3);
    }

    #[test]
    fn test_copy_constructor_equivalent() {
        let original = create_test_cut();
        let copied = Cut::new(
            original.x1(),
            original.y1(),
            original.x2(),
            original.y2(),
            original.original_width(),
            original.original_height(),
            original.is_horizontal(),
            original.cut_coord(),
            original.original_tile_id(),
            original.child1_tile_id(),
            original.child2_tile_id(),
        );
        
        assert_eq!(original, copied);
    }

    #[test]
    fn test_length_calculation() {
        let cut = Cut::new(0, 0, 3, 4, 100, 100, true, 50, 1, 2, 3);
        // Length should be |3-0| + |4-0| = 3 + 4 = 7
        assert_eq!(cut.length(), 7);
        
        let cut2 = Cut::new(10, 20, 10, 30, 100, 100, false, 25, 1, 2, 3);
        // Length should be |10-10| + |30-20| = 0 + 10 = 10
        assert_eq!(cut2.length(), 10);
    }

    #[test]
    fn test_width_and_height() {
        let cut = Cut::new(10, 20, 50, 60, 100, 80, true, 30, 1, 2, 3);
        assert_eq!(cut.width(), 40);  // |50-10| = 40
        assert_eq!(cut.height(), 40); // |60-20| = 40
    }

    #[test]
    fn test_area() {
        let cut = Cut::new(10, 20, 50, 60, 100, 80, true, 30, 1, 2, 3);
        assert_eq!(cut.area(), 1600); // 40 * 40 = 1600
    }

    #[test]
    fn test_is_vertical() {
        let horizontal_cut = Cut::new(0, 0, 10, 10, 100, 100, true, 5, 1, 2, 3);
        let vertical_cut = Cut::new(0, 0, 10, 10, 100, 100, false, 5, 1, 2, 3);
        
        assert!(!horizontal_cut.is_vertical());
        assert!(vertical_cut.is_vertical());
        assert!(horizontal_cut.is_horizontal());
        assert!(!vertical_cut.is_horizontal());
    }

    #[test]
    fn test_coordinates_tuple() {
        let cut = create_test_cut();
        assert_eq!(cut.coordinates(), (10, 20, 50, 60));
    }

    #[test]
    fn test_child_tile_ids_tuple() {
        let cut = create_test_cut();
        assert_eq!(cut.child_tile_ids(), (2, 3));
    }

    #[test]
    fn test_original_dimensions_tuple() {
        let cut = create_test_cut();
        assert_eq!(cut.original_dimensions(), (100, 80));
    }

    #[test]
    fn test_contains_point() {
        let cut = Cut::new(10, 20, 50, 60, 100, 80, true, 30, 1, 2, 3);
        
        // Points inside the rectangle
        assert!(cut.contains_point(30, 40));
        assert!(cut.contains_point(10, 20)); // corner
        assert!(cut.contains_point(50, 60)); // corner
        
        // Points outside the rectangle
        assert!(!cut.contains_point(5, 15));
        assert!(!cut.contains_point(55, 65));
        assert!(!cut.contains_point(30, 15));
        assert!(!cut.contains_point(55, 40));
    }

    #[test]
    fn test_contains_point_with_negative_coordinates() {
        let cut = Cut::new(-10, -20, 10, 20, 100, 80, true, 0, 1, 2, 3);
        
        assert!(cut.contains_point(0, 0));
        assert!(cut.contains_point(-5, -10));
        assert!(cut.contains_point(5, 10));
        assert!(!cut.contains_point(-15, -25));
        assert!(!cut.contains_point(15, 25));
    }

    #[test]
    fn test_intersects_with() {
        let cut1 = Cut::new(0, 0, 10, 10, 100, 100, true, 5, 1, 2, 3);
        let cut2 = Cut::new(5, 5, 15, 15, 100, 100, true, 10, 4, 5, 6);
        let cut3 = Cut::new(20, 20, 30, 30, 100, 100, true, 25, 7, 8, 9);
        
        // cut1 and cut2 should intersect
        assert!(cut1.intersects_with(&cut2));
        assert!(cut2.intersects_with(&cut1));
        
        // cut1 and cut3 should not intersect
        assert!(!cut1.intersects_with(&cut3));
        assert!(!cut3.intersects_with(&cut1));
        
        // cut2 and cut3 should not intersect
        assert!(!cut2.intersects_with(&cut3));
        assert!(!cut3.intersects_with(&cut2));
    }

    #[test]
    fn test_intersects_with_touching_edges() {
        let cut1 = Cut::new(0, 0, 10, 10, 100, 100, true, 5, 1, 2, 3);
        let cut2 = Cut::new(10, 0, 20, 10, 100, 100, true, 15, 4, 5, 6);
        
        // Cuts that touch at edges should intersect
        assert!(cut1.intersects_with(&cut2));
        assert!(cut2.intersects_with(&cut1));
    }

    #[test]
    fn test_display_formatting() {
        let cut = create_test_cut();
        let display_str = format!("{}", cut);
        
        assert!(display_str.contains("(10,20) -> (50,60)"));
        assert!(display_str.contains("horizontal"));
        assert!(display_str.contains("coord=30"));
        assert!(display_str.contains("original_tile=1"));
        assert!(display_str.contains("children=(2,3)"));
    }

    #[test]
    fn test_default() {
        let cut = Cut::default();
        
        assert_eq!(cut.x1(), 0);
        assert_eq!(cut.y1(), 0);
        assert_eq!(cut.x2(), 0);
        assert_eq!(cut.y2(), 0);
        assert_eq!(cut.original_width(), 0);
        assert_eq!(cut.original_height(), 0);
        assert!(cut.is_horizontal());
        assert_eq!(cut.cut_coord(), 0);
        assert_eq!(cut.original_tile_id(), 0);
        assert_eq!(cut.child1_tile_id(), 0);
        assert_eq!(cut.child2_tile_id(), 0);
    }

    #[test]
    fn test_clone_and_equality() {
        let cut1 = create_test_cut();
        let cut2 = cut1.clone();
        
        assert_eq!(cut1, cut2);
        assert_eq!(cut1.x1(), cut2.x1());
        assert_eq!(cut1.y1(), cut2.y1());
        assert_eq!(cut1.x2(), cut2.x2());
        assert_eq!(cut1.y2(), cut2.y2());
    }

    #[test]
    fn test_hash_consistency() {
        use std::collections::HashMap;
        
        let cut1 = create_test_cut();
        let cut2 = cut1.clone();
        
        let mut map = HashMap::new();
        map.insert(cut1, "value1");
        
        // Should be able to find using the cloned cut
        assert_eq!(map.get(&cut2), Some(&"value1"));
    }

    // Builder pattern tests
    #[test]
    fn test_builder_new() {
        let builder = CutBuilder::new();
        assert_eq!(builder.get_x1(), 0);
        assert_eq!(builder.get_y1(), 0);
        assert_eq!(builder.get_x2(), 0);
        assert_eq!(builder.get_y2(), 0);
    }

    #[test]
    fn test_builder_fluent_interface() {
        let cut = CutBuilder::new()
            .x1(10)
            .y1(20)
            .x2(50)
            .y2(60)
            .original_width(100)
            .original_height(80)
            .horizontal(true)
            .cut_coord(30)
            .original_tile_id(1)
            .child1_tile_id(2)
            .child2_tile_id(3)
            .build();
        
        let expected = create_test_cut();
        assert_eq!(cut, expected);
    }

    #[test]
    fn test_builder_coordinates_method() {
        let cut = CutBuilder::new()
            .coordinates(10, 20, 50, 60)
            .original_dimensions(100, 80)
            .horizontal(true)
            .cut_coord(30)
            .original_tile_id(1)
            .child_tile_ids(2, 3)
            .build();
        
        let expected = create_test_cut();
        assert_eq!(cut, expected);
    }

    #[test]
    fn test_builder_getters() {
        let builder = CutBuilder::new()
            .x1(10)
            .y1(20)
            .x2(50)
            .y2(60)
            .original_width(100)
            .original_height(80)
            .horizontal(true)
            .cut_coord(30)
            .original_tile_id(1)
            .child1_tile_id(2)
            .child2_tile_id(3);
        
        assert_eq!(builder.get_x1(), 10);
        assert_eq!(builder.get_y1(), 20);
        assert_eq!(builder.get_x2(), 50);
        assert_eq!(builder.get_y2(), 60);
        assert_eq!(builder.get_original_width(), 100);
        assert_eq!(builder.get_original_height(), 80);
        assert!(builder.get_is_horizontal());
        assert_eq!(builder.get_cut_coord(), 30);
        assert_eq!(builder.get_original_tile_id(), 1);
        assert_eq!(builder.get_child1_tile_id(), 2);
        assert_eq!(builder.get_child2_tile_id(), 3);
    }

    #[test]
    fn test_builder_default() {
        let builder = CutBuilder::default();
        let cut = builder.build();
        
        assert_eq!(cut, Cut::default());
    }

    #[test]
    fn test_serde_serialization() {
        let cut = create_test_cut();
        
        // Test JSON serialization
        let json = serde_json::to_string(&cut).expect("Failed to serialize to JSON");
        let deserialized: Cut = serde_json::from_str(&json).expect("Failed to deserialize from JSON");
        
        assert_eq!(cut, deserialized);
    }

    #[test]
    fn test_edge_cases_zero_dimensions() {
        let cut = Cut::new(0, 0, 0, 0, 0, 0, true, 0, 1, 2, 3);
        
        assert_eq!(cut.width(), 0);
        assert_eq!(cut.height(), 0);
        assert_eq!(cut.area(), 0);
        assert_eq!(cut.length(), 0);
    }

    #[test]
    fn test_negative_coordinates() {
        let cut = Cut::new(-10, -20, -5, -15, 100, 80, false, -12, 1, 2, 3);
        
        assert_eq!(cut.width(), 5);  // |-5 - (-10)| = 5
        assert_eq!(cut.height(), 5); // |-15 - (-20)| = 5
        assert_eq!(cut.area(), 25);
        assert_eq!(cut.length(), 10); // 5 + 5 = 10
    }

    #[test]
    fn test_large_coordinates() {
        let cut = Cut::new(
            i32::MAX - 100,
            i32::MAX - 100,
            i32::MAX - 50,
            i32::MAX - 50,
            1000,
            1000,
            true,
            i32::MAX - 75,
            1,
            2,
            3,
        );
        
        assert_eq!(cut.width(), 50);
        assert_eq!(cut.height(), 50);
        assert_eq!(cut.area(), 2500);
    }
}
