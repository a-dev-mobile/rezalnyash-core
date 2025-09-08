use serde::Serialize;

use super::tile_dimensions::TileDimensions;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Serialize)]
pub struct GroupedTileDimensions {
    pub group: u8,
    pub instance: TileDimensions,
}
impl GroupedTileDimensions {
    /// Calculates the area of the tile (delegated to TileDimensions)
    pub fn area(&self) -> u64 {
        self.instance.area()
    }

    pub fn id(&self) -> u32 {
        self.instance.id
    }

    pub fn width(&self) -> u32 {
        self.instance.width
    }

    pub fn height(&self) -> u32 {
        self.instance.height
    }

    pub fn is_rotated(&self) -> bool {
        self.instance.is_rotated
    }

    pub fn label(&self) -> Option<&str> {
        Some(&self.instance.label)
    }

    pub fn material(&self) -> &str {
        &self.instance.material
    }
    pub(crate) fn from_tile_dimension(tile_dimension: TileDimensions, group: u8) -> Self {
        Self {
            group,
            instance: tile_dimension,
        }
    }

    pub fn from_tile_dimensions(tile_dimension: TileDimensions, group: u8) -> Self {
        Self {
            group,
            instance: tile_dimension,
        }
    }
}
impl fmt::Display for GroupedTileDimensions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // ТОЧНАЯ копия Java toString(): "id=" + this.id + ", gropup=" + this.group + "[" + this.width + "x" + this.height + ']'

        write!(
            f,
            "id={}, group={}[{}x{}]",
            self.instance.id, self.group, self.instance.width, self.instance.height
        )
    }
}

// Implement PartialEq to compare GroupedTileDimensions instances
impl PartialEq for GroupedTileDimensions {
    fn eq(&self, other: &Self) -> bool {
        // Compare id, width, height, and group (as mentioned in the comment in basic_usage.rs)
        self.instance.id == other.instance.id &&
        self.instance.width == other.instance.width &&
        self.instance.height == other.instance.height &&
        self.group == other.group
    }
}

// Implement Eq trait which is required for Hash
impl Eq for GroupedTileDimensions {}

// Implement Hash trait
impl Hash for GroupedTileDimensions {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the same fields used in equality comparison
        self.instance.id.hash(state);
        self.instance.width.hash(state);
        self.instance.height.hash(state);
        self.group.hash(state);
    }
}
