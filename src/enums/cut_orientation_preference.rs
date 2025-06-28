
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CutOrientationPreference {
    /// Both horizontal and vertical cuts are considered (value: 0)
    Both = 0,
    /// Horizontal cuts are preferred (value: 1)  
    Horizontal = 1,
    /// Vertical cuts are preferred (value: 2)
    Vertical = 2,
}

impl CutOrientationPreference {
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

    /// Check if this preference allows horizontal cuts
    pub fn allows_horizontal(self) -> bool {
        matches!(self, Self::Both | Self::Horizontal)
    }

    /// Check if this preference allows vertical cuts
    pub fn allows_vertical(self) -> bool {
        matches!(self, Self::Both | Self::Vertical)
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
