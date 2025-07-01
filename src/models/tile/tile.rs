//! Tile model for cutting optimization
//! 
//! This module provides functionality for managing tile coordinates and properties
//! for cutting optimization algorithms. A tile represents a rectangular area defined
//! by its corner coordinates.

use std::fmt;
use std::hash::{Hash, Hasher};
use crate::models::TileDimensions;
use crate::errors::CoreError;

/// Represents a rectangular tile with coordinate boundaries
/// 
/// A tile is defined by its corner coordinates (x1, y1) to (x2, y2)
/// where (x1, y1) is typically the top-left corner and (x2, y2) is the bottom-right corner.
#[derive(Debug, Clone)]
pub struct Tile {
    /// Left x-coordinate
    x1: i32,
    /// Right x-coordinate  
    x2: i32,
    /// Top y-coordinate
    y1: i32,
    /// Bottom y-coordinate
    y2: i32,
}

impl Tile {
    /// Creates a new tile from TileDimensions, positioned at origin (0,0)
    /// 
    /// # Arguments
    /// * `tile_dimensions` - The dimensions to create the tile from
    /// 
    /// # Returns
    /// A new Tile positioned at origin with the specified dimensions
    /// 
    /// # Errors
    /// Returns `CoreError::InvalidInput` if dimensions would cause integer overflow
    pub fn from_tile_dimensions(tile_dimensions: &TileDimensions) -> Result<Self, CoreError> {
        let width = tile_dimensions.width() as i32;
        let height = tile_dimensions.height() as i32;
        
        // Check for potential overflow
        if width < 0 || height < 0 {
            return Err(CoreError::InvalidInput {
                details: "Tile dimensions too large for coordinate system".to_string(),
            });
        }
        
        Ok(Self {
            x1: 0,
            x2: width,
            y1: 0,
            y2: height,
        })
    }
    
    /// Creates a new tile from explicit coordinates
    /// 
    /// # Arguments
    /// * `x1` - Left x-coordinate
    /// * `x2` - Right x-coordinate  
    /// * `y1` - Top y-coordinate
    /// * `y2` - Bottom y-coordinate
    /// 
    /// # Returns
    /// A new Tile with the specified coordinates
    /// 
    /// # Errors
    /// Returns `CoreError::InvalidInput` if coordinates are invalid (x2 <= x1 or y2 <= y1)
    pub fn new(x1: i32, x2: i32, y1: i32, y2: i32) -> Result<Self, CoreError> {
        if x2 <= x1 {
            return Err(CoreError::InvalidInput {
                details: format!("Invalid x coordinates: x2 ({}) must be greater than x1 ({})", x2, x1),
            });
        }
        
        if y2 <= y1 {
            return Err(CoreError::InvalidInput {
                details: format!("Invalid y coordinates: y2 ({}) must be greater than y1 ({})", y2, y1),
            });
        }
        
        Ok(Self { x1, x2, y1, y2 })
    }
    
    /// Creates a new tile from explicit coordinates without validation
    /// 
    /// # Safety
    /// This function assumes coordinates are valid. Use only when you're certain
    /// that x2 > x1 and y2 > y1.
    /// 
    /// # Arguments
    /// * `x1` - Left x-coordinate
    /// * `x2` - Right x-coordinate  
    /// * `y1` - Top y-coordinate
    /// * `y2` - Bottom y-coordinate
    pub fn new_unchecked(x1: i32, x2: i32, y1: i32, y2: i32) -> Self {
        Self { x1, x2, y1, y2 }
    }
    
    /// Gets the left x-coordinate
    pub fn x1(&self) -> i32 {
        self.x1
    }
    
    /// Gets the right x-coordinate
    pub fn x2(&self) -> i32 {
        self.x2
    }
    
    /// Gets the top y-coordinate
    pub fn y1(&self) -> i32 {
        self.y1
    }
    
    /// Gets the bottom y-coordinate
    pub fn y2(&self) -> i32 {
        self.y2
    }
    
    /// Calculates the width of the tile
    pub fn width(&self) -> i32 {
        self.x2 - self.x1
    }
    
    /// Calculates the height of the tile
    pub fn height(&self) -> i32 {
        self.y2 - self.y1
    }
    
    /// Calculates the area of the tile
    /// 
    /// Returns the area as u64 to prevent overflow for large tiles
    pub fn area(&self) -> u64 {
        (self.width() as u64) * (self.height() as u64)
    }
    
    /// Gets the maximum side length (width or height)
    pub fn max_side(&self) -> i32 {
        self.width().max(self.height())
    }
    
    /// Checks if the tile is horizontally oriented (width > height)
    pub fn is_horizontal(&self) -> bool {
        self.width() > self.height()
    }
    
    /// Checks if the tile is vertically oriented (height > width)
    pub fn is_vertical(&self) -> bool {
        self.height() > self.width()
    }
    
    /// Checks if the tile is square (width == height)
    pub fn is_square(&self) -> bool {
        self.width() == self.height()
    }
    
    /// Creates a new tile translated by the given offsets
    /// 
    /// # Arguments
    /// * `dx` - Horizontal offset
    /// * `dy` - Vertical offset
    /// 
    /// # Returns
    /// A new Tile translated by the specified offsets
    /// 
    /// # Errors
    /// Returns `CoreError::InvalidInput` if translation would cause overflow
    pub fn translate(&self, dx: i32, dy: i32) -> Result<Self, CoreError> {
        let new_x1 = self.x1.checked_add(dx)
            .ok_or_else(|| CoreError::InvalidInput {
                details: "Translation would cause x1 overflow".to_string(),
            })?;
        let new_x2 = self.x2.checked_add(dx)
            .ok_or_else(|| CoreError::InvalidInput {
                details: "Translation would cause x2 overflow".to_string(),
            })?;
        let new_y1 = self.y1.checked_add(dy)
            .ok_or_else(|| CoreError::InvalidInput {
                details: "Translation would cause y1 overflow".to_string(),
            })?;
        let new_y2 = self.y2.checked_add(dy)
            .ok_or_else(|| CoreError::InvalidInput {
                details: "Translation would cause y2 overflow".to_string(),
            })?;
            
        Ok(Self {
            x1: new_x1,
            x2: new_x2,
            y1: new_y1,
            y2: new_y2,
        })
    }
    
    /// Checks if this tile contains the given point
    /// 
    /// # Arguments
    /// * `x` - X coordinate of the point
    /// * `y` - Y coordinate of the point
    /// 
    /// # Returns
    /// `true` if the point is inside the tile (inclusive of boundaries)
    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        x >= self.x1 && x <= self.x2 && y >= self.y1 && y <= self.y2
    }
    
    /// Checks if this tile overlaps with another tile
    /// 
    /// # Arguments
    /// * `other` - The other tile to check overlap with
    /// 
    /// # Returns
    /// `true` if the tiles overlap
    pub fn overlaps_with(&self, other: &Tile) -> bool {
        !(self.x2 <= other.x1 || other.x2 <= self.x1 || 
          self.y2 <= other.y1 || other.y2 <= self.y1)
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tile[({},{}) to ({},{}), {}x{}]", 
               self.x1, self.y1, self.x2, self.y2, self.width(), self.height())
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.x1 == other.x1 && self.x2 == other.x2 && 
        self.y1 == other.y1 && self.y2 == other.y2
    }
}

impl Eq for Tile {}

impl Hash for Tile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x1.hash(state);
        self.x2.hash(state);
        self.y1.hash(state);
        self.y2.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_tile_dimensions() {
        let tile_dims = TileDimensions::simple(100, 200);
        let tile = Tile::from_tile_dimensions(&tile_dims).unwrap();
        
        assert_eq!(tile.x1(), 0);
        assert_eq!(tile.x2(), 100);
        assert_eq!(tile.y1(), 0);
        assert_eq!(tile.y2(), 200);
        assert_eq!(tile.width(), 100);
        assert_eq!(tile.height(), 200);
    }
    
    #[test]
    fn test_new_valid_coordinates() {
        let tile = Tile::new(10, 50, 20, 80).unwrap();
        
        assert_eq!(tile.x1(), 10);
        assert_eq!(tile.x2(), 50);
        assert_eq!(tile.y1(), 20);
        assert_eq!(tile.y2(), 80);
        assert_eq!(tile.width(), 40);
        assert_eq!(tile.height(), 60);
    }
    
    #[test]
    fn test_new_invalid_coordinates() {
        // Invalid x coordinates (x2 <= x1)
        assert!(Tile::new(50, 50, 20, 80).is_err());
        assert!(Tile::new(50, 40, 20, 80).is_err());
        
        // Invalid y coordinates (y2 <= y1)
        assert!(Tile::new(10, 50, 80, 80).is_err());
        assert!(Tile::new(10, 50, 80, 70).is_err());
    }
    
    #[test]
    fn test_new_unchecked() {
        let tile = Tile::new_unchecked(10, 50, 20, 80);
        
        assert_eq!(tile.x1(), 10);
        assert_eq!(tile.x2(), 50);
        assert_eq!(tile.y1(), 20);
        assert_eq!(tile.y2(), 80);
    }
    
    #[test]
    fn test_area_calculation() {
        let tile = Tile::new(0, 10, 0, 20).unwrap();
        assert_eq!(tile.area(), 200);
        
        // Test large area that would overflow i32
        let large_tile = Tile::new(0, 50000, 0, 50000).unwrap();
        assert_eq!(large_tile.area(), 2_500_000_000);
    }
    
    #[test]
    fn test_max_side() {
        let horizontal_tile = Tile::new(0, 100, 0, 50).unwrap();
        let vertical_tile = Tile::new(0, 50, 0, 100).unwrap();
        let square_tile = Tile::new(0, 75, 0, 75).unwrap();
        
        assert_eq!(horizontal_tile.max_side(), 100);
        assert_eq!(vertical_tile.max_side(), 100);
        assert_eq!(square_tile.max_side(), 75);
    }
    
    #[test]
    fn test_orientation_checks() {
        let horizontal_tile = Tile::new(0, 100, 0, 50).unwrap();
        let vertical_tile = Tile::new(0, 50, 0, 100).unwrap();
        let square_tile = Tile::new(0, 75, 0, 75).unwrap();
        
        // Horizontal tile
        assert!(horizontal_tile.is_horizontal());
        assert!(!horizontal_tile.is_vertical());
        assert!(!horizontal_tile.is_square());
        
        // Vertical tile
        assert!(!vertical_tile.is_horizontal());
        assert!(vertical_tile.is_vertical());
        assert!(!vertical_tile.is_square());
        
        // Square tile
        assert!(!square_tile.is_horizontal());
        assert!(!square_tile.is_vertical());
        assert!(square_tile.is_square());
    }
    
    #[test]
    fn test_translate() {
        let tile = Tile::new(10, 50, 20, 80).unwrap();
        let translated = tile.translate(5, -10).unwrap();
        
        assert_eq!(translated.x1(), 15);
        assert_eq!(translated.x2(), 55);
        assert_eq!(translated.y1(), 10);
        assert_eq!(translated.y2(), 70);
        
        // Dimensions should remain the same
        assert_eq!(translated.width(), tile.width());
        assert_eq!(translated.height(), tile.height());
    }
    
    #[test]
    fn test_translate_overflow() {
        let tile = Tile::new(i32::MAX - 5, i32::MAX, 0, 10).unwrap();
        
        // This should cause overflow
        assert!(tile.translate(10, 0).is_err());
    }
    
    #[test]
    fn test_contains_point() {
        let tile = Tile::new(10, 50, 20, 80).unwrap();
        
        // Points inside
        assert!(tile.contains_point(25, 50));
        assert!(tile.contains_point(10, 20)); // Boundary
        assert!(tile.contains_point(50, 80)); // Boundary
        
        // Points outside
        assert!(!tile.contains_point(5, 50));
        assert!(!tile.contains_point(55, 50));
        assert!(!tile.contains_point(25, 15));
        assert!(!tile.contains_point(25, 85));
    }
    
    #[test]
    fn test_overlaps_with() {
        let tile1 = Tile::new(10, 50, 20, 60).unwrap();
        let tile2 = Tile::new(30, 70, 40, 80).unwrap(); // Overlaps
        let tile3 = Tile::new(60, 100, 20, 60).unwrap(); // No overlap
        let tile4 = Tile::new(50, 90, 20, 60).unwrap(); // Edge case - no overlap
        
        assert!(tile1.overlaps_with(&tile2));
        assert!(tile2.overlaps_with(&tile1)); // Symmetric
        assert!(!tile1.overlaps_with(&tile3));
        assert!(!tile1.overlaps_with(&tile4)); // Edge touching is not overlap
    }
    
    #[test]
    fn test_equality() {
        let tile1 = Tile::new(10, 50, 20, 80).unwrap();
        let tile2 = Tile::new(10, 50, 20, 80).unwrap();
        let tile3 = Tile::new(10, 50, 20, 81).unwrap();
        
        assert_eq!(tile1, tile2);
        assert_ne!(tile1, tile3);
    }
    
    #[test]
    fn test_hash() {
        let tile1 = Tile::new(10, 50, 20, 80).unwrap();
        let tile2 = Tile::new(10, 50, 20, 80).unwrap();
        
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(tile1.clone(), "value1");
        
        // Same coordinates should find the same entry
        assert_eq!(map.get(&tile2), Some(&"value1"));
    }
    
    #[test]
    fn test_display() {
        let tile = Tile::new(10, 50, 20, 80).unwrap();
        let display_str = format!("{}", tile);
        assert_eq!(display_str, "Tile[(10,20) to (50,80), 40x60]");
    }
    
    #[test]
    fn test_clone() {
        let original = Tile::new(10, 50, 20, 80).unwrap();
        let cloned = original.clone();
        
        assert_eq!(original, cloned);
        assert_eq!(original.x1(), cloned.x1());
        assert_eq!(original.x2(), cloned.x2());
        assert_eq!(original.y1(), cloned.y1());
        assert_eq!(original.y2(), cloned.y2());
    }
}
