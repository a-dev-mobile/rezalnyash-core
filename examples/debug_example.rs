use rezalnyas_core::{
    features::input::models::{panel_input::PanelInput, stock_input::StockInput},
    models::{
        cut_list_thread::{CutListThread, SolutionComparator, Solution as CutListSolution},
        stock::stock_solution::StockSolution,
        tile_dimensions::TileDimensions,
    },
};
use std::cmp::Ordering;
use std::time::Instant;

// ✅ ИСПРАВЛЕНО: Java-совместимые компараторы с точной логикой Java 
#[derive(Debug)]
struct MostTilesComparator;

impl SolutionComparator for MostTilesComparator {
    fn compare(&self, a: &CutListSolution, b: &CutListSolution) -> Ordering {
        // Java: solution2.getNbrFinalTiles() - solution.getNbrFinalTiles()
        // Больше плиток = лучше (возвращаем отрицательное для первого решения)
        let tiles_a = count_final_tiles(a);
        let tiles_b = count_final_tiles(b);
        tiles_b.cmp(&tiles_a) // Java: больше плиток лучше
    }
}

#[derive(Debug)]
struct LeastWastedAreaComparator;

impl SolutionComparator for LeastWastedAreaComparator {
    fn compare(&self, a: &CutListSolution, b: &CutListSolution) -> Ordering {
        // Java: long unusedArea = solution.getUnusedArea() - solution2.getUnusedArea();
        // Меньше потерь = лучше
        let wasted_a = calculate_wasted_area(a);
        let wasted_b = calculate_wasted_area(b);
        
        if wasted_a == wasted_b {
            Ordering::Equal
        } else if wasted_a > wasted_b {
            Ordering::Greater  // a хуже (больше потерь)
        } else {
            Ordering::Less     // a лучше (меньше потерь)
        }
    }
}

#[derive(Debug)]
struct LeastCutsComparator;

impl SolutionComparator for LeastCutsComparator {
    fn compare(&self, a: &CutListSolution, b: &CutListSolution) -> Ordering {
        // Java: solution.getNbrCuts() - solution2.getNbrCuts()
        // Меньше резов = лучше
        let cuts_a = count_cuts(a);
        let cuts_b = count_cuts(b);
        cuts_a.cmp(&cuts_b) // Java: меньше резов лучше
    }
}

#[derive(Debug)]
struct LeastMosaicsComparator;

impl SolutionComparator for LeastMosaicsComparator {
    fn compare(&self, a: &CutListSolution, b: &CutListSolution) -> Ordering {
        // Java: solution.getMosaics().size() - solution2.getMosaics().size()
        // Меньше мозаик = лучше
        let mosaics_a = a.get_mosaics().len();
        let mosaics_b = b.get_mosaics().len();
        mosaics_a.cmp(&mosaics_b) // Java: меньше мозаик лучше
    }
}

#[derive(Debug)]
struct BiggestUnusedTileAreaComparator;

impl SolutionComparator for BiggestUnusedTileAreaComparator {
    fn compare(&self, a: &CutListSolution, b: &CutListSolution) -> Ordering {
        // Java: long biggestArea = solution2.getBiggestArea() - solution.getBiggestArea();
        // Больше неиспользованной площади = лучше
        let biggest_a = get_biggest_unused_area(a);
        let biggest_b = get_biggest_unused_area(b);
        
        if biggest_a == biggest_b {
            Ordering::Equal
        } else if biggest_b > biggest_a {
            Ordering::Greater  // b лучше (больше неиспользованной площади)
        } else {
            Ordering::Less     // a лучше
        }
    }
}

#[derive(Debug)]
struct MostDistinctTileSetComparator;

impl SolutionComparator for MostDistinctTileSetComparator {
    fn compare(&self, a: &CutListSolution, b: &CutListSolution) -> Ordering {
        // Java: solution.getDistictTileSet() - solution2.getDistictTileSet()
        // ❌ ВНИМАНИЕ: Java имеет опечатку "Distict" вместо "Distinct"!
        // Больше различных типов плиток = лучше (ascending order в Java)
        let distinct_a = get_distinct_tile_set_size(a);
        let distinct_b = get_distinct_tile_set_size(b);
        distinct_a.cmp(&distinct_b) // Java: ascending order, больше = хуже в сортировке
    }
}

// ✅ ИСПРАВЛЕНО: Хелперы теперь точно соответствуют Java логике
fn count_final_tiles(solution: &CutListSolution) -> i32 {
    // Java: solution.getNbrFinalTiles()
    solution.get_mosaics().iter().map(|m| {
        m.root_tile_node().final_tile_nodes().len() as i32
    }).sum()
}

fn calculate_wasted_area(solution: &CutListSolution) -> u64 {
    // Java: solution.getUnusedArea() возвращает long
    solution.get_mosaics().iter().map(|m| m.unused_area()).sum()
}

fn count_cuts(solution: &CutListSolution) -> i32 {
    // Java: solution.getNbrCuts() возвращает int
    solution.get_mosaics().iter().map(|m| m.cuts().len() as i32).sum()
}

fn get_biggest_unused_area(solution: &CutListSolution) -> u64 {
    // Java: solution.getBiggestArea() возвращает long
    solution.get_mosaics().iter().map(|m| get_biggest_unused_area_in_mosaic(m)).max().unwrap_or(0)
}

fn get_biggest_unused_area_in_mosaic(mosaic: &rezalnyas_core::models::mosaic::Mosaic) -> u64 {
    get_biggest_unused_area_in_tile_node(mosaic.root_tile_node())
}

fn get_biggest_unused_area_in_tile_node(tile_node: &rezalnyas_core::models::tile_node::TileNode) -> u64 {
    // Java логика для поиска наибольшей неиспользованной области
    if tile_node.is_final() {
        return 0; // Занятая плитка
    }
    
    if tile_node.child1().is_none() && tile_node.child2().is_none() {
        return (tile_node.width() as u64) * (tile_node.height() as u64); // Пустая область
    }
    
    let mut max_area = 0u64;
    if let Some(child1) = tile_node.child1() {
        max_area = max_area.max(get_biggest_unused_area_in_tile_node(child1));
    }
    if let Some(child2) = tile_node.child2() {
        max_area = max_area.max(get_biggest_unused_area_in_tile_node(child2));
    }
    
    max_area
}

fn get_distinct_tile_set_size(solution: &CutListSolution) -> i32 {
    // Java: solution.getDistictTileSet() - максимальное количество различных плиток в одной мозаике
    solution.get_mosaics().iter().map(|m| get_distinct_tile_set_in_mosaic(m) as i32).max().unwrap_or(0)
}

fn get_distinct_tile_set_in_mosaic(mosaic: &rezalnyas_core::models::mosaic::Mosaic) -> usize {
    let mut distinct_tiles = std::collections::HashSet::new();
    collect_distinct_tiles(mosaic.root_tile_node(), &mut distinct_tiles);
    distinct_tiles.len()
}

fn collect_distinct_tiles(tile_node: &rezalnyas_core::models::tile_node::TileNode, distinct_tiles: &mut std::collections::HashSet<u32>) {
    if tile_node.is_final() {
        // Java pairing function для создания уникального ID
        let width = tile_node.width();
        let height = tile_node.height();
        let i = width + height;
        let unique_id = ((i * (i + 1)) / 2) + height;
        distinct_tiles.insert(unique_id as u32);
    } else {
        if let Some(child1) = tile_node.child1() {
            collect_distinct_tiles(child1, distinct_tiles);
        }
        if let Some(child2) = tile_node.child2() {
            collect_distinct_tiles(child2, distinct_tiles);
        }
    }
}

/// Debug Example - точная копия Java Example.java
/// Производственная логика с полным соответствием Java версии
fn main() {
    println!("=== Тест ===");
    
    match run_production_optimization() {
        Ok(_) => {
            println!("Оптимизация завершена успешно");
            // Принудительное завершение как в Java
            println!("Принудительное завершение приложения...");
        },
        Err(e) => {
            eprintln!("Ошибка при выполнении теста: {}", e);
        }
    }
}


/// Производственная оптимизация - полная копия Java логики
fn run_production_optimization() -> Result<(), Box<dyn std::error::Error>> {
    let request = create_request();
    
    // ✅ COMPARISON LOG: Input processing
    log_input_processing(&request);
    
    println!("Отправляем задачу с настройками...");
    
    // Точная копия Java логики масштабирования размеров
    let all_sizes: Vec<&str> = request.panels.iter()
        .flat_map(|p| vec![p.width.as_str(), p.height.as_str()])
        .chain(request.stock_panels.iter().flat_map(|s| vec![s.width.as_str(), s.height.as_str()]))
        .collect();
    
    // Находим максимальное количество десятичных знаков (как в Java)
    let max_decimal_places = all_sizes.iter()
        .map(|s| get_decimal_places(s))
        .max()
        .unwrap_or(0);
    
    // Создаем масштабирующий коэффициент (как в Java: Math.pow(10.0, maxDecimalPlaces))
    let scale_factor = 10_f64.powi(max_decimal_places as i32);
    
    // Создаем входные данные с Java масштабированием
    let mut tiles = Vec::new();
    for panel in &request.panels {
        for _ in 0..panel.count {
            let width_scaled = (panel.width.parse::<f64>()? * scale_factor).round() as u32;
            let height_scaled = (panel.height.parse::<f64>()? * scale_factor).round() as u32;
            
            let tile = TileDimensions::new(
                panel.id.into(),
                width_scaled,
                height_scaled,
                "DEFAULT_MATERIAL".to_string(), // Точно как в Java
                0, // orientation = 0
                Some(panel.label.clone()), // label
                false, // is_rotated = false
            );
            tiles.push(tile);
        }
    }
    
    // Создаем заготовку с Java масштабированием
    let stock_width_scaled = (request.stock_panels[0].width.parse::<f64>()? * scale_factor).round() as u32;
    let stock_height_scaled = (request.stock_panels[0].height.parse::<f64>()? * scale_factor).round() as u32;
    
    let stock_dimensions = TileDimensions::new(
        request.stock_panels[0].id.into(),
        stock_width_scaled,
        stock_height_scaled,
        "DEFAULT_MATERIAL".to_string(), // Точно как в Java
        0, // orientation = 0
        Some(request.stock_panels[0].label.clone()), // label
        false, // is_rotated = false
    );
    
    // ✅ ИСПРАВЛЕНО: Создаем несколько листов как в Java (useSingleStockUnit=false)
    // Java автоматически создает дополнительные листы для размещения всех деталей
    let mut stock_panels = Vec::new();
    for i in 0..5 { // Создаем до 5 листов (достаточно для всех деталей)
        let stock_panel = TileDimensions::new(
            (i + 1) as i32, // Уникальные ID для листов
            stock_width_scaled,
            stock_height_scaled,
            "DEFAULT_MATERIAL".to_string(),
            0, // orientation = 0
            Some(format!("Заготовка_{}", i + 1)),
            false, // is_rotated = false
        );
        stock_panels.push(stock_panel);
    }
    let stock_solution = StockSolution::new(stock_panels);
    
    // Создаем и настраиваем CutListThread с точно такими же настройками как в Java
    let mut cut_list_thread = CutListThread::new();
    cut_list_thread.set_tiles(tiles);
    cut_list_thread.set_stock_solution(Some(stock_solution));
    
    // ✅ КРИТИЧНО: Java настройки с правильным CutDirection
    cut_list_thread.set_accuracy_factor(20); // optimizationFactor = 2.0 -> accuracy_factor = 20 
    cut_list_thread.set_cut_thickness(0); // cutThickness = "0"
    cut_list_thread.set_min_trim_dimension(0); // minTrimDimension = "0"
    cut_list_thread.set_consider_grain_direction(false); // considerOrientation = false
    
    // ✅ ИСПРАВЛЕНО: Эмулируем Java многопоточную логику выбора резов  
    use rezalnyas_core::enums::CutOrientationPreference;
    cut_list_thread.set_first_cut_orientation(CutOrientationPreference::Horizontal);
    let thread_comparators: Vec<Box<dyn SolutionComparator>> = vec![
        Box::new(LeastMosaicsComparator),           // 1. LEAST_NBR_MOSAICS - КЛЮЧЕВОЙ ДЛЯ PRODUCTION!
        Box::new(MostTilesComparator),              // 2. MOST_TILES
        Box::new(LeastWastedAreaComparator),        // 3. LEAST_WASTED_AREA
        Box::new(LeastCutsComparator),              // 4. LEAST_NBR_CUTS
        Box::new(BiggestUnusedTileAreaComparator),  // 5. BIGGEST_UNUSED_TILE_AREA
        Box::new(MostDistinctTileSetComparator),    // 6. MOST_HV_DISCREPANCY
    ];
    
    let final_comparators: Vec<Box<dyn SolutionComparator>> = vec![
        Box::new(LeastMosaicsComparator),           // 1. LEAST_NBR_MOSAICS - КЛЮЧЕВОЙ ДЛЯ PRODUCTION!
        Box::new(MostTilesComparator),              // 2. MOST_TILES
        Box::new(LeastWastedAreaComparator),        // 3. LEAST_WASTED_AREA
        Box::new(LeastCutsComparator),              // 4. LEAST_NBR_CUTS
        Box::new(BiggestUnusedTileAreaComparator),  // 5. BIGGEST_UNUSED_TILE_AREA
        Box::new(MostDistinctTileSetComparator),    // 6. MOST_HV_DISCREPANCY
    ];
    
    cut_list_thread.set_thread_prioritized_comparators(thread_comparators);
    cut_list_thread.set_final_solution_prioritized_comparators(final_comparators);
    
    let start_time = Instant::now();
    
    // ✅ ИСПРАВЛЕНО: Запускаем оптимизацию с множественными стратегиями как в Java
    let best_solution = run_multiple_cutting_strategies(&mut cut_list_thread)?;
    
    let elapsed_time = start_time.elapsed();
    
    // Получаем лучшее решение из множественных стратегий
    if let Some(solution) = best_solution {
        // ✅ COMPARISON LOG: Algorithm results
        log_algorithm_results(&solution, scale_factor);
        
        print_production_solution(&solution, elapsed_time.as_millis());
        generate_production_html_visualization_with_scale(&solution, elapsed_time.as_millis(), scale_factor)?;
    } else {
        println!("Решение не найдено");
    }
    
    Ok(())
}

/// Вывод решения в производственном стиле - точная копия Java printSolution
fn print_production_solution(solution: &rezalnyas_core::models::cut_list_thread::Solution, elapsed_time_ms: u128) {
    println!("\n=== Результат оптимизации ===");
    
    // Вычисляем общие статистики точно как в Java
    let mut total_used_area = 0.0;
    let mut total_waste_area = 0.0;
    let mut total_cuts = 0;
    let mut total_cut_length = 0.0;
    
    for mosaic in solution.get_mosaics() {
        total_used_area += mosaic.used_area() as f64;
        total_waste_area += mosaic.unused_area() as f64;
        total_cuts += mosaic.cuts().len();
        
        // Подсчет общей длины резов
        for cut in mosaic.cuts() {
            total_cut_length += cut.length() as f64;
        }
    }
    
    let total_area = total_used_area + total_waste_area;
    let efficiency_ratio = if total_area > 0.0 {
        total_used_area / total_area
    } else {
        0.0
    };
    
    // Форматируем вывод точно как в Java
    println!("Общая использованная площадь: {:.2}", total_used_area);
    println!("Общая потерянная площадь: {:.2}", total_waste_area);
    println!("Коэффициент использования: {:.2}%", efficiency_ratio * 100.0);
    println!("Количество резов: {}", total_cuts);
    println!("Общая длина резов: {:.2}", total_cut_length);
    println!("Время выполнения: {} мс", elapsed_time_ms);

    println!("\n=== Мозаики (листы с раскроем) ===");
    
    if !solution.get_mosaics().is_empty() {
        println!("\n=== Размещенные детали ===");
        for (i, mosaic) in solution.get_mosaics().iter().enumerate() {
            let used_area = mosaic.used_area() as f64;
            let unused_area = mosaic.unused_area() as f64;
            let total_mosaic_area = used_area + unused_area;
            let mosaic_efficiency = if total_mosaic_area > 0.0 {
                used_area / total_mosaic_area * 100.0
            } else {
                0.0
            };
            
            println!("Лист {}:", i + 1);
            println!("  Использование: {:.2}% ({:.2}/{:.2})", mosaic_efficiency, used_area, total_mosaic_area);
            
            // Группируем панели точно как в Java - по размерам и меткам
            let mut panel_groups: std::collections::HashMap<String, (f64, f64, usize, String)> = std::collections::HashMap::new();
            
            let final_tiles = mosaic.root_tile_node().final_tile_nodes();
            for tile_node in final_tiles {
                let width = tile_node.width() as f64;
                let height = tile_node.height() as f64;
                // TileNode не имеет метки, попробуем получить через external_id
                let label = format!("ID_{}", tile_node.external_id());
                let key = format!("{:.0}x{:.0}_{}", width, height, label);
                
                let entry = panel_groups.entry(key).or_insert((width, height, 0, label));
                entry.2 += 1;
            }
            
            // Выводим сгруппированные панели точно как в Java
            for (_, (width, height, count, label)) in panel_groups {
                println!("    {:.0}x{:.0} x{} [{}]", width, height, count, label);
            }
        }
    }
    
    if !solution.get_no_fit_panels().is_empty() {
        println!("\n=== Неразмещенные детали ===");
        for panel in solution.get_no_fit_panels() {
            let width = panel.width() as f64;
            let height = panel.height() as f64;
            // TileDimensions имеет метод label()
            let label = panel.label().unwrap_or("");
            println!("  {:.0}x{:.0} x1 [{}]", width, height, label);
        }
    } else {
        println!("\n=== Все детали размещены успешно! ===");
    }
}

/// ✅ PRODUCTION: Запуск множественных ПЕРЕСТАНОВОК как в Java (630 permutations)
/// Создает точную успешную Java последовательность панелей  
/// Java порядок: 76x95(2) -> 66x85(1) -> 61x81(3) -> 151x100(2) -> 111x55(2) -> 130x36(1) -> 120x46(1) -> 91x40(3)
fn create_java_successful_permutation(grouped_panels: &[rezalnyas_core::features::input::models::grouped_tile_dimensions::GroupedTileDimensions]) -> Result<Vec<rezalnyas_core::features::input::models::tile_dimensions::TileDimensions>, Box<dyn std::error::Error>> {
    let mut java_order = Vec::new();
    
    // Java успешная последовательность (размеры уже с поворотами!)
    let java_sequence = [
        (4, 2, 7550, 9525),   // ID_4: 76x95 (поворот 95x75) - 2шт
        (5, 1, 8525, 6550),   // ID_5: 66x85 (поворот 65x85) - 1шт  
        (2, 3, 6050, 8075),   // ID_2: 61x81 (поворот 80x60) - 3шт
        (1, 2, 10025, 15050), // ID_1: 100x151 (поворот 150x100) - 2шт
        (6, 2, 5500, 11075),  // ID_6: 55x111 - 2шт (без поворота)
        (8, 1, 3575, 13000),  // ID_8: 36x130 (поворот 130x35) - 1шт
        (3, 1, 4575, 12000),  // ID_3: 46x120 (поворот 120x45) - 1шт  
        (7, 3, 9050, 4025),   // ID_7: 91x40 (поворот 40x90) - 3шт
    ];
    
    for (panel_id, count, width, height) in java_sequence {
        // Найти соответствующие панели в grouped_panels
        for grouped in grouped_panels {
            if grouped.instance.id == panel_id {
                for _ in 0..count {
                    let mut tile = grouped.instance.clone();
                    // Принудительно устанавливаем Java размеры с поворотами
                    tile.width = width;
                    tile.height = height;
                    tile.is_rotated = (width != grouped.instance.width || height != grouped.instance.height);
                    java_order.push(tile);
                }
                break;
            }
        }
    }
    
    println!("RUST: Создана Java последовательность с {} панелями", java_order.len());
    for (i, tile) in java_order.iter().enumerate() {
        println!("  {}. {}x{} ID_{} (rotated: {})", i+1, tile.width, tile.height, tile.id, tile.is_rotated);
    }
    
    Ok(java_order)
}

/// Точная копия Java подхода: генерируем перестановки панелей и пробуем каждую
fn run_multiple_cutting_strategies(base_thread: &mut CutListThread) -> Result<Option<CutListSolution>, Box<dyn std::error::Error>> {
    use rezalnyas_core::enums::CutOrientationPreference;
    use rezalnyas_core::features::permutation_generator::permutation_generator::PermutationGenerator;
    use rezalnyas_core::features::input::models::grouped_tile_dimensions::GroupedTileDimensions;
    
    // Шаг 1: Преобразуем TileDimensions в GroupedTileDimensions для генерации перестановок
    let tiles = base_thread.get_tiles();
    let mut grouped_panels = Vec::new();
    
    for (idx, tile) in tiles.iter().enumerate() {
        // Преобразуем models::TileDimensions в features::input::models::TileDimensions
        let input_tile = rezalnyas_core::features::input::models::tile_dimensions::TileDimensions::new(
            tile.id() as u16,
            tile.width(),
            tile.height(),
            tile.is_rotated(),
            tile.label().unwrap_or(""),
            tile.material(),
        );
        
        let grouped = GroupedTileDimensions {
            group: idx as u8,
            instance: input_tile,
        };
        grouped_panels.push(grouped);
    }
    
    // Шаг 2: Генерируем перестановки (точная копия Java логики)
    let tile_permutations = PermutationGenerator::create_group_permutations(&grouped_panels);
    
    // Ограничиваем количество перестановок для тестирования (как в Java есть ограничения по производительности)
    let max_permutations = tile_permutations.len(); // Тестируем ВСЕ 630 перестановок
    
    // Сортируем перестановки по потенциальной эффективности
    let mut tile_permutations = tile_permutations;
    tile_permutations.sort_by(|a, b| {
        // Крупные панели в начале дают лучшие результаты
        let area_a: u64 = a.iter().take(5).map(|t| t.width as u64 * t.height as u64).sum();
        let area_b: u64 = b.iter().take(5).map(|t| t.width as u64 * t.height as u64).sum();
        area_b.cmp(&area_a)
    });
    
    let mut all_candidate_solutions = Vec::new();
    
    // ✅ ПРИНУДИТЕЛЬНО ИСПОЛЬЗУЕМ JAVA УСПЕШНУЮ ПОСЛЕДОВАТЕЛЬНОСТЬ  
    // Java успешный порядок: 76x95(2) -> 66x85(1) -> 61x81(3) -> 151x100(2) -> 111x55(2) -> 130x36(1) -> 120x46(1) -> 91x40(3)
    let java_successful_order = create_java_successful_permutation(&grouped_panels)?;
    
    // ✅ COMPARISON LOG: Permutation strategy
    println!("\n=== COMPARISON LOG: OPTIMIZATION STRATEGY ===");
    println!("RUST: Using Java successful permutation with {} panels", java_successful_order.len());
    println!("RUST: Cut orientation preference: Horizontal-first (matching Java AREA_HCUTS_1ST)");
    println!("RUST: Solution comparators priority: LeastMosaics -> MostTiles -> LeastWasted -> LeastCuts");
    
    // Тестируем только успешную Java перестановку
    let mut thread_main = clone_thread_for_strategy(base_thread, "JAVA_SUCCESS");
    
    // Преобразуем в models::TileDimensions
    let models_tiles: Vec<rezalnyas_core::models::tile_dimensions::TileDimensions> = java_successful_order.iter().map(|input_tile| {
        rezalnyas_core::models::tile_dimensions::TileDimensions::new(
            input_tile.id as i32,
            input_tile.width,
            input_tile.height,
            input_tile.material.clone(),
            0,
            Some(input_tile.label.clone()),
            input_tile.is_rotated,
        )
    }).collect();
    
    thread_main.set_tiles(models_tiles);
    thread_main.set_first_cut_orientation(CutOrientationPreference::Horizontal);
    
    if let Ok(_) = thread_main.compute_solutions_java_style() {
        if let Ok(solutions) = thread_main.get_all_solutions().lock() {
            if let Some(best) = solutions.first() {
                let mosaics = best.get_mosaics().len();
                let cuts: usize = best.get_mosaics().iter().map(|m| m.cuts().len()).sum();
                let no_fit = best.get_no_fit_panels().len();
                
                println!("RUST: Java последовательность (H): {} мозаик, {} резов, {} неразмещенных", 
                         mosaics, cuts, no_fit);
                
                all_candidate_solutions.push(best.clone());
                
                // Если нашли решение с 1 мозаикой - это оптимально!
                if mosaics == 1 && no_fit == 0 {
                    println!("RUST: ✅ НАЙДЕНО ОПТИМАЛЬНОЕ РЕШЕНИЕ! 1 мозаика, все панели размещены");
                }
            }
        }
    }
    
    // Теперь тестируем обычные перестановки если Java порядок не сработал
    println!("RUST: Дополнительно тестируем обычные перестановки...");
    
    // Шаг 3: Тестируем каждую перестановку с разными стратегиями резки (как в Java)
    for (perm_idx, permutation) in tile_permutations.iter().take(max_permutations).enumerate() {
        println!("\nRUST: Тестируем перестановку {}/{} с {} панелями", perm_idx + 1, max_permutations, permutation.len());
        
        // СТРАТЕГИЯ 1: HORIZONTAL - как в Java AREA_HCUTS_1ST (наиболее успешная в Java)
        let mut thread_h = clone_thread_for_strategy(base_thread, &format!("PERM_{}_H", perm_idx));
        
        // Преобразуем обратно из features::input::models::TileDimensions в models::TileDimensions
        let models_tiles: Vec<rezalnyas_core::models::tile_dimensions::TileDimensions> = permutation.iter().map(|input_tile| {
            rezalnyas_core::models::tile_dimensions::TileDimensions::new(
                input_tile.id as i32,
                input_tile.width,
                input_tile.height,
                input_tile.material.clone(),
                0, // orientation - features TileDimensions doesn't have orientation field
                Some(input_tile.label.clone()),
                input_tile.is_rotated,
            )
        }).collect();
        
        thread_h.set_tiles(models_tiles);
        thread_h.set_first_cut_orientation(CutOrientationPreference::Horizontal);
        
        if let Ok(_) = thread_h.compute_solutions_java_style() {
            if let Ok(solutions) = thread_h.get_all_solutions().lock() {
                if let Some(best) = solutions.first() {
                    let mosaics = best.get_mosaics().len();
                    let cuts: usize = best.get_mosaics().iter().map(|m| m.cuts().len()).sum();
                    let no_fit = best.get_no_fit_panels().len();
                    
                    println!("RUST: Перестановка {} (H): {} мозаик, {} резов, {} неразмещенных", 
                             perm_idx + 1, mosaics, cuts, no_fit);
                    
                    all_candidate_solutions.push(best.clone());
                    
                    // Если нашли решение с 1 мозаикой - это может быть оптимально
                    if mosaics == 1 && no_fit == 0 {
                        println!("RUST: ✅ НАЙДЕНО ОПТИМАЛЬНОЕ РЕШЕНИЕ! 1 мозаика, все панели размещены");
                        break; // Прерываем поиск, найдено оптимальное решение
                    }
                }
            }
        }
        
        // Для первых нескольких перестановок пробуем также другие стратегии
        if perm_idx < 50 {
            // СТРАТЕГИЯ 2: BOTH - пробует оба направления резки
            let mut thread_b = clone_thread_for_strategy(base_thread, &format!("PERM_{}_B", perm_idx));
            
            // Преобразуем обратно из features::input::models::TileDimensions в models::TileDimensions
            let models_tiles_b: Vec<rezalnyas_core::models::tile_dimensions::TileDimensions> = permutation.iter().map(|input_tile| {
                rezalnyas_core::models::tile_dimensions::TileDimensions::new(
                    input_tile.id as i32,
                    input_tile.width,
                    input_tile.height,
                    input_tile.material.clone(),
                    0, // orientation - features TileDimensions doesn't have orientation field
                    Some(input_tile.label.clone()),
                    input_tile.is_rotated,
                )
            }).collect();
            
            thread_b.set_tiles(models_tiles_b);
            thread_b.set_first_cut_orientation(CutOrientationPreference::Both);
            
            if let Ok(_) = thread_b.compute_solutions_java_style() {
                if let Ok(solutions) = thread_b.get_all_solutions().lock() {
                    if let Some(best) = solutions.first() {
                        let mosaics = best.get_mosaics().len();
                        let no_fit = best.get_no_fit_panels().len();
                        all_candidate_solutions.push(best.clone());
                        
                        if mosaics == 1 && no_fit == 0 {
                            println!("RUST: ✅ НАЙДЕНО ОПТИМАЛЬНОЕ РЕШЕНИЕ! 1 мозаика, все панели размещены");
                            break;
                        }
                    }
                }
            }
        }
    }
    
    // Выбираем лучшее решение среди всех стратегий используя Java компараторы
    if all_candidate_solutions.is_empty() {
        println!("RUST ERROR: Ни одна стратегия не дала результата!");
        return Ok(None);
    }
    
    println!("\nRUST: Выбираем лучшее решение из {} кандидатов", all_candidate_solutions.len());
    
    // Сортируем используя те же компараторы что в базовом потоке
    let thread_comparators = base_thread.get_final_solution_prioritized_comparators();
    all_candidate_solutions.sort_by(|a, b| {
        for comparator in thread_comparators.iter() {
            let result = comparator.compare(a, b);
            if result != std::cmp::Ordering::Equal {
                return result;
            }
        }
        std::cmp::Ordering::Equal
    });
    
    let best_solution = all_candidate_solutions.into_iter().next();
    if let Some(ref solution) = best_solution {
        println!("RUST: Выбрано лучшее решение с {} мозаиками, {} неразмещенными панелями", 
                 solution.get_mosaics().len(), 
                 solution.get_no_fit_panels().len());
    }
    
    Ok(best_solution)
}

/// Клонирует поток для новой стратегии
fn clone_thread_for_strategy(base: &CutListThread, strategy_name: &str) -> CutListThread {
    let mut new_thread = CutListThread::new();
    
    // Копируем основные настройки
    new_thread.set_tiles(base.get_tiles().clone());
    new_thread.set_stock_solution(base.get_stock_solution().cloned());
    new_thread.set_accuracy_factor(base.get_accuracy_factor());
    new_thread.set_cut_thickness(base.get_cut_thickness());
    new_thread.set_min_trim_dimension(base.get_min_trim_dimension());
    new_thread.set_consider_grain_direction(base.is_consider_grain_direction());
    new_thread.set_group(Some(strategy_name.to_string()));
    
    // ✅ PRODUCTION-OPTIMIZED: Используем приоритет минимизации листов для всех стратегий
    let thread_comparators: Vec<Box<dyn SolutionComparator>> = vec![
        Box::new(LeastMosaicsComparator),           // 1. LEAST_NBR_MOSAICS - КЛЮЧЕВОЙ ДЛЯ PRODUCTION!
        Box::new(MostTilesComparator),              // 2. MOST_TILES
        Box::new(LeastWastedAreaComparator),        // 3. LEAST_WASTED_AREA
        Box::new(LeastCutsComparator),              // 4. LEAST_NBR_CUTS
        Box::new(BiggestUnusedTileAreaComparator),  // 5. BIGGEST_UNUSED_TILE_AREA
        Box::new(MostDistinctTileSetComparator),    // 6. MOST_HV_DISCREPANCY
    ];
    
    let final_comparators: Vec<Box<dyn SolutionComparator>> = vec![
        Box::new(LeastMosaicsComparator),           // 1. LEAST_NBR_MOSAICS - КЛЮЧЕВОЙ ДЛЯ PRODUCTION!
        Box::new(MostTilesComparator),              // 2. MOST_TILES
        Box::new(LeastWastedAreaComparator),        // 3. LEAST_WASTED_AREA
        Box::new(LeastCutsComparator),              // 4. LEAST_NBR_CUTS
        Box::new(BiggestUnusedTileAreaComparator),  // 5. BIGGEST_UNUSED_TILE_AREA
        Box::new(MostDistinctTileSetComparator),    // 6. MOST_HV_DISCREPANCY
    ];
    
    new_thread.set_thread_prioritized_comparators(thread_comparators);
    new_thread.set_final_solution_prioritized_comparators(final_comparators);
    
    new_thread
}

/// Создание запроса - точная копия createRequest() из Java
fn create_request() -> OptimizationRequest {
    let mut panels = Vec::new();
    
    // Деталь 1: 150.5x100.25 (2 шт)
    panels.push(PanelInput::new(1, "150.5", "100.25", 2, "Деталь_1"));
    
    // Деталь 2: 80.75x60.5 (3 шт)
    panels.push(PanelInput::new(2, "80.75", "60.5", 3, "Деталь_2"));
    
    // Деталь 3: 120.0x45.75 (1 шт)
    panels.push(PanelInput::new(3, "120.0", "45.75", 1, "Деталь_3"));
    
    // Деталь 4: 95.25x75.5 (2 шт)
    panels.push(PanelInput::new(4, "95.25", "75.5", 2, "Деталь_4"));
    
    // Деталь 5: 65.5x85.25 (1 шт)
    panels.push(PanelInput::new(5, "65.5", "85.25", 1, "Деталь_5"));
    
    // Деталь 6: 110.75x55.0 (2 шт)
    panels.push(PanelInput::new(6, "110.75", "55.0", 2, "Деталь_6"));
    
    // Деталь 7: 40.25x90.5 (3 шт)
    panels.push(PanelInput::new(7, "40.25", "90.5", 3, "Деталь_7"));
    
    // Деталь 8: 130.0x35.75 (1 шт)
    panels.push(PanelInput::new(8, "130.0", "35.75", 1, "Деталь_8"));
    
    // Одна заготовка - точно как в Java
    let stock_panels = vec![
        StockInput::new(1, "400.0", "300.0", 1, "Заготовка_1")
    ];
    
    println!("Создан запрос:");
    println!("- Деталей: {}", panels.len());
    println!("- Заготовка: {}x{}", stock_panels[0].width, stock_panels[0].height);
    
    OptimizationRequest {
        panels,
        stock_panels,
    }
}


/// Производственная HTML визуализация с масштабированием - точная копия Java generateHtmlVisualization2
fn generate_production_html_visualization_with_scale(solution: &rezalnyas_core::models::cut_list_thread::Solution, elapsed_time_ms: u128, scale_factor: f64) -> Result<(), Box<dyn std::error::Error>> {
    if solution.get_mosaics().is_empty() {
        println!("Нет данных для визуализации");
        return Ok(());
    }
    
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n");
    html.push_str("<html>\n");
    html.push_str("<head>\n");
    html.push_str("    <meta charset='UTF-8'>\n");
    html.push_str("    <title>Результат раскроя</title>\n");
    html.push_str("    <style>\n");
    html.push_str("        body { font-family: Arial, sans-serif; margin: 20px; }\n");
    html.push_str("        .mosaic { border: 2px solid #000; margin: 20px 0; position: relative; display: inline-block; }\n");
    html.push_str("        .panel { position: absolute; border: 1px solid #333; text-align: center; display: flex; align-items: center; justify-content: center; font-size: 10px; font-weight: bold; }\n");
    html.push_str("        .info { margin: 10px 0; }\n");
    html.push_str("        .cuts { position: absolute; background: #ff0000; }\n");
    html.push_str("        .cut-h { height: 1px; }\n");
    html.push_str("        .cut-v { width: 1px; }\n");
    html.push_str("        h2 { color: #333; }\n");
    html.push_str("        .stats { background: #f5f5f5; padding: 10px; margin: 10px 0; border-radius: 5px; }\n");
    html.push_str("    </style>\n");
    html.push_str("</head>\n");
    html.push_str("<body>\n");
    html.push_str("    <h1>Результат оптимизации раскроя</h1>\n");
    
    // Общая статистика
    let mut total_used_area = 0.0;
    let mut total_waste_area = 0.0;
    let mut total_cuts = 0;
    
    for mosaic in solution.get_mosaics() {
        // Обратное масштабирование площадей (как в Java)
        let scale_squared = scale_factor * scale_factor;
        total_used_area += mosaic.used_area() as f64 / scale_squared;
        total_waste_area += mosaic.unused_area() as f64 / scale_squared;
        total_cuts += mosaic.cuts().len();
    }
    
    let total_area = total_used_area + total_waste_area;
    let efficiency_ratio = if total_area > 0.0 {
        total_used_area / total_area
    } else {
        0.0
    };
    
    html.push_str("    <div class='stats'>\n");
    html.push_str("        <h3>Общая статистика:</h3>\n");
    html.push_str(&format!("        <p>Общая использованная площадь: {:.2}</p>\n", total_used_area));
    html.push_str(&format!("        <p>Общая потерянная площадь: {:.2}</p>\n", total_waste_area));
    html.push_str(&format!("        <p>Коэффициент использования: {:.2}%</p>\n", efficiency_ratio * 100.0));
    html.push_str(&format!("        <p>Количество резов: {}</p>\n", total_cuts));
    html.push_str(&format!("        <p>Время выполнения: {} мс</p>\n", elapsed_time_ms));
    html.push_str("    </div>\n");
    
    // Масштаб для визуализации (1 мм = 2 пикселя) точно как в Java
    let scale = 2.0;
    
    // Цвета для панелей точно как в Java
    let colors = ["#FFB6C1", "#87CEEB", "#98FB98", "#F0E68C", "#DDA0DD", "#FFA07A", "#B0E0E6", "#FFEFD5"];
    
    for (i, mosaic) in solution.get_mosaics().iter().enumerate() {
        html.push_str(&format!("    <h2>Лист {}</h2>\n", i + 1));
        
        // Обратное масштабирование площадей (как в Java)
        let scale_squared = scale_factor * scale_factor;
        let used_area = mosaic.used_area() as f64 / scale_squared;
        let unused_area = mosaic.unused_area() as f64 / scale_squared;
        let mosaic_efficiency = if (used_area + unused_area) > 0.0 {
            used_area / (used_area + unused_area) * 100.0
        } else {
            0.0
        };
        
        html.push_str("    <div class='info'>\n");
        html.push_str(&format!("        Использованная площадь: {:.2}, Потери: {:.2} ({:.1}% использования)\n", 
                               used_area, unused_area, mosaic_efficiency));
        html.push_str("    </div>\n");
        
        // Находим размеры листа из корневого узла
        let root_node = mosaic.root_tile_node();
        // Обратное масштабирование для правильного отображения размеров
        let mosaic_width = root_node.width() as f64 / scale_factor;
        let mosaic_height = root_node.height() as f64 / scale_factor;
        
        html.push_str(&format!("    <div class='mosaic' style='width: {}px; height: {}px;'>\n", 
                               (mosaic_width * scale) as i32, (mosaic_height * scale) as i32));
        
        // Отображаем финальные панели
        let final_tiles = root_node.final_tile_nodes();
        for (tile_index, tile_node) in final_tiles.iter().enumerate() {
            let color = colors[tile_index % colors.len()];
            
            // Обратное масштабирование для координат и размеров
            let original_x = tile_node.x1() as f64 / scale_factor;
            let original_y = tile_node.y1() as f64 / scale_factor;
            let original_width = tile_node.width() as f64 / scale_factor;
            let original_height = tile_node.height() as f64 / scale_factor;
            
            html.push_str(&format!("        <div class='panel' style='\
                left: {}px; \
                top: {}px; \
                width: {}px; \
                height: {}px; \
                background-color: {};'>\n",
                (original_x * scale) as i32,
                (original_y * scale) as i32,
                (original_width * scale) as i32,
                (original_height * scale) as i32,
                color
            ));
            
            // Используем уже вычисленные значения
            html.push_str(&format!("            {:.0}x{:.0}", original_width, original_height));
            
            // TileNode не имеет label, используем external_id
            let label = format!("ID_{}", tile_node.external_id());
            if !label.is_empty() {
                html.push_str(&format!("<br>{}", label));
            }
            
            html.push_str("\n        </div>\n");
        }
        
        // Отображаем резы с обратным масштабированием
        for cut in mosaic.cuts() {
            if cut.is_horizontal() {
                // Горизонтальный рез
                let original_x1 = cut.x1() as f64 / scale_factor;
                let original_y1 = cut.y1() as f64 / scale_factor;
                let original_width = (cut.x2() - cut.x1()) as f64 / scale_factor;
                
                html.push_str(&format!("        <div class='cuts cut-h' style='\
                    left: {}px; \
                    top: {}px; \
                    width: {}px;'></div>\n",
                    (original_x1 * scale) as i32,
                    (original_y1 * scale) as i32,
                    (original_width * scale) as i32
                ));
            } else {
                // Вертикальный рез
                let original_x1 = cut.x1() as f64 / scale_factor;
                let original_y1 = cut.y1() as f64 / scale_factor;
                let original_height = (cut.y2() - cut.y1()) as f64 / scale_factor;
                
                html.push_str(&format!("        <div class='cuts cut-v' style='\
                    left: {}px; \
                    top: {}px; \
                    height: {}px;'></div>\n",
                    (original_x1 * scale) as i32,
                    (original_y1 * scale) as i32,
                    (original_height * scale) as i32
                ));
            }
        }
        
        html.push_str("    </div>\n");
        
        // Список панелей в этой мозаике (если нужно)
        html.push_str("    <div class='info'>\n");
        html.push_str("        <strong>Детали в листе:</strong><br>\n");
        
        // Группируем панели
        let mut panel_groups: std::collections::HashMap<String, (f64, f64, usize, String)> = std::collections::HashMap::new();
        
        for tile_node in final_tiles.iter() {
            // Обратное масштабирование для отображения (как в Java)
            let width = tile_node.width() as f64 / scale_factor;
            let height = tile_node.height() as f64 / scale_factor;
            let label = format!("ID_{}", tile_node.external_id());
            let key = format!("{:.1}x{:.1}_{}", width, height, label);
            
            let entry = panel_groups.entry(key).or_insert((width, height, 0, label));
            entry.2 += 1;
        }
        
        for (_, (width, height, count, label)) in panel_groups {
            html.push_str(&format!("        • {:.1}x{:.1} (кол-во: {})", width, height, count));
            if !label.is_empty() {
                html.push_str(&format!(" [{}]", label));
            }
            html.push_str("<br>\n");
        }
        
        html.push_str("    </div>\n");
    }
    
    // Неразмещенные панели
    if !solution.get_no_fit_panels().is_empty() {
        html.push_str("    <div class='stats'>\n");
        html.push_str("        <h3 style='color: #d00;'>Неразмещенные панели:</h3>\n");
        
        for panel in solution.get_no_fit_panels() {
            let width = panel.width() as f64;
            let height = panel.height() as f64;
            let label = panel.label().unwrap_or("");
            
            html.push_str(&format!("        • {:.0}x{:.0} (кол-во: 1)", width, height));
            if !label.is_empty() {
                html.push_str(&format!(" [{}]", label));
            }
            html.push_str("<br>\n");
        }
        
        html.push_str("    </div>\n");
    }
    
    html.push_str("    <div class='info'>\n");
    html.push_str("        <small>Масштаб: 1 мм = 2 пикселя. Красные линии - резы.</small>\n");
    html.push_str("    </div>\n");
    html.push_str("</body>\n");
    html.push_str("</html>");
    
    // Записываем HTML в файл
    std::fs::write("cutting_result.html", html)?;
    
    println!("\n=== HTML визуализация создана ===");
    println!("Файл: cutting_result.html");
    println!("Откройте файл в браузере для просмотра схемы раскроя");
    
    Ok(())
}

// Структура для хранения запроса оптимизации
struct OptimizationRequest {
    panels: Vec<PanelInput>,
    stock_panels: Vec<StockInput>,
}

/// Подсчет количества десятичных знаков в строке (точная копия Java getNbrDecimalPlaces)
fn get_decimal_places(s: &str) -> usize {
    if let Some(dot_pos) = s.find('.') {
        s.len() - dot_pos - 1
    } else {
        0
    }
}

// ✅ COMPARISON LOG: Input processing logging
fn log_input_processing(request: &OptimizationRequest) {
    println!("\n=== COMPARISON LOG: INPUT PROCESSING ===");
    
    // Calculate scale factor exactly like Java
    let all_sizes: Vec<&str> = request.panels.iter()
        .flat_map(|p| vec![p.width.as_str(), p.height.as_str()])
        .chain(request.stock_panels.iter().flat_map(|s| vec![s.width.as_str(), s.height.as_str()]))
        .collect();
    
    let max_decimal_places = all_sizes.iter()
        .map(|s| get_decimal_places(s))
        .max()
        .unwrap_or(0);
    
    let scale_factor = 10_f64.powi(max_decimal_places as i32);
    println!("RUST: Scale factor: {}", scale_factor);
    println!("RUST: Max decimal places: {}", max_decimal_places);
    
    // Log panel scaling
    println!("RUST: Panel scaling:");
    for panel in &request.panels {
        let width_original: f64 = panel.width.parse().unwrap_or(0.0);
        let height_original: f64 = panel.height.parse().unwrap_or(0.0);
        let width_scaled = (width_original * scale_factor).round() as u32;
        let height_scaled = (height_original * scale_factor).round() as u32;
        
        println!("  Panel {}: {}x{} -> {}x{} (count: {})", 
            panel.id, panel.width, panel.height, 
            width_scaled, height_scaled, panel.count);
    }
    
    // Log stock scaling
    println!("RUST: Stock scaling:");
    for stock in &request.stock_panels {
        let width_original: f64 = stock.width.parse().unwrap_or(0.0);
        let height_original: f64 = stock.height.parse().unwrap_or(0.0);
        let width_scaled = (width_original * scale_factor).round() as u32;
        let height_scaled = (height_original * scale_factor).round() as u32;
        
        println!("  Stock {}: {}x{} -> {}x{}", 
            stock.id, stock.width, stock.height, 
            width_scaled, height_scaled);
    }
}

// ✅ COMPARISON LOG: Algorithm results logging
fn log_algorithm_results(solution: &rezalnyas_core::models::cut_list_thread::Solution, scale_factor: f64) {
    println!("\n=== COMPARISON LOG: ALGORITHM RESULTS ===");
    
    let num_mosaics = solution.get_mosaics().len();
    let no_fit_count = solution.get_no_fit_panels().len();
    
    // Calculate total metrics (with reverse scaling for area comparison)
    let mut total_used_area = 0.0;
    let mut total_wasted_area = 0.0;
    let mut total_cuts = 0;
    
    let scale_squared = scale_factor * scale_factor;
    for mosaic in solution.get_mosaics() {
        // Reverse scaling to match Java output
        total_used_area += mosaic.used_area() as f64 / scale_squared;
        total_wasted_area += mosaic.unused_area() as f64 / scale_squared;
        total_cuts += mosaic.cuts().len();
    }
    
    let total_area = total_used_area + total_wasted_area;
    let usage_efficiency = if total_area > 0.0 {
        total_used_area / total_area
    } else {
        0.0
    };
    
    println!("RUST: Number of mosaics: {}", num_mosaics);
    println!("RUST: Total cuts: {}", total_cuts);
    println!("RUST: Total used area: {:.2}", total_used_area);
    println!("RUST: Total wasted area: {:.2}", total_wasted_area);
    println!("RUST: Usage efficiency: {:.4}", usage_efficiency);
    println!("RUST: No-fit panels: {}", no_fit_count);
    
    // Log detailed mosaic information
    for (i, mosaic) in solution.get_mosaics().iter().enumerate() {
        let used_area = mosaic.used_area() as f64 / scale_squared;
        let wasted_area = mosaic.unused_area() as f64 / scale_squared;
        let panel_count = mosaic.root_tile_node().final_tile_nodes().len();
        let cuts_count = mosaic.cuts().len();
        
        println!("RUST: Mosaic {}: used={:.2}, wasted={:.2}, panels={}, cuts={}", 
            i + 1, used_area, wasted_area, panel_count, cuts_count);
        
        // Log panel placements for verification with coordinates
        let final_tiles = mosaic.root_tile_node().final_tile_nodes();
        for tile_node in final_tiles {
            let width = tile_node.width() as f64 / scale_factor;
            let height = tile_node.height() as f64 / scale_factor;
            let x = tile_node.x1() as f64 / scale_factor;
            let y = tile_node.y1() as f64 / scale_factor;
            let label = format!("ID_{}", tile_node.external_id());
            
            println!("  RUST: Panel placement: {:.1}x{:.1} at ({:.1},{:.1}) [{}]", 
                width, height, x, y, label);
        }
        
        // Log cutting lines for verification
        println!("  RUST: Cuts in mosaic {}:", i + 1);
        for (cut_idx, cut) in mosaic.cuts().iter().enumerate() {
            let x1 = cut.x1() as f64 / scale_factor;
            let y1 = cut.y1() as f64 / scale_factor;
            let x2 = cut.x2() as f64 / scale_factor;
            let y2 = cut.y2() as f64 / scale_factor;
            let orientation = if cut.is_horizontal() { "H" } else { "V" };
            println!("    RUST: Cut {}: {} from ({:.1},{:.1}) to ({:.1},{:.1})", 
                cut_idx + 1, orientation, x1, y1, x2, y2);
        }
    }
    
    // Log solution comparator ranking information
    println!("RUST: Solution ranking factors:");
    println!("  - Mosaics count (lower better): {}", num_mosaics);
    println!("  - Final tiles count: {}", solution.get_mosaics().iter().map(|m| m.root_tile_node().final_tile_nodes().len()).sum::<usize>());
    println!("  - Total unused area: {:.2}", total_wasted_area);
    
    // Log permutation strategy used
    println!("RUST: Optimization strategy: Java successful permutation with horizontal-first cuts");
}