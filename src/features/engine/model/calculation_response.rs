use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::enums::orientation::Orientation;
use crate::features::engine::model::calculation_request::{CalculationRequest, Edge};
use crate::features::engine::model::tile_node::TileNode;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationResponse {
    pub id: Option<String>,
    pub task_id: Option<String>,
    pub elapsed_time: u64,
    pub solution_elapsed_time: Option<u64>,
    pub total_used_area: f64,
    pub total_wasted_area: f64,
    pub total_used_area_ratio: f64,
    pub total_nbr_cuts: i64,
    pub total_cut_length: f64,
    pub request: CalculationRequest,
    pub panels: Vec<FinalTile>,
    pub used_stock_panels: Vec<FinalTile>,
    pub edge_bands: HashMap<String, f64>,
    pub no_fit_panels: Vec<NoFitTile>,
    pub mosaics: Vec<Mosaic>,
}

impl CalculationResponse {
    pub fn new() -> Self {
        Self {
            id: None,
            task_id: None,
            elapsed_time: 0,
            solution_elapsed_time: None,
            total_used_area: 0.0,
            total_wasted_area: 0.0,
            total_used_area_ratio: 0.0,
            total_nbr_cuts: 0,
            total_cut_length: 0.0,
            request: CalculationRequest::default(),
            panels: Vec::new(),
            used_stock_panels: Vec::new(),
            edge_bands: HashMap::new(),
            no_fit_panels: Vec::new(),
            mosaics: Vec::new(),
        }
    }

    pub fn version() -> &'static str {
        "1.2"
    }
}

impl Default for CalculationResponse {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// -= доработать
pub struct Mosaic {
    pub cuts: Vec<Cut>,
    pub material: Option<String>,
    pub orientation: Orientation,
    pub root_tile_node: Vec<TileNode>,

    pub cut_length: f64,

    
    pub edge_bands: HashMap<String, f64>,
    pub nbr_wasted_panels: i32,
    pub nbr_final_panels: i32,
    pub panels: Vec<FinalTile>,
    pub request_stock_id: Option<i32>,
    pub stock_label: Option<String>,
    pub used_area: f64,
    pub used_area_ratio: f32,
    pub wasted_area: f64,
}

impl Default for Mosaic {
    fn default() -> Self {
        Self {
            request_stock_id: None,
            stock_label: None,
            used_area: 0.0,
            wasted_area: 0.0,
            used_area_ratio: 0.0,
            nbr_final_panels: 0,
            nbr_wasted_panels: 0,
            cut_length: 0.0,
            material: None,
            edge_bands: HashMap::new(),
            panels: Vec::new(),
            tiles: Vec::new(),
            cuts: Vec::new(),

        }
    }
}

impl Mosaic {
    /// Java: public Mosaic(TileDimensions tileDimensions)
    pub fn from_tile_dimensions(tile_dimensions: &crate::features::input::models::tile_dimensions::TileDimensions) -> Self {
        let mut mosaic = Self::new();
        
        // Java: this.material = tileDimensions.getMaterial();
        mosaic.material = Some(tile_dimensions.material.clone());
        
        // Java: this.rootTileNode = new TileNode(0, tileDimensions.getWidth(), 0, tileDimensions.getHeight());
        let root_node = TileNode::new(
            0, 
            tile_dimensions.width as i32, 
            0, 
            tile_dimensions.height as i32
        );
     
        
        // Java: this.wastedArea = tileDimensions.getArea();
        mosaic.wasted_area = (tile_dimensions.width * tile_dimensions.height) as f64;
        
        mosaic
    }



 
    
    /// Calculate unused area - matches Java Mosaic.getUnusedArea()
    pub fn get_unused_area(&self) -> i64 {
        
            0 // Fallback if no root node
        
    }
    
    /// Java: public HashSet<Integer> getDistictTileSet()
    pub fn get_distict_tile_set(&self) -> HashSet<i32> {
      
            HashSet::new()
        
    }
    
    /// Java: public long getBiggestArea()
    pub fn get_biggest_area(&self) -> i64 {
        // Java: return this.rootTileNode.getBiggestArea();
      
            0
        
    }
    
    /// Java: public float getHVDiff()
    pub fn get_hvdiff(&self) -> f32 {
      
            0.0
        
    }
}

impl Default for Mosaic {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub id: i32,
    pub request_obj_id: Option<i32>,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub orientation: i32,
    pub label: Option<String>,
    pub is_final: bool,
    pub has_children: bool,
    pub edge: Edge,
    pub is_rotated: bool,
}

impl Tile {
    pub fn new() -> Self {
        Self {
            id: 0,
            request_obj_id: None,
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            orientation: 0,
            label: None,
            is_final: false,
            has_children: false,
            edge: Edge::new(),
            is_rotated: false,
        }
    }

    pub fn with_coords(id: i32, x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            id,
            x,
            y,
            width,
            height,
            ..Self::new()
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self::new()
    }
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoFitTile {
    pub id: u32,
    pub width: f64,
    pub height: f64,
    pub count: i32,
    pub label: Option<String>,
    pub material: Option<String>,
}

impl NoFitTile {
    pub fn new() -> Self {
        Self {
            id: 0,
            width: 0.0,
            height: 0.0,
            count: 0,
            label: None,
            material: None,
        }
    }

    pub fn with_params(id: u32, width: f64, height: f64, count: i32) -> Self {
        Self {
            id,
            width,
            height,
            count,
            label: None,
            material: None,
        }
    }
}

impl Default for NoFitTile {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalTile {
    pub request_obj_id: i32,
    pub width: f64,
    pub height: f64,
    pub label: Option<String>,
    pub count: i32,
}

impl FinalTile {
    pub fn new() -> Self {
        Self {
            request_obj_id: 0,
            width: 0.0,
            height: 0.0,
            label: None,
            count: 0,
        }
    }

    pub fn increment_count(&mut self) -> i32 {
        let old_count = self.count;
        self.count += 1;
        old_count
    }
}

impl Default for FinalTile {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cut {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub cut_coord: f64,
    pub is_horizontal: bool,
    pub original_tile_id: i32,
    pub original_width: f64,
    pub original_height: f64,
    pub child1_tile_id: i32,
    pub child2_tile_id: i32,
}

impl Cut {
    pub fn new() -> Self {
        Self {
            x1: 0.0,
            y1: 0.0,
            x2: 0.0,
            y2: 0.0,
            cut_coord: 0.0,
            is_horizontal: false,
            original_tile_id: 0,
            original_width: 0.0,
            original_height: 0.0,
            child1_tile_id: 0,
            child2_tile_id: 0,
        }
    }
}

impl Default for Cut {
    fn default() -> Self {
        Self::new()
    }
}
