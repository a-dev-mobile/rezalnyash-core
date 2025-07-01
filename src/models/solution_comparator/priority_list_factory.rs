//! Priority list factory for generating optimization priority lists
//!
//! This module provides functionality to generate prioritized lists of
//! optimization criteria based on configuration settings.

use crate::models::configuration::Configuration;
use crate::models::solution_comparator::OptimizationPriority;

/// Factory for creating prioritized lists of optimization criteria
///
/// This factory generates ordered lists of optimization priorities based on
/// configuration settings, allowing different optimization strategies to be
/// applied to solution comparison.
pub struct PriorityListFactory;

impl PriorityListFactory {
    /// Gets the final solution prioritized comparator list based on configuration
    ///
    /// This method generates a prioritized list of optimization criteria strings
    /// that can be used to create comparators for final solution ranking.
    /// The order depends on the optimization priority setting in the configuration.
    ///
    /// # Arguments
    /// * `configuration` - The configuration containing optimization settings
    ///
    /// # Returns
    /// A vector of optimization priority strings in priority order
    ///
    /// # Examples
    /// ```
    /// use rezalnyas_core::models::{Configuration, solution_comparator::PriorityListFactory};
    ///
    /// let mut config = Configuration::new();
    /// config.set_optimization_priority(0);
    /// 
    /// let priorities = PriorityListFactory::get_final_solution_prioritized_comparator_list(&config);
    /// assert!(!priorities.is_empty());
    /// assert_eq!(priorities[0], "MOST_TILES");
    /// ```
    pub fn get_final_solution_prioritized_comparator_list(
        configuration: &Configuration,
    ) -> Vec<String> {
        let mut priority_list = Vec::new();

        // Primary priorities based on optimization_priority setting
        if configuration.optimization_priority() == 0 {
            // Default priority order
            priority_list.push(OptimizationPriority::MostTiles.to_string());
            priority_list.push(OptimizationPriority::LeastWastedArea.to_string());
            priority_list.push(OptimizationPriority::LeastNbrCuts.to_string());
        } else {
            // Alternative priority order
            priority_list.push(OptimizationPriority::MostTiles.to_string());
            priority_list.push(OptimizationPriority::LeastNbrCuts.to_string());
            priority_list.push(OptimizationPriority::LeastWastedArea.to_string());
        }

        // Secondary priorities (always added in this order)
        priority_list.push(OptimizationPriority::LeastNbrMosaics.to_string());
        priority_list.push(OptimizationPriority::BiggestUnusedTileArea.to_string());
        priority_list.push(OptimizationPriority::MostHvDiscrepancy.to_string());

        priority_list
    }

    /// Gets a comprehensive prioritized list including all optimization criteria
    ///
    /// This method returns a complete list of all optimization priorities,
    /// ordered by their typical importance in cutting optimization.
    ///
    /// # Returns
    /// A vector containing all optimization priority strings in recommended order
    ///
    /// # Examples
    /// ```
    /// use rezalnyas_core::models::solution_comparator::PriorityListFactory;
    ///
    /// let all_priorities = PriorityListFactory::get_comprehensive_priority_list();
    /// assert_eq!(all_priorities.len(), 9);
    /// ```
    pub fn get_comprehensive_priority_list() -> Vec<String> {
        vec![
            OptimizationPriority::MostTiles.to_string(),
            OptimizationPriority::LeastWastedArea.to_string(),
            OptimizationPriority::LeastNbrCuts.to_string(),
            OptimizationPriority::LeastNbrMosaics.to_string(),
            OptimizationPriority::BiggestUnusedTileArea.to_string(),
            OptimizationPriority::MostHvDiscrepancy.to_string(),
            OptimizationPriority::SmallestCenterOfMassDistToOrigin.to_string(),
            OptimizationPriority::LeastNbrUnusedTiles.to_string(),
            OptimizationPriority::MostUnusedPanelArea.to_string(),
        ]
    }

    /// Gets a custom priority list based on specific optimization goals
    ///
    /// This method allows creating custom priority lists for specific optimization
    /// scenarios, such as minimizing waste, maximizing efficiency, or balancing
    /// multiple criteria.
    ///
    /// # Arguments
    /// * `primary_goals` - The primary optimization goals to prioritize
    /// * `include_secondary` - Whether to include secondary optimization criteria
    ///
    /// # Returns
    /// A vector of optimization priority strings in the specified order
    ///
    /// # Examples
    /// ```
    /// use rezalnyas_core::models::solution_comparator::{PriorityListFactory, OptimizationPriority};
    ///
    /// let primary = vec![OptimizationPriority::LeastWastedArea, OptimizationPriority::LeastNbrCuts];
    /// let custom_list = PriorityListFactory::get_custom_priority_list(&primary, true);
    /// assert!(custom_list.len() >= 2);
    /// assert_eq!(custom_list[0], "LEAST_WASTED_AREA");
    /// ```
    pub fn get_custom_priority_list(
        primary_goals: &[OptimizationPriority],
        include_secondary: bool,
    ) -> Vec<String> {
        let mut priority_list = Vec::new();

        // Add primary goals first
        for goal in primary_goals {
            priority_list.push(goal.to_string());
        }

        // Add secondary goals if requested
        if include_secondary {
            let all_priorities = OptimizationPriority::all();
            for priority in all_priorities {
                let priority_str = priority.to_string();
                if !priority_list.contains(&priority_str) {
                    priority_list.push(priority_str);
                }
            }
        }

        priority_list
    }

    /// Gets a priority list optimized for waste minimization
    ///
    /// This method returns a priority list specifically designed to minimize
    /// material waste in cutting operations.
    ///
    /// # Returns
    /// A vector of optimization priority strings focused on waste reduction
    ///
    /// # Examples
    /// ```
    /// use rezalnyas_core::models::solution_comparator::PriorityListFactory;
    ///
    /// let waste_focused = PriorityListFactory::get_waste_minimization_priority_list();
    /// assert_eq!(waste_focused[0], "LEAST_WASTED_AREA");
    /// ```
    pub fn get_waste_minimization_priority_list() -> Vec<String> {
        vec![
            OptimizationPriority::LeastWastedArea.to_string(),
            OptimizationPriority::BiggestUnusedTileArea.to_string(),
            OptimizationPriority::MostTiles.to_string(),
            OptimizationPriority::LeastNbrUnusedTiles.to_string(),
            OptimizationPriority::LeastNbrCuts.to_string(),
            OptimizationPriority::LeastNbrMosaics.to_string(),
            OptimizationPriority::MostHvDiscrepancy.to_string(),
            OptimizationPriority::SmallestCenterOfMassDistToOrigin.to_string(),
            OptimizationPriority::MostUnusedPanelArea.to_string(),
        ]
    }

    /// Gets a priority list optimized for cutting efficiency
    ///
    /// This method returns a priority list specifically designed to minimize
    /// the number of cuts and optimize cutting operations.
    ///
    /// # Returns
    /// A vector of optimization priority strings focused on cutting efficiency
    ///
    /// # Examples
    /// ```
    /// use rezalnyas_core::models::solution_comparator::PriorityListFactory;
    ///
    /// let cut_focused = PriorityListFactory::get_cutting_efficiency_priority_list();
    /// assert_eq!(cut_focused[0], "LEAST_NBR_CUTS");
    /// ```
    pub fn get_cutting_efficiency_priority_list() -> Vec<String> {
        vec![
            OptimizationPriority::LeastNbrCuts.to_string(),
            OptimizationPriority::LeastNbrMosaics.to_string(),
            OptimizationPriority::MostTiles.to_string(),
            OptimizationPriority::LeastWastedArea.to_string(),
            OptimizationPriority::MostHvDiscrepancy.to_string(),
            OptimizationPriority::BiggestUnusedTileArea.to_string(),
            OptimizationPriority::LeastNbrUnusedTiles.to_string(),
            OptimizationPriority::SmallestCenterOfMassDistToOrigin.to_string(),
            OptimizationPriority::MostUnusedPanelArea.to_string(),
        ]
    }

    /// Validates a priority list for completeness and correctness
    ///
    /// This method checks if a priority list contains valid optimization
    /// priority strings and optionally verifies completeness.
    ///
    /// # Arguments
    /// * `priority_list` - The priority list to validate
    /// * `require_complete` - Whether to require all priorities to be present
    ///
    /// # Returns
    /// `Ok(())` if the list is valid, `Err(String)` with error description otherwise
    ///
    /// # Examples
    /// ```
    /// use rezalnyas_core::models::solution_comparator::PriorityListFactory;
    ///
    /// let valid_list = vec!["MOST_TILES".to_string(), "LEAST_WASTED_AREA".to_string()];
    /// assert!(PriorityListFactory::validate_priority_list(&valid_list, false).is_ok());
    ///
    /// let invalid_list = vec!["INVALID_PRIORITY".to_string()];
    /// assert!(PriorityListFactory::validate_priority_list(&invalid_list, false).is_err());
    /// ```
    pub fn validate_priority_list(
        priority_list: &[String],
        require_complete: bool,
    ) -> Result<(), String> {
        if priority_list.is_empty() {
            return Err("Priority list cannot be empty".to_string());
        }

        // Check for valid priorities
        for priority_str in priority_list {
            if OptimizationPriority::from_str(priority_str).is_none() {
                return Err(format!("Invalid optimization priority: {}", priority_str));
            }
        }

        // Check for completeness if required
        if require_complete {
            let all_priorities = OptimizationPriority::all();
            for priority in all_priorities {
                let priority_str = priority.to_string();
                if !priority_list.contains(&priority_str) {
                    return Err(format!("Missing required priority: {}", priority_str));
                }
            }
        }

        // Check for duplicates
        let mut seen = std::collections::HashSet::new();
        for priority_str in priority_list {
            if !seen.insert(priority_str) {
                return Err(format!("Duplicate priority found: {}", priority_str));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_final_solution_prioritized_comparator_list_default() {
        let mut config = Configuration::new();
        config.set_optimization_priority(0);

        let priorities = PriorityListFactory::get_final_solution_prioritized_comparator_list(&config);
        
        assert_eq!(priorities.len(), 6);
        assert_eq!(priorities[0], "MOST_TILES");
        assert_eq!(priorities[1], "LEAST_WASTED_AREA");
        assert_eq!(priorities[2], "LEAST_NBR_CUTS");
        assert_eq!(priorities[3], "LEAST_NBR_MOSAICS");
        assert_eq!(priorities[4], "BIGGEST_UNUSED_TILE_AREA");
        assert_eq!(priorities[5], "MOST_HV_DISCREPANCY");
    }

    #[test]
    fn test_get_final_solution_prioritized_comparator_list_alternative() {
        let mut config = Configuration::new();
        config.set_optimization_priority(1);

        let priorities = PriorityListFactory::get_final_solution_prioritized_comparator_list(&config);
        
        assert_eq!(priorities.len(), 6);
        assert_eq!(priorities[0], "MOST_TILES");
        assert_eq!(priorities[1], "LEAST_NBR_CUTS");
        assert_eq!(priorities[2], "LEAST_WASTED_AREA");
        assert_eq!(priorities[3], "LEAST_NBR_MOSAICS");
        assert_eq!(priorities[4], "BIGGEST_UNUSED_TILE_AREA");
        assert_eq!(priorities[5], "MOST_HV_DISCREPANCY");
    }

    #[test]
    fn test_get_comprehensive_priority_list() {
        let priorities = PriorityListFactory::get_comprehensive_priority_list();
        assert_eq!(priorities.len(), 9);
        assert_eq!(priorities[0], "MOST_TILES");
    }

    #[test]
    fn test_get_custom_priority_list() {
        let primary = vec![
            OptimizationPriority::LeastWastedArea,
            OptimizationPriority::LeastNbrCuts,
        ];
        
        let custom_list = PriorityListFactory::get_custom_priority_list(&primary, false);
        assert_eq!(custom_list.len(), 2);
        assert_eq!(custom_list[0], "LEAST_WASTED_AREA");
        assert_eq!(custom_list[1], "LEAST_NBR_CUTS");

        let custom_list_with_secondary = PriorityListFactory::get_custom_priority_list(&primary, true);
        assert_eq!(custom_list_with_secondary.len(), 9);
        assert_eq!(custom_list_with_secondary[0], "LEAST_WASTED_AREA");
        assert_eq!(custom_list_with_secondary[1], "LEAST_NBR_CUTS");
    }

    #[test]
    fn test_get_waste_minimization_priority_list() {
        let priorities = PriorityListFactory::get_waste_minimization_priority_list();
        assert_eq!(priorities[0], "LEAST_WASTED_AREA");
        assert_eq!(priorities[1], "BIGGEST_UNUSED_TILE_AREA");
    }

    #[test]
    fn test_get_cutting_efficiency_priority_list() {
        let priorities = PriorityListFactory::get_cutting_efficiency_priority_list();
        assert_eq!(priorities[0], "LEAST_NBR_CUTS");
        assert_eq!(priorities[1], "LEAST_NBR_MOSAICS");
    }

    #[test]
    fn test_validate_priority_list_valid() {
        let valid_list = vec!["MOST_TILES".to_string(), "LEAST_WASTED_AREA".to_string()];
        assert!(PriorityListFactory::validate_priority_list(&valid_list, false).is_ok());
    }

    #[test]
    fn test_validate_priority_list_invalid() {
        let invalid_list = vec!["INVALID_PRIORITY".to_string()];
        assert!(PriorityListFactory::validate_priority_list(&invalid_list, false).is_err());
    }

    #[test]
    fn test_validate_priority_list_empty() {
        let empty_list = vec![];
        assert!(PriorityListFactory::validate_priority_list(&empty_list, false).is_err());
    }

    #[test]
    fn test_validate_priority_list_duplicates() {
        let duplicate_list = vec!["MOST_TILES".to_string(), "MOST_TILES".to_string()];
        assert!(PriorityListFactory::validate_priority_list(&duplicate_list, false).is_err());
    }

    #[test]
    fn test_validate_priority_list_complete() {
        let complete_list = PriorityListFactory::get_comprehensive_priority_list();
        assert!(PriorityListFactory::validate_priority_list(&complete_list, true).is_ok());

        let incomplete_list = vec!["MOST_TILES".to_string()];
        assert!(PriorityListFactory::validate_priority_list(&incomplete_list, true).is_err());
    }
}
