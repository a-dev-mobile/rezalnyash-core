use serde::{Deserialize, Serialize};
use std::fmt;
use crate::models::performance_thresholds::PerformanceThresholds;
use crate::enums::CutOrientationPreference;

/// Configuration struct that manages optimization settings for cutting operations.
/// 
/// This struct contains all the necessary parameters for configuring the cutting engine,
/// including optimization factors, orientation preferences, and performance thresholds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Configuration {
    /// Thickness of the cut (as string to maintain precision)
    cut_thickness: Option<String>,
    
    /// Whether to use a single stock unit for optimization
    use_single_stock_unit: bool,
    
    /// Factor used for optimization calculations (0.0 to 1.0)
    optimization_factor: f64,
    
    /// Priority level for optimization (higher values = higher priority)
    optimization_priority: i32,
    
    /// Preference for cut orientation
    cut_orientation_preference: CutOrientationPreference,
    
    /// Units used for measurements (optional)
    units: Option<i32>,
    
    /// Whether to consider orientation during optimization
    consider_orientation: bool,
    
    /// Minimum trim dimension (as string to maintain precision)
    min_trim_dimension: Option<String>,
    
    /// Performance thresholds configuration (not serialized)
    #[serde(skip)]
    performance_thresholds: Option<PerformanceThresholds>,
}

impl Configuration {
    /// Creates a new Configuration with default values
    pub fn new() -> Self {
        Self {
            cut_thickness: None,
            use_single_stock_unit: false,
            optimization_factor: 1.0,
            optimization_priority: 0,
            cut_orientation_preference: CutOrientationPreference::default(),
            units: None,
            consider_orientation: false,
            min_trim_dimension: None,
            performance_thresholds: None,
        }
    }

    /// Creates a new Configuration with specified values
    pub fn with_values(
        cut_thickness: Option<String>,
        use_single_stock_unit: bool,
        optimization_factor: f64,
        optimization_priority: i32,
        cut_orientation_preference: CutOrientationPreference,
        units: Option<i32>,
        consider_orientation: bool,
        min_trim_dimension: Option<String>,
        performance_thresholds: Option<PerformanceThresholds>,
    ) -> Result<Self, ConfigurationError> {
        // Validate optimization factor
        if !(0.0..=1.0).contains(&optimization_factor) {
            return Err(ConfigurationError::InvalidOptimizationFactor(optimization_factor));
        }

        Ok(Self {
            cut_thickness,
            use_single_stock_unit,
            optimization_factor,
            optimization_priority,
            cut_orientation_preference,
            units,
            consider_orientation,
            min_trim_dimension,
            performance_thresholds,
        })
    }

    // Getters
    pub fn cut_thickness(&self) -> Option<&str> {
        self.cut_thickness.as_deref()
    }

    pub fn use_single_stock_unit(&self) -> bool {
        self.use_single_stock_unit
    }

    pub fn optimization_factor(&self) -> f64 {
        self.optimization_factor
    }

    pub fn optimization_priority(&self) -> i32 {
        self.optimization_priority
    }

    pub fn cut_orientation_preference(&self) -> CutOrientationPreference {
        self.cut_orientation_preference
    }

    pub fn units(&self) -> Option<i32> {
        self.units
    }

    pub fn consider_orientation(&self) -> bool {
        self.consider_orientation
    }

    pub fn min_trim_dimension(&self) -> Option<&str> {
        self.min_trim_dimension.as_deref()
    }

    pub fn performance_thresholds(&self) -> Option<&PerformanceThresholds> {
        self.performance_thresholds.as_ref()
    }

    // Setters with validation
    pub fn set_cut_thickness(&mut self, cut_thickness: Option<String>) {
        self.cut_thickness = cut_thickness;
    }

    pub fn set_use_single_stock_unit(&mut self, use_single_stock_unit: bool) {
        self.use_single_stock_unit = use_single_stock_unit;
    }

    pub fn set_optimization_factor(&mut self, optimization_factor: f64) -> Result<(), ConfigurationError> {
        if !(0.0..=1.0).contains(&optimization_factor) {
            return Err(ConfigurationError::InvalidOptimizationFactor(optimization_factor));
        }
        self.optimization_factor = optimization_factor;
        Ok(())
    }

    pub fn set_optimization_priority(&mut self, optimization_priority: i32) {
        self.optimization_priority = optimization_priority;
    }

    pub fn set_cut_orientation_preference(&mut self, cut_orientation_preference: CutOrientationPreference) {
        self.cut_orientation_preference = cut_orientation_preference;
    }

    pub fn set_units(&mut self, units: Option<i32>) {
        self.units = units;
    }

    pub fn set_consider_orientation(&mut self, consider_orientation: bool) {
        self.consider_orientation = consider_orientation;
    }

    pub fn set_min_trim_dimension(&mut self, min_trim_dimension: Option<String>) {
        self.min_trim_dimension = min_trim_dimension;
    }

    pub fn set_performance_thresholds(&mut self, performance_thresholds: Option<PerformanceThresholds>) {
        self.performance_thresholds = performance_thresholds;
    }

    /// Validates the configuration and returns any validation errors
    pub fn validate(&self) -> Result<(), Vec<ConfigurationError>> {
        let mut errors = Vec::new();

        // Validate optimization factor
        if !(0.0..=1.0).contains(&self.optimization_factor) {
            errors.push(ConfigurationError::InvalidOptimizationFactor(self.optimization_factor));
        }

        // Validate cut thickness if present
        if let Some(ref thickness) = self.cut_thickness {
            if thickness.trim().is_empty() {
                errors.push(ConfigurationError::EmptyStringValue("cut_thickness".to_string()));
            }
        }

        // Validate min trim dimension if present
        if let Some(ref dimension) = self.min_trim_dimension {
            if dimension.trim().is_empty() {
                errors.push(ConfigurationError::EmptyStringValue("min_trim_dimension".to_string()));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Returns true if the configuration is valid
    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }

    // Test helper methods (only available in test builds)
    #[cfg(test)]
    pub(crate) fn set_optimization_factor_unchecked(&mut self, factor: f64) {
        self.optimization_factor = factor;
    }

    #[cfg(test)]
    pub(crate) fn set_cut_thickness_unchecked(&mut self, thickness: Option<String>) {
        self.cut_thickness = thickness;
    }

    #[cfg(test)]
    pub(crate) fn set_min_trim_dimension_unchecked(&mut self, dimension: Option<String>) {
        self.min_trim_dimension = dimension;
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Configuration{{cut_thickness={:?}, use_single_stock_unit={}, optimization_factor={}, optimization_priority={}, units={:?}, consider_orientation={}}}",
            self.cut_thickness,
            self.use_single_stock_unit,
            self.optimization_factor,
            self.optimization_priority,
            self.units,
            self.consider_orientation
        )
    }
}

/// Errors that can occur when working with Configuration
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigurationError {
    /// Invalid optimization factor (must be between 0.0 and 1.0)
    InvalidOptimizationFactor(f64),
    /// Empty string value for a field that shouldn't be empty
    EmptyStringValue(String),
}

impl fmt::Display for ConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigurationError::InvalidOptimizationFactor(factor) => {
                write!(f, "Invalid optimization factor: {}. Must be between 0.0 and 1.0", factor)
            }
            ConfigurationError::EmptyStringValue(field) => {
                write!(f, "Empty string value for field: {}", field)
            }
        }
    }
}

impl std::error::Error for ConfigurationError {}
