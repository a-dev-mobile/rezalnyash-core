//! Cut List Thread Builder - Java to Rust Mapping
//!
//! This module provides a builder pattern for constructing CutListThread instances
//! with proper validation and error handling.
//!
//! JAVA-RUST MAPPING:
//! ==================
//! Java Class: CutListThreadBuilder
//! Rust Struct: CutListThreadBuilder
//!
//! Java Fields -> Rust Fields:
//! - accuracyFactor -> accuracy_factor
//! - allSolutions -> all_solutions
//! - auxInfo -> aux_info
//! - configuration -> configuration
//! - cutListLogger -> cut_list_logger
//! - cutThickness -> cut_thickness
//! - finalSolutionPrioritizedComparators -> final_solution_prioritized_comparators
//! - firstCutOrientation -> first_cut_orientation
//! - group -> group
//! - minTrimDimension -> min_trim_dimension
//! - stockSolution -> stock_solution
//! - task -> task
//! - threadPrioritizedComparators -> thread_prioritized_comparators
//! - tiles -> tiles

use crate::enums::CutOrientationPreference;
use crate::errors::{AppError, CoreError, Result};
use crate::models::{
    configuration::Configuration,
    cut_list_thread::{CutListLogger, CutListThread, Solution, SolutionComparator},
    stock::stock_solution::StockSolution,
    task::Task,
    tile_dimensions::TileDimensions,
};
use std::sync::{Arc, Mutex};

/// Builder for creating CutListThread instances
///
/// JAVA EQUIVALENT: com.example.debug.engine.CutListThreadBuilder
///
/// This builder follows the Rust builder pattern and provides a fluent interface
/// for constructing CutListThread objects with proper validation.
#[derive(Debug, Default)]
pub struct CutListThreadBuilder {
    /// JAVA: accuracyFactor
    /// Accuracy factor for solution pruning
    accuracy_factor: Option<i32>,

    /// JAVA: allSolutions
    /// All solutions across threads
    all_solutions: Option<Arc<Mutex<Vec<Solution>>>>,

    /// JAVA: auxInfo
    /// Auxiliary information
    aux_info: Option<String>,

    /// JAVA: configuration
    /// Configuration object
    configuration: Option<Configuration>,

    /// JAVA: cutListLogger
    /// Cut list logger
    has_logger: bool,

    /// JAVA: cutThickness
    /// Cut thickness
    cut_thickness: Option<i32>,

    /// Final solution prioritized comparators count
    final_solution_comparator_count: Option<usize>,
    /// JAVA: firstCutOrientation
    /// First cut orientation preference
    first_cut_orientation: Option<CutOrientationPreference>,

    /// JAVA: group
    /// Thread group identifier
    group: Option<String>,

    /// JAVA: minTrimDimension
    /// Minimum trim dimension
    min_trim_dimension: Option<i32>,

    /// JAVA: stockSolution
    /// Stock solution
    stock_solution: Option<StockSolution>,

    /// JAVA: task
    /// Associated task
    task: Option<Arc<Mutex<Task>>>,

    /// Thread prioritized comparators count
    thread_comparator_count: Option<usize>,
    /// JAVA: tiles
    /// Tiles to process
    tiles: Option<Vec<TileDimensions>>,
}

impl CutListThreadBuilder {
    /// Creates a new CutListThreadBuilder with default values
    ///
    /// JAVA EQUIVALENT: new CutListThreadBuilder()
    pub fn new() -> Self {
        Self::default()
    }

    /// JAVA: setGroup(String str)
    /// Sets the thread group identifier
    pub fn set_group(mut self, group: String) -> Self {
        self.group = Some(group);
        self
    }

    /// JAVA: setAuxInfo(String str)
    /// Sets the auxiliary information
    pub fn set_aux_info(mut self, aux_info: String) -> Self {
        self.aux_info = Some(aux_info);
        self
    }

    /// JAVA: setAllSolutions(List<Solution> list)
    /// Sets all solutions across threads
    pub fn set_all_solutions(mut self, all_solutions: Vec<Solution>) -> Self {
        self.all_solutions = Some(Arc::new(Mutex::new(all_solutions)));
        self
    }

    /// Alternative method for setting all solutions with Arc<Mutex<Vec<Solution>>>
    /// JAVA: setAllSolutions(List<Solution> list) - alternative approach
    pub fn set_all_solutions_arc(mut self, all_solutions: Arc<Mutex<Vec<Solution>>>) -> Self {
        self.all_solutions = Some(all_solutions);
        self
    }

    /// JAVA: setTiles(List<TileDimensions> list)
    /// Sets the tiles to process
    pub fn set_tiles(mut self, tiles: Vec<TileDimensions>) -> Self {
        self.tiles = Some(tiles);
        self
    }

    /// JAVA: setConfiguration(Configuration configuration)
    /// Sets the configuration
    pub fn set_configuration(mut self, configuration: Configuration) -> Self {
        self.configuration = Some(configuration);
        self
    }

    /// JAVA: setCutThickness(int i)
    /// Sets the cut thickness
    pub fn set_cut_thickness(mut self, cut_thickness: i32) -> Self {
        self.cut_thickness = Some(cut_thickness);
        self
    }

    /// JAVA: setMinTrimDimension(int i)
    /// Sets the minimum trim dimension
    pub fn set_min_trim_dimension(mut self, min_trim_dimension: i32) -> Self {
        self.min_trim_dimension = Some(min_trim_dimension);
        self
    }

    /// JAVA: setFirstCutOrientation(CutDirection cutDirection)
    /// Sets the first cut orientation preference
    pub fn set_first_cut_orientation(
        mut self,
        first_cut_orientation: CutOrientationPreference,
    ) -> Self {
        self.first_cut_orientation = Some(first_cut_orientation);
        self
    }




    /// JAVA: setTask(Task task)
    /// Sets the associated task
    pub fn set_task(mut self, task: Arc<Mutex<Task>>) -> Self {
        self.task = Some(task);
        self
    }

    /// JAVA: setAccuracyFactor(int i)
    /// Sets the accuracy factor
    pub fn set_accuracy_factor(mut self, accuracy_factor: i32) -> Self {
        self.accuracy_factor = Some(accuracy_factor);
        self
    }

    /// JAVA: setStockSolution(StockSolution stockSolution)
    /// Sets the stock solution
    pub fn set_stock_solution(mut self, stock_solution: StockSolution) -> Self {
        self.stock_solution = Some(stock_solution);
        self
    }



    /// Validates the builder state before building
    ///
    /// JAVA EQUIVALENT: No direct equivalent - validation is done implicitly
    fn validate(&self) -> Result<()> {
        // Validate required fields
        if self.task.is_none() {
            return Err(CoreError::InvalidInput {
                details: "Task is required".to_string(),
            }
            .into());
        }

        if self.stock_solution.is_none() {
            return Err(CoreError::InvalidInput {
                details: "Stock solution is required".to_string(),
            }
            .into());
        }

        // Validate accuracy factor
        if let Some(factor) = self.accuracy_factor {
            if factor <= 0 {
                return Err(CoreError::InvalidInput {
                    details: format!("Accuracy factor must be positive, got: {}", factor),
                }
                .into());
            }
        }

        // Validate cut thickness
        if let Some(thickness) = self.cut_thickness {
            if thickness < 0 {
                return Err(CoreError::InvalidInput {
                    details: format!("Cut thickness cannot be negative, got: {}", thickness),
                }
                .into());
            }
        }

        // Validate min trim dimension
        if let Some(dimension) = self.min_trim_dimension {
            if dimension < 0 {
                return Err(CoreError::InvalidInput {
                    details: format!("Min trim dimension cannot be negative, got: {}", dimension),
                }
                .into());
            }
        }

        // Validate configuration if present
        if let Some(ref config) = self.configuration {
            config
                .validate()
                .map_err(|errors| CoreError::InvalidInput {
                    details: format!("Configuration validation failed: {:?}", errors),
                })?;
        }

        Ok(())
    }

    /// JAVA: build()
    /// Builds the CutListThread instance
    ///
    /// JAVA CODE EQUIVALENT:
    /// ```java
    /// public CutListThread build() {
    ///     CutListThread cutListThread = new CutListThread();
    ///     cutListThread.setGroup(this.group);
    ///     cutListThread.setAuxInfo(this.auxInfo);
    ///     cutListThread.setAllSolutions(this.allSolutions);
    ///     cutListThread.setTiles(this.tiles);
    ///     cutListThread.setConsiderGrainDirection(this.configuration.isConsiderOrientation());
    ///     cutListThread.setCutThickness(this.cutThickness);
    ///     cutListThread.setMinTrimDimension(this.minTrimDimension);
    ///     cutListThread.setFirstCutOrientation(this.firstCutOrientation);
    ///     cutListThread.setThreadPrioritizedComparators(this.threadPrioritizedComparators);
    ///     cutListThread.setFinalSolutionPrioritizedComparators(this.finalSolutionPrioritizedComparators);
    ///     cutListThread.setTask(this.task);
    ///     cutListThread.setAccuracyFactor(this.accuracyFactor);
    ///     cutListThread.setStockSolution(this.stockSolution);
    ///     cutListThread.setCutListLogger(this.cutListLogger);
    ///     this.task.addThread(cutListThread);
    ///     return cutListThread;
    /// }
    /// ```
    pub fn build(self) -> Result<CutListThread> {
        // Validate before building
        self.validate()?;

        // JAVA: CutListThread cutListThread = new CutListThread();
        let mut cut_list_thread = CutListThread::new();

        // Set task ID instead of task reference
        if let Some(task_arc) = &self.task {
            if let Ok(task_guard) = task_arc.lock() {
                cut_list_thread.set_task_id(Some(task_guard.id.clone()));
            }
        }

        // JAVA: cutListThread.setStockSolution(this.stockSolution);
        cut_list_thread.set_stock_solution(self.stock_solution);

        // Set optional fields with defaults
        // JAVA: cutListThread.setGroup(this.group);
        cut_list_thread.set_group(self.group);

        // JAVA: cutListThread.setAuxInfo(this.auxInfo);
        cut_list_thread.set_aux_info(self.aux_info);

        // JAVA: cutListThread.setAllSolutions(this.allSolutions);
        if let Some(all_solutions) = self.all_solutions {
            cut_list_thread.set_all_solutions(all_solutions);
        }

        // JAVA: cutListThread.setTiles(this.tiles);
        if let Some(tiles) = self.tiles {
            cut_list_thread.set_tiles(tiles);
        }

        // Set configuration-derived values
        // JAVA: cutListThread.setConsiderGrainDirection(this.configuration.isConsiderOrientation());
        if let Some(config) = self.configuration {
            cut_list_thread.set_consider_grain_direction(config.consider_orientation());

            // Set first cut orientation from configuration if not explicitly set
            if self.first_cut_orientation.is_none() {
                cut_list_thread.set_first_cut_orientation(config.cut_orientation_preference());
            }
        }

        // Set explicit values (these override configuration)
        // JAVA: cutListThread.setCutThickness(this.cutThickness);
        if let Some(cut_thickness) = self.cut_thickness {
            cut_list_thread.set_cut_thickness(cut_thickness);
        }

        // JAVA: cutListThread.setMinTrimDimension(this.minTrimDimension);
        if let Some(min_trim_dimension) = self.min_trim_dimension {
            cut_list_thread.set_min_trim_dimension(min_trim_dimension);
        }

        // JAVA: cutListThread.setFirstCutOrientation(this.firstCutOrientation);
        if let Some(first_cut_orientation) = self.first_cut_orientation {
            cut_list_thread.set_first_cut_orientation(first_cut_orientation);
        }

        if let Some(count) = self.thread_comparator_count {
            cut_list_thread.thread_comparator_count = count;
        }

        if let Some(count) = self.final_solution_comparator_count {
            cut_list_thread.final_comparator_count = count;
        }

        if let Some(accuracy_factor) = self.accuracy_factor {
            cut_list_thread.set_accuracy_factor(accuracy_factor);
        }

        cut_list_thread.has_logger = self.has_logger;

        // Add thread ID to task
        if let Some(task_arc) = &self.task {
            if let Ok(mut task_guard) = task_arc.lock() {
                task_guard.add_thread_id(cut_list_thread.id.clone());
            }
        }

        Ok(cut_list_thread)
    }

    /// Sets the cut list logger
    pub fn set_cut_list_logger(mut self, _logger: Box<dyn CutListLogger>) -> Self {
        self.has_logger = true;
        self
    }

    /// Sets the thread prioritized comparators
    pub fn set_thread_prioritized_comparators(
        mut self,
        comparators: Vec<Box<dyn SolutionComparator>>,
    ) -> Self {
        self.thread_comparator_count = Some(comparators.len());
        self
    }

    /// Sets the final solution prioritized comparators
    pub fn set_final_solution_prioritized_comparators(
        mut self,
        comparators: Vec<Box<dyn SolutionComparator>>,
    ) -> Self {
        self.final_solution_comparator_count = Some(comparators.len());
        self
    }
}

/// JAVA-RUST METHOD MAPPING REFERENCE:
/// ===================================
///
/// Java Method                           -> Rust Method
/// -----------                           -> -----------
/// setGroup(String)                      -> set_group(String)
/// setAuxInfo(String)                    -> set_aux_info(String)
/// setAllSolutions(List<Solution>)       -> set_all_solutions(Vec<Solution>)
/// setTiles(List<TileDimensions>)        -> set_tiles(Vec<TileDimensions>)
/// setConfiguration(Configuration)       -> set_configuration(Configuration)
/// setCutThickness(int)                  -> set_cut_thickness(i32)
/// setMinTrimDimension(int)              -> set_min_trim_dimension(i32)
/// setFirstCutOrientation(CutDirection)  -> set_first_cut_orientation(CutOrientationPreference)
/// setThreadPrioritizedComparators(...)  -> set_thread_prioritized_comparators(...)
/// setFinalSolutionPrioritizedComparators(...) -> set_final_solution_prioritized_comparators(...)
/// setTask(Task)                         -> set_task(Arc<Mutex<Task>>)
/// setAccuracyFactor(int)                -> set_accuracy_factor(i32)
/// setStockSolution(StockSolution)       -> set_stock_solution(StockSolution)
/// setCutListLogger(CutListLogger)       -> set_cut_list_logger(Box<dyn CutListLogger>)
/// build()                               -> build() -> Result<CutListThread>

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
        let builder = CutListThreadBuilder::new().set_group("test-group".to_string());
        assert_eq!(builder.group, Some("test-group".to_string()));
    }

    #[test]
    fn test_builder_set_aux_info() {
        let builder = CutListThreadBuilder::new().set_aux_info("test-aux".to_string());
        assert_eq!(builder.aux_info, Some("test-aux".to_string()));
    }

    #[test]
    fn test_builder_set_accuracy_factor() {
        let builder = CutListThreadBuilder::new().set_accuracy_factor(15);
        assert_eq!(builder.accuracy_factor, Some(15));
    }

    #[test]
    fn test_builder_set_cut_thickness() {
        let builder = CutListThreadBuilder::new().set_cut_thickness(5);
        assert_eq!(builder.cut_thickness, Some(5));
    }

    #[test]
    fn test_builder_set_min_trim_dimension() {
        let builder = CutListThreadBuilder::new().set_min_trim_dimension(10);
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
            assert!(e
                .to_string()
                .contains("Min trim dimension cannot be negative"));
        }
    }
}
