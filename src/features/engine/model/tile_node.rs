use std::sync::atomic::{AtomicU32, Ordering};
use std::collections::HashSet;
use serde::{Deserialize, Serialize};

static NODE_ID_COUNTER: AtomicU32 = AtomicU32::new(1);

#[derive(Debug, Clone, Serialize, Deserialize)]
// -= доработать
pub struct TileNode {
    pub id: u32,
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    pub external_id: Option<u32>,
    pub is_final: bool,
    pub is_rotated: bool,
    pub child1: Option<Box<TileNode>>,
    pub child2: Option<Box<TileNode>>,
}

impl TileNode {
    pub fn new(x1: i32, x2: i32, y1: i32, y2: i32) -> Self {
        Self {
            id: NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            x1,
            y1,
            x2,
            y2,
            external_id: None,
            is_final: false,
            is_rotated: false,
            child1: None,
            child2: None,
        }
    }
    
    // Java copy constructor: TileNode(TileNode tileNode) - PRESERVES EXISTING ID
    // CRITICAL: Java does SHALLOW copy of children (direct reference assignment)
    pub fn copy_node(other: &TileNode) -> Self {
        Self {
            id: other.id, // CRITICAL: Keep same ID like Java constructor
            x1: other.x1,
            y1: other.y1,
            x2: other.x2,
            y2: other.y2,
            external_id: other.external_id,
            is_final: other.is_final,
            is_rotated: other.is_rotated,
            // Java: this.child1 = tileNode.getChild1(); (SHALLOW copy - direct reference!)
            child1: other.child1.clone(), // Clone the Box (shallow copy of structure)
            child2: other.child2.clone(), // Clone the Box (shallow copy of structure)
        }
    }

    pub fn get_width(&self) -> i32 {
        self.x2 - self.x1
    }

    pub fn get_height(&self) -> i32 {
        self.y2 - self.y1
    }

    pub fn get_area(&self) -> i32 {
        self.get_width() * self.get_height()
    }
    
    /// Calculate used area - matches Java TileNode.getUsedArea()
    pub fn get_used_area(&self) -> i64 {
        // Java: if (this.isFinal) { return getArea(); }
        if self.is_final {
            return self.get_area() as i64;
        }
        
        // Java: TileNode tileNode = this.child1; long usedArea = tileNode != null ? tileNode.getUsedArea() : 0L;
        let used_area = if let Some(ref child1) = self.child1 {
            child1.get_used_area()
        } else {
            0
        };
        
        // Java: TileNode tileNode2 = this.child2; if (tileNode2 != null) { usedArea += tileNode2.getUsedArea(); }
        let used_area = if let Some(ref child2) = self.child2 {
            used_area + child2.get_used_area()
        } else {
            used_area
        };
        
        used_area
    }
    
    /// Calculate unused area - matches Java TileNode.getUnusedArea()
    pub fn get_unused_area(&self) -> i64 {
        // Java: return getArea() - getUsedArea();
        self.get_area() as i64 - self.get_used_area()
    }

    pub fn get_x1(&self) -> i32 { self.x1 }
    pub fn get_y1(&self) -> i32 { self.y1 }
    pub fn get_x2(&self) -> i32 { self.x2 }
    pub fn get_y2(&self) -> i32 { self.y2 }
    pub fn get_id(&self) -> u32 { self.id }

    pub fn set_external_id(&mut self, id: Option<u32>) {
        self.external_id = id;
    }

    pub fn set_final_tile(&mut self, is_final: bool) {
        self.is_final = is_final;
    }

    pub fn set_rotated(&mut self, is_rotated: bool) {
        self.is_rotated = is_rotated;
    }

    pub fn set_child1(&mut self, child: Option<Box<TileNode>>) {
        self.child1 = child;
    }

    pub fn set_child2(&mut self, child: Option<Box<TileNode>>) {
        self.child2 = child;
    }

    pub fn get_child1(&self) -> &Option<Box<TileNode>> {
        &self.child1
    }

    pub fn get_child2(&self) -> &Option<Box<TileNode>> {
        &self.child2
    }

    pub fn get_child1_mut(&mut self) -> Option<&mut TileNode> {
        self.child1.as_deref_mut()
    }

    pub fn get_child2_mut(&mut self) -> Option<&mut TileNode> {
        self.child2.as_deref_mut()
    }

    pub fn find_tile(&self, target: &TileNode) -> Option<&TileNode> {
        if self.id == target.id {
            return Some(self);
        }
        
        if let Some(ref child1) = self.child1 {
            if let Some(result) = child1.find_tile(target) {
                return Some(result);
            }
        }
        
        if let Some(ref child2) = self.child2 {
            if let Some(result) = child2.find_tile(target) {
                return Some(result);
            }
        }
        
        None
    }

    pub fn find_tile_mut(&mut self, target: &TileNode) -> Option<&mut TileNode> {
        if self.id == target.id {
            return Some(self);
        }
        
        if let Some(ref mut child1) = self.child1 {
            if let Some(result) = child1.find_tile_mut(target) {
                return Some(result);
            }
        }
        
        if let Some(ref mut child2) = self.child2 {
            if let Some(result) = child2.find_tile_mut(target) {
                return Some(result);
            }
        }
        
        None
    }

    pub fn to_string_identifier(&self) -> String {
        let mut result = String::new();
        self.append_to_string_identifier(&mut result);
        result
    }

    fn append_to_string_identifier(&self, sb: &mut String) {
        // Java: sb.append(this.tile.getX1()); sb.append(this.tile.getY1()); etc.
        sb.push_str(&format!("{}{}{}{}{}", self.x1, self.y1, self.x2, self.y2, self.is_final));
        
        // Java: if (tileNode != null) { tileNode.appendToStringIdentifier(sb); }
        if let Some(ref child1) = self.child1 {
            child1.append_to_string_identifier(sb);
        }
        if let Some(ref child2) = self.child2 {
            child2.append_to_string_identifier(sb);
        }
    }
    
    /// Java: public int getNbrFinalTiles()
    pub fn get_nbr_final_tiles(&self) -> i32 {
        if self.is_final {
            return 1;
        }
        
        let mut nbr_final_tiles = 0;
        if let Some(ref child1) = self.child1 {
            nbr_final_tiles += child1.get_nbr_final_tiles();
        }
        if let Some(ref child2) = self.child2 {
            nbr_final_tiles += child2.get_nbr_final_tiles();
        }
        
        nbr_final_tiles
    }
    
    /// Java: public HashSet<Integer> getDistictTileSet()
    pub fn get_distict_tile_set(&self) -> HashSet<i32> {
        let mut tile_set = HashSet::new();
        self.get_distict_tile_set_recursive(&mut tile_set);
        tile_set
    }
    
    /// Java: private HashSet<Integer> getDistictTileSet(HashSet<Integer> hashSet)
    fn get_distict_tile_set_recursive(&self, hash_set: &mut HashSet<i32>) {
        if self.is_final {
            // Java: int width = this.tile.getWidth(); int height = this.tile.getHeight(); int i = width + height;
            let width = self.get_width();
            let height = self.get_height();
            let i = width + height;
            // Java: hashSet.add(Integer.valueOf(((i * (i + 1)) / 2) + height));
            let pairing_value = ((i * (i + 1)) / 2) + height;
            hash_set.insert(pairing_value);
        } else {
            // Java: TileNode tileNode = this.child1; if (tileNode != null) { tileNode.getDistictTileSet(hashSet); }
            if let Some(ref child1) = self.child1 {
                child1.get_distict_tile_set_recursive(hash_set);
            }
            // Java: TileNode tileNode2 = this.child2; if (tileNode2 != null) { tileNode2.getDistictTileSet(hashSet); }
            if let Some(ref child2) = self.child2 {
                child2.get_distict_tile_set_recursive(hash_set);
            }
        }
    }
    
    /// Java: public long getBiggestArea()
    pub fn get_biggest_area(&self) -> i64 {
        // Java: long area = (getChild1() == null && getChild2() == null && !this.isFinal) ? getArea() : 0L;
        let area = if self.child1.is_none() && self.child2.is_none() && !self.is_final {
            self.get_area() as i64
        } else {
            0
        };
        
        // Java: TileNode tileNode = this.child1; if (tileNode != null) { area = Math.max(tileNode.getBiggestArea(), area); }
        let area = if let Some(ref child1) = self.child1 {
            area.max(child1.get_biggest_area())
        } else {
            area
        };
        
        // Java: TileNode tileNode2 = this.child2; return tileNode2 != null ? Math.max(tileNode2.getBiggestArea(), area) : area;
        if let Some(ref child2) = self.child2 {
            area.max(child2.get_biggest_area())
        } else {
            area
        }
    }
    
    /// Java: public boolean isHorizontal()
    pub fn is_horizontal(&self) -> bool {
        // Java: return getWidth() > getHeight();
        self.get_width() > self.get_height()
    }
    
    /// Java: public boolean isVertical()
    pub fn is_vertical(&self) -> bool {
        // Java: return getHeight() > getWidth();
        self.get_height() > self.get_width()
    }
    
    /// Java: public int getNbrFinalHorizontal()
    pub fn get_nbr_final_horizontal(&self) -> i32 {
        // Java: int nbrFinalHorizontal = (isFinal() && isHorizontal()) ? 1 : 0;
        let nbr_final_horizontal = if self.is_final && self.is_horizontal() { 1 } else { 0 };
        
        // Java: TileNode tileNode = this.child1; if (tileNode != null) { nbrFinalHorizontal += tileNode.getNbrFinalHorizontal(); }
        let nbr_final_horizontal = if let Some(ref child1) = self.child1 {
            nbr_final_horizontal + child1.get_nbr_final_horizontal()
        } else {
            nbr_final_horizontal
        };
        
        // Java: TileNode tileNode2 = this.child2; return tileNode2 != null ? nbrFinalHorizontal + tileNode2.getNbrFinalHorizontal() : nbrFinalHorizontal;
        if let Some(ref child2) = self.child2 {
            nbr_final_horizontal + child2.get_nbr_final_horizontal()
        } else {
            nbr_final_horizontal
        }
    }
    
    /// Java: public int getNbrFinalVertical()
    pub fn get_nbr_final_vertical(&self) -> i32 {
        // Java: int nbrFinalVertical = (isFinal() && isVertical()) ? 1 : 0;
        let nbr_final_vertical = if self.is_final && self.is_vertical() { 1 } else { 0 };
        
        // Java: TileNode tileNode = this.child1; if (tileNode != null) { nbrFinalVertical += tileNode.getNbrFinalVertical(); }
        let nbr_final_vertical = if let Some(ref child1) = self.child1 {
            nbr_final_vertical + child1.get_nbr_final_vertical()
        } else {
            nbr_final_vertical
        };
        
        // Java: TileNode tileNode2 = this.child2; return tileNode2 != null ? nbrFinalVertical + tileNode2.getNbrFinalVertical() : nbrFinalVertical;
        if let Some(ref child2) = self.child2 {
            nbr_final_vertical + child2.get_nbr_final_vertical()
        } else {
            nbr_final_vertical
        }
    }
}