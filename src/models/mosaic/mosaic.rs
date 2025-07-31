//! Mosaic implementation for cutting optimization
//! 
//! This module provides the Mosaic structure which represents a cutting solution
//! containing a tree of tile nodes and associated cuts.

use crate::models::{Cut, TileDimensions, TileNode};
use crate::errors::CoreError;
use std::collections::HashSet;

/// Represents a mosaic containing cuts and tile arrangements
/// 
/// A mosaic represents a complete cutting solution for a stock piece,
/// containing the root tile node (representing the entire stock) and
/// all cuts that have been made to create the final tile arrangement.
#[derive(Debug, Clone)]
pub struct Mosaic {
    /// Root tile node representing the entire stock
    root_tile_node: TileNode,
    /// List of cuts made on this mosaic
    cuts: Vec<Cut>,
    /// Stock identifier
    stock_id: i32,
    /// Material type
    material: String,
    /// Orientation of the mosaic
    orientation: i32,
}

impl Mosaic {
    /// Creates a new Mosaic from another mosaic (copy constructor equivalent)
    /// 
    /// # Arguments
    /// * `other` - The mosaic to copy
    /// 
    /// # Returns
    /// A new Mosaic that is a deep copy of the original
    pub fn from_mosaic(other: &Mosaic) -> Self {
        Mosaic {
            root_tile_node: TileNode::from_tile_node(&other.root_tile_node),
            cuts: other.cuts.clone(),
            stock_id: other.stock_id,
            material: other.material.clone(),
            orientation: other.orientation,
        }
    }

    /// Creates a new Mosaic from a TileNode and material
    /// 
    /// # Arguments
    /// * `tile_node` - The root tile node
    /// * `material` - The material type
    /// 
    /// # Returns
    /// A new Mosaic with the specified tile node and material
    pub fn from_tile_node(tile_node: &TileNode, material: String) -> Self {
        Mosaic {
            root_tile_node: TileNode::from_tile_node(tile_node),
            cuts: Vec::new(),
            stock_id: tile_node.external_id(),
            material,
            orientation: 0,
        }
    }

    /// Creates a new Mosaic from TileDimensions
    /// 
    /// # Arguments
    /// * `tile_dimensions` - The dimensions to create the mosaic from
    /// 
    /// # Returns
    /// A new Mosaic with a root tile node created from the dimensions
    /// 
    /// # Errors
    /// Returns `CoreError::InvalidInput` if tile dimensions are invalid
    pub fn from_tile_dimensions(tile_dimensions: &TileDimensions) -> Result<Self, CoreError> {
        let mut root_node = TileNode::from_tile_dimensions(tile_dimensions)?;
        root_node.set_external_id(tile_dimensions.id());
        
        Ok(Mosaic {
            root_tile_node: root_node,
            cuts: Vec::new(),
            stock_id: tile_dimensions.id(),
            material: tile_dimensions.material().to_string(),
            orientation: tile_dimensions.orientation() as i32,
        })
    }

    /// Creates a new Mosaic from TileDimensions (alias for from_tile_dimensions)
    /// 
    /// # Arguments
    /// * `tile_dimensions` - The dimensions to create the mosaic from
    /// 
    /// # Returns
    /// A new Mosaic with a root tile node created from the dimensions
    /// 
    /// # Errors
    /// Returns `CoreError::InvalidInput` if tile dimensions are invalid
    pub fn new(tile_dimensions: &TileDimensions) -> Result<Self, CoreError> {
        Self::from_tile_dimensions(tile_dimensions)
    }

    /// Creates a new Mosaic from a root tile node and optional material
    /// 
    /// # Arguments
    /// * `root_tile_node` - The root tile node for this mosaic
    /// * `material` - Optional material type
    /// 
    /// # Returns
    /// A new Mosaic with the specified root tile node
    pub fn from_root_tile_node(root_tile_node: TileNode, material: Option<String>) -> Self {
        Mosaic {
            stock_id: root_tile_node.external_id(),
            orientation: 0,
            material: material.unwrap_or_else(|| "Default".to_string()),
            cuts: Vec::new(),
            root_tile_node,
        }
    }

    /// Gets the root tile node
    pub fn root_tile_node(&self) -> &TileNode {
        &self.root_tile_node
    }

    /// Sets the root tile node
    pub fn set_root_tile_node(&mut self, tile_node: TileNode) {
        self.root_tile_node = tile_node;
    }

    /// Gets the cuts
    pub fn cuts(&self) -> &[Cut] {
        &self.cuts
    }

    /// Sets the cuts
    pub fn set_cuts(&mut self, cuts: Vec<Cut>) {
        self.cuts = cuts;
    }

    /// Gets mutable reference to cuts
    pub fn get_cuts_mut(&mut self) -> &mut Vec<Cut> {
        &mut self.cuts
    }

    /// Adds a cut to this mosaic
    pub fn add_cut(&mut self, cut: Cut) {
        self.cuts.push(cut);
    }

    /// Gets the number of cuts
    pub fn nbr_cuts(&self) -> usize {
        self.cuts.len()
    }

    /// Gets the stock ID
    pub fn stock_id(&self) -> i32 {
        self.stock_id
    }

    /// Sets the stock ID
    pub fn set_stock_id(&mut self, stock_id: i32) {
        self.stock_id = stock_id;
    }

    /// Gets the material
    pub fn material(&self) -> &str {
        &self.material
    }

    /// Sets the material
    pub fn set_material(&mut self, material: String) {
        self.material = material;
    }

    /// Gets the orientation
    pub fn orientation(&self) -> i32 {
        self.orientation
    }

    /// Sets the orientation
    pub fn set_orientation(&mut self, orientation: i32) {
        self.orientation = orientation;
    }

    /// Gets all final tile nodes
    /// 
    /// # Returns
    /// Vector of references to all final (leaf) tile nodes in the mosaic
    pub fn final_tile_nodes(&self) -> Vec<&TileNode> {
        self.root_tile_node.final_tile_nodes()
    }

    /// Gets the horizontal-vertical difference
    /// 
    /// # Returns
    /// The absolute difference between horizontal and vertical final tiles
    pub fn hv_diff(&self) -> f32 {
        (self.root_tile_node.nbr_final_horizontal() - self.root_tile_node.nbr_final_vertical()).abs() as f32
    }

    /// Gets distinct tile set
    /// 
    /// # Returns
    /// A HashSet containing unique tile dimension identifiers
    pub fn distinct_tile_set(&self) -> HashSet<i32> {
        self.root_tile_node.distinct_tile_set()
    }

    /// Gets the used area
    /// 
    /// # Returns
    /// The total used area in the mosaic
    pub fn used_area(&self) -> u64 {
        self.root_tile_node.used_area()
    }

    /// Gets the unused area
    /// 
    /// # Returns
    /// The total unused area in the mosaic
    pub fn unused_area(&self) -> u64 {
        self.root_tile_node.unused_area()
    }

    /// Gets the depth of the cutting tree
    /// 
    /// # Returns
    /// The maximum depth of the tile node tree
    pub fn depth(&self) -> i32 {
        self.root_tile_node.depth()
    }

    /// Gets the biggest unused tile
    /// 
    /// # Returns
    /// Some reference to the largest unused tile, or None if all tiles are used
    pub fn biggest_unused_tile(&self) -> Option<&TileNode> {
        let unused_tiles = self.root_tile_node.unused_tiles();
        unused_tiles.iter()
            .max_by_key(|tile| tile.area())
            .copied()
    }

    /// Calculates the center of mass distance to origin
    /// 
    /// This method calculates the center of mass of all final tiles
    /// and returns the distance from origin normalized by the diagonal length.
    /// 
    /// # Returns
    /// The normalized distance from origin to center of mass (0.0 to ~1.0)
    pub fn center_of_mass_distance_to_origin(&self) -> f32 {
        let used_area = self.used_area();
        if used_area == 0 {
            return 0.0;
        }

        let mut weighted_x = 0.0f32;
        let mut weighted_y = 0.0f32;

        for tile_node in self.final_tile_nodes() {
            let area = tile_node.area() as f32;
            let center_x = tile_node.x1() as f32 + (tile_node.width() as f32 * 0.5);
            let center_y = tile_node.y1() as f32 + (tile_node.height() as f32 * 0.5);
            
            weighted_x += area * center_x;
            weighted_y += area * center_y;
        }

        let center_of_mass_x = weighted_x / used_area as f32;
        let center_of_mass_y = weighted_y / used_area as f32;

        let distance = (center_of_mass_x.powi(2) + center_of_mass_y.powi(2)).sqrt();
        let diagonal = ((self.root_tile_node.width() as f32).powi(2) + (self.root_tile_node.height() as f32).powi(2)).sqrt();

        distance / diagonal
    }

    /// Gets the biggest area
    /// 
    /// # Returns
    /// The largest unused area in the mosaic
    pub fn biggest_area(&self) -> u64 {
        self.root_tile_node.biggest_area()
    }


    /// Removes a cut at the specified index
    /// 
    /// # Arguments
    /// * `index` - The index of the cut to remove
    /// 
    /// # Returns
    /// The removed cut, or an error if index is out of bounds
    /// 
    /// # Errors
    /// Returns `CoreError::InvalidInput` if index is out of bounds
    pub fn remove_cut(&mut self, index: usize) -> Result<Cut, CoreError> {
        if index >= self.cuts.len() {
            return Err(CoreError::InvalidInput {
                details: format!("Cut index {} is out of bounds", index)
            });
        }
        Ok(self.cuts.remove(index))
    }

    /// Clears all cuts from the mosaic
    pub fn clear_cuts(&mut self) {
        self.cuts.clear();
    }

    /// Gets the total area of the mosaic (root tile area)
    /// 
    /// # Returns
    /// The total area of the root tile
    pub fn total_area(&self) -> u64 {
        self.root_tile_node.area()
    }

    /// Calculates the used area ratio
    /// 
    /// # Returns
    /// The ratio of used area to total area (0.0 to 1.0)
    pub fn used_area_ratio(&self) -> f32 {
        self.root_tile_node.used_area_ratio()
    }

    /// Gets the number of final tiles
    /// 
    /// # Returns
    /// The total number of final (leaf) tiles
    pub fn nbr_final_tiles(&self) -> i32 {
        self.root_tile_node.nbr_final_tiles()
    }

    /// Gets the number of unused tiles
    /// 
    /// # Returns
    /// The total number of unused tiles
    pub fn nbr_unused_tiles(&self) -> i32 {
        self.root_tile_node.nbr_unused_tiles()
    }

    /// Checks if the mosaic has any final tiles
    /// 
    /// # Returns
    /// true if there are any final tiles in the mosaic
    pub fn has_final(&self) -> bool {
        self.root_tile_node.has_final()
    }

    /// Gets all unused tiles
    /// 
    /// # Returns
    /// Vector of references to all unused tiles
    pub fn unused_tiles(&self) -> Vec<&TileNode> {
        self.root_tile_node.unused_tiles()
    }

    /// Validates the mosaic structure
    /// 
    /// This method performs basic validation checks on the mosaic structure
    /// to ensure consistency and correctness.
    /// 
    /// # Returns
    /// Ok(()) if the mosaic is valid, or an error describing the issue
    /// 
    /// # Errors
    /// Returns `CoreError::InvalidInput` if validation fails
    pub fn validate(&self) -> Result<(), CoreError> {
        // Validate that stock_id is consistent
        if self.stock_id != self.root_tile_node.external_id() && self.root_tile_node.external_id() != -1 {
            return Err(CoreError::InvalidInput {
                details: "Stock ID mismatch between mosaic and root tile node".to_string(),
            });
        }

        // Validate that cuts don't exceed the number of internal nodes
        // This is a basic check - more sophisticated validation could be added
        let max_possible_cuts = self.nbr_final_tiles().saturating_sub(1).max(0) as usize;
        if self.cuts.len() > max_possible_cuts {
            return Err(CoreError::InvalidInput {
                details: format!(
                    "Too many cuts ({}) for the number of final tiles ({})",
                    self.cuts.len(),
                    self.nbr_final_tiles()
                ),
            });
        }

        Ok(())
    }

    /// Creates a summary string of the mosaic
    /// 
    /// # Returns
    /// A string containing key information about the mosaic
    pub fn summary(&self) -> String {
        format!(
            "Mosaic[stock_id={}, material={}, cuts={}, final_tiles={}, used_area={}, total_area={}, efficiency={:.2}%]",
            self.stock_id,
            self.material,
            self.nbr_cuts(),
            self.nbr_final_tiles(),
            self.used_area(),
            self.total_area(),
            self.used_area_ratio() * 100.0
        )
    }
}

impl PartialEq for Mosaic {
    fn eq(&self, other: &Self) -> bool {
        self.root_tile_node == other.root_tile_node
    }
}

impl Eq for Mosaic {}

impl std::fmt::Display for Mosaic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}
