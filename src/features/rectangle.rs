
/// Базовый прямоугольник для всех геометрических операций
#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rectangle {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn area(&self) -> i64 {
        (self.width as i64) * (self.height as i64)
    }

    pub fn fits(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}
