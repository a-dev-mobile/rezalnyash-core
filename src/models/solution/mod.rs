use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::atomic::AtomicU32;

use crate::models::mosaic::Mosaic;
use crate::models::no_fit_tile::NoFitTile;
use crate::models::tile_dimensions::TileDimensions;


/// Static counter for generating unique solution IDs
pub(super) static ID_COUNTER: AtomicU32 = AtomicU32::new(0);

/// Represents a complete cutting solution containing multiple mosaics
/// 
/// This is the Rust equivalent of the Java Solution class. It contains
/// all the mosaics (cutting patterns), unused stock panels, and panels
/// that couldn't be fit into any mosaic.
#[derive(Debug, Clone)]
pub struct Solution {
    /// Unique identifier for this solution
    pub id: u32,
    
    /// Timestamp when this solution was created (milliseconds since Unix epoch)
    pub timestamp: u64,
    
    /// List of mosaics (cutting patterns) in this solution
    pub mosaics: Vec<Mosaic>,
    
    /// Panels that couldn't be fit into any mosaic
    pub no_fit_panels: Vec<NoFitTile>,
    
    /// Unused stock panels available for cutting
    pub unused_stock_panels: VecDeque<TileDimensions>,
    
    /// Optional auxiliary information
    pub aux_info: Option<String>,
    
    /// Optional creator thread group identifier
    pub creator_thread_group: Option<String>,
}


impl Solution {
    pub fn new() -> Self {
        Self {
            id: 0,
            timestamp: 0,
            mosaics: Vec::new(),
            no_fit_panels: Vec::new(),
            aux_info: None,
            creator_thread_group: None,
            unused_stock_panels: VecDeque::new(),
        }
    }
    
    pub fn add_mosaic(&mut self, mosaic: Mosaic) {
        self.mosaics.push(mosaic);
    }
    
    pub fn replace_mosaic(&mut self, index: usize, new_mosaic: Mosaic) {
        if index < self.mosaics.len() {
            self.mosaics[index] = new_mosaic;
        }
    }
    
    pub fn add_no_fit_tile(&mut self, tile: TileDimensions) {
        use crate::models::no_fit_tile::NoFitTile;
        let no_fit = NoFitTile {
            id: tile.id,
            width: tile.width,
            height: tile.height,
            count: 1,
            label: None,
            material: None,
        };
        self.no_fit_panels.push(no_fit);
    }
}