use serde::Serialize;
use uuid::Uuid;

use crate::{
    constants::MaterialConstants, features::input::models::tile_dimensions::TileDimensions,
};

// 2. РАБОЧАЯ МОДЕЛЬ - после конвертации размеров, но до развертывания
#[derive(Serialize, Debug, Clone)]
pub struct Panel {
    pub id: u16,
    pub width: u32,
    pub height: u32,
    pub count: u16,
    pub label: String,
    pub material: String,
}

impl Panel {
    pub fn new(
        id: u16,
        width: u32,
        height: u32,
        count: u16,
        label: String,
        material: String,
    ) -> Self {
        Self {
            id,
            width,
            height,
            count,
            label,
            material,
        }
    }
    /// Проверить, является ли панель квадратной
    pub fn is_square(&self) -> bool {
        self.width == self.height
    }

    /// Повернуть панель на 90 градусов (поменять местами ширину и высоту)
    pub fn rotate(&self) -> Self {
        Self {
            id: self.id,
            width: self.height,
            height: self.width,
            count: self.count,
            label: self.label.clone(),
            material: self.material.clone(),
        }
    }
    pub fn expand(&self) -> Vec<TileDimensions> {
        (1..=self.count)
            .map(|_| {
                TileDimensions::new(
                    self.id,
                    self.width,
                    self.height,
                    false,
                    &self.label,
                    &self.material,
                )
            })
            .collect()
    }
}
