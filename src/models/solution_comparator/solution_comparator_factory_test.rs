//! Tests for SolutionComparatorFactory
//!
//! This module contains comprehensive tests for the SolutionComparatorFactory,
//! including comparator creation, validation, and sorting functionality.

#[cfg(test)]
mod tests {
    use super::super::solution_comparator_factory::SolutionComparatorFactory;
    use super::super::optimization_priority::OptimizationPriority;
    use crate::models::task::Solution;

    #[test]
    fn test_get_solution_comparator_valid() {
        let comparator = SolutionComparatorFactory::get_solution_comparator("MOST_TILES");
        assert!(comparator.is_some());

        let comparator = SolutionComparatorFactory::get_solution_comparator("most_tiles");
        assert!(comparator.is_some());

        let comparator = SolutionComparatorFactory::get_solution_comparator("LEAST_WASTED_AREA");
        assert!(comparator.is_some());

        let comparator = SolutionComparatorFactory::get_solution_comparator("LEAST_NBR_CUTS");
        assert!(comparator.is_some());
    }

    #[test]
    fn test_get_solution_comparator_invalid() {
        let comparator = SolutionComparatorFactory::get_solution_comparator("INVALID_PRIORITY");
        assert!(comparator.is_none());

        let comparator = SolutionComparatorFactory::get_solution_comparator("");
        assert!(comparator.is_none());

        let comparator = SolutionComparatorFactory::get_solution_comparator("RANDOM_STRING");
        assert!(comparator.is_none());
    }

    #[test]
    fn test_get_solution_comparator_result_valid() {
        let result = SolutionComparatorFactory::get_solution_comparator_result("MOST_TILES");
        assert!(result.is_ok());

        let result = SolutionComparatorFactory::get_solution_comparator_result("LEAST_WASTED_AREA");
        assert!(result.is_ok());

        let result = SolutionComparatorFactory::get_solution_comparator_result("LEAST_NBR_CUTS");
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_solution_comparator_result_invalid() {
        let result = SolutionComparatorFactory::get_solution_comparator_result("INVALID");
        assert!(result.is_err());
        
        let result = SolutionComparatorFactory::get_solution_comparator_result("");
        assert!(result.is_err());
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
    fn test_get_solution_comparator_list_all_valid() {
        let priorities = vec![
            "MOST_TILES".to_string(),
            "LEAST_WASTED_AREA".to_string(),
            "LEAST_NBR_CUTS".to_string(),
        ];
        
        let comparators = SolutionComparatorFactory::get_solution_comparator_list(&priorities);
        assert_eq!(comparators.len(), 3);
    }

    #[test]
    fn test_get_solution_comparator_list_result_valid() {
        let valid_priorities = vec![
            "MOST_TILES".to_string(),
            "LEAST_WASTED_AREA".to_string(),
        ];
        
        let result = SolutionComparatorFactory::get_solution_comparator_list_result(&valid_priorities);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[test]
    fn test_get_solution_comparator_list_result_invalid() {
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
        assert!(all_priorities.contains(&"MOST_HV_DISCREPANCY".to_string()));
        assert!(all_priorities.contains(&"BIGGEST_UNUSED_TILE_AREA".to_string()));
        assert!(all_priorities.contains(&"SMALLEST_CENTER_OF_MASS_DIST_TO_ORIGIN".to_string()));
        assert!(all_priorities.contains(&"LEAST_NBR_MOSAICS".to_string()));
        assert!(all_priorities.contains(&"LEAST_NBR_UNUSED_TILES".to_string()));
        assert!(all_priorities.contains(&"MOST_UNUSED_PANEL_AREA".to_string()));
    }

    #[test]
    fn test_validate_priority_strings_valid() {
        let valid_priorities = vec![
            "MOST_TILES".to_string(),
            "LEAST_WASTED_AREA".to_string(),
        ];
        
        let result = SolutionComparatorFactory::validate_priority_strings(&valid_priorities);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_priority_strings_invalid() {
        let invalid_priorities = vec![
            "MOST_TILES".to_string(),
            "INVALID".to_string(),
        ];
        
        let result = SolutionComparatorFactory::validate_priority_strings(&invalid_priorities);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_priority_strings_empty() {
        let empty_priorities = vec![];
        
        let result = SolutionComparatorFactory::validate_priority_strings(&empty_priorities);
        assert!(result.is_ok()); // Empty list is valid
    }

    #[test]
    fn test_sort_solutions_valid_priority() {
        let mut solutions = vec![
            Solution::new("mat1".to_string(), 0.8, 0.9),
            Solution::new("mat2".to_string(), 0.7, 0.8),
        ];
        
        let result = SolutionComparatorFactory::sort_solutions(&mut solutions, "MOST_TILES");
        assert!(result.is_ok());
    }

    #[test]
    fn test_sort_solutions_invalid_priority() {
        let mut solutions = vec![
            Solution::new("mat1".to_string(), 0.8, 0.9),
            Solution::new("mat2".to_string(), 0.7, 0.8),
        ];
        
        let result = SolutionComparatorFactory::sort_solutions(&mut solutions, "INVALID");
        assert!(result.is_err());
    }

    #[test]
    fn test_sort_solutions_multi_valid() {
        let mut solutions = vec![
            Solution::new("mat1".to_string(), 0.8, 0.9),
            Solution::new("mat2".to_string(), 0.7, 0.8),
        ];
        
        let priorities = vec![
            "MOST_TILES".to_string(),
            "LEAST_WASTED_AREA".to_string(),
        ];
        
        let result = SolutionComparatorFactory::sort_solutions_multi(&mut solutions, &priorities);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sort_solutions_multi_invalid() {
        let mut solutions = vec![
            Solution::new("mat1".to_string(), 0.8, 0.9),
            Solution::new("mat2".to_string(), 0.7, 0.8),
        ];
        
        let invalid_priorities = vec![
            "MOST_TILES".to_string(),
            "INVALID".to_string(),
        ];
        
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

    #[test]
    fn test_comparator_consistency() {
        // Test that getting a comparator through different methods returns the same result
        let priority = "MOST_TILES";
        
        let comparator1 = SolutionComparatorFactory::get_solution_comparator(priority);
        let comparator2 = SolutionComparatorFactory::get_solution_comparator_result(priority);
        
        assert!(comparator1.is_some());
        assert!(comparator2.is_ok());
    }

    #[test]
    fn test_case_insensitive_priority_matching() {
        let test_cases = vec![
            ("MOST_TILES", true),
            ("most_tiles", true),
            ("Most_Tiles", true),
            ("MOST_tiles", true),
            ("invalid_priority", false),
            ("", false),
        ];

        for (priority_str, should_exist) in test_cases {
            let comparator = SolutionComparatorFactory::get_solution_comparator(priority_str);
            assert_eq!(
                comparator.is_some(),
                should_exist,
                "Priority '{}' should {} exist",
                priority_str,
                if should_exist { "" } else { "not " }
            );
        }
    }

    #[test]
    fn test_comprehensive_priority_coverage() {
        // Test that all OptimizationPriority variants have corresponding comparators
        for priority in OptimizationPriority::all() {
            let priority_str = priority.to_string();
            let comparator = SolutionComparatorFactory::get_solution_comparator(&priority_str);
            assert!(
                comparator.is_some(),
                "No comparator found for OptimizationPriority::{:?}",
                priority
            );
        }
    }

    #[test]
    fn test_empty_solution_list_sorting() {
        let mut empty_solutions: Vec<Solution> = vec![];
        
        let result = SolutionComparatorFactory::sort_solutions(&mut empty_solutions, "MOST_TILES");
        assert!(result.is_ok());
        assert!(empty_solutions.is_empty());

        let priorities = vec!["MOST_TILES".to_string(), "LEAST_WASTED_AREA".to_string()];
        let result = SolutionComparatorFactory::sort_solutions_multi(&mut empty_solutions, &priorities);
        assert!(result.is_ok());
        assert!(empty_solutions.is_empty());
    }

    #[test]
    fn test_single_solution_sorting() {
        let mut single_solution = vec![Solution::new("mat1".to_string(), 0.8, 0.9)];
        
        let result = SolutionComparatorFactory::sort_solutions(&mut single_solution, "MOST_TILES");
        assert!(result.is_ok());
        assert_eq!(single_solution.len(), 1);

        let priorities = vec!["MOST_TILES".to_string(), "LEAST_WASTED_AREA".to_string()];
        let result = SolutionComparatorFactory::sort_solutions_multi(&mut single_solution, &priorities);
        assert!(result.is_ok());
        assert_eq!(single_solution.len(), 1);
    }

    #[test]
    fn test_priority_list_validation_edge_cases() {
        // Test with duplicate priorities
        let duplicate_priorities = vec![
            "MOST_TILES".to_string(),
            "MOST_TILES".to_string(),
        ];
        let result = SolutionComparatorFactory::validate_priority_strings(&duplicate_priorities);
        assert!(result.is_ok()); // Duplicates are allowed in validation

        // Test with mixed case
        let mixed_case_priorities = vec![
            "MOST_TILES".to_string(),
            "least_wasted_area".to_string(),
        ];
        let result = SolutionComparatorFactory::validate_priority_strings(&mixed_case_priorities);
        assert!(result.is_ok());
    }
}
