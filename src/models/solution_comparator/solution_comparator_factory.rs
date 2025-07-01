//! Solution comparator factory for creating comparator functions
//!
//! This module provides a factory for creating solution comparator functions
//! based on optimization priority strings. It maps priority names to their
//! corresponding comparison functions.

use crate::models::solution_comparator::{
    comparators::*, ComparatorError, ComparatorResult, OptimizationPriority, SolutionComparator,
};
use crate::models::task::Solution;
use std::cmp::Ordering;

/// Factory for creating solution comparator functions
///
/// This factory provides methods to create individual comparator functions
/// or lists of comparators based on optimization priority strings.
pub struct SolutionComparatorFactory;

impl SolutionComparatorFactory {
    /// Gets a solution comparator function for the given priority string
    ///
    /// This method maps optimization priority strings to their corresponding
    /// comparator functions. Returns None if the priority string is not recognized.
    ///
    /// # Arguments
    /// * `priority_str` - The optimization priority string (case-insensitive)
    ///
    /// # Returns
    /// Some(SolutionComparator) if the priority is recognized, None otherwise
    ///
    /// # Examples
    /// ```
    /// use rezalnyas_core::models::solution_comparator::SolutionComparatorFactory;
    ///
    /// let comparator = SolutionComparatorFactory::get_solution_comparator("MOST_TILES");
    /// assert!(comparator.is_some());
    ///
    /// let invalid_comparator = SolutionComparatorFactory::get_solution_comparator("INVALID");
    /// assert!(invalid_comparator.is_none());
    /// ```
    pub fn get_solution_comparator(priority_str: &str) -> Option<SolutionComparator> {
        let priority = OptimizationPriority::from_str(priority_str)?;
        
        match priority {
            OptimizationPriority::MostTiles => Some(most_nbr_tiles_comparator),
            OptimizationPriority::LeastWastedArea => Some(least_wasted_area_comparator),
            OptimizationPriority::LeastNbrCuts => Some(least_nbr_cuts_comparator),
            OptimizationPriority::MostHvDiscrepancy => Some(most_hv_discrepancy_comparator),
            OptimizationPriority::BiggestUnusedTileArea => Some(biggest_unused_tile_area_comparator),
            OptimizationPriority::SmallestCenterOfMassDistToOrigin => {
                Some(smallest_center_of_mass_dist_to_origin_comparator)
            }
            OptimizationPriority::LeastNbrMosaics => Some(least_nbr_mosaics_comparator),
            OptimizationPriority::LeastNbrUnusedTiles => Some(least_nbr_unused_tiles_comparator),
            OptimizationPriority::MostUnusedPanelArea => Some(most_unused_panel_area_comparator),
        }
    }

    /// Gets a solution comparator function with error handling
    ///
    /// This method is similar to `get_solution_comparator` but returns a Result
    /// with a descriptive error if the priority string is not recognized.
    ///
    /// # Arguments
    /// * `priority_str` - The optimization priority string (case-insensitive)
    ///
    /// # Returns
    /// Ok(SolutionComparator) if successful, Err(ComparatorError) otherwise
    ///
    /// # Examples
    /// ```
    /// use rezalnyas_core::models::solution_comparator::SolutionComparatorFactory;
    ///
    /// let comparator = SolutionComparatorFactory::get_solution_comparator_result("MOST_TILES");
    /// assert!(comparator.is_ok());
    ///
    /// let invalid_comparator = SolutionComparatorFactory::get_solution_comparator_result("INVALID");
    /// assert!(invalid_comparator.is_err());
    /// ```
    pub fn get_solution_comparator_result(
        priority_str: &str,
    ) -> ComparatorResult<SolutionComparator> {
        Self::get_solution_comparator(priority_str)
            .ok_or_else(|| ComparatorError::UnknownPriority(priority_str.to_string()))
    }

    /// Gets a list of solution comparator functions from priority strings
    ///
    /// This method creates a list of comparator functions from a list of
    /// optimization priority strings. Invalid priority strings are skipped.
    ///
    /// # Arguments
    /// * `priority_list` - List of optimization priority strings
    ///
    /// # Returns
    /// Vector of comparator functions (may be shorter than input if some priorities are invalid)
    ///
    /// # Examples
    /// ```
    /// use rezalnyas_core::models::solution_comparator::SolutionComparatorFactory;
    ///
    /// let priorities = vec!["MOST_TILES".to_string(), "LEAST_WASTED_AREA".to_string()];
    /// let comparators = SolutionComparatorFactory::get_solution_comparator_list(&priorities);
    /// assert_eq!(comparators.len(), 2);
    /// ```
    pub fn get_solution_comparator_list(priority_list: &[String]) -> Vec<SolutionComparator> {
        priority_list
            .iter()
            .filter_map(|priority_str| Self::get_solution_comparator(priority_str))
            .collect()
    }

    /// Gets a list of solution comparator functions with error handling
    ///
    /// This method creates a list of comparator functions from a list of
    /// optimization priority strings. Returns an error if any priority string
    /// is not recognized.
    ///
    /// # Arguments
    /// * `priority_list` - List of optimization priority strings
    ///
    /// # Returns
    /// Ok(Vec<SolutionComparator>) if all priorities are valid, Err(ComparatorError) otherwise
    ///
    /// # Examples
    /// ```
    /// use rezalnyas_core::models::solution_comparator::SolutionComparatorFactory;
    ///
    /// let valid_priorities = vec!["MOST_TILES".to_string(), "LEAST_WASTED_AREA".to_string()];
    /// let comparators = SolutionComparatorFactory::get_solution_comparator_list_result(&valid_priorities);
    /// assert!(comparators.is_ok());
    ///
    /// let invalid_priorities = vec!["MOST_TILES".to_string(), "INVALID".to_string()];
    /// let comparators = SolutionComparatorFactory::get_solution_comparator_list_result(&invalid_priorities);
    /// assert!(comparators.is_err());
    /// ```
    pub fn get_solution_comparator_list_result(
        priority_list: &[String],
    ) -> ComparatorResult<Vec<SolutionComparator>> {
        let mut comparators = Vec::new();
        
        for priority_str in priority_list {
            let comparator = Self::get_solution_comparator_result(priority_str)?;
            comparators.push(comparator);
        }
        
        Ok(comparators)
    }

    /// Gets all available optimization priority strings
    ///
    /// This method returns a list of all supported optimization priority strings
    /// that can be used with the comparator factory.
    ///
    /// # Returns
    /// Vector of all supported optimization priority strings
    ///
    /// # Examples
    /// ```
    /// use rezalnyas_core::models::solution_comparator::SolutionComparatorFactory;
    ///
    /// let all_priorities = SolutionComparatorFactory::get_all_priority_strings();
    /// assert_eq!(all_priorities.len(), 9);
    /// assert!(all_priorities.contains(&"MOST_TILES".to_string()));
    /// ```
    pub fn get_all_priority_strings() -> Vec<String> {
        OptimizationPriority::all()
            .iter()
            .map(|priority| priority.to_string())
            .collect()
    }

    /// Validates that all priority strings are supported
    ///
    /// This method checks if all provided priority strings are recognized
    /// by the comparator factory.
    ///
    /// # Arguments
    /// * `priority_list` - List of priority strings to validate
    ///
    /// # Returns
    /// Ok(()) if all priorities are valid, Err(ComparatorError) with the first invalid priority
    ///
    /// # Examples
    /// ```
    /// use rezalnyas_core::models::solution_comparator::SolutionComparatorFactory;
    ///
    /// let valid_priorities = vec!["MOST_TILES".to_string(), "LEAST_WASTED_AREA".to_string()];
    /// assert!(SolutionComparatorFactory::validate_priority_strings(&valid_priorities).is_ok());
    ///
    /// let invalid_priorities = vec!["MOST_TILES".to_string(), "INVALID".to_string()];
    /// assert!(SolutionComparatorFactory::validate_priority_strings(&invalid_priorities).is_err());
    /// ```
    pub fn validate_priority_strings(priority_list: &[String]) -> ComparatorResult<()> {
        for priority_str in priority_list {
            if OptimizationPriority::from_str(priority_str).is_none() {
                return Err(ComparatorError::UnknownPriority(priority_str.clone()));
            }
        }
        Ok(())
    }

    /// Sorts solutions using a single optimization priority
    ///
    /// This method sorts a vector of solutions using a single optimization criterion.
    ///
    /// # Arguments
    /// * `solutions` - Mutable reference to the vector of solutions to sort
    /// * `priority_str` - The optimization priority string to use for sorting
    ///
    /// # Returns
    /// Ok(()) if successful, Err(ComparatorError) if the priority is not recognized
    ///
    /// # Examples
    /// ```
    /// use rezalnyas_core::models::{task::Solution, solution_comparator::SolutionComparatorFactory};
    ///
    /// let mut solutions = vec![
    ///     Solution::new("mat1".to_string(), 0.8, 0.9),
    ///     Solution::new("mat2".to_string(), 0.7, 0.8),
    /// ];
    /// 
    /// let result = SolutionComparatorFactory::sort_solutions(&mut solutions, "MOST_TILES");
    /// assert!(result.is_ok());
    /// ```
    pub fn sort_solutions(
        solutions: &mut [Solution],
        priority_str: &str,
    ) -> ComparatorResult<()> {
        let comparator = Self::get_solution_comparator_result(priority_str)?;
        solutions.sort_by(comparator);
        Ok(())
    }

    /// Sorts solutions using multiple optimization priorities
    ///
    /// This method sorts a vector of solutions using multiple optimization criteria
    /// in order of priority.
    ///
    /// # Arguments
    /// * `solutions` - Mutable reference to the vector of solutions to sort
    /// * `priority_list` - List of optimization priority strings in order of importance
    ///
    /// # Returns
    /// Ok(()) if successful, Err(ComparatorError) if any priority is not recognized
    ///
    /// # Examples
    /// ```
    /// use rezalnyas_core::models::{task::Solution, solution_comparator::SolutionComparatorFactory};
    ///
    /// let mut solutions = vec![
    ///     Solution::new("mat1".to_string(), 0.8, 0.9),
    ///     Solution::new("mat2".to_string(), 0.7, 0.8),
    /// ];
    /// 
    /// let priorities = vec!["MOST_TILES".to_string(), "LEAST_WASTED_AREA".to_string()];
    /// let result = SolutionComparatorFactory::sort_solutions_multi(&mut solutions, &priorities);
    /// assert!(result.is_ok());
    /// ```
    pub fn sort_solutions_multi(
        solutions: &mut [Solution],
        priority_list: &[String],
    ) -> ComparatorResult<()> {
        let comparators = Self::get_solution_comparator_list_result(priority_list)?;
        
        solutions.sort_by(|solution1, solution2| {
            for comparator in &comparators {
                let result = comparator(solution1, solution2);
                if result != Ordering::Equal {
                    return result;
                }
            }
            Ordering::Equal
        });
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::task::Solution;

    #[test]
    fn test_get_solution_comparator_valid() {
        let comparator = SolutionComparatorFactory::get_solution_comparator("MOST_TILES");
        assert!(comparator.is_some());

        let comparator = SolutionComparatorFactory::get_solution_comparator("most_tiles");
        assert!(comparator.is_some());

        let comparator = SolutionComparatorFactory::get_solution_comparator("LEAST_WASTED_AREA");
        assert!(comparator.is_some());
    }

    #[test]
    fn test_get_solution_comparator_invalid() {
        let comparator = SolutionComparatorFactory::get_solution_comparator("INVALID_PRIORITY");
        assert!(comparator.is_none());

        let comparator = SolutionComparatorFactory::get_solution_comparator("");
        assert!(comparator.is_none());
    }

    #[test]
    fn test_get_solution_comparator_result() {
        let result = SolutionComparatorFactory::get_solution_comparator_result("MOST_TILES");
        assert!(result.is_ok());

        let result = SolutionComparatorFactory::get_solution_comparator_result("INVALID");
        assert!(result.is_err());
        
        if let Err(ComparatorError::UnknownPriority(priority)) = result {
            assert_eq!(priority, "INVALID");
        } else {
            panic!("Expected UnknownPriority error");
        }
    }

    #[test]
    fn test_get_solution_comparator_list() {
        let priorities = vec![
            "MOST_TILES".to_string(),
            "LEAST_WASTED_AREA".to_string(),
            "INVALID".to_string(),
        ];
        
        let comparators = SolutionComparatorFactory::get_solution_comparator_list(&priorities);
        assert_eq!(comparators.len(), 2); // Invalid priority is skipped
    }

    #[test]
    fn test_get_solution_comparator_list_result() {
        let valid_priorities = vec![
            "MOST_TILES".to_string(),
            "LEAST_WASTED_AREA".to_string(),
        ];
        
        let result = SolutionComparatorFactory::get_solution_comparator_list_result(&valid_priorities);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);

        let invalid_priorities = vec![
            "MOST_TILES".to_string(),
            "INVALID".to_string(),
        ];
        
        let result = SolutionComparatorFactory::get_solution_comparator_list_result(&invalid_priorities);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_all_priority_strings() {
        let all_priorities = SolutionComparatorFactory::get_all_priority_strings();
        assert_eq!(all_priorities.len(), 9);
        assert!(all_priorities.contains(&"MOST_TILES".to_string()));
        assert!(all_priorities.contains(&"LEAST_WASTED_AREA".to_string()));
        assert!(all_priorities.contains(&"LEAST_NBR_CUTS".to_string()));
    }

    #[test]
    fn test_validate_priority_strings() {
        let valid_priorities = vec!["MOST_TILES".to_string(), "LEAST_WASTED_AREA".to_string()];
        assert!(SolutionComparatorFactory::validate_priority_strings(&valid_priorities).is_ok());

        let invalid_priorities = vec!["MOST_TILES".to_string(), "INVALID".to_string()];
        assert!(SolutionComparatorFactory::validate_priority_strings(&invalid_priorities).is_err());
    }

    #[test]
    fn test_sort_solutions() {
        let mut solutions = vec![
            Solution::new("mat1".to_string(), 0.8, 0.9),
            Solution::new("mat2".to_string(), 0.7, 0.8),
        ];
        
        let result = SolutionComparatorFactory::sort_solutions(&mut solutions, "MOST_TILES");
        assert!(result.is_ok());

        let result = SolutionComparatorFactory::sort_solutions(&mut solutions, "INVALID");
        assert!(result.is_err());
    }

    #[test]
    fn test_sort_solutions_multi() {
        let mut solutions = vec![
            Solution::new("mat1".to_string(), 0.8, 0.9),
            Solution::new("mat2".to_string(), 0.7, 0.8),
        ];
        
        let priorities = vec!["MOST_TILES".to_string(), "LEAST_WASTED_AREA".to_string()];
        let result = SolutionComparatorFactory::sort_solutions_multi(&mut solutions, &priorities);
        assert!(result.is_ok());

        let invalid_priorities = vec!["MOST_TILES".to_string(), "INVALID".to_string()];
        let result = SolutionComparatorFactory::sort_solutions_multi(&mut solutions, &invalid_priorities);
        assert!(result.is_err());
    }

    #[test]
    fn test_all_priorities_have_comparators() {
        let all_priorities = SolutionComparatorFactory::get_all_priority_strings();
        
        for priority in all_priorities {
            let comparator = SolutionComparatorFactory::get_solution_comparator(&priority);
            assert!(comparator.is_some(), "No comparator found for priority: {}", priority);
        }
    }
}
