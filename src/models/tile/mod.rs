use serde::{Deserialize, Serialize};

/// Represents a rectangular tile with coordinates defining its position and boundaries
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Tile {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}

impl Tile {
  

    /// Create a new tile with explicit coordinates
    pub fn new(x1: i32, x2: i32, y1: i32, y2: i32) -> Self {
        Self { x1, x2, y1, y2 }
    }

    /// Create a copy of an existing tile
    pub fn from_tile(tile: &Tile) -> Self {
        Self {
            x1: tile.x1,
            x2: tile.x2,
            y1: tile.y1,
            y2: tile.y2,
        }
    }

    /// Get the x1 coordinate
    pub fn x1(&self) -> i32 {
        self.x1
    }

    /// Get the x2 coordinate
    pub fn x2(&self) -> i32 {
        self.x2
    }

    /// Get the y1 coordinate
    pub fn y1(&self) -> i32 {
        self.y1
    }

    /// Get the y2 coordinate
    pub fn y2(&self) -> i32 {
        self.y2
    }

    /// Calculate the width of the tile
    pub fn width(&self) -> i32 {
        self.x2 - self.x1
    }

    /// Calculate the height of the tile
    pub fn height(&self) -> i32 {
        self.y2 - self.y1
    }

    /// Calculate the area of the tile
    pub fn area(&self) -> i64 {
        (self.width() as i64) * (self.height() as i64)
    }

    /// Get the maximum side length (width or height)
    pub fn max_side(&self) -> i32 {
        self.width().max(self.height())
    }

    /// Check if the tile is horizontally oriented (width > height)
    pub fn is_horizontal(&self) -> bool {
        self.width() > self.height()
    }

    /// Check if the tile is vertically oriented (height > width)
    pub fn is_vertical(&self) -> bool {
        self.height() > self.width()
    }

    /// Check if the tile is square (width == height)
    pub fn is_square(&self) -> bool {
        self.width() == self.height()
    }

    /// Get the minimum side length (width or height)
    pub fn min_side(&self) -> i32 {
        self.width().min(self.height())
    }

    /// Check if this tile contains a point
    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        x >= self.x1 && x < self.x2 && y >= self.y1 && y < self.y2
    }

    /// Check if this tile overlaps with another tile
    pub fn overlaps_with(&self, other: &Tile) -> bool {
        !(self.x2 <= other.x1 || other.x2 <= self.x1 || self.y2 <= other.y1 || other.y2 <= self.y1)
    }

    /// Move the tile by the specified offset
    pub fn translate(&mut self, dx: i32, dy: i32) {
        self.x1 += dx;
        self.x2 += dx;
        self.y1 += dy;
        self.y2 += dy;
    }

    /// Create a new tile translated by the specified offset
    pub fn translated(&self, dx: i32, dy: i32) -> Self {
        Self {
            x1: self.x1 + dx,
            x2: self.x2 + dx,
            y1: self.y1 + dy,
            y2: self.y2 + dy,
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            x1: 0,
            x2: 0,
            y1: 0,
            y2: 0,
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Tile[({}, {}) -> ({}, {}), {}x{}]",
            self.x1,
            self.y1,
            self.x2,
            self.y2,
            self.width(),
            self.height()
        )
    }
}


use crate::models::{
    tile_dimensions::TileDimensions,
    grouped_tile_dimensions::GroupedTileDimensions,
};

/// Преобразует список GroupedTileDimensions в отсортированный список TileDimensions
/// 
/// Аналог Java метода groupedTileDimensionsList2TileDimensionsList
/// 
/// # Arguments
/// * `grouped_list` - Список сгруппированных размеров плиток в нужном порядке
/// * `original_tiles` - Исходный список всех плиток
/// 
/// # Returns
/// Отсортированный список TileDimensions согласно порядку в grouped_list
pub fn grouped_tile_dimensions_list_to_tile_dimensions_list(
    grouped_list: &[GroupedTileDimensions],
    original_tiles: &[impl AsRef<TileDimensions>],
) -> Vec<TileDimensions> {
    let mut result: Vec<TileDimensions> = original_tiles
        .iter()
        .map(|tile| tile.as_ref().clone())
        .collect();

    // Сортируем согласно порядку в grouped_list
    result.sort_by(|tile_a, tile_b| {
        // Находим индексы соответствующих GroupedTileDimensions в списке
        let index_a = find_grouped_tile_index(grouped_list, tile_a);
        let index_b = find_grouped_tile_index(grouped_list, tile_b);
        
        index_a.cmp(&index_b)
    });

    result
}

/// Находит индекс GroupedTileDimensions, соответствующего данной TileDimensions
/// 
/// # Arguments
/// * `grouped_list` - Список сгруппированных размеров плиток
/// * `tile` - Плитка для поиска
/// 
/// # Returns
/// Индекс в grouped_list или usize::MAX если не найдено
fn find_grouped_tile_index(
    grouped_list: &[GroupedTileDimensions], 
    tile: &TileDimensions
) -> usize {
    // Сначала ищем точное совпадение (включая группу)
    for (index, grouped_tile) in grouped_list.iter().enumerate() {
        if tiles_match(&grouped_tile.tile_dimensions, tile) {
            return index;
        }
    }
    
    // Если точного совпадения нет, ищем по размерам и id
    for (index, grouped_tile) in grouped_list.iter().enumerate() {
        if grouped_tile.tile_dimensions.id == tile.id
            && grouped_tile.tile_dimensions.width == tile.width
            && grouped_tile.tile_dimensions.height == tile.height
            && grouped_tile.tile_dimensions.orientation == tile.orientation
        {
            return index;
        }
    }
    
    // Если не найдено, возвращаем максимальное значение (будет в конце сортировки)
    usize::MAX
}

/// Проверяет, соответствуют ли две плитки друг другу
/// 
/// # Arguments
/// * `tile_a` - Первая плитка
/// * `tile_b` - Вторая плитка
/// 
/// # Returns
/// true если плитки соответствуют
fn tiles_match(tile_a: &TileDimensions, tile_b: &TileDimensions) -> bool {
    tile_a.id == tile_b.id
        && tile_a.width == tile_b.width
        && tile_a.height == tile_b.height
        && tile_a.orientation == tile_b.orientation
        && tile_a.is_rotated == tile_b.is_rotated
}

/// Упрощенная версия функции преобразования для случая, когда GroupedTileDimensions
/// содержат все необходимые TileDimensions
/// 
/// # Arguments
/// * `grouped_list` - Список сгруппированных размеров плиток в нужном порядке
/// 
/// # Returns
/// Список TileDimensions в том же порядке
pub fn grouped_to_tile_dimensions_simple(
    grouped_list: &[GroupedTileDimensions]
) -> Vec<TileDimensions> {
    grouped_list
        .iter()
        .map(|grouped| grouped.tile_dimensions.clone())
        .collect()
}

// Реализация AsRef<TileDimensions> для TileDimensions (для совместимости)
impl AsRef<TileDimensions> for TileDimensions {
    fn as_ref(&self) -> &TileDimensions {
        self
    }
}

// Реализация AsRef<TileDimensions> для GroupedTileDimensions
impl AsRef<TileDimensions> for GroupedTileDimensions {
    fn as_ref(&self) -> &TileDimensions {
        &self.tile_dimensions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::orientation::Orientation;

    #[test]
    fn test_grouped_to_tile_dimensions_simple() {
        let tile1 = TileDimensions {
            id: 1,
            width: 100,
            height: 50,
            orientation: Orientation::Default,
            is_rotated: false,
        };

        let tile2 = TileDimensions {
            id: 2,
            width: 200,
            height: 100,
            orientation: Orientation::Default,
            is_rotated: false,
        };

        let grouped_list = vec![
            GroupedTileDimensions::new(tile2.clone(), 0),
            GroupedTileDimensions::new(tile1.clone(), 1),
        ];

        let result = grouped_to_tile_dimensions_simple(&grouped_list);
        
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], tile2);
        assert_eq!(result[1], tile1);
    }

    #[test]
    fn test_tiles_match() {
        let tile1 = TileDimensions {
            id: 1,
            width: 100,
            height: 50,
            orientation: Orientation::Default,
            is_rotated: false,
        };

        let tile2 = TileDimensions {
            id: 1,
            width: 100,
            height: 50,
            orientation: Orientation::Default,
            is_rotated: false,
        };

        let tile3 = TileDimensions {
            id: 2,
            width: 100,
            height: 50,
            orientation: Orientation::Default,
            is_rotated: false,
        };

        assert!(tiles_match(&tile1, &tile2));
        assert!(!tiles_match(&tile1, &tile3));
    }

    #[test]
    fn test_find_grouped_tile_index() {
        let tile1 = TileDimensions {
            id: 1,
            width: 100,
            height: 50,
            orientation: Orientation::Default,
            is_rotated: false,
        };

        let tile2 = TileDimensions {
            id: 2,
            width: 200,
            height: 100,
            orientation: Orientation::Default,
            is_rotated: false,
        };

        let grouped_list = vec![
            GroupedTileDimensions::new(tile1.clone(), 0),
            GroupedTileDimensions::new(tile2.clone(), 1),
        ];

        assert_eq!(find_grouped_tile_index(&grouped_list, &tile1), 0);
        assert_eq!(find_grouped_tile_index(&grouped_list, &tile2), 1);
        
        let tile3 = TileDimensions {
            id: 3,
            width: 300,
            height: 150,
            orientation: Orientation::Default,
            is_rotated: false,
        };
        
        assert_eq!(find_grouped_tile_index(&grouped_list, &tile3), usize::MAX);
    }
}