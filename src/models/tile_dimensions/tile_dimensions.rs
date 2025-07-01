//! Tile dimensions model for cutting optimization
//! 
//! This module provides functionality for managing tile dimensions with validation,
//! area calculations, rotation operations, and fitting checks.

use std::fmt;
use std::hash::{Hash, Hasher};

/// Default material constant for tiles
pub const DEFAULT_MATERIAL: &str = "DEFAULT_MATERIAL";

/// Represents the dimensions and properties of a tile for cutting optimization
#[derive(Debug, Clone)]
pub struct TileDimensions {
    /// Unique identifier for the tile
    id: i32,
    /// Width of the tile
    width: u32,
    /// Height of the tile
    height: u32,
    /// Material type of the tile
    material: String,
    /// Orientation value (0, 1, 2, etc.)
    orientation: u32,
    /// Optional label for the tile
    label: Option<String>,
    /// Whether the tile has been rotated
    is_rotated: bool,
}

impl TileDimensions {
    /// Creates a new TileDimensions with all parameters
    pub fn new(
        id: i32,
        width: u32,
        height: u32,
        material: String,
        orientation: u32,
        label: Option<String>,
        is_rotated: bool,
    ) -> Self {
        Self {
            id,
            width,
            height,
            material,
            orientation,
            label,
            is_rotated,
        }
    }

    /// Creates a new TileDimensions with default rotation (false)
    pub fn new_with_defaults(
        id: i32,
        width: u32,
        height: u32,
        material: String,
        orientation: u32,
        label: Option<String>,
    ) -> Self {
        Self::new(id, width, height, material, orientation, label, false)
    }

    /// Creates a simple TileDimensions with just width and height
    /// Uses default values for other fields
    pub fn simple(width: u32, height: u32) -> Self {
        Self {
            id: -1,
            width,
            height,
            material: DEFAULT_MATERIAL.to_string(),
            orientation: 0,
            label: None,
            is_rotated: false,
        }
    }

    /// Gets the tile ID
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Gets the tile width
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Gets the tile height
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Gets the tile material
    pub fn material(&self) -> &str {
        &self.material
    }

    /// Gets the tile orientation
    pub fn orientation(&self) -> u32 {
        self.orientation
    }

    /// Gets the tile label
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Checks if the tile is rotated
    pub fn is_rotated(&self) -> bool {
        self.is_rotated
    }

    /// Gets the maximum dimension (width or height)
    pub fn max_dimension(&self) -> u32 {
        self.width.max(self.height)
    }

    /// Calculates the area of the tile
    pub fn area(&self) -> u64 {
        self.width as u64 * self.height as u64
    }

    /// Creates a new TileDimensions rotated 90 degrees
    pub fn rotate_90(&self) -> Self {
        let new_orientation = if self.orientation == 1 { 2 } else { 1 };
        
        Self {
            id: self.id,
            width: self.height,
            height: self.width,
            material: self.material.clone(),
            orientation: new_orientation,
            label: self.label.clone(),
            is_rotated: true,
        }
    }

    /// Checks if the tile is square (width equals height)
    pub fn is_square(&self) -> bool {
        self.width == self.height
    }

    /// Checks if the tile is horizontal (width > height)
    pub fn is_horizontal(&self) -> bool {
        self.width > self.height
    }

    /// Returns dimensions as a string in "widthxheight" format
    pub fn dimensions_to_string(&self) -> String {
        format!("{}x{}", self.width, self.height)
    }

    /// Checks if this tile has the same dimensions as another tile
    /// Considers both orientations (width×height and height×width)
    pub fn has_same_dimensions(&self, other: &TileDimensions) -> bool {
        (self.width == other.width && self.height == other.height) ||
        (self.width == other.height && self.height == other.width)
    }

    /// Checks if this tile can fit another tile (considering rotation)
    pub fn fits(&self, other: &TileDimensions) -> bool {
        (self.width >= other.width && self.height >= other.height) ||
        (self.width >= other.height && self.height >= other.width)
    }

    /// Calculates hash code based only on dimensions (width and height)
    pub fn dimensions_based_hash_code(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        (self.width * 31 + self.height).hash(&mut hasher);
        hasher.finish()
    }
}

impl fmt::Display for TileDimensions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "id={}[{}x{}]", self.id, self.width, self.height)
    }
}

impl PartialEq for TileDimensions {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.width == other.width && self.height == other.height
    }
}

impl Eq for TileDimensions {}

impl Hash for TileDimensions {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.width.hash(state);
        self.height.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tile_dimensions() {
        let tile = TileDimensions::new(
            1,
            100,
            200,
            "Wood".to_string(),
            1,
            Some("Test Tile".to_string()),
            false,
        );

        assert_eq!(tile.id(), 1);
        assert_eq!(tile.width(), 100);
        assert_eq!(tile.height(), 200);
        assert_eq!(tile.material(), "Wood");
        assert_eq!(tile.orientation(), 1);
        assert_eq!(tile.label(), Some("Test Tile"));
        assert!(!tile.is_rotated());
    }

    #[test]
    fn test_simple_constructor() {
        let tile = TileDimensions::simple(50, 75);

        assert_eq!(tile.id(), -1);
        assert_eq!(tile.width(), 50);
        assert_eq!(tile.height(), 75);
        assert_eq!(tile.material(), DEFAULT_MATERIAL);
        assert_eq!(tile.orientation(), 0);
        assert_eq!(tile.label(), None);
        assert!(!tile.is_rotated());
    }

    #[test]
    fn test_area_calculation() {
        let tile = TileDimensions::simple(10, 20);
        assert_eq!(tile.area(), 200);
    }

    #[test]
    fn test_max_dimension() {
        let tile1 = TileDimensions::simple(10, 20);
        let tile2 = TileDimensions::simple(30, 15);

        assert_eq!(tile1.max_dimension(), 20);
        assert_eq!(tile2.max_dimension(), 30);
    }

    #[test]
    fn test_rotate_90() {
        let tile = TileDimensions::new(
            1,
            100,
            200,
            "Wood".to_string(),
            1,
            Some("Test".to_string()),
            false,
        );

        let rotated = tile.rotate_90();

        assert_eq!(rotated.width(), 200);
        assert_eq!(rotated.height(), 100);
        assert_eq!(rotated.orientation(), 2);
        assert!(rotated.is_rotated());
        assert_eq!(rotated.id(), tile.id());
        assert_eq!(rotated.material(), tile.material());
    }

    #[test]
    fn test_is_square() {
        let square_tile = TileDimensions::simple(100, 100);
        let rect_tile = TileDimensions::simple(100, 200);

        assert!(square_tile.is_square());
        assert!(!rect_tile.is_square());
    }

    #[test]
    fn test_is_horizontal() {
        let horizontal_tile = TileDimensions::simple(200, 100);
        let vertical_tile = TileDimensions::simple(100, 200);
        let square_tile = TileDimensions::simple(100, 100);

        assert!(horizontal_tile.is_horizontal());
        assert!(!vertical_tile.is_horizontal());
        assert!(!square_tile.is_horizontal());
    }

    #[test]
    fn test_dimensions_to_string() {
        let tile = TileDimensions::simple(100, 200);
        assert_eq!(tile.dimensions_to_string(), "100x200");
    }

    #[test]
    fn test_has_same_dimensions() {
        let tile1 = TileDimensions::simple(100, 200);
        let tile2 = TileDimensions::simple(100, 200);
        let tile3 = TileDimensions::simple(200, 100);
        let tile4 = TileDimensions::simple(150, 200);

        assert!(tile1.has_same_dimensions(&tile2));
        assert!(tile1.has_same_dimensions(&tile3)); // Rotated version
        assert!(!tile1.has_same_dimensions(&tile4));
    }

    #[test]
    fn test_fits() {
        let large_tile = TileDimensions::simple(200, 300);
        let small_tile = TileDimensions::simple(100, 150);
        let rotated_small = TileDimensions::simple(150, 100);
        let too_large = TileDimensions::simple(250, 350); // Both dimensions larger

        assert!(large_tile.fits(&small_tile));
        assert!(large_tile.fits(&rotated_small));
        assert!(!large_tile.fits(&too_large));
        
        // Test edge case where one dimension fits when rotated
        let edge_case = TileDimensions::simple(250, 200);
        assert!(large_tile.fits(&edge_case)); // 300 >= 250 && 200 >= 200 when rotated
    }

    #[test]
    fn test_display() {
        let tile = TileDimensions::simple(100, 200);
        assert_eq!(format!("{}", tile), "id=-1[100x200]");
    }

    #[test]
    fn test_equality() {
        let tile1 = TileDimensions::new(
            1,
            100,
            200,
            "Wood".to_string(),
            1,
            Some("Test".to_string()),
            false,
        );
        let tile2 = TileDimensions::new(
            1,
            100,
            200,
            "Metal".to_string(), // Different material
            2,                   // Different orientation
            None,                // Different label
            true,                // Different rotation
        );
        let tile3 = TileDimensions::new(
            2,
            100,
            200,
            "Wood".to_string(),
            1,
            Some("Test".to_string()),
            false,
        );

        // Equality is based only on id, width, and height
        assert_eq!(tile1, tile2);
        assert_ne!(tile1, tile3);
    }

    #[test]
    fn test_hash() {
        let tile1 = TileDimensions::simple(100, 200);
        let tile2 = TileDimensions::simple(100, 200);
        let tile3 = TileDimensions::simple(150, 200);

        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(tile1.clone(), "value1");
        
        // Same dimensions should find the same entry
        assert_eq!(map.get(&tile2), Some(&"value1"));
        assert_eq!(map.get(&tile3), None);
    }

    #[test]
    fn test_dimensions_based_hash_code() {
        let tile1 = TileDimensions::simple(100, 200);
        let tile2 = TileDimensions::simple(100, 200);
        let tile3 = TileDimensions::simple(150, 200);

        assert_eq!(tile1.dimensions_based_hash_code(), tile2.dimensions_based_hash_code());
        assert_ne!(tile1.dimensions_based_hash_code(), tile3.dimensions_based_hash_code());
    }

    #[test]
    fn test_clone() {
        let original = TileDimensions::new(
            1,
            100,
            200,
            "Wood".to_string(),
            1,
            Some("Test".to_string()),
            false,
        );

        let cloned = original.clone();
        assert_eq!(original, cloned);
        assert_eq!(original.material(), cloned.material());
        assert_eq!(original.label(), cloned.label());
    }
}
