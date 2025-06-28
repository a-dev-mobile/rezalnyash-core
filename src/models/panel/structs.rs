use crate::constants::MaterialConstants;
use crate::models::edge::Edge;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a panel with dimensions, material, and configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Panel {
    pub id: i32,
    pub width: f32,
    pub height: f32,
    pub count: u16,
    pub material: Option<String>,
    pub enabled: bool,
    pub orientation: i32,
    pub edge: Option<Edge>,
}

impl Panel {
    pub fn new(id: i32, width: f32, height: f32, count: u16) -> Self {
        Self {
            id,
            width,
            height,
            count,
            material: None,
            enabled: true,
            orientation: 0,
            edge: None,
        }
    }
}

impl fmt::Display for Panel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let disabled_suffix = if self.enabled { "" } else { "-disabled" };

        write!(
            f,
            "[ w {}x h{} ]*{}{}",
            self.width, self.height, self.count, disabled_suffix
        )
    }
}
