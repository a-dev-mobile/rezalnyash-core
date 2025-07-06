use serde::Serialize;

use crate::features::{input::traits::dimensions::Dimensions};

/// Деталь для размещения
#[derive(Serialize, Debug, Clone)]
pub struct PanelInput {
    pub width: String,
    pub height: String,
    pub count: u8,
    pub label: String,
}

impl PanelInput {
    pub fn new(width: &str, height: &str, count: u8, label: &str) -> Self {
        Self {
            width: width.to_string(),
            height: height.to_string(),
            count,
            label: label.to_string(),
        }
    }
}
impl Dimensions for PanelInput {
    fn get_dimensions(&self) -> Vec<&str> {
        vec![&self.width, &self.height]
    }
}
