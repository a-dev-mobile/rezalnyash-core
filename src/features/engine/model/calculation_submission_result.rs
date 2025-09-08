use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationSubmissionResult {
    pub status_code: Option<String>,
    pub task_id: Option<String>,
}

impl CalculationSubmissionResult {
    pub fn new(status_code: String, task_id: String) -> Self {
        Self {
            status_code: Some(status_code),
            task_id: Some(task_id),
        }
    }

    pub fn with_status_code(status_code: String) -> Self {
        Self {
            status_code: Some(status_code),
            task_id: None,
        }
    }

    pub fn empty() -> Self {
        Self {
            status_code: None,
            task_id: None,
        }
    }
}

impl Default for CalculationSubmissionResult {
    fn default() -> Self {
        Self::empty()
    }
}
