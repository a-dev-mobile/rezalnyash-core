
use serde::{Deserialize, Serialize};

use crate::models::tile_dimensions::TileDimensions;

/// Represents a stock solution containing a collection of tile dimensions
/// This is the Rust equivalent of the Java StockSolution class
#[derive(Debug, Clone,)]
pub struct StockSolution {
    pub stock_tile_dimensions: Vec<TileDimensions>,
}
