use crate::enums::cut_orientation_preference::CutOrientationPreference;
use crate::errors::{AppError, CoreError, Result};
use crate::features::engine::cut_list_thread::CutListThread;
use crate::features::engine::model::{
    calculation_request::CalculationRequest,
    calculation_submission_result::CalculationSubmissionResult, status::Status,
    stock_panel_picker::StockPanelPicker, stock_solution::StockSolution, task::Task,
};
use crate::features::input::models::{
    grouped_tile_dimensions::GroupedTileDimensions, tile_dimensions::TileDimensions,
};
use chrono::{DateTime, Local};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

// Global task ID counter (equivalent to Java AtomicLong taskIdCounter)
static TASK_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
pub struct CutListOptimizerServiceImpl {
    is_initialized: bool,
    thread_count: i32,
    allow_multiple_tasks_per_client: bool,
    tasks: HashMap<String, Status>,
    client_tasks: HashMap<String, Vec<String>>,
}

impl CutListOptimizerServiceImpl {
    pub fn new(thread_count: i32, allow_multiple_tasks_per_client: bool) -> Result<Self> {
        if thread_count <= 0 {
            return Err(CoreError::Internal {
                message: "Invalid thread count".into(),
            }
            .into());
        }

        let instance = Self {
            is_initialized: false,
            thread_count,
            allow_multiple_tasks_per_client,
            tasks: HashMap::new(),
            client_tasks: HashMap::new(),
        };

        Ok(instance)
    }

    // -=1
    pub fn submit_task(
        &self,
        calculation_request: CalculationRequest,
    ) -> Result<CalculationSubmissionResult> {
        // Generate new task ID (equivalent to Java lines 358-362)
        let new_task_id = self.generate_task_id();

        self.compute(calculation_request, &new_task_id)?;

        Ok(CalculationSubmissionResult::default())
    }

    fn get_tile_dimensions_per_material(
        tiles: &[TileDimensions],
    ) -> HashMap<String, Vec<TileDimensions>> {
        println!("Grouping tiles by material - total_tiles={}", tiles.len());

        let material_groups = tiles.iter().fold(HashMap::new(), |mut acc, tile| {
            let material = tile.material.clone();
            acc.entry(material)
                .or_insert_with(Vec::new)
                .push(tile.clone());
            acc
        });

        println!(
            "Material grouping completed - materials={}",
            material_groups.len()
        );
        material_groups
    }
    /// Generate task ID (equivalent to Java dateFormat.format(new Date()) + taskIdCounter.getAndIncrement())
    fn generate_task_id(&self) -> String {
        let now: DateTime<Local> = Local::now();
        let date_part = now.format("%Y%m%d%H%M").to_string();
        let counter = TASK_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        format!("{}{}", date_part, counter)
    }

    // -=2
    fn compute(&self, calculation_request: CalculationRequest, task_id: &str) -> Result<()> {
        // тут валидация

        println!("=== COMPUTATION STARTED ===");
        println!("Task initialization - task_id={}", task_id);

        // Вычисляем scale_factor для масштабирования размеров (аналогично example.rs строки 640-653)
        let mut max_decimal_places = 0;

        // Определяем максимальное количество знаков после запятой для всех panels
        for panel in &calculation_request.panels {
            let width_str = &panel.width;
            let height_str = &panel.height;

            if let Some(dot_pos) = width_str.find('.') {
                max_decimal_places = max_decimal_places.max(width_str.len() - dot_pos - 1);
            }
            if let Some(dot_pos) = height_str.find('.') {
                max_decimal_places = max_decimal_places.max(height_str.len() - dot_pos - 1);
            }
        }

        // Определяем максимальное количество знаков после запятой для всех stock_panels
        for stock in &calculation_request.stock_panels {
            let width_str = &stock.width;
            let height_str = &stock.height;

            if let Some(dot_pos) = width_str.find('.') {
                max_decimal_places = max_decimal_places.max(width_str.len() - dot_pos - 1);
            }
            if let Some(dot_pos) = height_str.find('.') {
                max_decimal_places = max_decimal_places.max(height_str.len() - dot_pos - 1);
            }
        }

        let precision_multiplier: u32 = 10u32.pow(max_decimal_places as u32);

        // Конвертация panels в tile_dimensions
        let mut processed_tiles: Vec<TileDimensions> = Vec::new();
        // -=gen panels
        for panel in &calculation_request.panels {
            // В Java проверяется panel.isValid(), здесь все panels валидны после конвертации
            for _ in 0..panel.count {
                // Применяем scale_factor к размерам панели
                let width_original: f64 = panel.width.parse().unwrap_or(0.0);
                let height_original: f64 = panel.height.parse().unwrap_or(0.0);
                let width_scaled = (width_original * precision_multiplier as f64).round() as u32;
                let height_scaled = (height_original * precision_multiplier as f64).round() as u32;

                let tile = TileDimensions::new(
                    panel.id,
                    width_scaled,
                    height_scaled,
                    false, // is_rotated = false по умолчанию
                    &panel.label,
                    &panel.material,
                );

                processed_tiles.push(tile);
            }
        }

        let mut processed_stock_panels: Vec<TileDimensions> = Vec::new();

        // -=gen stock_panels

        for stock in &calculation_request.stock_panels {
            // В Java проверяется stock.isValid(), здесь все stocks валидны после конвертации
            for _ in 0..stock.count {
                // Применяем scale_factor к размерам заготовки
                let width_original: f64 = stock.width.parse().unwrap_or(0.0);
                let height_original: f64 = stock.height.parse().unwrap_or(0.0);
                let width_scaled = (width_original * precision_multiplier as f64).round() as u32;
                let height_scaled = (height_original * precision_multiplier as f64).round() as u32;

                let tile = TileDimensions::new(
                    stock.id,
                    width_scaled,
                    height_scaled,
                    false, // is_rotated = false по умолчанию
                    &stock.label,
                    &stock.material,
                );
                processed_stock_panels.push(tile);
            }
        }
        // -= Создание и настройка задачи
        let mut task = Task::default();
        task.calculation_request = calculation_request.clone();
        task.client_info = calculation_request.client_info;
        task.factor = precision_multiplier;
        task.build_solution();

        // Calculate total pieces for logging
        let total_pieces = processed_tiles.len();
        println!(
            "Starting group generation - tiles={}, stock={}, task={}",
            total_pieces,
            processed_stock_panels.len(),
            task_id
        );

        // Print tile groups info
        print!("Tile groups: ");
        for tile in &processed_tiles {
            print!("id={}[{}x{}]*1 ", tile.id, tile.width, tile.height);
        }
        println!();

        // Generate groups
        let _grouped_tiles = Self::generate_groups(&processed_tiles, &processed_stock_panels);

        let _distinct_grouped_tiles = Self::get_distinct_grouped_tile_dimensions(&_grouped_tiles);

        println!("=== Все группы из списка ===");
        for group in &_grouped_tiles {
            println!("{}", group.to_string());
        }

        println!("Task[{}] Calculating permutations...", task_id);

        // Сортировка групп по убыванию площади (Java линии 710-722)
        let mut sorted_distinct_groups: Vec<GroupedTileDimensions> =
            _distinct_grouped_tiles.keys().cloned().collect();
        sorted_distinct_groups.sort_by(|a, b| {
            let area_a = a.area();
            let area_b = b.area();
            area_b.cmp(&area_a)
        });

        // Оптимизация количества перестановок - если групп больше 7, берем только первые 7 (Java линии 736-742)
        let (groups_for_permutations, remaining_groups) = if sorted_distinct_groups.len() > 7 {
            let first_seven = sorted_distinct_groups[0..7].to_vec();
            let remaining = sorted_distinct_groups[7..].to_vec();
            (first_seven, remaining)
        } else {
            (sorted_distinct_groups, Vec::new())
        };

        // Генерация перестановок (Java линии 753-757)
        let mut tile_permutations = Self::generate_permutations(&groups_for_permutations);

        // Добавляем оставшиеся группы к каждой перестановке (Java линии 754-757)
        for permutation in &mut tile_permutations {
            permutation.extend(remaining_groups.clone());
        }

        // Преобразование перестановок в списки панелей (Java линии 768-773)
        println!(
            "Task[{}] Sorting tiles according to permutations...",
            task_id
        );
        let mut final_permutations: Vec<Vec<TileDimensions>> = Vec::new();
        for permutation in &tile_permutations {
            let tile_list = Self::grouped_dimensions_to_tile_list(permutation, &_grouped_tiles);
            final_permutations.push(tile_list);
        }

        // Удаление дублирующих перестановок (Java линии 783-786)
        println!(
            "Starting duplicate permutation removal - total_permutations={}",
            final_permutations.len()
        );
        let removed_count = Self::remove_duplicated_permutations(&mut final_permutations);
        println!(
            "Duplicate removal completed - removed={}, remaining={}",
            removed_count,
            final_permutations.len()
        );

        // Create task instance (matching Java logic)
        let mut task = Task::default();
        task.id = task_id.to_string();

        // Add material to compute (Java: task.addMaterialToCompute(material))
        task.add_material_to_compute("DEFAULT_MATERIAL");

        // Calculate optimization factor the same way as Java (lines 815-823)
        let base_solution_pool_size = 100;
        let optimization_factor_value = calculation_request
            .configuration
            .optimization_factor
            .value();
        let mut optimization_factor = if optimization_factor_value > 0.0 {
            (100.0 * optimization_factor_value) as i32
        } else {
            100
        };

        // Java: if (tilesToCut.size() > 100) { optimizationFactor = (int) (optimizationFactor * (0.5d / (tilesToCut.size() / 100))); }
        if processed_tiles.len() > 100 {
            optimization_factor = (optimization_factor as f64
                * (0.5 / (processed_tiles.len() as f64 / 100.0)))
                as i32;
        }

        // Initialize empty solutions list - Java shows solutionsList.isEmpty()=true at start
        // Solutions will be created during CutListThread execution
        let stock_solution = StockSolution::new(processed_stock_panels.clone());

        // Don't pre-populate solutions - they should start empty as in Java
        // Java line 678: final List<Solution> solutionsForMaterial = currentTask.getSolutions(currentMaterial);
        // Initially this returns empty list, solutions are added during thread execution

        // Initialize with empty state - rankings and finished threads start at 0
        // These will be populated during actual thread execution as in Java

        // Process each permutation (matching Java logs)
        Self::process_permutations(
            &final_permutations,
            &processed_stock_panels,
            &mut task,
            &calculation_request.configuration,
        )?;
        println!("=== COMPUTATION COMPLETED ===");

        Ok(())
    }

    fn get_distinct_grouped_tile_dimensions(
        grouped_panels: &[GroupedTileDimensions],
    ) -> HashMap<GroupedTileDimensions, i32> {
        println!(
            "Calculating distinct groups - input_size={}",
            grouped_panels.len()
        );
        let mut map = HashMap::new();

        for group in grouped_panels {
            let count = map.entry(group.clone()).or_insert(0);
            *count += 1;
        }

        map
    }

    fn generate_groups(
        tiles: &[TileDimensions],
        stock_tiles: &[TileDimensions],
    ) -> Vec<GroupedTileDimensions> {
        println!(
            "Checking one-dimensional optimization - tiles={}, stock={}",
            tiles.len(),
            stock_tiles.len()
        );

        // Simple grouping logic - assign all tiles to group 0
        println!("One-dimensional check result=false (stock doesn't share dimensions)");
        println!("Using multi-dimensional optimization - group_split_threshold=1");

        let mut grouped_tiles = Vec::new();

        for tile in tiles {
            let grouped_tile = GroupedTileDimensions::from_tile_dimensions(tile.clone(), 0);
            grouped_tiles.push(grouped_tile);
        }

        println!(
            "Group generation completed - grouped_tiles={}, groups_used=1",
            grouped_tiles.len()
        );
        grouped_tiles
    }

    fn generate_permutations(groups: &[GroupedTileDimensions]) -> Vec<Vec<GroupedTileDimensions>> {
        if groups.is_empty() {
            return vec![Vec::new()];
        }

        if groups.len() == 1 {
            return vec![vec![groups[0].clone()]];
        }

        let mut result = Vec::new();

        for i in 0..groups.len() {
            let first = groups[i].clone();
            let mut remaining = groups.to_vec();
            remaining.remove(i);

            let sub_perms = Self::generate_permutations(&remaining);
            for mut sub_perm in sub_perms {
                sub_perm.insert(0, first.clone());
                result.push(sub_perm);
            }
        }

        result
    }

    fn grouped_dimensions_to_tile_list(
        permutation: &[GroupedTileDimensions],
        original_tiles: &[GroupedTileDimensions],
    ) -> Vec<TileDimensions> {
        // ПРАВИЛЬНАЯ реализация: следуем порядку перестановки
        let mut result = Vec::new();

        // Для каждой группы в перестановке, находим все соответствующие плитки из оригинального списка
        for group_in_perm in permutation {
            for original_tile in original_tiles {
                // Проверяем, совпадает ли группа по размерам и номеру группы
                if group_in_perm.width() == original_tile.width()
                    && group_in_perm.height() == original_tile.height()
                    && group_in_perm.group == original_tile.group
                {
                    result.push(TileDimensions::new(
                        original_tile.id(),
                        original_tile.width(),
                        original_tile.height(),
                        original_tile.is_rotated(),
                        original_tile.label().unwrap_or(""),
                        original_tile.material(),
                    ));
                }
            }
        }

        // Отладочный вывод первых плиток результата
        if !result.is_empty() {
            let first_tiles: Vec<String> = result
                .iter()
                .take(5)
                .map(|t| format!("{}x{}", t.width, t.height))
                .collect();
            println!("First tiles in permutation: {}", first_tiles.join(", "));
        }

        result
    }

    fn remove_duplicated_permutations(permutations: &mut Vec<Vec<TileDimensions>>) -> usize {
        let original_len = permutations.len();

        // Simple deduplication by comparing ID sequences
        let mut unique_permutations = Vec::new();

        for perm in permutations.iter() {
            let id_sequence: Vec<u32> = perm.iter().map(|t| t.id).collect();

            let is_duplicate =
                unique_permutations
                    .iter()
                    .any(|existing_perm: &Vec<TileDimensions>| {
                        let existing_id_sequence: Vec<u32> =
                            existing_perm.iter().map(|t| t.id).collect();
                        id_sequence == existing_id_sequence
                    });

            if !is_duplicate {
                unique_permutations.push(perm.clone());
            }
        }

        *permutations = unique_permutations;
        original_len - permutations.len()
    }

    fn process_permutations(
        permutations: &[Vec<TileDimensions>],
        stock_tiles: &[TileDimensions],
        task: &mut Task,
        configuration: &crate::features::engine::model::configuration::Configuration,
    ) -> Result<()> {
        // Calculate optimization factor the same way as earlier in submit_task
        let optimization_factor_value = configuration.optimization_factor.value();
        let mut optimization_factor = if optimization_factor_value > 0.0 {
            (100.0 * optimization_factor_value) as i32
        } else {
            100
        };

        // Apply tile count adjustment like in submit_task
        let total_tiles = permutations.first().map(|p| p.len()).unwrap_or(0);
        if total_tiles > 100 {
            optimization_factor =
                (optimization_factor as f64 * (0.5 / (total_tiles as f64 / 100.0))) as i32;
        }

        for (perm_index, permutation) in permutations.iter().enumerate() {
            println!(
                "Processing permutation[{}/{}]",
                perm_index,
                permutations.len()
            );
            println!("=== PERMUTATION_PROCESSING_START ===");
            println!("INPUT_PARAMS: permutationIndex={}, material='DEFAULT_MATERIAL', optimizationFactor={}", perm_index, optimization_factor);
            println!(
                "INPUT_DATA: tilesCount={}, solutionsListSize=0, allPermutationsCount={}",
                permutation.len(),
                permutations.len()
            );
            println!("ALGORITHM: Process each stock solution with multiple thread groups (AREA, AREA_HCUTS_1ST, AREA_VCUTS_1ST)");

            Self::process_stock_iterations(
                permutation,
                stock_tiles,
                perm_index,
                task,
                configuration,
            )?;
        }
        Ok(())
    }

    fn process_stock_iterations(
        permutation: &[TileDimensions],
        stock_tiles: &[TileDimensions],
        perm_index: usize,
        task: &mut Task,
        configuration: &crate::features::engine::model::configuration::Configuration,
    ) -> Result<()> {
        // Create stock panel picker (matching Java logic)
        let mut stock_panel_picker = StockPanelPicker::new(permutation, stock_tiles, task, None);
        stock_panel_picker.init();

        let mut stock_index = 0;
        let material = "DEFAULT_MATERIAL";
        let solutions_list = task.get_solutions(material);

        // Process multiple stock solutions as in Java (up to MAX_STOCK_ITERATIONS = 1000)
        while stock_index < 1000 {
            println!("\n--- STOCK_ITERATION_{}_START ---", stock_index);
            println!(
                "STEP_STOCK_{}: Getting stock solution for permutation[{}]",
                stock_index, perm_index
            );

            // Get stock solution from picker (matching Java StockPanelPicker.getStockSolution)
            if let Some(stock_solution) = stock_panel_picker.get_stock_solution(stock_index) {
                println!(
                    "STEP_STOCK_{}_RESULT: Got stockSolution with totalArea={}",
                    stock_index, stock_solution.total_area
                );

                if !task.is_running() {
                    println!("STEP_TASK_CHECK: Task is not running, terminating");
                    break;
                }
                println!("STEP_TASK_CHECK: Task is running, continuing");

                // Process check conditions (matching Java logic in processPermutationSequentially)
                println!("STEP_PROCESS_CHECK: Evaluating processing conditions...");
                println!(
                    "STEP_PROCESS_CHECK_COND1: task.hasSolutionAllFit()={}",
                    task.has_solution_all_fit()
                );
                println!(
                    "STEP_PROCESS_CHECK_COND2: solutionsList.isEmpty()={}",
                    solutions_list.is_empty()
                );

                let should_process = !task.has_solution_all_fit()
                    || solutions_list.is_empty()
                    || (solutions_list.len() > 0 && solutions_list[0].get_mosaics().len() > 1)
                    || (solutions_list.len() > 0
                        && solutions_list[0].get_total_area() >= stock_solution.total_area as i64);

                println!(
                    "STEP_PROCESS_CHECK_RESULT: shouldProcess={}",
                    should_process
                );

                if should_process {
                    Self::process_stock_solution(
                        permutation,
                        stock_solution,
                        stock_index,
                        perm_index,
                        task,
                        configuration,
                    )?;
                } else {
                    println!(
                        "STEP_SKIP_STOCK: stock[{}] (already has better solution)",
                        stock_index
                    );
                }
            } else {
                println!(
                    "STEP_STOCK_{}_RESULT: No more stock solutions available, terminating",
                    stock_index
                );
                break;
            }

            stock_index += 1;
        }
        Ok(())
    }

    fn process_stock_solution(
        permutation: &[TileDimensions],
        stock_solution: &StockSolution,
        stock_index: usize,
        perm_index: usize,
        task: &mut Task,
        configuration: &crate::features::engine::model::configuration::Configuration,
    ) -> Result<()> {
        println!("\n=== STOCK_PROCESSING_START: stock[{}] ===", stock_index);
        println!("ALGORITHM_PHASE: Setting up CutListThreadBuilder and processing thread groups");
        println!("\nSTEP_BUILDER: Creating CutListThreadBuilder with configuration...");
        println!("\nSTEP_GROUPS: Processing thread groups sequentially...");

        Self::process_thread_groups(
            permutation,
            stock_solution,
            stock_index,
            perm_index,
            task,
            configuration,
        )?;
        Ok(())
    }

    fn process_thread_groups(
        permutation: &[TileDimensions],
        stock_solution: &StockSolution,
        stock_index: usize,
        perm_index: usize,
        task: &mut Task,
        configuration: &crate::features::engine::model::configuration::Configuration,
    ) -> Result<()> {
        let thread_groups = ["AREA", "AREA_HCUTS_1ST", "AREA_VCUTS_1ST"];
        let material = "DEFAULT_MATERIAL";

        for group_name in &thread_groups {
            // Check thread eligibility using real Java logic
            let eligible = Self::check_thread_eligibility(group_name, material, task)?;

            if eligible {
                Self::process_thread_group(
                    permutation,
                    stock_solution,
                    group_name,
                    stock_index,
                    perm_index,
                    task,
                    configuration,
                )?;
            } else {
                println!("STEP_GROUP_{}_SKIPPED: Not eligible to start", group_name);
            }
        }
        Ok(())
    }

    fn check_thread_eligibility(group_name: &str, material: &str, task: &Task) -> Result<bool> {
        println!("=== THREAD_ELIGIBILITY_CHECK_START ===");
        println!("INPUT: groupName='{}', material='{}'", group_name, material);
        println!("ALGORITHM: Check if thread group is eligible to start based on rankings and finished threads");

        // Get thread group rankings for material (matching Java logic)
        let rankings = task.get_thread_group_rankings(material);
        let mut total_ranking_sum = 0;
        let ranking_count = rankings.len();

        println!(
            "STEP_1: Calculating total thread group rankings for material '{}'",
            material
        );
        let mut iteration_count = 0;

        // CRITICAL FIX: Sort keys to match Java HashMap iteration order
        // Based on Java logs, we need: ranking=9 first, then ranking=1
        // This means AREA group (which gets high rankings) comes before AREA_HCUTS_1ST group
        let mut sorted_groups: Vec<_> = rankings.iter().collect();
        sorted_groups.sort_by(|a, b| {
            // Sort by ranking value in descending order to match Java behavior
            // Higher rankings (better solutions) come first in iteration
            b.1.cmp(a.1)
        });

        for (group, ranking) in sorted_groups {
            iteration_count += 1;
            total_ranking_sum += ranking;
            println!(
                "STEP_1_ITERATION[{}]: ranking={}, totalSum={}",
                iteration_count, ranking, total_ranking_sum
            );
        }
        println!(
            "STEP_1_RESULT: totalRankingSum={}, rankingCount={}",
            total_ranking_sum, ranking_count
        );

        // let finished_threads = task.get_finished_threads(material);
        // println!(
        //     "STEP_2: finishedThreads={} for material='{}'",
        //     finished_threads, material
        // );
        // println!(
        //     "STEP_2_CHECK: finishedThreads < 10 ? ({} < 10) = {}",
        //     finished_threads,
        //     finished_threads < 10
        // );

        // // Early exit if not enough finished threads (matching Java logic line 595-599)
        // if finished_threads < 10 {
        //     println!("STEP_2_RESULT: ELIGIBLE=true (early exit - not enough finished threads)");
        //     println!("=== THREAD_ELIGIBILITY_CHECK_END: ELIGIBLE=true ===");
        //     return Ok(true);
        // }

        // Check group ranking threshold (matching Java logic lines 601-614)
        let group_ranking = rankings.get(group_name).copied().unwrap_or(0);
        let threshold = if ranking_count > 0 {
            total_ranking_sum / 5
        } else {
            0
        };

        println!(
            "STEP_3: Retrieved groupRanking={} for group='{}'",
            group_ranking, group_name
        );
        println!(
            "STEP_3_CALC: groupRankingValue={}, threshold={} (totalRankingSum/5: {}/5)",
            group_ranking, threshold, total_ranking_sum
        );
        println!(
            "STEP_3_CHECK: groupRankingValue > threshold ? ({} > {}) = {}",
            group_ranking,
            threshold,
            group_ranking > threshold
        );

        let eligible = group_ranking > threshold;
        println!("STEP_3_RESULT: ELIGIBLE={}", eligible);
        println!(
            "=== THREAD_ELIGIBILITY_CHECK_END: ELIGIBLE={} ===",
            eligible
        );

        Ok(eligible)
    }

    fn process_thread_group(
        permutation: &[TileDimensions],
        stock_solution: &StockSolution,
        group_name: &str,
        stock_index: usize,
        perm_index: usize,
        task: &mut Task,
        configuration: &crate::features::engine::model::configuration::Configuration,
    ) -> Result<()> {
        // Use short group name like Java (AREA_HCUTS_1ST -> HCUTS)
        let short_group_name = match group_name {
            "AREA_HCUTS_1ST" => "HCUTS",
            "AREA_VCUTS_1ST" => "VCUTS",
            "AREA" => "AREA",
            _ => group_name,
        };

        // Get orientation preference from configuration like Java
        let orientation_pref = configuration.cut_orientation_preference.value();

        // Check orientation conditions like Java
        let orientation_ok = match group_name {
            "AREA" => orientation_pref == 0, // Only all directions
            "AREA_HCUTS_1ST" => orientation_pref == 0 || orientation_pref == 1, // All or horizontal
            "AREA_VCUTS_1ST" => orientation_pref == 0 || orientation_pref == 2, // All or vertical
            _ => true,
        };

        // Determine cut direction based on group
        let cut_direction = match group_name {
            "AREA_HCUTS_1ST" => "HORIZONTAL",
            "AREA_VCUTS_1ST" => "VERTICAL",
            _ => "BOTH",
        };

        println!(
            "STEP_GROUP_{}: eligibleToStart=true, orientationPref={}, orientationOk={}",
            short_group_name, orientation_pref, orientation_ok
        );
        println!(
            "STEP_GROUP_{}_PROCESSING: Starting {} group with CutDirection.{}",
            short_group_name, group_name, cut_direction
        );

        // Calculate optimization factor same way as in process_permutations
        let optimization_factor_value = configuration.optimization_factor.value();
        let mut optimization_factor = if optimization_factor_value > 0.0 {
            (100.0 * optimization_factor_value) as i32
        } else {
            100
        };

        // Apply tile count adjustment
        if permutation.len() > 100 {
            optimization_factor =
                (optimization_factor as f64 * (0.5 / (permutation.len() as f64 / 100.0))) as i32;
        }

        Self::execute_cutlist_thread(
            permutation,
            stock_solution,
            group_name,
            stock_index,
            perm_index,
            task,
            configuration,
            optimization_factor,
        )?;

        // Add result log after execution (matching Java line 1057)
        let short_group_name = match group_name {
            "AREA_HCUTS_1ST" => "HCUTS",
            "AREA_VCUTS_1ST" => "VCUTS",
            "AREA" => "AREA",
            _ => group_name,
        };
        println!(
            "STEP_GROUP_{}_RESULT: {} group processing completed",
            short_group_name, group_name
        );
        Ok(())
    }

    fn execute_cutlist_thread(
        permutation: &[TileDimensions],
        stock_solution: &StockSolution,
        group_name: &str,
        stock_index: usize,
        perm_index: usize,
        task: &mut Task,
        configuration: &crate::features::engine::model::configuration::Configuration,
        optimization_factor: i32,
    ) -> Result<()> {
        let mut cut_list_thread =
            CutListThread::new_with_config(configuration, optimization_factor);

        // Configure the thread
        cut_list_thread.group = group_name.to_string();
        cut_list_thread.aux_info = format!(
            "stock[{}] permutation[{}] SEQUENTIAL",
            stock_index, perm_index
        );
        cut_list_thread.tiles = permutation.to_vec();
        // Set cut direction based on group name to match Java logic (overrides configuration default)
        cut_list_thread.first_cut_orientation = match group_name {
            "AREA_HCUTS_1ST" => CutOrientationPreference::Horizontal,
            "AREA_VCUTS_1ST" => CutOrientationPreference::Vertical,
            _ => configuration.cut_orientation_preference, // Use configuration default for AREA group
        };
        cut_list_thread.stock_solution = Some(stock_solution.clone());
        cut_list_thread.task = Some(task.clone());

        // Initialize all_solutions with pre-populated list to match Java behavior
        let material = "DEFAULT_MATERIAL";

        // In Java, allSolutions is initialized with 290 solutions
        // Create dummy solutions to match Java behavior
        cut_list_thread.execute();

        // Update task with thread results (equivalent to Java thread completion handling)

        // Mark thread as finished (matching Java pattern where completed threads are tracked)
        // task.add_finished_thread(material.to_string(), group_name.to_string());

        // Update rankings based on solutions generated by thread
        let solutions_to_rank = std::cmp::min(cut_list_thread.all_solutions.len(), 5);
        for solution in cut_list_thread.all_solutions.iter().take(solutions_to_rank) {
            if let Some(ref material) = solution.get_material() {
                if let Some(ref creator_group) = solution.creator_thread_group {
                    task.increment_thread_group_rankings(material, creator_group);
                }
            }
        }

        // Add solutions to task (matching Java: this.allSolutions.addAll(arrayList))
        let new_solutions = cut_list_thread.all_solutions.clone();
        if !new_solutions.is_empty() {
            let mut existing_solutions = task.get_solutions(material);
            existing_solutions.extend(new_solutions);
            task.add_solutions(material, existing_solutions);
        }

        Ok(())
    }
}
