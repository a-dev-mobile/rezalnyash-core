use serde::{Deserialize, Serialize};

use crate::{
    constants::MaterialConstants,
    enums::orientation::Orientation,
    features::engine::model::{client_info::ClientInfo, configuration::Configuration, performance_thresholds::PerformanceThresholds},
    scaled_math::ScaledNumber,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationRequest {
    pub configuration: Configuration,
    pub panels: Vec<Panel>,
    pub stock_panels: Vec<Panel>,
    pub client_info: ClientInfo,
    pub performance_thresholds: PerformanceThresholds,
}

impl Default for CalculationRequest {
    fn default() -> Self {
        Self {
            configuration: Configuration::default(),
            panels: Vec::new(),
            stock_panels: Vec::new(),
            client_info: ClientInfo::default(),
            performance_thresholds: PerformanceThresholds::default(),
        }
    }
}

impl CalculationRequest {
    pub fn tiles_to_string(&self) -> String {
        let mut result = String::new();
        for panel in &self.panels {
            if panel.count > 0 {
                result.push(' ');
                result.push_str(&panel.to_string());
            }
        }
        result
    }

    pub fn base_tiles_to_string(&self) -> String {
        let mut result = String::new();
        for panel in &self.stock_panels {
            if panel.count > 0 {
                result.push(' ');
                result.push_str(&panel.to_string());
            }
        }
        result
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Panel {
    pub id: u32,
    pub width: String,
    pub height: String,
    pub count: u32,
    pub material: String,
    pub enabled: bool,
    pub orientation: Orientation,
    pub label: String,
    pub edge: Option<Edge>,
}

impl Panel {
    pub fn new(id: u32, width: &str, height: &str, count: u32, label: &str) -> Self {
        Self {
            id,
            width: width.to_string(),
            height: height.to_string(),
            count,

            material: MaterialConstants::DEFAULT_MATERIAL.to_string(),

            enabled: false,
            orientation: Orientation::default(),
            label: label.to_string(),
            edge: None,
        }
    }

    pub fn set_material(&mut self, material: Option<String>) {
        if let Some(mat) = material {
            self.material = mat;
        }
    }

    pub fn is_valid(&self) -> bool {
        if !self.enabled || self.count <= 0 {
            return false;
        }

        let width_valid = self.width.parse::<f64>().unwrap_or(0.0) > 0.0;
        let height_valid = self.height.parse::<f64>().unwrap_or(0.0) > 0.0;

        width_valid && height_valid
    }
}

impl std::fmt::Display for Panel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let disabled = if self.enabled { "" } else { "-disabled" };
        write!(
            f,
            "[{}x{}]*{}{}",
            self.width, self.height, self.count, disabled
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub top: Option<String>,
    pub left: Option<String>,
    pub bottom: Option<String>,
    pub right: Option<String>,
}

impl Edge {
    pub fn new() -> Self {
        Self {
            top: None,
            left: None,
            bottom: None,
            right: None,
        }
    }
}
