//! Tests for Calculation Response Builder
//!
//! This module contains comprehensive tests for the CalculationResponseBuilder,
//! covering all methods and edge cases.

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::models::{
        CalculationRequest, CalculationResponse, Task, Solution, TileDimensions,
        Panel, Edge as RequestEdge, Mosaic, NoFitTile, FinalTile
    };
    use crate::enums::Status;
    use std::collections::HashMap;

    fn create_test_task() -> Task {
        let mut task = Task::new("test-task-001".to_string());
        task.factor = 1000.0; // Common scaling factor
        task.status = Status::Running;
        task
    }

    fn create_test_calculation_request() -> CalculationRequest {
        let mut request = CalculationRequest::new();
        
        // Add test panels
        let mut panels = Vec::new();
        panels.push(Panel {
            id: 1,
            width: Some(100.0),
            height: Some(200.0),
            label: Some("Panel-1".to_string()),
            material: Some("Wood".to_string()),
            orientation: Some(0),
            edge: Some(RequestEdge {
                top: Some("PVC".to_string()),
                left: Some("PVC".to_string()),
                bottom: None,
                right: None,
            }),
            ..Default::default()
        });
        
        panels.push(Panel {
            id: 2,
            width: Some(150.0),
            height: Some(250.0),
            label: Some("Panel-2".to_string()),
            material: Some("Wood".to_string()),
            orientation: Some(90),
            edge: None,
            ..Default::default()
        });

        request.panels = Some(panels);

        // Add test stock panels
        let mut stock_panels = Vec::new();
        stock_panels.push(Panel {
            id: 101,
            width: Some(2440.0),
            height: Some(1220.0),
            label: Some("Stock-1".to_string()),
            material: Some("Wood".to_string()),
            orientation: Some(0),
            edge: None,
            ..Default::default()
        });

        request.stock_panels = Some(stock_panels);
        request
    }

    fn create_test_solutions() -> HashMap<String, Vec<Solution>> {
        let mut solutions = HashMap::new();
        
        // Create a solution with response
        let mut response = CalculationResponse::new();
        response.id = Some("123".to_string());
        response.solution_elapsed_time = Some(5000);
        response.total_used_area = 20000.0;
        response.total_wasted_area = 5000.0;
        
        // Add a test mosaic
        let mut mosaic = Mosaic::new();
        mosaic.request_stock_id = Some(101);
        mosaic.used_area = 20000.0;
        mosaic.wasted_area = 5000.0;
        mosaic.used_area_ratio = 0.8;
        mosaic.material = Some("Wood".to_string());
        
        response.mosaics.push(mosaic);
        
        let solution = Solution {
            material: "Wood".to_string(),
            score: 85.5,
            efficiency: 0.8,
            response: Some(response),
        };
        
        solutions.insert("Wood".to_string(), vec![solution]);
        solutions
    }

    #[test]
    fn test_new_builder() {
        let builder = CalculationResponseBuilder::new();
        assert!(builder.task.is_none());
        assert!(builder.calculation_request.is_none());
        assert!(builder.solutions.is_none());
        assert!(builder.no_stock_material_panels.is_none());
    }

    #[test]
    fn test_default_builder() {
        let builder = CalculationResponseBuilder::default();
        assert!(builder.task.is_none());
        assert!(builder.calculation_request.is_none());
        assert!(builder.solutions.is_none());
        assert!(builder.no_stock_material_panels.is_none());
    }

    #[test]
    fn test_set_task() {
        let task = create_test_task();
        let task_id = task.id.clone();
        
        let builder = CalculationResponseBuilder::new().set_task(task);
        
        assert!(builder.task.is_some());
        assert_eq!(builder.task.unwrap().id, task_id);
    }

    #[test]
    fn test_set_calculation_request() {
        let request = create_test_calculation_request();
        let panel_count = request.panels.as_ref().unwrap().len();
        
        let builder = CalculationResponseBuilder::new().set_calculation_request(request);
        
        assert!(builder.calculation_request.is_some());
        assert_eq!(
            builder.calculation_request.unwrap().panels.unwrap().len(),
            panel_count
        );
    }

    #[test]
    fn test_set_solutions() {
        let solutions = create_test_solutions();
        let material_count = solutions.len();
        
        let builder = CalculationResponseBuilder::new().set_solutions(solutions);
        
        assert!(builder.solutions.is_some());
        assert_eq!(builder.solutions.unwrap().len(), material_count);
    }

    #[test]
    fn test_set_no_stock_material_panels() {
        let panels = vec![
            TileDimensions {
                id: 1,
                width: 100.0,
                height: 200.0,
                ..Default::default()
            },
            TileDimensions {
                id: 2,
                width: 150.0,
                height: 250.0,
                ..Default::default()
            },
        ];
        let panel_count = panels.len();
        
        let builder = CalculationResponseBuilder::new().set_no_stock_material_panels(panels);
        
        assert!(builder.no_stock_material_panels.is_some());
        assert_eq!(builder.no_stock_material_panels.unwrap().len(), panel_count);
    }

    #[test]
    fn test_builder_chaining() {
        let task = create_test_task();
        let request = create_test_calculation_request();
        let solutions = create_test_solutions();
        let panels = vec![TileDimensions::default()];

        let builder = CalculationResponseBuilder::new()
            .set_task(task)
            .set_calculation_request(request)
            .set_solutions(solutions)
            .set_no_stock_material_panels(panels);

        assert!(builder.task.is_some());
        assert!(builder.calculation_request.is_some());
        assert!(builder.solutions.is_some());
        assert!(builder.no_stock_material_panels.is_some());
    }

    #[test]
    fn test_build_missing_task() {
        let request = create_test_calculation_request();
        
        let builder = CalculationResponseBuilder::new()
            .set_calculation_request(request);
        
        let result = builder.build();
        assert!(result.is_err());
        
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("Task is required"));
    }

    #[test]
    fn test_build_missing_calculation_request() {
        let task = create_test_task();
        
        let builder = CalculationResponseBuilder::new()
            .set_task(task);
        
        let result = builder.build();
        assert!(result.is_err());
        
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("CalculationRequest is required"));
    }

    #[test]
    fn test_build_success_empty_solutions() {
        let task = create_test_task();
        let request = create_test_calculation_request();
        let solutions = HashMap::new();

        let builder = CalculationResponseBuilder::new()
            .set_task(task)
            .set_calculation_request(request)
            .set_solutions(solutions);

        let result = builder.build();
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.task_id, Some("test-task-001".to_string()));
        assert!(response.mosaics.is_empty());
        assert!(response.no_fit_panels.is_empty());
    }

    #[test]
    fn test_build_success_with_solutions() {
        let task = create_test_task();
        let request = create_test_calculation_request();
        let solutions = create_test_solutions();

        let builder = CalculationResponseBuilder::new()
            .set_task(task)
            .set_calculation_request(request)
            .set_solutions(solutions);

        let result = builder.build();
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.task_id, Some("test-task-001".to_string()));
        assert!(!response.mosaics.is_empty());
        assert_eq!(response.total_used_area, 20.0); // Scaled by factor 1000
        assert_eq!(response.total_wasted_area, 5.0); // Scaled by factor 1000
    }

    #[test]
    fn test_build_with_no_stock_material_panels() {
        let task = create_test_task();
        let request = create_test_calculation_request();
        let solutions = HashMap::new();
        let no_stock_panels = vec![
            TileDimensions {
                id: 99,
                width: 100000.0, // Will be scaled down by factor 1000
                height: 200000.0,
                ..Default::default()
            }
        ];

        let builder = CalculationResponseBuilder::new()
            .set_task(task)
            .set_calculation_request(request)
            .set_solutions(solutions)
            .set_no_stock_material_panels(no_stock_panels);

        let result = builder.build();
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.no_fit_panels.len(), 1);
        assert_eq!(response.no_fit_panels[0].id, 99);
        assert_eq!(response.no_fit_panels[0].width, 100.0); // Scaled down
        assert_eq!(response.no_fit_panels[0].height, 200.0); // Scaled down
    }

    #[test]
    fn test_build_calculates_totals_correctly() {
        let task = create_test_task();
        let request = create_test_calculation_request();
        let solutions = create_test_solutions();

        let builder = CalculationResponseBuilder::new()
            .set_task(task)
            .set_calculation_request(request)
            .set_solutions(solutions);

        let result = builder.build();
        assert!(result.is_ok());

        let response = result.unwrap();
        
        // Check that totals are calculated
        assert!(response.total_used_area > 0.0);
        assert!(response.total_wasted_area > 0.0);
        assert!(response.total_used_area_ratio > 0.0);
        assert!(response.total_used_area_ratio <= 1.0);
        
        // Check efficiency calculation
        let expected_ratio = response.total_used_area / 
            (response.total_used_area + response.total_wasted_area);
        assert!((response.total_used_area_ratio - expected_ratio).abs() < 0.001);
    }

    #[test]
    fn test_build_sets_elapsed_time() {
        let mut task = create_test_task();
        task.end_time = task.start_time + 10000; // 10 seconds elapsed
        
        let request = create_test_calculation_request();
        let solutions = HashMap::new();

        let builder = CalculationResponseBuilder::new()
            .set_task(task)
            .set_calculation_request(request)
            .set_solutions(solutions);

        let result = builder.build();
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.elapsed_time, 10000);
    }

    #[test]
    fn test_build_sets_solution_elapsed_time() {
        let mut task = create_test_task();
        task.start_time = 1000;
        
        let request = create_test_calculation_request();
        let solutions = create_test_solutions();

        let builder = CalculationResponseBuilder::new()
            .set_task(task)
            .set_calculation_request(request)
            .set_solutions(solutions);

        let result = builder.build();
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.solution_elapsed_time.is_some());
        assert_eq!(response.solution_elapsed_time.unwrap(), 4000); // 5000 - 1000
    }

    #[test]
    fn test_add_no_fit_tile_new() {
        let task = create_test_task();
        let request = create_test_calculation_request();
        let mut response = CalculationResponse::new();
        
        let tile_dim = TileDimensions {
            id: 1,
            width: 100000.0, // Will be scaled by factor 1000
            height: 200000.0,
            ..Default::default()
        };

        let builder = CalculationResponseBuilder::new()
            .set_task(task)
            .set_calculation_request(request);

        let result = builder.add_no_fit_tile(&mut response, &tile_dim, &builder.calculation_request.as_ref().unwrap());
        assert!(result.is_ok());

        assert_eq!(response.no_fit_panels.len(), 1);
        assert_eq!(response.no_fit_panels[0].id, 1);
        assert_eq!(response.no_fit_panels[0].width, 100.0); // Scaled down
        assert_eq!(response.no_fit_panels[0].height, 200.0); // Scaled down
        assert_eq!(response.no_fit_panels[0].count, 1);
        assert_eq!(response.no_fit_panels[0].label, Some("Panel-1".to_string()));
        assert_eq!(response.no_fit_panels[0].material, Some("Wood".to_string()));
    }

    #[test]
    fn test_add_no_fit_tile_existing() {
        let task = create_test_task();
        let request = create_test_calculation_request();
        let mut response = CalculationResponse::new();
        
        // Add initial no-fit tile
        response.no_fit_panels.push(NoFitTile {
            id: 1,
            width: 100.0,
            height: 200.0,
            count: 1,
            label: Some("Panel-1".to_string()),
            material: Some("Wood".to_string()),
        });

        let tile_dim = TileDimensions {
            id: 1, // Same ID as existing
            width: 100000.0,
            height: 200000.0,
            ..Default::default()
        };

        let builder = CalculationResponseBuilder::new()
            .set_task(task)
            .set_calculation_request(request);

        let result = builder.add_no_fit_tile(&mut response, &tile_dim, &builder.calculation_request.as_ref().unwrap());
        assert!(result.is_ok());

        // Should still have only one tile but with incremented count
        assert_eq!(response.no_fit_panels.len(), 1);
        assert_eq!(response.no_fit_panels[0].count, 2); // Incremented
    }

    #[test]
    fn test_display_formatting() {
        let builder = CalculationResponseBuilder::new();
        let display = format!("{}", builder);
        
        assert!(display.contains("CalculationResponseBuilder"));
        assert!(display.contains("task: false"));
        assert!(display.contains("request: false"));
        assert!(display.contains("solutions: 0"));
    }

    #[test]
    fn test_display_with_data() {
        let task = create_test_task();
        let request = create_test_calculation_request();
        let solutions = create_test_solutions();

        let builder = CalculationResponseBuilder::new()
            .set_task(task)
            .set_calculation_request(request)
            .set_solutions(solutions);

        let display = format!("{}", builder);
        
        assert!(display.contains("task: true"));
        assert!(display.contains("request: true"));
        assert!(display.contains("solutions: 1"));
    }

    #[test]
    fn test_build_handles_materials_without_solutions() {
        let mut task = create_test_task();
        
        // Set up tile dimensions per material
        let mut tile_dims_per_material = HashMap::new();
        tile_dims_per_material.insert("Metal".to_string(), vec![
            TileDimensions {
                id: 10,
                width: 50000.0,
                height: 100000.0,
                ..Default::default()
            }
        ]);
        task.tile_dimensions_per_material = Some(tile_dims_per_material);
        
        let request = create_test_calculation_request();
        
        // Solutions only for Wood, not Metal
        let mut solutions = HashMap::new();
        solutions.insert("Metal".to_string(), Vec::new()); // Empty solutions for Metal

        let builder = CalculationResponseBuilder::new()
            .set_task(task)
            .set_calculation_request(request)
            .set_solutions(solutions);

        let result = builder.build();
        assert!(result.is_ok());

        let response = result.unwrap();
        // Should have no-fit tiles for Metal material
        assert_eq!(response.no_fit_panels.len(), 1);
        assert_eq!(response.no_fit_panels[0].id, 10);
    }

    #[test]
    fn test_solution_id_generation() {
        let task = create_test_task();
        let request = create_test_calculation_request();
        let solutions = create_test_solutions();

        let builder = CalculationResponseBuilder::new()
            .set_task(task)
            .set_calculation_request(request)
            .set_solutions(solutions);

        let result = builder.build();
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.id.is_some());
        
        // ID should be generated from solution IDs hash
        let id = response.id.unwrap();
        assert!(!id.is_empty());
        assert!(id.parse::<i32>().is_ok());
    }

    #[test]
    fn test_efficiency_ratio_calculation() {
        let task = create_test_task();
        let request = create_test_calculation_request();
        let solutions = create_test_solutions();

        let builder = CalculationResponseBuilder::new()
            .set_task(task)
            .set_calculation_request(request)
            .set_solutions(solutions);

        let result = builder.build();
        assert!(result.is_ok());

        let response = result.unwrap();
        
        // Verify efficiency ratio is calculated correctly
        let expected_ratio = response.total_used_area / 
            (response.total_used_area + response.total_wasted_area);
        assert!((response.total_used_area_ratio - expected_ratio).abs() < 0.001);
        
        // Should be between 0 and 1
        assert!(response.total_used_area_ratio >= 0.0);
        assert!(response.total_used_area_ratio <= 1.0);
    }
}
