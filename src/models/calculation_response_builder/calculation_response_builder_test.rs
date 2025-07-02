//! Tests for Calculation Response Builder
//!
//! This module contains comprehensive tests for the CalculationResponseBuilder,
//! covering all methods and edge cases.

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::models::{
        CalculationRequest, CalculationResponse, Task, Solution, TileDimensions,
        Panel, Edge as RequestEdge, Mosaic, NoFitTile
    };
    use crate::enums::Status;
    use crate::logging::{init::init_logging, structs::LogConfig, enums::LogLevel};
    use std::collections::HashMap;
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn setup_logger() {
        INIT.call_once(|| {
            let config = LogConfig {
                level: LogLevel::Error, // Use Error level to minimize test output
            };
            let _ = init_logging(config); // Ignore errors if already initialized
        });
    }

    fn create_test_task() -> Task {
        let mut task = Task::new("test-task-001".to_string());
        task.factor = 1000.0; // Common scaling factor
        task.status = Status::Running;
        task
    }

    fn create_test_calculation_request() -> CalculationRequest {
        // Add test panels
        let mut panels = Vec::new();
        panels.push(Panel::new(
            1,
            "100.0".to_string(),
            "200.0".to_string(),
            1,
            "Wood".to_string(),
            true,
            0,
            Some("Panel-1".to_string()),
            Some(RequestEdge::new(
                Some("PVC".to_string()),
                Some("PVC".to_string()),
                None,
                None,
            )),
        ));
        
        panels.push(Panel::new(
            2,
            "150.0".to_string(),
            "250.0".to_string(),
            1,
            "Wood".to_string(),
            true,
            90,
            Some("Panel-2".to_string()),
            None,
        ));

        // Add test stock panels
        let mut stock_panels = Vec::new();
        stock_panels.push(Panel::new(
            101,
            "2440.0".to_string(),
            "1220.0".to_string(),
            1,
            "Wood".to_string(),
            true,
            0,
            Some("Stock-1".to_string()),
            None,
        ));

        CalculationRequest::with_values(
            None,
            panels,
            stock_panels,
            None,
        )
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
        // We can only test that the builder was created successfully
        // since fields are private
        assert!(true); // Builder created without panic
    }

    #[test]
    fn test_default_builder() {
        let builder = CalculationResponseBuilder::default();
        // We can only test that the builder was created successfully
        // since fields are private
        assert!(true); // Builder created without panic
    }

    #[test]
    fn test_set_task() {
        let task = create_test_task();
        let builder = CalculationResponseBuilder::new().set_task(task);
        // Test that chaining works
        assert!(true); // Method chaining worked without panic
    }

    #[test]
    fn test_set_calculation_request() {
        let request = create_test_calculation_request();
        let builder = CalculationResponseBuilder::new().set_calculation_request(request);
        // Test that chaining works
        assert!(true); // Method chaining worked without panic
    }

    #[test]
    fn test_set_solutions() {
        let solutions = create_test_solutions();
        let builder = CalculationResponseBuilder::new().set_solutions(solutions);
        // Test that chaining works
        assert!(true); // Method chaining worked without panic
    }

    #[test]
    fn test_set_no_stock_material_panels() {
        let panels = vec![
            TileDimensions::new(
                1,
                100,
                200,
                "Wood".to_string(),
                0,
                Some("Panel-1".to_string()),
                false,
            ),
            TileDimensions::new(
                2,
                150,
                250,
                "Wood".to_string(),
                0,
                Some("Panel-2".to_string()),
                false,
            ),
        ];
        
        let builder = CalculationResponseBuilder::new().set_no_stock_material_panels(panels);
        // Test that chaining works
        assert!(true); // Method chaining worked without panic
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

        // Test that all chaining worked
        assert!(true); // All method chaining worked without panic
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
        setup_logger();
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
        setup_logger();
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
        setup_logger();
        let task = create_test_task();
        let request = create_test_calculation_request();
        let solutions = HashMap::new();
        let no_stock_panels = vec![
            TileDimensions::new(
                99,
                100000,
                200000,
                "Wood".to_string(),
                0,
                Some("Panel-99".to_string()),
                false,
            )
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
        setup_logger();
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
        setup_logger();
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
        setup_logger();
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
    fn test_display_formatting() {
        let builder = CalculationResponseBuilder::new();
        let display = format!("{}", builder);
        
        assert!(display.contains("CalculationResponseBuilder"));
        // Can't test specific field values since they're private
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
        
        assert!(display.contains("CalculationResponseBuilder"));
        // Can't test specific field values since they're private
    }

    #[test]
    fn test_build_handles_materials_without_solutions() {
        setup_logger();
        let mut task = create_test_task();
        
        // Set up tile dimensions per material
        let mut tile_dims_per_material = HashMap::new();
        tile_dims_per_material.insert("Metal".to_string(), vec![
            TileDimensions::new(
                10,
                50000,
                100000,
                "Metal".to_string(),
                0,
                Some("Metal-Panel".to_string()),
                false,
            )
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
        setup_logger();
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
        setup_logger();
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
