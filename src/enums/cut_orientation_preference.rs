
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CutOrientationPreference {
    /// Both horizontal and vertical cuts are considered (value: 0)
    Both = 0,
    /// Horizontal cuts are preferred (value: 1)  
    Horizontal = 1,
    /// Vertical cuts are preferred (value: 2)
    Vertical = 2,
}

impl CutOrientationPreference {
    /// Converts to numeric value (for compatibility with Java)
    pub fn to_numeric(&self) -> i32 {
        match self {
            CutOrientationPreference::Both => 0,
            CutOrientationPreference::Horizontal => 1,
            CutOrientationPreference::Vertical => 2,
        }
    }

    /// Checks if horizontal cuts are allowed
    pub fn allows_horizontal(&self) -> bool {
        matches!(self, CutOrientationPreference::Both | CutOrientationPreference::Horizontal)
    }

    /// Checks if vertical cuts are allowed
    pub fn allows_vertical(&self) -> bool {
        matches!(self, CutOrientationPreference::Both | CutOrientationPreference::Vertical)
    }


    /// Convert from integer value
    pub fn from_int(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Both),
            1 => Some(Self::Horizontal),
            2 => Some(Self::Vertical),
            _ => None,
        }
    }

    /// Convert to integer value
    pub fn to_int(self) -> i32 {
        self as i32
    }


}

impl Default for CutOrientationPreference {
    fn default() -> Self {
        Self::Both
    }
}

impl From<i32> for CutOrientationPreference {
    fn from(value: i32) -> Self {
        Self::from_int(value).unwrap_or_default()
    }
}

impl From<CutOrientationPreference> for i32 {
    fn from(preference: CutOrientationPreference) -> Self {
        preference.to_int()
    }
}
