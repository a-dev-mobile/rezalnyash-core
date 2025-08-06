use crate::features::{ input::models::panel::Panel, placement::Placement};

/// Полное решение задачи раскроя
#[derive(Debug, Clone)]
pub struct Solution {
    pub placements: Vec<Placement>,
    pub unplaced_panels: Vec<Panel>,
    pub stocks: Vec<Panel>, // Добавляем информацию о заготовках
    pub total_efficiency: f64,
    pub total_cuts: usize,
    pub total_cut_length: i32,
    pub total_used_area: i64,
    pub total_waste_area: i64,
}

impl Solution {
    pub fn new() -> Self {
        Self {
            placements: Vec::new(),
            unplaced_panels: Vec::new(),
            stocks: Vec::new(),
            total_efficiency: 0.0,
            total_cuts: 0,
            total_cut_length: 0,
            total_used_area: 0,
            total_waste_area: 0,
        }
    }

    pub fn new_with_stocks(stocks: Vec<Panel>) -> Self {
        Self {
            placements: Vec::new(),
            unplaced_panels: Vec::new(),
            stocks,
            total_efficiency: 0.0,
            total_cuts: 0,
            total_cut_length: 0,
            total_used_area: 0,
            total_waste_area: 0,
        }
    }

    pub fn calculate_totals(&mut self) {
        self.total_used_area = self.placements.iter().map(|p| p.used_area).sum();
        self.total_waste_area = self.placements.iter().map(|p| p.waste_area).sum();
        self.total_cuts = self.placements.iter().map(|p| p.cuts.len()).sum();
        self.total_cut_length = self
            .placements
            .iter()
            .flat_map(|p| &p.cuts)
            .map(|c| c.length())
            .sum();

        let total_area = self.total_used_area + self.total_waste_area;
        self.total_efficiency = if total_area > 0 {
            self.total_used_area as f64 / total_area as f64
        } else {
            0.0
        };
    }

    /// ✅ ТОЧНАЯ КОПИЯ JAVA: getNbrCuts() из Solution.java
    pub fn get_nbr_cuts(&self) -> i32 {
        self.placements.iter()
            .map(|placement| placement.cuts.len() as i32)
            .sum()
    }
    
    /// ✅ ТОЧНАЯ КОПИЯ JAVA: getNbrUnusedTiles() из Solution.java  
    pub fn get_nbr_unused_tiles(&self) -> i32 {
        self.placements.iter()
            .map(|placement| placement.root_node.get_unused_tiles_count())
            .sum()
    }
    
    /// ✅ ТОЧНАЯ КОПИЯ JAVA: getBiggestArea() из Solution.java
    pub fn get_biggest_area(&self) -> i64 {
        self.placements.iter()
            .map(|placement| placement.root_node.get_biggest_unused_area())
            .max()
            .unwrap_or(0)
    }
    
    /// ✅ ТОЧНАЯ КОПИЯ JAVA: количество мозаик (листов)
    pub fn get_nbr_mosaics(&self) -> i32 {
        self.placements.len() as i32
    }
    
    /// ✅ ТОЧНАЯ КОПИЯ JAVA: количество размещенных панелей
    pub fn get_nbr_tiles(&self) -> i32 {
        self.placements.iter()
            .map(|placement| placement.placed_panels.len() as i32)
            .sum()
    }
    
    /// ✅ ТОЧНАЯ КОПИЯ JAVA: общая потерянная площадь
    pub fn get_wasted_area(&self) -> i64 {
        self.total_waste_area
    }
}
