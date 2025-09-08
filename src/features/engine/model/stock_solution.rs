use serde::{Deserialize, Serialize};

use crate::features::input::models::tile_dimensions::TileDimensions;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockSolution {
    pub stock_tiles: Vec<TileDimensions>,
    pub total_area: u64,
}

impl StockSolution {
    pub fn new(stock_tiles: Vec<TileDimensions>) -> Self {
        let total_area = stock_tiles.iter()
            .map(|tile| tile.width as u64 * tile.height as u64)
            .sum();
        
        Self {
            stock_tiles,
            total_area,
        }
    }
}