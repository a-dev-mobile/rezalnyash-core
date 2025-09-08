use serde::{Deserialize, Serialize};

use crate::features::engine::task_report::TaskReport;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub nbr_idle_tasks: i64,
    pub nbr_running_tasks: i64,
    pub nbr_finished_tasks: i64,
    pub nbr_stopped_tasks: i64,
    pub nbr_terminated_tasks: i64,
    pub nbr_error_tasks: i64,
    pub nbr_running_threads: i32,
    pub nbr_queued_threads: i32,
    pub nbr_finished_threads: i64,
    pub task_reports: Vec<TaskReport>,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            nbr_idle_tasks: 0,
            nbr_running_tasks: 0,
            nbr_finished_tasks: 0,
            nbr_stopped_tasks: 0,
            nbr_terminated_tasks: 0,
            nbr_error_tasks: 0,
            nbr_running_threads: 0,
            nbr_queued_threads: 0,
            nbr_finished_threads: 0,
            task_reports: Vec::new(),
        }
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}