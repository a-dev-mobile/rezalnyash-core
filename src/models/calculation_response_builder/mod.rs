//! Calculation Response Builder Module
//!
//! This module provides the CalculationResponseBuilder for constructing
//! CalculationResponse objects from task data and solutions.

pub mod calculation_response_builder;

pub use calculation_response_builder::CalculationResponseBuilder;

#[cfg(test)]
mod calculation_response_builder_test;