use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    enums::{
        cut_orientation_preference::CutOrientationPreference, optimization_priority::OptimizationPriority, orientation::Orientation, status::Status
    }, log_debug, models::{
        calculation_request::structs::CalculationRequest, configuration::Configuration, grouped_tile_dimensions::GroupedTileDimensions, performance_thresholds::structs::PerformanceThresholds, task::structs::Task
    }
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

        let sheet_tiles = vec![
            TileDimensions {
                id: 3,
                width: 200,
                height: 100,
                orientation: Orientation::Default,
                is_rotated: false,
            },
        ];

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

        let sheet_tiles = vec![
            TileDimensions {
                id: 3,
                width: 100,
                height: 200,
                orientation: Orientation::Landscape,
                is_rotated: false,
            },
        ];

        assert!(is_one_dimensional_optimization(&tiles, &sheet_tiles));
    }
}
