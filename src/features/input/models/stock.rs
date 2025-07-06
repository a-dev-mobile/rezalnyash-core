use serde::Serialize;

use crate::features::input::models::tile_dimensions::TileDimensions;

/// Заготовка (обработанная модель с числовыми размерами)
#[derive(Serialize, Debug, Clone)]
pub struct Stock {
    pub original_id: u16,
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Stock {
    pub fn new(original_id: u16, width: u32, height: u32, label: &str) -> Self {
        Self {
            original_id,
            width,
            height,
            label: label.to_string(),
        }
    }

    /// Преобразует Stock в TileDimensions
    /// Поскольку Stock представляет заготовку, мы используем instance_number = 1
    /// и is_rotated = false по умолчанию
    pub fn to_tile_dimensions(&self) -> TileDimensions {
        TileDimensions::new(
            self.width,
            self.height,
            self.original_id,
            1,     // instance_number = 1 для заготовки
            false, // is_rotated = false по умолчанию
        )
    }
}
