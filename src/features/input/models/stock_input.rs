use crate::{constants::MaterialConstants, features::input::traits::dimensions::Dimensions};

/// Заготовка (исходный лист материала)

#[derive(Debug, Clone)]
pub struct StockInput {
    pub id: u16, // Идентификатор заготовки
    pub width: String,
    pub height: String,
    pub label: String,
    pub material: String,
    pub count: u16,
}

impl StockInput {
    pub fn new(id: u16, width: &str, height: &str, count: u16, label: &str) -> Self {
        Self {
            id,
            width: width.to_string(),
            height: height.to_string(),
            label: label.to_string(),
            material: MaterialConstants::DEFAULT_MATERIAL.to_string(),
            count,
        }
    }
}

impl Dimensions for StockInput {
    fn get_dimensions(&self) -> Vec<&str> {
        vec![&self.width, &self.height]
    }
}
