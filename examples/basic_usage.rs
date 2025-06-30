use rayon::vec;

use rezalnyas_core::{
    enums::{cut_orientation_preference::CutOrientationPreference, status::Status},
    log_debug, log_error, log_info, log_warn,
    logging::{init_logging, LogConfig, LogLevel},
    models::{
        calculation_request::CalculationRequest,
        configuration::Configuration,
        grouped_tile_dimensions::{get_distinct_grouped_tile_dimensions, GroupedTileDimensions},
        mosaic::Mosaic,
        panel::structs::Panel,
        performance_thresholds::PerformanceThresholds,
        permutation_thread_spawner::{PermutationThreadSpawner, ProgressTracker},
        solution::Solution,
        stock_solution::StockPanelPicker,
        task::Task,
        tile::grouped_tile_dimensions_list_to_tile_dimensions_list,
        tile_dimensions::{
            count_duplicate_permutations, generate_groups, generate_groups_improved,
            generate_groups_java_compatible, remove_duplicated_permutations,
            remove_duplicated_permutations_java_compatible, TileDimensions,
        },
        tile_node::TileNode,
    },
    scaled_math::{PrecisionAnalyzer, ScaledConverter, ScaledNumber},
    services::{
        arrangement::generate_permutations,
        computation::{process_permutation_with_all_stock_solutions, OptimizationPriority},
    },
    CutListOptimizerService, CuttingRequest, Material, OptimizationConfig, OptimizationStrategy,
};

const MAX_ALLOWED_DIGITS: u8 = 6;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut unique_tile_id = 1i32;

    // Инициализация логирования
    init_logging(LogConfig {
        level: LogLevel::Debug,
    });

    log_info!("Приложение запущено");

    let panels: Vec<Panel> = vec![
        // Деталь 1: 150.5x100.25 (2 шт)
        Panel::new(1, "150.5".to_string(), "100.25".to_string(), 2),
        // Деталь 2: 80.75x60.5 (3 шт)
        Panel::new(2, "80.75".to_string(), "60.5".to_string(), 3),
        // Деталь 3: 120.0x45.75 (1 шт)
        Panel::new(3, "120.0".to_string(), "45.75".to_string(), 1),
        // Деталь 4: 95.25x75.5 (2 шт)
        Panel::new(4, "95.25".to_string(), "75.5".to_string(), 2),
        // Деталь 5: 65.5x85.25 (1 шт)
        Panel::new(5, "65.5".to_string(), "85.25".to_string(), 1),
        // Деталь 6: 110.75x55.0 (2 шт)
        Panel::new(6, "110.75".to_string(), "55.0".to_string(), 2),
        // Деталь 7: 40.25x90.5 (3 шт)
        Panel::new(7, "40.25".to_string(), "90.5".to_string(), 3),
        // Деталь 8: 130.0x35.75 (1 шт)
        Panel::new(8, "130.0".to_string(), "35.75".to_string(), 1),
    ];

    // Одна заготовка (такая же как в Java)
    let stock_panels: Vec<Panel> = vec![Panel::new(1, "400.0".to_string(), "300.0".to_string(), 1)];

    let config = Configuration {
        cut_thickness: 0.0,
        use_single_stock_unit: false,
        optimization_factor: 2.0,
        optimization_priority: vec![
            OptimizationPriority::MostTiles,
            OptimizationPriority::LeastWastedArea,
            OptimizationPriority::LeastNbrCuts,
            OptimizationPriority::LeastNbrMosaics,
            OptimizationPriority::BiggestUnusedTileArea,
            OptimizationPriority::MostHvDiscrepancy,
        ],
        cut_orientation_preference: CutOrientationPreference::Both, // Все направления резов

        consider_orientation: false, // Учитывать ориентацию волокон
        min_trim_dimension: 0.0, //  это минимальный полезный размер остатка в любом направлении, в тех же единицах, что и остальные размеры (мм, см, дюймы и т.д.).
        performance_thresholds: PerformanceThresholds {
            max_simultaneous_tasks: 1,   // Максимум потоков
            max_simultaneous_threads: 1, // Максимум потоков на задачу
            thread_check_interval: 100,
        },
    };
    let request = CalculationRequest {
        configuration: config,
        panels,
        stock_panels,
    };

    log_debug!("RUST DEBUG: === Создание запроса ===");
    log_debug!("JAVA аналог: CalculationRequest создан");
    log_debug!("RUST: Создан запрос:");
    log_debug!("RUST: - Деталей: {}", request.panels.len());
    log_debug!(
        "RUST: - Заготовка: {}x{}",
        request.stock_panels[0].width,
        request.stock_panels[0].height
    );

    // Вычисление точности
    let panels = &request.panels;
    let stock_panels = &request.stock_panels;
    let configuration = &request.configuration;

    log_debug!("RUST DEBUG: === Вычисление точности ===");
    log_debug!("JAVA аналог: precision calculation");

    // Вычисление максимального количества знаков после запятой
    let max_decimal_places_panels = Panel::get_max_decimal_places(panels);
    let max_decimal_places_stock = Panel::get_max_decimal_places(stock_panels);

    // Получение точности для толщины реза и минимального размера обрезки
    let cut_thickness_precision =
        PrecisionAnalyzer::count_decimal_places(&configuration.cut_thickness.to_string());
    let min_trim_precision =
        PrecisionAnalyzer::count_decimal_places(&configuration.min_trim_dimension.to_string());

    // Определение максимальной точности

    let max_decimal_places = [
        max_decimal_places_panels,
        max_decimal_places_stock,
        cut_thickness_precision,
        min_trim_precision,
    ]
    .iter()
    .max()
    .copied()
    .unwrap_or(0);

    // Вычисление максимального количества цифр до запятой
    let max_integer_places_panels = Panel::get_max_integer_places(panels);
    let max_integer_places_stock = Panel::get_max_integer_places(stock_panels);

    let cut_thickness_integer =
        PrecisionAnalyzer::count_integer_places(&configuration.cut_thickness.to_string());
    let min_trim_integer =
        PrecisionAnalyzer::count_integer_places(&configuration.min_trim_dimension.to_string());

    let max_integer_places = [
        max_integer_places_panels,
        max_integer_places_stock,
        cut_thickness_integer,
        min_trim_integer,
    ]
    .iter()
    .max()
    .copied()
    .unwrap_or(0);

    // Проверка на превышение максимального количества цифр
    // iMax2 - final_precision
    let final_precision = if max_decimal_places + max_integer_places > MAX_ALLOWED_DIGITS {
        log_warn!(
            "Maximum allowed digits exceeded: maxDecimalPlaces[{}] maxIntegerPlaces[{}] maxAllowedDigits[{}]",
            max_decimal_places, max_integer_places, MAX_ALLOWED_DIGITS
        );
        MAX_ALLOWED_DIGITS.saturating_sub(max_integer_places).max(0)
    } else {
        max_decimal_places
    };

    log_debug!("RUST DEBUG: final_precision = {}", final_precision);

    // Создание конвертера с определенной точностью
    let converter = ScaledConverter::new(final_precision)?;
    // dPow - scale_factor
    let scale_factor = 10_i64.pow(final_precision as u32);

    log_debug!("RUST DEBUG: scale_factor = {}", scale_factor);
    log_debug!("JAVA аналог: double dPow = Math.pow(10.0d, iMax);");

    // Создание списков для результатов
    let mut tiles = Vec::new();
    let mut stock_tiles = Vec::new();

    log_debug!("RUST DEBUG: === Создание TileDimensions ===");
    log_debug!("JAVA аналог: ArrayList arrayList = new ArrayList();");

    // Обработка панелей (tiles)
    for panel in panels {
        for _ in 0..panel.count {
            let width_scaled = ScaledNumber::from_str(&panel.width, final_precision)?;
            let height_scaled = ScaledNumber::from_str(&panel.height, final_precision)?;

            let tile = TileDimensions {
                id: unique_tile_id, // КАЖДАЯ ДЕТАЛЬ ПОЛУЧАЕТ УНИКАЛЬНЫЙ ID
                width: width_scaled.raw_value() as u64,
                height: height_scaled.raw_value() as u64,
                orientation: panel.orientation,
                is_rotated: false,
            };

            log_debug!(
                "RUST DEBUG: Создана панель ID {}: {}x{} (масштаб: {} -> {}x{})",
                unique_tile_id,
                panel.width,
                panel.height,
                scale_factor,
                tile.width,
                tile.height
            );
            log_debug!("JAVA аналог: arrayList.add(new TileDimensions(...));");

            tiles.push(tile);
            unique_tile_id += 1; // Увеличиваем ID для следующей детали
        }
    }

    // Обработка складских панелей (stock tiles)
    log_debug!("RUST DEBUG: === Создание Stock TileDimensions ===");
    log_debug!("JAVA аналог: ArrayList arrayList2 = new ArrayList();");

    for stock_panel in stock_panels {
        for _ in 0..stock_panel.count {
            let width_scaled = ScaledNumber::from_str(&stock_panel.width, final_precision)?;
            let height_scaled = ScaledNumber::from_str(&stock_panel.height, final_precision)?;

            let stock_tile = TileDimensions {
                id: unique_tile_id, // КАЖДАЯ ЗАГОТОВКА ПОЛУЧАЕТ УНИКАЛЬНЫЙ ID
                width: width_scaled.raw_value() as u64,
                height: height_scaled.raw_value() as u64,

                orientation: stock_panel.orientation,
                is_rotated: false,
            };

            log_debug!(
                "RUST DEBUG: Создан лист ID {}: {}x{} (масштаб: {} -> {}x{})",
                unique_tile_id,
                stock_panel.width,
                stock_panel.height,
                scale_factor,
                stock_tile.width,
                stock_tile.height
            );
            log_debug!("JAVA аналог: arrayList2.add(new TileDimensions(...));");

            stock_tiles.push(stock_tile);
        }
    }

    log_debug!("RUST DEBUG: === Итого создано ===");
    log_debug!("RUST: Панелей: {}", tiles.len());
    log_debug!("RUST: Листов: {}", stock_tiles.len());
    log_debug!(
        "JAVA аналог: arrayList.size() = {}, arrayList2.size() = {}",
        tiles.len(),
        stock_tiles.len()
    );

    let mut task = Task {
        id: "test_task".to_string(),
        calculation_request: request.clone(),
        factor: scale_factor,
        solutions: Vec::new(),
        status: Status::Idle,
        percentage_done: 0,
        iterations_completed: 0,
        error_message: None,
        best_solution: None,
        start_time: None,
    };
    log_debug!("RUST DEBUG: === Task создан ===");
    log_debug!("JAVA аналог: final Task task = new Task(...);");
    // Генерация групп
    let list_generate_groups = generate_groups_java_compatible(&tiles, &stock_tiles, &task);
    //получаем уникальные группы
    let distinct_grouped_tile_dimensions =
        get_distinct_grouped_tile_dimensions(&list_generate_groups, &configuration);
    log_debug!("Task[{}] Calculating permutations...", task.id);

    let mut group_info = String::new();
    for (i, (group, count)) in distinct_grouped_tile_dimensions.iter().enumerate() {
        group_info.push_str(&format!(" group[{}:{}*{}] ", i + 1, group, count));
    }

    log_debug!("Task[{}] Groups: {}", task.id, group_info);
    log_info!("Generated {} groups", list_generate_groups.len());
    log_info!(
        "Distinct groups: {}",
        distinct_grouped_tile_dimensions.len()
    );
    //  Создание отсортированного списка ключей
    let mut sorted_grouped_tiles: Vec<GroupedTileDimensions> =
        distinct_grouped_tile_dimensions.keys().cloned().collect();

    // Сортировка по площади в убывающем порядке (аналог Java Comparator)
    sorted_grouped_tiles.sort_by(|a, b| {
        // Сравниваем по площади в убывающем порядке (как в Java: b.area - a.area)
        b.get_area().cmp(&a.get_area())
    });

    log_debug!(
        "Successfully sorted {} distinct tile dimensions by area",
        sorted_grouped_tiles.len()
    );

    // Логирование отсортированного списка для отладки
    for (i, grouped_tile) in sorted_grouped_tiles.iter().enumerate().take(5) {
        // Показываем первые 5
        log_debug!(
            "Sorted[{}]: {} area={}",
            i,
            grouped_tile,
            grouped_tile.get_area()
        );
    }

    log_info!(
        "Sorted {} distinct grouped tiles by area",
        sorted_grouped_tiles.len()
    );
    /*
     * Оптимизация количества перестановок
     *
     * Если групп больше 7, то для перестановок берем только первые 7 (самые большие)
     * Остальные группы добавим в конец каждой перестановки без изменения порядка
     * Это сокращает количество вариантов с факториала до разумного числа
     */
    let (permutation_tiles, remaining_tiles) = if sorted_grouped_tiles.len() > 7 {
        let permutation_tiles = sorted_grouped_tiles[0..7].to_vec();
        let remaining_tiles = sorted_grouped_tiles[7..].to_vec();

        log_debug!(
            "Task[{}] Optimization: Using first 7 groups for permutations, {} groups will be appended",
            task.id,
            remaining_tiles.len()
        );

        (permutation_tiles, remaining_tiles)
    } else {
        log_debug!(
            "Task[{}] Using all {} groups for permutations",
            task.id,
            sorted_grouped_tiles.len()
        );

        (sorted_grouped_tiles, Vec::new())
    };

    log_info!(
        "Permutation groups: {}, Remaining groups: {}",
        permutation_tiles.len(),
        remaining_tiles.len()
    );

    // Логирование групп для перестановок
    for (i, tile) in permutation_tiles.iter().enumerate() {
        log_debug!("Permutation[{}]: {} area={}", i, tile, tile.get_area());
    }

    // Логирование оставшихся групп
    for (i, tile) in remaining_tiles.iter().enumerate() {
        log_debug!("Remaining[{}]: {} area={}", i, tile, tile.get_area());
    }

    /*
     * Генерация перестановок
     *
     * Генерируем все возможные перестановки первых 7 групп
     * К каждой перестановке добавляем оставшиеся группы в исходном порядке
     */

    // Генерируем все перестановки для групп permutation_tiles
    let mut list_generate_permutations = generate_permutations(permutation_tiles);

    log_debug!(
        "Task[{}] Generated {} permutations from {} groups",
        task.id,
        list_generate_permutations.len(),
        list_generate_permutations.first().map_or(0, |p| p.len())
    );

    // К каждой перестановке добавляем оставшиеся группы в исходном порядке
    for permutation in &mut list_generate_permutations {
        permutation.extend_from_slice(&remaining_tiles);
    }

    log_debug!(
        "Task[{}] Final permutations: {} permutations with {} total groups each",
        task.id,
        list_generate_permutations.len(),
        list_generate_permutations.first().map_or(0, |p| p.len())
    );

    log_info!(
        "Generated {} total permutations with {} groups each",
        list_generate_permutations.len(),
        list_generate_permutations.first().map_or(0, |p| p.len())
    );

    // Логирование первых нескольких перестановок для отладки
    for (i, permutation) in list_generate_permutations.iter().take(3).enumerate() {
        let perm_info: String = permutation
            .iter()
            .enumerate()
            .map(|(j, group)| format!("{}[area={}]", j, group.get_area()))
            .collect::<Vec<_>>()
            .join(" ");

        log_debug!("Permutation[{}]: {}", i, perm_info);
    }

    if list_generate_permutations.len() > 3 {
        log_debug!(
            "... and {} more permutations",
            list_generate_permutations.len() - 3
        );
    }

    /*
     * Преобразование перестановок в списки панелей
     *
     * Преобразуем каждую перестановку групп обратно в последовательность отдельных панелей
     * Теперь sorted_tile_lists содержит различные порядки размещения панелей
     */

    log_debug!(
        "Task[{}] Sorting tiles according to permutations...",
        task.id
    );

    let mut sorted_tile_lists: Vec<Vec<TileDimensions>> = Vec::new();

    for permutation in &list_generate_permutations {
        let sorted_tiles = grouped_tile_dimensions_list_to_tile_dimensions_list(
            permutation,
            &list_generate_groups,
        );
        sorted_tile_lists.push(sorted_tiles);
    }

    log_debug!(
        "Task[{}] Created {} sorted tile lists from permutations",
        task.id,
        sorted_tile_lists.len()
    );

    log_info!(
        "Created {} sorted tile arrangements from permutations",
        sorted_tile_lists.len()
    );

    // Логирование первых нескольких отсортированных списков для отладки
    for (i, sorted_list) in sorted_tile_lists.iter().take(2).enumerate() {
        let tiles_info: String = sorted_list
            .iter()
            .take(5) // Показываем первые 5 плиток
            .map(|tile| format!("{}[{}x{}]", tile.id, tile.width, tile.height))
            .collect::<Vec<_>>()
            .join(" ");

        log_debug!(
            "SortedList[{}] ({} tiles): {} {}",
            i,
            sorted_list.len(),
            tiles_info,
            if sorted_list.len() > 5 { "..." } else { "" }
        );
    }

    if sorted_tile_lists.len() > 2 {
        log_debug!(
            "... and {} more sorted tile lists",
            sorted_tile_lists.len() - 2
        );
    }

    /*
     * Удаление дублирующих перестановок
     *
     * Удаляем перестановки, которые приводят к одинаковым результатам
     * Это экономит время вычислений
     */

    log_debug!("Task[{}] Removing duplicated permutations...", task.id);

    // Сначала подсчитаем дубликаты для статистики
    let (total_before, unique_before, duplicates_before) =
        count_duplicate_permutations(&sorted_tile_lists);

    log_debug!(
        "Task[{}] Before deduplication: {} total permutations, {} unique, {} duplicates",
        task.id,
        total_before,
        unique_before,
        duplicates_before
    );

    // Удаляем дублированные перестановки
    let removed_count = remove_duplicated_permutations_java_compatible(&mut sorted_tile_lists);

    log_debug!(
        "Task[{}] After deduplication: {} permutations remaining ({} removed)",
        task.id,
        sorted_tile_lists.len(),
        removed_count
    );

    log_info!(
        "Removed {} duplicate permutations, {} unique arrangements remaining",
        removed_count,
        sorted_tile_lists.len()
    );

    // Логирование эффективности дедупликации
    if total_before > 0 {
        let efficiency_percent = (removed_count as f64 / total_before as f64) * 100.0;
        log_debug!(
            "Task[{}] Deduplication efficiency: {:.1}% duplicates removed",
            task.id,
            efficiency_percent
        );

        if efficiency_percent > 10.0 {
            log_debug!(
                "Task[{}] High deduplication efficiency - significant optimization achieved",
                task.id
            );
        }
    }

    /*
     * Установка статуса и инициализация генератора листов
     *
     * Помечаем задачу как выполняющуюся
     * Создаем генератор комбинаций исходных листов
     * Инициализируем его (запускается отдельный поток для генерации вариантов листов)
     */

    // Устанавливаем статус задачи как "выполняющаяся"
    task.status = Status::Running;

    // Определяем максимальное количество листов (если включен режим одного листа)
    let max_stock_units = if configuration.use_single_stock_unit {
        Some(1)
    } else {
        None
    };

    // Создаем генератор комбинаций листов
    let mut stock_panel_picker = StockPanelPicker::new(
        tiles.clone(),
        stock_tiles.clone(),
        task.clone(),
        max_stock_units,
    );

    // Инициализируем генератор (запускает отдельный поток)
    stock_panel_picker.init(tiles.clone(), stock_tiles.clone(), max_stock_units);

    log_debug!(
        "Task[{}] Initialized stock panel picker with {} stock tile types",
        task.id,
        stock_tiles.len()
    );

    /*
     * Расчет размера пула решений
     *
     * Определяем размер пула лучших решений, которые будем хранить
     * Базовый размер = 100 * коэффициент оптимизации из конфигурации
     * Если панелей много (>100), уменьшаем размер пула для экономии памяти
     */

    let mut optimization_factor = if configuration.optimization_factor > 0.0 {
        100.0 * configuration.optimization_factor // В Java это ДВОЙКА * 100 = 200!
    } else {
        100.0
    };

    log_info!(
        "Initial optimization factor calculation: config.optimization_factor={}, 100 * {} = {}",
        configuration.optimization_factor,
        configuration.optimization_factor,
        optimization_factor
    );
    if tiles.len() > 100 {
        let reduction_factor = 0.5 / (tiles.len() as f64 / 100.0);
        optimization_factor = (optimization_factor * reduction_factor);

        log_info!(
            "Tiles count {} > 100, applying reduction: {} * {:.4} = {}",
            tiles.len(),
            100.0 * configuration.optimization_factor,
            reduction_factor,
            optimization_factor
        );
        log_info!(
            "Limiting solution pool elements to [{}]",
            optimization_factor
        );
    }

    log_info!(
        "Final optimization factor: {}, solution pool size: {}",
        optimization_factor,
        optimization_factor
    );

    log_info!("\n=== Алгоритм выполнен успешно ===");
    log_info!("Обработано {} деталей", tiles.len());
    log_info!("Создано {} вариантов размещения", sorted_tile_lists.len());
    log_info!("Оптимизационный фактор: {}", optimization_factor);

    /*
     * Инициализация менеджеров потоков (однопоточная версия)
     *
     * Создаем менеджер для обработки перестановок
     * Создаем трекер прогресса выполнения
     * Настраиваем ограничения (в однопоточном режиме они не критичны)
     */

    let mut permutation_thread_spawner = PermutationThreadSpawner::new();
    let mut progress_tracker = ProgressTracker::new(
        sorted_tile_lists.len(),
        &mut task,
        "DEFAULT_MATERIAL".to_string(),
    );

    permutation_thread_spawner.set_progress_tracker(&mut progress_tracker);
    permutation_thread_spawner.set_max_alive_spawner_threads(
        configuration
            .performance_thresholds
            .max_simultaneous_threads,
    );
    permutation_thread_spawner.set_interval_between_max_alive_check(
        configuration.performance_thresholds.thread_check_interval,
    );

    log_info!(
        "Initialized thread managers: max_threads={}, check_interval={}ms",
        configuration
            .performance_thresholds
            .max_simultaneous_threads,
        configuration.performance_thresholds.thread_check_interval
    );

    /*
     * Основной цикл обработки перестановок - ТОЧНАЯ КОПИЯ JAVA ЛОГИКИ
     *
     * Java: while (true) { if (i >= arrayList4.size()) { break; } ... }
     *
     * Цикл по каждой перестановке панелей
     * Проверяем, не остановлена ли задача
     * Если уже найдено решение "все панели помещаются" и запущено достаточно потоков - прекращаем
     */

    const MAX_PERMUTATIONS_WITH_SOLUTION: usize = 150; // Java: MAX_PERMUTATIONS_WITH_SOLUTION = 150
    let mut i = 0; // Java: int i = 0 (счетчик перестановок)
    let str3 = "]"; // Java: String str3 = "]"; (для логирования)
    let i3 = 100; // Java: int i3 = 100; (базовое значение процента)
    let material_str = "DEFAULT_MATERIAL"; // Java: final String materialStr = str;

    // Java: while (true)
    loop {
        // Java: if (i >= arrayList4.size()) { break; }
        if i >= sorted_tile_lists.len() {
            log_debug!("Reached end of permutation list at index[{}]", i);
            break;
        }

        // Java: final List<TileDimensions> list3 = (List<TileDimensions>) arrayList4.get(i);
        let tile_arrangement = &sorted_tile_lists[i];

        // Java: if (!task.isRunning()) { logger.debug("Tasked no longer has running status..."); break; }
        if !task.is_running() {
            log_debug!(
                "Task no longer has running status. Stopping permutationSpawnerThread spawner at permutationIdx[{}{}",
                i, str3
            );
            break;
        }

        // Java: if (task.hasSolutionAllFit() && permutationThreadSpawner.getNbrTotalThreads() > MAX_PERMUTATIONS_WITH_SOLUTION)
        if task.has_solution_all_fit()
            && permutation_thread_spawner.get_nbr_total_threads() > MAX_PERMUTATIONS_WITH_SOLUTION
        {
            // Java: task2.setMaterialPercentageDone(str2, Integer.valueOf(i3));
            // task.set_material_percentage_done(material_str, i3); // TODO: реализовать когда будет структура
            log_debug!("Task has solution and spawned max permutations threads");
            break;
        }

        /*
         * Запуск потока для обработки перестановки - АДАПТАЦИЯ ДЛЯ ОДНОПОТОЧНОСТИ
         *
         * Java создает новый поток:
         * permutationThreadSpawner2.spawn(new Thread(new Runnable() { ... }));
         *
         * В Rust однопоточной версии вызываем метод напрямую:
         * m301x52dbbde3(...) - это Java метод обработки одной перестановки
         */

        log_debug!(
            "Processing permutation[{}] - spawning thread equivalent for {} tiles",
            i,
            tile_arrangement.len()
        );

        // Java: final variables для передачи в лямбду
        // В Rust передаем напрямую в функцию
        let stock_panel_picker_ref = &stock_panel_picker;
        let permutation_idx = i; // Java: final int i5 = i;
        let progress_tracker_ref = &mut progress_tracker; // Java: final ProgressTracker progressTracker2 = progressTracker;
        let performance_thresholds_ref = &configuration.performance_thresholds; // Java: final PerformanceThresholds performanceThresholds2 = performanceThresholds;

        // Java: CutListOptimizerServiceImpl.this.m301x52dbbde3(...)
        // Rust: process_permutation_with_all_stock_solutions (аналог m301x52dbbde3)
        match process_permutation_with_all_stock_solutions(
            stock_panel_picker_ref,
            permutation_idx,
            &mut task,
            &Vec::new(), // solutions - пока пустой список
            &sorted_tile_lists,
            &configuration,
            tile_arrangement,
            optimization_factor,
            performance_thresholds_ref,
            progress_tracker_ref,
            material_str,
        ) {
            Ok(_) => {
                log_debug!("Successfully processed permutation[{}]", permutation_idx);
            }
            Err(e) => {
                log_error!("Error processing permutation[{}]: {}", permutation_idx, e);
                // Java: Thread.currentThread().interrupt(); - в однопоточности не актуально
            }
        }

        // Java: i++; (увеличиваем счетчик перестановок)
        i += 1;

        // Регистрируем завершение обработки перестановки
        permutation_thread_spawner.register_completed_permutation();

        // Обновляем прогресс
        progress_tracker.refresh_task_status_info(&permutation_thread_spawner);

        // Каждые 10 перестановок выводим прогресс (дополнительно для отладки)
        if i % 10 == 0 {
            log_info!(
                "Progress: {}/{} permutations processed ({:.1}%) - Java loop iteration {}",
                i,
                sorted_tile_lists.len(),
                (i as f64 / sorted_tile_lists.len() as f64) * 100.0,
                i
            );
        }

        // Java: обновление ссылок переменных (в однопоточности не требуется)
        // permutationThreadSpawner = permutationThreadSpawner2;
        // stockPanelPicker = stockPanelPicker;
        // progressTracker = progressTracker2;
        // str3 = str3;
        // arrayList4 = arrayList6;
        // performanceThresholds = performanceThresholds;
        // i3 = 100;
        // task2 = task;
    }

    /*
     * Финализация
     *
     * Устанавливаем статус задачи как завершенная
     * Логируем итоговую статистику
     */

    if task.status == Status::Running {
        task.status = Status::Finished;
    }

    log_info!(
        "Task[{}] Processing completed. Status: {:?}",
        task.id,
        task.status
    );

    log_info!("\n=== Подготовка к раскрою завершена ===");
    // log_info!("Обработано {} перестановок панелей", processed_permutations);
    log_info!("Оптимизационный фактор: {}", optimization_factor);
    log_info!("Сгенерированы варианты комбинаций листов");
    log_info!("Готово к реализации алгоритма размещения панелей");

    log_debug!("RUST DEBUG: === НАЧИНАЕМ АЛГОРИТМ РАЗМЕЩЕНИЯ ===");
    log_debug!("JAVA аналог: CutListThread.computeSolutions()");
    log_debug!("RUST: Аналог Java метода computeSolutions()");

    // ЭТАП 1: Создание начального решения
    log_debug!("RUST DEBUG ЭТАП 1: === Создание начального решения ===");
    log_debug!("JAVA аналог: List<Solution> arrayList = new ArrayList<>();");
    log_debug!("JAVA аналог: arrayList.add(new Solution(this.stockSolution));");

    log_debug!("RUST ЭТАП 1: === Создание начального решения ===");
    log_debug!("RUST аналог Java: List<Solution> arrayList = new ArrayList<>();");
    log_debug!("RUST аналог Java: arrayList.add(new Solution(this.stockSolution));");

    let mut solution = Solution::new();

    // Создаем мозаику для каждого листа
    for (i, stock_tile) in stock_tiles.iter().enumerate() {
        log_debug!(
            "RUST: Создаем мозаику для листа {}: {}x{}",
            i + 1,
            stock_tile.width,
            stock_tile.height
        );

        // Создаем корневой узел для листа
        let root_node = TileNode::new(0, 0, stock_tile.width as i32, stock_tile.height as i32);

        // Создаем мозаику из этого узла
        let mosaic = Mosaic::from_tile_node(&root_node, "DEFAULT_MATERIAL".to_string());
        solution.add_mosaic(mosaic);

        log_debug!(
            "RUST: Мозаика {} создана, размер: {}x{}",
            i + 1,
            stock_tile.width,
            stock_tile.height
        );
    }
let mut current_solutions = vec![solution.clone()]; // Начальное решение с одной мозаикой
    log_debug!(
        "RUST ЭТАП 1 ЗАВЕРШЕН: Создано решение с {} мозаиками",
        solution.mosaics.len()
    );
    log_debug!("JAVA аналог: arrayList.size() = {}", solution.mosaics.len());

    // Логируем информацию о stock tiles (аналог stockSolution)
    log_debug!(
        "RUST DEBUG: Stock tiles содержит {} листов:",
        stock_tiles.len()
    );
    for (stock_idx, stock_tile) in stock_tiles.iter().enumerate() {
        log_debug!(
            "RUST:   Лист {}: {}x{} (ID: {})",
            stock_idx + 1,
            stock_tile.width,
            stock_tile.height,
            stock_tile.id
        );
    }
    log_debug!("JAVA аналог: StockSolution информация");

    // if task.is_running() {
    log_debug!(
        "RUST DEBUG: Task is running, начинаем обработку {} панелей",
        tiles.len()
    );
    log_debug!("JAVA аналог: if (this.task.isRunning())");
    assert_eq!(tiles.len(), 15);
    // ЭТАП 2: Главный цикл размещения панелей
    for (tile_index, tile) in tiles.iter().enumerate() {
        let i = tile_index + 1; // Java использует 1-based индекс

        log_debug!(
            "RUST DEBUG ЭТАП 2: === Размещение панели {} из {} ===",
            i,
            tiles.len()
        );
        log_debug!("JAVA аналог: for (TileDimensions tileDimensions : this.tiles)");
        log_debug!(
            "RUST: Панель {}x{}, ID: {}",
            tile.width,
            tile.height,
            tile.id
        );
        log_debug!("JAVA аналог: i = {}", i);
        let a = 10;
//   let new_solutions = place_single_tile_debug(tile, &current_solutions, tile_index, &task)?;
            
  println!("\nRUST place_single_tile_debug(): === Размещение панели {} ===", tile_index + 1);
    println!("JAVA аналог: for (TileDimensions tileDimensions : this.tiles) - итерация {}", tile_index + 1);
    println!("RUST: Панель {}x{}, ID: {}", tile.width, tile.height, tile.id);
    
    let mut new_solutions: Vec<Solution> = Vec::new();
    
    println!("RUST: ArrayList<Solution> arrayList2 = new ArrayList();");
    println!("JAVA аналог: новый список решений создан");


    println!("RUST: ArrayList<Solution> arrayList2 = new ArrayList();");
    println!("JAVA аналог: новый список решений создан");
    
    // Пробуем разместить панель в каждом существующем решении
    println!("RUST: Iterator<Solution> it = arrayList.iterator();");



    








let mut solution_idx = 0;
    for solution in current_solutions.clone() {
        solution_idx += 1;
        println!("RUST: === Проверяем решение {} ===", solution_idx);
        println!("JAVA аналог: Iterator<Solution> it = arrayList.iterator() - решение {}", solution_idx);
        println!("RUST: Решение содержит {} мозаик", solution.mosaics.len());
        
        let mut tile_placed = false;
        
        // Пробуем каждую мозаику в решении
        let mut mosaic_idx = 0;
        for mosaic in &solution.mosaics {
            mosaic_idx += 1;
            println!("RUST: === Проверяем мозаику {} ===", mosaic_idx);
            println!("JAVA аналог: ListIterator<Mosaic> listIterator - мозаика {}", mosaic_idx);
            println!("RUST: Мозаика размер: {}x{}", mosaic.width(), mosaic.height());
            println!("RUST: Мозаика материал: {}", mosaic.material());
            
            // Проверка совместимости материалов
            println!("RUST: Проверяем совместимость материалов");
            println!("JAVA аналог: if (next3.getMaterial() != null && !next3.getMaterial().equals(tileDimensions.getMaterial()))");
            
            // В упрощенной версии все материалы совместимы
            println!("RUST: Материалы совместимы, пробуем разместить");
            println!("JAVA аналог: материалы совместимы");
            
            // ЭТАП 2.1: Проверяем, помещается ли панель
            if fits(tile, mosaic) {
                println!("RUST: Панель МОЖЕТ поместиться в мозаику {}", mosaic_idx);
                println!("JAVA аналог: List<Mosaic> arrayList3 = new ArrayList<>();");
                
                // ЭТАП 2.2: Пробуем разместить
                if let Some(new_mosaic) = try_place_tile_simple(tile, mosaic)? {
                    println!("RUST: Панель УСПЕШНО размещена в мозаике {}", mosaic_idx);
                    println!("JAVA аналог: arrayList3.size() > 0");
                    
                    // Создаем новое решение
                    let mut new_solution = solution.clone();
                    new_solution.replace_mosaic(mosaic_idx - 1, new_mosaic); // -1 так как mosaic_idx 1-based
                    new_solutions.push(new_solution);
                    tile_placed = true;
                    
                    println!("JAVA аналог: Solution solution = new Solution(next2, next3);");
                    println!("JAVA аналог: arrayList2.add(solution);");
                    break; // Размещаем только в первой подходящей мозаике
                } else {
                    println!("RUST: Размещение НЕ удалось в мозаике {}", mosaic_idx);
                }
            } else {
                println!("RUST: Панель НЕ помещается в мозаику {}", mosaic_idx);
            }
        }
        
        // Если панель не поместилась ни в одну мозаику
        if !tile_placed {
            println!("RUST: Панель НЕ поместилась ни в одну мозаику решения {}", solution_idx);
            println!("JAVA аналог: next2.getNoFitPanels().add(tileDimensions);");
            
            let mut failed_solution = solution.clone();
            failed_solution.add_no_fit_tile(tile.clone());
            new_solutions.push(failed_solution);
        }
    }
    
    println!("RUST place_single_tile_debug() ЗАВЕРШЕН: Создано {} новых решений", new_solutions.len());
    println!("JAVA аналог: arrayList2.size() = {}", new_solutions.len());
    













    }

    Ok(())
}





/// JAVA аналог: TileDimensions.fits() метод
/// 
/// Java код:
/// ```java
/// public boolean fits(TileDimensions tileDimensions) {
///     int i = this.width;  // available space width  
///     int i2 = tileDimensions.width;  // tile width
///     return (i >= i2 && this.height >= tileDimensions.height) || 
///            (this.height >= i2 && i >= tileDimensions.height);
/// }
/// ```
fn fits(tile: &TileDimensions, mosaic: &Mosaic) -> bool {
    println!("RUST can_tile_fit_in_mosaic(): Проверяем размещение");
    println!("JAVA аналог: stockPanel.fits(tileDimensions)");
    
    // В Java: int i = this.width; (доступное пространство)
    let available_width = mosaic.width() as u64;
    let available_height = mosaic.height() as u64;
    
    // В Java: int i2 = tileDimensions.width; (размер панели)
    let tile_width = tile.width;
    let tile_height = tile.height;
    
    println!("RUST: Проверяем: панель {}x{} в мозаику {}x{}", 
        tile_width, tile_height, available_width, available_height);
    println!("JAVA аналог: проверка размеров для размещения");
    
    // Java логика: (i >= i2 && this.height >= tileDimensions.height)
    let fits_normal = available_width >= tile_width && available_height >= tile_height;
    
    // Java логика: (this.height >= i2 && i >= tileDimensions.height)  
    let fits_rotated = available_height >= tile_width && available_width >= tile_height;
    
    // Java логика: return condition1 || condition2
    let can_fit = fits_normal || fits_rotated;
    
    println!("RUST: Результат проверки: {} (normal: {}, rotated: {})", 
        can_fit, fits_normal, fits_rotated);
    println!("JAVA аналог: return ({} >= {} && {} >= {}) || ({} >= {} && {} >= {})",
        available_width, tile_width, available_height, tile_height,
        available_height, tile_width, available_width, tile_height);
    
    can_fit
}




/// ЭТАП 2: Размещение одной панели - аналог Java главного цикла
fn place_single_tile(
    tile: &TileDimensions,
    current_solutions: &[Solution],
    tile_index: usize,
    task: &Task,
) -> Result<Vec<Solution>, Box<dyn std::error::Error>> {
    log_debug!(
        "\nRUST ЭТАП 2: === Размещение панели {} ===",
        tile_index + 1
    );
    log_debug!("JAVA аналог: for (TileDimensions tileDimensions : this.tiles)");
    log_debug!(
        "RUST: Панель {}x{}, ID: {}",
        tile.width,
        tile.height,
        tile.id
    );

    let mut new_solutions: Vec<Solution> = Vec::new();

    log_debug!("JAVA аналог: ArrayList<Solution> arrayList2 = new ArrayList();");

    // Пробуем разместить панель в каждом существующем решении
    for (sol_idx, solution) in current_solutions.iter().enumerate() {
        log_debug!(
            "RUST: Пробуем решение {} с {} мозаиками",
            sol_idx + 1,
            solution.mosaics.len()
        );
        log_debug!("JAVA аналог: Iterator<Solution> it = arrayList.iterator();");

        let mut tile_placed = false;

        // Пробуем каждую мозаику в решении
        for (mosaic_idx, mosaic) in solution.mosaics.iter().enumerate() {
            log_debug!(
                "RUST: Пробуем мозаику {} размером {}x{}",
                mosaic_idx + 1,
                mosaic.width(),
                mosaic.height()
            );

            // ЭТАП 2.1: Проверяем, помещается ли панель
            if fits(tile, mosaic) {
                log_debug!(
                    "RUST: Панель МОЖЕТ поместиться в мозаику {}",
                    mosaic_idx + 1
                );

                // ЭТАП 2.2: Пробуем разместить
                if let Some(new_mosaic) = try_place_tile_simple(tile, mosaic)? {
                    log_debug!(
                        "RUST: Панель УСПЕШНО размещена в мозаике {}",
                        mosaic_idx + 1
                    );

                    // Создаем новое решение
                    let mut new_solution = solution.clone();
                    new_solution.replace_mosaic(mosaic_idx, new_mosaic);
                    new_solutions.push(new_solution);
                    tile_placed = true;

                    log_debug!("JAVA аналог: Solution solution = new Solution(next2, next3);");
                    log_debug!("JAVA аналог: arrayList2.add(solution);");
                    break; // Размещаем только в первой подходящей мозаике
                }
            } else {
                log_debug!("RUST: Панель НЕ помещается в мозаику {}", mosaic_idx + 1);
            }
        }

        // Если панель не поместилась ни в одну мозаику
        if !tile_placed {
            log_debug!(
                "RUST: Панель НЕ поместилась ни в одну мозаику решения {}",
                sol_idx + 1
            );
            log_debug!("JAVA аналог: next2.getNoFitPanels().add(tileDimensions);");

            let mut failed_solution = solution.clone();
            failed_solution.add_no_fit_tile(tile.clone());
            new_solutions.push(failed_solution);
        }
    }

    log_debug!(
        "RUST ЭТАП 2 ЗАВЕРШЕН: Создано {} новых решений",
        new_solutions.len()
    );

    Ok(new_solutions)
}

fn try_place_tile_simple(
    tile: &TileDimensions,
    mosaic: &Mosaic,
) -> Result<Option<Mosaic>, Box<dyn std::error::Error>> {
    
    println!("RUST try_place_tile_simple_debug(): === Размещение панели ===");
    println!("RUST: Пытаемся разместить панель {}x{} (ID: {})", tile.width, tile.height, tile.id);
    println!("JAVA аналог: попытка размещения в мозаике");
    
    // Пока что просто создаем копию мозаики и помечаем панель как размещенную
    let mut new_mosaic = mosaic.clone();
    
    // Создаем финальный узел для панели (упрощенная версия)
    let mut root_node = new_mosaic.root_tile_node().clone();
    root_node.set_external_id(Some(tile.id as i32));
    root_node.set_final(true);
    root_node.set_rotated(tile.is_rotated);
    
    new_mosaic.set_root_tile_node(root_node);
    
    println!("RUST: Панель размещена как финальная в корневом узле");
    println!("JAVA аналог: tileNodeFindTile.setExternalId(tileDimensions.getId());");
    println!("JAVA аналог: tileNodeFindTile.setFinal(true);");
    println!("RUST try_place_tile_simple_debug(): Размещение успешно");
    
    Ok(Some(new_mosaic))
}