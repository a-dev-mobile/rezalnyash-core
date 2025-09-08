
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    pub max_simultaneous_tasks: i32,
    pub max_simultaneous_threads: i32,
    pub thread_check_interval: i64,
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            max_simultaneous_tasks: 1,
            max_simultaneous_threads: 0,
            thread_check_interval: 0,
        }
    }
}
