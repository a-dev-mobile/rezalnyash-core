use std::fmt;

use serde::{Deserialize, Serialize};

/// Enum representing different optimization levels for the cut list optimization algorithm.
/// 
/// The optimization factor controls the size of the solution pool and affects the balance
/// between computation time and solution quality.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum OptimizationFactor {

    Fast,

    Standard,

    High,
    

    Ultra,
  
    Custom(f64),
}

impl OptimizationFactor {
    /// Returns the numeric optimization factor value
    pub fn value(&self) -> f64 {
        match self {
            OptimizationFactor::Fast => 0.5,
            OptimizationFactor::Standard => 1.0,
            OptimizationFactor::High => 2.0,
            OptimizationFactor::Ultra => 3.0,
            OptimizationFactor::Custom(factor) => *factor,
        }
    }
    
}

impl Default for OptimizationFactor {
    fn default() -> Self {
        OptimizationFactor::Standard
    }
}
