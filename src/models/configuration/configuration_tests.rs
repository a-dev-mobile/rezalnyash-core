#[cfg(test)]
mod tests {
    use super::super::configuration::{Configuration, ConfigurationError};
    use crate::models::performance_thresholds::PerformanceThresholds;
    use crate::enums::CutOrientationPreference;

    #[test]
    fn test_new_configuration() {
        let config = Configuration::new();
        
        assert_eq!(config.cut_thickness(), None);
        assert!(!config.use_single_stock_unit());
        assert_eq!(config.optimization_factor(), 1.0);
        assert_eq!(config.optimization_priority(), 0);
        assert_eq!(config.cut_orientation_preference(), CutOrientationPreference::default());
        assert_eq!(config.units(), None);
        assert!(!config.consider_orientation());
        assert_eq!(config.min_trim_dimension(), None);
        assert!(config.performance_thresholds().is_none());
    }

    #[test]
    fn test_default_configuration() {
        let config = Configuration::default();
        let new_config = Configuration::new();
        
        assert_eq!(config, new_config);
    }

    #[test]
    fn test_with_values_valid() {
        let result = Configuration::with_values(
            Some("2.5".to_string()),
            true,
            0.8,
            5,
            CutOrientationPreference::Horizontal,
            Some(1),
            true,
            Some("1.0".to_string()),
            None,
        );

        assert!(result.is_ok());
        let config = result.unwrap();
        
        assert_eq!(config.cut_thickness(), Some("2.5"));
        assert!(config.use_single_stock_unit());
        assert_eq!(config.optimization_factor(), 0.8);
        assert_eq!(config.optimization_priority(), 5);
        assert_eq!(config.cut_orientation_preference(), CutOrientationPreference::Horizontal);
        assert_eq!(config.units(), Some(1));
        assert!(config.consider_orientation());
        assert_eq!(config.min_trim_dimension(), Some("1.0"));
    }

    #[test]
    fn test_with_values_invalid_optimization_factor() {
        let result = Configuration::with_values(
            None,
            false,
            1.5, // Invalid: > 1.0
            0,
            CutOrientationPreference::default(),
            None,
            false,
            None,
            None,
        );

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ConfigurationError::InvalidOptimizationFactor(1.5));
    }

    #[test]
    fn test_with_values_invalid_negative_optimization_factor() {
        let result = Configuration::with_values(
            None,
            false,
            -0.1, // Invalid: < 0.0
            0,
            CutOrientationPreference::default(),
            None,
            false,
            None,
            None,
        );

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ConfigurationError::InvalidOptimizationFactor(-0.1));
    }

    #[test]
    fn test_getters() {
        let mut config = Configuration::new();
        config.set_cut_thickness(Some("3.0".to_string()));
        config.set_use_single_stock_unit(true);
        config.set_optimization_factor(0.5).unwrap();
        config.set_optimization_priority(10);
        config.set_cut_orientation_preference(CutOrientationPreference::Vertical);
        config.set_units(Some(2));
        config.set_consider_orientation(true);
        config.set_min_trim_dimension(Some("0.5".to_string()));

        assert_eq!(config.cut_thickness(), Some("3.0"));
        assert!(config.use_single_stock_unit());
        assert_eq!(config.optimization_factor(), 0.5);
        assert_eq!(config.optimization_priority(), 10);
        assert_eq!(config.cut_orientation_preference(), CutOrientationPreference::Vertical);
        assert_eq!(config.units(), Some(2));
        assert!(config.consider_orientation());
        assert_eq!(config.min_trim_dimension(), Some("0.5"));
    }

    #[test]
    fn test_set_cut_thickness() {
        let mut config = Configuration::new();
        
        config.set_cut_thickness(Some("2.5".to_string()));
        assert_eq!(config.cut_thickness(), Some("2.5"));
        
        config.set_cut_thickness(None);
        assert_eq!(config.cut_thickness(), None);
    }

    #[test]
    fn test_set_use_single_stock_unit() {
        let mut config = Configuration::new();
        
        config.set_use_single_stock_unit(true);
        assert!(config.use_single_stock_unit());
        
        config.set_use_single_stock_unit(false);
        assert!(!config.use_single_stock_unit());
    }

    #[test]
    fn test_set_optimization_factor_valid() {
        let mut config = Configuration::new();
        
        assert!(config.set_optimization_factor(0.0).is_ok());
        assert_eq!(config.optimization_factor(), 0.0);
        
        assert!(config.set_optimization_factor(0.5).is_ok());
        assert_eq!(config.optimization_factor(), 0.5);
        
        assert!(config.set_optimization_factor(1.0).is_ok());
        assert_eq!(config.optimization_factor(), 1.0);
    }

    #[test]
    fn test_set_optimization_factor_invalid() {
        let mut config = Configuration::new();
        
        let result = config.set_optimization_factor(1.1);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ConfigurationError::InvalidOptimizationFactor(1.1));
        
        let result = config.set_optimization_factor(-0.1);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ConfigurationError::InvalidOptimizationFactor(-0.1));
    }

    #[test]
    fn test_set_optimization_priority() {
        let mut config = Configuration::new();
        
        config.set_optimization_priority(5);
        assert_eq!(config.optimization_priority(), 5);
        
        config.set_optimization_priority(-1);
        assert_eq!(config.optimization_priority(), -1);
    }

    #[test]
    fn test_set_cut_orientation_preference() {
        let mut config = Configuration::new();
        
        config.set_cut_orientation_preference(CutOrientationPreference::Horizontal);
        assert_eq!(config.cut_orientation_preference(), CutOrientationPreference::Horizontal);
        
        config.set_cut_orientation_preference(CutOrientationPreference::Vertical);
        assert_eq!(config.cut_orientation_preference(), CutOrientationPreference::Vertical);
    }

    #[test]
    fn test_set_units() {
        let mut config = Configuration::new();
        
        config.set_units(Some(1));
        assert_eq!(config.units(), Some(1));
        
        config.set_units(None);
        assert_eq!(config.units(), None);
    }

    #[test]
    fn test_set_consider_orientation() {
        let mut config = Configuration::new();
        
        config.set_consider_orientation(true);
        assert!(config.consider_orientation());
        
        config.set_consider_orientation(false);
        assert!(!config.consider_orientation());
    }

    #[test]
    fn test_set_min_trim_dimension() {
        let mut config = Configuration::new();
        
        config.set_min_trim_dimension(Some("1.5".to_string()));
        assert_eq!(config.min_trim_dimension(), Some("1.5"));
        
        config.set_min_trim_dimension(None);
        assert_eq!(config.min_trim_dimension(), None);
    }

    #[test]
    fn test_set_performance_thresholds() {
        let mut config = Configuration::new();
        let thresholds = PerformanceThresholds::new();
        
        config.set_performance_thresholds(Some(thresholds.clone()));
        assert_eq!(config.performance_thresholds(), Some(&thresholds));
        
        config.set_performance_thresholds(None);
        assert!(config.performance_thresholds().is_none());
    }

    #[test]
    fn test_validate_valid_configuration() {
        let config = Configuration::new();
        assert!(config.validate().is_ok());
        assert!(config.is_valid());
    }

    #[test]
    fn test_validate_invalid_optimization_factor() {
        let mut config = Configuration::new();
        // Bypass setter validation to test validate method
        config.set_optimization_factor_unchecked(1.5);
        
        let result = config.validate();
        assert!(result.is_err());
        assert!(!config.is_valid());
        
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0], ConfigurationError::InvalidOptimizationFactor(1.5));
    }

    #[test]
    fn test_validate_empty_string_values() {
        let mut config = Configuration::new();
        config.set_cut_thickness_unchecked(Some("   ".to_string())); // Empty after trim
        config.set_min_trim_dimension_unchecked(Some("".to_string())); // Empty string
        
        let result = config.validate();
        assert!(result.is_err());
        assert!(!config.is_valid());
        
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 2);
        assert!(errors.contains(&ConfigurationError::EmptyStringValue("cut_thickness".to_string())));
        assert!(errors.contains(&ConfigurationError::EmptyStringValue("min_trim_dimension".to_string())));
    }

    #[test]
    fn test_validate_multiple_errors() {
        let mut config = Configuration::new();
        config.set_optimization_factor_unchecked(-0.5); // Invalid
        config.set_cut_thickness_unchecked(Some("".to_string())); // Empty
        
        let result = config.validate();
        assert!(result.is_err());
        
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 2);
        assert!(errors.contains(&ConfigurationError::InvalidOptimizationFactor(-0.5)));
        assert!(errors.contains(&ConfigurationError::EmptyStringValue("cut_thickness".to_string())));
    }

    #[test]
    fn test_display() {
        let config = Configuration::new();
        let display_string = format!("{}", config);
        
        assert!(display_string.contains("Configuration{"));
        assert!(display_string.contains("cut_thickness=None"));
        assert!(display_string.contains("use_single_stock_unit=false"));
        assert!(display_string.contains("optimization_factor=1"));
        assert!(display_string.contains("optimization_priority=0"));
        assert!(display_string.contains("units=None"));
        assert!(display_string.contains("consider_orientation=false"));
    }

    #[test]
    fn test_clone() {
        let mut config = Configuration::new();
        config.set_cut_thickness(Some("2.0".to_string()));
        config.set_use_single_stock_unit(true);
        
        let cloned_config = config.clone();
        
        assert_eq!(config, cloned_config);
        assert_eq!(cloned_config.cut_thickness(), Some("2.0"));
        assert!(cloned_config.use_single_stock_unit());
    }

    #[test]
    fn test_debug() {
        let config = Configuration::new();
        let debug_string = format!("{:?}", config);
        
        assert!(debug_string.contains("Configuration"));
        assert!(debug_string.contains("cut_thickness: None"));
        assert!(debug_string.contains("use_single_stock_unit: false"));
    }

    #[test]
    fn test_serde_serialization() {
        let mut config = Configuration::new();
        config.set_cut_thickness(Some("2.5".to_string()));
        config.set_use_single_stock_unit(true);
        config.set_optimization_factor(0.8).unwrap();
        
        // Test serialization
        let serialized = serde_json::to_string(&config).unwrap();
        assert!(serialized.contains("\"cut_thickness\":\"2.5\""));
        assert!(serialized.contains("\"use_single_stock_unit\":true"));
        assert!(serialized.contains("\"optimization_factor\":0.8"));
        
        // Performance thresholds should not be serialized (serde(skip))
        assert!(!serialized.contains("performance_thresholds"));
    }

    #[test]
    fn test_serde_deserialization() {
        let json = r#"{
            "cut_thickness": "3.0",
            "use_single_stock_unit": false,
            "optimization_factor": 0.6,
            "optimization_priority": 2,
            "cut_orientation_preference": "Horizontal",
            "units": 1,
            "consider_orientation": true,
            "min_trim_dimension": "0.5"
        }"#;
        
        let config: Configuration = serde_json::from_str(json).unwrap();
        
        assert_eq!(config.cut_thickness(), Some("3.0"));
        assert!(!config.use_single_stock_unit());
        assert_eq!(config.optimization_factor(), 0.6);
        assert_eq!(config.optimization_priority(), 2);
        assert_eq!(config.cut_orientation_preference(), CutOrientationPreference::Horizontal);
        assert_eq!(config.units(), Some(1));
        assert!(config.consider_orientation());
        assert_eq!(config.min_trim_dimension(), Some("0.5"));
        assert!(config.performance_thresholds().is_none()); // Should be None after deserialization
    }

    #[test]
    fn test_configuration_error_display() {
        let error1 = ConfigurationError::InvalidOptimizationFactor(1.5);
        assert_eq!(
            format!("{}", error1),
            "Invalid optimization factor: 1.5. Must be between 0.0 and 1.0"
        );
        
        let error2 = ConfigurationError::EmptyStringValue("test_field".to_string());
        assert_eq!(
            format!("{}", error2),
            "Empty string value for field: test_field"
        );
    }

    #[test]
    fn test_configuration_error_debug() {
        let error = ConfigurationError::InvalidOptimizationFactor(2.0);
        let debug_string = format!("{:?}", error);
        assert!(debug_string.contains("InvalidOptimizationFactor"));
        assert!(debug_string.contains("2.0"));
    }

    #[test]
    fn test_edge_case_optimization_factor_boundaries() {
        let mut config = Configuration::new();
        
        // Test exact boundaries
        assert!(config.set_optimization_factor(0.0).is_ok());
        assert!(config.set_optimization_factor(1.0).is_ok());
        
        // Test just outside boundaries
        assert!(config.set_optimization_factor(-0.000001).is_err());
        assert!(config.set_optimization_factor(1.000001).is_err());
    }

    #[test]
    fn test_string_field_whitespace_handling() {
        let mut config = Configuration::new();
        
        // Test strings with only whitespace using the unchecked setters
        config.set_cut_thickness_unchecked(Some("   \t\n   ".to_string()));
        config.set_min_trim_dimension_unchecked(Some("\t".to_string()));
        
        let result = config.validate();
        assert!(result.is_err());
        
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 2);
    }
}
