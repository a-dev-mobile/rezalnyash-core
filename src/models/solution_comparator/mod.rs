//! Solution comparator module for optimization priority-based sorting
//!
//! This module provides comparators for sorting solutions based on various
//! optimization criteria such as tile count, wasted area, cuts, etc.

pub mod optimization_priority;
pub mod priority_list_factory;
pub mod solution_comparator_factory;
pub mod comparators;

pub use optimization_priority::OptimizationPriority;
pub use priority_list_factory::PriorityListFactory;
pub use solution_comparator_factory::SolutionComparatorFactory;
pub use comparators::*;

use crate::models::task::Solution;
use std::cmp::Ordering;

/// Type alias for solution comparison functions
pub type SolutionComparator = fn(&Solution, &Solution) -> Ordering;

/// Result type for comparator operations
pub type ComparatorResult<T> = Result<T, ComparatorError>;

/// Errors that can occur during comparator operations
#[derive(Debug, Clone, PartialEq)]
pub enum ComparatorError {
    /// Unknown optimization priority string
    UnknownPriority(String),
    /// Invalid comparison data
    InvalidComparisonData(String),
    /// Missing required data for comparison
    MissingData(String),
}

impl std::fmt::Display for ComparatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComparatorError::UnknownPriority(priority) => {
                write!(f, "Unknown optimization priority: {}", priority)
            }
            ComparatorError::InvalidComparisonData(details) => {
                write!(f, "Invalid comparison data: {}", details)
            }
            ComparatorError::MissingData(details) => {
                write!(f, "Missing required data for comparison: {}", details)
            }
        }
    }
}

impl std::error::Error for ComparatorError {}

// Test modules
#[cfg(test)]
pub mod optimization_priority_test;
#[cfg(test)]
pub mod priority_list_factory_test;
#[cfg(test)]
pub mod solution_comparator_factory_test;
