#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Orientation {
    /// Default/neutral orientation
    Default = 0,
    /// Portrait orientation (height > width)
    Portrait = 1,
    /// Landscape orientation (width > height)
    Landscape = 2,
}

impl Orientation {
    /// Create orientation from integer value (for compatibility with Java code)
    pub fn from_int(value: i32) -> Self {
        match value {
            0 => Orientation::Default,
            1 => Orientation::Portrait,
            2 => Orientation::Landscape,
            _ => Orientation::Default, // fallback for invalid values
        }
    }

    /// Convert orientation to integer value (for compatibility with Java code)
    pub fn to_int(self) -> i32 {
        self as i32
    }

    /// Rotate orientation by 90 degrees (equivalent to Java's rotate90() logic)
    pub fn rotate_90(self) -> Self {
        match self {
            Orientation::Default => Orientation::Default,
            Orientation::Portrait => Orientation::Landscape,
            Orientation::Landscape => Orientation::Portrait,
        }
    }
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation::Default
    }
}

impl From<i32> for Orientation {
    fn from(value: i32) -> Self {
        Self::from_int(value)
    }
}

impl From<Orientation> for i32 {
    fn from(orientation: Orientation) -> Self {
        orientation.to_int()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orientation_from_int() {
        assert_eq!(Orientation::from_int(0), Orientation::Default);
        assert_eq!(Orientation::from_int(1), Orientation::Portrait);
        assert_eq!(Orientation::from_int(2), Orientation::Landscape);
        assert_eq!(Orientation::from_int(99), Orientation::Default); // fallback
    }

    #[test]
    fn test_orientation_to_int() {
        assert_eq!(Orientation::Default.to_int(), 0);
        assert_eq!(Orientation::Portrait.to_int(), 1);
        assert_eq!(Orientation::Landscape.to_int(), 2);
    }

    #[test]
    fn test_rotate_90() {
        assert_eq!(Orientation::Default.rotate_90(), Orientation::Default);
        assert_eq!(Orientation::Portrait.rotate_90(), Orientation::Landscape);
        assert_eq!(Orientation::Landscape.rotate_90(), Orientation::Portrait);
    }

    #[test]
    fn test_from_trait() {
        let orientation: Orientation = 1.into();
        assert_eq!(orientation, Orientation::Portrait);
        
        let int_val: i32 = Orientation::Landscape.into();
        assert_eq!(int_val, 2);
    }
}