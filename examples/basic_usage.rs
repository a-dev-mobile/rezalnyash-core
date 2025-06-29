use rayon::vec;
use rezalnyas_core::{
    enums::{
        cut_orientation_preference::CutOrientationPreference,
        optimization_priority::OptimizationPriority, status::Status,
    }, log_debug, log_error, log_info, log_warn, logging::{init_logging, LogConfig, LogLevel}, models::{
        calculation_request::CalculationRequest, configuration::Configuration, grouped_tile_dimensions::{get_distinct_grouped_tile_dimensions, GroupedTileDimensions}, panel::structs::Panel, performance_thresholds::PerformanceThresholds, task::structs::Task, tile::tile_conversion::grouped_tile_dimensions_list_to_tile_dimensions_list, tile_dimensions::{count_duplicate_permutations, generate_groups, generate_groups_improved, generate_groups_java_compatible, remove_duplicated_permutations, remove_duplicated_permutations_java_compatible, TileDimensions}
    }, scaled_math::{PrecisionAnalyzer, ScaledConverter, ScaledNumber}, services::arrangement::generate_permutations, CutListOptimizerService, CuttingRequest, Material, OptimizationConfig, OptimizationStrategy
};

const MAX_ALLOWED_DIGITS: u8 = 6;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut unique_tile_id = 1u8;
    println!("🐛 DEBUG MODE: Single-threaded optimization");
    println!(
        "💻 Available cores: {}",
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
    );

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
    let stock_panels: Vec<Panel> = vec![Panel::new(
        1,
        "400.0".to_string(),
        "300.0".to_string(),
        1,
    )];

    let config = Configuration {
        cut_thickness: 0.0,
        use_single_stock_unit: false,
        optimization_factor: 2,
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

    println!("Создан запрос:");
    println!("- Деталей: {}", request.panels.len());
    println!("- Заготовка: {}x{}", 
        request.stock_panels[0].width, 
        request.stock_panels[0].height);

    // Вычисление точности
    let panels = &request.panels;
    let stock_panels = &request.stock_panels;
    let configuration = &request.configuration;

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

    // Создание конвертера с определенной точностью
    let converter = ScaledConverter::new(final_precision)?;
    // dPow - scale_factor
    let scale_factor = 10_i64.pow(final_precision as u32);
    // Создание списков для результатов
    let mut tiles = Vec::new();
    let mut stock_tiles = Vec::new();
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

            tiles.push(tile);
           unique_tile_id += 1; // Увеличиваем ID для следующей детали
        }
    }

    // Обработка складских панелей (stock tiles)
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

            stock_tiles.push(stock_tile);
        }
    }
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
        log_debug!("... and {} more permutations", list_generate_permutations.len() - 3);
    }



    /*
     * Преобразование перестановок в списки панелей
     * 
     * Преобразуем каждую перестановку групп обратно в последовательность отдельных панелей
     * Теперь sorted_tile_lists содержит различные порядки размещения панелей
     */
    
    log_debug!("Task[{}] Sorting tiles according to permutations...", task.id);
    

    let mut sorted_tile_lists: Vec<Vec<TileDimensions>> = Vec::new();
    
    for permutation in &list_generate_permutations {
        let sorted_tiles = grouped_tile_dimensions_list_to_tile_dimensions_list(
            permutation,
            &list_generate_groups
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
        log_debug!("... and {} more sorted tile lists", sorted_tile_lists.len() - 2);
    }


    /*
     * Удаление дублирующих перестановок
     * 
     * Удаляем перестановки, которые приводят к одинаковым результатам
     * Это экономит время вычислений
     */
    
    log_debug!("Task[{}] Removing duplicated permutations...", task.id);
    

    // Сначала подсчитаем дубликаты для статистики
    let (total_before, unique_before, duplicates_before) = count_duplicate_permutations(&sorted_tile_lists);
    
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
            log_debug!("Task[{}] High deduplication efficiency - significant optimization achieved", task.id);
        }
    }

    log_info!("\n=== Алгоритм выполнен успешно ===");
    log_info!("Обработано {} деталей", tiles.len());
    log_info!("Создано {} вариантов размещения", sorted_tile_lists.len());

    Ok(())
}
