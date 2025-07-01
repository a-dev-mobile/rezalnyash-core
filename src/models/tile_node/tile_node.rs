//! TileNode implementation for cutting optimization
//! 
//! This module provides the TileNode structure which represents nodes in a binary tree
//! used for cutting optimization algorithms. Each node represents a rectangular tile
//! that can be split into child tiles.

use crate::models::{Tile, TileDimensions};
use crate::errors::CoreError;
use std::collections::HashSet;
use std::sync::atomic::{AtomicI32, Ordering};

/// Global counter for generating unique tile node IDs
static NEXT_ID: AtomicI32 = AtomicI32::new(0);

/// Represents a node in the cutting tree structure
/// 
/// Each TileNode represents a rectangular area that can be either:
/// - A leaf node (final tile that won't be cut further)
/// - An internal node with two children (representing a cut)
#[derive(Debug, Clone)]
pub struct TileNode {
    /// Unique identifier for this node
    id: i32,
    /// External identifier (from original tile dimensions)
    external_id: i32,
    /// The tile representing this node's area
    tile: Tile,
    /// Whether this is a final tile (leaf node)
    is_final: bool,
    /// Whether the area is totally used (optimization flag)
    is_area_totally_used: bool,
    /// Cached totally used area value
    totally_used_area: u64,
    /// Whether this tile has been rotated
    is_rotated: bool,
    /// Left child node
    child1: Option<Box<TileNode>>,
    /// Right child node  
    child2: Option<Box<TileNode>>,
}

impl TileNode {
    /// Creates a new TileNode from coordinates
    /// 
    /// # Arguments
    /// * `x1` - Left x-coordinate
    /// * `y1` - Top y-coordinate
    /// * `x2` - Right x-coordinate
    /// * `y2` - Bottom y-coordinate
    /// 
    /// # Returns
    /// A new TileNode with the specified coordinates
    /// 
    /// # Errors
    /// Returns `CoreError::InvalidInput` if coordinates are invalid
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Result<Self, CoreError> {
        let tile = Tile::new(x1, x2, y1, y2)?;
        
        Ok(Self {
            id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
            external_id: -1,
            tile,
            is_final: false,
            is_area_totally_used: false,
            totally_used_area: 0,
            is_rotated: false,
            child1: None,
            child2: None,
        })
    }
    
    /// Creates a new TileNode from TileDimensions
    /// 
    /// # Arguments
    /// * `tile_dimensions` - The dimensions to create the tile from
    /// 
    /// # Returns
    /// A new TileNode positioned at origin with the specified dimensions
    /// 
    /// # Errors
    /// Returns `CoreError::InvalidInput` if dimensions are invalid
    pub fn from_tile_dimensions(tile_dimensions: &TileDimensions) -> Result<Self, CoreError> {
        let tile = Tile::from_tile_dimensions(tile_dimensions)?;
        
        Ok(Self {
            id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
            external_id: -1,
            tile,
            is_final: false,
            is_area_totally_used: false,
            totally_used_area: 0,
            is_rotated: false,
            child1: None,
            child2: None,
        })
    }
    
    /// Copy constructor - creates a deep copy of another TileNode
    /// 
    /// # Arguments
    /// * `other` - The TileNode to copy
    /// 
    /// # Returns
    /// A new TileNode that is a deep copy of the original
    pub fn from_tile_node(other: &TileNode) -> Self {
        Self {
            id: other.id,
            external_id: other.external_id,
            tile: other.tile.clone(),
            is_final: other.is_final,
            is_area_totally_used: false, // Reset optimization flags
            totally_used_area: 0,
            is_rotated: other.is_rotated,
            child1: other.child1.as_ref().map(|child| Box::new(Self::from_tile_node(child))),
            child2: other.child2.as_ref().map(|child| Box::new(Self::from_tile_node(child))),
        }
    }
    
    /// Gets the tile associated with this node
    pub fn tile(&self) -> &Tile {
        &self.tile
    }
    
    /// Sets the tile for this node
    pub fn set_tile(&mut self, tile: Tile) {
        self.tile = tile;
    }
    
    /// Checks if this is a final tile (leaf node)
    pub fn is_final(&self) -> bool {
        self.is_final
    }
    
    /// Sets whether this is a final tile
    pub fn set_final(&mut self, is_final: bool) {
        self.is_final = is_final;
    }
    
    /// Gets the external ID
    pub fn external_id(&self) -> i32 {
        self.external_id
    }
    
    /// Sets the external ID
    pub fn set_external_id(&mut self, external_id: i32) {
        self.external_id = external_id;
    }
    
    /// Gets the unique ID
    pub fn id(&self) -> i32 {
        self.id
    }
    
    /// Gets the first child node
    pub fn child1(&self) -> Option<&TileNode> {
        self.child1.as_deref()
    }
    
    /// Sets the first child node
    pub fn set_child1(&mut self, child: Option<TileNode>) {
        self.child1 = child.map(Box::new);
    }
    
    /// Gets the second child node
    pub fn child2(&self) -> Option<&TileNode> {
        self.child2.as_deref()
    }
    
    /// Sets the second child node
    pub fn set_child2(&mut self, child: Option<TileNode>) {
        self.child2 = child.map(Box::new);
    }
    
    /// Checks if this node has children
    pub fn has_children(&self) -> bool {
        self.child1.is_some() || self.child2.is_some()
    }
    
    /// Checks if this tile is rotated
    pub fn is_rotated(&self) -> bool {
        self.is_rotated
    }
    
    /// Sets whether this tile is rotated
    pub fn set_rotated(&mut self, is_rotated: bool) {
        self.is_rotated = is_rotated;
    }
    
    /// Finds a tile node in the tree
    /// 
    /// # Arguments
    /// * `target` - The tile node to find
    /// 
    /// # Returns
    /// Some reference to the found node, or None if not found
    pub fn find_tile(&self, target: &TileNode) -> Option<&TileNode> {
        if self == target {
            return Some(self);
        }
        
        if let Some(child1) = &self.child1 {
            if let Some(found) = child1.find_tile(target) {
                return Some(found);
            }
        }
        
        if let Some(child2) = &self.child2 {
            return child2.find_tile(target);
        }
        
        None
    }
    
    /// Replaces a tile node in the tree
    /// 
    /// # Arguments
    /// * `new_node` - The new node to insert
    /// * `target` - The node to replace
    /// 
    /// # Returns
    /// Some reference to the replaced node, or None if target not found
    pub fn replace_tile(&mut self, new_node: TileNode, target: &TileNode) -> Option<&TileNode> {
        if let Some(child1) = &self.child1 {
            if child1.find_tile(target).is_some() {
                self.child1 = Some(Box::new(new_node));
                return self.child1.as_deref();
            }
        }
        
        if let Some(child2) = &self.child2 {
            if child2.find_tile(target).is_some() {
                self.child2 = Some(Box::new(new_node));
                return self.child2.as_deref();
            }
        }
        
        None
    }
    
    /// Converts to string representation with indentation
    /// 
    /// # Arguments
    /// * `indent` - The indentation string to use
    /// 
    /// # Returns
    /// String representation of the tree structure
    pub fn append_to_string(&self, indent: &str) -> String {
        let mut result = format!(
            "\n{}({}, {})({}, {})",
            indent,
            self.tile.x1(),
            self.tile.y1(),
            self.tile.x2(),
            self.tile.y2()
        );
        
        if self.is_final {
            result.push('*');
        }
        
        if let Some(child1) = &self.child1 {
            let child_indent = format!("{}    ", indent);
            result.push_str(&child1.append_to_string(&child_indent));
        }
        
        if let Some(child2) = &self.child2 {
            let child_indent = format!("{}    ", indent);
            result.push_str(&child2.append_to_string(&child_indent));
        }
        
        result
    }
    
    /// Creates a string identifier for this tile node
    /// 
    /// # Returns
    /// A string that uniquely identifies this tile node structure
    pub fn to_string_identifier(&self) -> String {
        let mut result = String::new();
        self.append_to_string_identifier(&mut result);
        result
    }
    
    fn append_to_string_identifier(&self, result: &mut String) {
        result.push_str(&format!(
            "{}{}{}{}{}",
            self.tile.x1(),
            self.tile.y1(),
            self.tile.x2(),
            self.tile.y2(),
            self.is_final
        ));
        
        if let Some(child1) = &self.child1 {
            child1.append_to_string_identifier(result);
        }
        
        if let Some(child2) = &self.child2 {
            child2.append_to_string_identifier(result);
        }
    }
    
    /// Calculates the used area in this subtree
    /// 
    /// # Returns
    /// The total used area
    pub fn used_area(&self) -> u64 {
        if self.is_area_totally_used {
            return self.totally_used_area;
        }
        
        if self.is_final {
            return self.tile.area();
        }
        
        let mut used = 0u64;
        
        if let Some(child1) = &self.child1 {
            used += child1.used_area();
        }
        
        if let Some(child2) = &self.child2 {
            used += child2.used_area();
        }
        
        // Cache if totally used
        if used == self.tile.area() {
            // Note: We can't modify self in this method due to borrowing rules
            // The caching would need to be done differently in Rust
        }
        
        used
    }
    
    /// Gets all unused tiles in this subtree
    /// 
    /// # Returns
    /// Vector of references to unused tile nodes
    pub fn unused_tiles(&self) -> Vec<&TileNode> {
        let mut unused = Vec::new();
        self.collect_unused_tiles(&mut unused);
        unused
    }
    
    fn collect_unused_tiles<'a>(&'a self, unused: &mut Vec<&'a TileNode>) {
        if !self.is_final && self.child1.is_none() && self.child2.is_none() {
            unused.push(self);
        }
        
        if let Some(child1) = &self.child1 {
            child1.collect_unused_tiles(unused);
        }
        
        if let Some(child2) = &self.child2 {
            child2.collect_unused_tiles(unused);
        }
    }
    
    /// Gets all final tiles in this subtree
    /// 
    /// # Returns
    /// Vector of references to final tiles
    pub fn final_tiles(&self) -> Vec<&Tile> {
        let mut finals = Vec::new();
        self.collect_final_tiles(&mut finals);
        finals
    }
    
    fn collect_final_tiles<'a>(&'a self, finals: &mut Vec<&'a Tile>) {
        if self.is_final {
            finals.push(&self.tile);
        }
        
        if let Some(child1) = &self.child1 {
            child1.collect_final_tiles(finals);
        }
        
        if let Some(child2) = &self.child2 {
            child2.collect_final_tiles(finals);
        }
    }
    
    /// Gets all final tile nodes in this subtree
    /// 
    /// # Returns
    /// Vector of references to final tile nodes
    pub fn final_tile_nodes(&self) -> Vec<&TileNode> {
        let mut finals = Vec::new();
        self.collect_final_tile_nodes(&mut finals);
        finals
    }
    
    fn collect_final_tile_nodes<'a>(&'a self, finals: &mut Vec<&'a TileNode>) {
        if self.is_final {
            finals.push(self);
        }
        
        if let Some(child1) = &self.child1 {
            child1.collect_final_tile_nodes(finals);
        }
        
        if let Some(child2) = &self.child2 {
            child2.collect_final_tile_nodes(finals);
        }
    }
    
    /// Calculates the unused area
    /// 
    /// # Returns
    /// The unused area (total area - used area)
    pub fn unused_area(&self) -> u64 {
        self.tile.area() - self.used_area()
    }
    
    /// Calculates the used area ratio
    /// 
    /// # Returns
    /// The ratio of used area to total area (0.0 to 1.0)
    pub fn used_area_ratio(&self) -> f32 {
        let total_area = self.tile.area();
        if total_area == 0 {
            0.0
        } else {
            self.used_area() as f32 / total_area as f32
        }
    }
    
    /// Checks if this subtree has any final tiles
    /// 
    /// # Returns
    /// true if there are final tiles in this subtree
    pub fn has_final(&self) -> bool {
        if self.is_final {
            return true;
        }
        
        if let Some(child1) = &self.child1 {
            if child1.has_final() {
                return true;
            }
        }
        
        if let Some(child2) = &self.child2 {
            if child2.has_final() {
                return true;
            }
        }
        
        false
    }
    
    /// Counts the number of unused tiles
    /// 
    /// # Returns
    /// The number of unused tiles in this subtree
    pub fn nbr_unused_tiles(&self) -> i32 {
        let mut count = 0;
        
        if !self.is_final && self.child1.is_none() && self.child2.is_none() {
            count += 1;
        }
        
        if let Some(child1) = &self.child1 {
            count += child1.nbr_unused_tiles();
        }
        
        if let Some(child2) = &self.child2 {
            count += child2.nbr_unused_tiles();
        }
        
        count
    }
    
    /// Calculates the depth of this subtree
    /// 
    /// # Returns
    /// The maximum depth from this node to any leaf
    pub fn depth(&self) -> i32 {
        let mut max_depth = 0;
        
        if let Some(child1) = &self.child1 {
            max_depth = max_depth.max(1 + child1.depth());
        }
        
        if let Some(child2) = &self.child2 {
            max_depth = max_depth.max(1 + child2.depth());
        }
        
        max_depth
    }
    
    /// Counts the number of final tiles
    /// 
    /// # Returns
    /// The number of final tiles in this subtree
    pub fn nbr_final_tiles(&self) -> i32 {
        let mut count = if self.is_final { 1 } else { 0 };
        
        if let Some(child1) = &self.child1 {
            count += child1.nbr_final_tiles();
        }
        
        if let Some(child2) = &self.child2 {
            count += child2.nbr_final_tiles();
        }
        
        count
    }
    
    /// Finds the biggest unused area in this subtree
    /// 
    /// # Returns
    /// The largest unused area
    pub fn biggest_area(&self) -> u64 {
        let mut max_area = if self.child1.is_none() && self.child2.is_none() && !self.is_final {
            self.tile.area()
        } else {
            0
        };
        
        if let Some(child1) = &self.child1 {
            max_area = max_area.max(child1.biggest_area());
        }
        
        if let Some(child2) = &self.child2 {
            max_area = max_area.max(child2.biggest_area());
        }
        
        max_area
    }
    
    /// Counts final horizontal tiles
    /// 
    /// # Returns
    /// The number of final horizontal tiles
    pub fn nbr_final_horizontal(&self) -> i32 {
        let mut count = if self.is_final && self.tile.is_horizontal() { 1 } else { 0 };
        
        if let Some(child1) = &self.child1 {
            count += child1.nbr_final_horizontal();
        }
        
        if let Some(child2) = &self.child2 {
            count += child2.nbr_final_horizontal();
        }
        
        count
    }
    
    /// Counts final vertical tiles
    /// 
    /// # Returns
    /// The number of final vertical tiles
    pub fn nbr_final_vertical(&self) -> i32 {
        let mut count = if self.is_final && self.tile.is_vertical() { 1 } else { 0 };
        
        if let Some(child1) = &self.child1 {
            count += child1.nbr_final_vertical();
        }
        
        if let Some(child2) = &self.child2 {
            count += child2.nbr_final_vertical();
        }
        
        count
    }
    
    /// Gets the set of distinct tile dimensions
    /// 
    /// # Returns
    /// A HashSet containing unique tile dimension identifiers
    pub fn distinct_tile_set(&self) -> HashSet<i32> {
        let mut set = HashSet::new();
        self.collect_distinct_tile_set(&mut set);
        set
    }
    
    fn collect_distinct_tile_set(&self, set: &mut HashSet<i32>) {
        if self.is_final {
            let width = self.tile.width();
            let height = self.tile.height();
            let sum = width + height;
            // Cantor pairing function for unique identifier
            let identifier = ((sum * (sum + 1)) / 2) + height;
            set.insert(identifier);
        } else {
            if let Some(child1) = &self.child1 {
                child1.collect_distinct_tile_set(set);
            }
            
            if let Some(child2) = &self.child2 {
                child2.collect_distinct_tile_set(set);
            }
        }
    }
    
    /// Converts to TileDimensions
    /// 
    /// # Returns
    /// A TileDimensions representing this tile's dimensions
    pub fn to_tile_dimensions(&self) -> TileDimensions {
        TileDimensions::simple(self.tile.width() as u32, self.tile.height() as u32)
    }
    
    // Delegate methods to the underlying tile
    
    /// Gets the left x-coordinate
    pub fn x1(&self) -> i32 {
        self.tile.x1()
    }
    
    /// Gets the right x-coordinate
    pub fn x2(&self) -> i32 {
        self.tile.x2()
    }
    
    /// Gets the top y-coordinate
    pub fn y1(&self) -> i32 {
        self.tile.y1()
    }
    
    /// Gets the bottom y-coordinate
    pub fn y2(&self) -> i32 {
        self.tile.y2()
    }
    
    /// Gets the width
    pub fn width(&self) -> i32 {
        self.tile.width()
    }
    
    /// Gets the height
    pub fn height(&self) -> i32 {
        self.tile.height()
    }
    
    /// Gets the area
    pub fn area(&self) -> u64 {
        self.tile.area()
    }
    
    /// Gets the maximum side length
    pub fn max_side(&self) -> i32 {
        self.tile.max_side()
    }
    
    /// Checks if the tile is horizontal
    pub fn is_horizontal(&self) -> bool {
        self.tile.is_horizontal()
    }
    
    /// Checks if the tile is vertical
    pub fn is_vertical(&self) -> bool {
        self.tile.is_vertical()
    }
}

impl std::fmt::Display for TileNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.append_to_string(""))
    }
}

impl PartialEq for TileNode {
    fn eq(&self, other: &Self) -> bool {
        // For structural equality, we compare the tile, final status, and children
        // but not the unique ID, as equivalent structures may have different IDs
        self.tile == other.tile 
            && self.is_final == other.is_final
            && self.external_id == other.external_id
            && self.is_rotated == other.is_rotated
            && match (&self.child1, &other.child1) {
                (None, None) => true,
                (Some(c1), Some(c2)) => c1 == c2,
                _ => false,
            }
            && match (&self.child2, &other.child2) {
                (None, None) => true,
                (Some(c1), Some(c2)) => c1 == c2,
                _ => false,
            }
    }
}

impl Eq for TileNode {}
