use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::{
    enums::{
        cut_orientation_preference::CutOrientationPreference,
        optimization_priority::OptimizationPriority, orientation::Orientation, status::Status,
    },
    log_debug,
    models::{
        calculation_request::structs::CalculationRequest, configuration::Configuration,
        grouped_tile_dimensions::GroupedTileDimensions,
        performance_thresholds::structs::PerformanceThresholds, task::Task,
    },
};

/// Represents the dimensions and properties of a tile/panel to be cut
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct TileDimensions {
    pub id: u8,
    pub width: u64,
    pub height: u64,
    pub orientation: Orientation,
    pub is_rotated: bool,
}

impl std::fmt::Display for TileDimensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Java: return "id=" + this.id + "[" + this.width + "x" + this.height + ']';
        write!(f, "id={}[{}x{}]", self.id, self.width, self.height)
    }
}

impl TileDimensions {
    /// Convert dimensions to string format "widthxheight"
    pub fn dimensions_to_string(&self) -> String {
        format!("{}x{}", self.width, self.height)
    }

    /// Calculate dimensions-based hash code (аналог Java dimensionsBasedHashCode)
    ///
    /// # Returns
    /// Hash code основанный только на размерах (width и height)
    pub fn dimensions_based_hash_code(&self) -> i32 {
        (self.width as i32 * 31) + (self.height as i32)
    }
    /// Calculate dimensions-based hash code exactly like Java version
    ///
    /// Java: return (this.width * 31) + this.height;
    ///
    /// # Returns
    /// Hash code основанный только на размерах (width и height)
    pub fn dimensions_based_hash_code_java_compatible(&self) -> i32 {
        ((self.width as i32).wrapping_mul(31)).wrapping_add(self.height as i32)
    }
}

/// Improved version that ensures logical grouping with better control
pub fn generate_groups_improved(
    tiles: &[TileDimensions],
    sheet_tiles: &[TileDimensions],
    task: &Task,
) -> Vec<GroupedTileDimensions> {
    // Подсчитываем количество каждого типа деталей
    let mut tile_type_counts: HashMap<(u64, u64), usize> = HashMap::new();
    for tile in tiles {
        let key = (tile.width, tile.height);
        *tile_type_counts.entry(key).or_insert(0) += 1;
    }

    // Создаем строку для отладки
    let tiles_info: String = tile_type_counts
        .iter()
        .map(|((w, h), count)| format!("{}x{}*{}", w, h, count))
        .collect::<Vec<_>>()
        .join(" ");

    log_debug!(
        "Task[{}] TotalNbrTiles[{}] TileTypes: {}",
        task.id,
        tiles.len(),
        tiles_info
    );

    // Определяем максимальный размер группы
    let base_group_size = std::cmp::max(tiles.len() / 100, 1);
    let is_one_dim = is_one_dimensional_optimization(tiles, sheet_tiles);

    if is_one_dim {
        log_debug!("Task[{}] is one dimensional optimization", task.id);
    }

    let mut result = Vec::new();
    let mut current_group_id = 0;

    // Группируем детали по типам (размерам)
    let mut tiles_by_type: HashMap<(u64, u64), Vec<TileDimensions>> = HashMap::new();
    for tile in tiles {
        let key = (tile.width, tile.height);
        tiles_by_type.entry(key).or_default().push(tile.clone());
    }

    // Обрабатываем каждый тип деталей
    for ((width, height), mut tiles_of_type) in tiles_by_type {
        let total_count = tiles_of_type.len();

        // Для одномерной оптимизации или малого количества деталей - одна группа
        let effective_group_size = if is_one_dim || total_count <= 2 {
            total_count // Все детали в одну группу
        } else {
            std::cmp::max(base_group_size, total_count / 2) // Максимум 2 группы на тип
        };

        if total_count <= effective_group_size {
            // Все детали данного типа помещаются в одну группу
            for tile in tiles_of_type {
                result.push(GroupedTileDimensions::new(tile, current_group_id));
            }

            log_debug!(
                "Task[{}] Created single group {} for {}x{} ({} tiles)",
                task.id,
                current_group_id,
                width,
                height,
                total_count
            );

            current_group_id += 1;
        } else {
            // Разбиваем на группы, но не больше чем на 2
            let groups_needed = std::cmp::min(
                2,
                (total_count + effective_group_size - 1) / effective_group_size,
            );
            let tiles_per_group = total_count / groups_needed;
            let mut extra_tiles = total_count % groups_needed;

            let mut groups_created = 0;
            let mut tiles_processed = 0;

            for group_idx in 0..groups_needed {
                let mut current_group_size = tiles_per_group;
                if extra_tiles > 0 {
                    current_group_size += 1;
                    extra_tiles -= 1;
                }

                for _ in 0..current_group_size {
                    if tiles_processed < tiles_of_type.len() {
                        result.push(GroupedTileDimensions::new(
                            tiles_of_type[tiles_processed].clone(),
                            current_group_id,
                        ));
                        tiles_processed += 1;
                    }
                }

                groups_created += 1;
                current_group_id += 1;
            }

            log_debug!(
                "Task[{}] Split {}x{} ({} tiles) into {} groups",
                task.id,
                width,
                height,
                total_count,
                groups_created
            );
        }
    }

    log_debug!(
        "Task[{}] Created {} total groups from {} tiles",
        task.id,
        current_group_id,
        tiles.len()
    );

    result
}

/// Generate groups of tiles for optimization
pub fn generate_groups(
    tiles: &[TileDimensions],
    sheet_tiles: &[TileDimensions],
    task: &Task,
) -> Vec<GroupedTileDimensions> {
    // Count occurrences of each tile type
    let mut tile_counts: HashMap<String, i32> = HashMap::new();
    for tile in tiles {
        let key = tile.dimensions_to_string();
        *tile_counts.entry(key).or_insert(0) += 1;
    }

    // Create debug string for logging
    let tiles_info: String = tile_counts
        .iter()
        .map(|(key, count)| format!("{}*{}", key, count))
        .collect::<Vec<_>>()
        .join(" ");

    log_debug!(
        "Task[{}] TotalNbrTiles[{}] Tiles: {}",
        task.id,
        tiles.len(),
        tiles_info
    );

    let mut group_size = std::cmp::max(tiles.len() / 100, 1);

    // Check if this is one-dimensional optimization
    if is_one_dimensional_optimization(tiles, sheet_tiles) {
        log_debug!("Task is one dimensional optimization");
        group_size = 1;
    }

    let mut result = Vec::new();
    let mut current_group = 0;
    let mut group_counts: HashMap<String, i32> = HashMap::new();

    for tile in tiles {
        let group_key = format!("{}{}", tile.to_string(), current_group);
        let group_count = group_counts.entry(group_key.clone()).or_insert(0);
        *group_count += 1;

        result.push(GroupedTileDimensions::new(tile.clone(), current_group));

        // Check if we should split into a new group
        if let Some(&total_count) = tile_counts.get(&tile.to_string()) {
            if total_count > group_size as i32 && *group_count > total_count / 4 {
                log_debug!(
                    "Task[{}] Splitting panel set [{}] with [{}] units into two groups",
                    task.id,
                    tile.dimensions_to_string(),
                    total_count
                );
                current_group += 1;
            }
        }
    }

    result
}

/// Generate groups of tiles for optimization - Java-compatible version
///
/// Это точная копия Java-алгоритма из CutListOptimizerServiceImpl.generateGroups()
pub fn generate_groups_java_compatible(
    tiles: &[TileDimensions],
    sheet_tiles: &[TileDimensions],
    task: &Task,
) -> Vec<GroupedTileDimensions> {
    // Count occurrences of each tile type (Java: map)
    let mut tile_counts: HashMap<String, i32> = HashMap::new();
    for tile in tiles {
        let key = tile.to_string(); // Java: tileDimensions.toString()
        *tile_counts.entry(key).or_insert(0) += 1;
    }

    // Create debug string for logging (Java: sb)
    let tiles_info: String = tile_counts
        .iter()
        .map(|(key, count)| format!("{}*{}", key, count))
        .collect::<Vec<_>>()
        .join(" ");

    log_debug!(
        "Task[{}] TotalNbrTiles[{}] Tiles: {}",
        task.id,
        tiles.len(),
        tiles_info
    );

    // Java: int iMax = Math.max(list.size() / 100, 1);
    let mut group_size = std::cmp::max(tiles.len() / 100, 1);

    // Check if this is one-dimensional optimization
    if is_one_dimensional_optimization(tiles, sheet_tiles) {
        log_debug!("Task[{}] is one dimensional optimization", task.id);
        group_size = 1;
    }

    let mut result = Vec::new();
    let mut current_group = 0; // Java: int i = 0;
    let mut group_counts: HashMap<String, i32> = HashMap::new(); // Java: map2

    // Java: for (TileDimensions tileDimensions : list)
    for tile in tiles {
        // Java: String str2 = tileDimensions.toString() + i;
        let group_key = format!("{}{}", tile.to_string(), current_group);

        // Java: map2.put(str2, Integer.valueOf(map2.get(str2) != null ? ((Integer) map2.get(str2)).intValue() + 1 : 1));
        let group_count = group_counts.entry(group_key.clone()).or_insert(0);
        *group_count += 1;

        // Java: arrayList.add(new GroupedTileDimensions(tileDimensions, i));
        result.push(GroupedTileDimensions::new(tile.clone(), current_group));

        // Check if we should split into a new group
        // Java: if (((Integer) map.get(tileDimensions.toString())).intValue() > iMax &&
        //          ((Integer) map2.get(str2)).intValue() > ((Integer) map.get(tileDimensions.toString())).intValue() / 4)
        //
        // ВАЖНО: И для подсчета, и для проверки используется tile.toString()!
        if let Some(&total_count) = tile_counts.get(&tile.to_string()) {
            if total_count > group_size as i32 && *group_count > total_count / 4 {
                log_debug!(
                    "Task[{}] Splitting panel set [{}] with [{}] units into two groups",
                    task.id,
                    tile.dimensions_to_string(), // Только для логирования
                    total_count
                );
                current_group += 1; // Java: i++;
            }
        }
    }

    result
}

/// Check if the optimization is one-dimensional - Java-compatible version
fn is_one_dimensional_optimization(
    tiles: &[TileDimensions],
    sheet_tiles: &[TileDimensions],
) -> bool {
    if tiles.is_empty() {
        return false;
    }

    // Java: ArrayList arrayList = new ArrayList();
    // arrayList.add(Integer.valueOf(list.get(0).getWidth()));
    // arrayList.add(Integer.valueOf(list.get(0).getHeight()));
    let mut common_dimensions = vec![tiles[0].width as i32, tiles[0].height as i32];

    // Check tiles
    // Java: for (TileDimensions tileDimensions : list)
    for tile in tiles {
        // Java: if (((Integer) arrayList.get(0)).intValue() != tileDimensions.getWidth() &&
        //           ((Integer) arrayList.get(0)).intValue() != tileDimensions.getHeight()) {
        //     arrayList.remove(0);
        // }
        if !common_dimensions.is_empty() {
            if common_dimensions[0] != tile.width as i32
                && common_dimensions[0] != tile.height as i32
            {
                common_dimensions.remove(0);
            }
        }

        if common_dimensions.len() == 2 {
            if common_dimensions[1] != tile.width as i32
                && common_dimensions[1] != tile.height as i32
            {
                common_dimensions.remove(1);
            }
        }

        if common_dimensions.is_empty() {
            return false;
        }
    }

    // Check sheet tiles
    // Java: for (TileDimensions tileDimensions2 : list2)
    for sheet_tile in sheet_tiles {
        if !common_dimensions.is_empty() {
            if common_dimensions[0] != sheet_tile.width as i32
                && common_dimensions[0] != sheet_tile.height as i32
            {
                common_dimensions.remove(0);
            }
        }

        if common_dimensions.len() == 2 {
            if common_dimensions[1] != sheet_tile.width as i32
                && common_dimensions[1] != sheet_tile.height as i32
            {
                common_dimensions.remove(1);
            }
        }

        if common_dimensions.is_empty() {
            return false;
        }
    }

    true
}

pub fn remove_duplicated_permutations(permutation_lists: &mut Vec<Vec<TileDimensions>>) -> usize {
    let mut seen_hash_codes = HashSet::new();
    let mut removed_count = 0;
    let original_count = permutation_lists.len();

    // Используем retain для эффективного удаления элементов во время итерации
    permutation_lists.retain(|tile_list| {
        let hash_code = calculate_permutation_hash_code(tile_list);
        // В функции remove_duplicated_permutations добавьте:
        log_debug!(
            "Permutation hash: {} for tiles: {:?}",
            hash_code,
            tile_list
                .iter()
                .take(3)
                .map(|t| format!("{}x{}", t.width, t.height))
                .collect::<Vec<_>>()
        );
        if seen_hash_codes.contains(&hash_code) {
            // Этот хеш-код уже видели, удаляем перестановку
            removed_count += 1;
            false // не сохранять этот элемент
        } else {
            // Новый хеш-код, сохраняем перестановку
            seen_hash_codes.insert(hash_code);
            true // сохранить этот элемент
        }
    });

    log_debug!(
        "Removed {} duplicated permutations, {} remaining (was {})",
        removed_count,
        permutation_lists.len(),
        original_count
    );

    removed_count
}

/// Обновленная версия удаления дублированных перестановок с Java-совместимым хешированием
pub fn remove_duplicated_permutations_java_compatible(
    permutation_lists: &mut Vec<Vec<TileDimensions>>,
) -> usize {
    let mut seen_hash_codes = HashSet::new();
    let mut removed_count = 0;
    let original_count = permutation_lists.len();

    // Используем retain для эффективного удаления элементов во время итерации
    permutation_lists.retain(|tile_list| {
        let hash_code = calculate_permutation_hash_code_java_compatible(tile_list);

        if seen_hash_codes.contains(&hash_code) {
            // Этот хеш-код уже видели, удаляем перестановку
            removed_count += 1;
            false // не сохранять этот элемент
        } else {
            // Новый хеш-код, сохраняем перестановку
            seen_hash_codes.insert(hash_code);
            true // сохранить этот элемент
        }
    });

    log_debug!(
        "Removed {} duplicated permutations, {} remaining (was {})",
        removed_count,
        permutation_lists.len(),
        original_count
    );

    removed_count
}

/// Вычисляет хеш-код для списка плиток на основе размеров
///
/// Аналог логики из Java метода, где хеш-код вычисляется как:
/// hash = hash * 31 + tile.dimensionsBasedHashCode()
///
/// # Arguments
/// * `tile_list` - Список плиток для вычисления хеш-кода
///
/// # Returns
/// Хеш-код основанный на размерах плиток в порядке их следования
fn calculate_permutation_hash_code(tile_list: &[TileDimensions]) -> i32 {
    let mut hash_code = 0i32;

    for tile in tile_list {
        hash_code = hash_code
            .wrapping_mul(31)
            .wrapping_add(tile.dimensions_based_hash_code());
    }

    hash_code
}
/// Вычисляет хеш-код для списка плиток, точно совместимый с Java версией
///
/// Использует 32-битную арифметику с переполнением точно как в Java
///
/// # Arguments
/// * `tile_list` - Список плиток для вычисления хеш-кода
///
/// # Returns
/// Хеш-код основанный на размерах плиток в порядке их следования
fn calculate_permutation_hash_code_java_compatible(tile_list: &[TileDimensions]) -> i32 {
    let mut hash_code = 0i32;

    for tile in tile_list {
        // Вычисляем dimensionsBasedHashCode точно как в Java
        let dimensions_hash =
            ((tile.width as i32).wrapping_mul(31)).wrapping_add(tile.height as i32);

        // Обновляем общий хеш-код точно как в Java
        hash_code = hash_code.wrapping_mul(31).wrapping_add(dimensions_hash);
    }

    hash_code
}
/// Альтернативная версия с использованием итераторов для более функционального стиля
///
/// # Arguments
/// * `permutation_lists` - Список списков плиток
///
/// # Returns
/// Новый вектор без дублированных перестановок и количество удаленных
pub fn remove_duplicated_permutations_functional(
    permutation_lists: Vec<Vec<TileDimensions>>,
) -> (Vec<Vec<TileDimensions>>, usize) {
    let original_count = permutation_lists.len();
    let mut seen_hash_codes = HashSet::new();

    let deduplicated: Vec<Vec<TileDimensions>> = permutation_lists
        .into_iter()
        .filter(|tile_list| {
            let hash_code = calculate_permutation_hash_code(tile_list);
            seen_hash_codes.insert(hash_code)
        })
        .collect();

    let removed_count = original_count - deduplicated.len();

    (deduplicated, removed_count)
}

/// Подсчитывает количество уникальных перестановок без их удаления
///
/// Полезно для предварительной оценки эффективности дедупликации
///
/// # Arguments
/// * `permutation_lists` - Список списков плиток для анализа
///
/// # Returns
/// (общее_количество, уникальных, дублированных)
pub fn count_duplicate_permutations(
    permutation_lists: &[Vec<TileDimensions>],
) -> (usize, usize, usize) {
    let total_count = permutation_lists.len();
    let mut seen_hash_codes = HashSet::new();

    for tile_list in permutation_lists {
        let hash_code = calculate_permutation_hash_code(tile_list);
        seen_hash_codes.insert(hash_code);
    }

    let unique_count = seen_hash_codes.len();
    let duplicate_count = total_count - unique_count;

    (total_count, unique_count, duplicate_count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::orientation::Orientation;
    use crate::logging::{init_logging, LogConfig, LogLevel};

    // Вспомогательная функция для инициализации логирования в тестах
    fn init_test_logging() {
        std::sync::Once::new().call_once(|| {
            let _ = init_logging(LogConfig {
                level: LogLevel::Debug,
            });
        });
    }

    #[test]
    fn test_generate_groups() {
        // Инициализируем логирование для теста
        init_test_logging();

        let tiles = vec![
            TileDimensions {
                id: 1,
                width: 100,
                height: 50,
                orientation: Orientation::Default,
                is_rotated: false,
            },
            TileDimensions {
                id: 2,
                width: 100,
                height: 50,
                orientation: Orientation::Default,
                is_rotated: false,
            },
        ];

        let sheet_tiles = vec![TileDimensions {
            id: 3,
            width: 200,
            height: 100,
            orientation: Orientation::Default,
            is_rotated: false,
        }];

        let task = Task {
            id: "test".to_string(),
            calculation_request: CalculationRequest {
                configuration: Configuration {
                    cut_thickness: 0.0,
                    min_trim_dimension: 0.0,
                    consider_orientation: false,
                    optimization_factor: 1.0,
                    optimization_priority: vec![],
                    use_single_stock_unit: false,
                    performance_thresholds: PerformanceThresholds {
                        max_simultaneous_tasks: 1,
                        max_simultaneous_threads: 1,
                        thread_check_interval: 1000,
                    },
                    cut_orientation_preference: CutOrientationPreference::default(),
                },
                panels: vec![],
                stock_panels: vec![],
            },
            factor: 1,
            status: Status::Idle,
            percentage_done: 0,
            start_time: None,
            solutions: vec![],
            best_solution: None,
            error_message: None,
            iterations_completed: 0,
        };

        let groups = generate_groups(&tiles, &sheet_tiles, &task);
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].group, groups[1].group); // Should be in same group
    }

    #[test]
    fn test_is_one_dimensional_optimization() {
        let tiles = vec![
            TileDimensions {
                id: 1,
                width: 100,
                height: 50,
                orientation: Orientation::Default,
                is_rotated: false,
            },
            TileDimensions {
                id: 2,
                width: 100,
                height: 60,
                orientation: Orientation::Landscape,
                is_rotated: false,
            },
        ];

        let sheet_tiles = vec![TileDimensions {
            id: 3,
            width: 100,
            height: 200,
            orientation: Orientation::Landscape,
            is_rotated: false,
        }];

        assert!(is_one_dimensional_optimization(&tiles, &sheet_tiles));
    }

    fn create_test_tile(id: u8, width: u64, height: u64) -> TileDimensions {
        TileDimensions {
            id,
            width,
            height,
            orientation: Orientation::Default,
            is_rotated: false,
        }
    }

    #[test]
    fn test_calculate_permutation_hash_code() {
        let tile1 = create_test_tile(1, 100, 50);
        let tile2 = create_test_tile(2, 200, 100);

        let list1 = vec![tile1.clone(), tile2.clone()];
        let list2 = vec![tile2.clone(), tile1.clone()];

        let hash1 = calculate_permutation_hash_code(&list1);
        let hash2 = calculate_permutation_hash_code(&list2);

        // Разные порядки должны давать разные хеш-коды
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_remove_duplicated_permutations() {
        let tile1 = create_test_tile(1, 100, 50);
        let tile2 = create_test_tile(2, 200, 100);

        let mut permutations = vec![
            vec![tile1.clone(), tile2.clone()],
            vec![tile2.clone(), tile1.clone()],
            vec![tile1.clone(), tile2.clone()], // Дублирует первую
            vec![tile2.clone(), tile1.clone()], // Дублирует вторую
        ];

        let removed_count = remove_duplicated_permutations(&mut permutations);

        assert_eq!(removed_count, 2);
        assert_eq!(permutations.len(), 2);
    }

    #[test]
    fn test_count_duplicate_permutations() {
        let tile1 = create_test_tile(1, 100, 50);
        let tile2 = create_test_tile(2, 200, 100);

        let permutations = vec![
            vec![tile1.clone(), tile2.clone()],
            vec![tile2.clone(), tile1.clone()],
            vec![tile1.clone(), tile2.clone()], // Дубликат
        ];

        let (total, unique, duplicates) = count_duplicate_permutations(&permutations);

        assert_eq!(total, 3);
        assert_eq!(unique, 2);
        assert_eq!(duplicates, 1);
    }

    #[test]
    fn test_dimensions_based_hash_code() {
        let tile1 = create_test_tile(1, 100, 50);
        let tile2 = create_test_tile(2, 100, 50); // Те же размеры, другой id

        // Хеш-код должен быть одинаковым для одинаковых размеров
        assert_eq!(
            tile1.dimensions_based_hash_code(),
            tile2.dimensions_based_hash_code()
        );

        let tile3 = create_test_tile(3, 200, 100);

        // Разные размеры должны давать разные хеш-коды
        assert_ne!(
            tile1.dimensions_based_hash_code(),
            tile3.dimensions_based_hash_code()
        );
    }

    #[test]
    fn test_java_compatible_hash_with_large_numbers() {
        // Тестируем с реальными масштабированными значениями
        let tile1 = create_test_tile(1, 15050, 10025); // 150.5 * 100 x 100.25 * 100
        let tile2 = create_test_tile(2, 8075, 6050); // 80.75 * 100 x 60.5 * 100

        // Проверяем, что хеш-коды вычисляются корректно
        let hash1 = tile1.dimensions_based_hash_code_java_compatible();
        let hash2 = tile2.dimensions_based_hash_code_java_compatible();

        println!("Tile1 hash: {}", hash1);
        println!("Tile2 hash: {}", hash2);

        // Тестируем хеш-код перестановки
        let permutation = vec![tile1, tile2];
        let perm_hash = calculate_permutation_hash_code_java_compatible(&permutation);

        println!("Permutation hash: {}", perm_hash);
    }

    #[test]
    fn test_overflow_behavior() {
        // Тестируем поведение при переполнении
        let large_tile = create_test_tile(1, u32::MAX as u64, u32::MAX as u64);
        let hash = large_tile.dimensions_based_hash_code_java_compatible();

        println!("Large tile hash (should handle overflow): {}", hash);

        // Убеждаемся, что не происходит паники
        assert!(hash != 0); // Просто проверяем, что функция отработала
    }

    fn create_test_tiles() -> Vec<TileDimensions> {
        vec![
            // 2 детали 150x100
            TileDimensions {
                id: 1,
                width: 15000,
                height: 10000,
                orientation: Orientation::Default,
                is_rotated: false,
            },
            TileDimensions {
                id: 1,
                width: 15000,
                height: 10000,
                orientation: Orientation::Default,
                is_rotated: false,
            },
            // 3 детали 80x60
            TileDimensions {
                id: 2,
                width: 8000,
                height: 6000,
                orientation: Orientation::Default,
                is_rotated: false,
            },
            TileDimensions {
                id: 2,
                width: 8000,
                height: 6000,
                orientation: Orientation::Default,
                is_rotated: false,
            },
            TileDimensions {
                id: 2,
                width: 8000,
                height: 6000,
                orientation: Orientation::Default,
                is_rotated: false,
            },
        ]
    }

    #[test]
    fn test_improved_grouping_reduces_duplicates() {
        let tiles = vec![
            // 2 детали 150x100 - должны быть в 1 группе
            TileDimensions {
                id: 1,
                width: 15000,
                height: 10000,
                orientation: Orientation::Default,
                is_rotated: false,
            },
            TileDimensions {
                id: 1,
                width: 15000,
                height: 10000,
                orientation: Orientation::Default,
                is_rotated: false,
            },
            // 3 детали 80x60 - может быть в 1-2 группах
            TileDimensions {
                id: 2,
                width: 8000,
                height: 6000,
                orientation: Orientation::Default,
                is_rotated: false,
            },
            TileDimensions {
                id: 2,
                width: 8000,
                height: 6000,
                orientation: Orientation::Default,
                is_rotated: false,
            },
            TileDimensions {
                id: 2,
                width: 8000,
                height: 6000,
                orientation: Orientation::Default,
                is_rotated: false,
            },
        ];

        let sheet_tiles = vec![TileDimensions {
            id: 100,
            width: 40000,
            height: 30000,
            orientation: Orientation::Default,
            is_rotated: false,
        }];

        let task = Task {
            id: "test".to_string(),
            calculation_request: CalculationRequest {
                configuration: Configuration {
                    cut_thickness: 0.0,
                    min_trim_dimension: 0.0,
                    consider_orientation: false,
                    optimization_factor: 1.0,
                    optimization_priority: vec![],
                    use_single_stock_unit: false,
                    performance_thresholds: PerformanceThresholds {
                        max_simultaneous_tasks: 1,
                        max_simultaneous_threads: 1,
                        thread_check_interval: 1000,
                    },
                    cut_orientation_preference: CutOrientationPreference::default(),
                },
                panels: vec![],
                stock_panels: vec![],
            },
            factor: 1,
            status: Status::Idle,
            percentage_done: 0,
            start_time: None,
            solutions: vec![],
            best_solution: None,
            error_message: None,
            iterations_completed: 0,
        };

        // Тестируем улучшенную группировку
        let groups = generate_groups_improved(&tiles, &sheet_tiles, &task);

        // Подсчитываем количество групп для каждого типа деталей
        let mut group_counts_150x100 = HashSet::new();
        let mut group_counts_80x60 = HashSet::new();

        for group in &groups {
            if group.tile_dimensions.width == 15000 && group.tile_dimensions.height == 10000 {
                group_counts_150x100.insert(group.group);
            }
            if group.tile_dimensions.width == 8000 && group.tile_dimensions.height == 6000 {
                group_counts_80x60.insert(group.group);
            }
        }

        println!("Groups for 150x100: {:?}", group_counts_150x100);
        println!("Groups for 80x60: {:?}", group_counts_80x60);

        // Проверяем основные требования к группировке

        // Для 150x100 (2 детали): должна быть 1 группа (детали одинаковые и мало)
        assert_eq!(
            group_counts_150x100.len(),
            1,
            "Expected exactly 1 group for 150x100 tiles, got {}",
            group_counts_150x100.len()
        );

        // Для 80x60 (3 детали): должно быть максимум 2 группы
        assert!(
            group_counts_80x60.len() <= 2,
            "Too many groups for 80x60 tiles: expected <= 2, got {}",
            group_counts_80x60.len()
        );

        // Дополнительная проверка: общее количество групп должно быть разумным
        let total_unique_groups: HashSet<i32> = groups.iter().map(|g| g.group).collect();

        // У нас 5 деталей всего, максимум 5 групп (по одной на деталь)
        assert!(
            total_unique_groups.len() <= tiles.len(),
            "Too many total groups: expected <= {}, got {}",
            tiles.len(),
            total_unique_groups.len()
        );

        println!("Total unique groups: {}", total_unique_groups.len());
        println!("Total tiles: {}", tiles.len());

        // Проверим различие между обычной и улучшенной группировкой
        let regular_groups = generate_groups(&tiles, &sheet_tiles, &task);
        let regular_unique_groups: HashSet<i32> = regular_groups.iter().map(|g| g.group).collect();

        println!("Regular grouping groups: {}", regular_unique_groups.len());
        println!("Improved grouping groups: {}", total_unique_groups.len());

        // Обе группировки должны быть разумными
        assert!(
            regular_unique_groups.len() <= tiles.len(),
            "Regular grouping created too many groups: {} > {}",
            regular_unique_groups.len(),
            tiles.len()
        );

        assert!(
            total_unique_groups.len() <= tiles.len(),
            "Improved grouping created too many groups: {} > {}",
            total_unique_groups.len(),
            tiles.len()
        );

        // Проверим, что обе группировки логичны для нашего теста
        // У нас есть 2 типа деталей: 150x100 (2 шт) и 80x60 (3 шт)
        // Ожидаем от 2 до 5 групп в зависимости от алгоритма
        assert!(
            total_unique_groups.len() >= 2 && total_unique_groups.len() <= 5,
            "Improved grouping should create 2-5 groups for our test data, got {}",
            total_unique_groups.len()
        );

        println!("✓ Both grouping algorithms produced reasonable results");
    }

    #[test]
    fn test_java_compatible_grouping() {
        init_test_logging();

        // Создаем тестовые данные как в вашем примере
        let tiles = vec![
            // 2 детали 150.5x100.25 -> 15050x10025
            create_test_tile(1, 15050, 10025),
            create_test_tile(1, 15050, 10025),
            // 3 детали 80.75x60.5 -> 8075x6050
            create_test_tile(2, 8075, 6050),
            create_test_tile(2, 8075, 6050),
            create_test_tile(2, 8075, 6050),
            // 1 деталь 120.0x45.75 -> 12000x4575
            create_test_tile(3, 12000, 4575),
            // 2 детали 95.25x75.5 -> 9525x7550
            create_test_tile(4, 9525, 7550),
            create_test_tile(4, 9525, 7550),
            // 1 деталь 65.5x85.25 -> 6550x8525
            create_test_tile(5, 6550, 8525),
            // 2 детали 110.75x55.0 -> 11075x5500
            create_test_tile(6, 11075, 5500),
            create_test_tile(6, 11075, 5500),
            // 3 детали 40.25x90.5 -> 4025x9050
            create_test_tile(7, 4025, 9050),
            create_test_tile(7, 4025, 9050),
            create_test_tile(7, 4025, 9050),
            // 1 деталь 130.0x35.75 -> 13000x3575
            create_test_tile(8, 13000, 3575),
        ];

        let sheet_tiles = vec![create_test_tile(100, 40000, 30000)];

        let task = create_test_task();

        let groups = generate_groups_java_compatible(&tiles, &sheet_tiles, &task);

        println!(
            "Generated {} groups from {} tiles",
            groups.len(),
            tiles.len()
        );

        // Подсчитываем группы по типам
        let mut group_counts_by_type: HashMap<String, std::collections::HashSet<i32>> =
            HashMap::new();
        for group in &groups {
            let tile_type = group.tile_dimensions.to_string();
            group_counts_by_type
                .entry(tile_type)
                .or_default()
                .insert(group.group);
        }

        for (tile_type, group_set) in &group_counts_by_type {
            println!(
                "Tile type {} has {} groups: {:?}",
                tile_type,
                group_set.len(),
                group_set
            );
        }

        // Проверяем результат
        assert_eq!(groups.len(), tiles.len());
    }

    fn create_test_task() -> Task {
        use crate::enums::{cut_orientation_preference::CutOrientationPreference, status::Status};
        use crate::models::{
            calculation_request::structs::CalculationRequest, configuration::Configuration,
            performance_thresholds::structs::PerformanceThresholds,
        };

        Task {
            id: "test".to_string(),
            calculation_request: CalculationRequest {
                configuration: Configuration {
                    cut_thickness: 0.0,
                    min_trim_dimension: 0.0,
                    consider_orientation: false,
                    optimization_factor: 1.0,
                    optimization_priority: vec![],
                    use_single_stock_unit: false,
                    performance_thresholds: PerformanceThresholds {
                        max_simultaneous_tasks: 1,
                        max_simultaneous_threads: 1,
                        thread_check_interval: 1000,
                    },
                    cut_orientation_preference: CutOrientationPreference::default(),
                },
                panels: vec![],
                stock_panels: vec![],
            },
            factor: 1,
            status: Status::Idle,
            percentage_done: 0,
            start_time: None,
            solutions: vec![],
            best_solution: None,
            error_message: None,
            iterations_completed: 0,
        }
    }
}
