//! Optimization priority enumeration for solution comparison
//!
//! This module defines the OptimizationPriority enum which represents
//! different criteria for optimizing and comparing cutting solutions.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Enumeration of optimization priorities for solution comparison
///
/// Each variant represents a different optimization criterion that can be used
/// to compare and rank cutting solutions. The priorities determine which
/// aspects of a solution are most important for the optimization process.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OptimizationPriority {
    /// Prioritize solutions with the most tiles
    MostTiles,
    /// Prioritize solutions with the least wasted area
    LeastWastedArea,
    /// Prioritize solutions with the least number of cuts
    LeastNbrCuts,
    /// Prioritize solutions with the most horizontal-vertical discrepancy
    MostHvDiscrepancy,
    /// Prioritize solutions with the biggest unused tile area
    BiggestUnusedTileArea,
    /// Prioritize solutions with the smallest center of mass distance to origin
    SmallestCenterOfMassDistToOrigin,
    /// Prioritize solutions with the least number of mosaics
    LeastNbrMosaics,
    /// Prioritize solutions with the least number of unused tiles
    LeastNbrUnusedTiles,
    /// Prioritize solutions with the most unused panel area
    MostUnusedPanelArea,
}

impl OptimizationPriority {
    /// Returns all optimization priority variants
    ///
    /// # Returns
    /// A vector containing all possible optimization priority values
    ///
    /// # Examples
    /// ```
    /// use rezalnyas_core::models::solution_comparator::OptimizationPriority;
    ///
    /// let all_priorities = OptimizationPriority::all();
    /// assert_eq!(all_priorities.len(), 9);
    /// ```
    pub fn all() -> Vec<OptimizationPriority> {
        vec![
            OptimizationPriority::MostTiles,
            OptimizationPriority::LeastWastedArea,
            OptimizationPriority::LeastNbrCuts,
            OptimizationPriority::MostHvDiscrepancy,
            OptimizationPriority::BiggestUnusedTileArea,
            OptimizationPriority::SmallestCenterOfMassDistToOrigin,
            OptimizationPriority::LeastNbrMosaics,
            OptimizationPriority::LeastNbrUnusedTiles,
            OptimizationPriority::MostUnusedPanelArea,
        ]
    }

    /// Converts a string to an OptimizationPriority
    ///
    /// # Arguments
    /// * `s` - The string to parse (case-insensitive)
    ///
    /// # Returns
    /// Some(OptimizationPriority) if the string matches a known priority,
    /// None otherwise
    ///
    /// # Examples
    /// ```
    /// use rezalnyas_core::models::solution_comparator::OptimizationPriority;
    ///
    /// let priority = OptimizationPriority::from_str("MOST_TILES");
    /// assert_eq!(priority, Some(OptimizationPriority::MostTiles));
    ///
    /// let priority = OptimizationPriority::from_str("most_tiles");
    /// assert_eq!(priority, Some(OptimizationPriority::MostTiles));
    ///
    /// let priority = OptimizationPriority::from_str("invalid");
    /// assert_eq!(priority, None);
    /// ```
    pub fn from_str(s: &str) -> Option<OptimizationPriority> {
        match s.to_uppercase().as_str() {
            "MOST_TILES" => Some(OptimizationPriority::MostTiles),
            "LEAST_WASTED_AREA" => Some(OptimizationPriority::LeastWastedArea),
            "LEAST_NBR_CUTS" => Some(OptimizationPriority::LeastNbrCuts),
            "MOST_HV_DISCREPANCY" => Some(OptimizationPriority::MostHvDiscrepancy),
            "BIGGEST_UNUSED_TILE_AREA" => Some(OptimizationPriority::BiggestUnusedTileArea),
            "SMALLEST_CENTER_OF_MASS_DIST_TO_ORIGIN" => {
                Some(OptimizationPriority::SmallestCenterOfMassDistToOrigin)
            }
            "LEAST_NBR_MOSAICS" => Some(OptimizationPriority::LeastNbrMosaics),
            "LEAST_NBR_UNUSED_TILES" => Some(OptimizationPriority::LeastNbrUnusedTiles),
            "MOST_UNUSED_PANEL_AREA" => Some(OptimizationPriority::MostUnusedPanelArea),
            _ => None,
        }
    }

    /// Converts the optimization priority to its string representation
    ///
    /// # Returns
    /// The string representation of the optimization priority
    ///
    /// # Examples
    /// ```
    /// use rezalnyas_core::models::solution_comparator::OptimizationPriority;
    ///
    /// let priority = OptimizationPriority::MostTiles;
    /// assert_eq!(priority.as_str(), "MOST_TILES");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            OptimizationPriority::MostTiles => "MOST_TILES",
            OptimizationPriority::LeastWastedArea => "LEAST_WASTED_AREA",
            OptimizationPriority::LeastNbrCuts => "LEAST_NBR_CUTS",
            OptimizationPriority::MostHvDiscrepancy => "MOST_HV_DISCREPANCY",
            OptimizationPriority::BiggestUnusedTileArea => "BIGGEST_UNUSED_TILE_AREA",
            OptimizationPriority::SmallestCenterOfMassDistToOrigin => {
                "SMALLEST_CENTER_OF_MASS_DIST_TO_ORIGIN"
            }
            OptimizationPriority::LeastNbrMosaics => "LEAST_NBR_MOSAICS",
            OptimizationPriority::LeastNbrUnusedTiles => "LEAST_NBR_UNUSED_TILES",
            OptimizationPriority::MostUnusedPanelArea => "MOST_UNUSED_PANEL_AREA",
        }
    }

    /// Returns a human-readable description of the optimization priority
    ///
    /// # Returns
    /// A string describing what this optimization priority optimizes for
    ///
    /// # Examples
    /// ```
    /// use rezalnyas_core::models::solution_comparator::OptimizationPriority;
    ///
    /// let priority = OptimizationPriority::MostTiles;
    /// assert!(priority.description().contains("most tiles"));
    /// ```
    pub fn description(&self) -> &'static str {
        match self {
            OptimizationPriority::MostTiles => {
                "Optimize for solutions with the most tiles"
            }
            OptimizationPriority::LeastWastedArea => {
                "Optimize for solutions with the least wasted area"
            }
            OptimizationPriority::LeastNbrCuts => {
                "Optimize for solutions with the least number of cuts"
            }
            OptimizationPriority::MostHvDiscrepancy => {
                "Optimize for solutions with the most horizontal-vertical discrepancy"
            }
            OptimizationPriority::BiggestUnusedTileArea => {
                "Optimize for solutions with the biggest unused tile area"
            }
            OptimizationPriority::SmallestCenterOfMassDistToOrigin => {
                "Optimize for solutions with the smallest center of mass distance to origin"
            }
            OptimizationPriority::LeastNbrMosaics => {
                "Optimize for solutions with the least number of mosaics"
            }
            OptimizationPriority::LeastNbrUnusedTiles => {
                "Optimize for solutions with the least number of unused tiles"
            }
            OptimizationPriority::MostUnusedPanelArea => {
                "Optimize for solutions with the most unused panel area"
            }
        }
    }
}

impl Default for OptimizationPriority {
    fn default() -> Self {
        OptimizationPriority::MostTiles
    }
}

impl fmt::Display for OptimizationPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for OptimizationPriority {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
            .ok_or_else(|| format!("Unknown optimization priority: {}", s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_priorities() {
        let all = OptimizationPriority::all();
        assert_eq!(all.len(), 9);
        assert!(all.contains(&OptimizationPriority::MostTiles));
        assert!(all.contains(&OptimizationPriority::LeastWastedArea));
    }

    #[test]
    fn test_from_str() {
        assert_eq!(
            OptimizationPriority::from_str("MOST_TILES"),
            Some(OptimizationPriority::MostTiles)
        );
        assert_eq!(
            OptimizationPriority::from_str("most_tiles"),
            Some(OptimizationPriority::MostTiles)
        );
        assert_eq!(
            OptimizationPriority::from_str("Most_Tiles"),
            Some(OptimizationPriority::MostTiles)
        );
        assert_eq!(OptimizationPriority::from_str("invalid"), None);
    }

    #[test]
    fn test_as_str() {
        assert_eq!(OptimizationPriority::MostTiles.as_str(), "MOST_TILES");
        assert_eq!(
            OptimizationPriority::LeastWastedArea.as_str(),
            "LEAST_WASTED_AREA"
        );
        assert_eq!(
            OptimizationPriority::SmallestCenterOfMassDistToOrigin.as_str(),
            "SMALLEST_CENTER_OF_MASS_DIST_TO_ORIGIN"
        );
    }

    #[test]
    fn test_display() {
        let priority = OptimizationPriority::MostTiles;
        assert_eq!(format!("{}", priority), "MOST_TILES");
    }

    #[test]
    fn test_from_str_trait() {
        use std::str::FromStr;
        
        let priority: OptimizationPriority = "MOST_TILES".parse().unwrap();
        assert_eq!(priority, OptimizationPriority::MostTiles);

        let result: Result<OptimizationPriority, _> = "invalid".parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_description() {
        let priority = OptimizationPriority::MostTiles;
        assert!(priority.description().contains("most tiles"));
        
        let priority = OptimizationPriority::LeastWastedArea;
        assert!(priority.description().contains("least wasted area"));
    }

    #[test]
    fn test_serialization() {
        let priority = OptimizationPriority::MostTiles;
        let serialized = serde_json::to_string(&priority).unwrap();
        let deserialized: OptimizationPriority = serde_json::from_str(&serialized).unwrap();
        assert_eq!(priority, deserialized);
    }

    #[test]
    fn test_default() {
        let default_priority = OptimizationPriority::default();
        assert_eq!(default_priority, OptimizationPriority::MostTiles);
    }

    #[test]
    fn test_all_variants_have_string_representation() {
        for priority in OptimizationPriority::all() {
            let str_repr = priority.as_str();
            assert!(!str_repr.is_empty());
            
            // Test round-trip conversion
            let parsed = OptimizationPriority::from_str(str_repr);
            assert_eq!(parsed, Some(priority));
        }
    }
}
