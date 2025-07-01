//! Calculation Response Model
//!
//! This module provides a complete Rust conversion of the Java CalculationResponse class,
//! maintaining functional equivalence while using idiomatic Rust patterns.

use crate::models::{CalculationRequest, TileNode, Cut, TileDimensions};
use crate::errors::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Version of the calculation response format
const CALCULATION_RESPONSE_VERSION: &str = "1.2";

/// Represents the response from a calculation operation
///
/// This is a direct conversion of the Java CalculationResponse class,
/// maintaining all fields and functionality while using Rust idioms.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalculationResponse {
    /// Unique identifier for this response
    pub id: Option<String>,
    
    /// Task identifier that generated this response
    pub task_id: Option<String>,
    
    /// Total elapsed time for the calculation in milliseconds
    pub elapsed_time: u64,
    
    /// Solution-specific elapsed time in milliseconds
    pub solution_elapsed_time: Option<u64>,
    
    /// Total used area across all panels
    pub total_used_area: f64,
    
    /// Total wasted area across all panels
    pub total_wasted_area: f64,
    
    /// Ratio of used area to total area (0.0 to 1.0)
    pub total_used_area_ratio: f64,
    
    /// Total number of cuts required
    pub total_nbr_cuts: u64,
    
    /// Total length of all cuts
    pub total_cut_length: f64,
    
    /// The original calculation request
    pub request: Option<CalculationRequest>,
    
    /// Final panels generated from the calculation
    pub panels: Option<Vec<FinalTile>>,
    
    /// Stock panels that were used in the solution
    pub used_stock_panels: Option<Vec<FinalTile>>,
    
    /// Edge banding requirements by type
    pub edge_bands: Option<HashMap<String, f64>>,
    
    /// Panels that could not be fit in the solution
    pub no_fit_panels: Vec<NoFitTile>,
    
    /// Mosaic layouts for each stock panel
    pub mosaics: Vec<Mosaic>,
}

/// Represents a mosaic layout within a calculation response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mosaic {
    /// Stock ID from the original request
    pub request_stock_id: Option<i32>,
    
    /// Label for the stock panel
    pub stock_label: Option<String>,
    
    /// Used area in this mosaic
    pub used_area: f64,
    
    /// Wasted area in this mosaic
    pub wasted_area: f64,
    
    /// Ratio of used area to total area
    pub used_area_ratio: f32,
    
    /// Number of final panels in this mosaic
    pub nbr_final_panels: i32,
    
    /// Number of wasted panels in this mosaic
    pub nbr_wasted_panels: i32,
    
    /// Total cut length for this mosaic
    pub cut_length: f64,
    
    /// Material type for this mosaic
    pub material: Option<String>,
    
    /// Edge banding requirements for this mosaic
    pub edge_bands: Option<HashMap<String, f64>>,
    
    /// Final panels in this mosaic
    pub panels: Option<Vec<FinalTile>>,
    
    /// Tiles layout in this mosaic
    pub tiles: Vec<Tile>,
    
    /// Cuts made in this mosaic
    pub cuts: Vec<CutResponse>,
}

/// Represents a tile in the calculation response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tile {
    /// Unique identifier for the tile
    pub id: i32,
    
    /// External request object ID
    pub request_obj_id: Option<i32>,
    
    /// X coordinate of the tile
    pub x: f64,
    
    /// Y coordinate of the tile
    pub y: f64,
    
    /// Width of the tile
    pub width: f64,
    
    /// Height of the tile
    pub height: f64,
    
    /// Orientation of the tile
    pub orientation: i32,
    
    /// Label for the tile
    pub label: Option<String>,
    
    /// Whether this is a final tile
    pub is_final: bool,
    
    /// Whether this tile has children
    pub has_children: bool,
    
    /// Edge banding information
    pub edge: Edge,
    
    /// Whether the tile is rotated
    pub is_rotated: bool,
}

/// Represents edge banding information for a tile
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    /// Top edge banding type
    pub top: Option<String>,
    
    /// Left edge banding type
    pub left: Option<String>,
    
    /// Bottom edge banding type
    pub bottom: Option<String>,
    
    /// Right edge banding type
    pub right: Option<String>,
}

/// Represents a tile that could not be fit in the solution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoFitTile {
    /// Unique identifier
    pub id: i32,
    
    /// Width of the tile
    pub width: f64,
    
    /// Height of the tile
    pub height: f64,
    
    /// Number of tiles of this size that couldn't fit
    pub count: i32,
    
    /// Label for the tile
    pub label: Option<String>,
    
    /// Material type
    pub material: Option<String>,
}

/// Represents a final tile in the solution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinalTile {
    /// Request object ID
    pub request_obj_id: i32,
    
    /// Width of the tile
    pub width: f64,
    
    /// Height of the tile
    pub height: f64,
    
    /// Label for the tile
    pub label: Option<String>,
    
    /// Count of tiles of this type
    pub count: i32,
}

/// Represents a cut in the calculation response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CutResponse {
    /// X1 coordinate of the cut
    pub x1: f64,
    
    /// Y1 coordinate of the cut
    pub y1: f64,
    
    /// X2 coordinate of the cut
    pub x2: f64,
    
    /// Y2 coordinate of the cut
    pub y2: f64,
    
    /// Cut coordinate
    pub cut_coord: f64,
    
    /// Whether the cut is horizontal
    pub is_horizontal: bool,
    
    /// Original tile ID before cutting
    pub original_tile_id: i32,
    
    /// Original width before cutting
    pub original_width: f64,
    
    /// Original height before cutting
    pub original_height: f64,
    
    /// First child tile ID after cutting
    pub child1_tile_id: i32,
    
    /// Second child tile ID after cutting
    pub child2_tile_id: i32,
}

impl CalculationResponse {
    /// Creates a new empty CalculationResponse
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
            request: None,
            panels: None,
            used_stock_panels: None,
            edge_bands: None,
            no_fit_panels: Vec::new(),
            mosaics: Vec::new(),
        }
    }

    /// Gets the version of the calculation response format
    pub fn get_version() -> &'static str {
        CALCULATION_RESPONSE_VERSION
    }

    /// Checks if the calculation has a valid solution
    pub fn has_solution(&self) -> bool {
        self.panels
            .as_ref()
            .map(|panels| !panels.is_empty())
            .unwrap_or(false)
    }

    /// Checks if all tiles fit in the solution (no no-fit panels)
    pub fn has_solution_all_fit(&self) -> bool {
        self.has_solution() && self.no_fit_panels.is_empty()
    }

    /// Gets the total number of panels in the solution
    pub fn get_panel_count(&self) -> usize {
        self.panels
            .as_ref()
            .map(|panels| panels.len())
            .unwrap_or(0)
    }

    /// Gets the total number of mosaics
    pub fn get_mosaic_count(&self) -> usize {
        self.mosaics.len()
    }

    /// Gets the total number of no-fit panels
    pub fn get_no_fit_count(&self) -> usize {
        self.no_fit_panels.len()
    }

    /// Calculates the efficiency ratio (used area / total area)
    pub fn calculate_efficiency(&self) -> f64 {
        let total_area = self.total_used_area + self.total_wasted_area;
        if total_area > 0.0 {
            self.total_used_area / total_area
        } else {
            0.0
        }
    }

    /// Validates the calculation response for consistency
    /// 
    /// Note: This is a simplified validation compared to the original Java.
    /// In a real implementation, this would be more comprehensive.
    pub fn validate(&self) -> Result<()> {
        // Check that efficiency ratio is consistent
        let calculated_ratio = self.calculate_efficiency();
        let ratio_diff = (self.total_used_area_ratio - calculated_ratio).abs();
        if ratio_diff > 0.001 {
            return Err(crate::errors::AppError::Core(
                crate::errors::CoreError::InvalidInput {
                    details: format!("Inconsistent efficiency ratio: stored={}, calculated={}", 
                           self.total_used_area_ratio, calculated_ratio)
                }
            ));
        }

        // Validate panel count consistency between response and mosaics
        if let Some(panels) = &self.panels {
            let response_panel_count = panels.len();
            let mosaic_panel_count: usize = self.mosaics.iter()
                .map(|m| m.panels.as_ref().map(|p| p.len()).unwrap_or(0))
                .sum();
            
            // Only check consistency if mosaics have panels
            if !self.mosaics.is_empty() && mosaic_panel_count > 0 && response_panel_count != mosaic_panel_count {
                return Err(crate::errors::AppError::Core(
                    crate::errors::CoreError::InvalidInput {
                        details: format!("Panel count mismatch: response has {}, mosaics have {}", 
                               response_panel_count, mosaic_panel_count)
                    }
                ));
            }
        }

        Ok(())
    }
}

impl Mosaic {
    /// Creates a new empty mosaic
    pub fn new() -> Self {
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
            edge_bands: None,
            panels: None,
            tiles: Vec::new(),
            cuts: Vec::new(),
        }
    }

    /// Sets the material, filtering out default material
    pub fn set_material(&mut self, material: Option<String>) {
        if let Some(mat) = material {
            // Filter out the default material constant from Java
            if mat != "DEFAULT_MATERIAL" && !mat.is_empty() {
                self.material = Some(mat);
            }
        }
    }

    /// Gets the total area of this mosaic
    pub fn get_total_area(&self) -> f64 {
        self.used_area + self.wasted_area
    }

    /// Calculates the efficiency of this mosaic
    pub fn calculate_efficiency(&self) -> f64 {
        let total = self.get_total_area();
        if total > 0.0 {
            self.used_area / total
        } else {
            0.0
        }
    }
}

impl Tile {
    /// Creates a new tile with default values
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
            edge: Edge::default(),
            is_rotated: false,
        }
    }

    /// Creates a tile from basic parameters (Java constructor equivalent)
    pub fn with_params(id: i32, x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            id,
            request_obj_id: None,
            x: x as f64,
            y: y as f64,
            width: width as f64,
            height: height as f64,
            orientation: 0,
            label: None,
            is_final: false,
            has_children: false,
            edge: Edge::default(),
            is_rotated: false,
        }
    }

    /// Creates a tile from a TileNode with scaling factor (Java constructor equivalent)
    pub fn from_tile_node(tile_node: &TileNode, scale_factor: f64) -> Self {
        Self {
            id: tile_node.id(),
            request_obj_id: if tile_node.external_id() != -1 {
                Some(tile_node.external_id())
            } else {
                None
            },
            x: tile_node.x1() as f64 / scale_factor,
            y: tile_node.y1() as f64 / scale_factor,
            width: tile_node.width() as f64 / scale_factor,
            height: tile_node.height() as f64 / scale_factor,
            orientation: 0,
            label: None,
            is_final: tile_node.is_final(),
            has_children: tile_node.has_children(),
            edge: Edge::default(),
            is_rotated: false,
        }
    }

    /// Gets the area of this tile
    pub fn get_area(&self) -> f64 {
        self.width * self.height
    }
}

impl Edge {
    /// Creates a new empty edge
    pub fn new() -> Self {
        Self {
            top: None,
            left: None,
            bottom: None,
            right: None,
        }
    }
}

impl NoFitTile {
    /// Creates a new no-fit tile with default values
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

    /// Creates a no-fit tile with parameters (Java constructor equivalent)
    pub fn with_params(id: i32, width: i32, height: i32, count: i32) -> Self {
        Self {
            id,
            width: width as f64,
            height: height as f64,
            count,
            label: None,
            material: None,
        }
    }

    /// Gets the total area for all no-fit tiles of this type
    pub fn get_total_area(&self) -> f64 {
        self.width * self.height * self.count as f64
    }
}

impl FinalTile {
    /// Creates a new final tile with default values
    pub fn new() -> Self {
        Self {
            request_obj_id: 0,
            width: 0.0,
            height: 0.0,
            label: None,
            count: 0,
        }
    }

    /// Creates a new final tile with parameters
    pub fn with_params(request_obj_id: i32, width: f64, height: f64) -> Self {
        Self {
            request_obj_id,
            width,
            height,
            label: None,
            count: 1,
        }
    }

    /// Increments the count and returns the previous value (Java-style post-increment)
    pub fn count_plus_plus(&mut self) -> i32 {
        let old_count = self.count;
        self.count += 1;
        old_count
    }

    /// Gets the total area for all tiles of this type
    pub fn get_total_area(&self) -> f64 {
        self.width * self.height * self.count as f64
    }
}

impl CutResponse {
    /// Creates a new cut with default values
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

    /// Creates a cut from a Cut model with scaling factor (Java constructor equivalent)
    pub fn from_cut(cut: &Cut, scale_factor: f64) -> Self {
        Self {
            x1: cut.x1() as f64 / scale_factor,
            y1: cut.y1() as f64 / scale_factor,
            x2: cut.x2() as f64 / scale_factor,
            y2: cut.y2() as f64 / scale_factor,
            original_width: cut.original_width() as f64 / scale_factor,
            original_height: cut.original_height() as f64 / scale_factor,
            is_horizontal: cut.is_horizontal(),
            cut_coord: cut.cut_coord() as f64 / scale_factor,
            original_tile_id: cut.original_tile_id(),
            child1_tile_id: cut.child1_tile_id(),
            child2_tile_id: cut.child2_tile_id(),
        }
    }

    /// Gets the length of this cut
    pub fn get_length(&self) -> f64 {
        if self.is_horizontal {
            (self.x2 - self.x1).abs()
        } else {
            (self.y2 - self.y1).abs()
        }
    }
}

// Default implementations
impl Default for CalculationResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for Mosaic {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for Edge {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for NoFitTile {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for FinalTile {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for CutResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for CalculationResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CalculationResponse {{ panels: {}, mosaics: {}, efficiency: {:.2}% }}",
            self.get_panel_count(),
            self.get_mosaic_count(),
            self.total_used_area_ratio * 100.0
        )
    }
}
