// TODO: Implement StockSolution model
// use crate::models::stock_solution::StockSolution;

/// Result of stock solution generation
#[derive(Debug, Clone)]
pub enum StockSolutionResult {
    /// A valid solution was found
    /// TODO: Replace String with proper StockSolution type when implemented
    Solution(String),
    /// No solution could be found with current constraints
    NoSolution,
    /// All possible solutions have been excluded
    AllExcluded,
}
