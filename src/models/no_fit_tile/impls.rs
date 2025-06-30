//! NoFitTile implementation

use super::NoFitTile;

impl NoFitTile {
    /// Creates a new NoFitTile with the specified parameters
    /// 
    /// # Arguments
    /// * `id` - Unique identifier for the tile
    /// * `width` - Width of the tile
    /// * `height` - Height of the tile  
    /// * `count` - Number of tiles needed
    pub fn new(id: i32, width: u64, height: u64, count: i32) -> Self {
        Self {
            id,
            width,
            height,
            count,
            label: None,
            material: None,
        }
    }

    pub fn area(&self) -> u64 {
        self.width * self.height
    }

    /// Calculates the total area for all tiles of this type
    pub fn total_area(&self) -> u64 {
        self.area() * self.count as u64
    }
}
