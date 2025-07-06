
/// Рез на листе
#[derive(Debug, Clone)]
pub struct Cut {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    pub is_horizontal: bool,
}

impl Cut {
    pub fn new_horizontal(x: i32, y: i32, length: i32) -> Self {
        Self {
            x1: x,
            y1: y,
            x2: x + length,
            y2: y,
            is_horizontal: true,
        }
    }

    pub fn new_vertical(x: i32, y: i32, length: i32) -> Self {
        Self {
            x1: x,
            y1: y,
            x2: x,
            y2: y + length,
            is_horizontal: false,
        }
    }

    pub fn length(&self) -> i32 {
        if self.is_horizontal {
            self.x2 - self.x1
        } else {
            self.y2 - self.y1
        }
    }
}
