//! Grouped tile dimensions model for cutting optimization
//! 
//! This module provides functionality for managing tile dimensions with group information,
//! extending the base TileDimensions functionality with grouping capabilities.

use crate::models::tile_dimensions::TileDimensions;
use std::fmt;
use std::hash::{Hash, Hasher};

/// Represents tile dimensions with an associated group identifier.
/// This is the Rust equivalent of the Java GroupedTileDimensions class.
#[derive(Debug, Clone)]
pub struct GroupedTileDimensions {
    /// The base tile dimensions
    tile_dimensions: TileDimensions,
    /// Group identifier for this tile
    group: i32,
}

impl GroupedTileDimensions {
    /// Creates a new GroupedTileDimensions by copying from another instance
    pub fn from_grouped(other: &GroupedTileDimensions) -> Self {
        Self {
            tile_dimensions: other.tile_dimensions.clone(),
            group: other.group,
        }
    }

    /// Creates a new GroupedTileDimensions from existing TileDimensions and group
    pub fn from_tile_dimensions(tile_dimensions: TileDimensions, group: i32) -> Self {
        Self {
            tile_dimensions,
            group,
        }
    }

    /// Creates a new GroupedTileDimensions with width, height, and group
    pub fn new(width: u32, height: u32, group: i32) -> Self {
        Self {
            tile_dimensions: TileDimensions::simple(width, height),
            group,
        }
    }

    /// Creates a new GroupedTileDimensions with all TileDimensions parameters and group
    pub fn new_with_full_params(
        id: i32,
        width: u32,
        height: u32,
        material: String,
        orientation: u32,
        label: Option<String>,
        is_rotated: bool,
        group: i32,
    ) -> Self {
        Self {
            tile_dimensions: TileDimensions::new(
                id,
                width,
                height,
                material,
                orientation,
                label,
                is_rotated,
            ),
            group,
        }
    }

    /// Gets the group identifier
    pub fn group(&self) -> i32 {
        self.group
    }

    /// Gets a reference to the underlying TileDimensions
    pub fn tile_dimensions(&self) -> &TileDimensions {
        &self.tile_dimensions
    }

    /// Gets the tile ID (delegated to TileDimensions)
    pub fn id(&self) -> i32 {
        self.tile_dimensions.id()
    }

    /// Gets the tile width (delegated to TileDimensions)
    pub fn width(&self) -> u32 {
        self.tile_dimensions.width()
    }

    /// Gets the tile height (delegated to TileDimensions)
    pub fn height(&self) -> u32 {
        self.tile_dimensions.height()
    }

    /// Gets the tile material (delegated to TileDimensions)
    pub fn material(&self) -> &str {
        self.tile_dimensions.material()
    }

    /// Gets the tile orientation (delegated to TileDimensions)
    pub fn orientation(&self) -> u32 {
        self.tile_dimensions.orientation()
    }

    /// Gets the tile label (delegated to TileDimensions)
    pub fn label(&self) -> Option<&str> {
        self.tile_dimensions.label()
    }

    /// Checks if the tile is rotated (delegated to TileDimensions)
    pub fn is_rotated(&self) -> bool {
        self.tile_dimensions.is_rotated()
    }

    /// Gets the maximum dimension (delegated to TileDimensions)
    pub fn max_dimension(&self) -> u32 {
        self.tile_dimensions.max_dimension()
    }

    /// Calculates the area of the tile (delegated to TileDimensions)
    pub fn area(&self) -> u64 {
        self.tile_dimensions.area()
    }

    /// Creates a new GroupedTileDimensions rotated 90 degrees
    pub fn rotate_90(&self) -> Self {
        Self {
            tile_dimensions: self.tile_dimensions.rotate_90(),
            group: self.group,
        }
    }

    /// Checks if the tile is square (delegated to TileDimensions)
    pub fn is_square(&self) -> bool {
        self.tile_dimensions.is_square()
    }

    /// Checks if the tile is horizontal (delegated to TileDimensions)
    pub fn is_horizontal(&self) -> bool {
        self.tile_dimensions.is_horizontal()
    }

    /// Returns dimensions as a string (delegated to TileDimensions)
    pub fn dimensions_to_string(&self) -> String {
        self.tile_dimensions.dimensions_to_string()
    }

    /// Checks if this tile has the same dimensions as another tile (delegated to TileDimensions)
    pub fn has_same_dimensions(&self, other: &GroupedTileDimensions) -> bool {
        self.tile_dimensions.has_same_dimensions(&other.tile_dimensions)
    }

    /// Checks if this tile can fit another tile (delegated to TileDimensions)
    pub fn fits(&self, other: &GroupedTileDimensions) -> bool {
        self.tile_dimensions.fits(&other.tile_dimensions)
    }

    /// Calculates hash code based on dimensions and group
    pub fn hash_code(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl fmt::Display for GroupedTileDimensions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Note: The original Java had a typo "gropup" instead of "group"
        // Keeping it for compatibility, but you may want to fix this
        write!(
            f,
            "id={}, gropup={}[{}x{}]",
            self.id(),
            self.group,
            self.width(),
            self.height()
        )
    }
}

impl PartialEq for GroupedTileDimensions {
    fn eq(&self, other: &Self) -> bool {
        self.tile_dimensions == other.tile_dimensions && self.group == other.group
    }
}

impl Eq for GroupedTileDimensions {}

impl Hash for GroupedTileDimensions {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.tile_dimensions.hash(state);
        self.group.hash(state);
    }
}

// Implement Deref to allow direct access to TileDimensions methods
impl std::ops::Deref for GroupedTileDimensions {
    type Target = TileDimensions;

    fn deref(&self) -> &Self::Target {
        &self.tile_dimensions
    }
}
