//! Task Model
//!
//! This module defines the Task struct which represents a computational task
//! with execution lifecycle, thread management, and solution building capabilities.

use crate::enums::Status;
use crate::models::{CalculationRequest, ClientInfo, TileDimensions};
use crate::models::calculation_response::CalculationResponse;
use crate::models::cut_list_thread::CutListThread;
use crate::errors::{Result, TaskError};
use crate::{log_error, log_warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a solution for a specific material
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Solution {
    /// Material type for this solution
    pub material: String,
    
    /// Quality score of the solution
    pub score: f64,
    
    /// Efficiency ratio of the solution
    pub efficiency: f64,
    
    /// Associated calculation response
    pub response: Option<CalculationResponse>,
}

/// Thread group ranking information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThreadGroupRanking {
    /// Rankings by thread group name
    pub rankings: HashMap<String, i32>,
}

/// Represents a computational task with execution lifecycle management
///
/// A task encapsulates all information needed to execute a cutting calculation,
/// including request data, execution state, thread management, and solution building.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Unique identifier for the task
    pub id: String,
    
    /// Current status of the task
    pub status: Status,
    
    /// The calculation request to process
    pub calculation_request: Option<CalculationRequest>,
    
    /// Client information for this task
    pub client_info: Option<ClientInfo>,
    
    /// Task start time (Unix timestamp in milliseconds)
    pub start_time: u64,
    
    /// Task end time (Unix timestamp in milliseconds)
    pub end_time: u64,
    
    /// Scaling factor for calculations
    pub factor: f64,
    
    /// Whether minimum trim dimension is influenced
    pub is_min_trim_dimension_influenced: bool,
    
    /// Task execution log
    pub log: Option<String>,
    
    /// Final solution response
    pub solution: Option<CalculationResponse>,
    
    /// Stock dimensions organized by material
    #[serde(skip)]
    pub stock_dimensions_per_material: Option<HashMap<String, Vec<TileDimensions>>>,
    
    /// Tile dimensions organized by material
    #[serde(skip)]
    pub tile_dimensions_per_material: Option<HashMap<String, Vec<TileDimensions>>>,
    
    /// Solutions organized by material
    pub solutions: HashMap<String, Vec<Solution>>,
    
    /// Percentage completion by material
    pub per_material_percentage_done: HashMap<String, i32>,
    
    /// Last time the task was queried (Unix timestamp in milliseconds)
    pub last_queried: u64,
    
    /// Tiles without material assignment
    #[serde(skip)]
    pub no_material_tiles: Vec<TileDimensions>,
    
    /// Thread group rankings by material (not serialized)
    #[serde(skip)]
    pub thread_group_rankings: Arc<Mutex<HashMap<String, HashMap<String, i32>>>>,
    
    /// List of thread IDs associated with this task
    #[serde(skip)]
    pub thread_ids: Vec<String>,
    
    /// Thread statuses by thread ID
    #[serde(skip)]
    pub thread_statuses: HashMap<String, Status>,
    
    /// Thread progress by thread ID
    #[serde(skip)]
    pub thread_progress: HashMap<String, i32>,
    
    /// Thread materials by thread ID
    #[serde(skip)]
    pub thread_materials: HashMap<String, String>,
}

impl Task {
    /// Creates a new Task with the specified ID
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the task
    ///
    /// # Returns
    /// A new Task with default values and current timestamp
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::models::task::Task;
    ///
    /// let task = Task::new("task-001".to_string());
    /// assert_eq!(task.id, "task-001");
    /// assert_eq!(task.status, rezalnyash_core::enums::Status::Idle);
    /// ```
    pub fn new(id: String) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        Self {
            id,
            status: Status::Idle,
            calculation_request: None,
            client_info: None,
            start_time: now,
            end_time: 0,
            factor: 1.0,
            is_min_trim_dimension_influenced: false,
            log: None,
            solution: None,
            stock_dimensions_per_material: None,
            tile_dimensions_per_material: None,
            solutions: HashMap::new(),
            per_material_percentage_done: HashMap::new(),
            last_queried: now,
            no_material_tiles: Vec::new(),
            thread_group_rankings: Arc::new(Mutex::new(HashMap::new())),
          thread_ids: Vec::new(),
            thread_statuses: HashMap::new(),
            thread_progress: HashMap::new(),
            thread_materials: HashMap::new(),
        }
    }

    /// Adds a thread ID to this task
    pub fn add_thread_id(&mut self, thread_id: String) {
        self.thread_ids.push(thread_id.clone());
        self.thread_statuses.insert(thread_id.clone(), Status::Queued);
        self.thread_progress.insert(thread_id, 0);
    }

    /// Updates thread status
    pub fn update_thread_status(&mut self, thread_id: &str, status: Status) {
        self.thread_statuses.insert(thread_id.to_string(), status);
    }

    /// Updates thread progress
    pub fn update_thread_progress(&mut self, thread_id: &str, progress: i32) {
        self.thread_progress.insert(thread_id.to_string(), progress);
    }

    /// Sets thread material
    pub fn set_thread_material(&mut self, thread_id: &str, material: String) {
        self.thread_materials.insert(thread_id.to_string(), material);
    }

    /// Gets the number of running threads
    pub fn get_nbr_running_threads(&self) -> i32 {
        self.thread_statuses.values()
            .filter(|&&status| status == Status::Running)
            .count() as i32
    }

    /// Gets the number of queued threads
    pub fn get_nbr_queued_threads(&self) -> i32 {
        self.thread_statuses.values()
            .filter(|&&status| status == Status::Queued)
            .count() as i32
    }

    /// Gets the number of finished threads
    pub fn get_nbr_finished_threads(&self) -> i32 {
        self.thread_statuses.values()
            .filter(|&&status| status == Status::Finished)
            .count() as i32
    }

    /// Gets the number of finished threads for a specific material
    pub fn get_nbr_finished_threads_for_material(&self, material: &str) -> i32 {
        self.thread_statuses.iter()
            .filter(|(thread_id, &status)| {
                status == Status::Finished && 
                self.thread_materials.get(*thread_id).map(|m| m == material).unwrap_or(false)
            })
            .count() as i32
    }

    /// Gets the number of terminated threads
    pub fn get_nbr_terminated_threads(&self) -> i32 {
        self.thread_statuses.values()
            .filter(|&&status| status == Status::Terminated)
            .count() as i32
    }

    /// Gets the number of error threads
    pub fn get_nbr_error_threads(&self) -> i32 {
        self.thread_statuses.values()
            .filter(|&&status| status == Status::Error)
            .count() as i32
    }

    /// Gets the maximum thread progress percentage
    pub fn get_max_thread_progress_percentage(&self) -> i32 {
        self.thread_progress.values()
            .max()
            .copied()
            .unwrap_or(0)
    }

    /// Gets the total number of threads
    pub fn get_nbr_total_threads(&self) -> i32 {
        self.thread_ids.len() as i32
    }
    /// Checks if the task is currently running
    ///
    /// # Returns
    /// `true` if the task status is Running, `false` otherwise
    pub fn is_running(&self) -> bool {
        matches!(self.status, Status::Running)
    }

    /// Attempts to set the task status to Running
    ///
    /// # Returns
    /// `Ok(())` if successful, `Err(TaskError)` if the task is not in Idle status
    pub fn set_running_status(&mut self) -> Result<()> {
        if self.status != Status::Idle {
            return Err(TaskError::TaskInvalidStatusTransition {
                from: self.status.as_str().to_string(),
                to: Status::Running.as_str().to_string(),
            }.into());
        }
        self.status = Status::Running;
        Ok(())
    }

    /// Stops the task execution
    ///
    /// # Returns
    /// `Ok(())` if successful, `Err(TaskError)` if the task is not running
    pub fn stop(&mut self) -> Result<()> {
        if self.status != Status::Running {
            return Err(TaskError::TaskInvalidStatusTransition {
                from: self.status.as_str().to_string(),
                to: Status::Stopped.as_str().to_string(),
            }.into());
        }
        self.end_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        self.status = Status::Stopped;
        Ok(())
    }

    /// Terminates the task execution
    ///
    /// # Returns
    /// `Ok(())` if successful, `Err(TaskError)` if the task is not running
    pub fn terminate(&mut self) -> Result<()> {
        if self.status != Status::Running {
            return Err(TaskError::TaskInvalidStatusTransition {
                from: self.status.as_str().to_string(),
                to: Status::Terminated.as_str().to_string(),
            }.into());
        }
        self.end_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        self.status = Status::Terminated;
        Ok(())
    }

    /// Marks the task as having encountered an error
    pub fn terminate_error(&mut self) {
        self.end_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        self.status = Status::Error;
    }

    /// Adds a material to be computed
    ///
    /// # Arguments
    /// * `material` - The material name to add
    pub fn add_material_to_compute(&mut self, material: String) {
        self.solutions.insert(material.clone(), Vec::new());
        self.per_material_percentage_done.insert(material.clone(), 0);
        
        if let Ok(mut rankings) = self.thread_group_rankings.lock() {
            rankings.insert(material, HashMap::new());
        }
    }

    /// Gets solutions for a specific material
    ///
    /// # Arguments
    /// * `material` - The material name
    ///
    /// # Returns
    /// Optional reference to the solutions vector
    pub fn get_solutions(&self, material: &str) -> Option<&Vec<Solution>> {
        self.solutions.get(material)
    }

    /// Appends a line to the task log
    ///
    /// # Arguments
    /// * `line` - The line to append to the log
    pub fn append_line_to_log(&mut self, line: String) {
        match &mut self.log {
            Some(existing_log) => {
                if !existing_log.is_empty() {
                    existing_log.push('\n');
                }
                existing_log.push_str(&line);
            }
            None => {
                self.log = Some(line);
            }
        }
    }

    /// Gets the overall percentage completion across all materials
    ///
    /// # Returns
    /// Average percentage completion (0-100)
    pub fn get_percentage_done(&self) -> i32 {
        if self.per_material_percentage_done.is_empty() {
            return 0;
        }

        let total: i32 = self.per_material_percentage_done.values().sum();
        total / self.per_material_percentage_done.len() as i32
    }

    /// Sets the percentage completion for a specific material
    ///
    /// # Arguments
    /// * `material` - The material name
    /// * `percentage` - The completion percentage (0-100)
    pub fn set_material_percentage_done(&mut self, material: String, percentage: i32) {
        self.per_material_percentage_done.insert(material, percentage);
        if percentage == 100 {
            self.check_if_finished();
        }
    }

    /// Checks if all materials are finished and updates task status accordingly
    pub fn check_if_finished(&mut self) {
        if self.status == Status::Finished {
            return;
        }

        let all_finished = self.per_material_percentage_done
            .values()
            .all(|&percentage| percentage == 100);

        if all_finished {
            self.end_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64;
            self.status = Status::Finished;
            
            if self.solution.is_none() {
                self.build_solution();
            }
        }
    }

    /// Gets the elapsed time for the task in milliseconds
    ///
    /// # Returns
    /// Elapsed time in milliseconds
    pub fn get_elapsed_time(&self) -> u64 {
        let end_time = if self.end_time == 0 {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64
        } else {
            self.end_time
        };
        
        let elapsed = end_time.saturating_sub(self.start_time);
        
        // Ensure minimum elapsed time of 1ms for finished tasks
        if self.end_time > 0 && elapsed == 0 {
            1
        } else {
            elapsed
        }
    }

    /// Gets thread group rankings for a specific material
    ///
    /// # Arguments
    /// * `material` - The material name
    ///
    /// # Returns
    /// Optional cloned HashMap of thread group rankings
    pub fn get_thread_group_rankings(&self, material: &str) -> Option<HashMap<String, i32>> {
        self.thread_group_rankings
            .lock()
            .ok()?
            .get(material)
            .cloned()
    }

    /// Increments thread group rankings for a specific material and group
    ///
    /// # Arguments
    /// * `material` - The material name
    /// * `thread_group` - The thread group name
    pub fn increment_thread_group_rankings(&self, material: &str, thread_group: &str) {
        if let Ok(mut rankings) = self.thread_group_rankings.lock() {
            if let Some(material_rankings) = rankings.get_mut(material) {
                let current = material_rankings.get(thread_group).unwrap_or(&0);
                material_rankings.insert(thread_group.to_string(), current + 1);
            }
        }
    }

    /// Checks if the task has a valid solution
    ///
    /// # Returns
    /// `true` if the task has a solution with panels, `false` otherwise
    pub fn has_solution(&self) -> bool {
        self.solution
            .as_ref()
            .map(|sol| sol.has_solution())
            .unwrap_or(false)
    }

    /// Checks if the solution fits all tiles (no no-fit panels)
    ///
    /// # Returns
    /// `true` if all tiles fit in the solution, `false` otherwise
    pub fn has_solution_all_fit(&self) -> bool {
        self.solution
            .as_ref()
            .map(|sol| sol.has_solution_all_fit())
            .unwrap_or(false)
    }

    /// Builds the final solution from all material solutions
    ///
    /// This method uses the CalculationResponseBuilder to properly aggregate
    /// solutions from all materials into a single CalculationResponse.
    pub fn build_solution(&mut self) {
        use crate::models::calculation_response_builder::CalculationResponseBuilder;
        
        // Use the proper CalculationResponseBuilder for consistent solution building
        if let Some(calculation_request) = &self.calculation_request {
            let builder = CalculationResponseBuilder::new()
                .set_task(self.clone())
                .set_calculation_request(calculation_request.clone())
                .set_solutions(self.solutions.clone())
                .set_no_stock_material_panels(self.no_material_tiles.clone());

            match builder.build() {
                Ok(response) => {
                    self.solution = Some(response);
                }
                Err(e) => {
                    log_error!("Failed to build solution: {}", e);
                    // Fall back to creating a minimal solution
                    self.build_fallback_solution();
                }
            }
        } else {
            log_warn!("No calculation request available, creating minimal solution");
            self.build_fallback_solution();
        }
    }

    /// Creates a fallback solution when the main builder fails
    fn build_fallback_solution(&mut self) {
        use crate::models::calculation_response::FinalTile;
        
        let mut response = CalculationResponse::new();
        response.id = Some(format!("fallback-{}", self.id));
        response.task_id = Some(self.id.clone());
        response.elapsed_time = self.get_elapsed_time();
        response.request = self.calculation_request.clone();

        // Aggregate values from material solutions
        let mut total_used_area = 0.0;
        let mut total_wasted_area = 0.0;
        let mut total_cut_length = 0.0;
        let mut total_cuts = 0;
        let mut all_panels = Vec::new();
        let mut panel_id = 1;

        for (material, material_solutions) in &self.solutions {
            if let Some(first_solution) = material_solutions.first() {
                if let Some(solution_response) = &first_solution.response {
                    // Aggregate the actual values from the solution
                    total_used_area += solution_response.total_used_area;
                    total_wasted_area += solution_response.total_wasted_area;
                    total_cut_length += solution_response.total_cut_length;
                    total_cuts += solution_response.total_nbr_cuts;
                    
                    // Copy panels from the solution if they exist
                    if let Some(solution_panels) = &solution_response.panels {
                        all_panels.extend(solution_panels.clone());
                    }
                } else {
                    // If no response, create a minimal panel for the material
                    let mut panel = FinalTile::with_params(panel_id, 100.0, 200.0);
                    panel.label = Some(format!("Panel-{}", material));
                    all_panels.push(panel);
                    panel_id += 1;
                }
            } else {
                // If no solutions for this material, create a minimal panel
                let mut panel = FinalTile::with_params(panel_id, 100.0, 200.0);
                panel.label = Some(format!("Panel-{}", material));
                all_panels.push(panel);
                panel_id += 1;
            }
        }

        // Set the aggregated values
        response.total_used_area = total_used_area;
        response.total_wasted_area = total_wasted_area;
        response.total_cut_length = total_cut_length;
        response.total_nbr_cuts = total_cuts as u64;
        
        // Calculate used area ratio
        let total_area = total_used_area + total_wasted_area;
        response.total_used_area_ratio = if total_area > 0.0 {
            total_used_area / total_area
        } else {
            0.0
        };

        response.panels = if all_panels.is_empty() {
            None
        } else {
            Some(all_panels)
        };

        self.solution = Some(response);
    }

    /// Updates the last queried timestamp to current time
    pub fn update_last_queried(&mut self) {
        self.last_queried = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
    }

    /// Validates the task for consistency and completeness
    ///
    /// # Returns
    /// `Ok(())` if valid, `Err(TaskError)` if invalid
    pub fn validate(&self) -> Result<()> {
        // Check that running tasks have client info
        if self.is_running() && self.client_info.is_none() {
            return Err(TaskError::TaskMissingClientInfo.into());
        }

        // Check that finished tasks have solutions
        if self.status == Status::Finished && !self.has_solution() {
            return Err(TaskError::TaskInvalidState {
                current_state: "Finished task without solution".to_string(),
            }.into());
        }

        // Validate solution if present
        if let Some(solution) = &self.solution {
            solution.validate()?;
        }

        Ok(())
    }
}

impl Solution {
    /// Creates a new Solution
    ///
    /// # Arguments
    /// * `material` - The material type
    /// * `score` - Quality score of the solution
    /// * `efficiency` - Efficiency ratio of the solution
    ///
    /// # Returns
    /// A new Solution instance
    pub fn new(material: String, score: f64, efficiency: f64) -> Self {
        Self {
            material,
            score,
            efficiency,
            response: None,
        }
    }

    /// Creates a Solution with a response
    ///
    /// # Arguments
    /// * `material` - The material type
    /// * `score` - Quality score of the solution
    /// * `efficiency` - Efficiency ratio of the solution
    /// * `response` - The calculation response
    ///
    /// # Returns
    /// A new Solution instance with response
    pub fn with_response(
        material: String, 
        score: f64, 
        efficiency: f64, 
        response: CalculationResponse
    ) -> Self {
        Self {
            material,
            score,
            efficiency,
            response: Some(response),
        }
    }

    /// Checks if this solution is better than another based on score
    ///
    /// # Arguments
    /// * `other` - The other solution to compare against
    ///
    /// # Returns
    /// `true` if this solution has a higher score
    pub fn is_better_than(&self, other: &Solution) -> bool {
        self.score > other.score
    }
}

impl Default for Task {
    fn default() -> Self {
        Self::new("default-task".to_string())
    }
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Task {{ id: {}, status: {:?}, progress: {}% }}",
            self.id,
            self.status,
            self.get_percentage_done()
        )
    }
}
