//! Calculation Submission Result Module
//!
//! This module contains the Rust conversion of the Java CalculationSubmissionResult class.

mod calculation_submission_result;

pub use calculation_submission_result::{
    CalculationSubmissionResult,
    CalculationSubmissionResultBuilder,
};

#[cfg(test)]
mod calculation_submission_result_test;
