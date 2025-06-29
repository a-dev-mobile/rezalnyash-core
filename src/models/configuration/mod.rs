use serde::{Deserialize, Serialize};

use crate::{
    enums::cut_orientation_preference::CutOrientationPreference,
    models::performance_thresholds::PerformanceThresholds, services::computation::OptimizationPriority,
};

#[derive(Debug, Clone)]
pub struct Configuration {
    /// Thickness of the cutting blade (kerf)
    pub cut_thickness: f64,

    /// Minimum trim dimension (waste edge)
    pub min_trim_dimension: f64,

    /// Whether to consider grain orientation
    pub consider_orientation: bool,

    /// Optimization accuracy factor (1-10, higher = more accurate but slower)
    pub optimization_factor: f64,

    /// Primary optimization goal
    pub optimization_priority: Vec<OptimizationPriority>,

    /// Whether to use only single stock unit per solution
    pub use_single_stock_unit: bool,

    /// Performance constraints
    pub performance_thresholds: PerformanceThresholds,

    pub cut_orientation_preference: CutOrientationPreference,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            cut_thickness: 3.0,
            min_trim_dimension: 10.0,
            consider_orientation: false,
            optimization_factor: 5.0,
            optimization_priority: vec![OptimizationPriority::default()],
            use_single_stock_unit: false,
            performance_thresholds: PerformanceThresholds::default(),
            cut_orientation_preference: CutOrientationPreference::default(),
        }
    }
}
