//! Task model module
//!
//! This module contains the Task struct and its associated functionality.

pub mod task;
pub mod task_tests;
pub mod task_report;


pub use task::{Task, Solution};
pub use task_report::TaskReport;


#[cfg(test)]
pub mod task_report_tests;