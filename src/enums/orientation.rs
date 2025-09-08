use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Orientation {
    /// Default/neutral orientation
    Default = 0,
    /// Portrait orientation (height > width)
    Portrait = 1,
    /// Landscape orientation (width > height)
    Landscape = 2,
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation::Default
    }
}

impl Orientation {
    /// Get numeric value for logging (matches Java behavior)
    pub fn to_numeric(&self) -> u8 {
        *self as u8
    }
}