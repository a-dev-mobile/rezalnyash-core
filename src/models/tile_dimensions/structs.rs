use serde::{Deserialize, Serialize};

use crate::enums::orientation::Orientation;



/// Represents the dimensions and properties of a tile/panel to be cut
#[derive(Debug, Clone, PartialEq)]
pub struct TileDimensions {
    pub id: u8,
    pub width: u32,
    pub height: u32,
    pub orientation: Orientation,
    pub is_rotated: bool,
}
