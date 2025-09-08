use std::{
    collections::LinkedList,
    sync::atomic::{AtomicI32, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};

use crate::features::{
    engine::model::{calculation_response::Mosaic, stock_solution::StockSolution},
    input::models::tile_dimensions::TileDimensions,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Solution {
    pub id: i32,
    pub timestamp: u64,
    pub mosaics: Vec<Mosaic>,
    pub unused_stock_panels: LinkedList<TileDimensions>,
    pub no_fit_panels: Vec<TileDimensions>,
    pub aux_info: Option<String>,
    pub creator_thread_group: Option<String>,
}
static SOLUTION_ID_COUNTER: AtomicI32 = AtomicI32::new(0);
impl Default for Solution {
    fn default() -> Self {
        Self {
            id: SOLUTION_ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap() 
                .as_millis() as u64,
            mosaics: Vec::new(),
            unused_stock_panels: LinkedList::new(),
            no_fit_panels: Vec::new(),
            aux_info: None,
            creator_thread_group: None,
        }
    }
}

impl Solution {
    /// Java: public Solution(StockSolution stockSolution)
    pub fn from_stock_solution(stock_solution: &StockSolution) -> Self {
        let mut solution = Self::default();

        // Java: while (it.hasNext()) { this.unusedStockPanels.add(new TileDimensions(it.next())); }
        for stock_tile in &stock_solution.stock_tiles {
            solution.unused_stock_panels.push_back(stock_tile.clone());
        }

        // Java: addMosaic(new Mosaic(this.unusedStockPanels.poll()));
        if let Some(first_stock_tile) = solution.unused_stock_panels.pop_front() {
            let mut mosaic = Mosaic::new();
            mosaic.material = Some(first_stock_tile.material.clone());

            // Create root tile node for this mosaic (like Java constructor Mosaic(TileDimensions))
            let root_node = crate::features::engine::model::tile_node::TileNode::new(
                0,
                first_stock_tile.width as i32,
                0,
                first_stock_tile.height as i32,
            );
       
            mosaic.wasted_area = (first_stock_tile.width * first_stock_tile.height) as f64;

            solution.add_mosaic(mosaic);
        }

        solution
    }

    /// Java: public Solution(Solution solution, Mosaic mosaic)
    pub fn from_solution_excluding_mosaic(solution: &Solution, mosaic_to_exclude: &Mosaic) -> Self {
        let mut new_solution = Self::default();

        // Java: for (Mosaic mosaic2 : solution.mosaics) { if (mosaic2 != mosaic) { this.mosaics.add(new Mosaic(mosaic2)); } }
        for mosaic in &solution.mosaics {
            // Compare by reference-like behavior (using some unique identifier)
            if !std::ptr::eq(mosaic as *const _, mosaic_to_exclude as *const _) {
                new_solution.mosaics.push(mosaic.clone());
            }
        }

        // Java: while (it.hasNext()) { this.unusedStockPanels.add(new TileDimensions(it.next())); }
        for unused_stock in &solution.unused_stock_panels {
            new_solution
                .unused_stock_panels
                .push_back(unused_stock.clone());
        }

        // Java: this.noFitPanels = new ArrayList(solution.getNoFitPanels());
        new_solution.no_fit_panels = solution.no_fit_panels.clone();

        new_solution
    }

    /// Sort mosaics by unused area (ascending) - matches Java's sortMosaics() method
    fn sort_mosaics(&mut self) {
        self.mosaics.sort_by(|a, b| {
            // Java: Long.compare(mosaic.getUnusedArea(), mosaic2.getUnusedArea())
            let unused_a = a.get_unused_area();
            let unused_b = b.get_unused_area();
            unused_a.cmp(&unused_b)
        });
    }

    /// Add a mosaic and automatically sort - matches Java's addMosaic() method
    pub fn add_mosaic(&mut self, mosaic: Mosaic) {
        // Java: this.mosaics.add(mosaic);
        self.mosaics.push(mosaic);
        // Java: sortMosaics();
        self.sort_mosaics();
    }

    pub fn add_all_mosaics(&mut self, mosaics: Vec<Mosaic>) {
        // Java: this.mosaics.addAll(mosaics);
        self.mosaics.extend(mosaics);
        // Java: sortMosaics();
        self.sort_mosaics();
    }

    pub fn add_all_no_fit_panels(&mut self, no_fit_panels: Vec<TileDimensions>) {
        // Java: this.noFitPanels.addAll(noFitPanels);
        self.no_fit_panels.extend(no_fit_panels);
    }

    /// Java: public void setCreatorThreadGroup(String str)
    pub fn set_creator_thread_group(&mut self, group: String) {
        self.creator_thread_group = Some(group);
    }

    /// Java: public void setAuxInfo(String str)
    pub fn set_aux_info(&mut self, aux_info: String) {
        self.aux_info = Some(aux_info);
    }

    /// Java: public String getMaterial()
    pub fn get_material(&self) -> Option<String> {
        if !self.mosaics.is_empty() {
            self.mosaics[0].material.clone()
        } else {
            None
        }
    }

    /// Java: public LinkedList<TileDimensions> getUnusedStockPanels()
    pub fn get_unused_stock_panels(&self) -> &LinkedList<TileDimensions> {
        &self.unused_stock_panels
    }

    /// Java: public List<TileDimensions> getNoFitPanels()
    pub fn get_no_fit_panels(&self) -> &Vec<TileDimensions> {
        &self.no_fit_panels
    }

    /// Java: public List<TileDimensions> getNoFitPanels()
    pub fn get_no_fit_panels_mut(&mut self) -> &mut Vec<TileDimensions> {
        &mut self.no_fit_panels
    }

    /// Java: public final List<Mosaic> getMosaics()
    pub fn get_mosaics(&self) -> &Vec<Mosaic> {
        &self.mosaics
    }

    /// Get mutable access to mosaics (needed for Java iterator pattern)
    pub fn get_mosaics_mut(&mut self) -> &mut Vec<Mosaic> {
        &mut self.mosaics
    }

    /// Get mutable access to unused stock panels (needed for Java iterator pattern)
    pub fn get_unused_stock_panels_mut(&mut self) -> &mut LinkedList<TileDimensions> {
        &mut self.unused_stock_panels
    }

    /// Java: public int getNbrFinalTiles()
    pub fn get_nbr_final_tiles(&self) -> i32 {
        let mut nbr_final_tiles = 0;
        for mosaic in &self.mosaics {
        
        }
        nbr_final_tiles
    }

    /// Java: public long getUnusedArea()
    pub fn get_unused_area(&self) -> i64 {
        let mut unused_area = 0;
        for mosaic in &self.mosaics {
            unused_area += mosaic.get_unused_area();
        }
        unused_area
    }

    /// Java: public int getNbrCuts()
    pub fn get_nbr_cuts(&self) -> i32 {
        let mut nbr_cuts = 0;
        for mosaic in &self.mosaics {
            nbr_cuts += mosaic.cuts.len() as i32;
        }
        nbr_cuts
    }

    /// Java: public int getNbrMosaics()
    pub fn get_nbr_mosaics(&self) -> i32 {
        self.mosaics.len() as i32
    }

    /// Java: public long getTotalArea()
    pub fn get_total_area(&self) -> i64 {
        let mut total_area = 0;
        for mosaic in &self.mosaics {
       
        }
        total_area
    }

    /// Java: public int getDistictTileSet()
    pub fn get_distict_tile_set(&self) -> i32 {
        // Java: Iterator<Mosaic> it = this.mosaics.iterator(); int iMax = 0;
        let mut i_max = 0;
        // Java: while (it.hasNext()) { iMax = Math.max(it.next().getDistictTileSet().size(), iMax); }
        for mosaic in &self.mosaics {
            let tile_set_size = mosaic.get_distict_tile_set().len() as i32;
            i_max = i_max.max(tile_set_size);
        }
        i_max
    }

    /// Java: public long getBiggestArea()
    pub fn get_biggest_area(&self) -> i64 {
        // Java: Iterator<Mosaic> it = this.mosaics.iterator(); long jMax = 0;
        let mut j_max = 0;
        // Java: while (it.hasNext()) { jMax = Math.max(it.next().getBiggestArea(), jMax); }
        for mosaic in &self.mosaics {
            j_max = j_max.max(mosaic.get_biggest_area());
        }
        j_max
    }

    /// Java: public float getHVDiff()
    pub fn get_hvdiff(&self) -> f32 {
        // Java: Iterator<Mosaic> it = this.mosaics.iterator(); float hVDiff = 0.0f;
        let mut hv_diff = 0.0;
        // Java: while (it.hasNext()) { hVDiff += it.next().getHVDiff(); }
        for mosaic in &self.mosaics {
            hv_diff += mosaic.get_hvdiff();
        }
        // Java: return hVDiff / this.mosaics.size();
        if self.mosaics.is_empty() {
            0.0
        } else {
            hv_diff / self.mosaics.len() as f32
        }
    }
}
