//! Cut List Thread Builder
//!
//! This module provides a builder pattern for constructing CutListThread instances
//! with proper validation and error handling.

use crate::enums::CutOrientationPreference;
use crate::errors::{AppError, CoreError, Result};
use crate::models::{
    configuration::Configuration,
    cut_list_thread::{CutListThread, Solution, SolutionComparator, CutListLogger},
    stock::stock_solution::StockSolution,
    task::Task,
    tile_dimensions::TileDimensions,
};
use std::sync::{Arc, Mutex};

/// Builder for creating CutListThread instances
///
/// This builder follows the Rust builder pattern and provides a fluent interface
/// for constructing CutListThread objects with proper validation.
///
/// # Examples
///
/// ```
/// use rezalnyash_core::models::cut_list_thread::CutListThreadBuilder;
/// use rezalnyash_core::models::task::Task;
/// use std::sync::{Arc, Mutex};
///
/// let task = Arc::new(Mutex::new(Task::new("test-task".to_string())));
/// let builder = CutListThreadBuilder::new()
///     .set_group("test-group".to_string())
///     .set_task(task)
///     .set_accuracy_factor(10);
///
/// let result = builder.build();
/// assert!(result.is_ok());
/// ```
#[derive(Debug, Default)]
pub struct CutListThreadBuilder {
    /// Accuracy factor for solution pruning
    accuracy_factor: Option<i32>,
    /// All solutions across threads
    all_solutions: Option<Arc<Mutex<Vec<Solution>>>>,
    /// Auxiliary information
    aux_info: Option<String>,
    /// Configuration object
    configuration: Option<Configuration>,
    /// Cut list logger
    cut_list_logger: Option<Box<dyn CutListLogger>>,
    /// Cut thickness
    cut_thickness: Option<i32>,
    /// Final solution prioritized comparators
    final_solution_prioritized_comparators: Option<Vec<Box<dyn SolutionComparator>>>,
    /// First cut orientation preference
    first_cut_orientation: Option<CutOrientationPreference>,
    /// Thread group identifier
    group: Option<String>,
    /// Minimum trim dimension
    min_trim_dimension: Option<i32>,
    /// Stock solution
    stock_solution: Option<StockSolution>,
    /// Associated task
    task: Option<Arc<Mutex<Task>>>,
    /// Thread prioritized comparators
    thread_prioritized_comparators: Option<Vec<Box<dyn SolutionComparator>>>,
    /// Tiles to process
    tiles: Option<Vec<TileDimensions>>,
}

impl CutListThreadBuilder {
    /// Creates a new CutListThreadBuilder with default values
    ///
    /// # Returns
    /// A new builder instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the thread group identifier
    ///
    /// # Arguments
    /// * `group` - The group identifier
    ///
    /// # Returns
    /// Self for method chaining
    pub fn set_group(mut self, group: String) -> Self {
        self.group = Some(group);
        self
    }

    /// Sets the auxiliary information
    ///
    /// # Arguments
    /// * `aux_info` - The auxiliary information
    ///
    /// # Returns
    /// Self for method chaining
    pub fn set_aux_info(mut self, aux_info: String) -> Self {
        self.aux_info = Some(aux_info);
        self
    }

    /// Sets all solutions across threads
    ///
    /// # Arguments
    /// * `all_solutions` - Vector of solutions
    ///
    /// # Returns
    /// Self for method chaining
    pub fn set_all_solutions(mut self, all_solutions: Vec<Solution>) -> Self {
        self.all_solutions = Some(Arc::new(Mutex::new(all_solutions)));
        self
    }

    /// Sets all solutions with Arc<Mutex<Vec<Solution>>>
    ///
    /// # Arguments
    /// * `all_solutions` - Arc<Mutex<Vec<Solution>>>
    ///
    /// # Returns
    /// Self for method chaining
    pub fn set_all_solutions_arc(mut self, all_solutions: Arc<Mutex<Vec<Solution>>>) -> Self {
        self.all_solutions = Some(all_solutions);
        self
    }

    /// Sets the tiles to process
    ///
    /// # Arguments
    /// * `tiles` - Vector of tile dimensions
    ///
    /// # Returns
    /// Self for method chaining
    pub fn set_tiles(mut self, tiles: Vec<TileDimensions>) -> Self {
        self.tiles = Some(tiles);
        self
    }

    /// Sets the configuration
    ///
    /// # Arguments
    /// * `configuration` - The configuration object
    ///
    /// # Returns
    /// Self for method chaining
    pub fn set_configuration(mut self, configuration: Configuration) -> Self {
        self.configuration = Some(configuration);
        self
    }

    /// Sets the cut thickness
    ///
    /// # Arguments
    /// * `cut_thickness` - The cut thickness in scaled units
    ///
    /// # Returns
    /// Self for method chaining
    pub fn set_cut_thickness(mut self, cut_thickness: i32) -> Self {
        self.cut_thickness = Some(cut_thickness);
        self
    }

    /// Sets the minimum trim dimension
    ///
    /// # Arguments
    /// * `min_trim_dimension` - The minimum trim dimension in scaled units
    ///
    /// # Returns
    /// Self for method chaining
    pub fn set_min_trim_dimension(mut self, min_trim_dimension: i32) -> Self {
        self.min_trim_dimension = Some(min_trim_dimension);
        self
    }

    /// Sets the first cut orientation preference
    ///
    /// # Arguments
    /// * `first_cut_orientation` - The cut orientation preference
    ///
    /// # Returns
    /// Self for method chaining
    pub fn set_first_cut_orientation(mut self, first_cut_orientation: CutOrientationPreference) -> Self {
        self.first_cut_orientation = Some(first_cut_orientation);
        self
    }

    /// Sets the thread prioritized comparators
    ///
    /// # Arguments
    /// * `thread_prioritized_comparators` - Vector of solution comparators
    ///
    /// # Returns
    /// Self for method chaining
    pub fn set_thread_prioritized_comparators(
        mut self,
        thread_prioritized_comparators: Vec<Box<dyn SolutionComparator>>,
    ) -> Self {
        self.thread_prioritized_comparators = Some(thread_prioritized_comparators);
        self
    }

    /// Sets the final solution prioritized comparators
    ///
    /// # Arguments
    /// * `final_solution_prioritized_comparators` - Vector of solution comparators
    ///
    /// # Returns
    /// Self for method chaining
    pub fn set_final_solution_prioritized_comparators(
        mut self,
        final_solution_prioritized_comparators: Vec<Box<dyn SolutionComparator>>,
    ) -> Self {
        self.final_solution_prioritized_comparators = Some(final_solution_prioritized_comparators);
        self
    }

    /// Sets the associated task
    ///
    /// # Arguments
    /// * `task` - The task to associate with this thread
    ///
    /// # Returns
    /// Self for method chaining
    pub fn set_task(mut self, task: Arc<Mutex<Task>>) -> Self {
        self.task = Some(task);
        self
    }

    /// Sets the accuracy factor
    ///
    /// # Arguments
    /// * `accuracy_factor` - The accuracy factor for solution pruning
    ///
    /// # Returns
    /// Self for method chaining
    pub fn set_accuracy_factor(mut self, accuracy_factor: i32) -> Self {
        self.accuracy_factor = Some(accuracy_factor);
        self
    }

    /// Sets the stock solution
    ///
    /// # Arguments
    /// * `stock_solution` - The stock solution to use
    ///
    /// # Returns
    /// Self for method chaining
    pub fn set_stock_solution(mut self, stock_solution: StockSolution) -> Self {
        self.stock_solution = Some(stock_solution);
        self
    }

    /// Sets the cut list logger
    ///
    /// # Arguments
    /// * `cut_list_logger` - The logger implementation
    ///
    /// # Returns
    /// Self for method chaining
    pub fn set_cut_list_logger(mut self, cut_list_logger: Box<dyn CutListLogger>) -> Self {
        self.cut_list_logger = Some(cut_list_logger);
        self
    }

    /// Validates the builder state before building
    ///
    /// # Returns
    /// `Ok(())` if valid, `Err(CoreError)` if invalid
    fn validate(&self) -> Result<()> {
        // Validate required fields
        if self.task.is_none() {
            return Err(CoreError::InvalidInput {
                details: "Task is required".to_string(),
            }.into());
        }

        if self.stock_solution.is_none() {
            return Err(CoreError::InvalidInput {
                details: "Stock solution is required".to_string(),
            }.into());
        }

        // Validate accuracy factor
        if let Some(factor) = self.accuracy_factor {
            if factor <= 0 {
                return Err(CoreError::InvalidInput {
                    details: format!("Accuracy factor must be positive, got: {}", factor),
                }.into());
            }
        }

        // Validate cut thickness
        if let Some(thickness) = self.cut_thickness {
            if thickness < 0 {
                return Err(CoreError::InvalidInput {
                    details: format!("Cut thickness cannot be negative, got: {}", thickness),
                }.into());
            }
        }

        // Validate min trim dimension
        if let Some(dimension) = self.min_trim_dimension {
            if dimension < 0 {
                return Err(CoreError::InvalidInput {
                    details: format!("Min trim dimension cannot be negative, got: {}", dimension),
                }.into());
            }
        }

        // Validate configuration if present
        if let Some(ref config) = self.configuration {
            config.validate().map_err(|errors| {
                CoreError::InvalidInput {
                    details: format!("Configuration validation failed: {:?}", errors),
                }
            })?;
        }

        Ok(())
    }

    /// Builds the CutListThread instance
    ///
    /// # Returns
    /// `Ok(CutListThread)` if successful, `Err(CoreError)` if validation fails
    ///
    /// # Errors
    /// Returns an error if:
    /// - Required fields are missing (task, stock_solution)
    /// - Validation fails for any field
    /// - Configuration is invalid
    pub fn build(self) -> Result<CutListThread> {
        // Validate before building
        self.validate()?;

        let mut cut_list_thread = CutListThread::new();

        // Set required fields (we know these exist due to validation)
        cut_list_thread.set_task(self.task);
        cut_list_thread.set_stock_solution(self.stock_solution);

        // Set optional fields with defaults
        cut_list_thread.set_group(self.group);
        cut_list_thread.set_aux_info(self.aux_info);

        if let Some(all_solutions) = self.all_solutions {
            cut_list_thread.set_all_solutions(all_solutions);
        }

        if let Some(tiles) = self.tiles {
            cut_list_thread.set_tiles(tiles);
        }

        // Set configuration-derived values
        if let Some(config) = self.configuration {
            cut_list_thread.set_consider_grain_direction(config.consider_orientation());
            
            // Set first cut orientation from configuration if not explicitly set
            if self.first_cut_orientation.is_none() {
                cut_list_thread.set_first_cut_orientation(config.cut_orientation_preference());
            }
        }

        // Set explicit values (these override configuration)
        if let Some(cut_thickness) = self.cut_thickness {
            cut_list_thread.set_cut_thickness(cut_thickness);
        }

        if let Some(min_trim_dimension) = self.min_trim_dimension {
            cut_list_thread.set_min_trim_dimension(min_trim_dimension);
        }

        if let Some(first_cut_orientation) = self.first_cut_orientation {
            cut_list_thread.set_first_cut_orientation(first_cut_orientation);
        }

        if let Some(thread_comparators) = self.thread_prioritized_comparators {
            cut_list_thread.set_thread_prioritized_comparators(thread_comparators);
        }

        if let Some(final_comparators) = self.final_solution_prioritized_comparators {
            cut_list_thread.set_final_solution_prioritized_comparators(final_comparators);
        }

        if let Some(accuracy_factor) = self.accuracy_factor {
            cut_list_thread.set_accuracy_factor(accuracy_factor);
        }

        if let Some(logger) = self.cut_list_logger {
            cut_list_thread.set_cut_list_logger(Some(logger));
        }

        Ok(cut_list_thread)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::task::Task;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_builder_new() {
        let builder = CutListThreadBuilder::new();
        assert!(builder.group.is_none());
        assert!(builder.task.is_none());
    }

    #[test]
    fn test_builder_set_group() {
        let builder = CutListThreadBuilder::new()
            .set_group("test-group".to_string());
        assert_eq!(builder.group, Some("test-group".to_string()));
    }

    #[test]
    fn test_builder_set_aux_info() {
        let builder = CutListThreadBuilder::new()
            .set_aux_info("test-aux".to_string());
        assert_eq!(builder.aux_info, Some("test-aux".to_string()));
    }

    #[test]
    fn test_builder_set_accuracy_factor() {
        let builder = CutListThreadBuilder::new()
            .set_accuracy_factor(15);
        assert_eq!(builder.accuracy_factor, Some(15));
    }

    #[test]
    fn test_builder_set_cut_thickness() {
        let builder = CutListThreadBuilder::new()
            .set_cut_thickness(5);
        assert_eq!(builder.cut_thickness, Some(5));
    }

    #[test]
    fn test_builder_set_min_trim_dimension() {
        let builder = CutListThreadBuilder::new()
            .set_min_trim_dimension(10);
        assert_eq!(builder.min_trim_dimension, Some(10));
    }

    #[test]
    fn test_builder_validation_missing_task() {
        let builder = CutListThreadBuilder::new();
        let result = builder.validate();
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Task is required"));
        }
    }

    #[test]
    fn test_builder_validation_negative_accuracy_factor() {
        let task = Arc::new(Mutex::new(Task::new("test".to_string())));
        let stock_solution = StockSolution::new(vec![]);
        let builder = CutListThreadBuilder::new()
            .set_task(task)
            .set_stock_solution(stock_solution)
            .set_accuracy_factor(-1);
        let result = builder.validate();
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Accuracy factor must be positive"));
        }
    }

    #[test]
    fn test_builder_validation_negative_cut_thickness() {
        let task = Arc::new(Mutex::new(Task::new("test".to_string())));
        let stock_solution = StockSolution::new(vec![]);
        let builder = CutListThreadBuilder::new()
            .set_task(task)
            .set_stock_solution(stock_solution)
            .set_cut_thickness(-1);
        let result = builder.validate();
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Cut thickness cannot be negative"));
        }
    }

    #[test]
    fn test_builder_validation_negative_min_trim_dimension() {
        let task = Arc::new(Mutex::new(Task::new("test".to_string())));
        let stock_solution = StockSolution::new(vec![]);
        let builder = CutListThreadBuilder::new()
            .set_task(task)
            .set_stock_solution(stock_solution)
            .set_min_trim_dimension(-1);
        let result = builder.validate();
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Min trim dimension cannot be negative"));
        }
    }
}
