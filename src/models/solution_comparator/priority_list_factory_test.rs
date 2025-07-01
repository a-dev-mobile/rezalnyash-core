//! Tests for PriorityListFactory
//!
//! This module contains comprehensive tests for the PriorityListFactory,
//! including priority list generation, validation, and edge cases.

#[cfg(test)]
mod tests {
    use super::super::priority_list_factory::PriorityListFactory;
    use super::super::optimization_priority::OptimizationPriority;
    use crate::models::configuration::Configuration;

    #[test]
    fn test_get_final_solution_prioritized_comparator_list_default() {
        let mut config = Configuration::new();
        config.set_optimization_priority(0);

        let priorities = PriorityListFactory::get_final_solution_prioritized_comparator_list(&config);
        
        assert_eq!(priorities.len(), 6);
        assert_eq!(priorities[0], OptimizationPriority::MostTiles.to_string());
        assert_eq!(priorities[1], OptimizationPriority::LeastWastedArea.to_string());
        assert_eq!(priorities[2], OptimizationPriority::LeastNbrCuts.to_string());
    }

    #[test]
    fn test_get_final_solution_prioritized_comparator_list_alternative() {
        let mut config = Configuration::new();
        config.set_optimization_priority(1);

        let priorities = PriorityListFactory::get_final_solution_prioritized_comparator_list(&config);
        
        assert_eq!(priorities.len(), 6);
        assert_eq!(priorities[0], OptimizationPriority::MostTiles.to_string());
        assert_eq!(priorities[1], OptimizationPriority::LeastNbrCuts.to_string());
        assert_eq!(priorities[2], OptimizationPriority::LeastWastedArea.to_string());
    }

    #[test]
    fn test_get_comprehensive_priority_list() {
        let priorities = PriorityListFactory::get_comprehensive_priority_list();
        
        assert_eq!(priorities.len(), 9);
        assert_eq!(priorities[0], OptimizationPriority::MostTiles.to_string());
        assert!(priorities.contains(&OptimizationPriority::LeastWastedArea.to_string()));
        assert!(priorities.contains(&OptimizationPriority::LeastNbrCuts.to_string()));
    }

    #[test]
    fn test_get_custom_priority_list_without_secondary() {
        let primary = vec![
            OptimizationPriority::LeastWastedArea,
            OptimizationPriority::LeastNbrCuts,
        ];
        
        let custom_list = PriorityListFactory::get_custom_priority_list(&primary, false);
        
        assert_eq!(custom_list.len(), 2);
        assert_eq!(custom_list[0], OptimizationPriority::LeastWastedArea.to_string());
        assert_eq!(custom_list[1], OptimizationPriority::LeastNbrCuts.to_string());
    }

    #[test]
    fn test_get_custom_priority_list_with_secondary() {
        let primary = vec![
            OptimizationPriority::LeastWastedArea,
            OptimizationPriority::LeastNbrCuts,
        ];
        
        let custom_list = PriorityListFactory::get_custom_priority_list(&primary, true);
        
        assert_eq!(custom_list.len(), 9);
        assert_eq!(custom_list[0], OptimizationPriority::LeastWastedArea.to_string());
        assert_eq!(custom_list[1], OptimizationPriority::LeastNbrCuts.to_string());
        
        // Verify all priorities are included
        for priority in OptimizationPriority::all() {
            assert!(custom_list.contains(&priority.to_string()));
        }
    }

    #[test]
    fn test_get_waste_minimization_priority_list() {
        let priorities = PriorityListFactory::get_waste_minimization_priority_list();
        
        assert_eq!(priorities[0], OptimizationPriority::LeastWastedArea.to_string());
        assert_eq!(priorities[1], OptimizationPriority::BiggestUnusedTileArea.to_string());
        assert_eq!(priorities[2], OptimizationPriority::MostTiles.to_string());
    }

    #[test]
    fn test_get_cutting_efficiency_priority_list() {
        let priorities = PriorityListFactory::get_cutting_efficiency_priority_list();
        
        assert_eq!(priorities[0], OptimizationPriority::LeastNbrCuts.to_string());
        assert_eq!(priorities[1], OptimizationPriority::LeastNbrMosaics.to_string());
        assert_eq!(priorities[2], OptimizationPriority::MostTiles.to_string());
    }

    #[test]
    fn test_validate_priority_list_valid() {
        let valid_list = vec![
            OptimizationPriority::MostTiles.to_string(),
            OptimizationPriority::LeastWastedArea.to_string(),
        ];
        
        let result = PriorityListFactory::validate_priority_list(&valid_list, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_priority_list_invalid() {
        let invalid_list = vec!["INVALID_PRIORITY".to_string()];
        
        let result = PriorityListFactory::validate_priority_list(&invalid_list, false);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid optimization priority"));
    }

    #[test]
    fn test_validate_priority_list_empty() {
        let empty_list = vec![];
        
        let result = PriorityListFactory::validate_priority_list(&empty_list, false);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[test]
    fn test_validate_priority_list_duplicates() {
        let duplicate_list = vec![
            OptimizationPriority::MostTiles.to_string(),
            OptimizationPriority::MostTiles.to_string(),
        ];
        
        let result = PriorityListFactory::validate_priority_list(&duplicate_list, false);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Duplicate priority"));
    }

    #[test]
    fn test_validate_priority_list_complete_valid() {
        let complete_list = PriorityListFactory::get_comprehensive_priority_list();
        
        let result = PriorityListFactory::validate_priority_list(&complete_list, true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_priority_list_incomplete() {
        let incomplete_list = vec![OptimizationPriority::MostTiles.to_string()];
        
        let result = PriorityListFactory::validate_priority_list(&incomplete_list, true);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Missing required priority"));
    }

    #[test]
    fn test_priority_list_consistency() {
        // Test that all factory methods return valid priority lists
        let config_default = {
            let mut config = Configuration::new();
            config.set_optimization_priority(0);
            config
        };
        
        let config_alt = {
            let mut config = Configuration::new();
            config.set_optimization_priority(1);
            config
        };

        let lists = vec![
            PriorityListFactory::get_final_solution_prioritized_comparator_list(&config_default),
            PriorityListFactory::get_final_solution_prioritized_comparator_list(&config_alt),
            PriorityListFactory::get_comprehensive_priority_list(),
            PriorityListFactory::get_waste_minimization_priority_list(),
            PriorityListFactory::get_cutting_efficiency_priority_list(),
        ];

        for list in lists {
            assert!(PriorityListFactory::validate_priority_list(&list, false).is_ok());
            assert!(!list.is_empty());
        }
    }

    #[test]
    fn test_custom_priority_list_edge_cases() {
        // Empty primary goals
        let empty_primary = vec![];
        let result = PriorityListFactory::get_custom_priority_list(&empty_primary, false);
        assert!(result.is_empty());

        let result_with_secondary = PriorityListFactory::get_custom_priority_list(&empty_primary, true);
        assert_eq!(result_with_secondary.len(), 9);

        // Single primary goal
        let single_primary = vec![OptimizationPriority::MostTiles];
        let result = PriorityListFactory::get_custom_priority_list(&single_primary, false);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], OptimizationPriority::MostTiles.to_string());
    }

    #[test]
    fn test_priority_list_ordering() {
        // Test that different configurations produce different orderings
        let config_0 = {
            let mut config = Configuration::new();
            config.set_optimization_priority(0);
            config
        };
        
        let config_1 = {
            let mut config = Configuration::new();
            config.set_optimization_priority(1);
            config
        };

        let list_0 = PriorityListFactory::get_final_solution_prioritized_comparator_list(&config_0);
        let list_1 = PriorityListFactory::get_final_solution_prioritized_comparator_list(&config_1);

        // Both should start with MostTiles
        assert_eq!(list_0[0], OptimizationPriority::MostTiles.to_string());
        assert_eq!(list_1[0], OptimizationPriority::MostTiles.to_string());

        // But second and third priorities should be different
        assert_ne!(list_0[1], list_1[1]);
        assert_ne!(list_0[2], list_1[2]);
    }

    #[test]
    fn test_specialized_priority_lists() {
        let waste_list = PriorityListFactory::get_waste_minimization_priority_list();
        let efficiency_list = PriorityListFactory::get_cutting_efficiency_priority_list();

        // Waste minimization should prioritize area-related optimizations
        assert_eq!(waste_list[0], OptimizationPriority::LeastWastedArea.to_string());
        assert_eq!(waste_list[1], OptimizationPriority::BiggestUnusedTileArea.to_string());

        // Cutting efficiency should prioritize cut-related optimizations
        assert_eq!(efficiency_list[0], OptimizationPriority::LeastNbrCuts.to_string());
        assert_eq!(efficiency_list[1], OptimizationPriority::LeastNbrMosaics.to_string());

        // Both lists should contain all priorities
        assert_eq!(waste_list.len(), 9);
        assert_eq!(efficiency_list.len(), 9);
    }
}
