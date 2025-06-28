use crate::{
    enums::orientation::Orientation,
    models::{task::structs::Task, tile_dimensions::TileDimensions},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents tile dimensions with an associated group identifier
#[derive(Debug, Clone, PartialEq)]
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
    pub fn with_dimensions(width: u32, height: u32, group: i32) -> Self {
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

impl std::hash::Hash for GroupedTileDimensions {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tile_dimensions.hash(state);
        self.group.hash(state);
    }
}