use crate::features::input::traits::dimensions::Dimensions;

/// Заготовка (исходный лист материала)

#[derive(Debug, Clone)]
pub struct StockInput {
    pub width: String,
    pub height: String,
    pub label: String,
}

impl StockInput {
    pub fn new(width: &str, height: &str, label: &str) -> Self {
        Self {
            width: width.to_string(),
            height: height.to_string(),
            label: label.to_string(),
        }
    }
}

impl Dimensions for StockInput {
    fn get_dimensions(&self) -> Vec<&str> {
        vec![&self.width, &self.height]
    }
}
