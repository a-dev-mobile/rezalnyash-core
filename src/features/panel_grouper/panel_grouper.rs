// features/panel_grouper.rs
use crate::features::input::models::{
    grouped_tile_dimensions::GroupedTileDimensions, tile_dimensions::TileDimensions,
};

use std::collections::HashMap;

/// Отвечает за группировку панелей по размерам (точная копия Java generateGroups)
pub struct PanelGrouper;

impl PanelGrouper {
    /// Check if optimization is one-dimensional
    fn is_one_dimensional_optimization(tiles: &[TileDimensions], stock_tiles: &[TileDimensions]) -> bool {
        if tiles.is_empty() {
            return false;
        }

        // Initialize with first tile's dimensions
        let mut common_dimensions = vec![tiles[0].width, tiles[0].height];

        // Process all tiles
        for tile in tiles {
            let mut surviving_dimensions = Vec::new();

            for &dim in &common_dimensions {
                if dim == tile.width || dim == tile.height {
                    surviving_dimensions.push(dim);
                }
            }

            common_dimensions = surviving_dimensions;

            // Early exit if no common dimensions remain
            if common_dimensions.is_empty() {
                return false;
            }
        }

        // Process stock tiles
        for tile in stock_tiles {
            let mut surviving_dimensions = Vec::new();

            for &dim in &common_dimensions {
                if dim == tile.width || dim == tile.height {
                    surviving_dimensions.push(dim);
                }
            }

            common_dimensions = surviving_dimensions;

            // Early exit if no common dimensions remain
            if common_dimensions.is_empty() {
                return false;
            }
        }

        !common_dimensions.is_empty()
    }

    /// Группирует панели - точная копия Java логики generateGroups
    pub fn group_panels(tiles: &[TileDimensions], stock_tiles: &[TileDimensions]) -> Vec<GroupedTileDimensions> {
        if tiles.is_empty() {
            return Vec::new();
        }

        // Шаг 1: Подсчет количества каждого типа панели (map в Java)
        let mut tile_counts = HashMap::new();
        for tile in tiles {
            let key = tile.to_string(); // используем toString() как в Java
            let count = tile_counts.entry(key).or_insert(0);
            *count += 1;
        }

        // Логирование статистики панелей
        let mut sb = String::new();
        for (tile_type, count) in &tile_counts {
            sb.push_str(&format!("{}*{} ", tile_type, count));
        }
        println!("TotalNbrTiles[{}] Tiles: {}", tiles.len(), sb);

        // Шаг 2: Определение порога для разбивки (iMax в Java)
        let mut max_group_size = std::cmp::max(tiles.len() / 100, 1);

        // Шаг 3: Проверка одномерной оптимизации
        if Self::is_one_dimensional_optimization(tiles, stock_tiles) {
        
            max_group_size = 1;
        }

        // Шаг 4: Группировка панелей - точная копия Java логики
        let mut result = Vec::new();
        let mut current_group = 0; // переменная i в Java
        let mut group_counts = HashMap::new(); // map2 в Java

        for tile in tiles {
            let tile_key = tile.to_string();
            let group_key = format!("{}{}", tile_key, current_group); // str2 в Java

            // Увеличиваем счетчик для данной группы (точная копия Java логики map2.put)
            let group_count = group_counts.entry(group_key.clone()).or_insert(0);
            *group_count += 1;

            // Добавляем панель в текущую группу
            result.push(GroupedTileDimensions::from_tile_dimension(
                tile.clone(),
                current_group as u8,
            ));

            // Проверяем условие разбивки ПОСЛЕ добавления панели (точная копия Java логики)
            let total_for_tile_type = tile_counts.get(&tile_key).unwrap_or(&0);
            
            // Точная копия Java условия:
            // ((Integer) map.get(tileDimensions.toString())).intValue() > iMax && 
            // ((Integer) map2.get(str2)).intValue() > ((Integer) map.get(tileDimensions.toString())).intValue() / 4
            if *total_for_tile_type > max_group_size && *group_count > (*total_for_tile_type / 4) {
              
                current_group += 1;
                // НЕ очищаем group_counts! В Java map2 не очищается
            }
        }

        result
    }
}



