use serde::{Deserialize, Serialize};

use crate::models::CalculationResponse;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskStatusResponse {
    init_percentage: i32,
    percentage_done: i32,
    solution: Option<CalculationResponse>,
    status: Option<String>,
}

impl TaskStatusResponse {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_status(&self) -> Option<&str> {
        self.status.as_deref()
    }

    pub fn set_status(&mut self, status: String) {
        self.status = Some(status);
    }

    pub fn setstatusopt(&mut self, status: Option<String>) {
        self.status = status;
    }

    pub fn getpercentagedone(&self) -> i32 {
        self.percentage_done
    }

    pub fn set_percentage_done(&mut self, percentage: i32) {
        self.percentage_done = percentage;
    }

    pub fn getinitpercentage(&self) -> i32 {
        self.init_percentage
    }

    pub fn set_init_percentage(&mut self, percentage: i32) {
        self.init_percentage = percentage;
    }

    pub fn get_solution(&self) -> Option<&CalculationResponse> {
        self.solution.as_ref()
    }

    pub fn set_solution(&mut self, solution: CalculationResponse) {
        self.solution = Some(solution);
    }

    pub fn setsolutionopt(&mut self, solution: Option<CalculationResponse>) {
        self.solution = solution;
    }

    pub fn take_solution(&mut self) -> Option<CalculationResponse> {
        self.solution.take()
    }

    // Builder pattern methods for fluent API
    pub fn with_status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }

    pub fn withpercentagedone(mut self, percentage: i32) -> Self {
        self.percentage_done = percentage;
        self
    }

    pub fn withinitpercentage(mut self, percentage: i32) -> Self {
        self.init_percentage = percentage;
        self
    }

    pub fn with_solution(mut self, solution: CalculationResponse) -> Self {
        self.solution = Some(solution);
        self
    }
}
