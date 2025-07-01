#[cfg(test)]
mod tests {
    use crate::models::TileNode;
    use crate::models::mosaic::Mosaic;
    use crate::models::tile_dimensions::TileDimensions;
    use crate::models::cut::Cut;
    use crate::errors::core_errors::CoreError;

    fn create_test_tile_dimensions() -> TileDimensions {
        TileDimensions::new(
            1,
            100,
            200,
            "Wood".to_string(),
            1,
            Some("Test Tile".to_string()),
            false,
        )
    }

    fn create_test_cut() -> Cut {
        // Using the actual Cut constructor based on the existing Cut structure
        Cut::new(
            0,      // x1
            0,      // y1
            100,    // x2
            50,     // y2
            100,    // original_width
            200,    // original_height
            true,   // is_horizontal
            50,     // cut_coord
            1,      // original_tile_id
            2,      // child1_tile_id
            3,      // child2_tile_id
        )
    }

    #[test]
    fn test_tile_node_creation() {
        let tile_dims = create_test_tile_dimensions();
        let tile_node = TileNode::from_tile_dimensions(&tile_dims).unwrap();

        assert_eq!(tile_node.external_id(), -1);
        assert_eq!(tile_node.x1(), 0);
        assert_eq!(tile_node.y1(), 0);
        assert_eq!(tile_node.width(), 100);
        assert_eq!(tile_node.height(), 200);
        assert_eq!(tile_node.area(), 20000);
    }

    #[test]
    fn test_tile_node_new() {
        let tile_node = TileNode::new(10, 20, 50, 100).unwrap();

        assert_eq!(tile_node.external_id(), -1);
        assert_eq!(tile_node.x1(), 10);
        assert_eq!(tile_node.y1(), 20);
        assert_eq!(tile_node.width(), 40);
        assert_eq!(tile_node.height(), 80);
        assert_eq!(tile_node.area(), 3200);
    }

    #[test]
    fn test_tile_node_copy() {
        let original = TileNode::new(10, 20, 50, 100).unwrap();
        let copy = TileNode::from_tile_node(&original);

        assert_eq!(original, copy);
        assert_eq!(copy.x1(), 10);
        assert_eq!(copy.y1(), 20);
        assert_eq!(copy.width(), 40);
        assert_eq!(copy.height(), 80);
    }

    #[test]
    fn test_tile_node_external_id() {
        let mut tile_node = TileNode::new(0, 0, 100, 200).unwrap();
        assert_eq!(tile_node.external_id(), -1);

        tile_node.set_external_id(42);
        assert_eq!(tile_node.external_id(), 42);
    }

    #[test]
    fn test_tile_node_area() {
        let tile_node = TileNode::new(0, 0, 25, 40).unwrap();
        assert_eq!(tile_node.area(), 1000); // 25 * 40 = 1000
    }

    #[test]
    fn test_tile_node_final_nodes_leaf() {
        let tile_node = TileNode::new(0, 0, 100, 200).unwrap();
        let final_nodes = tile_node.final_tile_nodes();
        
        assert_eq!(final_nodes.len(), 0); // Not final by default
    }

    #[test]
    fn test_tile_node_used_area_unused() {
        let tile_node = TileNode::new(0, 0, 100, 200).unwrap();
        assert_eq!(tile_node.used_area(), 0);
        assert_eq!(tile_node.unused_area(), 20000);
    }

    #[test]
    fn test_tile_node_depth_leaf() {
        let tile_node = TileNode::new(0, 0, 100, 200).unwrap();
        assert_eq!(tile_node.depth(), 0);
    }

    #[test]
    fn test_tile_node_unused_tiles() {
        let tile_node = TileNode::new(0, 0, 100, 200).unwrap();
        let unused_tiles = tile_node.unused_tiles();
        
        assert_eq!(unused_tiles.len(), 1);
        assert_eq!(unused_tiles[0].area(), 20000);
    }

    #[test]
    fn test_tile_node_biggest_area() {
        let tile_node = TileNode::new(0, 0, 100, 200).unwrap();
        assert_eq!(tile_node.biggest_area(), 20000);
    }

    #[test]
    fn test_tile_node_distinct_tile_set_empty() {
        let tile_node = TileNode::new(0, 0, 100, 200).unwrap();
        let distinct_set = tile_node.distinct_tile_set();
        assert!(distinct_set.is_empty());
    }

    #[test]
    fn test_mosaic_from_tile_dimensions() {
        let tile_dims = create_test_tile_dimensions();
        let mosaic = Mosaic::from_tile_dimensions(&tile_dims).unwrap();

        assert_eq!(mosaic.stock_id(), 1);
        assert_eq!(mosaic.material(), "Wood");
        assert_eq!(mosaic.orientation(), 1);
        assert_eq!(mosaic.nbr_cuts(), 0);
        assert_eq!(mosaic.root_tile_node().width(), 100);
        assert_eq!(mosaic.root_tile_node().height(), 200);
    }

    #[test]
    fn test_mosaic_from_tile_node() {
        let tile_node = TileNode::new(0, 0, 150, 250).unwrap();
        let mosaic = Mosaic::from_tile_node(&tile_node, "Metal".to_string());

        assert_eq!(mosaic.stock_id(), -1);
        assert_eq!(mosaic.material(), "Metal");
        assert_eq!(mosaic.orientation(), 0);
        assert_eq!(mosaic.nbr_cuts(), 0);
        assert_eq!(mosaic.root_tile_node().width(), 150);
        assert_eq!(mosaic.root_tile_node().height(), 250);
    }

    #[test]
    fn test_mosaic_copy() {
        let tile_dims = create_test_tile_dimensions();
        let original = Mosaic::from_tile_dimensions(&tile_dims).unwrap();
        let copy = Mosaic::from_mosaic(&original);

        assert_eq!(original, copy);
        assert_eq!(copy.stock_id(), 1);
        assert_eq!(copy.material(), "Wood");
        assert_eq!(copy.orientation(), 1);
    }

    #[test]
    fn test_mosaic_setters() {
        let tile_dims = create_test_tile_dimensions();
        let mut mosaic = Mosaic::from_tile_dimensions(&tile_dims).unwrap();

        mosaic.set_stock_id(42);
        assert_eq!(mosaic.stock_id(), 42);

        mosaic.set_material("Plastic".to_string());
        assert_eq!(mosaic.material(), "Plastic");

        mosaic.set_orientation(90);
        assert_eq!(mosaic.orientation(), 90);
    }

    #[test]
    fn test_mosaic_cuts_management() {
        let tile_dims = create_test_tile_dimensions();
        let mut mosaic = Mosaic::from_tile_dimensions(&tile_dims).unwrap();
        let cut = create_test_cut();

        assert_eq!(mosaic.nbr_cuts(), 0);
        assert!(mosaic.cuts().is_empty());

        mosaic.add_cut(cut.clone());
        assert_eq!(mosaic.nbr_cuts(), 1);
        assert_eq!(mosaic.cuts().len(), 1);

        let cuts = vec![cut.clone(), cut.clone()];
        mosaic.set_cuts(cuts);
        assert_eq!(mosaic.nbr_cuts(), 2);
    }

    #[test]
    fn test_mosaic_remove_cut() {
        let tile_dims = create_test_tile_dimensions();
        let mut mosaic = Mosaic::from_tile_dimensions(&tile_dims).unwrap();
        let cut = create_test_cut();

        mosaic.add_cut(cut.clone());
        assert_eq!(mosaic.nbr_cuts(), 1);

        let removed_cut = mosaic.remove_cut(0).unwrap();
        assert_eq!(removed_cut, cut);
        assert_eq!(mosaic.nbr_cuts(), 0);
    }

    #[test]
    fn test_mosaic_remove_cut_invalid_index() {
        let tile_dims = create_test_tile_dimensions();
        let mut mosaic = Mosaic::from_tile_dimensions(&tile_dims).unwrap();

        let result = mosaic.remove_cut(0);
        assert!(result.is_err());
        
        match result {
            Err(CoreError::InvalidInput { details }) => {
                assert!(details.contains("out of bounds"));
            }
            _ => panic!("Expected InvalidInput error"),
        }
    }

    #[test]
    fn test_mosaic_root_tile_node_operations() {
        let tile_dims = create_test_tile_dimensions();
        let mut mosaic = Mosaic::from_tile_dimensions(&tile_dims).unwrap();
        let new_root = TileNode::new(0, 0, 300, 400).unwrap();

        assert_eq!(mosaic.root_tile_node().width(), 100);
        assert_eq!(mosaic.root_tile_node().height(), 200);

        mosaic.set_root_tile_node(new_root);
        assert_eq!(mosaic.root_tile_node().width(), 300);
        assert_eq!(mosaic.root_tile_node().height(), 400);
    }

    #[test]
    fn test_mosaic_final_tile_nodes() {
        let tile_dims = create_test_tile_dimensions();
        let mosaic = Mosaic::from_tile_dimensions(&tile_dims).unwrap();
        let final_nodes = mosaic.final_tile_nodes();

        assert_eq!(final_nodes.len(), 0); // No final tiles by default
    }

    #[test]
    fn test_mosaic_hv_diff() {
        let tile_dims = create_test_tile_dimensions();
        let mosaic = Mosaic::from_tile_dimensions(&tile_dims).unwrap();
        
        // Since nbr_final_horizontal and nbr_final_vertical return 0,
        // hv_diff should be 0.0
        assert_eq!(mosaic.hv_diff(), 0.0);
    }

    #[test]
    fn test_mosaic_distinct_tile_set() {
        let tile_dims = create_test_tile_dimensions();
        let mosaic = Mosaic::from_tile_dimensions(&tile_dims).unwrap();
        let distinct_set = mosaic.distinct_tile_set();

        // Should be empty since the root tile is not marked as used
        assert!(distinct_set.is_empty());
    }

    #[test]
    fn test_mosaic_area_calculations() {
        let tile_dims = create_test_tile_dimensions();
        let mosaic = Mosaic::from_tile_dimensions(&tile_dims).unwrap();

        assert_eq!(mosaic.used_area(), 0); // No tiles marked as used
        assert_eq!(mosaic.unused_area(), 20000); // 100 * 200
        assert_eq!(mosaic.biggest_area(), 20000);
    }

    #[test]
    fn test_mosaic_depth() {
        let tile_dims = create_test_tile_dimensions();
        let mosaic = Mosaic::from_tile_dimensions(&tile_dims).unwrap();

        assert_eq!(mosaic.depth(), 0); // Single root node, no children
    }

    #[test]
    fn test_mosaic_biggest_unused_tile() {
        let tile_dims = create_test_tile_dimensions();
        let mosaic = Mosaic::from_tile_dimensions(&tile_dims).unwrap();
        let biggest_unused = mosaic.biggest_unused_tile();

        assert!(biggest_unused.is_some());
        let tile = biggest_unused.unwrap();
        assert_eq!(tile.area(), 20000);
    }

    #[test]
    fn test_mosaic_center_of_mass_distance_zero_used_area() {
        let tile_dims = create_test_tile_dimensions();
        let mosaic = Mosaic::from_tile_dimensions(&tile_dims).unwrap();

        // Since no tiles are marked as used, used_area is 0
        assert_eq!(mosaic.center_of_mass_distance_to_origin(), 0.0);
    }

    #[test]
    fn test_mosaic_validate() {
        let tile_dims = create_test_tile_dimensions();
        let mosaic = Mosaic::from_tile_dimensions(&tile_dims).unwrap();

        let result = mosaic.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_mosaic_equality() {
        let tile_dims = create_test_tile_dimensions();
        let mosaic1 = Mosaic::from_tile_dimensions(&tile_dims).unwrap();
        let mosaic2 = Mosaic::from_tile_dimensions(&tile_dims).unwrap();

        assert_eq!(mosaic1, mosaic2);
    }

    #[test]
    fn test_mosaic_inequality_different_root() {
        let tile_dims1 = create_test_tile_dimensions();
        let tile_dims2 = TileDimensions::new(
            2,
            150,
            250,
            "Metal".to_string(),
            2,
            Some("Test Tile 2".to_string()),
            false,
        );
        
        let mosaic1 = Mosaic::from_tile_dimensions(&tile_dims1).unwrap();
        let mosaic2 = Mosaic::from_tile_dimensions(&tile_dims2).unwrap();

        assert_ne!(mosaic1, mosaic2);
    }

    #[test]
    fn test_tile_node_coordinates() {
        let tile_node = TileNode::new(15, 25, 100, 200).unwrap();

        assert_eq!(tile_node.x1(), 15);
        assert_eq!(tile_node.y1(), 25);
    }

    #[test]
    fn test_tile_node_dimensions() {
        let tile_node = TileNode::new(0, 0, 123, 678).unwrap();

        assert_eq!(tile_node.width(), 123);
        assert_eq!(tile_node.height(), 678);
    }

    #[test]
    fn test_mosaic_cuts_slice() {
        let tile_dims = create_test_tile_dimensions();
        let mut mosaic = Mosaic::from_tile_dimensions(&tile_dims).unwrap();
        let cut1 = create_test_cut();
        let cut2 = create_test_cut();

        mosaic.add_cut(cut1);
        mosaic.add_cut(cut2);

        let cuts_slice = mosaic.cuts();
        assert_eq!(cuts_slice.len(), 2);
    }

    #[test]
    fn test_tile_node_area_calculation() {
        let tile_node = TileNode::new(0, 0, 10, 20).unwrap();
        // 10 * 20 = 200
        assert_eq!(tile_node.area(), 200);
    }

    #[test]
    fn test_mosaic_material_reference() {
        let tile_dims = create_test_tile_dimensions();
        let mosaic = Mosaic::from_tile_dimensions(&tile_dims).unwrap();

        // Test that material() returns a string slice reference
        let material_ref: &str = mosaic.material();
        assert_eq!(material_ref, "Wood");
    }

    #[test]
    fn test_tile_node_from_tile_dimensions_properties() {
        let tile_dims = TileDimensions::new(
            99,
            55,
            77,
            "Glass".to_string(),
            2,
            Some("Glass Tile".to_string()),
            false,
        );
        let mut tile_node = TileNode::from_tile_dimensions(&tile_dims).unwrap();
        tile_node.set_external_id(99);

        assert_eq!(tile_node.external_id(), 99);
        assert_eq!(tile_node.x1(), 0);
        assert_eq!(tile_node.y1(), 0);
        assert_eq!(tile_node.width(), 55);
        assert_eq!(tile_node.height(), 77);
    }

    #[test]
    fn test_mosaic_from_tile_dimensions_orientation_conversion() {
        let tile_dims = TileDimensions::new(
            1,
            100,
            200,
            "Wood".to_string(),
            2,
            Some("Vertical Tile".to_string()),
            false,
        );
        let mosaic = Mosaic::from_tile_dimensions(&tile_dims).unwrap();

        assert_eq!(mosaic.orientation(), 2);
    }

    #[test]
    fn test_tile_node_final_functionality() {
        let mut tile_node = TileNode::new(0, 0, 100, 200).unwrap();
        
        assert!(!tile_node.is_final());
        tile_node.set_final(true);
        assert!(tile_node.is_final());
        
        let final_nodes = tile_node.final_tile_nodes();
        assert_eq!(final_nodes.len(), 1);
        assert_eq!(final_nodes[0].area(), 20000);
    }

    #[test]
    fn test_tile_node_children() {
        let mut parent = TileNode::new(0, 0, 200, 200).unwrap();
        let child1 = TileNode::new(0, 0, 100, 200).unwrap();
        let child2 = TileNode::new(100, 0, 200, 200).unwrap();
        
        assert!(!parent.has_children());
        
        parent.set_child1(Some(child1));
        parent.set_child2(Some(child2));
        
        assert!(parent.has_children());
        assert!(parent.child1().is_some());
        assert!(parent.child2().is_some());
    }

    #[test]
    fn test_mosaic_additional_methods() {
        let tile_dims = create_test_tile_dimensions();
        let mosaic = Mosaic::from_tile_dimensions(&tile_dims).unwrap();

        assert_eq!(mosaic.total_area(), 20000);
        assert_eq!(mosaic.used_area_ratio(), 0.0);
        assert_eq!(mosaic.nbr_final_tiles(), 0);
        assert_eq!(mosaic.nbr_unused_tiles(), 1);
        assert!(!mosaic.has_final());
        
        let unused_tiles = mosaic.unused_tiles();
        assert_eq!(unused_tiles.len(), 1);
    }
}
