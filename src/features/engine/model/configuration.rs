use serde::{Deserialize, Serialize};

use crate::{enums::{cut_orientation_preference::CutOrientationPreference, optimization_level::OptimizationFactor, optimization_priority::OptimizationPriority, orientation::Orientation}, features::engine::model::performance_thresholds::PerformanceThresholds};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub consider_orientation: bool,
    pub cut_orientation_preference: CutOrientationPreference,
    pub cut_thickness: Option<String>,
    pub min_trim_dimension: Option<String>,
    pub optimization_factor: OptimizationFactor,
    pub optimization_priority: OptimizationPriority,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_thresholds: Option<PerformanceThresholds>,
    
    pub units: Option<i32>,
    pub use_single_stock_unit: bool,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            consider_orientation: false,
            cut_orientation_preference: CutOrientationPreference::default(),
            cut_thickness: None,
            min_trim_dimension: None,
            optimization_factor: OptimizationFactor::default(),
            optimization_priority: OptimizationPriority::default(),
            performance_thresholds: None,
            units: None,
            use_single_stock_unit: false,
        }
    }
}
