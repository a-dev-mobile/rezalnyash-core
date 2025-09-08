use crate::features::input::models::tile_dimensions::TileDimensions;
use crate::features::engine::model::{stock_solution::StockSolution, task::Task};

#[derive(Debug)]
pub struct StockPanelPicker {
    pub stock_solutions: Vec<StockSolution>,
    pub current_index: usize,
}

impl StockPanelPicker {
    pub fn new(tiles: &[TileDimensions], stock_tiles: &[TileDimensions], task: &Task, single_stock: Option<i32>) -> Self {
        let mut stock_solutions = Vec::new();
        
        // Create stock solutions from available stock
        for stock_tile in stock_tiles {
            stock_solutions.push(StockSolution::new(vec![stock_tile.clone()]));
        }
        
        Self {
            stock_solutions,
            current_index: 0,
        }
    }
    
    pub fn init(&mut self) {
        // Initialize the picker - in Java this starts a separate thread
        // For simplicity, we'll keep it synchronous
    }
    
    pub fn get_stock_solution(&mut self, index: usize) -> Option<&StockSolution> {
        self.stock_solutions.get(index)
    }
}