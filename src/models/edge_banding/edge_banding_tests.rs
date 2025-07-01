//! Comprehensive tests for EdgeBanding functionality
//! 
//! This module contains unit tests that verify all aspects of edge banding
//! calculations, including error handling, edge cases, and performance scenarios.

#[cfg(test)]
mod tests {
    use super::super::edge_banding::{EdgeBanding, EdgeBandingError};
    use crate::models::{TileNode, calculation_request::{Panel, Edge}};
    use std::collections::HashMap;

    /// Helper function to create a test tile node
    fn create_test_tile_node(id: i32, external_id: i32, width: i32, height: i32, is_rotated: bool) -> TileNode {
        let mut node = TileNode::new(0, 0, width, height).unwrap();
        node.set_external_id(external_id);
        node.set_rotated(is_rotated);
        node
    }

    /// Helper function to create a test panel with edges
    fn create_test_panel_with_edges(
        id: i32,
        width: &str,
        height: &str,
        top: Option<&str>,
        left: Option<&str>,
        bottom: Option<&str>,
        right: Option<&str>,
    ) -> Panel {
        let edge = if top.is_some() || left.is_some() || bottom.is_some() || right.is_some() {
            Some(Edge::new(
                top.map(|s| s.to_string()),
                left.map(|s| s.to_string()),
                bottom.map(|s| s.to_string()),
                right.map(|s| s.to_string()),
            ))
        } else {
            None
        };

        Panel::new(
            id,
            width.to_string(),
            height.to_string(),
            1,
            "DEFAULT_MATERIAL".to_string(),
            true,
            0,
            None,
            edge,
        )
    }

    #[test]
    fn test_calc_edge_bands_empty_inputs() {
        let result = EdgeBanding::calc_edge_bands(&[], &[], 1000.0);
        assert!(result.is_ok());
        let edge_bands = result.unwrap();
        assert!(edge_bands.is_empty());
    }

    #[test]
    fn test_calc_edge_bands_invalid_scale_factor() {
        let tile_nodes = vec![];
        let panels = vec![];

        // Test zero scale factor
        let result = EdgeBanding::calc_edge_bands(&tile_nodes, &panels, 0.0);
        assert!(matches!(result, Err(EdgeBandingError::InvalidScaleFactor(0.0))));

        // Test negative scale factor
        let result = EdgeBanding::calc_edge_bands(&tile_nodes, &panels, -1.0);
        assert!(matches!(result, Err(EdgeBandingError::InvalidScaleFactor(-1.0))));
    }

    #[test]
    fn test_calc_edge_bands_no_matching_tile_nodes() {
        let tile_nodes = vec![create_test_tile_node(1, 999, 100, 200, false)];
        let panels = vec![create_test_panel_with_edges(
            1, "100", "200", Some("edge_top"), None, None, None
        )];

        let result = EdgeBanding::calc_edge_bands(&tile_nodes, &panels, 1000.0);
        assert!(result.is_ok());
        let edge_bands = result.unwrap();
        assert!(edge_bands.is_empty()); // No matching external_id
    }

    #[test]
    fn test_calc_edge_bands_panels_without_edges() {
        let tile_nodes = vec![create_test_tile_node(1, 1, 100, 200, false)];
        let panels = vec![Panel::simple(1, "100".to_string(), "200".to_string(), 1)];

        let result = EdgeBanding::calc_edge_bands(&tile_nodes, &panels, 1000.0);
        assert!(result.is_ok());
        let edge_bands = result.unwrap();
        assert!(edge_bands.is_empty()); // No edge information
    }

    #[test]
    fn test_calc_edge_bands_single_panel_all_edges() {
        let tile_nodes = vec![create_test_tile_node(1, 1, 1000, 2000, false)];
        let panels = vec![create_test_panel_with_edges(
            1, "1000", "2000",
            Some("top_edge"),
            Some("left_edge"),
            Some("bottom_edge"),
            Some("right_edge"),
        )];

        let result = EdgeBanding::calc_edge_bands(&tile_nodes, &panels, 1000.0);
        assert!(result.is_ok());
        let edge_bands = result.unwrap();

        assert_eq!(edge_bands.len(), 4);
        assert_eq!(edge_bands.get("top_edge"), Some(&1.0)); // width / scale_factor = 1000 / 1000
        assert_eq!(edge_bands.get("bottom_edge"), Some(&1.0));
        assert_eq!(edge_bands.get("left_edge"), Some(&2.0)); // height / scale_factor = 2000 / 1000
        assert_eq!(edge_bands.get("right_edge"), Some(&2.0));
    }

    #[test]
    fn test_calc_edge_bands_rotated_tile() {
        let tile_nodes = vec![create_test_tile_node(1, 1, 1000, 2000, true)]; // Rotated
        let panels = vec![create_test_panel_with_edges(
            1, "1000", "2000",
            Some("top_edge"),
            Some("left_edge"),
            Some("bottom_edge"),
            Some("right_edge"),
        )];

        let result = EdgeBanding::calc_edge_bands(&tile_nodes, &panels, 1000.0);
        assert!(result.is_ok());
        let edge_bands = result.unwrap();

        assert_eq!(edge_bands.len(), 4);
        // When rotated: effective_width = height, effective_height = width
        assert_eq!(edge_bands.get("top_edge"), Some(&2.0)); // height / scale_factor = 2000 / 1000
        assert_eq!(edge_bands.get("bottom_edge"), Some(&2.0));
        assert_eq!(edge_bands.get("left_edge"), Some(&1.0)); // width / scale_factor = 1000 / 1000
        assert_eq!(edge_bands.get("right_edge"), Some(&1.0));
    }

    #[test]
    fn test_calc_edge_bands_multiple_panels_same_edge_type() {
        let tile_nodes = vec![
            create_test_tile_node(1, 1, 1000, 2000, false),
            create_test_tile_node(2, 2, 1500, 1000, false),
        ];
        let panels = vec![
            create_test_panel_with_edges(1, "1000", "2000", Some("common_edge"), None, None, None),
            create_test_panel_with_edges(2, "1500", "1000", Some("common_edge"), None, None, None),
        ];

        let result = EdgeBanding::calc_edge_bands(&tile_nodes, &panels, 1000.0);
        assert!(result.is_ok());
        let edge_bands = result.unwrap();

        assert_eq!(edge_bands.len(), 1);
        // Should sum: (1000 / 1000) + (1500 / 1000) = 1.0 + 1.5 = 2.5
        assert_eq!(edge_bands.get("common_edge"), Some(&2.5));
    }

    #[test]
    fn test_calc_edge_bands_partial_edges() {
        let tile_nodes = vec![create_test_tile_node(1, 1, 1000, 2000, false)];
        let panels = vec![create_test_panel_with_edges(
            1, "1000", "2000",
            Some("top_edge"),
            None, // No left edge
            Some("bottom_edge"),
            None, // No right edge
        )];

        let result = EdgeBanding::calc_edge_bands(&tile_nodes, &panels, 1000.0);
        assert!(result.is_ok());
        let edge_bands = result.unwrap();

        assert_eq!(edge_bands.len(), 2);
        assert_eq!(edge_bands.get("top_edge"), Some(&1.0));
        assert_eq!(edge_bands.get("bottom_edge"), Some(&1.0));
        assert!(!edge_bands.contains_key("left_edge"));
        assert!(!edge_bands.contains_key("right_edge"));
    }

    #[test]
    fn test_calc_edge_bands_different_scale_factors() {
        let tile_nodes = vec![create_test_tile_node(1, 1, 1000, 2000, false)];
        let panels = vec![create_test_panel_with_edges(
            1, "1000", "2000", Some("edge"), None, None, None
        )];

        // Test with scale factor 1000
        let result1 = EdgeBanding::calc_edge_bands(&tile_nodes, &panels, 1000.0);
        assert!(result1.is_ok());
        assert_eq!(result1.unwrap().get("edge"), Some(&1.0));

        // Test with scale factor 100
        let result2 = EdgeBanding::calc_edge_bands(&tile_nodes, &panels, 100.0);
        assert!(result2.is_ok());
        assert_eq!(result2.unwrap().get("edge"), Some(&10.0));

        // Test with scale factor 10000
        let result3 = EdgeBanding::calc_edge_bands(&tile_nodes, &panels, 10000.0);
        assert!(result3.is_ok());
        assert_eq!(result3.unwrap().get("edge"), Some(&0.1));
    }

    #[test]
    fn test_calc_edge_bands_with_validation_panel_not_found() {
        let tile_nodes = vec![create_test_tile_node(1, 999, 1000, 2000, false)]; // external_id = 999
        let panels = vec![Panel::new(
            1, // panel id = 1, but tile node external_id = 999
            "1000".to_string(),
            "2000".to_string(),
            1,
            "DEFAULT_MATERIAL".to_string(),
            true,
            0,
            None,
            Some(Edge::new(Some("edge".to_string()), None, None, None)),
        )];

        let result = EdgeBanding::calc_edge_bands_with_validation(&tile_nodes, &panels, 1000.0);
        assert!(matches!(
            result,
            Err(EdgeBandingError::PanelNotFound(1))
        ));
    }
    #[test]
    fn test_calc_edge_bands_with_validation_valid_case() {
        let tile_nodes = vec![create_test_tile_node(1, 1, 1000, 2000, false)];
        let panels = vec![Panel::new(
            1,
            "1000".to_string(),
            "2000".to_string(),
            1,
            "DEFAULT_MATERIAL".to_string(),
            true,
            0,
            None,
            Some(Edge::new(Some("edge".to_string()), None, None, None)),
        )];

        let result = EdgeBanding::calc_edge_bands_with_validation(&tile_nodes, &panels, 1000.0);
        assert!(result.is_ok());
        let edge_bands = result.unwrap();
        assert_eq!(edge_bands.get("edge"), Some(&1.0));
    }

    #[test]
    fn test_calc_single_panel_edge_bands() {
        let tile_node = create_test_tile_node(1, 1, 1000, 2000, false);
        let panel = create_test_panel_with_edges(
            1, "1000", "2000",
            Some("top"),
            Some("left"),
            None,
            None,
        );

        let result = EdgeBanding::calc_single_panel_edge_bands(&tile_node, &panel, 1000.0);
        assert!(result.is_ok());
        let edge_bands = result.unwrap();

        assert_eq!(edge_bands.len(), 2);
        assert_eq!(edge_bands.get("top"), Some(&1.0));
        assert_eq!(edge_bands.get("left"), Some(&2.0));
    }

    #[test]
    fn test_calc_single_panel_edge_bands_invalid_scale_factor() {
        let tile_node = create_test_tile_node(1, 1, 1000, 2000, false);
        let panel = create_test_panel_with_edges(1, "1000", "2000", Some("edge"), None, None, None);

        let result = EdgeBanding::calc_single_panel_edge_bands(&tile_node, &panel, 0.0);
        assert!(matches!(result, Err(EdgeBandingError::InvalidScaleFactor(0.0))));
    }

    #[test]
    fn test_calc_single_panel_edge_bands_no_edges() {
        let tile_node = create_test_tile_node(1, 1, 1000, 2000, false);
        let panel = Panel::simple(1, "1000".to_string(), "2000".to_string(), 1);

        let result = EdgeBanding::calc_single_panel_edge_bands(&tile_node, &panel, 1000.0);
        assert!(result.is_ok());
        let edge_bands = result.unwrap();
        assert!(edge_bands.is_empty());
    }

    #[test]
    fn test_total_edge_band_length() {
        let mut edge_bands = HashMap::new();
        edge_bands.insert("type1".to_string(), 10.5);
        edge_bands.insert("type2".to_string(), 20.3);
        edge_bands.insert("type3".to_string(), 30.2);

        let total = EdgeBanding::total_edge_band_length(&edge_bands);
        assert!((total - 61.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_total_edge_band_length_empty() {
        let edge_bands = HashMap::new();
        let total = EdgeBanding::total_edge_band_length(&edge_bands);
        assert_eq!(total, 0.0);
    }

    #[test]
    fn test_get_edge_types() {
        let mut edge_bands = HashMap::new();
        edge_bands.insert("type_a".to_string(), 10.0);
        edge_bands.insert("type_b".to_string(), 20.0);
        edge_bands.insert("type_c".to_string(), 30.0);

        let mut types = EdgeBanding::get_edge_types(&edge_bands);
        types.sort(); // Sort for consistent testing

        assert_eq!(types.len(), 3);
        assert!(types.contains(&"type_a".to_string()));
        assert!(types.contains(&"type_b".to_string()));
        assert!(types.contains(&"type_c".to_string()));
    }

    #[test]
    fn test_get_edge_types_empty() {
        let edge_bands = HashMap::new();
        let types = EdgeBanding::get_edge_types(&edge_bands);
        assert!(types.is_empty());
    }

    #[test]
    fn test_filter_by_min_length() {
        let mut edge_bands = HashMap::new();
        edge_bands.insert("small".to_string(), 5.0);
        edge_bands.insert("medium".to_string(), 15.0);
        edge_bands.insert("large".to_string(), 25.0);
        edge_bands.insert("exact".to_string(), 10.0);

        let filtered = EdgeBanding::filter_by_min_length(&edge_bands, 10.0);
        assert_eq!(filtered.len(), 3);
        assert!(filtered.contains_key("medium"));
        assert!(filtered.contains_key("large"));
        assert!(filtered.contains_key("exact"));
        assert!(!filtered.contains_key("small"));

        // Test with higher threshold
        let filtered_high = EdgeBanding::filter_by_min_length(&edge_bands, 20.0);
        assert_eq!(filtered_high.len(), 1);
        assert!(filtered_high.contains_key("large"));
    }

    #[test]
    fn test_filter_by_min_length_empty() {
        let edge_bands = HashMap::new();
        let filtered = EdgeBanding::filter_by_min_length(&edge_bands, 10.0);
        assert!(filtered.is_empty());
    }

    #[test]
    fn test_filter_by_min_length_none_qualify() {
        let mut edge_bands = HashMap::new();
        edge_bands.insert("small1".to_string(), 1.0);
        edge_bands.insert("small2".to_string(), 2.0);

        let filtered = EdgeBanding::filter_by_min_length(&edge_bands, 10.0);
        assert!(filtered.is_empty());
    }

    #[test]
    fn test_process_edge_side_division_by_zero() {
        // This test verifies that division by zero is properly handled
        // Note: This is testing internal logic, but the public API should prevent this
        let mut edge_bands: HashMap<String, f64> = HashMap::new();
        
        // The process_edge_side method is private, so we test through public API
        let tile_nodes = vec![create_test_tile_node(1, 1, 1000, 2000, false)];
        let panels = vec![create_test_panel_with_edges(
            1, "1000", "2000", Some("edge"), None, None, None
        )];

        // This should be caught by the scale factor validation
        let result = EdgeBanding::calc_edge_bands(&tile_nodes, &panels, 0.0);
        assert!(matches!(result, Err(EdgeBandingError::InvalidScaleFactor(0.0))));
    }

    #[test]
    fn test_complex_scenario_multiple_panels_and_rotations() {
        let tile_nodes = vec![
            create_test_tile_node(1, 1, 1000, 2000, false),  // Not rotated
            create_test_tile_node(2, 2, 1500, 1000, true),   // Rotated
            create_test_tile_node(3, 3, 2000, 2000, false),  // Square, not rotated
        ];

        let panels = vec![
            create_test_panel_with_edges(
                1, "1000", "2000",
                Some("wood_edge"),
                Some("metal_edge"),
                Some("wood_edge"),
                Some("metal_edge"),
            ),
            create_test_panel_with_edges(
                2, "1500", "1000",
                Some("wood_edge"),
                None,
                Some("plastic_edge"),
                Some("metal_edge"),
            ),
            create_test_panel_with_edges(
                3, "2000", "2000",
                Some("premium_edge"),
                Some("premium_edge"),
                Some("premium_edge"),
                Some("premium_edge"),
            ),
        ];

        let result = EdgeBanding::calc_edge_bands(&tile_nodes, &panels, 1000.0);
        assert!(result.is_ok());
        let edge_bands = result.unwrap();

        // Panel 1 (not rotated): width=1000, height=2000
        // top/bottom: wood_edge += 1.0 each = 2.0 total from this panel
        // left/right: metal_edge += 2.0 each = 4.0 total from this panel

        // Panel 2 (rotated): original width=1500, height=1000, but rotated so effective width=1000, height=1500
        // top: wood_edge += 1.0 (total wood_edge = 2.0 + 1.0 = 3.0)
        // bottom: plastic_edge += 1.0
        // right: metal_edge += 1.5 (total metal_edge = 4.0 + 1.5 = 5.5)

        // Panel 3 (square, not rotated): width=2000, height=2000
        // all sides: premium_edge += 2.0 each = 8.0 total

        assert_eq!(edge_bands.get("wood_edge"), Some(&3.0));
        assert_eq!(edge_bands.get("metal_edge"), Some(&5.5));
        assert_eq!(edge_bands.get("plastic_edge"), Some(&1.0));
        assert_eq!(edge_bands.get("premium_edge"), Some(&8.0));
    }

    #[test]
    fn test_precision_with_small_dimensions() {
        let tile_nodes = vec![create_test_tile_node(1, 1, 1, 1, false)];
        let panels = vec![create_test_panel_with_edges(
            1, "1", "1", Some("tiny_edge"), None, None, None
        )];

        let result = EdgeBanding::calc_edge_bands(&tile_nodes, &panels, 1000.0);
        assert!(result.is_ok());
        let edge_bands = result.unwrap();

        assert_eq!(edge_bands.get("tiny_edge"), Some(&0.001));
    }

    #[test]
    fn test_precision_with_large_dimensions() {
        let tile_nodes = vec![create_test_tile_node(1, 1, 1000000, 2000000, false)];
        let panels = vec![create_test_panel_with_edges(
            1, "1000000", "2000000", Some("huge_edge"), None, None, None
        )];

        let result = EdgeBanding::calc_edge_bands(&tile_nodes, &panels, 1000.0);
        assert!(result.is_ok());
        let edge_bands = result.unwrap();

        assert_eq!(edge_bands.get("huge_edge"), Some(&1000.0));
    }

    #[test]
    fn test_edge_banding_error_display() {
        let error1 = EdgeBandingError::InvalidScaleFactor(-5.0);
        assert_eq!(
            format!("{}", error1),
            "Invalid scale factor: -5 (must be positive)"
        );

        let error2 = EdgeBandingError::PanelNotFound(42);
        assert_eq!(
            format!("{}", error2),
            "Panel with ID 42 not found in tile nodes"
        );

        let error3 = EdgeBandingError::DivisionByZero;
        assert_eq!(
            format!("{}", error3),
            "Division by zero in edge banding calculations"
        );

        let error4 = EdgeBandingError::InvalidPanelDimensions {
            panel_id: 42,
            width: "invalid".to_string(),
            height: "200".to_string(),
        };
        assert_eq!(
            format!("{}", error4),
            "Invalid dimensions for panel 42: width='invalid', height='200'"
        );
    }

    #[test]
    fn test_edge_banding_error_debug() {
        let error = EdgeBandingError::InvalidScaleFactor(0.0);
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("InvalidScaleFactor"));
        assert!(debug_str.contains("0.0"));
    }

    #[test]
    fn test_edge_banding_error_clone_and_eq() {
        let error1 = EdgeBandingError::InvalidScaleFactor(1.5);
        let error2 = error1.clone();
        assert_eq!(error1, error2);

        let error3 = EdgeBandingError::DivisionByZero;
        assert_ne!(error1, error3);
    }


    
}
