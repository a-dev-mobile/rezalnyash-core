use std::collections::HashMap;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskReport {
    pub task_id: Option<String>,
    pub client_id: Option<String>,
    pub status: Option<String>,
    pub nbr_running_threads: i32,
    pub nbr_queued_threads: i32,
    pub nbr_completed_threads: i32,
    pub nbr_panels: i32,
    pub percentage_done: i32,
    pub elapsed_time: Option<String>,
}

impl TaskReport {
    pub fn new() -> Self {
        Self {
            task_id: None,
            client_id: None,
            status: None,
            nbr_running_threads: 0,
            nbr_queued_threads: 0,
            nbr_completed_threads: 0,
            nbr_panels: 0,
            percentage_done: 0,
            elapsed_time: None,
        }
    }
}

impl Default for TaskReport {
    fn default() -> Self {
        Self::new()
    }
}