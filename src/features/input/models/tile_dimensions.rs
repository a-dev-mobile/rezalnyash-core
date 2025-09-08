use serde::{Deserialize, Serialize};
use crate::enums::orientation::Orientation;


// 3. РАЗВЕРНУТАЯ МОДЕЛЬ - готова для алгоритма размещения (count всегда = 1)
#[derive(Serialize, Debug, Clone,  Deserialize)]
pub struct TileDimensions {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub material: String,
    pub label: String,
    pub orientation: Orientation,
    pub is_rotated: bool,
}

impl TileDimensions {
    pub fn new(
        id: u32,
        width: u32,
        height: u32,
        is_rotated: bool,
        label: &str,
        material: &str,
    ) -> Self {
        Self {
            id,
            width,
            height,
            label: label.to_string(),
            material: material.to_string(),
            orientation: Orientation::Default,
            is_rotated,
        }
    }
    //
    /// Реализуем toString() ТОЧНО как в Java Это критично для правильной работы HashMap в алгоритме группировки
    pub fn to_string(&self) -> String {
        format!("id={}[{}x{}]", self.id, self.width, self.height)
    }

    /// Calculates hash code based only on dimensions (width and height)
    /// Port from Java: (this.width * 31) + this.height
    pub fn dimensions_based_hash_code(&self) -> i32 {
        (self.width as i32).wrapping_mul(31).wrapping_add(self.height as i32)
    }


        /// Calculates the area of the tile
    pub fn area(&self) -> u64 {
        self.width as u64 * self.height as u64
    }

    /// Check if the tile is square
    pub fn is_square(&self) -> bool {
        self.width == self.height
    }

    /// Rotate the tile 90 degrees
    pub fn rotate_90(&self) -> Self {
        Self {
            id: self.id,
            width: self.height,
            height: self.width,
            material: self.material.clone(),
            label: self.label.clone(),
            orientation: self.orientation,
            is_rotated: !self.is_rotated,
        }
    }

    /// Check if this tile fits in another tile (dimensions)
    pub fn fits(&self, other: &TileDimensions) -> bool {
        (self.width >= other.width && self.height >= other.height) ||
        (self.width >= other.height && self.height >= other.width)
    }

}
