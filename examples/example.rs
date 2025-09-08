use chrono::{DateTime, Local};
use rezalnyas_core::{enums::optimization_level::OptimizationFactor, features::{
    engine::{
        cutlist_optimizer_service_impl::CutListOptimizerServiceImpl,
        model::{
            calculation_request::{CalculationRequest, Panel},
            client_info::{self, ClientInfo},
            configuration::Configuration,
            performance_thresholds::PerformanceThresholds,
        },
    },
    input::models::{
        grouped_tile_dimensions::GroupedTileDimensions, tile_dimensions::TileDimensions,
    },
}};

use std::{
    collections::HashMap,
    sync::atomic::{AtomicU64, Ordering},
    time::Instant,
};

fn main() {
    println!("=== APPLICATION START ===");

    println!("Initializing optimizer service");
    let optimizer =
        CutListOptimizerServiceImpl::new(1, false).expect("Failed to initialize optimizer service");

    println!("Creating calculation request");

    let request = create_request();

    // Log input processing details
    log_input_processing(&request);
    println!("Submitting task to optimizer");

    // =============================================================================
    // TASK SUBMISSION
    // =============================================================================
    let calculation_submission_result = optimizer.submit_task(request);
    
    // =============================================================================
    // CLEANUP
    // =============================================================================

}

fn create_request() -> CalculationRequest {
    println!("=== REQUEST CREATION ===");

    let mut client_info = ClientInfo::default();

    client_info.id = Some("test_client".to_string());
    client_info.device = Some("test_device".to_string());
    client_info.device_id = Some("test-device-001".to_string());

    println!(
        "Client info created - id={}, device={}",
        &client_info.id.as_deref().unwrap(),
        &client_info.device.as_deref().unwrap()
    );

    // Создаем детали
    let panels: Vec<Panel> = vec![
        Panel::new(1, "200.0", "80.0", 1, "Деталь_1"),
        Panel::new(2, "60.0", "40.0", 1, "Деталь_2"),
        Panel::new(3, "80.0", "50.0", 1, "Деталь_3"),
    ];

    let total_panel_count = panels.len();
    let total_pieces: u32 = panels.iter().map(|p| p.count).sum();
    let total_panel_area: f64 = panels
        .iter()
        .map(|p| p.width.parse::<f64>().unwrap_or(0.0) * p.height.parse::<f64>().unwrap_or(0.0))
        .sum();

    println!(
        "Panels created - types={}, total_pieces={}, total_area={:.2}",
        total_panel_count, total_pieces, total_panel_area
    );
    // Заготовка - аналогично Java коду
    let stock_panels: Vec<Panel> = vec![Panel::new(1, "400.0", "300.0", 1, "Заготовка_1")];

    let stock_area = stock_panels[0].width.parse().unwrap_or(0.0)
        * stock_panels[0].height.parse().unwrap_or(0.0);
    let utilization_ratio = total_panel_area / stock_area;
    println!(
        "Stock created - size={}x{}, area={:.2}, theoretical_utilization={:.2}%",
        stock_panels[0].width,
        stock_panels[0].height,
        stock_area,
        utilization_ratio * 100.0
    );

    let mut config = Configuration::default();
    config.cut_thickness = Some("0.0".to_string());
    config.use_single_stock_unit = true;
    config.min_trim_dimension = Some("0.0".to_string());
    config.consider_orientation = false;
    config.optimization_factor = OptimizationFactor::Ultra;

    let mut thresholds = PerformanceThresholds::default();
    thresholds.max_simultaneous_threads = 1;
    thresholds.max_simultaneous_tasks = 1;
    thresholds.thread_check_interval = 1000;

    let mut request = CalculationRequest::default();

    
    println!("Configuration created - cut_thickness={}, min_trim={}, optimization_factor={}, max_threads={}",
    config.cut_thickness.as_deref().unwrap(),
    config.min_trim_dimension.as_deref().unwrap(),
    config.optimization_factor.value(),
    thresholds.max_simultaneous_threads
    
);
request.panels = panels;
request.stock_panels = stock_panels;
request.configuration = config;
request.client_info = client_info;
request.performance_thresholds = thresholds;

    println!("=== REQUEST CREATION COMPLETED ===");
    request
}

/// Port from Java CutListOptimizerServiceImpl.removeDuplicatedPermutations method
/// Removes duplicate permutations based on dimensions hash code
fn remove_duplicated_permutations(permutations: &mut Vec<Vec<TileDimensions>>) -> i32 {
    println!(
        "Starting duplicate permutation removal - total_permutations={}",
        permutations.len()
    );

    // ТОЧНАЯ копия Java ArrayList arrayList = new ArrayList();
    let mut array_list = Vec::new();
    let mut removed_count = 0;

    // Отладочный вывод первых перестановок
    if !permutations.is_empty() {
        for (idx, perm) in permutations.iter().take(3).enumerate() {
            let mut hash = 0i32;
            let mut tiles_str = String::new();
            for tile in perm {
                let tile_hash = (tile.width as i32)
                    .wrapping_mul(31)
                    .wrapping_add(tile.height as i32);
                hash = hash.wrapping_mul(31).wrapping_add(tile_hash);
                tiles_str.push_str(&format!("{}x{},", tile.width, tile.height));
            }
        }
    }

    // ТОЧНАЯ копия Java Iterator логики
    let mut i = 0;
    while i < permutations.len() {
        let mut i_dimensions_based_hash_code = 0i32;

        // ТОЧНАЯ копия Java: while (permutation.hasNext())
        for tile in &permutations[i] {
            // ТОЧНАЯ копия Java: iDimensionsBasedHashCode = (iDimensionsBasedHashCode * 31) + permutation.next().dimensionsBasedHashCode();
            let tile_dimensions_hash = (tile.width as i32)
                .wrapping_mul(31)
                .wrapping_add(tile.height as i32);
            i_dimensions_based_hash_code = i_dimensions_based_hash_code
                .wrapping_mul(31)
                .wrapping_add(tile_dimensions_hash);
        }

        // ТОЧНАЯ копия Java: if (arrayList.contains(Integer.valueOf(iDimensionsBasedHashCode)))
        if array_list.contains(&i_dimensions_based_hash_code) {
            // ТОЧНАЯ копия Java: it.remove(); removedCount++;
            permutations.remove(i);
            removed_count += 1;
        } else {
            // ТОЧНАЯ копия Java: arrayList.add(Integer.valueOf(iDimensionsBasedHashCode));
            array_list.push(i_dimensions_based_hash_code);
            i += 1;
        }
    }

    removed_count
}
fn get_distinct_grouped_tile_dimensions(
    grouped_panels: &[GroupedTileDimensions],
) -> HashMap<GroupedTileDimensions, i32> {
    println!(
        "Calculating distinct groups - input_size={}",
        grouped_panels.len()
    );
    let mut map = HashMap::new();

    // Java логика: HashMap<T, Integer> где T - это GroupedTileDimensions
    // Java использует equals() и hashCode() GroupedTileDimensions
    // которые включают id, width, height И group
    for group in grouped_panels {
        let count = map.entry(group.clone()).or_insert(0);
        *count += 1;
    }

    map
}

/// Generate groups from tiles (equivalent to Java generateGroups method)
fn generate_groups(
    tiles: &[TileDimensions],
    stock_tiles: &[TileDimensions],
) -> Vec<GroupedTileDimensions> {
    let task_id = generate_task_id();
    println!(
        "Starting group generation - tiles={}, stock={}, task={}",
        tiles.len(),
        stock_tiles.len(),
        task_id
    );

    // Count tile types
    let mut tile_counts = HashMap::new();
    for tile in tiles {
        let key = format!("{}x{}", tile.width, tile.height);
        *tile_counts.entry(key).or_insert(0) += 1;
    }

    // Create tile groups info string like Java using original tile IDs
    // Java outputs in a specific order, we need to maintain original tile order
    let mut group_info = String::new();
    let mut seen_dimensions = std::collections::HashSet::new();

    // Java shows tiles in the order they appear in the original input
    // but grouped by unique dimensions with their counts
    for tile in tiles {
        let key = format!("{}x{}", tile.width, tile.height);
        if !seen_dimensions.contains(&key) {
            seen_dimensions.insert(key.clone());
            let count = tile_counts.get(&key).unwrap_or(&0);
            group_info.push_str(&format!("id={}[{}]*{} ", tile.id, key, count));
        }
    }
    println!("Tile groups: {}", group_info.trim());

    // Determine if this is one-dimensional optimization
    println!(
        "Checking one-dimensional optimization - tiles={}, stock={}",
        tiles.len(),
        stock_tiles.len()
    );
    let is_one_dimensional = is_one_dimensional_optimization(tiles, stock_tiles);

    let max_per_group = if is_one_dimensional {
        println!("Using one-dimensional optimization - group_split_threshold=1");
        1000 // Large number for one-dimensional
    } else {
        // Java: Math.max(tiles.size() / 100, 1)
        let threshold = std::cmp::max(tiles.len() / 100, 1);
        println!(
            "Using multi-dimensional optimization - group_split_threshold={}",
            threshold
        );
        threshold
    };

    let mut result = Vec::new();
    let mut group_counters = HashMap::new();
    let mut current_group = 0;

    // Process tiles exactly like Java
    for tile in tiles {
        let tile_key = format!("{}x{}", tile.width, tile.height);
        let group_key = format!("{}-{}", tile_key, current_group);

        let count_in_group = group_counters.entry(group_key.clone()).or_insert(0);
        *count_in_group += 1;

        // Convert features TileDimensions to models TileDimensions
        let models_tile = TileDimensions::new(
            tile.id,
            tile.width,
            tile.height,
            tile.is_rotated,
            &tile.label,
            &tile.material,
        );

        let grouped_tile = GroupedTileDimensions::from_tile_dimensions(models_tile, current_group);
        result.push(grouped_tile);

        // Java splitting logic:
        // if (totalCount > groupSplitThreshold && countInGroup > totalCount / 4)
        if let Some(&total_count) = tile_counts.get(&tile_key) {
            if total_count > max_per_group && *count_in_group > total_count / 4 {
                println!(
                    "Splitting group for tile {} - total_count={}, group_threshold={}",
                    tile_key, total_count, max_per_group
                );
                current_group += 1;
            }
        }
    }

    // Java formula: groups_used = currentGroupIndex + 1
    let groups_used = current_group + 1;

    println!(
        "Group generation completed - grouped_tiles={}, groups_used={}",
        result.len(),
        groups_used
    );

    result
}

/// Check if this is one-dimensional optimization
fn is_one_dimensional_optimization(
    tiles: &[TileDimensions],
    stock_tiles: &[TileDimensions],
) -> bool {
    if tiles.is_empty() || stock_tiles.is_empty() {
        return false;
    }

    // Get unique dimensions from first tile
    let mut common_dimensions = vec![tiles[0].width, tiles[0].height];

    // Check all tiles
    for tile in tiles {
        common_dimensions.retain(|&dim| dim == tile.width || dim == tile.height);
        if common_dimensions.is_empty() {
            println!("One-dimensional check result=false (tiles don't share dimensions)");
            return false;
        }
    }

    // Check stock tiles
    for stock_tile in stock_tiles {
        common_dimensions.retain(|&dim| dim == stock_tile.width || dim == stock_tile.height);
        if common_dimensions.is_empty() {
            println!("One-dimensional check result=false (stock doesn't share dimensions)");
            return false;
        }
    }

    let result = !common_dimensions.is_empty();
    if result {
        println!(
            "One-dimensional check result=true, common_dimensions={}",
            common_dimensions.len()
        );
    }
    result
}

// Глобальный счетчик задач (аналог AtomicLong в Java)
static TASK_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Generate all permutations of grouped tile dimensions (equivalent to Arrangement.generatePermutations)
fn generate_permutations(groups: &[GroupedTileDimensions]) -> Vec<Vec<GroupedTileDimensions>> {
    if groups.is_empty() {
        return vec![vec![]];
    }

    if groups.len() == 1 {
        return vec![vec![groups[0].clone()]];
    }

    let mut result = Vec::new();

    for i in 0..groups.len() {
        let current = &groups[i];
        let remaining: Vec<GroupedTileDimensions> = groups
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != i)
            .map(|(_, item)| item.clone())
            .collect();

        let sub_perms = generate_permutations(&remaining);

        for mut sub_perm in sub_perms {
            let mut new_perm = vec![current.clone()];
            new_perm.append(&mut sub_perm);
            result.push(new_perm);
        }
    }

    result
}

/// Convert grouped dimensions permutation back to tile list (equivalent to groupedTileDimensionsList2TileDimensionsList)
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
    }

    result
}

fn generate_task_id() -> String {
    // Получаем текущую дату и время
    let now: DateTime<Local> = Local::now();

    // Форматируем в том же формате, что и в Java: "yyyyMMddHHmm"
    let date_part = now.format("%Y%m%d%H%M").to_string();

    // Получаем следующий номер счетчика (аналог taskIdCounter.getAndIncrement())
    let counter = TASK_ID_COUNTER.fetch_add(1, Ordering::SeqCst);

    // Объединяем дату и счетчик
    format!("{}{}", date_part, counter)
}

// Helper method for decimal places calculation
fn get_decimal_places(value: &str) -> i32 {
    match value.find('.') {
        Some(dot_index) => (value.len() - dot_index - 1) as i32,
        None => 0,
    }
}

// ✅ COMPARISON LOG: Input processing logging
fn log_input_processing(request: &CalculationRequest) {
    println!("\n=== COMPARISON LOG: INPUT PROCESSING ===");
    
    // Calculate scale factor
    let mut max_decimal_places = 0i32;
    
    for panel in &request.panels {
        max_decimal_places = max_decimal_places.max(get_decimal_places(&panel.width));
        max_decimal_places = max_decimal_places.max(get_decimal_places(&panel.height));
    }
    
     
    for stock in &request.stock_panels {
        max_decimal_places = max_decimal_places.max(get_decimal_places(&stock.width));
        max_decimal_places = max_decimal_places.max(get_decimal_places(&stock.height));
    }
    
    let scale_factor = 10.0_f64.powi(max_decimal_places);
    println!("Scale factor: {}", scale_factor);
    println!("Max decimal places: {}", max_decimal_places);
    
    // Log panel scaling
    println!("Panel scaling:");
    for panel in &request.panels {
        let width_original: f64 = panel.width.parse().unwrap_or(0.0);
        let height_original: f64 = panel.height.parse().unwrap_or(0.0);
        let width_scaled = (width_original * scale_factor).round() as i32;
        let height_scaled = (height_original * scale_factor).round() as i32;
        
        println!(
            "  Panel {}: {}x{} -> {}x{} (count: {})",
            panel.id, panel.width, panel.height, width_scaled, height_scaled, panel.count
        );
    }
    
    // Log stock scaling
    println!("Stock scaling:");
    for stock in &request.stock_panels {
        let width_original: f64 = stock.width.parse().unwrap_or(0.0);
        let height_original: f64 = stock.height.parse().unwrap_or(0.0);
        let width_scaled = (width_original * scale_factor).round() as i32;
        let height_scaled = (height_original * scale_factor).round() as i32;
        
        println!(
            "  Stock {}: {}x{} -> {}x{}",
            stock.id, stock.width, stock.height, width_scaled, height_scaled
        );
    }
}
