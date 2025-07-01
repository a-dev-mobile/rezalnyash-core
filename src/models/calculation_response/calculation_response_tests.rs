//! Tests for CalculationResponse model

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::models::CalculationRequest;
    use std::collections::HashMap;

    #[test]
    fn test_calculation_response_new() {
        let response = CalculationResponse::new();
        
        assert!(response.id.is_none());
        assert!(response.task_id.is_none());
        assert_eq!(response.elapsed_time, 0);
        assert!(response.solution_elapsed_time.is_none());
        assert_eq!(response.total_used_area, 0.0);
        assert_eq!(response.total_wasted_area, 0.0);
        assert_eq!(response.total_used_area_ratio, 0.0);
        assert_eq!(response.total_nbr_cuts, 0);
        assert_eq!(response.total_cut_length, 0.0);
        assert!(response.request.is_none());
        assert!(response.panels.is_none());
        assert!(response.used_stock_panels.is_none());
        assert!(response.edge_bands.is_none());
        assert!(response.no_fit_panels.is_empty());
        assert!(response.mosaics.is_empty());
    }

    #[test]
    fn test_calculation_response_version() {
        assert_eq!(CalculationResponse::get_version(), "1.2");
    }

    #[test]
    fn test_has_solution() {
        let mut response = CalculationResponse::new();
        assert!(!response.has_solution());
        
        // Empty panels vector should still return false
        response.panels = Some(Vec::new());
        assert!(!response.has_solution());
        
        // Non-empty panels vector should return true
        let final_tile = FinalTile::with_params(1, 100.0, 200.0);
        response.panels = Some(vec![final_tile]);
        assert!(response.has_solution());
    }

    #[test]
    fn test_has_solution_all_fit() {
        let mut response = CalculationResponse::new();
        let final_tile = FinalTile::with_params(1, 100.0, 200.0);
        response.panels = Some(vec![final_tile]);
        
        // No no-fit panels should return true
        assert!(response.has_solution_all_fit());
        
        // With no-fit panels should return false
        let no_fit_tile = NoFitTile::with_params(1, 50, 50, 2);
        response.no_fit_panels.push(no_fit_tile);
        assert!(!response.has_solution_all_fit());
    }

    #[test]
    fn test_panel_counts() {
        let mut response = CalculationResponse::new();
        assert_eq!(response.get_panel_count(), 0);
        assert_eq!(response.get_mosaic_count(), 0);
        assert_eq!(response.get_no_fit_count(), 0);
        
        let final_tile = FinalTile::with_params(1, 100.0, 200.0);
        response.panels = Some(vec![final_tile.clone(), final_tile]);
        assert_eq!(response.get_panel_count(), 2);
        
        let mosaic = Mosaic::new();
        response.mosaics.push(mosaic);
        assert_eq!(response.get_mosaic_count(), 1);
        
        let no_fit_tile = NoFitTile::with_params(1, 50, 50, 3);
        response.no_fit_panels.push(no_fit_tile);
        assert_eq!(response.get_no_fit_count(), 1);
    }

    #[test]
    fn test_calculate_efficiency() {
        let mut response = CalculationResponse::new();
        
        // Zero areas should return 0.0
        assert_eq!(response.calculate_efficiency(), 0.0);
        
        // Normal case
        response.total_used_area = 80.0;
        response.total_wasted_area = 20.0;
        assert_eq!(response.calculate_efficiency(), 0.8);
        
        // Only used area
        response.total_wasted_area = 0.0;
        assert_eq!(response.calculate_efficiency(), 1.0);
    }

    #[test]
    fn test_validate_efficiency_ratio() {
        let mut response = CalculationResponse::new();
        response.total_used_area = 80.0;
        response.total_wasted_area = 20.0;
        response.total_used_area_ratio = 0.8;
        
        // Should pass validation
        assert!(response.validate().is_ok());
        
        // Should fail with inconsistent ratio
        response.total_used_area_ratio = 0.5;
        assert!(response.validate().is_err());
    }

    #[test]
    fn test_mosaic_new() {
        let mosaic = Mosaic::new();
        
        assert!(mosaic.request_stock_id.is_none());
        assert!(mosaic.stock_label.is_none());
        assert_eq!(mosaic.used_area, 0.0);
        assert_eq!(mosaic.wasted_area, 0.0);
        assert_eq!(mosaic.used_area_ratio, 0.0);
        assert_eq!(mosaic.nbr_final_panels, 0);
        assert_eq!(mosaic.nbr_wasted_panels, 0);
        assert_eq!(mosaic.cut_length, 0.0);
        assert!(mosaic.material.is_none());
        assert!(mosaic.edge_bands.is_none());
        assert!(mosaic.panels.is_none());
        assert!(mosaic.tiles.is_empty());
        assert!(mosaic.cuts.is_empty());
    }

    #[test]
    fn test_mosaic_set_material() {
        let mut mosaic = Mosaic::new();
        
        // Should set normal material
        mosaic.set_material(Some("Wood".to_string()));
        assert_eq!(mosaic.material, Some("Wood".to_string()));
        
        // Should not set default material
        mosaic.set_material(Some("DEFAULT_MATERIAL".to_string()));
        assert_eq!(mosaic.material, Some("Wood".to_string())); // Should remain unchanged
        
        // Should handle None
        mosaic.set_material(None);
        assert_eq!(mosaic.material, Some("Wood".to_string())); // Should remain unchanged
    }

    #[test]
    fn test_mosaic_areas() {
        let mut mosaic = Mosaic::new();
        mosaic.used_area = 75.0;
        mosaic.wasted_area = 25.0;
        
        assert_eq!(mosaic.get_total_area(), 100.0);
        assert_eq!(mosaic.calculate_efficiency(), 0.75);
        
        // Zero total area
        mosaic.used_area = 0.0;
        mosaic.wasted_area = 0.0;
        assert_eq!(mosaic.calculate_efficiency(), 0.0);
    }

    #[test]
    fn test_tile_new() {
        let tile = Tile::with_params(1, 10, 20, 100, 200);
        
        assert_eq!(tile.id, 1);
        assert!(tile.request_obj_id.is_none());
        assert_eq!(tile.x, 10.0);
        assert_eq!(tile.y, 20.0);
        assert_eq!(tile.width, 100.0);
        assert_eq!(tile.height, 200.0);
        assert_eq!(tile.orientation, 0);
        assert!(tile.label.is_none());
        assert!(!tile.is_final);
        assert!(!tile.has_children);
        assert!(!tile.is_rotated);
    }

    #[test]
    fn test_tile_get_area() {
        let tile = Tile::with_params(1, 0, 0, 10, 20);
        assert_eq!(tile.get_area(), 200.0);
    }

    #[test]
    fn test_cut_get_length() {
        let mut cut = CutResponse {
            x1: 0.0,
            y1: 0.0,
            x2: 10.0,
            y2: 0.0,
            cut_coord: 5.0,
            is_horizontal: true,
            original_tile_id: 1,
            original_width: 10.0,
            original_height: 10.0,
            child1_tile_id: 2,
            child2_tile_id: 3,
        };
        
        // Horizontal cut
        assert_eq!(cut.get_length(), 10.0);
        
        // Vertical cut
        cut.is_horizontal = false;
        cut.x2 = 0.0;
        cut.y2 = 15.0;
        assert_eq!(cut.get_length(), 15.0);
    }

    #[test]
    fn test_final_tile_new() {
        let tile = FinalTile::with_params(123, 50.0, 75.0);
        
        assert_eq!(tile.request_obj_id, 123);
        assert_eq!(tile.width, 50.0);
        assert_eq!(tile.height, 75.0);
        assert!(tile.label.is_none());
        assert_eq!(tile.count, 1);
    }

    #[test]
    fn test_final_tile_count_plus_plus() {
        let mut tile = FinalTile::with_params(1, 10.0, 10.0);
        
        let old_count = tile.count_plus_plus();
        assert_eq!(old_count, 1);
        assert_eq!(tile.count, 2);
        
        let old_count = tile.count_plus_plus();
        assert_eq!(old_count, 2);
        assert_eq!(tile.count, 3);
    }

    #[test]
    fn test_final_tile_get_total_area() {
        let mut tile = FinalTile::with_params(1, 10.0, 20.0);
        tile.count = 3;
        
        assert_eq!(tile.get_total_area(), 600.0); // 10 * 20 * 3
    }

    #[test]
    fn test_no_fit_tile_new() {
        let tile = NoFitTile::with_params(1, 30, 40, 5);
        
        assert_eq!(tile.id, 1);
        assert_eq!(tile.width, 30.0);
        assert_eq!(tile.height, 40.0);
        assert_eq!(tile.count, 5);
        assert!(tile.label.is_none());
        assert!(tile.material.is_none());
    }

    #[test]
    fn test_no_fit_tile_get_total_area() {
        let tile = NoFitTile::with_params(1, 15, 25, 4);
        assert_eq!(tile.get_total_area(), 1500.0); // 15 * 25 * 4
    }

    #[test]
    fn test_edge_default() {
        let edge = Edge::default();
        
        assert!(edge.top.is_none());
        assert!(edge.left.is_none());
        assert!(edge.bottom.is_none());
        assert!(edge.right.is_none());
    }

    #[test]
    fn test_calculation_response_display() {
        let mut response = CalculationResponse::new();
        response.total_used_area_ratio = 0.85;
        
        let final_tile = FinalTile::with_params(1, 100.0, 200.0);
        response.panels = Some(vec![final_tile]);
        
        let mosaic = Mosaic::new();
        response.mosaics.push(mosaic);
        
        let display_str = format!("{}", response);
        assert!(display_str.contains("panels: 1"));
        assert!(display_str.contains("mosaics: 1"));
        assert!(display_str.contains("85.00%"));
    }

    #[test]
    fn test_serialization() {
        let mut response = CalculationResponse::new();
        response.id = Some("test-response".to_string());
        response.elapsed_time = 1500;
        response.total_used_area = 85.5;
        
        // Test serialization
        let serialized = serde_json::to_string(&response).expect("Failed to serialize");
        assert!(serialized.contains("test-response"));
        assert!(serialized.contains("1500"));
        assert!(serialized.contains("85.5"));
        
        // Test deserialization
        let deserialized: CalculationResponse = serde_json::from_str(&serialized)
            .expect("Failed to deserialize");
        assert_eq!(deserialized.id, Some("test-response".to_string()));
        assert_eq!(deserialized.elapsed_time, 1500);
        assert_eq!(deserialized.total_used_area, 85.5);
    }

    #[test]
    fn test_complex_validation() {
        let mut response = CalculationResponse::new();
        
        // Create panels
        let final_tile1 = FinalTile::with_params(1, 100.0, 200.0);
        let final_tile2 = FinalTile::with_params(2, 150.0, 100.0);
        response.panels = Some(vec![final_tile1.clone(), final_tile2.clone()]);
        
        // Create mosaic with matching panel count
        let mut mosaic = Mosaic::new();
        mosaic.panels = Some(vec![final_tile1, final_tile2]);
        response.mosaics.push(mosaic);
        
        // Set consistent efficiency
        response.total_used_area = 80.0;
        response.total_wasted_area = 20.0;
        response.total_used_area_ratio = 0.8;
        
        // Should pass validation
        assert!(response.validate().is_ok());
        
        // Break panel count consistency
        response.panels = Some(vec![FinalTile::with_params(3, 50.0, 50.0)]);
        assert!(response.validate().is_err());
    }
}
