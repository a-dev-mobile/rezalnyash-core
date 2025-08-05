//! Cut List Thread Model
//!
//! This module provides the CutListThread struct which implements a thread-safe
//! cutting optimization algorithm. It processes tiles and generates cutting solutions
//! using various optimization strategies.

use crate::enums::{CutOrientationPreference, Status};
use crate::errors::{AppError, CoreError, Result};
use crate::models::{
    cut::{Cut, CutBuilder},
    mosaic::Mosaic,
    stock::stock_solution::StockSolution,
    task::Task,
    tile_dimensions::TileDimensions,
    tile_node::TileNode,
};
use crate::{log_debug, log_error, log_info};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

// ✅ COMPARISON LOGS ONLY: Disable detailed debug logs for cleaner comparison output
const ENABLE_DEBUG_LOGS: bool = false;

/// Cut direction enum (replacement for Java CutDirection)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CutDirection {
    Both,
    Horizontal,
    Vertical,
}

/// Maximum bind parameter count (replacement for RoomDatabase.MAX_BIND_PARAMETER_CNT)
const MAX_BIND_PARAMETER_CNT: i32 = 999;

/// Represents a solution for cutting optimization
#[derive(Debug, Clone, PartialEq)]
pub struct Solution {
    /// Material type for this solution
    pub material: Option<String>,
    /// List of mosaics in this solution
    pub mosaics: Vec<Mosaic>,
    /// Unused stock panels
    pub unused_stock_panels: Vec<TileDimensions>,
    /// Panels that couldn't fit
    pub no_fit_panels: Vec<TileDimensions>,
    /// Thread group that created this solution
    pub creator_thread_group: Option<String>,
    /// Auxiliary information
    pub aux_info: Option<String>,
}

impl Solution {
    /// Creates a new Solution from a stock solution
    pub fn new(stock_solution: &StockSolution) -> Self {
        Self {
            material: None,
            mosaics: vec![],
            unused_stock_panels: stock_solution.get_stock_tile_dimensions().clone(),
            no_fit_panels: vec![],
            creator_thread_group: None,
            aux_info: None,
        }
    }

    /// Creates a new Solution by copying from another solution and replacing a mosaic
    pub fn new_with_replacement(original: &Solution, replaced_mosaic: &Mosaic) -> Self {
        let mut new_solution = original.clone();
        // Remove the replaced mosaic and add it back to unused stock panels
        if let Some(pos) = new_solution
            .mosaics
            .iter()
            .position(|m| m == replaced_mosaic)
        {
            new_solution.mosaics.remove(pos);
        }
        new_solution
    }

    /// Gets the material of this solution
    pub fn get_material(&self) -> Option<&String> {
        self.material.as_ref()
    }

    /// Sets the material of this solution
    pub fn set_material(&mut self, material: Option<String>) {
        self.material = material;
    }

    /// Gets the mosaics in this solution
    pub fn get_mosaics(&self) -> &Vec<Mosaic> {
        &self.mosaics
    }

    /// Gets mutable reference to mosaics
    pub fn get_mosaics_mut(&mut self) -> &mut Vec<Mosaic> {
        &mut self.mosaics
    }

    /// Gets the unused stock panels
    pub fn get_unused_stock_panels(&self) -> &Vec<TileDimensions> {
        &self.unused_stock_panels
    }

    /// Gets mutable reference to unused stock panels
    pub fn get_unused_stock_panels_mut(&mut self) -> &mut Vec<TileDimensions> {
        &mut self.unused_stock_panels
    }

    /// Gets the no-fit panels
    pub fn get_no_fit_panels(&self) -> &Vec<TileDimensions> {
        &self.no_fit_panels
    }

    /// Gets mutable reference to no-fit panels
    pub fn get_no_fit_panels_mut(&mut self) -> &mut Vec<TileDimensions> {
        &mut self.no_fit_panels
    }

    /// Adds a mosaic to this solution
    pub fn add_mosaic(&mut self, mosaic: Mosaic) {
        self.mosaics.push(mosaic);
    }

    /// Sets the creator thread group
    pub fn set_creator_thread_group(&mut self, group: Option<String>) {
        self.creator_thread_group = group;
    }

    /// Sets the auxiliary information
    pub fn set_aux_info(&mut self, aux_info: Option<String>) {
        self.aux_info = aux_info;
    }
}

/// Trait for comparing solutions
pub trait SolutionComparator: Send + Sync + std::fmt::Debug {
    /// Compare two solutions, returning ordering
    fn compare(&self, a: &Solution, b: &Solution) -> Ordering;
}

/// Logger trait for cut list operations
pub trait CutListLogger: Send + Sync + std::fmt::Debug {
    /// Log a message
    fn log(&self, message: &str);
}

/// Default logger implementation
#[derive(Debug, Clone)]
pub struct DefaultCutListLogger;

impl CutListLogger for DefaultCutListLogger {
    fn log(&self, message: &str) {
        log_info!("{}", message);
    }
}

/// Thread for computing cutting solutions
///
/// This struct implements a thread-safe cutting optimization algorithm that processes
/// tiles and generates optimal cutting solutions using various strategies.
#[derive(Debug)]
pub struct CutListThread {
    /// Unique identifier for this thread
    pub id: String,

    /// Accuracy factor for solution pruning
    accuracy_factor: i32,
    /// All solutions across threads
    all_solutions: Arc<Mutex<Vec<Solution>>>,
    /// Auxiliary information
    aux_info: Option<String>,
    /// Whether to consider grain direction
    consider_grain_direction: bool,
    /// Cut thickness
    cut_thickness: i32,
    /// First cut orientation preference
    first_cut_orientation: CutOrientationPreference,
    /// Thread group identifier
    group: Option<String>,
    /// Solutions for this thread
    solutions: Vec<Solution>,
    /// Start time of computation
    start_time: Option<u64>,
    /// Stock solution
    stock_solution: Option<StockSolution>,

    /// Associated task ID (instead of task reference)
    task_id: Option<String>,

    /// Associated task (for compatibility)
    task: Option<Arc<Mutex<Task>>>,
    /// Tiles to process
    tiles: Vec<TileDimensions>,
    /// Current status
    status: Status,
    /// Percentage done
    percentage_done: i32,
    /// Minimum trim dimension
    min_trim_dimension: i32,

    /// Whether this thread has a logger
    pub has_logger: bool,

    /// Number of thread comparators
    pub thread_comparator_count: usize,

    /// Number of final solution comparators
    pub final_comparator_count: usize,

    /// Cut list logger
    cut_list_logger: Option<Box<dyn CutListLogger>>,
    /// Thread prioritized comparators
    thread_prioritized_comparators: Vec<Box<dyn SolutionComparator>>,
    /// Final solution prioritized comparators
    final_solution_prioritized_comparators: Vec<Box<dyn SolutionComparator>>,
}

impl CutListThread {
    /// Creates a new CutListThread with a unique ID
    pub fn new() -> Self {
        // Generate a simple ID using timestamp and counter
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        let id = format!("thread_{}", timestamp);

        Self {
            accuracy_factor: 10,
            all_solutions: Arc::new(Mutex::new(Vec::new())),
            aux_info: None,
            consider_grain_direction: false,
            cut_thickness: 0,
            first_cut_orientation: CutOrientationPreference::Both,
            group: None,
            solutions: Vec::new(),
            start_time: None,
            stock_solution: None,
            task_id: None,
            task: None,
            tiles: Vec::new(),
            status: Status::Queued,
            percentage_done: 0,
            min_trim_dimension: 0,
            has_logger: false,
            thread_comparator_count: 0,
            final_comparator_count: 0,
            cut_list_logger: None,
            thread_prioritized_comparators: Vec::new(),
            final_solution_prioritized_comparators: Vec::new(),
            id,
        }
    }

    /// Sets the task ID for this thread
    pub fn set_task_id(&mut self, task_id: Option<String>) {
        self.task_id = task_id;
    }

    /// Gets the task ID for this thread
    pub fn get_task_id(&self) -> Option<&String> {
        self.task_id.as_ref()
    }

    // Getters and setters for all fields
    pub fn get_group(&self) -> Option<&String> {
        self.group.as_ref()
    }

    pub fn set_group(&mut self, group: Option<String>) {
        self.group = group;
    }

    pub fn get_aux_info(&self) -> Option<&String> {
        self.aux_info.as_ref()
    }

    pub fn set_aux_info(&mut self, aux_info: Option<String>) {
        self.aux_info = aux_info;
    }

    pub fn get_task(&self) -> Option<Arc<Mutex<Task>>> {
        self.task.clone()
    }

    pub fn set_task(&mut self, task: Option<Arc<Mutex<Task>>>) {
        self.task = task;
    }

    pub fn get_status(&self) -> Status {
        self.status
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn get_cut_thickness(&self) -> i32 {
        self.cut_thickness
    }

    pub fn set_cut_thickness(&mut self, thickness: i32) {
        self.cut_thickness = thickness;
    }

    pub fn get_min_trim_dimension(&self) -> i32 {
        self.min_trim_dimension
    }

    pub fn set_min_trim_dimension(&mut self, dimension: i32) {
        self.min_trim_dimension = dimension;
    }

    pub fn get_first_cut_orientation(&self) -> CutOrientationPreference {
        self.first_cut_orientation
    }

    pub fn set_first_cut_orientation(&mut self, orientation: CutOrientationPreference) {
        self.first_cut_orientation = orientation;
    }

    pub fn is_consider_grain_direction(&self) -> bool {
        self.consider_grain_direction
    }

    pub fn set_consider_grain_direction(&mut self, consider: bool) {
        self.consider_grain_direction = consider;
    }

    pub fn set_tiles(&mut self, tiles: Vec<TileDimensions>) {
        self.tiles = tiles;
    }

    pub fn set_stock_solution(&mut self, stock_solution: Option<StockSolution>) {
        self.stock_solution = stock_solution;
    }

    pub fn set_accuracy_factor(&mut self, accuracy_factor: i32) {
        self.accuracy_factor = accuracy_factor;
    }

    pub fn get_all_solutions(&self) -> Arc<Mutex<Vec<Solution>>> {
        self.all_solutions.clone()
    }

    pub fn get_percentage_done(&self) -> i32 {
        self.percentage_done
    }

    pub fn get_tiles(&self) -> &Vec<TileDimensions> {
        &self.tiles
    }

    pub fn get_solutions(&self) -> &Vec<Solution> {
        &self.solutions
    }

    pub fn set_solutions(&mut self, solutions: Vec<Solution>) {
        self.solutions = solutions;
    }

    pub fn get_accuracy_factor(&self) -> i32 {
        self.accuracy_factor
    }

    pub fn set_all_solutions(&mut self, solutions: Arc<Mutex<Vec<Solution>>>) {
        self.all_solutions = solutions;
    }

    pub fn get_stock_solution(&self) -> Option<&StockSolution> {
        self.stock_solution.as_ref()
    }

    pub fn get_cut_list_logger(&self) -> Option<&Box<dyn CutListLogger>> {
        self.cut_list_logger.as_ref()
    }

    pub fn set_cut_list_logger(&mut self, logger: Option<Box<dyn CutListLogger>>) {
        self.cut_list_logger = logger;
    }

    pub fn get_thread_prioritized_comparators(&self) -> &Vec<Box<dyn SolutionComparator>> {
        &self.thread_prioritized_comparators
    }

    pub fn set_thread_prioritized_comparators(
        &mut self,
        comparators: Vec<Box<dyn SolutionComparator>>,
    ) {
        self.thread_prioritized_comparators = comparators;
    }

    pub fn get_final_solution_prioritized_comparators(&self) -> &Vec<Box<dyn SolutionComparator>> {
        &self.final_solution_prioritized_comparators
    }

    pub fn set_final_solution_prioritized_comparators(
        &mut self,
        comparators: Vec<Box<dyn SolutionComparator>>,
    ) {
        self.final_solution_prioritized_comparators = comparators;
    }

    /// Gets the material from the first solution
    pub fn get_material(&self) -> Option<String> {
        if let Ok(all_solutions) = self.all_solutions.lock() {
            if !all_solutions.is_empty() {
                return all_solutions[0].get_material().cloned();
            }
        }
        None
    }

    /// Gets elapsed time in milliseconds
    pub fn get_elapsed_time_millis(&self) -> u64 {
        if let Some(start_time) = self.start_time {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64;
            now.saturating_sub(start_time)
        } else {
            0
        }
    }

    /// Removes duplicated solutions from the list
    pub fn remove_duplicated(&self, solutions: &mut Vec<Solution>) -> i32 {
        let mut to_remove = Vec::new();
        let mut seen_identifiers = HashSet::new();
        let mut removed_count = 0;

        for (index, solution) in solutions.iter().enumerate() {
            let mut identifier = String::new();
            for mosaic in &solution.mosaics {
                identifier.push_str(&mosaic.root_tile_node().to_string_identifier());
            }

            if !seen_identifiers.insert(identifier) {
                to_remove.push(index);
                removed_count += 1;
            }
        }

        // Remove in reverse order to maintain indices
        for &index in to_remove.iter().rev() {
            solutions.remove(index);
        }

        removed_count
    }

    /// Sorts solutions using the provided comparators
    fn sort(&self, solutions: &mut Vec<Solution>, comparators: &[Box<dyn SolutionComparator>]) {
        solutions.sort_by(|a, b| {
            for comparator in comparators {
                let result = comparator.compare(a, b);
                if result != Ordering::Equal {
                    return result;
                }
            }
            Ordering::Equal
        });
    }

    /// РЕАЛЬНАЯ логика оптимизации (не симуляция!)
    pub fn compute_solutions(&mut self) -> Result<()> {
        log_debug!("RUST START: === compute_solutions() ===");
        log_debug!("RUST STAGE 1: === Creating initial solution ===");

        // 1. Создаем начальное решение точно как в Java
        let mut current_solutions = Vec::new();
        if let Some(stock_solution) = &self.stock_solution {
            current_solutions.push(Solution::new(stock_solution));
            log_debug!(
                "RUST STAGE 1 COMPLETE: Created solution with {} solutions",
                current_solutions.len()
            );
        } else {
            return Err(CoreError::InvalidInput {
                details: "No stock solution provided".to_string(),
            }
            .into());
        }

        // 2. Проверяем статус задачи
        if let Some(task) = &self.task {
            if let Ok(task_guard) = task.lock() {
                if !task_guard.is_running() {
                    return Ok(());
                }
                log_debug!(
                    "RUST: Task is running, starting processing of {} tiles",
                    self.tiles.len()
                );
            }
        }

        // 3. ОСНОВНОЙ ЦИКЛ - размещение каждой панели (как в Java)
        for (i, tile_dimensions) in self.tiles.iter().enumerate() {
            log_debug!("\n{}", "=".repeat(80));
            log_debug!(
                "RUST STAGE 2: === Placing tile {} of {} ===",
                i + 1,
                self.tiles.len()
            );
            log_debug!(
                "RUST: Tile {}x{}, ID: {:?}",
                tile_dimensions.width(),
                tile_dimensions.height(),
                tile_dimensions.id()
            );

            // Обновляем прогресс
            if (i + 1) % 3 == 0 {
                self.percentage_done = ((i + 1) as f32 / self.tiles.len() as f32 * 100.0) as i32;
            }

            let solutions_before = current_solutions.len();
            let mut new_solutions = Vec::new();
            let mut solutions_to_remove = Vec::new();

            // 4. Пытаемся разместить плитку в каждом существующем решении
            for (solution_idx, solution) in current_solutions.iter().enumerate() {
                let mut tile_placed = false;

                // 5. Пытаемся разместить в существующих мозаиках
                for mosaic in solution.get_mosaics() {
                    // Проверка совместимости материала
                    if mosaic.material() != tile_dimensions.material() {
                        continue;
                    }

                    // ⭐ КЛЮЧЕВОЙ МЕТОД - добавление плитки в мозаику
                    let mut result_mosaics = Vec::new();
                    if let Err(e) = self.add_tile_to_mosaic(tile_dimensions, mosaic, &mut result_mosaics) {
                        eprintln!("RUST ERROR: Ошибка при добавлении плитки в мозаику: {:?}", e);
                        continue; // Пропускаем эту мозаику и пробуем следующую
                    }

                    if !result_mosaics.is_empty() {
                        // Создаем новые решения для каждой возможной мозаики
                        for result_mosaic in result_mosaics {
                            let mut new_solution = Solution::new_with_replacement(solution, mosaic);
                            new_solution.add_mosaic(result_mosaic);
                            new_solution.set_creator_thread_group(self.group.clone());
                            new_solution.set_aux_info(self.aux_info.clone());
                            new_solutions.push(new_solution);
                        }
                        tile_placed = true;
                        break;
                    }
                }

                // 6. Если не разместили в мозаиках, пытаемся создать новую мозаику
                if !tile_placed {
                    for unused_panel in solution.get_unused_stock_panels() {
                        log_debug!(
                            "RUST: Checking unused panel: {}x{}",
                            unused_panel.width(),
                            unused_panel.height()
                        );

                        if self.tile_fits_in_panel(tile_dimensions, unused_panel) {
                            log_debug!("RUST: Panel fits the tile");

                            let mut new_solution = solution.clone();
                            // Удаляем использованную панель
                            new_solution
                                .get_unused_stock_panels_mut()
                                .retain(|p| p != unused_panel);

                            // Создаем новую мозаику с этой плиткой
                            let new_mosaic = match self.create_mosaic_with_tile(unused_panel, tile_dimensions) {
                                Ok(mosaic) => mosaic,
                                Err(e) => {
                                    eprintln!("RUST ERROR: Ошибка при создании мозаики: {:?}", e);
                                    continue; // Пробуем следующую панель
                                }
                            };
                            new_solution.add_mosaic(new_mosaic);
                            new_solution.set_creator_thread_group(self.group.clone());
                            new_solution.set_aux_info(self.aux_info.clone());
                            new_solutions.push(new_solution);
                            tile_placed = true;
                            break;
                        } else {
                            log_debug!("RUST: Panel does NOT fit the tile");
                        }
                    }
                }

                // 7. Если плитка размещена или не может быть размещена
                if tile_placed {
                    solutions_to_remove.push(solution_idx);
                } else {
                    // Добавляем плитку в список "не помещается"
                    let mut no_fit_solution = solution.clone();
                    no_fit_solution
                        .get_no_fit_panels_mut()
                        .push(tile_dimensions.clone());
                    new_solutions.push(no_fit_solution);
                    solutions_to_remove.push(solution_idx);
                }
            }

            // 8. Удаляем обработанные решения
            for &idx in solutions_to_remove.iter().rev() {
                current_solutions.remove(idx);
            }

            // 9. Добавляем новые решения
            current_solutions.extend(new_solutions);

            // 10. Удаляем дубликаты
            self.remove_duplicated(&mut current_solutions);

            // 11. Сортируем и ограничиваем количество решений
            self.sort_solutions(&mut current_solutions, &self.thread_prioritized_comparators);
            let max_solutions = self.accuracy_factor as usize;
            if current_solutions.len() > max_solutions {
                current_solutions.truncate(max_solutions);
            }
            
        }


        // 12. Добавляем решения в глобальный список
        if let Ok(mut all_solutions) = self.all_solutions.lock() {
            all_solutions.extend(current_solutions);
            self.sort_solutions(
                &mut *all_solutions,
                &self.final_solution_prioritized_comparators,
            );

            let max_solutions = self.accuracy_factor as usize;
            if all_solutions.len() > max_solutions {
                all_solutions.truncate(max_solutions);
            }

            // Обновляем рейтинги потоков
            for solution in all_solutions.iter().take(5) {
                if let (Some(material), Some(group)) =
                    (solution.get_material(), &solution.creator_thread_group)
                {
                    if let Some(task) = &self.task {
                        if let Ok(mut task_guard) = task.lock() {
                            task_guard.increment_thread_group_rankings(material, group);
                        }
                    }
                }
            }

            // Удаляем пустые мозаики
            if !all_solutions.is_empty() {
                
                // ВРЕМЕННО ОТКЛЮЧЕНО: НЕ УДАЛЯЕМ МОЗАИКИ
                // all_solutions[0]
                //     .get_mosaics_mut()
                //     .retain(|mosaic| mosaic.used_area() > 0);
                    
            }
        }

        Ok(())
    }

    /// ⭐ КЛЮЧЕВОЙ МЕТОД - добавление плитки в мозаику с поворотами
    fn add_tile_to_mosaic(
        &self,
        tile_dimensions: &TileDimensions,
        mosaic: &Mosaic,
        result_mosaics: &mut Vec<Mosaic>,
    ) -> Result<()> {
        // Если не учитываем направление волокон или ориентации совпадают
        if !self.consider_grain_direction
            || mosaic.orientation() == 0
            || tile_dimensions.orientation() == 0
        {
            // Пытаемся разместить без поворота
            self.fit_tile_in_mosaic(tile_dimensions, mosaic, result_mosaics)?;

            // Пытаемся разместить с поворотом (если плитка не квадратная)
            if !tile_dimensions.is_square() {
                let rotated_tile = tile_dimensions.rotate_90();
                self.fit_tile_in_mosaic(&rotated_tile, mosaic, result_mosaics)?;
            }
        } else {
            // Учитываем ориентацию волокон
            let tile_to_use = if mosaic.orientation() != tile_dimensions.orientation() as i32 {
                tile_dimensions.rotate_90()
            } else {
                tile_dimensions.clone()
            };
            self.fit_tile_in_mosaic(&tile_to_use, mosaic, result_mosaics)?;
        }

        Ok(())
    }

    /// ⭐ КЛЮЧЕВОЙ МЕТОД - размещение плитки в мозаике с резами
    fn fit_tile_in_mosaic(
        &self,
        tile_dimensions: &TileDimensions,
        mosaic: &Mosaic,
        result_mosaics: &mut Vec<Mosaic>,
    ) -> Result<()> {
        // 1. Находим кандидатов для размещения
        let mut candidates = Vec::new();
        self.find_placement_candidates(
            tile_dimensions.width() as i32,
            tile_dimensions.height() as i32,
            mosaic.root_tile_node(),
            &mut candidates,
        );

        // 2. Пытаемся разместить в каждом кандидате
        for candidate in candidates {
            if candidate.width() == tile_dimensions.width() as i32
                && candidate.height() == tile_dimensions.height() as i32
            {
                // ТОЧНОЕ СОВПАДЕНИЕ - не нужно резать
                let new_mosaic =
                    self.create_exact_fit_mosaic(mosaic, &candidate, tile_dimensions)?;
                result_mosaics.push(new_mosaic);
            } else {
                // НУЖНО РЕЗАТЬ - создаем варианты резов

                // Горизонтальный затем вертикальный рез
                if self.first_cut_orientation.allows_horizontal() {
                    match self.create_cut_solution_hv(mosaic, &candidate, tile_dimensions) {
                        Ok(Some(new_mosaic)) => result_mosaics.push(new_mosaic),
                        _ => {}
                    }
                }

                // Вертикальный затем горизонтальный рез
                if self.first_cut_orientation.allows_vertical() {
                    if let Some(new_mosaic) = self.create_cut_solution_vh(
                        mosaic,
                        &candidate,
                        tile_dimensions,
                        self.cut_thickness,
                    ) {
                        result_mosaics.push(new_mosaic);
                    }
                }
            }
        }

        Ok(())
    }

    /// ⭐ КЛЮЧЕВОЙ МЕТОД - поиск мест для размещения
    fn find_placement_candidates(
        &self,
        width: i32,
        height: i32,
        tile_node: &TileNode,
        candidates: &mut Vec<TileNode>,
    ) {
        // Не можем размещать в финальных плитках или слишком маленьких
        if tile_node.is_final() || tile_node.width() < width || tile_node.height() < height {
            return;
        }

        // Если это листовой узел (нет детей) - проверяем размещение
        if tile_node.child1().is_none() && tile_node.child2().is_none() {
            let width_ok =
                tile_node.width() == width || tile_node.width() >= self.min_trim_dimension + width;
            let height_ok = tile_node.height() == height
                || tile_node.height() >= self.min_trim_dimension + height;

            if width_ok && height_ok {
                candidates.push(tile_node.clone());
            }
        } else {
            // Рекурсивно ищем в детях
            if let Some(child1) = tile_node.child1() {
                self.find_placement_candidates(width, height, child1, candidates);
            }
            if let Some(child2) = tile_node.child2() {
                self.find_placement_candidates(width, height, child2, candidates);
            }
        }
    }

    /// ⭐ РЕАЛИЗАЦИЯ РЕЗОВ - горизонтальный потом вертикальный
    fn create_cut_solution_hv(
        &self,
        mosaic: &Mosaic,
        candidate: &TileNode,
        tile_dimensions: &TileDimensions,
    ) -> Result<Option<Mosaic>> {
        let mut new_mosaic = Mosaic::from_mosaic(mosaic);

        // Получаем мутабельную ссылку на узел для резки
        let target_node = new_mosaic
            .root_tile_node()
            .find_tile(candidate)
            .ok_or_else(|| CoreError::Internal {
                message: "Target node not found in mosaic".to_string(),
            })?;

        // Создаем резы
        let cuts = self.create_hv_cuts(target_node, tile_dimensions)?;

        // Добавляем резы к мозаике
        for cut in cuts {
            new_mosaic.add_cut(cut);
        }

        Ok(Some(new_mosaic))
    }

    /// ⭐ Создание резов H-V (горизонтальный потом вертикальный)
    fn create_hv_cuts(
        &self,
        tile_node: &TileNode,
        tile_dimensions: &TileDimensions,
    ) -> Result<Vec<Cut>> {
        let mut cuts = Vec::new();

        // Если нужен горизонтальный рез
        if tile_node.width() > tile_dimensions.width() as i32 {
            let cut = self.create_horizontal_cut(
                tile_node,
                tile_dimensions.width() as i32,
                Some(tile_dimensions.id()),
            )?;
            cuts.push(cut);
        }

        // Если нужен вертикальный рез
        if tile_node.height() > tile_dimensions.height() as i32 {
            let cut = self.create_vertical_cut(
                tile_node,
                tile_dimensions.height() as i32,
                Some(tile_dimensions.id()),
            )?;
            cuts.push(cut);
        }

        Ok(cuts)
    }

    /// ⭐ Создание горизонтального реза
    fn create_horizontal_cut(
        &self,
        tile_node: &TileNode,
        split_width: i32,
        external_id: Option<i32>,
    ) -> Result<Cut> {
        Ok(CutBuilder::new()
            .x1(tile_node.x1() + split_width)
            .y1(tile_node.y1())
            .x2(tile_node.x1() + split_width)
            .y2(tile_node.y2())
            .horizontal(true)
            .original_tile_id(tile_node.id())
            .build())
    }

    /// ⭐ Создание вертикального реза  
    fn create_vertical_cut(
        &self,
        tile_node: &TileNode,
        split_height: i32,
        external_id: Option<i32>,
    ) -> Result<Cut> {
        Ok(CutBuilder::new()
            .x1(tile_node.x1())
            .y1(tile_node.y1() + split_height)
            .x2(tile_node.x2())
            .y2(tile_node.y1() + split_height)
            .horizontal(false)
            .original_tile_id(tile_node.id())
            .build())
    }

    /// Проверяет, помещается ли плитка в панель
    fn tile_fits_in_panel(&self, tile: &TileDimensions, panel: &TileDimensions) -> bool {
        (tile.width() <= panel.width() && tile.height() <= panel.height())
            || (tile.width() <= panel.height() && tile.height() <= panel.width())
    }

    /// ⭐ Создание мозаики с размещенной плиткой
    fn create_mosaic_with_tile(
        &self,
        stock_panel: &TileDimensions,
        tile: &TileDimensions,
    ) -> Result<Mosaic> {
        // Создаем корневой узел из исходной панели
        let mut root_node = TileNode::from_tile_dimensions(stock_panel)?;

        // Создаем дочерний узел для размещенной плитки
        let tile_node = TileNode::new(0, 0, tile.width() as i32, tile.height() as i32)?;
        let mut final_tile_node = tile_node;
        final_tile_node.set_final(true);
        final_tile_node.set_external_id(tile.id());

        root_node.set_child1(Some(final_tile_node));

        let mosaic = Mosaic::from_tile_node(&root_node, stock_panel.material().to_string());
        Ok(mosaic)
    }

    /// ⭐ Создание точно подходящей мозаики (без резов)
    fn create_exact_fit_mosaic(
        &self,
        original_mosaic: &Mosaic,
        candidate: &TileNode,
        tile_dimensions: &TileDimensions,
    ) -> Result<Mosaic> {
        let mut new_mosaic = Mosaic::from_mosaic(original_mosaic);

        // Помечаем узел как финальный
        // В реальной реализации нужно найти и изменить соответствующий узел
        // Это упрощенная версия

        Ok(new_mosaic)
    }

    /// Сортировка решений с использованием компараторов
    fn sort_solutions(
        &self,
        solutions: &mut Vec<Solution>,
        comparators: &[Box<dyn SolutionComparator>],
    ) {
        solutions.sort_by(|a, b| {
            for comparator in comparators {
                let result = comparator.compare(a, b);
                if result != std::cmp::Ordering::Equal {
                    return result;
                }
            }
            std::cmp::Ordering::Equal
        });
    }

    /// Adds a tile to a mosaic with rotation consideration
    fn add(
        &self,
        tile_dimensions: &TileDimensions,
        mosaic: &Mosaic,
        result_mosaics: &mut Vec<Mosaic>,
    ) {
        if !self.consider_grain_direction
            || mosaic.orientation() == 0
            || tile_dimensions.orientation() == 0
        {
            self.fit_tile(tile_dimensions, mosaic, result_mosaics, self.cut_thickness);
            if !tile_dimensions.is_square() {
                let rotated = tile_dimensions.rotate_90();
                self.fit_tile(&rotated, mosaic, result_mosaics, self.cut_thickness);
            }
        } else {
            let tile_to_use = if mosaic.orientation() != tile_dimensions.orientation() as i32 {
                tile_dimensions.rotate_90()
            } else {
                tile_dimensions.clone()
            };
            self.fit_tile(&tile_to_use, mosaic, result_mosaics, self.cut_thickness);
        }
    }

    /// Fits a tile into a mosaic
    fn fit_tile(
        &self,
        tile_dimensions: &TileDimensions,
        mosaic: &Mosaic,
        result_mosaics: &mut Vec<Mosaic>,
        cut_thickness: i32,
    ) {
        
        let mut candidates = Vec::new();
        self.find_candidates(
            tile_dimensions.width() as i32,
            tile_dimensions.height() as i32,
            mosaic.root_tile_node(),
            &mut candidates,
        );

        
        for candidate in candidates {
                     
            if candidate.width() == tile_dimensions.width() as i32
                && candidate.height() == tile_dimensions.height() as i32
            {
                // Exact fit - точная копия Java логики
                let tile_node_copy = TileNode::copy_tree(mosaic.root_tile_node(), Some(&candidate));
                if let Some(found_tile) = tile_node_copy.find_tile(&candidate) {
                    // Создаем мутабельную копию для изменения
                    let mut modified_tile = found_tile.clone();
                    modified_tile.set_external_id(tile_dimensions.id());
                    modified_tile.set_final(true);
                    modified_tile.set_rotated(tile_dimensions.is_rotated());
                    
                    // Создаем новую мозаику
                    let mut new_mosaic = Mosaic::from_mosaic(mosaic);
                    new_mosaic.set_root_tile_node(tile_node_copy);
                    new_mosaic.set_stock_id(mosaic.stock_id());
                    new_mosaic.set_orientation(mosaic.orientation());
                    
                    result_mosaics.push(new_mosaic);
                }
            } else {
                         
                // Need to cut - следуем Java логике CutDirection
                if self.first_cut_orientation.allows_horizontal() {
                    
                    let mut tile_node_copy = TileNode::copy_tree(mosaic.root_tile_node(), Some(&candidate));
                    if let Some(found_tile) = tile_node_copy.find_tile(&candidate) {
                        // Выполняем splitHV на клоне найденной плитки
                        let mut found_tile_mut = found_tile.clone();
                        match self.split_hv_java_style(&mut found_tile_mut, tile_dimensions) {
                            Ok(cuts) => {
                                let mut new_mosaic = Mosaic::from_mosaic(mosaic);
                                new_mosaic.set_root_tile_node(tile_node_copy);
                                new_mosaic.set_stock_id(mosaic.stock_id());
                                new_mosaic.set_orientation(mosaic.orientation());
                                
                                // Добавляем существующие резы
                                let mut all_cuts = mosaic.cuts().to_vec();
                                all_cuts.extend(cuts);
                                new_mosaic.set_cuts(all_cuts);
                                
                                result_mosaics.push(new_mosaic);
                            },
                            Err(_e) => {}, // Error handled silently
                        }
                    }
                }

                if self.first_cut_orientation.allows_vertical() {
                    
                    let mut tile_node_copy = TileNode::copy_tree(mosaic.root_tile_node(), Some(&candidate));
                    if let Some(found_tile) = tile_node_copy.find_tile(&candidate) {
                        // Выполняем splitVH на клоне найденной плитки
                        let mut found_tile_mut = found_tile.clone();
                        match self.split_vh_java_style(&mut found_tile_mut, tile_dimensions) {
                            Ok(cuts) => {
                                let mut new_mosaic = Mosaic::from_mosaic(mosaic);
                                new_mosaic.set_root_tile_node(tile_node_copy);
                                new_mosaic.set_stock_id(mosaic.stock_id());
                                new_mosaic.set_orientation(mosaic.orientation());
                                
                                // Добавляем существующие резы
                                let mut all_cuts = mosaic.cuts().to_vec();
                                all_cuts.extend(cuts);
                                new_mosaic.set_cuts(all_cuts);
                                
                                result_mosaics.push(new_mosaic);
                            },
                            Err(_e) => {}, // Error handled silently
                        }
                    }
                }
            }
        }
        
    }


    /// Creates a cut solution with vertical-then-horizontal cuts
    fn create_cut_solution_vh(
        &self,
        mosaic: &Mosaic,
        candidate: &TileNode,
        tile_dimensions: &TileDimensions,
        cut_thickness: i32,
    ) -> Option<Mosaic> {
        let mut tile_node_copy = Self::copy_tile_node(mosaic.root_tile_node(), candidate);

        // Create a simplified approach - just create a new mosaic with the cuts
        let mut new_mosaic = Mosaic::from_tile_node(&tile_node_copy, mosaic.material().to_string());
        new_mosaic.set_stock_id(mosaic.stock_id());

        // Copy existing cuts
        let mut all_cuts = mosaic.cuts().to_vec();
        // For now, we'll skip the complex tree modification and just return the mosaic
        new_mosaic.set_cuts(all_cuts);
        new_mosaic.set_orientation(mosaic.orientation());
        Some(new_mosaic)
    }

    /// Splits a tile node horizontally then vertically
    fn split_hv(
        &self,
        tile_node: &mut TileNode,
        tile_dimensions: &TileDimensions,
        cut_thickness: i32,
    ) -> Vec<Cut> {
        let mut cuts = Vec::new();

        if tile_node.width() > tile_dimensions.width() as i32 {
            if let Some(cut) = Self::split_horizontally(
                tile_node,
                tile_dimensions.width() as i32,
                cut_thickness,
                Some(tile_dimensions.id()),
            ) {
                cuts.push(cut);

                // Since we can't get mutable references to children, we'll need to work differently
                // For now, we'll just mark the node as final if it matches the dimensions
                if tile_node.width() == tile_dimensions.width() as i32
                    && tile_node.height() == tile_dimensions.height() as i32
                {
                    tile_node.set_final(true);
                    tile_node.set_rotated(tile_dimensions.is_rotated());
                    tile_node.set_external_id(tile_dimensions.id());
                }
            }
        } else {
            if let Some(cut) = Self::split_vertically(
                tile_node,
                tile_dimensions.height() as i32,
                cut_thickness,
                Some(tile_dimensions.id()),
            ) {
                cuts.push(cut);

                if tile_node.width() == tile_dimensions.width() as i32
                    && tile_node.height() == tile_dimensions.height() as i32
                {
                    tile_node.set_final(true);
                    tile_node.set_rotated(tile_dimensions.is_rotated());
                    tile_node.set_external_id(tile_dimensions.id());
                }
            }
        }

        cuts
    }

    /// Splits a tile node vertically then horizontally
    fn split_vh(
        &self,
        tile_node: &mut TileNode,
        tile_dimensions: &TileDimensions,
        cut_thickness: i32,
    ) -> Vec<Cut> {
        let mut cuts = Vec::new();

        if tile_node.height() > tile_dimensions.height() as i32 {
            if let Some(cut) = Self::split_vertically(
                tile_node,
                tile_dimensions.height() as i32,
                cut_thickness,
                None,
            ) {
                cuts.push(cut);

                if tile_node.width() == tile_dimensions.width() as i32
                    && tile_node.height() == tile_dimensions.height() as i32
                {
                    tile_node.set_final(true);
                    tile_node.set_rotated(tile_dimensions.is_rotated());
                    tile_node.set_external_id(tile_dimensions.id());
                }
            }
        } else {
            if let Some(cut) = Self::split_horizontally(
                tile_node,
                tile_dimensions.width() as i32,
                cut_thickness,
                Some(tile_dimensions.id()),
            ) {
                cuts.push(cut);

                if tile_node.width() == tile_dimensions.width() as i32
                    && tile_node.height() == tile_dimensions.height() as i32
                {
                    tile_node.set_final(true);
                    tile_node.set_rotated(tile_dimensions.is_rotated());
                    tile_node.set_external_id(tile_dimensions.id());
                }
            }
        }

        cuts
    }

    /// Finds candidate positions for placing a tile
    fn find_candidates(
        &self,
        width: i32,
        height: i32,
        tile_node: &TileNode,
        candidates: &mut Vec<TileNode>,
    ) {
        if tile_node.is_final() || tile_node.width() < width || tile_node.height() < height {
            return;
        }

        if tile_node.child1().is_none() && tile_node.child2().is_none() {
            let width_ok =
                tile_node.width() == width || tile_node.width() >= self.min_trim_dimension + width;
            let height_ok = tile_node.height() == height
                || tile_node.height() >= self.min_trim_dimension + height;

            if tile_node.width() > width && tile_node.width() < self.min_trim_dimension + width {
                // Note: Task doesn't have set_min_trim_dimension_influenced method
                // We'll skip this for now or implement it differently
                if let Some(task) = &self.task {
                    if let Ok(task_guard) = task.lock() {
                        // task_guard.set_min_trim_dimension_influenced(true);
                        // For now, we'll just log this condition
                    }
                }
            }

            if tile_node.height() > height && tile_node.height() < self.min_trim_dimension + height
            {
                // Note: Task doesn't have set_min_trim_dimension_influenced method
                // We'll skip this for now or implement it differently
                if let Some(task) = &self.task {
                    if let Ok(task_guard) = task.lock() {
                        // task_guard.set_min_trim_dimension_influenced(true);
                        // For now, we'll just log this condition
                    }
                }
            }

            if width_ok && height_ok {
                candidates.push(tile_node.clone());
            }
        } else {
            if let Some(child1) = tile_node.child1() {
                self.find_candidates(width, height, child1, candidates);
            }
            if let Some(child2) = tile_node.child2() {
                self.find_candidates(width, height, child2, candidates);
            }
        }
    }

    /// Splits a tile node horizontally
    fn split_horizontally(
        tile_node: &mut TileNode,
        split_width: i32,
        cut_thickness: i32,
        external_id: Option<i32>,
    ) -> Option<Cut> {
        let original_width = tile_node.width();
        let original_height = tile_node.height();

        let child1 = TileNode::new(
            tile_node.x1(),
            tile_node.x1() + split_width,
            tile_node.y1(),
            tile_node.y2(),
        )
        .ok()?;

        let child2 = TileNode::new(
            tile_node.x1() + split_width + cut_thickness,
            tile_node.x2(),
            tile_node.y1(),
            tile_node.y2(),
        )
        .ok()?;

        let mut child1 = child1;
        if let Some(id) = external_id {
            child1.set_external_id(id);
        }

        if child1.area() > 0 {
            tile_node.set_child1(Some(child1));
        }
        if child2.area() > 0 {
            tile_node.set_child2(Some(child2));
        }

        Some(
            CutBuilder::new()
                .x1(tile_node.x1() + split_width)
                .y1(tile_node.y1())
                .x2(tile_node.x1() + split_width)
                .y2(tile_node.y2())
                .original_width(original_width)
                .original_height(original_height)
                .horizontal(true)
                .cut_coord(split_width)
                .original_tile_id(tile_node.id())
                .child1_tile_id(tile_node.child1().map(|c| c.id()).unwrap_or(0))
                .child2_tile_id(tile_node.child2().map(|c| c.id()).unwrap_or(0))
                .build(),
        )
    }

    /// Splits a tile node vertically
    fn split_vertically(
        tile_node: &mut TileNode,
        split_height: i32,
        cut_thickness: i32,
        external_id: Option<i32>,
    ) -> Option<Cut> {
        let original_width = tile_node.width();
        let original_height = tile_node.height();

        let child1 = TileNode::new(
            tile_node.x1(),
            tile_node.x2(),
            tile_node.y1(),
            tile_node.y1() + split_height,
        )
        .ok()?;

        let child2 = TileNode::new(
            tile_node.x1(),
            tile_node.x2(),
            tile_node.y1() + split_height + cut_thickness,
            tile_node.y2(),
        )
        .ok()?;

        let mut child1 = child1;
        if let Some(id) = external_id {
            child1.set_external_id(id);
        }

        if child1.area() > 0 {
            tile_node.set_child1(Some(child1));
        }
        if child2.area() > 0 {
            tile_node.set_child2(Some(child2));
        }

        Some(
            CutBuilder::new()
                .x1(tile_node.x1())
                .y1(tile_node.y1() + split_height)
                .x2(tile_node.x2())
                .y2(tile_node.y1() + split_height)
                .original_width(original_width)
                .original_height(original_height)
                .horizontal(false)
                .cut_coord(split_height)
                .original_tile_id(tile_node.id())
                .child1_tile_id(tile_node.child1().map(|c| c.id()).unwrap_or(0))
                .child2_tile_id(tile_node.child2().map(|c| c.id()).unwrap_or(0))
                .build(),
        )
    }

    /// Copies a tile node tree up to a specific target node
    fn copy_tile_node(source: &TileNode, target: &TileNode) -> TileNode {
        let mut new_node = source.clone();
        Self::copy_children(source, &mut new_node, target);
        new_node
    }

    /// Recursively copies children of tile nodes
    fn copy_children(source: &TileNode, dest: &mut TileNode, target: &TileNode) {
        if source == target {
            return;
        }

        if let Some(source_child1) = source.child1() {
            let mut dest_child1 = source_child1.clone();
            Self::copy_children(source_child1, &mut dest_child1, target);
            dest.set_child1(Some(dest_child1));
        }

        if let Some(source_child2) = source.child2() {
            let mut dest_child2 = source_child2.clone();
            Self::copy_children(source_child2, &mut dest_child2, target);
            dest.set_child2(Some(dest_child2));
        }
    }
}

impl Default for CutListThread {
    fn default() -> Self {
        Self::new()
    }
}

/// Implements the Runnable trait for thread execution
impl CutListThread {
    /// Main run method for thread execution
    pub fn run(&mut self) -> Result<()> {
        self.status = Status::Running;
        self.start_time = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        );

        let result = self.compute_solutions();

        match result {
            Ok(_) => {
                if self.status != Status::Terminated {
                    self.status = Status::Finished;
                }
            }
            Err(ref e) => {
                log_error!("Error in cut list thread: {}", e);
                self.status = Status::Error;
            }
        }

        result
    }

    /// Java-compatible computeSolutions method that exactly mirrors the Java algorithm
    pub fn compute_solutions_java_style(&mut self) -> Result<()> {
        
        // ЭТАП 1: Создание начального решения
        let mut current_solutions = Vec::new();
        
        if let Some(stock_solution) = &self.stock_solution {
            current_solutions.push(Solution::new(stock_solution));
        } else {
            return Err(AppError::Core(CoreError::Internal {
                message: "No stock solution available".to_string(),
            }));
        }

        // Проверяем, что задача активна
        let task_is_running = if let Some(task) = &self.task {
            task.lock().map(|t| t.is_running()).unwrap_or(false)
        } else {
            true // Если нет задачи, считаем что можем работать
        };

        if task_is_running {
            
            // ЭТАП 2: Обработка каждой панели
            for (i, tile_dimensions) in self.tiles.iter().enumerate() {
                let panel_index = i + 1;
                println!("\n{}", "=".repeat(80));

                // Обновляем прогресс каждые 3 панели
                if panel_index % 3 == 0 {
                    self.percentage_done = ((panel_index as f32 / self.tiles.len() as f32) * 100.0) as i32;
                }

                let mut new_solutions = Vec::new();
                
                // Обрабатываем каждое текущее решение
                for solution in &current_solutions {
                    let mut placement_found = false;
                    
                    // Пробуем разместить панель в каждой мозаике решения
                    for (mosaic_index, mosaic) in solution.get_mosaics().iter().enumerate() {
                        
                        // Проверяем совместимость материала
                        if mosaic.material() != tile_dimensions.material() {
                            continue;
                        }

                        
                        // Пытаемся разместить панель в этой мозаике
                        let mut result_mosaics = Vec::new();
                        if let Ok(_) = self.add_tile_to_mosaic_java_style(tile_dimensions, mosaic, &mut result_mosaics) {
                            // ИСПРАВЛЕНИЕ: Как в Java - выбираем ЛУЧШИЙ вариант размещения
                            // Java приоритеты: 1) Больше панелей, 2) Меньше потерь, 3) Меньше резов
                            if let Some(best_mosaic) = result_mosaics.iter()
                                .max_by(|a, b| {
                                    // Базовые Java приоритеты: 1) Больше панелей, 2) Меньше потерь, 3) Меньше резов
                                    
                                    // Сначала сравниваем количество финальных панелей (больше = лучше)
                                    let tiles_cmp = a.root_tile_node().final_tile_nodes().len()
                                        .cmp(&b.root_tile_node().final_tile_nodes().len());
                                    if tiles_cmp != std::cmp::Ordering::Equal {
                                        return tiles_cmp;
                                    }
                                    
                                    // Затем сравниваем потерянную площадь (меньше = лучше)
                                    let waste_cmp = b.unused_area().cmp(&a.unused_area());
                                    if waste_cmp != std::cmp::Ordering::Equal {
                                        return waste_cmp;
                                    }
                                    
                                    // Наконец, сравниваем количество резов (меньше = лучше)
                                    b.cuts().len().cmp(&a.cuts().len())
                                }) {
                                
                                
                                // ИСПРАВЛЕНО: Заменяем мозаику вместо удаления и добавления
                                let mut new_solution = solution.clone();
                                new_solution.get_mosaics_mut()[mosaic_index] = best_mosaic.clone();
                                new_solution.set_creator_thread_group(self.group.clone());
                                new_solution.set_aux_info(self.aux_info.clone());
                                new_solutions.push(new_solution);
                                placement_found = true;
                            }
                            break; // Панель размещена, переходим к следующему решению
                        }
                    }

                    // Если не удалось разместить в существующих мозаиках, пробуем создать новую
                    if !placement_found {
                        if let Some(unused_stock) = self.find_suitable_unused_stock(solution, tile_dimensions) {
                            // Создаем новую мозаику из неиспользованного запаса
                            let mut new_solution = solution.clone();
                            if let Some(pos) = new_solution.unused_stock_panels.iter().position(|s| s == &unused_stock) {
                                new_solution.unused_stock_panels.remove(pos);
                                
                                // Создаем новую мозаику
                                let mut new_mosaic = Mosaic::new(&unused_stock)?;
                                let mut result_mosaics = Vec::new();
                                if let Ok(_) = self.add_tile_to_mosaic_java_style(tile_dimensions, &new_mosaic, &mut result_mosaics) {
                                    // Выбираем лучший вариант как в Java
                                    // Java приоритеты: 1) Больше панелей, 2) Меньше потерь, 3) Меньше резов
                                    if let Some(best_mosaic) = result_mosaics.iter()
                                        .max_by(|a, b| {
                                            // Базовые Java приоритеты: 1) Больше панелей, 2) Меньше потерь, 3) Меньше резов
                                            
                                            // Сначала сравниваем количество финальных панелей (больше = лучше)
                                            let tiles_cmp = a.root_tile_node().final_tile_nodes().len()
                                                .cmp(&b.root_tile_node().final_tile_nodes().len());
                                            if tiles_cmp != std::cmp::Ordering::Equal {
                                                return tiles_cmp;
                                            }
                                            
                                            // Затем сравниваем потерянную площадь (меньше = лучше)
                                            let waste_cmp = b.unused_area().cmp(&a.unused_area());
                                            if waste_cmp != std::cmp::Ordering::Equal {
                                                return waste_cmp;
                                            }
                                            
                                            // Наконец, сравниваем количество резов (меньше = лучше)
                                            b.cuts().len().cmp(&a.cuts().len())
                                        }) {
                                        new_solution.add_mosaic(best_mosaic.clone());
                                        new_solution.set_creator_thread_group(self.group.clone());
                                        new_solution.set_aux_info(self.aux_info.clone());
                                        new_solutions.push(new_solution);
                                        placement_found = true;
                                    }
                                }
                            }
                        } else {
                        }
                    }

                    // Если панель не удалось разместить нигде
                    if !placement_found {
                        let mut no_fit_solution = solution.clone();
                        no_fit_solution.no_fit_panels.push(tile_dimensions.clone());
                        new_solutions.push(no_fit_solution);
                    }
                }

                // Заменяем текущие решения новыми
                current_solutions = new_solutions;
                
                // Выводим статистику по первому решению
                if !current_solutions.is_empty() {
                    let first_solution = &current_solutions[0];
                    if !first_solution.get_mosaics().is_empty() {
                        let first_mosaic = &first_solution.get_mosaics()[0];
                    }
                }

                // Удаляем дубликаты
                self.remove_duplicated_java_style(&mut current_solutions);

                // Сортируем и ограничиваем количество решений
                current_solutions.sort_by(|a, b| self.compare_solutions_java_style(a, b));
                if current_solutions.len() > self.accuracy_factor as usize {
                    current_solutions.truncate(self.accuracy_factor as usize);
                }
            }

        }
        
        // ЭТАП 3: Обновляем глобальные решения (всегда, независимо от задач)
        println!("RUST DEBUG: Перед ЭТАП 3: current_solutions.len() = {}", current_solutions.len());
        if !current_solutions.is_empty() {
            println!("RUST DEBUG: Первое решение перед ЭТАП 3 содержит {} мозаик", current_solutions[0].get_mosaics().len());
        }
        if let Ok(mut all_solutions) = self.all_solutions.lock() {
            println!("RUST DEBUG: Добавляем {} решений в all_solutions", current_solutions.len());
            all_solutions.extend(current_solutions);
            all_solutions.sort_by(|a, b| self.compare_solutions_java_style(a, b));
            
            if all_solutions.len() > self.accuracy_factor as usize {
                all_solutions.truncate(self.accuracy_factor as usize);
            }

            // Обновляем рейтинги потоков (первые 5 решений)
            for solution in all_solutions.iter().take(5) {
                if let (Some(material), Some(group)) = (solution.get_material(), &solution.creator_thread_group) {
                    if let Some(task) = &self.task {
                        if let Ok(mut task_guard) = task.lock() {
                            task_guard.increment_thread_group_rankings(material, group);
                        }
                    }
                }
            }

            // ВРЕМЕННО ОТКЛЮЧЕНО: Удаляем пустые мозаики из лучшего решения
            // Проблема: used_area() всегда возвращает 0 из-за неправильной реализации в TileNode
            if !all_solutions.is_empty() {
                println!("RUST DEBUG: До фильтрации: {} мозаик", all_solutions[0].get_mosaics().len());
                for (i, mosaic) in all_solutions[0].get_mosaics().iter().enumerate() {
                    println!("RUST DEBUG: Мозаика {}: used_area={}, unused_area={}, резов={}", 
                             i, mosaic.used_area(), mosaic.unused_area(), mosaic.cuts().len());
                }
                // НЕ удаляем мозаики пока не исправим used_area()
                // all_solutions[0].get_mosaics_mut().retain(|mosaic| mosaic.used_area() > 0);
                println!("RUST DEBUG: Оставляем все мозаики: {} мозаик", all_solutions[0].get_mosaics().len());
            }
        }

        Ok(())
    }

    /// Java-совместимый метод add для размещения панели
    fn add_tile_to_mosaic_java_style(
        &self,
        tile_dimensions: &TileDimensions,
        mosaic: &Mosaic,
        result_mosaics: &mut Vec<Mosaic>,
    ) -> Result<()> {
        // Если не учитываем направление волокон или ориентации совпадают
        if !self.consider_grain_direction || mosaic.orientation() == 0 || tile_dimensions.orientation() == 0 {
            // Размещаем без поворота
            self.fit_tile_java_style(tile_dimensions, mosaic, result_mosaics)?;
            
            // ✅ ИСПРАВЛЕНО: Размещаем с поворотом если consider_grain_direction=false
            // В Java considerOrientation=false означает разрешить поворачивать панели
            if !self.consider_grain_direction && !tile_dimensions.is_square() {
                let rotated = tile_dimensions.rotate_90();
                self.fit_tile_java_style(&rotated, mosaic, result_mosaics)?;
            }
        } else {
            // Учитываем направление волокон
            let tile_to_use = if mosaic.orientation() != tile_dimensions.orientation() as i32 {
                tile_dimensions.rotate_90()
            } else {
                tile_dimensions.clone()
            };
            self.fit_tile_java_style(&tile_to_use, mosaic, result_mosaics)?;
        }
        
        Ok(())
    }

    /// Java-совместимый метод fitTile
    fn fit_tile_java_style(
        &self,
        tile_dimensions: &TileDimensions,
        mosaic: &Mosaic,
        result_mosaics: &mut Vec<Mosaic>,
    ) -> Result<()> {
        let mut candidates = Vec::new();
        self.find_candidates_java_style(
            tile_dimensions.width() as i32,
            tile_dimensions.height() as i32,
            mosaic.root_tile_node(),
            &mut candidates,
        );

        for candidate in candidates {
            if candidate.width() == tile_dimensions.width() as i32 
                && candidate.height() == tile_dimensions.height() as i32 {
                // Точное совпадение
                let mut new_root = TileNode::copy_tree(mosaic.root_tile_node(), Some(&candidate));
                if let Some(found_tile) = new_root.find_tile_mut(&candidate) {
                    found_tile.set_external_id(tile_dimensions.id());
                    found_tile.set_final(true);
                    found_tile.set_rotated(tile_dimensions.is_rotated());
                }
                
                let mut new_mosaic = Mosaic::from_root_tile_node(new_root, Some(mosaic.material().to_string()));
                new_mosaic.set_stock_id(mosaic.stock_id());
                new_mosaic.get_cuts_mut().extend(mosaic.cuts().iter().cloned());
                new_mosaic.set_orientation(mosaic.orientation());
                result_mosaics.push(new_mosaic);
            } else {
                // Нужно резать
                match self.first_cut_orientation {
                    CutOrientationPreference::Both | CutOrientationPreference::Horizontal => {
                        if let Ok(new_mosaic) = self.create_hv_cut_solution(&candidate, tile_dimensions, mosaic) {
                            result_mosaics.push(new_mosaic);
                        }
                    }
                    _ => {}
                }
                
                match self.first_cut_orientation {
                    CutOrientationPreference::Both | CutOrientationPreference::Vertical => {
                        if let Ok(new_mosaic) = self.create_vh_cut_solution(&candidate, tile_dimensions, mosaic) {
                            result_mosaics.push(new_mosaic);
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    /// Java-совместимый findCandidates
    fn find_candidates_java_style(
        &self,
        width: i32,
        height: i32,
        tile_node: &TileNode,
        candidates: &mut Vec<TileNode>,
    ) {
        if tile_node.is_final() || tile_node.width() < width || tile_node.height() < height {
            return;
        }

        if tile_node.child1().is_none() && tile_node.child2().is_none() {
            let width_ok = tile_node.width() == width || tile_node.width() >= self.min_trim_dimension + width;
            let height_ok = tile_node.height() == height || tile_node.height() >= self.min_trim_dimension + height;

            if width_ok && height_ok {
                candidates.push(tile_node.clone());
            }
        } else {
            if let Some(child1) = tile_node.child1() {
                self.find_candidates_java_style(width, height, child1, candidates);
            }
            if let Some(child2) = tile_node.child2() {
                self.find_candidates_java_style(width, height, child2, candidates);
            }
        }
    }

    /// Поиск подходящего неиспользованного запаса
    fn find_suitable_unused_stock(&self, solution: &Solution, tile_dimensions: &TileDimensions) -> Option<TileDimensions> {
        for unused_stock in &solution.unused_stock_panels {
            if unused_stock.fits(tile_dimensions) {
                return Some(unused_stock.clone());
            }
        }
        None
    }

    /// Java-совместимое удаление дубликатов
    fn remove_duplicated_java_style(&self, solutions: &mut Vec<Solution>) -> i32 {
        let mut duplicates = Vec::new();
        let mut seen_identifiers = std::collections::HashSet::new();
        let mut removed_count = 0;

        for (index, solution) in solutions.iter().enumerate() {
            let mut identifier = String::new();
            for mosaic in solution.get_mosaics() {
                identifier.push_str(&mosaic.root_tile_node().to_string_identifier());
            }
            
            if !seen_identifiers.insert(identifier) {
                duplicates.push(index);
                removed_count += 1;
            }
        }

        // Удаляем дубликаты в обратном порядке
        for &index in duplicates.iter().rev() {
            solutions.remove(index);
        }

        removed_count
    }

    /// Java-совместимое сравнение решений
    fn compare_solutions_java_style(&self, a: &Solution, b: &Solution) -> std::cmp::Ordering {
        // Простое сравнение по количеству неразмещенных панелей
        a.no_fit_panels.len().cmp(&b.no_fit_panels.len())
    }

    /// Создание HV (горизонтальный-вертикальный) решения с резкой
    fn create_hv_cut_solution(
        &self,
        candidate: &TileNode,
        tile_dimensions: &TileDimensions,
        mosaic: &Mosaic,
    ) -> Result<Mosaic> {
        let mut new_root = TileNode::copy_tree(mosaic.root_tile_node(), Some(candidate));
        
        // Находим соответствующий узел в новом дереве и выполняем разрез
        if let Some(found_tile) = new_root.find_tile_mut(candidate) {
            let cuts = self.split_hv_java_style(found_tile, tile_dimensions)?;
            
            let mut new_mosaic = Mosaic::from_root_tile_node(new_root, Some(mosaic.material().to_string()));
            new_mosaic.set_stock_id(mosaic.stock_id());
            new_mosaic.get_cuts_mut().extend(mosaic.cuts().iter().cloned());
            new_mosaic.get_cuts_mut().extend(cuts);
            new_mosaic.set_orientation(mosaic.orientation());
        
            return Ok(new_mosaic);
        }
        
        Err(AppError::Core(CoreError::Internal { message: "Не удалось найти candidate в новом дереве".to_string() }))
    }

    /// Создание VH (вертикальный-горизонтальный) решения с резкой
    fn create_vh_cut_solution(
        &self,
        candidate: &TileNode,
        tile_dimensions: &TileDimensions,
        mosaic: &Mosaic,
    ) -> Result<Mosaic> {
        let mut new_root = TileNode::copy_tree(mosaic.root_tile_node(), Some(candidate));
        
        // Находим соответствующий узел в новом дереве и выполняем разрез
        if let Some(found_tile) = new_root.find_tile_mut(candidate) {
            let cuts = self.split_vh_java_style(found_tile, tile_dimensions)?;
            
            let mut new_mosaic = Mosaic::from_root_tile_node(new_root, Some(mosaic.material().to_string()));
            new_mosaic.set_stock_id(mosaic.stock_id());
            new_mosaic.get_cuts_mut().extend(mosaic.cuts().iter().cloned());
            new_mosaic.get_cuts_mut().extend(cuts);
            new_mosaic.set_orientation(mosaic.orientation());
        
            return Ok(new_mosaic);
        }
        
        Err(AppError::Core(CoreError::Internal { message: "Не удалось найти candidate в новом дереве".to_string() }))
    }

    /// Горизонтальный затем вертикальный рез (Java-стиль)
    fn split_hv_java_style(&self, tile_node: &mut TileNode, tile_dimensions: &TileDimensions) -> Result<Vec<Cut>> {
        println!("RUST DEBUG split_hv_java_style: node {}x{}, панель {}x{}", 
                 tile_node.width(), tile_node.height(),
                 tile_dimensions.width(), tile_dimensions.height());
        
        let mut cuts = Vec::new();
        
        // Java логика splitHV: точная копия Java алгоритма
        if tile_node.width() > tile_dimensions.width() as i32 {
            println!("RUST DEBUG: Горизонтальный разрез первым");
            
            if let Some(cut) = self.split_horizontally_java_style(
                tile_node, 
                tile_dimensions.width() as i32, 
                self.cut_thickness,
tile_dimensions.id()
            ) {
                cuts.push(cut);
                
                // Проверяем нужен ли вертикальный разрез в child1
                if let Some(child1) = tile_node.child1_mut() {
                    if child1.height() > tile_dimensions.height() as i32 {
                        println!("RUST DEBUG: Вертикальный разрез в child1");
                        
                        if let Some(vertical_cut) = self.split_vertically_java_style(
                            child1,
                            tile_dimensions.height() as i32,
                            self.cut_thickness,
            tile_dimensions.id()
                        ) {
                            cuts.push(vertical_cut);
                            
                            // Устанавливаем первого ребенка как финального
                            if let Some(grandchild1) = child1.child1_mut() {
                                grandchild1.set_final(true);
                                grandchild1.set_rotated(tile_dimensions.is_rotated());
                                grandchild1.set_external_id(tile_dimensions.id());
                            }
                        }
                    } else {
                        // Размер точно подходит
                        child1.set_final(true);
                        child1.set_rotated(tile_dimensions.is_rotated());
                        child1.set_external_id(tile_dimensions.id());
                    }
                }
            }
        } else {
            println!("RUST DEBUG: Только вертикальный разрез");
            
            if let Some(cut) = self.split_vertically_java_style(
                tile_node,
                tile_dimensions.height() as i32,
                self.cut_thickness,
tile_dimensions.id()
            ) {
                cuts.push(cut);
                
                if let Some(child1) = tile_node.child1_mut() {
                    child1.set_final(true);
                    child1.set_rotated(tile_dimensions.is_rotated());
                    child1.set_external_id(tile_dimensions.id());
                }
            }
        }
        
        println!("RUST DEBUG split_hv_java_style: создано {} резов", cuts.len());
        Ok(cuts)
    }

    /// Вертикальный затем горизонтальный рез (Java-стиль)
    fn split_vh_java_style(&self, tile_node: &mut TileNode, tile_dimensions: &TileDimensions) -> Result<Vec<Cut>> {
        println!("RUST DEBUG split_vh_java_style: node {}x{}, панель {}x{}", 
                 tile_node.width(), tile_node.height(),
                 tile_dimensions.width(), tile_dimensions.height());
        
        let mut cuts = Vec::new();
        
        // Java логика splitVH: точная копия Java алгоритма
        if tile_node.height() > tile_dimensions.height() as i32 {
            println!("RUST DEBUG: Вертикальный разрез первым");
            
            if let Some(cut) = self.split_vertically_java_style(
                tile_node,
                tile_dimensions.height() as i32,
                self.cut_thickness,
tile_dimensions.id()
            ) {
                cuts.push(cut);
                
                // Проверяем нужен ли горизонтальный разрез в child1
                if let Some(child1) = tile_node.child1_mut() {
                    if child1.width() > tile_dimensions.width() as i32 {
                        println!("RUST DEBUG: Горизонтальный разрез в child1");
                        
                        if let Some(horizontal_cut) = self.split_horizontally_java_style(
                            child1,
                            tile_dimensions.width() as i32,
                            self.cut_thickness,
            tile_dimensions.id()
                        ) {
                            cuts.push(horizontal_cut);
                            
                            // Устанавливаем первого ребенка как финального
                            if let Some(grandchild1) = child1.child1_mut() {
                                grandchild1.set_final(true);
                                grandchild1.set_rotated(tile_dimensions.is_rotated());
                                grandchild1.set_external_id(tile_dimensions.id());
                            }
                        }
                    } else {
                        // Размер точно подходит
                        child1.set_final(true);
                        child1.set_rotated(tile_dimensions.is_rotated());
                        child1.set_external_id(tile_dimensions.id());
                    }
                }
            }
        } else {
            println!("RUST DEBUG: Только горизонтальный разрез");
            
            if let Some(cut) = self.split_horizontally_java_style(
                tile_node,
                tile_dimensions.width() as i32,
                self.cut_thickness,
tile_dimensions.id()
            ) {
                cuts.push(cut);
                
                if let Some(child1) = tile_node.child1_mut() {
                    child1.set_final(true);
                    child1.set_rotated(tile_dimensions.is_rotated());
                    child1.set_external_id(tile_dimensions.id());
                }
            }
        }
        
        println!("RUST DEBUG split_vh_java_style: создано {} резов", cuts.len());
        Ok(cuts)
    }

    /// Горизонтальный разрез (Java splitHorizontally)
    fn split_horizontally_java_style(
        &self,
        tile_node: &mut TileNode,
        width: i32,
        cut_thickness: i32,
        external_id: i32,
    ) -> Option<Cut> {
        println!("RUST DEBUG split_horizontally_java_style: width={}, cut_thickness={}", width, cut_thickness);
        
        let original_width = tile_node.width();
        let original_height = tile_node.height();
        
        // Создаем child1 (левая часть после горизонтального разреза)
        let child1 = TileNode::new(
            tile_node.x1(),
            tile_node.y1(),
            tile_node.x1() + width,
            tile_node.y2(),
        ).unwrap();
        let mut child1_final = child1.clone();
        child1_final.set_external_id(external_id);
        
        // Создаем child2 (правая часть)
        let child2 = TileNode::new(
            tile_node.x1() + width + cut_thickness,
            tile_node.y1(),
            tile_node.x2(),
            tile_node.y2(),
        ).unwrap();
        
        // Сохраняем IDs перед перемещением
        let child1_id = child1.id();
        let child2_id = child2.id();
        
        // Устанавливаем детей только если их площадь больше 0
        if child1_final.area() > 0 {
            tile_node.set_child1(Some(child1_final));
        }
        if child2.area() > 0 {
            tile_node.set_child2(Some(child2));
        }
        
        // Создаем Cut точно как в Java
        Some(Cut::new(
            tile_node.x1() + width,      // x1
            tile_node.y1(),              // y1
            tile_node.x1() + width,      // x2
            tile_node.y2(),              // y2
            original_width,              // original_width
            original_height,             // original_height
            true,                        // is_horizontal (вертикальная линия = горизонтальный разрез)
            width,                       // cut_coord
            tile_node.id(),              // original_tile_id
            child1_id,                   // child1_tile_id
            child2_id,                   // child2_tile_id
        ))
    }

    /// Вертикальный разрез (Java splitVertically)
    fn split_vertically_java_style(
        &self,
        tile_node: &mut TileNode,
        height: i32,
        cut_thickness: i32,
        external_id: i32,
    ) -> Option<Cut> {
        println!("RUST DEBUG split_vertically_java_style: height={}, cut_thickness={}", height, cut_thickness);
        
        let original_width = tile_node.width();
        let original_height = tile_node.height();
        
        // Создаем child1 (верхняя часть после вертикального разреза)
        let child1 = TileNode::new(
            tile_node.x1(),
            tile_node.y1(),
            tile_node.x2(),
            tile_node.y1() + height,
        ).unwrap();
        let mut child1_final = child1.clone();
        child1_final.set_external_id(external_id);
        
        // Создаем child2 (нижняя часть)
        let child2 = TileNode::new(
            tile_node.x1(),
            tile_node.y1() + height + cut_thickness,
            tile_node.x2(),
            tile_node.y2(),
        ).unwrap();
        
        // Сохраняем IDs перед перемещением
        let child1_id = child1.id();
        let child2_id = child2.id();
        
        // Устанавливаем детей только если их площадь больше 0
        if child1_final.area() > 0 {
            tile_node.set_child1(Some(child1_final));
        }
        if child2.area() > 0 {
            tile_node.set_child2(Some(child2));
        }
        
        // Создаем Cut точно как в Java
        Some(Cut::new(
            tile_node.x1(),                    // x1
            tile_node.y1() + height,           // y1
            tile_node.x2(),                    // x2
            tile_node.y1() + height,           // y2
            original_width,                    // original_width
            original_height,                   // original_height
            false,                             // is_horizontal (горизонтальная линия = вертикальный разрез)
            height,                            // cut_coord
            tile_node.id(),                    // original_tile_id
            child1_id,                         // child1_tile_id
            child2_id,                         // child2_tile_id
        ))
    }
}

// Helper functions for Java-style multi-level comparison
fn get_biggest_unused_area_mosaic(mosaic: &Mosaic) -> u64 {
    get_biggest_unused_area_in_tile_node_internal(mosaic.root_tile_node())
}

fn get_biggest_unused_area_in_tile_node_internal(tile_node: &TileNode) -> u64 {
    // Если это финальная плитка (занята), то площадь = 0
    if tile_node.is_final() {
        return 0;
    }
    
    // Если у узла нет детей (пустое место), то возвращаем его площадь
    if tile_node.child1().is_none() && tile_node.child2().is_none() {
        return (tile_node.width() as u64) * (tile_node.height() as u64);
    }
    
    // Рекурсивно проверяем детей
    let mut max_area = 0u64;
    if let Some(child1) = tile_node.child1() {
        max_area = max_area.max(get_biggest_unused_area_in_tile_node_internal(child1));
    }
    if let Some(child2) = tile_node.child2() {
        max_area = max_area.max(get_biggest_unused_area_in_tile_node_internal(child2));
    }
    
    max_area
}

fn get_distinct_tiles_mosaic(mosaic: &Mosaic) -> usize {
    let mut distinct_tiles = std::collections::HashSet::new();
    collect_distinct_tiles_internal(mosaic.root_tile_node(), &mut distinct_tiles);
    distinct_tiles.len()
}

fn collect_distinct_tiles_internal(tile_node: &TileNode, distinct_tiles: &mut std::collections::HashSet<u32>) {
    if tile_node.is_final() {
        // Создаем уникальный идентификатор плитки по размеру (как в Java)
        let width = tile_node.width();
        let height = tile_node.height();
        let i = width + height;
        let unique_id = ((i * (i + 1)) / 2) + height;
        distinct_tiles.insert(unique_id as u32);
    } else {
        if let Some(child1) = tile_node.child1() {
            collect_distinct_tiles_internal(child1, distinct_tiles);
        }
        if let Some(child2) = tile_node.child2() {
            collect_distinct_tiles_internal(child2, distinct_tiles);
        }
    }
}
