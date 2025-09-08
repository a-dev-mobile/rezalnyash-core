use serde::{Deserialize, Serialize};

use crate::features::engine::model::calculation_response::CalculationResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStatusResponse {
    pub status: Option<String>,
    pub percentage_done: i32,
    pub init_percentage: i32,
    pub solution: Option<CalculationResponse>,
}

impl TaskStatusResponse {
    pub fn new() -> Self {
        Self {
            status: None,
            percentage_done: 0,
            init_percentage: 0,
            solution: None,
        }
    }
}

impl Default for TaskStatusResponse {
    fn default() -> Self {
        Self::new()
    }
}