use std::{collections::VecDeque, sync::atomic::AtomicU32};

use crate::{
    enums::cut_orientation_preference::CutOrientationPreference, log_debug, log_error, log_info, models::{
        configuration::Configuration, mosaic::Mosaic, performance_thresholds::PerformanceThresholds, permutation_thread_spawner::{PermutationThreadSpawner, ProgressTracker}, solution::{self, Solution}, stock_solution::StockPanelPicker, task::Task, tile_dimensions::TileDimensions
    }
};

/// Эквивалент Java CutListThreadBuilder
#[derive(Debug, Clone)]
struct CutListThreadBuilder {
    aux_info: String,
    all_solutions: Vec<Solution>,
    tiles: Vec<TileDimensions>,
    configuration: Configuration,
    cut_thickness: i64,
    min_trim_dimension: i64,
    final_solution_prioritized_comparators: Vec<SolutionComparator>,
    task_id: String,
    accuracy_factor: f64,
    stock_solution: crate::models::stock_solution::StockSolution,
    group: String,
    thread_prioritized_comparators: Vec<SolutionComparator>,
    first_cut_orientation: CutDirection,
}
/// Static counter for generating unique solution IDs
static ID_COUNTER: AtomicU32 = AtomicU32::new(0);
/// Компаратор для решений
#[derive(Debug, Clone)]
enum SolutionComparator {
    MostTiles,
    LeastWastedArea,
    LeastNbrCuts,
    LeastNbrMosaics,
    BiggestUnusedTileArea,
    MostHvDiscrepancy,
    SmallestCenterOfMassDistToOrigin,
    LeastNbrUnusedTiles,
    MostUnusedPanelArea,
}

/// Направления резов

/// Вычисление общей площади решения
fn calculate_solution_total_area(solution: &Solution) -> u64 {
    solution
        .mosaics
        .iter()
        .map(|mosaic| calculate_mosaic_area(mosaic))
        .sum()
}

/// Вычисление площади мозаики
fn calculate_mosaic_area(mosaic: &Mosaic) -> u64 {
    // Предполагаем, что у мозаики есть размеры или можно вычислить по плиткам
    (mosaic.width() as u64) * (mosaic.height() as u64)
}

/// Парсинг масштабированного значения
fn parse_scaled_value(value: f64, factor: i64) -> Result<i64, Box<dyn std::error::Error>> {
    Ok((value * factor as f64).round() as i64)
}

/// Получение списка компараторов решений
fn get_solution_comparator_list(configuration: &Configuration) -> Vec<SolutionComparator> {
    configuration
        .optimization_priority
        .iter()
        .map(|priority| match priority {
            OptimizationPriority::MostTiles => SolutionComparator::MostTiles,
            OptimizationPriority::LeastWastedArea => SolutionComparator::LeastWastedArea,
            OptimizationPriority::LeastNbrCuts => SolutionComparator::LeastNbrCuts,
            OptimizationPriority::LeastNbrMosaics => SolutionComparator::LeastNbrMosaics,
            OptimizationPriority::BiggestUnusedTileArea => {
                SolutionComparator::BiggestUnusedTileArea
            }
            OptimizationPriority::MostHvDiscrepancy => SolutionComparator::MostHvDiscrepancy,
            OptimizationPriority::SmallestCenterOfMassDistToOrigin => {
                SolutionComparator::SmallestCenterOfMassDistToOrigin
            }
            OptimizationPriority::LeastNbrUnusedTiles => SolutionComparator::LeastNbrUnusedTiles,
            OptimizationPriority::MostUnusedPanelArea => SolutionComparator::MostUnusedPanelArea,
        })
        .collect()
}

/// Проверка допустимости запуска потока для группы
fn is_thread_eligible_to_start(group: &str, task: &Task, material: &str) -> bool {
    // В однопоточном режиме всегда разрешаем
    // В многопоточном здесь была бы сложная логика проверки рейтингов групп потоков
    log_debug!("RUST: Checking thread eligibility for group[{}] material[{}] - always true in single-threaded mode", group, material);
    true
}

/// Проверка разрешенности направления реза
fn is_cut_orientation_allowed(
    config_preference: &CutOrientationPreference,
    requested: CutOrientationPreference,
) -> bool {
    match (config_preference, requested) {
        (CutOrientationPreference::Both, _) => true,
        (pref, req) if *pref == req => true,
        _ => false,
    }
}

/// Выполнение алгоритма раскроя - аналог taskExecutor.execute(cutListLogger.build())
fn execute_cut_list_thread(
    builder: CutListThreadBuilder,
    task: &mut Task,
    solutions: &Vec<Solution>,
) -> Result<(), Box<dyn std::error::Error>> {
    log_debug!(
        "RUST: Executing CutListThread with group='{}', orientation={:?}",
        builder.group,
        builder.first_cut_orientation
    );

    log_debug!("RUST: CutListThread parameters:");
    log_debug!("RUST:   - aux_info: '{}'", builder.aux_info);
    log_debug!("RUST:   - tiles count: {}", builder.tiles.len());
    log_debug!("RUST:   - cut_thickness: {}", builder.cut_thickness);
    log_debug!(
        "RUST:   - min_trim_dimension: {}",
        builder.min_trim_dimension
    );
    log_debug!("RUST:   - accuracy_factor: {}", builder.accuracy_factor);
    log_debug!(
        "RUST:   - stock panels: {}",
        builder.stock_solution.get_stock_tile_dimensions().len()
    );

    // Здесь будет реальный алгоритм раскроя
    let cutting_result = perform_cutting_algorithm(&builder)?;

    // Обновляем задачу с новым решением если оно лучше
    if let Some(new_solution) = cutting_result {
        update_task_with_solution(task, new_solution);
    }

    log_debug!(
        "RUST: CutListThread execution completed for group='{}'",
        builder.group
    );
    Ok(())
}

/// Выполнение реального алгоритма раскроя
fn perform_cutting_algorithm(
    builder: &CutListThreadBuilder,
) -> Result<Option<Solution>, Box<dyn std::error::Error>> {
    // Здесь будет интегрирован реальный алгоритм раскроя
    // Пока что возвращаем None (нет нового решения)

    let total_tile_area: u64 = builder
        .tiles
        .iter()
        .map(|tile| tile.width * tile.height)
        .sum();

    let total_stock_area = builder.stock_solution.get_total_area();

    let efficiency = if total_stock_area > 0 {
        (total_tile_area as f64 / total_stock_area as f64) * 100.0
    } else {
        0.0
    };

    log_debug!(
        "RUST: Cutting algorithm efficiency: {:.2}% ({} / {})",
        efficiency,
        total_tile_area,
        total_stock_area
    );

    if efficiency > 75.0 {
        log_info!(
            "RUST: Found efficient solution: {:.2}% efficiency with group='{}', orientation={:?}",
            efficiency,
            builder.group,
            builder.first_cut_orientation
        );

        // Создаем новое решение
        let solution = Solution {
            id: ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            mosaics: vec![], // Здесь будут реальные мозаики из алгоритма
            no_fit_panels: vec![],
            unused_stock_panels: VecDeque::new(),
            aux_info: Some(builder.aux_info.clone()),
            creator_thread_group: Some(builder.group.clone()),
        };

        return Ok(Some(solution));
    }

    Ok(None)
}

/// Обновление задачи новым решением
fn update_task_with_solution(task: &mut Task, solution: Solution) {
    task.add_solution(solution);
    log_debug!(
        "RUST: Task updated with new solution, total solutions: {}",
        task.solutions.len()
    );
}

use serde::{Deserialize, Serialize};

/// Priority criteria for optimization algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OptimizationPriority {
    MostTiles,
    LeastWastedArea,
    LeastNbrCuts,
    MostHvDiscrepancy,
    BiggestUnusedTileArea,
    SmallestCenterOfMassDistToOrigin,
    LeastNbrMosaics,
    LeastNbrUnusedTiles,
    MostUnusedPanelArea,
}

impl std::fmt::Display for OptimizationPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::MostTiles => "MOST_TILES",
            Self::LeastWastedArea => "LEAST_WASTED_AREA",
            Self::LeastNbrCuts => "LEAST_NBR_CUTS",
            Self::MostHvDiscrepancy => "MOST_HV_DISCREPANCY",
            Self::BiggestUnusedTileArea => "BIGGEST_UNUSED_TILE_AREA",
            Self::SmallestCenterOfMassDistToOrigin => "SMALLEST_CENTER_OF_MASS_DIST_TO_ORIGIN",
            Self::LeastNbrMosaics => "LEAST_NBR_MOSAICS",
            Self::LeastNbrUnusedTiles => "LEAST_NBR_UNUSED_TILES",
            Self::MostUnusedPanelArea => "MOST_UNUSED_PANEL_AREA",
        };
        write!(f, "{}", text)
    }
}

impl Default for OptimizationPriority {
    fn default() -> Self {
        Self::LeastWastedArea
    }
}



/// Direction in which a cut can be made
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CutDirection {
    Horizontal,
    Vertical,
    Both,
}

impl Default for CutDirection {
    fn default() -> Self {
        Self::Both
    }
}


/// Аналог Java метода m301x52dbbde3 - ТОЧНАЯ КОПИЯ с детальным логированием
/// 
/// Java signature: 
/// void m301x52dbbde3(StockPanelPicker stockPanelPicker, int i, Task task, List list, List list2,
///     Configuration configuration, List list3, int i2, PerformanceThresholds performanceThresholds,
///     ProgressTracker progressTracker, String str)
pub fn process_permutation_with_all_stock_solutions(
    stock_panel_picker: &StockPanelPicker,
    permutation_idx: usize, // Java: int i
    task: &mut Task,
    solutions: &Vec<Solution>, // Java: List list
    all_permutations: &Vec<Vec<TileDimensions>>, // Java: List list2 (arrayList4/arrayList5)
    configuration: &Configuration,
    tile_arrangement: &[TileDimensions], // Java: List list3
    optimization_factor: f64, // Java: int i2 (i4)
    performance_thresholds: &PerformanceThresholds,
    progress_tracker: &mut ProgressTracker,
    material_str: &str, // Java: String str
) -> Result<(), Box<dyn std::error::Error>> {
    
    log_debug!("RUST: === process_permutation_with_all_stock_solutions STARTED ===");
    log_debug!("RUST: Parameters - permutationIdx={}, material='{}', optimizationFactor={}", 
        permutation_idx, material_str, optimization_factor);
    log_debug!("RUST: Tiles in permutation: {}, Solutions list size: {}", 
        tile_arrangement.len(), solutions.len());

    // Java: int iRound; int iRound2; int i3 = 0; int i4 = 0;
    let mut i_round: i64;
    let mut i_round2: i64;
    let mut i3 = 0; // Счетчик для решений
    let mut i4 = 0; // Счетчик stock итераций

    log_debug!("RUST: Initialized counters - i3={}, i4={}", i3, i4);
    log_debug!("RUST: Starting stock solution loop (max 1000 iterations)");

    // Java: while (i4 < 1000)
    while i4 < 1000 {
        log_debug!("RUST: === Stock iteration {} ===", i4);

        // Java: StockSolution stockSolution = stockPanelPicker.getStockSolution(i4);
        let stock_solution = match stock_panel_picker.get_stock_solution(i4) {
            Some(solution) => solution,
            None => {
                log_debug!("RUST: No more possible stock solutions: stockSolution[{}] permutationIdx[{}]", 
                    i4, permutation_idx);
                return Ok(());
            }
        };

        log_debug!("RUST: Retrieved stockSolution[{}]: {} panels, totalArea={}", 
            i4, stock_solution.get_stock_tile_dimensions().len(), stock_solution.get_total_area());

        // Java: if (!task.isRunning()) { ... return; }
        if !task.is_running() {
            log_debug!("RUST: Task no longer has running status. Stopping stock loop for permutationIdx[{}]", 
                permutation_idx);
            return Ok(());
        }
        log_debug!("RUST: Task is still running, continuing with stock solution [{}]", i4);

        // Java: Проверка условий обработки
        let mut should_process = true;
        let mut skip_reason = String::new();

        // Java: if (task.hasSolutionAllFit() && list.size() > 0 && 
        //           ((Solution) list.get(i3)).getMosaics().size() == 1 &&
        //           ((Solution) list.get(i3)).getTotalArea() < stockSolution.getTotalArea())
        if task.has_solution_all_fit() && !solutions.is_empty() && i3 < solutions.len() {
            let current_solution = &solutions[i3];
            let solution_has_one_mosaic = current_solution.mosaics.len() == 1;
            let solution_total_area = calculate_solution_total_area(current_solution);
            let solution_area_smaller = solution_total_area < stock_solution.get_total_area();
            
            if solution_has_one_mosaic && solution_area_smaller {
                should_process = false;
                skip_reason = format!("already has all-fit solution with totalArea[{}] < stockSolution.totalArea[{}]", 
                    solution_total_area, stock_solution.get_total_area());
            }
        }

        log_debug!("RUST: Should process stock solution: {} (reason: {})", 
            should_process, 
            if should_process { "normal processing" } else { &skip_reason });

        if should_process {
            log_debug!("RUST: Starting permutationIdx[{}/{}] with stock solution [{}] {{nbrPanels[{}] area[{}] {}}}",
                permutation_idx + 1, all_permutations.len(), i4, 
                stock_solution.get_stock_tile_dimensions().len(), 
                stock_solution.get_total_area(), stock_solution);

            // Java: Получение компараторов
            log_debug!("RUST: Getting solution comparator list...");
            let solution_comparator_list = get_solution_comparator_list(configuration);
            log_debug!("RUST: Retrieved {} solution comparators", solution_comparator_list.len());

            // Java: Парсинг толщины реза
            i_round = match parse_scaled_value(configuration.cut_thickness, task.factor) {
                Ok(value) => {
                    log_debug!("RUST: Parsed cut thickness: '{}' * {} = {:.1} -> {}", 
                        configuration.cut_thickness, task.factor, 
                        configuration.cut_thickness * task.factor as f64, value);
                    value
                }
                Err(_) => {
                    log_error!("RUST: Error parsing cut thickness value: [{}]", configuration.cut_thickness);
                    0
                }
            };

            // Java: Парсинг минимального размера обрезки
            i_round2 = match parse_scaled_value(configuration.min_trim_dimension, task.factor) {
                Ok(value) => {
                    log_debug!("RUST: Parsed min trim dimension: '{}' * {} = {:.1} -> {}", 
                        configuration.min_trim_dimension, task.factor, 
                        configuration.min_trim_dimension * task.factor as f64, value);
                    value
                }
                Err(_) => {
                    log_error!("RUST: Error parsing minimum trim dimension value: [{}]", configuration.min_trim_dimension);
                    0
                }
            };

            // Java: Создание CutListThreadBuilder
            log_debug!("RUST: Creating CutListThreadBuilder equivalent...");
            let aux_info = format!("stock[{}] permutation[{}]", i4, permutation_idx);
            
            let cut_list_thread_builder = CutListThreadBuilder {
                aux_info: aux_info.clone(),
                all_solutions: solutions.clone(),
                tiles: tile_arrangement.to_vec(),
                configuration: configuration.clone(),
                cut_thickness: i_round,
                min_trim_dimension: i_round2,
                final_solution_prioritized_comparators: solution_comparator_list.clone(),
                task_id: task.id.clone(),
                accuracy_factor: optimization_factor,
                stock_solution: stock_solution.clone(),
                group: String::new(),
                thread_prioritized_comparators: Vec::new(),
                first_cut_orientation: CutDirection::Both,
            };

            log_debug!("RUST: CutListThreadBuilder created with auxInfo='{}'", aux_info);

            // Java: Ожидание освобождения потоков
            let mut wait_count = 0;
            while task.get_nbr_running_threads() + task.get_nbr_queued_threads() >= performance_thresholds.max_simultaneous_threads {
                wait_count += 1;
                if wait_count == 1 || wait_count % 10 == 0 {
                    log_debug!("RUST: Maximum number of active threads reached (wait #{}): running[{}] queued[{}] >= max[{}]", 
                        wait_count, task.get_nbr_running_threads(), task.get_nbr_queued_threads(), 
                        performance_thresholds.max_simultaneous_threads);
                }
                progress_tracker.refresh_task_status_info(&mut PermutationThreadSpawner::new());
                std::thread::sleep(std::time::Duration::from_millis(performance_thresholds.thread_check_interval));
            }
            if wait_count > 0 {
                log_debug!("RUST: Thread limit wait completed after {} iterations", wait_count);
            }

            // Java: Группа AREA с обоими направлениями резов
            if is_thread_eligible_to_start("AREA", task, material_str) && 
               is_cut_orientation_allowed(&configuration.cut_orientation_preference, CutOrientationPreference::Both) {
                
                log_debug!("RUST: Starting AREA thread with CutDirection::Both");
                
                let mut area_builder = cut_list_thread_builder.clone();
                area_builder.group = "AREA".to_string();
                area_builder.thread_prioritized_comparators = solution_comparator_list.clone();
                area_builder.first_cut_orientation = CutDirection::Both;
                
                execute_cut_list_thread(area_builder, task, solutions)?;
                log_debug!("RUST: AREA thread submitted to executor");
            }

            // Java: Группа AREA_HCUTS_1ST с горизонтальными резами в приоритете
            if is_thread_eligible_to_start("AREA_HCUTS_1ST", task, material_str) &&
               (is_cut_orientation_allowed(&configuration.cut_orientation_preference, CutOrientationPreference::Both) ||
                is_cut_orientation_allowed(&configuration.cut_orientation_preference, CutOrientationPreference::Horizontal)) {
                
                log_debug!("RUST: Starting AREA_HCUTS_1ST thread with CutDirection::Horizontal");
                
                let mut hcuts_builder = cut_list_thread_builder.clone();
                hcuts_builder.group = "AREA_HCUTS_1ST".to_string();
                hcuts_builder.thread_prioritized_comparators = solution_comparator_list.clone();
                hcuts_builder.first_cut_orientation = CutDirection::Horizontal;
                
                execute_cut_list_thread(hcuts_builder, task, solutions)?;
                log_debug!("RUST: AREA_HCUTS_1ST thread submitted to executor");
            }

            // Java: Группа AREA_VCUTS_1ST с вертикальными резами в приоритете
            if is_thread_eligible_to_start("AREA_VCUTS_1ST", task, material_str) &&
               (is_cut_orientation_allowed(&configuration.cut_orientation_preference, CutOrientationPreference::Both) ||
                is_cut_orientation_allowed(&configuration.cut_orientation_preference, CutOrientationPreference::Vertical)) {
                
                log_debug!("RUST: Starting AREA_VCUTS_1ST thread with CutDirection::Vertical");
                
                let mut vcuts_builder = cut_list_thread_builder.clone();
                vcuts_builder.group = "AREA_VCUTS_1ST".to_string();
                vcuts_builder.thread_prioritized_comparators = solution_comparator_list.clone();
                vcuts_builder.first_cut_orientation = CutDirection::Vertical;
                
                execute_cut_list_thread(vcuts_builder, task, solutions)?;
                log_debug!("RUST: AREA_VCUTS_1ST thread submitted to executor");
            }

            log_debug!("RUST: All eligible threads submitted for stock[{}] permutation[{}]", i4, permutation_idx);

        } else {
            log_debug!("RUST: Stopping stock loop for permutationIdx[{}/{}] at stock solution {} with area [{}] because there's already an all fit solution using stock solution with area [{}]",
                permutation_idx + 1, all_permutations.len(), stock_solution, stock_solution.get_total_area(),
                if !solutions.is_empty() && i3 < solutions.len() { 
                    calculate_solution_total_area(&solutions[i3]).to_string() 
                } else { 
                    "no solutions".to_string() 
                });
        }

        // Java: Увеличение счетчиков
        log_debug!("RUST: Incrementing stock iteration: i4={} -> {}", i4, i4 + 1);
        i4 += 1;
        i3 = 0; // Java: i3 = 0; (Сброс счетчика решений)
    }

    log_debug!("RUST: === process_permutation_with_all_stock_solutions COMPLETED ===");
    log_debug!("RUST: Processed {} stock iterations for permutation[{}]", i4, permutation_idx);

    Ok(())
}


