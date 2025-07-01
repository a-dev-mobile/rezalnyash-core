use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a geometric cut operation with coordinates and tile relationships
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Cut {
    /// Starting x coordinate
    x1: i32,
    /// Starting y coordinate  
    y1: i32,
    /// Ending x coordinate
    x2: i32,
    /// Ending y coordinate
    y2: i32,
    /// Original width before cut
    original_width: i32,
    /// Original height before cut
    original_height: i32,
    /// Whether the cut is horizontal (true) or vertical (false)
    is_horizontal: bool,
    /// Coordinate where the cut is made
    cut_coord: i32,
    /// ID of the original tile being cut
    original_tile_id: i32,
    /// ID of the first child tile after cut
    child1_tile_id: i32,
    /// ID of the second child tile after cut
    child2_tile_id: i32,
}

impl Cut {
    /// Creates a new Cut with all parameters
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        original_width: i32,
        original_height: i32,
        is_horizontal: bool,
        cut_coord: i32,
        original_tile_id: i32,
        child1_tile_id: i32,
        child2_tile_id: i32,
    ) -> Self {
        Self {
            x1,
            y1,
            x2,
            y2,
            original_width,
            original_height,
            is_horizontal,
            cut_coord,
            original_tile_id,
            child1_tile_id,
            child2_tile_id,
        }
    }

    /// Gets the starting x coordinate
    pub fn x1(&self) -> i32 {
        self.x1
    }

    /// Gets the starting y coordinate
    pub fn y1(&self) -> i32 {
        self.y1
    }

    /// Gets the ending x coordinate
    pub fn x2(&self) -> i32 {
        self.x2
    }

    /// Gets the ending y coordinate
    pub fn y2(&self) -> i32 {
        self.y2
    }

    /// Gets the original tile ID
    pub fn original_tile_id(&self) -> i32 {
        self.original_tile_id
    }

    /// Gets the first child tile ID
    pub fn child1_tile_id(&self) -> i32 {
        self.child1_tile_id
    }

    /// Gets the second child tile ID
    pub fn child2_tile_id(&self) -> i32 {
        self.child2_tile_id
    }

    /// Gets the original width
    pub fn original_width(&self) -> i32 {
        self.original_width
    }

    /// Gets the original height
    pub fn original_height(&self) -> i32 {
        self.original_height
    }

    /// Gets whether the cut is horizontal
    pub fn is_horizontal(&self) -> bool {
        self.is_horizontal
    }

    /// Gets the cut coordinate
    pub fn cut_coord(&self) -> i32 {
        self.cut_coord
    }

    /// Calculates the length of the cut
    /// Note: Fixed typo from original Java "getLenght" to "get_length"
    pub fn length(&self) -> u32 {
        ((self.x2 - self.x1).abs() + (self.y2 - self.y1).abs()) as u32
    }

    /// Gets the width of the cut area
    pub fn width(&self) -> u32 {
        (self.x2 - self.x1).abs() as u32
    }

    /// Gets the height of the cut area
    pub fn height(&self) -> u32 {
        (self.y2 - self.y1).abs() as u32
    }

    /// Gets the area of the cut
    pub fn area(&self) -> u32 {
        self.width() * self.height()
    }

    /// Checks if the cut is vertical
    pub fn is_vertical(&self) -> bool {
        !self.is_horizontal
    }

    /// Gets the coordinates as a tuple (x1, y1, x2, y2)
    pub fn coordinates(&self) -> (i32, i32, i32, i32) {
        (self.x1, self.y1, self.x2, self.y2)
    }

    /// Gets the child tile IDs as a tuple
    pub fn child_tile_ids(&self) -> (i32, i32) {
        (self.child1_tile_id, self.child2_tile_id)
    }

    /// Gets the original dimensions as a tuple (width, height)
    pub fn original_dimensions(&self) -> (i32, i32) {
        (self.original_width, self.original_height)
    }

    /// Checks if the cut contains a given point
    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        let min_x = self.x1.min(self.x2);
        let max_x = self.x1.max(self.x2);
        let min_y = self.y1.min(self.y2);
        let max_y = self.y1.max(self.y2);
        
        x >= min_x && x <= max_x && y >= min_y && y <= max_y
    }

    /// Checks if this cut intersects with another cut
    pub fn intersects_with(&self, other: &Cut) -> bool {
        let (min_x1, max_x1) = (self.x1.min(self.x2), self.x1.max(self.x2));
        let (min_y1, max_y1) = (self.y1.min(self.y2), self.y1.max(self.y2));
        let (min_x2, max_x2) = (other.x1.min(other.x2), other.x1.max(other.x2));
        let (min_y2, max_y2) = (other.y1.min(other.y2), other.y1.max(other.y2));

        !(max_x1 < min_x2 || max_x2 < min_x1 || max_y1 < min_y2 || max_y2 < min_y1)
    }
}

impl fmt::Display for Cut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Cut[({},{}) -> ({},{}), {}, coord={}, original_tile={}, children=({},{})]",
            self.x1,
            self.y1,
            self.x2,
            self.y2,
            if self.is_horizontal { "horizontal" } else { "vertical" },
            self.cut_coord,
            self.original_tile_id,
            self.child1_tile_id,
            self.child2_tile_id
        )
    }
}

impl Default for Cut {
    fn default() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, true, 0, 0, 0, 0)
    }
}

/// Builder pattern for constructing Cut instances
#[derive(Debug, Clone)]
pub struct CutBuilder {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    original_width: i32,
    original_height: i32,
    is_horizontal: bool,
    cut_coord: i32,
    original_tile_id: i32,
    child1_tile_id: i32,
    child2_tile_id: i32,
}

impl Default for CutBuilder {
    fn default() -> Self {
        Self {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
            original_width: 0,
            original_height: 0,
            is_horizontal: true, // Match Cut::default()
            cut_coord: 0,
            original_tile_id: 0,
            child1_tile_id: 0,
            child2_tile_id: 0,
        }
    }
}

impl CutBuilder {
    /// Creates a new CutBuilder
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the starting x coordinate
    pub fn x1(mut self, x1: i32) -> Self {
        self.x1 = x1;
        self
    }

    /// Sets the starting y coordinate
    pub fn y1(mut self, y1: i32) -> Self {
        self.y1 = y1;
        self
    }

    /// Sets the ending x coordinate
    pub fn x2(mut self, x2: i32) -> Self {
        self.x2 = x2;
        self
    }

    /// Sets the ending y coordinate
    pub fn y2(mut self, y2: i32) -> Self {
        self.y2 = y2;
        self
    }

    /// Sets the original width
    pub fn original_width(mut self, width: i32) -> Self {
        self.original_width = width;
        self
    }

    /// Sets the original height
    pub fn original_height(mut self, height: i32) -> Self {
        self.original_height = height;
        self
    }

    /// Sets whether the cut is horizontal
    pub fn horizontal(mut self, is_horizontal: bool) -> Self {
        self.is_horizontal = is_horizontal;
        self
    }

    /// Sets the cut coordinate
    pub fn cut_coord(mut self, coord: i32) -> Self {
        self.cut_coord = coord;
        self
    }

    /// Sets the original tile ID
    pub fn original_tile_id(mut self, id: i32) -> Self {
        self.original_tile_id = id;
        self
    }

    /// Sets the first child tile ID
    pub fn child1_tile_id(mut self, id: i32) -> Self {
        self.child1_tile_id = id;
        self
    }

    /// Sets the second child tile ID
    pub fn child2_tile_id(mut self, id: i32) -> Self {
        self.child2_tile_id = id;
        self
    }

    /// Sets both child tile IDs
    pub fn child_tile_ids(mut self, child1: i32, child2: i32) -> Self {
        self.child1_tile_id = child1;
        self.child2_tile_id = child2;
        self
    }

    /// Sets the coordinates
    pub fn coordinates(mut self, x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        self.x1 = x1;
        self.y1 = y1;
        self.x2 = x2;
        self.y2 = y2;
        self
    }

    /// Sets the original dimensions
    pub fn original_dimensions(mut self, width: i32, height: i32) -> Self {
        self.original_width = width;
        self.original_height = height;
        self
    }

    /// Builds the Cut instance
    pub fn build(self) -> Cut {
        Cut::new(
            self.x1,
            self.y1,
            self.x2,
            self.y2,
            self.original_width,
            self.original_height,
            self.is_horizontal,
            self.cut_coord,
            self.original_tile_id,
            self.child1_tile_id,
            self.child2_tile_id,
        )
    }

    /// Gets the current x1 value
    pub fn get_x1(&self) -> i32 {
        self.x1
    }

    /// Gets the current y1 value
    pub fn get_y1(&self) -> i32 {
        self.y1
    }

    /// Gets the current x2 value
    pub fn get_x2(&self) -> i32 {
        self.x2
    }

    /// Gets the current y2 value
    pub fn get_y2(&self) -> i32 {
        self.y2
    }

    /// Gets the current original width
    pub fn get_original_width(&self) -> i32 {
        self.original_width
    }

    /// Gets the current original height
    pub fn get_original_height(&self) -> i32 {
        self.original_height
    }

    /// Gets whether the cut is horizontal
    pub fn get_is_horizontal(&self) -> bool {
        self.is_horizontal
    }

    /// Gets the current cut coordinate
    pub fn get_cut_coord(&self) -> i32 {
        self.cut_coord
    }

    /// Gets the current original tile ID
    pub fn get_original_tile_id(&self) -> i32 {
        self.original_tile_id
    }

    /// Gets the current first child tile ID
    pub fn get_child1_tile_id(&self) -> i32 {
        self.child1_tile_id
    }

    /// Gets the current second child tile ID
    pub fn get_child2_tile_id(&self) -> i32 {
        self.child2_tile_id
    }
}
