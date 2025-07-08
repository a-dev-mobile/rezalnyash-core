use serde::Serialize;
use uuid::Uuid;

use crate::{
    constants::MaterialConstants,
    features::input::{models::edge::Edge, traits::dimensions::Dimensions},
};

// 1. ВХОДНАЯ МОДЕЛЬ - то что приходит от пользователя
#[derive(Serialize, Debug, Clone)]
pub struct PanelInput {
    pub id: u16,
    pub width: String,
    pub height: String,
    pub count: u16,
    pub enabled: bool,
    pub label: String,
    pub material: String,
    pub edge: Option<Edge>,
}

impl PanelInput {
    pub fn new(id: u16, width: &str, height: &str, count: u16, label: &str) -> Self {
        Self {
            id,
            width: width.to_string(),
            height: height.to_string(),
            count,
            enabled: true,
            label: label.to_string(),
            material: MaterialConstants::DEFAULT_MATERIAL.to_string(),
            edge: None,
        }
    }
}
impl Dimensions for PanelInput {
    fn get_dimensions(&self) -> Vec<&str> {
        vec![&self.width, &self.height]
    }
}
