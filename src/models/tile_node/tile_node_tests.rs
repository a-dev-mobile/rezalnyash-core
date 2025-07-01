//! Tests for TileNode implementation

#[cfg(test)]
mod tests {
    use super::super::TileNode;
    use crate::models::TileDimensions;
    use crate::errors::CoreError;
    use std::collections::HashSet;

    #[test]
    fn test_new_tile_node() {
        let node = TileNode::new(0, 0, 100, 200).unwrap();
        
        assert_eq!(node.x1(), 0);
        assert_eq!(node.y1(), 0);
        assert_eq!(node.x2(), 100);
        assert_eq!(node.y2(), 200);
        assert_eq!(node.width(), 100);
        assert_eq!(node.height(), 200);
        assert_eq!(node.area(), 20000);
        assert_eq!(node.external_id(), -1);
        assert!(!node.is_final());
        assert!(!node.is_rotated());
        assert!(!node.has_children());
    }

    #[test]
    fn test_new_tile_node_invalid_coordinates() {
        // Invalid x coordinates (x2 <= x1)
        assert!(TileNode::new(100, 0, 100, 200).is_err());
        assert!(TileNode::new(100, 0, 50, 200).is_err());
        
        // Invalid y coordinates (y2 <= y1)
        assert!(TileNode::new(0, 200, 100, 200).is_err());
        assert!(TileNode::new(0, 200, 100, 150).is_err());
    }

    #[test]
    fn test_from_tile_dimensions() {
        let tile_dims = TileDimensions::new(
            1,
            100,
            200,
            "Wood".to_string(),
            1,
            Some("Test".to_string()),
            false,
        );
        
        let node = TileNode::from_tile_dimensions(&tile_dims).unwrap();
        
        assert_eq!(node.width(), 100);
        assert_eq!(node.height(), 200);
        assert_eq!(node.x1(), 0);
        assert_eq!(node.y1(), 0);
        assert_eq!(node.x2(), 100);
        assert_eq!(node.y2(), 200);
        assert_eq!(node.external_id(), -1);
    }

    #[test]
    fn test_from_tile_node_copy() {
        let original = TileNode::new(10, 20, 110, 220).unwrap();
        let mut original = original;
        original.set_external_id(42);
        original.set_final(true);
        original.set_rotated(true);
        
        let copy = TileNode::from_tile_node(&original);
        
        assert_eq!(copy.id(), original.id());
        assert_eq!(copy.external_id(), original.external_id());
        assert_eq!(copy.is_final(), original.is_final());
        assert_eq!(copy.is_rotated(), original.is_rotated());
        assert_eq!(copy.x1(), original.x1());
        assert_eq!(copy.y1(), original.y1());
        assert_eq!(copy.x2(), original.x2());
        assert_eq!(copy.y2(), original.y2());
    }

    #[test]
    fn test_setters_and_getters() {
        let mut node = TileNode::new(0, 0, 100, 200).unwrap();
        
        node.set_external_id(42);
        assert_eq!(node.external_id(), 42);
        
        node.set_final(true);
        assert!(node.is_final());
        
        node.set_rotated(true);
        assert!(node.is_rotated());
    }

    #[test]
    fn test_children_management() {
        let mut parent = TileNode::new(0, 0, 200, 200).unwrap();
        let child1 = TileNode::new(0, 0, 100, 200).unwrap();
        let child2 = TileNode::new(100, 0, 200, 200).unwrap();
        
        assert!(!parent.has_children());
        assert!(parent.child1().is_none());
        assert!(parent.child2().is_none());
        
        parent.set_child1(Some(child1));
        parent.set_child2(Some(child2));
        
        assert!(parent.has_children());
        assert!(parent.child1().is_some());
        assert!(parent.child2().is_some());
        
        assert_eq!(parent.child1().unwrap().width(), 100);
        assert_eq!(parent.child2().unwrap().width(), 100);
    }

    #[test]
    fn test_find_tile() {
        let mut parent = TileNode::new(0, 0, 200, 200).unwrap();
        let child1 = TileNode::new(0, 0, 100, 200).unwrap();
        let child2 = TileNode::new(100, 0, 200, 200).unwrap();
        let target_id = child1.id();
        
        parent.set_child1(Some(child1));
        parent.set_child2(Some(child2));
        
        // Find the child1 node
        let found = parent.child1().unwrap();
        let search_result = parent.find_tile(found);
        assert!(search_result.is_some());
        assert_eq!(search_result.unwrap().id(), target_id);
        
        // Try to find a non-existent node
        let non_existent = TileNode::new(300, 300, 400, 400).unwrap();
        assert!(parent.find_tile(&non_existent).is_none());
    }

    #[test]
    fn test_area_calculations() {
        let node = TileNode::new(0, 0, 100, 200).unwrap();
        assert_eq!(node.area(), 20000);
        assert_eq!(node.max_side(), 200);
        assert!(node.is_vertical());
        assert!(!node.is_horizontal());
        
        let horizontal_node = TileNode::new(0, 0, 200, 100).unwrap();
        assert!(horizontal_node.is_horizontal());
        assert!(!horizontal_node.is_vertical());
    }

    #[test]
    fn test_used_area_calculation() {
        let mut parent = TileNode::new(0, 0, 200, 200).unwrap();
        let mut child1 = TileNode::new(0, 0, 100, 200).unwrap();
        let mut child2 = TileNode::new(100, 0, 200, 200).unwrap();
        
        // Initially no area is used
        assert_eq!(parent.used_area(), 0);
        
        // Mark child1 as final (used)
        child1.set_final(true);
        parent.set_child1(Some(child1));
        parent.set_child2(Some(child2));
        
        // Now child1's area should be counted as used
        assert_eq!(parent.used_area(), 20000); // 100 * 200
        assert_eq!(parent.unused_area(), 20000); // Total 40000 - used 20000
    }

    #[test]
    fn test_final_tile_collection() {
        let mut parent = TileNode::new(0, 0, 200, 200).unwrap();
        let mut child1 = TileNode::new(0, 0, 100, 200).unwrap();
        let mut child2 = TileNode::new(100, 0, 200, 200).unwrap();
        
        child1.set_final(true);
        child2.set_final(true);
        
        parent.set_child1(Some(child1));
        parent.set_child2(Some(child2));
        
        let final_nodes = parent.final_tile_nodes();
        assert_eq!(final_nodes.len(), 2);
        
        let final_tiles = parent.final_tiles();
        assert_eq!(final_tiles.len(), 2);
        
        assert!(parent.has_final());
        assert_eq!(parent.nbr_final_tiles(), 2);
    }

    #[test]
    fn test_unused_tile_collection() {
        let mut parent = TileNode::new(0, 0, 200, 200).unwrap();
        let child1 = TileNode::new(0, 0, 100, 200).unwrap(); // Not final, so unused
        let mut child2 = TileNode::new(100, 0, 200, 200).unwrap();
        child2.set_final(true); // Final, so used
        
        parent.set_child1(Some(child1));
        parent.set_child2(Some(child2));
        
        let unused_tiles = parent.unused_tiles();
        assert_eq!(unused_tiles.len(), 1);
        assert_eq!(unused_tiles[0].width(), 100);
        
        assert_eq!(parent.nbr_unused_tiles(), 1);
    }

    #[test]
    fn test_depth_calculation() {
        let mut parent = TileNode::new(0, 0, 400, 400).unwrap();
        let mut child1 = TileNode::new(0, 0, 200, 400).unwrap();
        let child2 = TileNode::new(200, 0, 400, 400).unwrap();
        
        // Initially depth is 0 (no children)
        assert_eq!(parent.depth(), 0);
        
        parent.set_child1(Some(child1.clone()));
        parent.set_child2(Some(child2));
        
        // Now depth is 1 (one level of children)
        assert_eq!(parent.depth(), 1);
        
        // Add grandchildren to child1
        let grandchild1 = TileNode::new(0, 0, 100, 400).unwrap();
        let grandchild2 = TileNode::new(100, 0, 200, 400).unwrap();
        child1.set_child1(Some(grandchild1));
        child1.set_child2(Some(grandchild2));
        
        // Update parent with modified child1
        parent.set_child1(Some(child1));
        
        // Now depth should be 2
        assert_eq!(parent.depth(), 2);
    }

    #[test]
    fn test_biggest_area() {
        let mut parent = TileNode::new(0, 0, 300, 300).unwrap();
        let child1 = TileNode::new(0, 0, 100, 300).unwrap(); // Area: 30000, unused
        let child2 = TileNode::new(100, 0, 300, 150).unwrap(); // Area: 30000, unused
        
        parent.set_child1(Some(child1));
        parent.set_child2(Some(child2));
        
        // Both children are unused, so biggest area should be 30000
        assert_eq!(parent.biggest_area(), 30000);
    }

    #[test]
    fn test_horizontal_vertical_counts() {
        let mut parent = TileNode::new(0, 0, 300, 200).unwrap();
        let mut horizontal_child = TileNode::new(0, 0, 200, 100).unwrap(); // 200x100 (horizontal)
        let mut vertical_child = TileNode::new(200, 0, 300, 200).unwrap(); // 100x200 (vertical)
        
        horizontal_child.set_final(true);
        vertical_child.set_final(true);
        
        parent.set_child1(Some(horizontal_child));
        parent.set_child2(Some(vertical_child));
        
        assert_eq!(parent.nbr_final_horizontal(), 1);
        assert_eq!(parent.nbr_final_vertical(), 1);
    }

    #[test]
    fn test_distinct_tile_set() {
        let mut parent = TileNode::new(0, 0, 300, 200).unwrap();
        let mut child1 = TileNode::new(0, 0, 100, 200).unwrap(); // 100x200
        let mut child2 = TileNode::new(100, 0, 200, 200).unwrap(); // 100x200 (same dimensions)
        let mut child3 = TileNode::new(200, 0, 300, 200).unwrap(); // 100x200 (same dimensions)
        
        child1.set_final(true);
        child2.set_final(true);
        child3.set_final(true);
        
        parent.set_child1(Some(child1));
        parent.set_child2(Some(child2));
        
        let distinct_set = parent.distinct_tile_set();
        
        // All children have same dimensions (100x200), so should have only one unique identifier
        assert_eq!(distinct_set.len(), 1);
        
        // The identifier should be calculated using Cantor pairing: ((300 * 301) / 2) + 200
        let expected_id = ((300 * 301) / 2) + 200;
        assert!(distinct_set.contains(&expected_id));
    }

    #[test]
    fn test_to_tile_dimensions() {
        let node = TileNode::new(10, 20, 110, 220).unwrap();
        let tile_dims = node.to_tile_dimensions();
        
        assert_eq!(tile_dims.width(), 100);
        assert_eq!(tile_dims.height(), 200);
    }

    #[test]
    fn test_used_area_ratio() {
        let mut parent = TileNode::new(0, 0, 200, 200).unwrap(); // Total area: 40000
        let mut child1 = TileNode::new(0, 0, 100, 200).unwrap(); // Area: 20000
        let child2 = TileNode::new(100, 0, 200, 200).unwrap(); // Area: 20000, unused
        
        child1.set_final(true); // Mark as used
        parent.set_child1(Some(child1));
        parent.set_child2(Some(child2));
        
        // Used area ratio should be 0.5 (20000 / 40000)
        assert!((parent.used_area_ratio() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_string_representations() {
        let node = TileNode::new(10, 20, 110, 220).unwrap();
        
        let string_repr = node.append_to_string("");
        assert!(string_repr.contains("(10, 20)(110, 220)"));
        
        let identifier = node.to_string_identifier();
        assert!(identifier.contains("10"));
        assert!(identifier.contains("20"));
        assert!(identifier.contains("110"));
        assert!(identifier.contains("220"));
        assert!(identifier.contains("false")); // is_final = false
    }

    #[test]
    fn test_display_trait() {
        let node = TileNode::new(0, 0, 100, 200).unwrap();
        let display_string = format!("{}", node);
        assert!(display_string.contains("(0, 0)(100, 200)"));
    }

    #[test]
    fn test_equality() {
        let node1 = TileNode::new(0, 0, 100, 200).unwrap();
        let node2 = TileNode::from_tile_node(&node1);
        let node3 = TileNode::new(0, 0, 150, 200).unwrap();
        
        assert_eq!(node1, node2);
        assert_ne!(node1, node3);
    }

    #[test]
    fn test_replace_tile() {
        let mut parent = TileNode::new(0, 0, 200, 200).unwrap();
        let child1 = TileNode::new(0, 0, 100, 200).unwrap();
        let child2 = TileNode::new(100, 0, 200, 200).unwrap();
        let replacement = TileNode::new(0, 0, 50, 200).unwrap();
        
        parent.set_child1(Some(child1.clone()));
        parent.set_child2(Some(child2));
        
        // Replace child1 with replacement
        let replaced = parent.replace_tile(replacement, &child1);
        assert!(replaced.is_some());
        assert_eq!(replaced.unwrap().width(), 50);
        
        // Verify the replacement took place
        assert_eq!(parent.child1().unwrap().width(), 50);
    }

    #[test]
    fn test_complex_tree_operations() {
        // Create a more complex tree structure
        let mut root = TileNode::new(0, 0, 400, 400).unwrap();
        
        let mut left_branch = TileNode::new(0, 0, 200, 400).unwrap();
        let mut right_branch = TileNode::new(200, 0, 400, 400).unwrap();
        
        // Left branch children
        let mut left_left = TileNode::new(0, 0, 100, 400).unwrap();
        let mut left_right = TileNode::new(100, 0, 200, 400).unwrap();
        left_left.set_final(true);
        left_right.set_final(true);
        
        left_branch.set_child1(Some(left_left));
        left_branch.set_child2(Some(left_right));
        
        // Right branch children
        let mut right_left = TileNode::new(200, 0, 300, 400).unwrap();
        let right_right = TileNode::new(300, 0, 400, 400).unwrap(); // Unused
        right_left.set_final(true);
        
        right_branch.set_child1(Some(right_left));
        right_branch.set_child2(Some(right_right));
        
        root.set_child1(Some(left_branch));
        root.set_child2(Some(right_branch));
        
        // Test various operations on the complex tree
        assert_eq!(root.nbr_final_tiles(), 3);
        assert_eq!(root.nbr_unused_tiles(), 1);
        assert_eq!(root.depth(), 2);
        assert_eq!(root.used_area(), 120000); // 3 * 40000
        assert_eq!(root.unused_area(), 40000); // 1 * 40000
        
        let final_nodes = root.final_tile_nodes();
        assert_eq!(final_nodes.len(), 3);
        
        let unused_tiles = root.unused_tiles();
        assert_eq!(unused_tiles.len(), 1);
        assert_eq!(unused_tiles[0].x1(), 300);
    }
}
