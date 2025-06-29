use crate::{
    enums::orientation::Orientation,
    models::{ configuration::Configuration, task::structs::Task, tile_dimensions::TileDimensions},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents tile dimensions with an associated group identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GroupedTileDimensions {
    /// The base tile dimensions (composition instead of inheritance)
    pub tile_dimensions: TileDimensions,
    /// The group identifier for this tile
    pub group: i32,
}

impl GroupedTileDimensions {
    /// Create a new GroupedTileDimensions from another GroupedTileDimensions
    pub fn from_grouped(other: &GroupedTileDimensions) -> Self {
        Self {
            tile_dimensions: other.tile_dimensions.clone(),
            group: other.group,
        }
    }

    /// Create a new GroupedTileDimensions from TileDimensions and group
    pub fn new(tile_dimensions: TileDimensions, group: i32) -> Self {
        Self {
            tile_dimensions,
            group,
        }
    }

    /// Create a new GroupedTileDimensions with direct dimensions
    pub fn with_dimensions(width: u64, height: u64, group: i32) -> Self {
        Self {
            tile_dimensions: TileDimensions {
                id: 0, // Default ID
                width,
                height,
                orientation: crate::enums::orientation::Orientation::Default, // Default orientation
                is_rotated: false,
            },
            group,
        }
    }

    pub fn get_group(&self) -> i32 {
        self.group
    }

    /// Calculate the area of the tile
    pub fn get_area(&self) -> u64 {
        (self.tile_dimensions.width ) * (self.tile_dimensions.height)
    }
}

impl std::fmt::Display for GroupedTileDimensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id={}, group={} [{}x{}]",
            self.tile_dimensions.id,
            self.group,
            self.tile_dimensions.width,
            self.tile_dimensions.height
        )
    }
}

pub fn get_distinct_grouped_tile_dimensions<T>(
    list: &[T], 
    _configuration: &Configuration
) -> HashMap<T, i32> 
where 
    T: Clone + Eq + std::hash::Hash,
{
    let mut map = HashMap::new();
    
    for item in list {
        let count = map.entry(item.clone()).or_insert(0);
        *count += 1;
    }
    
    map
}


/// Специализированная версия для GroupedTileDimensions
pub fn get_distinct_grouped_tile_dimensions_for_tiles(
    list: &[GroupedTileDimensions], 
    _configuration: &Configuration
) -> HashMap<GroupedTileDimensions, i32> {
    get_distinct_grouped_tile_dimensions(list, _configuration)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::tile_dimensions::TileDimensions;
    use crate::enums::orientation::Orientation;
    
    #[test]
    fn test_get_distinct_grouped_tile_dimensions() {
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
        
        let grouped_tiles = vec![
            GroupedTileDimensions::new(tile1, 0),
            GroupedTileDimensions::new(tile2, 0),
        ];
        
        let config = Configuration::default();
        let result = get_distinct_grouped_tile_dimensions_for_tiles(&grouped_tiles, &config);
        
        // Если tile1 и tile2 одинаковые и в одной группе, должно быть 1 уникальный элемент с count = 2
        assert_eq!(result.len(), 1);
        assert_eq!(*result.values().next().unwrap(), 2);
    }

    #[test]
    fn test_grouped_tile_dimensions_equality() {
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
        
        let grouped1 = GroupedTileDimensions::new(tile1, 0);
        let grouped2 = GroupedTileDimensions::new(tile2, 0);
        
        // Проверяем, что одинаковые элементы равны
        assert_eq!(grouped1, grouped2);
    }

    #[test]
    fn test_grouped_tile_dimensions_different_groups() {
        let tile = TileDimensions {
            id: 1,
            width: 100,
            height: 50,
            orientation: Orientation::Default,
            is_rotated: false,
        };
        
        let grouped1 = GroupedTileDimensions::new(tile.clone(), 0);
        let grouped2 = GroupedTileDimensions::new(tile, 1);
        
        // Проверяем, что элементы с разными группами не равны
        assert_ne!(grouped1, grouped2);
    }
}