use std::fmt;

  use serde::{Deserialize, Serialize};

  /// Enum representing different optimization priorities for the cut list optimization algorithm.
  /// 
  /// The optimization priority determines the order of criteria used when evaluating and 
  /// ranking cut list solutions during the optimization process.
  #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
  pub enum OptimizationPriority {
      /// Priority 0: Focuses on minimizing material waste
      /// Order: Most tiles → Least wasted area → Least number of cuts
      MaterialEfficiency,

      /// Priority != 0: Focuses on minimizing cutting operations
      /// Order: Most tiles → Least number of cuts → Least wasted area
      CuttingEfficiency,
  }

  impl OptimizationPriority {
      /// Returns the numeric priority value used by the Java engine
      pub fn value(&self) -> u8 {
          match self {
              OptimizationPriority::MaterialEfficiency => 0,
              OptimizationPriority::CuttingEfficiency => 1,
          }
      }

     
  }

  impl Default for OptimizationPriority {
      fn default() -> Self {
          OptimizationPriority::MaterialEfficiency
      }
  }

  