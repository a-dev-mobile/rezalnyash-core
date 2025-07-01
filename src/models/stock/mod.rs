//! Stock solution models and algorithms
//!
//! This module provides functionality for stock panel optimization,
//! including stock solution generation and panel picking algorithms.

pub mod stock_solution;
pub mod stock_solution_generator;
pub mod stock_panel_picker;

pub use stock_solution::StockSolution;
pub use stock_solution_generator::StockSolutionGenerator;
pub use stock_panel_picker::StockPanelPicker;
