//! Calculation response model module
//!
//! This module contains the CalculationResponse struct and its associated functionality.

pub mod calculation_response;
pub mod calculation_response_tests;

pub use calculation_response::{
    CalculationResponse, Mosaic, Tile, Edge, NoFitTile, FinalTile, CutResponse
};
