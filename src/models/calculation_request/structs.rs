use serde::{Deserialize, Serialize};

use crate::models::{configuration::structs::Configuration, panel::structs::Panel};

/// Request structure for cutting calculations containing configuration and panel data
#[derive(Debug, Clone)]
pub struct CalculationRequest {
    /// Configuration parameters for the optimization process
    pub configuration: Configuration,

    /// List of panels to be cut
    pub panels: Vec<Panel>,

    /// List of available stock panels
    pub stock_panels: Vec<Panel>,
}
