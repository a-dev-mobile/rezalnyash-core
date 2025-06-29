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
        performance_thresholds::structs::PerformanceThresholds, task::structs::Task,
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
        write!(
            f,
            "id={}, width={}, height={}, orientation={:?}, is_rotated={}",
            self.id, self.width, self.height, self.orientation, self.is_rotated
        )
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
        let key = tile.to_string();
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

/// Check if the optimization is one-dimensional
fn is_one_dimensional_optimization(
    tiles: &[TileDimensions],
    sheet_tiles: &[TileDimensions],
) -> bool {
    if tiles.is_empty() {
        return false;
    }

    let mut common_dimensions = vec![tiles[0].width, tiles[0].height];

    // Check tiles
    for tile in tiles {
        common_dimensions.retain(|&dim| dim == tile.width || dim == tile.height);
        if common_dimensions.is_empty() {
            return false;
        }
    }

    // Check sheet tiles
    for sheet_tile in sheet_tiles {
        common_dimensions.retain(|&dim| dim == sheet_tile.width || dim == sheet_tile.height);
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
                    optimization_factor: 1,
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
}
