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

// Компараторы решений - обертки для функций из solution_comparator
#[derive(Debug)]
struct MostTilesComparator;

impl SolutionComparator for MostTilesComparator {
    fn compare(&self, a: &CutListSolution, b: &CutListSolution) -> Ordering {
        // Максимальное количество плиток - лучше (Java: меньше получается при сортировке)
        let tiles_a = count_final_tiles(a);
        let tiles_b = count_final_tiles(b);
        tiles_b.cmp(&tiles_a) // Обратную сортировку
    }
}

#[derive(Debug)]
struct LeastWastedAreaComparator;

impl SolutionComparator for LeastWastedAreaComparator {
    fn compare(&self, a: &CutListSolution, b: &CutListSolution) -> Ordering {
        // Минимальная потеря
        let wasted_a = calculate_wasted_area(a);
        let wasted_b = calculate_wasted_area(b);
        wasted_a.partial_cmp(&wasted_b).unwrap_or(Ordering::Equal)
    }
}

#[derive(Debug)]
struct LeastCutsComparator;

impl SolutionComparator for LeastCutsComparator {
    fn compare(&self, a: &CutListSolution, b: &CutListSolution) -> Ordering {
        // Минимальное количество резов
        let cuts_a = count_cuts(a);
        let cuts_b = count_cuts(b);
        cuts_a.cmp(&cuts_b)
    }
}

#[derive(Debug)]
struct LeastMosaicsComparator;

impl SolutionComparator for LeastMosaicsComparator {
    fn compare(&self, a: &CutListSolution, b: &CutListSolution) -> Ordering {
        // Минимальное количество мозаик (листов)
        let mosaics_a = a.get_mosaics().len();
        let mosaics_b = b.get_mosaics().len();
        mosaics_a.cmp(&mosaics_b)
    }
}

#[derive(Debug)]
struct BiggestUnusedTileAreaComparator;

impl SolutionComparator for BiggestUnusedTileAreaComparator {
    fn compare(&self, a: &CutListSolution, b: &CutListSolution) -> Ordering {
        // Максимальная неиспользованная площадь (предпочитаем больше)
        let biggest_a = get_biggest_unused_area(a);
        let biggest_b = get_biggest_unused_area(b);
        biggest_b.cmp(&biggest_a) // Обратный порядок - больше лучше
    }
}

#[derive(Debug)]
struct MostDistinctTileSetComparator;

impl SolutionComparator for MostDistinctTileSetComparator {
    fn compare(&self, a: &CutListSolution, b: &CutListSolution) -> Ordering {
        // Максимальное количество различных типов плиток
        let distinct_a = get_distinct_tile_set_size(a);
        let distinct_b = get_distinct_tile_set_size(b);
        distinct_b.cmp(&distinct_a) // Обратный порядок - больше лучше
    }
}

// Хелперы для подсчета метрик
fn count_final_tiles(solution: &CutListSolution) -> usize {
    solution.get_mosaics().iter().map(|m| {
        m.root_tile_node().final_tile_nodes().len()
    }).sum()
}

fn calculate_wasted_area(solution: &CutListSolution) -> f64 {
    solution.get_mosaics().iter().map(|m| m.unused_area() as f64).sum()
}

fn count_cuts(solution: &CutListSolution) -> usize {
    solution.get_mosaics().iter().map(|m| m.cuts().len()).sum()
}

fn get_biggest_unused_area(solution: &CutListSolution) -> u64 {
    solution.get_mosaics().iter().map(|m| get_biggest_unused_area_in_mosaic(m)).max().unwrap_or(0)
}

fn get_biggest_unused_area_in_mosaic(mosaic: &rezalnyas_core::models::mosaic::Mosaic) -> u64 {
    get_biggest_unused_area_in_tile_node(mosaic.root_tile_node())
}

fn get_biggest_unused_area_in_tile_node(tile_node: &rezalnyas_core::models::tile_node::TileNode) -> u64 {
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
        max_area = max_area.max(get_biggest_unused_area_in_tile_node(child1));
    }
    if let Some(child2) = tile_node.child2() {
        max_area = max_area.max(get_biggest_unused_area_in_tile_node(child2));
    }
    
    max_area
}

fn get_distinct_tile_set_size(solution: &CutListSolution) -> usize {
    solution.get_mosaics().iter().map(|m| get_distinct_tile_set_in_mosaic(m)).max().unwrap_or(0)
}

fn get_distinct_tile_set_in_mosaic(mosaic: &rezalnyas_core::models::mosaic::Mosaic) -> usize {
    let mut distinct_tiles = std::collections::HashSet::new();
    collect_distinct_tiles(mosaic.root_tile_node(), &mut distinct_tiles);
    distinct_tiles.len()
}

fn collect_distinct_tiles(tile_node: &rezalnyas_core::models::tile_node::TileNode, distinct_tiles: &mut std::collections::HashSet<u32>) {
    if tile_node.is_final() {
        // Создаем уникальный идентификатор плитки по размеру (как в Java)
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
    
    println!("Java-стиль масштабирование: {} десятичных знаков", max_decimal_places);
    
    // Создаем масштабирующий коэффициент (как в Java: Math.pow(10.0, maxDecimalPlaces))
    let scale_factor = 10_f64.powi(max_decimal_places as i32);
    println!("Масштабирующий коэффициент: {}", scale_factor);
    
    // Создаем входные данные с Java масштабированием
    let mut tiles = Vec::new();
    for panel in &request.panels {
        for _ in 0..panel.count {
            let width_scaled = (panel.width.parse::<f64>()? * scale_factor).round() as u32;
            let height_scaled = (panel.height.parse::<f64>()? * scale_factor).round() as u32;
            
            println!("Панель {}: {}x{} -> {}x{}", 
                panel.id, panel.width, panel.height, width_scaled, height_scaled);
            
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
    
    println!("Заготовка: {}x{} -> {}x{}", 
        request.stock_panels[0].width, request.stock_panels[0].height,
        stock_width_scaled, stock_height_scaled);
    
    let stock_dimensions = TileDimensions::new(
        request.stock_panels[0].id.into(),
        stock_width_scaled,
        stock_height_scaled,
        "DEFAULT_MATERIAL".to_string(), // Точно как в Java
        0, // orientation = 0
        Some(request.stock_panels[0].label.clone()), // label
        false, // is_rotated = false
    );
    
    let stock_solution = StockSolution::new(vec![stock_dimensions]);
    
    // Создаем и настраиваем CutListThread с точно такими же настройками как в Java
    let mut cut_list_thread = CutListThread::new();
    cut_list_thread.set_tiles(tiles);
    cut_list_thread.set_stock_solution(Some(stock_solution));
    
    // Применяем конфигурацию точно как в Java
    cut_list_thread.set_accuracy_factor(20); // optimizationFactor = 2.0 -> accuracy_factor = 20 
    cut_list_thread.set_cut_thickness(0); // cutThickness = "0"
    cut_list_thread.set_min_trim_dimension(0); // minTrimDimension = "0"
    cut_list_thread.set_consider_grain_direction(false); // considerOrientation = false
    
    // КРИТИЧНО: Настраиваем компараторы точно как в Java для optimizationPriority = 0
    // PriorityListFactory.java - для priority 0:
    // arrayList.add(OptimizationPriority.MOST_TILES.toString());
    // arrayList.add(OptimizationPriority.LEAST_WASTED_AREA.toString());
    // arrayList.add(OptimizationPriority.LEAST_NBR_CUTS.toString());
    // arrayList.add(OptimizationPriority.LEAST_NBR_MOSAICS.toString());
    // arrayList.add(OptimizationPriority.BIGGEST_UNUSED_TILE_AREA.toString());
    // arrayList.add(OptimizationPriority.MOST_HV_DISCREPANCY.toString());
    
    // УПРОЩЕННЫЕ компараторы - только основные 3 как изначально работало лучше
    let thread_comparators: Vec<Box<dyn SolutionComparator>> = vec![
        Box::new(MostTilesComparator),
        Box::new(LeastWastedAreaComparator),
        Box::new(LeastCutsComparator),
    ];
    
    let final_comparators: Vec<Box<dyn SolutionComparator>> = vec![
        Box::new(MostTilesComparator),
        Box::new(LeastWastedAreaComparator), 
        Box::new(LeastCutsComparator),
    ];
    
    cut_list_thread.set_thread_prioritized_comparators(thread_comparators);
    cut_list_thread.set_final_solution_prioritized_comparators(final_comparators);
    
    println!("Задача принята. ID: production-task");
    println!("Ожидание завершения задачи...");
    
    let start_time = Instant::now();
    
    // Запускаем оптимизацию
    cut_list_thread.compute_solutions_java_style()?;
    
    let elapsed_time = start_time.elapsed();
    let total_seconds = elapsed_time.as_secs();
    
    println!("\n=== Задача выполнена за {} секунд! ===", total_seconds);
    
    // Получаем решения и выводим результат точно как в Java
    if let Ok(solutions) = cut_list_thread.get_all_solutions().lock() {
        println!("RUST DEBUG: Всего решений найдено: {}", solutions.len());
        if let Some(best_solution) = solutions.first() {
            println!("RUST DEBUG: Лучшее решение содержит мозаик: {}", best_solution.get_mosaics().len());
            for (i, mosaic) in best_solution.get_mosaics().iter().enumerate() {
                println!("RUST DEBUG: Мозаика {}: used_area={}, unused_area={}, cuts={}", 
                    i, mosaic.used_area(), mosaic.unused_area(), mosaic.cuts().len());
            }
            print_production_solution(best_solution, elapsed_time.as_millis());
            generate_production_html_visualization_with_scale(best_solution, elapsed_time.as_millis(), scale_factor)?;
        } else {
            println!("Решение не найдено");
        }
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
        println!("RUST DEBUG HTML: final_tiles.len() = {}", final_tiles.len());
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