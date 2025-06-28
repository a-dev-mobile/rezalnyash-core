use crate::enums::orientation::Orientation;
use crate::models::edge::Edge;
use crate::{constants::MaterialConstants, scaled_math::PrecisionAnalyzer};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a panel with dimensions, material, and configuration
#[derive(Debug, Clone, PartialEq)]
pub struct Panel {
    pub id: u8,
    pub width: String,
    pub height: String,
    pub count: u16,
    pub material: Option<String>,
    pub enabled: bool,
    pub orientation: Orientation,
    pub edge: Option<Edge>,
}

impl Panel {
    pub fn new(id: u8, width: String, height: String, count: u16) -> Self {
        Self {
            id,
            width,
            height,
            count,
            material: None,
            enabled: true,
            orientation: Orientation::Default,
            edge: None,
        }
    }
   

pub fn get_max_decimal_places(panels: &[Panel]) -> u8 {
    panels
        .iter()
        .flat_map(|panel| {
            vec![
                PrecisionAnalyzer::count_decimal_places(&panel.width),
                PrecisionAnalyzer::count_decimal_places(&panel.height),
            ]
        })
        .max()
        .unwrap_or(0)
}

    pub fn get_max_integer_places(panels: &[Panel]) -> u8 {
        panels
            .iter()
            .flat_map(|panel| {
                vec![
                    PrecisionAnalyzer::count_integer_places(&panel.width),
                    PrecisionAnalyzer::count_integer_places(&panel.height),
                ]
            })
            .max()
            .unwrap_or(0)
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
