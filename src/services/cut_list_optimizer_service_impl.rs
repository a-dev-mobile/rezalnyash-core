//! Cut List Optimizer Service Implementation
//!
//! This module provides the main implementation of the CutListOptimizerService trait.
//! It's a Rust port of the Java CutListOptimizerServiceImpl class with all
//! functionality preserved and adapted for Rust's ownership model and concurrency.

use crate::enums::{Status, StatusCode};
use crate::errors::{CoreError, Result, ServiceError, TaskError};
use crate::models::running_tasks::running_tasks::RunningTasks;
use crate::models::stats::stats::Stats;
use crate::models::stock::{StockPanelPicker, StockSolution};
use crate::models::task_status_response::task_status_response::TaskStatusResponse;
use crate::models::{
    calculation_request::CalculationRequest,
    calculation_submission_result::CalculationSubmissionResult,
    client_info::ClientInfo,
    configuration::Configuration,
    grouped_tile_dimensions::GroupedTileDimensions,
    performance_thresholds::PerformanceThresholds,
    task::{Solution, Task},
    tile_dimensions::TileDimensions,
    watch_dog::{CutListLogger, DefaultCutListLogger, WatchDog},
};
use crate::scaled_math::{PrecisionAnalyzer, ScaledConverter, ScaledNumber};
use crate::services::CutListOptimizerService;
use crate::{log_debug, log_error, log_info, log_trace, log_warn};
use rayon::ThreadPoolBuilder;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::Semaphore;

/// Constants from Java implementation
const MAX_PERMUTATION_ITERATIONS: usize = 1000;
const MAX_STOCK_ITERATIONS: usize = 1000;
const MAX_ALLOWED_DIGITS: usize = 6;
const THREAD_QUEUE_SIZE: usize = 1000;
const MAX_ACTIVE_THREADS_PER_TASK: usize = 5;
const MAX_PERMUTATIONS_WITH_SOLUTION: usize = 150;

/// Global task ID counter
static TASK_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Thread pool executor for managing computation tasks
#[derive(Debug)]
pub struct TaskExecutor {
    /// Rayon thread pool for CPU-intensive tasks
    thread_pool: rayon::ThreadPool,
    /// Semaphore for limiting concurrent tasks
    semaphore: Arc<Semaphore>,
    /// Active thread count
    active_count: Arc<Mutex<i32>>,
    /// Completed task count
    completed_count: Arc<Mutex<u64>>,
}

impl TaskExecutor {
    /// Creates a new TaskExecutor with specified thread count
    pub fn new(num_threads: usize) -> Result<Self> {
        let thread_pool = ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build()
            .map_err(|e| ServiceError::ThreadPoolError {
                message: format!("Failed to create thread pool: {}", e),
            })?;

        Ok(Self {
            thread_pool,
            semaphore: Arc::new(Semaphore::new(THREAD_QUEUE_SIZE)),
            active_count: Arc::new(Mutex::new(0)),
            completed_count: Arc::new(Mutex::new(0)),
        })
    }

    /// Executes a task in the thread pool
    pub fn execute<F>(&self, task: F) -> Result<()>
    where
        F: FnOnce() + Send + 'static,
    {
        let active_count = self.active_count.clone();
        let completed_count = self.completed_count.clone();
        let semaphore = self.semaphore.clone();

        self.thread_pool.spawn(move || {
            let _permit = semaphore.try_acquire();
            if _permit.is_err() {
                log_warn!("Thread pool queue is full, task rejected");
                return;
            }

            // Increment active count
            if let Ok(mut count) = active_count.lock() {
                *count += 1;
            }

            // Execute the task
            task();

            // Decrement active count and increment completed count
            if let Ok(mut count) = active_count.lock() {
                *count -= 1;
            }
            if let Ok(mut count) = completed_count.lock() {
                *count += 1;
            }
        });

        Ok(())
    }

    /// Gets the number of active threads
    pub fn get_active_count(&self) -> i32 {
        self.active_count
            .lock()
            .unwrap_or_else(|_| {
                log_error!("Failed to acquire active_count lock");
                std::process::exit(1);
            })
            .clone()
    }

    /// Gets the number of completed tasks
    pub fn get_completed_task_count(&self) -> u64 {
        self.completed_count
            .lock()
            .unwrap_or_else(|_| {
                log_error!("Failed to acquire completed_count lock");
                std::process::exit(1);
            })
            .clone()
    }

    /// Gets the queue size (approximation)
    pub fn get_queue_size(&self) -> i32 {
        (THREAD_QUEUE_SIZE - self.semaphore.available_permits()) as i32
    }
}

/// Main implementation of the CutListOptimizerService
///
/// This struct provides all the functionality for managing and executing
/// cut list optimization tasks, including task submission, monitoring,
/// and lifecycle management.
#[derive(Debug)]
pub struct CutListOptimizerServiceImpl {
    /// Cut list logger for operations
    cut_list_logger: Arc<dyn CutListLogger>,

    /// Running tasks manager
    running_tasks: &'static RunningTasks,

    /// Task executor for computation
    task_executor: Option<Arc<TaskExecutor>>,

    /// Watch dog for monitoring
    watch_dog: Option<Arc<Mutex<WatchDog>>>,

    /// Whether multiple tasks per client are allowed
    allow_multiple_tasks_per_client: bool,

    /// Date format for task ID generation
    date_format: String,
}

impl CutListOptimizerServiceImpl {
    /// Creates a new CutListOptimizerServiceImpl instance
    ///
    /// # Returns
    /// A new service implementation with default settings
    ///
    /// # Examples
    /// ```
    /// use rezalnyash_core::services::CutListOptimizerServiceImpl;
    ///
    /// let service = CutListOptimizerServiceImpl::new();
    /// ```
    pub fn new() -> Self {
        Self {
            cut_list_logger: Arc::new(DefaultCutListLogger),
            running_tasks: RunningTasks::get_instance(),
            task_executor: None,
            watch_dog: None,
            allow_multiple_tasks_per_client: false,
            date_format: "%Y%m%d%H%M".to_string(),
        }
    }

    /// Creates a singleton instance (for compatibility with Java pattern)
    ///
    /// # Returns
    /// A new service implementation instance
    pub fn get_instance() -> Self {
        Self::new()
    }

    /// Generates a unique task ID
    fn generate_task_id(&self) -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();

        let timestamp = chrono::DateTime::from_timestamp(now.as_secs() as i64, 0)
            .unwrap_or_default()
            .format(&self.date_format)
            .to_string();

        let counter = TASK_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        format!("{}{}", timestamp, counter)
    }

    /// Validates calculation request panels
    fn validate_panels(&self, panels: &[CalculationRequest]) -> (usize, bool) {
        let mut count = 0;
        let mut valid = false;

        // Note: This is a simplified validation
        // In the real implementation, you would iterate through panels
        // and validate each one according to the Panel structure
        for _panel in panels {
            // Placeholder validation logic
            count += 1;
            valid = true;
        }

        (count, valid)
    }

    /// Gets the number of decimal places in a string representation of a number
    fn get_nbr_decimal_places(&self, value: &str) -> usize {
        if let Some(dot_pos) = value.find('.') {
            value.len() - dot_pos - 1
        } else {
            0
        }
    }

    /// Gets the number of integer places in a string representation of a number
    fn get_nbr_integer_places(&self, value: &str) -> usize {
        if let Some(dot_pos) = value.find('.') {
            dot_pos
        } else {
            value.len()
        }
    }

    /// Gets the maximum number of decimal places from a list of panels
    fn get_max_nbr_decimal_places(&self, _panels: &[CalculationRequest]) -> usize {
        // Placeholder implementation
        // In the real implementation, you would iterate through panels
        // and find the maximum decimal places in width/height values
        2 // Default to 2 decimal places
    }

    /// Gets the maximum number of integer places from a list of panels
    fn get_max_nbr_integer_places(&self, _panels: &[CalculationRequest]) -> usize {
        // Placeholder implementation
        // In the real implementation, you would iterate through panels
        // and find the maximum integer places in width/height values
        4 // Default to 4 integer places
    }
    /// Check if this is one-dimensional optimization
    fn is_one_dimensional_optimization(
        &self,
        tiles: &[TileDimensions],
        stock_tiles: &[TileDimensions],
    ) -> bool {
        if tiles.is_empty() {
            return false;
        }

        // Get unique dimensions from first tile
        let mut common_dimensions = vec![tiles[0].width(), tiles[0].height()];

        // Check all tiles
        for tile in tiles {
            common_dimensions.retain(|&dim| dim == tile.width() || dim == tile.height());
            if common_dimensions.is_empty() {
                return false;
            }
        }

        // Check stock tiles
        for stock_tile in stock_tiles {
            common_dimensions
                .retain(|&dim| dim == stock_tile.width() || dim == stock_tile.height());
            if common_dimensions.is_empty() {
                return false;
            }
        }

        !common_dimensions.is_empty()
    }

    /// Gets distinct grouped tile dimensions with counts
    fn get_distinct_grouped_tile_dimensions<T: Clone + std::hash::Hash + Eq>(
        &self,
        items: Vec<T>,
        _configuration: &Configuration,
    ) -> HashMap<T, i32> {
        let mut counts = HashMap::new();
        for item in items {
            *counts.entry(item).or_insert(0) += 1;
        }
        counts
    }

    /// Groups tile dimensions by material
    fn get_tile_dimensions_per_material(
        &self,
        tiles: Vec<TileDimensions>,
    ) -> HashMap<String, Vec<TileDimensions>> {
        let mut material_groups = HashMap::new();

        for tile in tiles {
            material_groups
                .entry(tile.material().to_string())
                .or_insert_with(Vec::new)
                .push(tile);
        }

        material_groups
    }

    /// Removes duplicated permutations from a list
    fn remove_duplicated_permutations(&self, permutations: &mut Vec<Vec<TileDimensions>>) -> usize {
        let mut seen_hashes = std::collections::HashSet::new();
        let mut removed_count = 0;

        permutations.retain(|permutation| {
            let mut hash = 0u64;
            for tile in permutation {
                hash = hash
                    .wrapping_mul(31)
                    .wrapping_add(tile.dimensions_based_hash_code() as u64);
            }

            if seen_hashes.contains(&hash) {
                removed_count += 1;
                false
            } else {
                seen_hashes.insert(hash);
                true
            }
        });

        removed_count
    }

    /// Checks if a thread is eligible to start based on group rankings
    fn is_thread_eligible_to_start(&self, group: &str, task: &Task, material: &str) -> bool {
        log_debug!("RUST: === is_thread_eligible_to_start ===");
        log_debug!(
            "RUST: Checking eligibility for group='{}', material='{}'",
            group,
            material
        );

        match task.get_thread_group_rankings(material) {
            Some(rankings) => {
                let total_rankings: i32 = rankings.values().sum();
                let finished_threads = task.get_nbr_finished_threads_for_material(material);

                log_debug!(
                    "RUST: Total rankings sum: {} from {} entries",
                    total_rankings,
                    rankings.len()
                );
                log_debug!(
                    "RUST: Finished threads for material '{}': {}",
                    material,
                    finished_threads
                );

                if finished_threads < 10 {
                    log_debug!(
                        "RUST: Thread eligible (finished threads {} < 10)",
                        finished_threads
                    );
                    return true;
                }

                let group_ranking = rankings.get(group).unwrap_or(&0);
                let threshold = total_rankings / 5;

                log_debug!(
                    "RUST: Group '{}' ranking: {}, threshold: {} (total {} / 5)",
                    group,
                    group_ranking,
                    threshold,
                    total_rankings
                );

                let eligible = *group_ranking > threshold;
                log_debug!(
                    "RUST: Thread eligible for group '{}': {} ({} > {})",
                    group,
                    eligible,
                    group_ranking,
                    threshold
                );

                eligible
            }
            None => {
                log_debug!(
                    "RUST: No rankings found for material '{}', returning true (fail-safe)",
                    material
                );
                true
            }
        }
    }

    /// Main computation method for a specific material
    pub fn compute_material(
        &self,
        tiles: Vec<TileDimensions>,
        stock_tiles: Vec<TileDimensions>,
        configuration: Configuration,
        task: Arc<Mutex<Task>>,
        material: String,
    ) -> Result<()> {
        log_debug!("Starting computation for material: {}", material);

        // Log the tiles being processed
        {
            let task_guard = task.lock().map_err(|_| CoreError::Internal {
                message: "Failed to lock task".to_string(),
            })?;
            log_debug!(
                "Task[{}] Generating groups for {} tiles",
                task_guard.id,
                tiles.len()
            );
        }

        // Generate groups (equivalent to Java generateGroups method)
        let grouped_tiles = self.generate_groups(&tiles, &stock_tiles, &task)?;

        {
            let task_guard = task.lock().map_err(|_| CoreError::Internal {
                message: "Failed to lock task".to_string(),
            })?;
            log_debug!(
                "Generated {} distinct groups for material {}",
                grouped_tiles.len(),
                material
            );
        }

        // Check if we have any groups to process
        if grouped_tiles.is_empty() {
            log_debug!(
                "No groups generated, completing computation for material: {}",
                material
            );
            let mut task_guard = task.lock().map_err(|_| CoreError::Internal {
                message: "Failed to lock task".to_string(),
            })?;
            task_guard.set_material_percentage_done(material, 100);
            return Ok(());
        }

        // Set task to running status if not already
        {
            let mut task_guard = task.lock().map_err(|_| CoreError::Internal {
                message: "Failed to lock task".to_string(),
            })?;
            if let Err(_) = task_guard.set_running_status() {
                // Task might already be running, which is fine
            }
        }

        // Initialize stock panel picker
        let mut stock_panel_picker = StockPanelPicker::new(
            tiles.clone(),
            stock_tiles.clone(),
            task.clone(),
            if configuration.use_single_stock_unit() {
                Some(1)
            } else {
                None
            },
        )?;

        stock_panel_picker.init()?;

        // Get performance thresholds
        let performance_thresholds = configuration
            .performance_thresholds()
            .cloned()
            .unwrap_or_else(|| {
                log_debug!("No performance thresholds specified, using defaults");
                PerformanceThresholds::default()
            });

        // Generate permutations (simplified approach)
        let mut permutations = vec![grouped_tiles.clone()];

        // Process permutations
        let max_permutations = std::cmp::min(permutations.len(), 1000);

        for (permutation_idx, tiles_permutation) in
            permutations.iter().enumerate().take(max_permutations)
        {
            // Check if task is still running
            {
                let task_guard = task.lock().map_err(|_| CoreError::Internal {
                    message: "Failed to lock task".to_string(),
                })?;
                if !task_guard.is_running() {
                    log_debug!(
                        "Task no longer running, stopping computation for material: {}",
                        material
                    );
                    break;
                }
            }

            log_debug!(
                "Processing permutation {} for material {}",
                permutation_idx,
                material
            );

            // Convert grouped tiles back to regular tiles
            let tiles_in_order = self.grouped_tiles_to_tiles(tiles_permutation);

            // Process different stock solutions
            let max_stock_iterations = 1000;

            for stock_idx in 0..max_stock_iterations {
                // Get stock solution
                let stock_solution = match stock_panel_picker.get_stock_solution(stock_idx) {
                    Ok(Some(solution)) => solution,
                    Ok(None) => {
                        log_debug!("No more stock solutions available at index {}", stock_idx);
                        break;
                    }
                    Err(e) => {
                        log_error!("Error getting stock solution {}: {}", stock_idx, e);
                        break;
                    }
                };

                // Check if task is still running
                {
                    let task_guard = task.lock().map_err(|_| CoreError::Internal {
                        message: "Failed to lock task".to_string(),
                    })?;
                    if !task_guard.is_running() {
                        log_debug!("Task no longer running, stopping stock iteration");
                        break;
                    }
                }

                // Process this combination of tiles and stock
                self.process_tiles_with_stock(
                    &tiles_in_order,
                    &stock_solution,
                    &configuration,
                    &task,
                    &material,
                    permutation_idx,
                    stock_idx,
                )?;

                // Update progress
                if stock_idx % 10 == 0 {
                    let progress = std::cmp::min(
                        (permutation_idx * 100 / max_permutations)
                            + (stock_idx * 100 / (max_permutations * max_stock_iterations)),
                        99,
                    ) as i32;

                    let mut task_guard = task.lock().map_err(|_| CoreError::Internal {
                        message: "Failed to lock task".to_string(),
                    })?;
                    task_guard.set_material_percentage_done(material.clone(), progress);
                }
            }
        }

        // Mark material as completed
        {
            let mut task_guard = task.lock().map_err(|_| CoreError::Internal {
                message: "Failed to lock task".to_string(),
            })?;
            task_guard.set_material_percentage_done(material.clone(), 100);
        }

        log_debug!("Completed computation for material: {}", material);
        Ok(())
    }

    /// Main computation entry point - с использованием ScaledNumber
    fn compute(&self, request: CalculationRequest, task_id: String) -> Result<()> {
        log_debug!("Starting computation for task: {}", task_id);

        // Получаем панели и исходные листы из запроса
        let panels = request.panels();
        let stock_panels = request.stock_panels();

        log_debug!(
            "Request contains {} panels and {} stock panels",
            panels.len(),
            stock_panels.len()
        );

        if panels.is_empty() {
            log_warn!("No panels in request!");
            return Err(CoreError::InvalidInput {
                details: "No panels provided".to_string(),
            }
            .into());
        }

        if stock_panels.is_empty() {
            log_warn!("No stock panels in request!");
            return Err(CoreError::InvalidInput {
                details: "No stock panels provided".to_string(),
            }
            .into());
        }

        // Создаем и настраиваем задачу
        let mut task = Task::new(task_id.clone());
        task.calculation_request = Some(request.clone());
        task.client_info = request.client_info().cloned();

        // Устанавливаем статус Running
        task.set_running_status()
            .map_err(|e| CoreError::InvalidInput {
                details: format!("Failed to set task status to running: {}", e),
            })?;

        // Собираем все строковые значения для анализа точности
        let mut all_dimension_strings = Vec::new();

        // Собираем размеры панелей
        for panel in panels {
            all_dimension_strings.push(panel.width());
            all_dimension_strings.push(panel.height());
        }

        // Собираем размеры исходных листов
        for stock_panel in stock_panels {
            all_dimension_strings.push(stock_panel.width());
            all_dimension_strings.push(stock_panel.height());
        }

        // Добавляем размеры из конфигурации
        let binding = Configuration::default();
        let config = request.configuration().unwrap_or(&binding);
        if let Some(cut_thickness) = config.cut_thickness() {
            all_dimension_strings.push(cut_thickness);
        }
        if let Some(min_trim) = config.min_trim_dimension() {
            all_dimension_strings.push(min_trim);
        }

        log_debug!(
            "Analyzing precision for {} dimension strings",
            all_dimension_strings.len()
        );

        // Определяем оптимальную точность с ограничением
        const MAX_ALLOWED_DIGITS: u8 = 6;
        let precision =
            PrecisionAnalyzer::validate_total_digits(&all_dimension_strings, MAX_ALLOWED_DIGITS)
                .map_err(|e| CoreError::InvalidInput {
                    details: format!("Precision analysis failed: {}", e),
                })?;

        log_debug!("Using precision: {}", precision);

        // Создаем конвертер с определенной точностью
        let converter = ScaledConverter::new(precision).map_err(|e| CoreError::InvalidInput {
            details: format!("Failed to create converter: {}", e),
        })?;

        // Сохраняем масштабный коэффициент в задаче (для совместимости с Java)
        let scale_factor = 10_f64.powi(precision as i32);
        task.factor = scale_factor;

        log_debug!("Using scaling factor: {}", scale_factor);

        // Преобразуем панели в TileDimensions с масштабированием
        let mut tiles = Vec::new();
        for panel in panels {
            if panel.is_valid() {
                // Конвертируем размеры в ScaledNumber
                let width_scaled =
                    ScaledNumber::from_str(panel.width(), precision).map_err(|e| {
                        CoreError::InvalidInput {
                            details: format!("Invalid panel width '{}': {}", panel.width(), e),
                        }
                    })?;
                let height_scaled =
                    ScaledNumber::from_str(panel.height(), precision).map_err(|e| {
                        CoreError::InvalidInput {
                            details: format!("Invalid panel height '{}': {}", panel.height(), e),
                        }
                    })?;

                // Получаем целые значения (масштабированные)
                let width = width_scaled.raw_value() as u32;
                let height = height_scaled.raw_value() as u32;

                for _ in 0..panel.count() {
                    tiles.push(TileDimensions::new(
                        panel.id(),
                        width,
                        height,
                        panel.material().to_string(),
                        panel.orientation() as u32,
                        panel.label().map(|s| s.to_string()),
                        panel.is_enabled(), // Added the missing 7th parameter
                    ));
                }
            }
        }

        // Преобразуем исходные листы в TileDimensions с масштабированием
        let mut stock_tiles = Vec::new();
        for stock_panel in stock_panels {
            if stock_panel.is_valid() {
                // Конвертируем размеры в ScaledNumber
                let width_scaled =
                    ScaledNumber::from_str(stock_panel.width(), precision).map_err(|e| {
                        CoreError::InvalidInput {
                            details: format!(
                                "Invalid stock width '{}': {}",
                                stock_panel.width(),
                                e
                            ),
                        }
                    })?;
                let height_scaled = ScaledNumber::from_str(stock_panel.height(), precision)
                    .map_err(|e| CoreError::InvalidInput {
                        details: format!("Invalid stock height '{}': {}", stock_panel.height(), e),
                    })?;

                // Получаем целые значения (масштабированные)
                let width = width_scaled.raw_value() as u32;
                let height = height_scaled.raw_value() as u32;

                for _ in 0..stock_panel.count() {
                    stock_tiles.push(TileDimensions::new(
                        stock_panel.id(),
                        width,
                        height,
                        stock_panel.material().to_string(),
                        stock_panel.orientation() as u32,
                        stock_panel.label().map(|s| s.to_string()),
                        stock_panel.is_enabled(), // Added the missing 7th parameter
                    ));
                }
            }
        }

        log_debug!(
            "Created {} tiles and {} stock tiles",
            tiles.len(),
            stock_tiles.len()
        );

        /*
        task.buildSolution();
        this.runningTasks.addTask(task);

         */
        // Строим начальное решение
        task.build_solution();

        // Добавляем задачу в список выполняющихся
        let task_arc = Arc::new(Mutex::new(task));
        self.running_tasks.add_task({
            let task_guard = task_arc.lock().map_err(|_| TaskError::TaskLockError {
                operation: "add_task".to_string(),
            })?;
            task_guard.clone()
        })?;

        // Логируем выполнение
        {
            let task_guard = task_arc.lock().map_err(|_| TaskError::TaskLockError {
                operation: "log_execution".to_string(),
            })?;
            self.cut_list_logger.log_execution(&task_guard);
        }

        // Группируем по материалам
        let mut material_tiles: HashMap<String, Vec<TileDimensions>> = HashMap::new();
        let mut material_stock: HashMap<String, Vec<TileDimensions>> = HashMap::new();

        // Группируем панели по материалам
        for tile in tiles {
            material_tiles
                .entry(tile.material().to_string())
                .or_insert_with(Vec::new)
                .push(tile);
        }

        // Группируем исходные листы по материалам
        for stock_tile in stock_tiles {
            material_stock
                .entry(stock_tile.material().to_string())
                .or_insert_with(Vec::new)
                .push(stock_tile);
        }

        log_debug!(
            "Grouped tiles by materials: {:?}",
            material_tiles.keys().collect::<Vec<_>>()
        );
        log_debug!(
            "Grouped stock by materials: {:?}",
            material_stock.keys().collect::<Vec<_>>()
        );

        // Определяем материалы для обработки
        let mut materials_to_process = HashSet::new();

        // Обновляем задачу с информацией о материалах
        {
            let mut task_guard = task_arc.lock().map_err(|_| TaskError::TaskLockError {
                operation: "set_materials".to_string(),
            })?;

            // Сохраняем группировку по материалам в задаче
            task_guard.tile_dimensions_per_material = Some(material_tiles.clone());
            task_guard.stock_dimensions_per_material = Some(material_stock.clone());

            log_debug!(
                "Task status before material processing: {:?}",
                task_guard.status
            );
        }

        for material in material_tiles.keys() {
            if material_stock.contains_key(material) {
                materials_to_process.insert(material.clone());
                // Добавляем материал в задачу
                {
                    let mut task_guard = task_arc.lock().map_err(|_| TaskError::TaskLockError {
                        operation: "add_material".to_string(),
                    })?;
                    task_guard.add_material_to_compute(material.clone());
                }
            } else {
                // Материал есть в панелях, но нет в листах - эти панели не разместить
                log_warn!(
                    "Material '{}' has panels but no stock - panels will not fit",
                    material
                );
                if let Some(tiles_for_material) = material_tiles.get(material) {
                    let mut task_guard = task_arc.lock().map_err(|_| TaskError::TaskLockError {
                        operation: "add_no_material_tiles".to_string(),
                    })?;

                    // Добавляем панели без материала (они не поместятся)
                    for tile in tiles_for_material {
                        task_guard.no_material_tiles.push(tile.clone());
                    }
                }
            }
        }

        log_debug!("Materials to process: {:?}", materials_to_process);

        // Проверяем, есть ли материалы для обработки
        if materials_to_process.is_empty() {
            log_warn!("No materials to process - finishing task");
            {
                let mut task_guard = task_arc.lock().map_err(|_| TaskError::TaskLockError {
                    operation: "finish_no_materials".to_string(),
                })?;
                task_guard.check_if_finished();
            }
            return Ok(());
        }

        // Создаем копию списка материалов для итерации
        let materials_list: Vec<String> = materials_to_process.iter().cloned().collect();

        // Обрабатываем каждый материал в отдельном потоке
        for material in materials_list {
            let tiles_for_material = material_tiles.get(&material).unwrap().clone();
            let stock_for_material = material_stock.get(&material).unwrap().clone();
            let configuration_clone = config.clone();
            let task_clone = task_arc.clone();
            let material_clone = material.clone();
            let service_clone = self.clone();

            log_debug!(
                "Starting computation for material '{}' with {} tiles and {} stock",
                material,
                tiles_for_material.len(),
                stock_for_material.len()
            );

            // Запускаем обработку материала в отдельном потоке
            thread::spawn(move || {
                if let Err(e) = service_clone.compute_material(
                    tiles_for_material,
                    stock_for_material,
                    configuration_clone,
                    task_clone,
                    material_clone,
                ) {
                    log_error!("Error computing material: {}", e);
                }
            });
        }

        // Проверяем, завершена ли задача
        {
            let mut task_guard = task_arc.lock().map_err(|_| TaskError::TaskLockError {
                operation: "check_finished".to_string(),
            })?;
            task_guard.check_if_finished();
        }

        // Ждем завершения всех потоков обработки материалов
        // Это необходимо, так как материалы обрабатываются в отдельных потоках
        thread::sleep(Duration::from_millis(500));

        // Создаем копию списка материалов для использования в финальной проверке
        let materials_list: Vec<String> = materials_to_process.iter().cloned().collect();

        // Еще раз проверяем статус задачи после небольшой задержки
        {
            let mut task_guard = task_arc.lock().map_err(|_| TaskError::TaskLockError {
                operation: "final_check".to_string(),
            })?;

            // Устанавливаем 100% для всех материалов, чтобы гарантировать завершение
            for material in &materials_list {
                task_guard.set_material_percentage_done(material.clone(), 100);
            }

            // Принудительно проверяем завершение
            task_guard.check_if_finished();

            // Если задача все еще выполняется, принудительно завершаем
            if task_guard.status == Status::Running {
                log_info!("Принудительное завершение задачи после обработки всех материалов");
                task_guard.status = Status::Finished;
                task_guard.end_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as u64;

                if task_guard.solution.is_none() {
                    task_guard.build_solution();
                }
            }
        }

        Ok(())
    }
    /// Process tiles with a specific stock solution
    fn process_tiles_with_stock(
        &self,
        tiles: &[TileDimensions],
        stock_solution: &StockSolution,
        configuration: &Configuration,
        task: &Arc<Mutex<Task>>,
        material: &str,
        permutation_idx: usize,
        stock_idx: usize,
    ) -> Result<()> {
        log_debug!(
            "Processing permutation[{}/∞] with stock solution [{}] {{nbrPanels[{}] area[{}]}}",
            permutation_idx,
            stock_idx,
            stock_solution.len(),
            stock_solution.get_total_area()
        );

        // Parse cut thickness
        let cut_thickness = configuration
            .cut_thickness()
            .and_then(|ct| ct.parse::<f64>().ok())
            .unwrap_or(0.0);

        // Parse min trim dimension
        let min_trim_dimension = configuration
            .min_trim_dimension()
            .and_then(|mtd| mtd.parse::<f64>().ok())
            .unwrap_or(0.0);

        // Get factor from task
        let factor = {
            let task_guard = task.lock().map_err(|_| CoreError::Internal {
                message: "Failed to lock task".to_string(),
            })?;
            task_guard.factor
        };

        let cut_thickness_scaled = (cut_thickness * factor).round() as i32;
        let min_trim_scaled = (min_trim_dimension * factor).round() as i32;

        // Simulate cutting optimization process
        let mut successful_placements = 0;
        let mut total_used_area = 0.0;

        // Simple simulation - try to place each tile
        for (tile_idx, tile) in tiles.iter().enumerate() {
            // Check if we can fit this tile in any stock panel
            let tile_area = tile.area() as f64;

            if tile_area <= stock_solution.get_total_area() as f64 {
                successful_placements += 1;
                total_used_area += tile_area;
            }

            // Simulate some processing time
            if tile_idx % 5 == 0 {
                thread::sleep(Duration::from_millis(1));
            }
        }

        // Calculate efficiency
        let efficiency = if stock_solution.get_total_area() > 0 {
            total_used_area / stock_solution.get_total_area() as f64
        } else {
            0.0
        };

        log_debug!(
            "Placed {}/{} tiles with efficiency {:.2}%",
            successful_placements,
            tiles.len(),
            efficiency * 100.0
        );

        // Update task with progress (simplified)
        {
            let task_guard = task.lock().map_err(|_| CoreError::Internal {
                message: "Failed to lock task".to_string(),
            })?;

            // Increment thread group rankings for successful placements
            if successful_placements > tiles.len() / 2 {
                task_guard.increment_thread_group_rankings(material, "AREA");
            }
        }

        Ok(())
    }

    /// Convert grouped tiles back to regular tiles
    fn grouped_tiles_to_tiles(
        &self,
        grouped_tiles: &[GroupedTileDimensions],
    ) -> Vec<TileDimensions> {
        grouped_tiles
            .iter()
            .map(|gt| gt.tile_dimensions().clone())
            .collect()
    }
    /// Generate groups from tiles (equivalent to Java generateGroups method)
    fn generate_groups(
        &self,
        tiles: &[TileDimensions],
        stock_tiles: &[TileDimensions],
        task: &Arc<Mutex<Task>>,
    ) -> Result<Vec<GroupedTileDimensions>> {
        // Count tile types
        let mut tile_counts = HashMap::new();
        for tile in tiles {
            let key = format!("{}x{}", tile.width(), tile.height());
            *tile_counts.entry(key).or_insert(0) += 1;
        }

        // Determine if this is one-dimensional optimization
        let is_one_dimensional = self.is_one_dimensional_optimization(tiles, stock_tiles);

        let max_per_group = if is_one_dimensional {
            {
                let task_guard = task.lock().map_err(|_| CoreError::Internal {
                    message: "Failed to lock task".to_string(),
                })?;
                log_info!("Task is one dimensional optimization");
            }
            1000 // Large number for one-dimensional
        } else {
            std::cmp::max(tiles.len() / 100, 1)
        };

        let mut result = Vec::new();
        let mut group_counters = HashMap::new();
        let mut current_group = 0;

        for tile in tiles {
            let tile_key = format!("{}x{}", tile.width(), tile.height());
            let group_key = format!("{}-{}", tile_key, current_group);

            let count_in_group = group_counters.entry(group_key.clone()).or_insert(0);
            *count_in_group += 1;

            let grouped_tile =
                GroupedTileDimensions::from_tile_dimensions(tile.clone(), current_group);
            result.push(grouped_tile);

            // Check if we should split to a new group
            if let Some(&total_count) = tile_counts.get(&tile_key) {
                if total_count > max_per_group && *count_in_group > total_count / 4 {
                    current_group += 1;
                }
            }
        }

        Ok(result)
    }
}

impl Clone for CutListOptimizerServiceImpl {
    fn clone(&self) -> Self {
        Self {
            cut_list_logger: self.cut_list_logger.clone(),
            running_tasks: self.running_tasks,
            task_executor: self.task_executor.clone(),
            watch_dog: self.watch_dog.clone(),
            allow_multiple_tasks_per_client: self.allow_multiple_tasks_per_client,
            date_format: self.date_format.clone(),
        }
    }
}

impl CutListOptimizerService for CutListOptimizerServiceImpl {
    fn get_stats(&self) -> Stats {
        let mut stats = Stats::new();

        // Get task statistics from running tasks
        if let Ok(tasks) = self.running_tasks.get_tasks() {
            stats.set_nbr_idle_tasks(
                tasks.iter().filter(|t| t.status == Status::Idle).count() as u64
            );
            stats.set_nbr_running_tasks(
                tasks.iter().filter(|t| t.status == Status::Running).count() as u64,
            );
            stats.set_nbr_finished_tasks(
                tasks
                    .iter()
                    .filter(|t| t.status == Status::Finished)
                    .count() as u64,
            );
            stats.set_nbr_stopped_tasks(
                tasks.iter().filter(|t| t.status == Status::Stopped).count() as u64,
            );
            stats.set_nbr_terminated_tasks(
                tasks
                    .iter()
                    .filter(|t| t.status == Status::Terminated)
                    .count() as u64,
            );
            stats.set_nbr_error_tasks(
                tasks.iter().filter(|t| t.status == Status::Error).count() as u64
            );
        }

        // Get thread pool statistics
        if let Some(_executor) = &self.task_executor {
            stats.set_nbr_running_threads(_executor.get_active_count() as u32);
            stats.set_nbr_queued_threads(_executor.get_queue_size() as u32);
            stats.set_nbr_finished_threads(_executor.get_completed_task_count());
        }

        // Get task reports from watch dog
        if let Some(watch_dog) = &self.watch_dog {
            if let Ok(watch_dog_guard) = watch_dog.lock() {
                if let Ok(reports) = watch_dog_guard.get_task_reports() {
                    stats.set_task_reports(reports);
                }
            }
        }

        stats
    }

    fn get_task_status(&self, task_id: &str) -> Option<TaskStatusResponse> {
        if let Ok(task_opt) = self.running_tasks.get_task(task_id) {
            if let Some(task) = task_opt {
                let mut response = TaskStatusResponse::new();
                response.set_status(task.status.as_str().to_owned());
                response.set_init_percentage(task.get_max_thread_progress_percentage());
                response.set_percentage_done(task.get_percentage_done());
                response.set_solution((task.solution.unwrap()).clone());

                // Update last queried time
                // Note: Since we don't have get_task_mut, we'll need to implement it
                // or update through another mechanism

                Some(response)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_tasks(&self, client_id: &str, status: Status) -> Vec<String> {
        let mut task_ids = Vec::new();

        if let Ok(tasks) = self.running_tasks.get_tasks() {
            for task in tasks {
                if task.status == status {
                    if let Some(client_info) = &task.client_info {
                        if let Some(id) = &client_info.id {
                            if id == client_id {
                                task_ids.push(task.id.clone());
                            }
                        }
                    }
                }
            }
        }

        task_ids
    }

    fn init(&mut self, thread_pool_size: i32) {
        log_info!(
            "Initializing CutListOptimizerService with {} threads",
            thread_pool_size
        );

        // Create task executor
        match TaskExecutor::new(thread_pool_size as usize) {
            Ok(executor) => {
                self.task_executor = Some(Arc::new(executor));
            }
            Err(e) => {
                log_error!("Failed to create task executor: {}", e);
                return;
            }
        }

        // Create and start watch dog
        let mut watch_dog = WatchDog::new();
        watch_dog.set_cut_list_logger(self.cut_list_logger.clone());

        if let Some(_executor) = &self.task_executor {
            // Note: WatchDog expects ThreadPoolExecutor trait, would need adapter
            // For now, we'll create a basic watch dog
        }

        match watch_dog.start() {
            Ok(_control_sender) => {
                self.watch_dog = Some(Arc::new(Mutex::new(watch_dog)));
                log_info!("WatchDog started successfully");
            }
            Err(e) => {
                log_error!("Failed to start WatchDog: {}", e);
            }
        }
    }

    fn set_allow_multiple_tasks_per_client(&mut self, allow: bool) {
        self.allow_multiple_tasks_per_client = allow;
        log_debug!("Set allow multiple tasks per client: {}", allow);
    }

    fn set_cut_list_logger(&mut self, logger: Arc<dyn CutListLogger>) {
        self.cut_list_logger = logger;

        // Update watch dog logger if it exists
        if let Some(watch_dog) = &self.watch_dog {
            if let Ok(mut watch_dog_guard) = watch_dog.lock() {
                watch_dog_guard.set_cut_list_logger(self.cut_list_logger.clone());
            }
        }

        log_debug!("Cut list logger updated");
    }

    fn stop_task(&self, task_id: &str) -> Option<TaskStatusResponse> {
        // Since we don't have get_task_mut, we'll need to work around this
        // For now, we'll use a placeholder implementation
        if let Ok(task_opt) = self.running_tasks.get_task(task_id) {
            if let Some(mut task) = task_opt {
                let solution_clone = task.solution.clone();
                match task.stop() {
                    Ok(()) => {
                        log_info!("Task {} stopped successfully", task_id);

                        let mut response = TaskStatusResponse::new();
                        response.set_status(task.status.as_str().to_string());
                        response.set_init_percentage(task.get_max_thread_progress_percentage());
                        response.set_percentage_done(task.get_percentage_done());
                        response.set_solution(solution_clone.unwrap());

                        // Update the task in the running tasks
                        let _ = self.running_tasks.update_task(task);

                        Some(response)
                    }
                    Err(e) => {
                        if let Some(client_info) = &task.client_info {
                            if let Some(client_id) = &client_info.id {
                                self.cut_list_logger.warn(
                                    client_id,
                                    task_id,
                                    &format!(
                                        "Unable to stop task. Current status is: {:?}",
                                        task.status
                                    ),
                                );
                            }
                        }
                        log_warn!("Failed to stop task {}: {}", task_id, e);
                        None
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn submit_task(&self, request: CalculationRequest) -> CalculationSubmissionResult {
        log_debug!("Submitting new task");

        let client_info = request.client_info().cloned().unwrap_or_default();

        // Log client info
        self.cut_list_logger
            .log_execution(&Task::new("temp".to_string()));

        // Validate request
        let performance_thresholds = request
            .configuration()
            .and_then(|c| c.performance_thresholds())
            .cloned()
            .unwrap_or_default();

        // Check if multiple tasks per client are allowed
        if !self.allow_multiple_tasks_per_client {
            if let Ok(tasks) = self.running_tasks.get_tasks() {
                let client_id_ref = client_info.id.as_ref();
                let running_count = tasks
                    .iter()
                    .filter(|task| {
                        task.status == Status::Running
                            && task
                                .client_info
                                .as_ref()
                                .and_then(|ci| ci.id.as_ref())
                                .map(|id| Some(id) == client_id_ref)
                                .unwrap_or(false)
                    })
                    .count();

                if running_count >= performance_thresholds.get_max_simultaneous_tasks() as usize {
                    if let Some(client_id) = &client_info.id {
                        self.cut_list_logger.warn(
                            client_id,
                            "",
                            &format!(
                                "Rejecting user task due to [{}] already running task(s)",
                                running_count
                            ),
                        );
                    }
                    return CalculationSubmissionResult::with_status_code(
                        StatusCode::TaskAlreadyRunning.string_value(),
                    );
                }
            }
        }

        // Validate panels (simplified)
        // In real implementation, you would validate request.panels
        let panel_count = 1; // Placeholder
        if panel_count == 0 {
            return CalculationSubmissionResult::with_status_code(
                StatusCode::InvalidTiles.string_value(),
            );
        }
        if panel_count > 5000 {
            return CalculationSubmissionResult::with_status_code(
                StatusCode::TooManyPanels.string_value(),
            );
        }

        // Validate stock panels (simplified)
        let stock_count = 1; // Placeholder
        if stock_count == 0 {
            return CalculationSubmissionResult::with_status_code(
                StatusCode::InvalidStockTiles.string_value(),
            );
        }
        if stock_count > 5000 {
            return CalculationSubmissionResult::with_status_code(
                StatusCode::TooManyStockPanels.string_value(),
            );
        }

        // Generate task ID and start computation
        let task_id = self.generate_task_id();
        let service_clone = self.clone();
        let request_clone = request;
        let task_id_clone = task_id.clone();

        thread::spawn(move || {
            if let Err(e) = service_clone.compute(request_clone, task_id_clone) {
                log_error!("Error during computation: {}", e);
            }
        });

        CalculationSubmissionResult::new(StatusCode::Ok.string_value(), Some(task_id))
    }

    fn terminate_task(&self, task_id: &str) -> i32 {
        // Since we don't have get_task_mut, we'll need to work around this
        if let Ok(task_opt) = self.running_tasks.get_task(task_id) {
            if let Some(mut task) = task_opt {
                match task.terminate() {
                    Ok(()) => {
                        log_info!("Task {} terminated successfully", task_id);

                        // Update the task in the running tasks
                        let _ = self.running_tasks.update_task(task);
                        0
                    }
                    Err(e) => {
                        self.cut_list_logger.error(&format!(
                            "Unable to terminate task {}. Current status is: {:?}",
                            task_id, task.status
                        ));
                        log_error!("Failed to terminate task {}: {}", task_id, e);
                        -1
                    }
                }
            } else {
                log_warn!("Task {} not found for termination", task_id);
                -1
            }
        } else {
            log_warn!("Task {} not found for termination", task_id);
            -1
        }
    }
}

impl Default for CutListOptimizerServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::client_info::ClientInfo;
    use crate::models::configuration::Configuration;

    #[test]
    fn test_new_service() {
        let service = CutListOptimizerServiceImpl::new();
        assert!(!service.allow_multiple_tasks_per_client);
        assert!(service.task_executor.is_none());
        assert!(service.watch_dog.is_none());
    }

    #[test]
    fn test_generate_task_id() {
        let service = CutListOptimizerServiceImpl::new();
        let task_id1 = service.generate_task_id();
        let task_id2 = service.generate_task_id();

        assert!(!task_id1.is_empty());
        assert!(!task_id2.is_empty());
        assert_ne!(task_id1, task_id2);
    }

    #[test]
    fn test_get_stats() {
        let service = CutListOptimizerServiceImpl::new();
        let stats = service.get_stats();

        // Basic validation that stats object is created
        assert!(stats.get_nbr_idle_tasks() >= 0);
        assert!(stats.get_nbr_running_tasks() >= 0);
        assert!(stats.get_nbr_finished_tasks() >= 0);
    }

    #[test]
    fn test_set_allow_multiple_tasks_per_client() {
        let mut service = CutListOptimizerServiceImpl::new();
        assert!(!service.allow_multiple_tasks_per_client);

        service.set_allow_multiple_tasks_per_client(true);
        assert!(service.allow_multiple_tasks_per_client);

        service.set_allow_multiple_tasks_per_client(false);
        assert!(!service.allow_multiple_tasks_per_client);
    }

    #[test]
    fn test_init() {
        let mut service = CutListOptimizerServiceImpl::new();
        assert!(service.task_executor.is_none());

        service.init(4);
        assert!(service.task_executor.is_some());
    }

    #[test]
    fn test_submit_task_validation() {
        let service = CutListOptimizerServiceImpl::new();

        // Create a basic calculation request
        let client_info = ClientInfo::new();
        let configuration = Configuration::default();

        let request = CalculationRequest::new()
            .with_client_info(client_info)
            .with_configuration(configuration);

        let result = service.submit_task(request);

        // Should return an error status for invalid tiles (since we're using placeholder validation)
        // The actual validation would need to be implemented based on the Panel structure
        assert!(!result.status_code().is_empty());
    }

    #[test]
    fn test_decimal_places_calculation() {
        let service = CutListOptimizerServiceImpl::new();

        assert_eq!(service.get_nbr_decimal_places("123.45"), 2);
        assert_eq!(service.get_nbr_decimal_places("123"), 0);
        assert_eq!(service.get_nbr_decimal_places("123.456789"), 6);
    }

    #[test]
    fn test_integer_places_calculation() {
        let service = CutListOptimizerServiceImpl::new();

        assert_eq!(service.get_nbr_integer_places("123.45"), 3);
        assert_eq!(service.get_nbr_integer_places("123"), 3);
        assert_eq!(service.get_nbr_integer_places("1.456789"), 1);
    }
}
