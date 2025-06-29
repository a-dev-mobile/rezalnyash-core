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