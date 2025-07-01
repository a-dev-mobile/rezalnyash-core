//! Tests for OptimizationPriority enum
//!
//! This module contains comprehensive tests for the OptimizationPriority enum,
//! including conversion from strings, display formatting, and edge cases.

#[cfg(test)]
mod tests {
    use super::super::optimization_priority::OptimizationPriority;

    #[test]
    fn test_from_str_valid_priorities() {
        // Test all valid priority strings
        assert_eq!(
            OptimizationPriority::from_str("MOST_TILES"),
            Some(OptimizationPriority::MostTiles)
        );
        assert_eq!(
            OptimizationPriority::from_str("LEAST_WASTED_AREA"),
            Some(OptimizationPriority::LeastWastedArea)
        );
        assert_eq!(
            OptimizationPriority::from_str("LEAST_NBR_CUTS"),
            Some(OptimizationPriority::LeastNbrCuts)
        );
        assert_eq!(
            OptimizationPriority::from_str("MOST_HV_DISCREPANCY"),
            Some(OptimizationPriority::MostHvDiscrepancy)
        );
        assert_eq!(
            OptimizationPriority::from_str("BIGGEST_UNUSED_TILE_AREA"),
            Some(OptimizationPriority::BiggestUnusedTileArea)
        );
        assert_eq!(
            OptimizationPriority::from_str("SMALLEST_CENTER_OF_MASS_DIST_TO_ORIGIN"),
            Some(OptimizationPriority::SmallestCenterOfMassDistToOrigin)
        );
        assert_eq!(
            OptimizationPriority::from_str("LEAST_NBR_MOSAICS"),
            Some(OptimizationPriority::LeastNbrMosaics)
        );
        assert_eq!(
            OptimizationPriority::from_str("LEAST_NBR_UNUSED_TILES"),
            Some(OptimizationPriority::LeastNbrUnusedTiles)
        );
        assert_eq!(
            OptimizationPriority::from_str("MOST_UNUSED_PANEL_AREA"),
            Some(OptimizationPriority::MostUnusedPanelArea)
        );
    }

    #[test]
    fn test_from_str_case_insensitive() {
        // Test case insensitive parsing
        assert_eq!(
            OptimizationPriority::from_str("most_tiles"),
            Some(OptimizationPriority::MostTiles)
        );
        assert_eq!(
            OptimizationPriority::from_str("Most_Tiles"),
            Some(OptimizationPriority::MostTiles)
        );
        assert_eq!(
            OptimizationPriority::from_str("MOST_TILES"),
            Some(OptimizationPriority::MostTiles)
        );
    }

    #[test]
    fn test_from_str_invalid_priority() {
        // Test invalid priority strings - from_str returns Option, so use is_none()
        assert!(OptimizationPriority::from_str("INVALID_PRIORITY").is_none());
        assert!(OptimizationPriority::from_str("").is_none());
        assert!(OptimizationPriority::from_str("RANDOM_STRING").is_none());
    }

    #[test]
    fn test_display_formatting() {
        // Test Display trait implementation
        assert_eq!(
            format!("{}", OptimizationPriority::MostTiles),
            "MOST_TILES"
        );
        assert_eq!(
            format!("{}", OptimizationPriority::LeastWastedArea),
            "LEAST_WASTED_AREA"
        );
        assert_eq!(
            format!("{}", OptimizationPriority::LeastNbrCuts),
            "LEAST_NBR_CUTS"
        );
        assert_eq!(
            format!("{}", OptimizationPriority::MostHvDiscrepancy),
            "MOST_HV_DISCREPANCY"
        );
        assert_eq!(
            format!("{}", OptimizationPriority::BiggestUnusedTileArea),
            "BIGGEST_UNUSED_TILE_AREA"
        );
        assert_eq!(
            format!("{}", OptimizationPriority::SmallestCenterOfMassDistToOrigin),
            "SMALLEST_CENTER_OF_MASS_DIST_TO_ORIGIN"
        );
        assert_eq!(
            format!("{}", OptimizationPriority::LeastNbrMosaics),
            "LEAST_NBR_MOSAICS"
        );
        assert_eq!(
            format!("{}", OptimizationPriority::LeastNbrUnusedTiles),
            "LEAST_NBR_UNUSED_TILES"
        );
        assert_eq!(
            format!("{}", OptimizationPriority::MostUnusedPanelArea),
            "MOST_UNUSED_PANEL_AREA"
        );
    }

    #[test]
    fn test_round_trip_conversion() {
        // Test that converting to string and back gives the same result
        let priorities = vec![
            OptimizationPriority::MostTiles,
            OptimizationPriority::LeastWastedArea,
            OptimizationPriority::LeastNbrCuts,
            OptimizationPriority::MostHvDiscrepancy,
            OptimizationPriority::BiggestUnusedTileArea,
            OptimizationPriority::SmallestCenterOfMassDistToOrigin,
            OptimizationPriority::LeastNbrMosaics,
            OptimizationPriority::LeastNbrUnusedTiles,
            OptimizationPriority::MostUnusedPanelArea,
        ];

        for priority in priorities {
            let string_repr = format!("{}", priority);
            let parsed_priority = OptimizationPriority::from_str(&string_repr);
            assert_eq!(Some(priority), parsed_priority);
        }
    }

    #[test]
    fn test_clone_and_debug() {
        let priority = OptimizationPriority::MostTiles;
        let cloned = priority.clone();
        assert_eq!(priority, cloned);

        // Test Debug formatting
        let debug_str = format!("{:?}", priority);
        assert!(debug_str.contains("MostTiles"));
    }

    #[test]
    fn test_partial_eq() {
        assert_eq!(
            OptimizationPriority::MostTiles,
            OptimizationPriority::MostTiles
        );
        assert_ne!(
            OptimizationPriority::MostTiles,
            OptimizationPriority::LeastWastedArea
        );
    }

    #[test]
    fn test_all_variants_covered() {
        // Ensure all variants can be created and converted
        let all_variants = vec![
            "MOST_TILES",
            "LEAST_WASTED_AREA",
            "LEAST_NBR_CUTS",
            "MOST_HV_DISCREPANCY",
            "BIGGEST_UNUSED_TILE_AREA",
            "SMALLEST_CENTER_OF_MASS_DIST_TO_ORIGIN",
            "LEAST_NBR_MOSAICS",
            "LEAST_NBR_UNUSED_TILES",
            "MOST_UNUSED_PANEL_AREA",
        ];

        for variant_str in all_variants {
            let priority = OptimizationPriority::from_str(variant_str);
            assert!(priority.is_some(), "Failed to parse: {}", variant_str);
            
            let parsed = priority.unwrap();
            let back_to_string = format!("{}", parsed);
            assert_eq!(variant_str, back_to_string);
        }
    }

    #[test]
    fn test_fromstr_trait() {
        // Test the FromStr trait implementation
        use std::str::FromStr;
        
        let result: Result<OptimizationPriority, _> = "MOST_TILES".parse();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), OptimizationPriority::MostTiles);
        
        let result: Result<OptimizationPriority, _> = "INVALID".parse();
        assert!(result.is_err());
    }
}
