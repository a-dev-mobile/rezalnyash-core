//! Tests for calculation request model
//! 
//! This module contains comprehensive unit tests for CalculationRequest, Panel, and Edge.

#[cfg(test)]
mod tests {
    use super::super::calculation_request::*;
    use crate::models::configuration::Configuration;
    use crate::models::client_info::ClientInfo;

    // CalculationRequest Tests
    #[test]
    fn test_calculation_request_new() {
        let request = CalculationRequest::new();
        
        assert!(request.configuration().is_none());
        assert!(request.panels().is_empty());
        assert!(request.stock_panels().is_empty());
        assert!(request.client_info().is_none());
    }

    #[test]
    fn test_calculation_request_with_values() {
        let config = Configuration::new();
        let panels = vec![Panel::simple(1, "100".to_string(), "200".to_string(), 5)];
        let stock_panels = vec![Panel::simple(2, "300".to_string(), "400".to_string(), 2)];
        let client_info = ClientInfo::new();

        let request = CalculationRequest::with_values(
            Some(config.clone()),
            panels.clone(),
            stock_panels.clone(),
            Some(client_info.clone()),
        );

        assert!(request.configuration().is_some());
        assert_eq!(request.panels().len(), 1);
        assert_eq!(request.stock_panels().len(), 1);
        assert!(request.client_info().is_some());
    }

    #[test]
    fn test_calculation_request_setters() {
        let mut request = CalculationRequest::new();
        let config = Configuration::new();
        let panels = vec![Panel::simple(1, "100".to_string(), "200".to_string(), 5)];
        let stock_panels = vec![Panel::simple(2, "300".to_string(), "400".to_string(), 2)];
        let client_info = ClientInfo::new();

        request.set_configuration(Some(config));
        request.set_panels(panels);
        request.set_stock_panels(stock_panels);
        request.set_client_info(Some(client_info));

        assert!(request.configuration().is_some());
        assert_eq!(request.panels().len(), 1);
        assert_eq!(request.stock_panels().len(), 1);
        assert!(request.client_info().is_some());
    }

    #[test]
    fn test_calculation_request_builder_pattern() {
        let config = Configuration::new();
        let panel1 = Panel::simple(1, "100".to_string(), "200".to_string(), 5);
        let panel2 = Panel::simple(2, "150".to_string(), "250".to_string(), 3);
        let stock_panel = Panel::simple(3, "300".to_string(), "400".to_string(), 2);
        let client_info = ClientInfo::new();

        let request = CalculationRequest::new()
            .with_configuration(config)
            .add_panel(panel1)
            .add_panel(panel2)
            .add_stock_panel(stock_panel)
            .with_client_info(client_info);

        assert!(request.configuration().is_some());
        assert_eq!(request.panels().len(), 2);
        assert_eq!(request.stock_panels().len(), 1);
        assert!(request.client_info().is_some());
    }

    #[test]
    fn test_tiles_to_string() {
        let panel1 = Panel::simple(1, "100".to_string(), "200".to_string(), 5);
        let panel2 = Panel::simple(2, "150".to_string(), "250".to_string(), 0); // count = 0, should be excluded
        let panel3 = Panel::simple(3, "120".to_string(), "180".to_string(), 3);

        let request = CalculationRequest::new()
            .add_panel(panel1)
            .add_panel(panel2)
            .add_panel(panel3);

        let result = request.tiles_to_string();
        
        // Should include panel1 and panel3, but not panel2 (count = 0)
        assert!(result.contains("[100x200]*5"));
        assert!(result.contains("[120x180]*3"));
        assert!(!result.contains("[150x250]*0"));
    }

    #[test]
    fn test_base_tiles_to_string() {
        let stock1 = Panel::simple(1, "300".to_string(), "400".to_string(), 2);
        let stock2 = Panel::simple(2, "350".to_string(), "450".to_string(), 0); // count = 0, should be excluded
        let stock3 = Panel::simple(3, "320".to_string(), "420".to_string(), 1);

        let request = CalculationRequest::new()
            .add_stock_panel(stock1)
            .add_stock_panel(stock2)
            .add_stock_panel(stock3);

        let result = request.base_tiles_to_string();
        
        // Should include stock1 and stock3, but not stock2 (count = 0)
        assert!(result.contains("[300x400]*2"));
        assert!(result.contains("[320x420]*1"));
        assert!(!result.contains("[350x450]*0"));
    }

    #[test]
    fn test_calculation_request_validation_empty_panels() {
        let request = CalculationRequest::new(); // No panels
        
        let result = request.validate();
        assert!(result.is_err());
        
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(matches!(errors[0], CalculationRequestError::EmptyPanels));
        assert!(!request.is_valid());
    }

    #[test]
    fn test_calculation_request_validation_invalid_panels() {
        let invalid_panel = Panel::simple(1, "invalid".to_string(), "200".to_string(), 5);
        let request = CalculationRequest::new().add_panel(invalid_panel);
        
        let result = request.validate();
        assert!(result.is_err());
        
        let errors = result.unwrap_err();
        assert!(!errors.is_empty());
        assert!(matches!(errors[0], CalculationRequestError::InvalidPanel { .. }));
    }

    #[test]
    fn test_calculation_request_validation_valid() {
        let valid_panel = Panel::simple(1, "100".to_string(), "200".to_string(), 5);
        let request = CalculationRequest::new().add_panel(valid_panel);
        
        assert!(request.validate().is_ok());
        assert!(request.is_valid());
    }

    // Panel Tests
    #[test]
    fn test_panel_new() {
        let edge = Edge::new(
            Some("top".to_string()),
            Some("left".to_string()),
            Some("bottom".to_string()),
            Some("right".to_string()),
        );

        let panel = Panel::new(
            1,
            "100".to_string(),
            "200".to_string(),
            5,
            "Wood".to_string(),
            true,
            1,
            Some("Test Panel".to_string()),
            Some(edge),
        );

        assert_eq!(panel.id(), 1);
        assert_eq!(panel.width(), "100");
        assert_eq!(panel.height(), "200");
        assert_eq!(panel.count(), 5);
        assert_eq!(panel.material(), "Wood");
        assert!(panel.is_enabled());
        assert_eq!(panel.orientation(), 1);
        assert_eq!(panel.label(), Some("Test Panel"));
        assert!(panel.edge().is_some());
    }

    #[test]
    fn test_panel_simple() {
        let panel = Panel::simple(1, "100".to_string(), "200".to_string(), 5);

        assert_eq!(panel.id(), 1);
        assert_eq!(panel.width(), "100");
        assert_eq!(panel.height(), "200");
        assert_eq!(panel.count(), 5);
        assert_eq!(panel.material(), "DEFAULT_MATERIAL");
        assert!(panel.is_enabled());
        assert_eq!(panel.orientation(), 0);
        assert!(panel.label().is_none());
        assert!(panel.edge().is_none());
    }

    #[test]
    fn test_panel_setters() {
        let mut panel = Panel::simple(1, "100".to_string(), "200".to_string(), 5);
        let edge = Edge::empty();

        panel.set_id(2);
        panel.set_width("150".to_string());
        panel.set_height("250".to_string());
        panel.set_count(10);
        panel.set_material(Some("Metal".to_string()));
        panel.set_enabled(false);
        panel.set_orientation(2);
        panel.set_label(Some("Updated Panel".to_string()));
        panel.set_edge(Some(edge));

        assert_eq!(panel.id(), 2);
        assert_eq!(panel.width(), "150");
        assert_eq!(panel.height(), "250");
        assert_eq!(panel.count(), 10);
        assert_eq!(panel.material(), "Metal");
        assert!(!panel.is_enabled());
        assert_eq!(panel.orientation(), 2);
        assert_eq!(panel.label(), Some("Updated Panel"));
        assert!(panel.edge().is_some());
    }

    #[test]
    fn test_panel_set_material_none() {
        let mut panel = Panel::simple(1, "100".to_string(), "200".to_string(), 5);
        let original_material = panel.material().to_string();
        
        panel.set_material(None); // Should not change material
        assert_eq!(panel.material(), original_material);
    }

    #[test]
    fn test_panel_builder_pattern() {
        let edge = Edge::empty();
        
        let panel = Panel::simple(1, "100".to_string(), "200".to_string(), 5)
            .with_id(10)
            .with_width("150".to_string())
            .with_height("250".to_string())
            .with_count(8)
            .with_material("Glass".to_string())
            .with_enabled(false)
            .with_orientation(3)
            .with_label("Builder Panel".to_string())
            .with_edge(edge);

        assert_eq!(panel.id(), 10);
        assert_eq!(panel.width(), "150");
        assert_eq!(panel.height(), "250");
        assert_eq!(panel.count(), 8);
        assert_eq!(panel.material(), "Glass");
        assert!(!panel.is_enabled());
        assert_eq!(panel.orientation(), 3);
        assert_eq!(panel.label(), Some("Builder Panel"));
        assert!(panel.edge().is_some());
    }

    #[test]
    fn test_panel_is_valid() {
        // Valid panel
        let valid_panel = Panel::simple(1, "100.5".to_string(), "200.7".to_string(), 5);
        assert!(valid_panel.is_valid());

        // Disabled panel
        let disabled_panel = Panel::simple(1, "100".to_string(), "200".to_string(), 5).with_enabled(false);
        assert!(!disabled_panel.is_valid());

        // Zero count
        let zero_count_panel = Panel::simple(1, "100".to_string(), "200".to_string(), 0);
        assert!(!zero_count_panel.is_valid());

        // Negative count
        let negative_count_panel = Panel::simple(1, "100".to_string(), "200".to_string(), -1);
        assert!(!negative_count_panel.is_valid());

        // Invalid width
        let invalid_width_panel = Panel::simple(1, "invalid".to_string(), "200".to_string(), 5);
        assert!(!invalid_width_panel.is_valid());

        // Invalid height
        let invalid_height_panel = Panel::simple(1, "100".to_string(), "invalid".to_string(), 5);
        assert!(!invalid_height_panel.is_valid());

        // Zero width
        let zero_width_panel = Panel::simple(1, "0".to_string(), "200".to_string(), 5);
        assert!(!zero_width_panel.is_valid());

        // Zero height
        let zero_height_panel = Panel::simple(1, "100".to_string(), "0".to_string(), 5);
        assert!(!zero_height_panel.is_valid());

        // Negative dimensions
        let negative_width_panel = Panel::simple(1, "-100".to_string(), "200".to_string(), 5);
        assert!(!negative_width_panel.is_valid());

        let negative_height_panel = Panel::simple(1, "100".to_string(), "-200".to_string(), 5);
        assert!(!negative_height_panel.is_valid());
    }

    #[test]
    fn test_panel_validate() {
        // Valid panel
        let valid_panel = Panel::simple(1, "100.5".to_string(), "200.7".to_string(), 5);
        assert!(valid_panel.validate().is_ok());

        // Panel with negative count
        let negative_count_panel = Panel::simple(1, "100".to_string(), "200".to_string(), -5);
        let result = negative_count_panel.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| matches!(e, PanelError::NegativeCount(-5))));

        // Panel with empty width
        let empty_width_panel = Panel::simple(1, "".to_string(), "200".to_string(), 5);
        let result = empty_width_panel.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| matches!(e, PanelError::EmptyDimension(_))));

        // Panel with invalid width
        let invalid_width_panel = Panel::simple(1, "abc".to_string(), "200".to_string(), 5);
        let result = invalid_width_panel.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| matches!(e, PanelError::InvalidDimension { .. })));

        // Panel with zero width
        let zero_width_panel = Panel::simple(1, "0".to_string(), "200".to_string(), 5);
        let result = zero_width_panel.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| matches!(e, PanelError::NonPositiveDimension { .. })));

        // Panel with multiple errors
        let multi_error_panel = Panel::simple(1, "invalid".to_string(), "-100".to_string(), -5);
        let result = multi_error_panel.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.len() >= 2); // Should have multiple errors
    }

    #[test]
    fn test_panel_dimension_parsing() {
        let panel = Panel::simple(1, "100.5".to_string(), "200.7".to_string(), 5);
        
        assert_eq!(panel.width_as_f64(), Some(100.5));
        assert_eq!(panel.height_as_f64(), Some(200.7));
        assert_eq!(panel.area(), Some(100.5 * 200.7));

        let invalid_panel = Panel::simple(1, "invalid".to_string(), "abc".to_string(), 5);
        assert_eq!(invalid_panel.width_as_f64(), None);
        assert_eq!(invalid_panel.height_as_f64(), None);
        assert_eq!(invalid_panel.area(), None);
    }

    #[test]
    fn test_panel_display() {
        let enabled_panel = Panel::simple(1, "100".to_string(), "200".to_string(), 5);
        assert_eq!(format!("{}", enabled_panel), "[100x200]*5");

        let disabled_panel = Panel::simple(1, "100".to_string(), "200".to_string(), 5).with_enabled(false);
        assert_eq!(format!("{}", disabled_panel), "[100x200]*5-disabled");
    }

    // Edge Tests
    #[test]
    fn test_edge_new() {
        let edge = Edge::new(
            Some("top".to_string()),
            Some("left".to_string()),
            Some("bottom".to_string()),
            Some("right".to_string()),
        );

        assert_eq!(edge.top(), Some("top"));
        assert_eq!(edge.left(), Some("left"));
        assert_eq!(edge.bottom(), Some("bottom"));
        assert_eq!(edge.right(), Some("right"));
    }

    #[test]
    fn test_edge_empty() {
        let edge = Edge::empty();

        assert_eq!(edge.top(), None);
        assert_eq!(edge.left(), None);
        assert_eq!(edge.bottom(), None);
        assert_eq!(edge.right(), None);
    }

    #[test]
    fn test_edge_setters() {
        let mut edge = Edge::empty();

        edge.set_top(Some("new_top".to_string()));
        edge.set_left(Some("new_left".to_string()));
        edge.set_bottom(Some("new_bottom".to_string()));
        edge.set_right(Some("new_right".to_string()));

        assert_eq!(edge.top(), Some("new_top"));
        assert_eq!(edge.left(), Some("new_left"));
        assert_eq!(edge.bottom(), Some("new_bottom"));
        assert_eq!(edge.right(), Some("new_right"));
    }

    #[test]
    fn test_edge_builder_pattern() {
        let edge = Edge::empty()
            .with_top("builder_top".to_string())
            .with_left("builder_left".to_string())
            .with_bottom("builder_bottom".to_string())
            .with_right("builder_right".to_string());

        assert_eq!(edge.top(), Some("builder_top"));
        assert_eq!(edge.left(), Some("builder_left"));
        assert_eq!(edge.bottom(), Some("builder_bottom"));
        assert_eq!(edge.right(), Some("builder_right"));
    }

    #[test]
    fn test_edge_has_any_edge() {
        let empty_edge = Edge::empty();
        assert!(!empty_edge.has_any_edge());

        let partial_edge = Edge::empty().with_top("top".to_string());
        assert!(partial_edge.has_any_edge());

        let full_edge = Edge::new(
            Some("top".to_string()),
            Some("left".to_string()),
            Some("bottom".to_string()),
            Some("right".to_string()),
        );
        assert!(full_edge.has_any_edge());
    }

    #[test]
    fn test_edge_has_all_edges() {
        let empty_edge = Edge::empty();
        assert!(!empty_edge.has_all_edges());

        let partial_edge = Edge::empty()
            .with_top("top".to_string())
            .with_left("left".to_string());
        assert!(!partial_edge.has_all_edges());

        let full_edge = Edge::new(
            Some("top".to_string()),
            Some("left".to_string()),
            Some("bottom".to_string()),
            Some("right".to_string()),
        );
        assert!(full_edge.has_all_edges());
    }

    #[test]
    fn test_edge_default() {
        let edge = Edge::default();
        assert!(!edge.has_any_edge());
        assert_eq!(edge.top(), None);
        assert_eq!(edge.left(), None);
        assert_eq!(edge.bottom(), None);
        assert_eq!(edge.right(), None);
    }

    // Error Tests
    #[test]
    fn test_calculation_request_error_display() {
        let empty_panels_error = CalculationRequestError::EmptyPanels;
        assert_eq!(
            format!("{}", empty_panels_error),
            "Calculation request must contain at least one panel"
        );

        let panel_error = CalculationRequestError::InvalidPanel {
            index: 2,
            error: PanelError::NegativeCount(-5),
        };
        assert!(format!("{}", panel_error).contains("Invalid panel at index 2"));

        let stock_panel_error = CalculationRequestError::InvalidStockPanel {
            index: 1,
            error: PanelError::EmptyDimension("width".to_string()),
        };
        assert!(format!("{}", stock_panel_error).contains("Invalid stock panel at index 1"));
    }

    #[test]
    fn test_panel_error_display() {
        let negative_count_error = PanelError::NegativeCount(-5);
        assert_eq!(
            format!("{}", negative_count_error),
            "Panel count cannot be negative: -5"
        );

        let empty_dimension_error = PanelError::EmptyDimension("width".to_string());
        assert_eq!(
            format!("{}", empty_dimension_error),
            "Panel width cannot be empty"
        );

        let invalid_dimension_error = PanelError::InvalidDimension {
            field: "height".to_string(),
            value: "abc".to_string(),
        };
        assert_eq!(
            format!("{}", invalid_dimension_error),
            "Invalid height dimension: 'abc' cannot be parsed as number"
        );

        let non_positive_dimension_error = PanelError::NonPositiveDimension {
            field: "width".to_string(),
            value: -10.5,
        };
        assert_eq!(
            format!("{}", non_positive_dimension_error),
            "Panel width must be positive: -10.5"
        );
    }

    // Integration Tests
    #[test]
    fn test_calculation_request_serialization() {
        let panel = Panel::simple(1, "100".to_string(), "200".to_string(), 5);
        let request = CalculationRequest::new().add_panel(panel);

        // Test serialization to JSON
        let json = serde_json::to_string(&request).expect("Should serialize to JSON");
        assert!(json.contains("100"));
        assert!(json.contains("200"));

        // Test deserialization from JSON
        let deserialized: CalculationRequest = serde_json::from_str(&json)
            .expect("Should deserialize from JSON");
        assert_eq!(deserialized.panels().len(), 1);
        assert_eq!(deserialized.panels()[0].width(), "100");
        assert_eq!(deserialized.panels()[0].height(), "200");
    }

    #[test]
    fn test_panel_equality() {
        let panel1 = Panel::simple(1, "100".to_string(), "200".to_string(), 5);
        let panel2 = Panel::simple(1, "100".to_string(), "200".to_string(), 5);
        let panel3 = Panel::simple(2, "100".to_string(), "200".to_string(), 5);

        assert_eq!(panel1, panel2);
        assert_ne!(panel1, panel3);
    }

    #[test]
    fn test_edge_equality() {
        let edge1 = Edge::new(
            Some("top".to_string()),
            Some("left".to_string()),
            None,
            None,
        );
        let edge2 = Edge::new(
            Some("top".to_string()),
            Some("left".to_string()),
            None,
            None,
        );
        let edge3 = Edge::new(
            Some("top".to_string()),
            Some("right".to_string()),
            None,
            None,
        );

        assert_eq!(edge1, edge2);
        assert_ne!(edge1, edge3);
    }

    #[test]
    fn test_calculation_request_clone() {
        let panel = Panel::simple(1, "100".to_string(), "200".to_string(), 5);
        let original = CalculationRequest::new().add_panel(panel);
        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert_eq!(original.panels().len(), cloned.panels().len());
    }

    #[test]
    fn test_complex_scenario() {
        // Create a complex calculation request with multiple panels and validation
        let config = Configuration::new();
        let client_info = ClientInfo::new();

        let edge = Edge::new(
            Some("veneer".to_string()),
            Some("veneer".to_string()),
            None,
            None,
        );

        let panel1 = Panel::new(
            1,
            "600.5".to_string(),
            "400.25".to_string(),
            3,
            "MDF".to_string(),
            true,
            1,
            Some("Kitchen Cabinet Door".to_string()),
            Some(edge.clone()),
        );

        let panel2 = Panel::new(
            2,
            "800".to_string(),
            "300".to_string(),
            2,
            "Plywood".to_string(),
            true,
            0,
            Some("Shelf".to_string()),
            None,
        );

        let stock_panel = Panel::new(
            100,
            "2440".to_string(),
            "1220".to_string(),
            5,
            "MDF".to_string(),
            true,
            0,
            Some("Standard Sheet".to_string()),
            None,
        );

        let request = CalculationRequest::new()
            .with_configuration(config)
            .with_client_info(client_info)
            .add_panel(panel1)
            .add_panel(panel2)
            .add_stock_panel(stock_panel);

        // Validate the request
        assert!(request.is_valid());
        assert_eq!(request.panels().len(), 2);
        assert_eq!(request.stock_panels().len(), 1);

        // Test string representations
        let tiles_string = request.tiles_to_string();
        assert!(tiles_string.contains("[600.5x400.25]*3"));
        assert!(tiles_string.contains("[800x300]*2"));

        let base_tiles_string = request.base_tiles_to_string();
        assert!(base_tiles_string.contains("[2440x1220]*5"));

        // Test individual panel properties
        let first_panel = &request.panels()[0];
        assert_eq!(first_panel.area(), Some(600.5 * 400.25));
        assert_eq!(first_panel.label(), Some("Kitchen Cabinet Door"));
        assert!(first_panel.edge().is_some());
        assert!(first_panel.edge().unwrap().has_any_edge());

        let second_panel = &request.panels()[1];
        assert_eq!(second_panel.area(), Some(800.0 * 300.0));
        assert!(second_panel.edge().is_none());
    }
}
