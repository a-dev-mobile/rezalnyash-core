use serde::{Deserialize, Serialize};

use crate::features::engine::model::{
    calculation_response::{Cut, Mosaic}, solution::Solution, status::Status, stock_solution::StockSolution, task::Task, tile_node::TileNode
};
use crate::features::input::models::tile_dimensions::TileDimensions;
use crate::enums::cut_orientation_preference::CutOrientationPreference;
use crate::enums::orientation::Orientation;
use crate::features::engine::comparator::{PriorityListFactory, SolutionComparator};

#[derive(Clone,Debug, Serialize, Deserialize)]
pub struct CutListThread {
    pub accuracy_factor: i32,
    pub all_solutions: Vec<Solution>,
    pub aux_info: String,
    pub consider_grain_direction: bool,
    pub cut_thickness: i32,
    pub first_cut_orientation: CutOrientationPreference,
    pub group: String,
    pub solutions: Vec<Solution>,
    pub start_time: Option<i64>,
    pub stock_solution: Option<StockSolution>,
    pub task: Option<Task>,
    pub tiles: Vec<TileDimensions>,
    pub status: Status,
    pub percentage_done: i32,
    pub min_trim_dimension: i32,
}

impl CutListThread {
    pub fn new() -> Self {
        Self {
            accuracy_factor: 200, // Will be overridden from configuration
            all_solutions: Vec::new(),
            aux_info: String::new(),
            consider_grain_direction: false, // Will be overridden from configuration
            cut_thickness: 0, // Will be overridden from configuration
            first_cut_orientation: CutOrientationPreference::Both, // Will be overridden based on group
            group: String::new(),
            solutions: Vec::new(),
            start_time: None,
            stock_solution: None,
            task: None,
            tiles: Vec::new(),
            status: Status::Queued,
            percentage_done: 0,
            min_trim_dimension: 0, // Will be overridden from configuration
        }
    }

    pub fn new_with_config(
        configuration: &crate::features::engine::model::configuration::Configuration,
        optimization_factor: i32,
    ) -> Self {
        // Parse cut_thickness from configuration (Java: Double.parseDouble)
        let cut_thickness = if let Some(ref cut_thickness_str) = configuration.cut_thickness {
            cut_thickness_str.parse::<f64>().unwrap_or(0.0) as i32
        } else {
            0
        };
        
        // Parse min_trim_dimension from configuration (Java: Double.parseDouble)
        let min_trim_dimension = if let Some(ref min_trim_str) = configuration.min_trim_dimension {
            min_trim_str.parse::<f64>().unwrap_or(0.0) as i32
        } else {
            0
        };

        Self {
            accuracy_factor: optimization_factor,
            all_solutions: Vec::new(),
            aux_info: String::new(),
            consider_grain_direction: configuration.consider_orientation,
            cut_thickness,
            first_cut_orientation: configuration.cut_orientation_preference,
            group: String::new(),
            solutions: Vec::new(),
            start_time: None,
            stock_solution: None,
            task: None,
            tiles: Vec::new(),
            status: Status::Queued,
            percentage_done: 0,
            min_trim_dimension,
        }
    }

    pub fn execute(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("=== CUTLIST_THREAD_EXECUTE_START ===");
        println!("INPUT_PARAMS: group={}, auxInfo={}, tilesCount={}, cutDirection={:?}, accuracyFactor={}, cutThickness={}, minTrimDimension={}", 
                 self.group, self.aux_info, self.tiles.len(), self.first_cut_orientation, 
                 self.accuracy_factor, self.cut_thickness, self.min_trim_dimension);
        println!("ALGORITHM: Cut list thread processing with sequential tile placement");

        self.status = Status::Running;
        self.start_time = Some(chrono::Utc::now().timestamp_millis());
        println!("STEP_STATUS_CHANGE: Status.QUEUED -> Status.RUNNING");
        println!("STEP_TIMER: startTime={}", self.start_time.unwrap_or(0));

        println!("STEP_COMPUTE: Calling computeSolutions()");
        match self.compute_solutions() {
            Ok(_) => {
                if self.status != Status::Terminated {
                    self.status = Status::Finished;
                    println!("STEP_STATUS_CHANGE: Status.RUNNING -> Status.FINISHED");
                } else {
                    println!("STEP_STATUS_FINAL: Status remains Status.TERMINATED");
                }
                println!("=== CUTLIST_THREAD_EXECUTE_END: SUCCESS ===");
                Ok(())
            }
            Err(e) => {
                println!("STEP_ERROR: Exception caught: {}", e);
                self.status = Status::Error;
                println!("STEP_STATUS_CHANGE: Status.RUNNING -> Status.ERROR");
                println!("=== CUTLIST_THREAD_EXECUTE_END: ERROR ===");
                Err(e)
            }
        }
    }

    pub fn get_material(&self) -> Option<String> {
        if self.all_solutions.is_empty() {
            None
        } else {
            Some("DEFAULT_MATERIAL".to_string()) // Simplified for now
        }
    }

    pub fn get_elapsed_time_millis(&self) -> i64 {
        if let Some(start_time) = self.start_time {
            chrono::Utc::now().timestamp_millis() - start_time
        } else {
            0
        }
    }

    pub fn remove_duplicated(&self, solutions: &mut Vec<Solution>) -> usize {
        let original_len = solutions.len();
        let mut unique_solutions = Vec::new();
        let mut seen_signatures = std::collections::HashSet::new();
        
        for solution in solutions.iter() {
            let mut signature = String::new();
            for mosaic in solution.get_mosaics() {
                // Java: str = str + it.next().getRootTileNode().toStringIdentifier();
          
            }
            
            if seen_signatures.insert(signature) {
                unique_solutions.push(solution.clone());
            }
        }
        
        *solutions = unique_solutions;
        original_len - solutions.len()
    }

    pub fn compute_solutions(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n=== COMPUTE_SOLUTIONS_START ===");
        
        let stock_solution = if let Some(ref stock_solution) = self.stock_solution {
            stock_solution
        } else {
            return Err("Stock solution is not available".into());
        };
        
        println!("INPUT_DATA: stockSolution=available, tilesCount={}, allSolutionsSize={}", 
                 self.tiles.len(), self.all_solutions.len());
        println!("ALGORITHM: Sequential tile placement with solution branching and pruning");

        println!("\nSTEP_1_INIT: Creating initial solution set");
        println!("STEP_1_CODE: List<Solution> arrayList = new ArrayList<>()");
        let mut solutions = Vec::new();
        println!("STEP_1_CODE: arrayList.add(new Solution(this.stockSolution))");
        
        // Java: arrayList.add(new Solution(this.stockSolution));
        let initial_solution = Solution::from_stock_solution(stock_solution);
        solutions.push(initial_solution);
        
        println!("STEP_1_RESULT: Created {} initial solutions", solutions.len());
        if !solutions.is_empty() {
            println!("STEP_1_DETAIL: First solution has {} mosaics", solutions[0].get_mosaics().len());
        }

        if let Some(ref task) = self.task {
            if task.is_running() {
                println!("\nSTEP_2_TASK_CHECK: task.isRunning()=true, processing {} tiles", self.tiles.len());
                println!("STEP_2_ALGORITHM: For each tile, try to place it in all existing solutions");

                for (i, tile_dimensions) in self.tiles.iter().enumerate() {
                    let tile_index = i + 1;
                    println!("\n{}", "=".repeat(60));
                    println!("TILE_PLACEMENT_{}_START: Processing tile {}/{}", tile_index, tile_index, self.tiles.len());
                    println!("TILE_{}_INPUT: size={}x{}, id={}, material={}, isSquare={}", 
                             tile_index, tile_dimensions.width, tile_dimensions.height, 
                             tile_dimensions.id, tile_dimensions.material, tile_dimensions.is_square());
                    println!("TILE_{}_SOLUTIONS_BEFORE: {} solutions to try", tile_index, solutions.len());

                    if i % 3 == 0 {
                        self.percentage_done = ((i as f32 / self.tiles.len() as f32) * 100.0) as i32;
                    }

                    // Java: ArrayList<Solution> newSolutions = new ArrayList();
                    let mut new_solutions: Vec<Solution> = Vec::new();
                    // Java: Iterator<Solution> solutionIterator = arrayList.iterator();
                    let mut solutions_to_remove = Vec::new();
                    // Java: boolean tileWasPlaced = false;
                    let mut tile_was_placed = false;
                    
                    // Collect solutions that need modification
                    let mut modified_solutions = Vec::new();
                    
                    // Java: while (solutionIterator.hasNext())
                    for (solution_idx, current_solution) in solutions.iter().enumerate() {
                        // Java: Solution next2 = solutionIterator.next();
                        // Clone the solution so we can mutate it during processing
                        let mut working_solution = current_solution.clone();
                        
                        // Java: ListIterator<Mosaic> listIterator = next2.getMosaics().listIterator();
                        let mut mosaic_idx = 0;
                        let mut can_fit_tile = false;
                        
                        // Java: Mosaic next3 = listIterator.next();
                        // Java: while (true)
                        loop {
                            let mut next3_opt: Option<&Mosaic> = None;
                            
                            // Try to get current mosaic from existing mosaics first
                            if mosaic_idx < working_solution.get_mosaics().len() {
                                next3_opt = Some(&working_solution.get_mosaics()[mosaic_idx]);
                                mosaic_idx += 1;
                            }
                            
                            // Java: if (next3 == null) { canFitTile = true; break; }
                            if next3_opt.is_none() {
                                can_fit_tile = true;
                                break;
                            }
                            
                            let next3 = next3_opt.unwrap();
                            
                            // Java: if (next3.getMaterial() != null && !next3.getMaterial().equals(tileDimensions.getMaterial()))
                            if let Some(ref mosaic_material) = next3.material {
                                if mosaic_material != &tile_dimensions.material {
                                    // Material mismatch - continue to next mosaic
                                    continue;
                                }
                            }
                            
                            // Java: List<Mosaic> arrayList3 = new ArrayList<>();
                            let mut arrayList3 = Vec::new();
                            // Java: add(tileDimensions, next3, arrayList3);
                            self.add_tile(tile_dimensions, next3, &mut arrayList3);
                            
                            // Java: for (Mosaic mosaic2 : arrayList3) {
                            for mosaic2 in arrayList3.iter() {
                                // Java: Solution solution = new Solution(next2, next3);
                                let solution = Solution::from_solution_excluding_mosaic(&working_solution, next3);
                                let mut solution = solution;
                                // Java: solution.addMosaic(mosaic2);
                                solution.add_mosaic(mosaic2.clone());
                                // Java: newSolutions.add(solution);
                                new_solutions.push(solution);
                            }
                            
                            // Java: if (arrayList3.size() > 0) {
                            if !arrayList3.is_empty() {
                                can_fit_tile = true;
                                tile_was_placed = true;
                                break; // Java: break from while (true)
                            }
                            
                            // THE MISSING CRITICAL LOGIC FROM JAVA (lines 342-373)
                            // Java: if (listIterator.hasNext()) { currentMosaic = listIterator.next(); next3 = currentMosaic; }
                            if mosaic_idx < working_solution.get_mosaics().len() {
                                // Continue to next mosaic - the loop will handle this
                                println!("Переходим к следующей мозаике в решении");
                                continue;
                            } else {
                                // Java: else { Iterator<TileDimensions> it2 = next2.getUnusedStockPanels().iterator(); ... }
                                println!("Мозаики закончились, ищем новый лист");
                                let mut current_tile: Option<crate::features::input::models::tile_dimensions::TileDimensions> = None;
                                
                                // Java: while (true) { if (it2.hasNext()) { currentTile = it2.next(); if (currentTile.fits(tileDimensions)) { break; } } else { currentTile = null; break; } }
                                for unused_stock in working_solution.get_unused_stock_panels() {
                                    println!("Проверяем неиспользованный лист: {}x{}", unused_stock.width, unused_stock.height);
                                    if unused_stock.fits(tile_dimensions) {
                                        println!("Лист подходит для панели");
                                        current_tile = Some(unused_stock.clone());
                                        break;
                                    } else {
                                        println!("Лист НЕ подходит для панели");
                                    }
                                }
                                
                                // Java: if (currentTile != null) { next2.getUnusedStockPanels().remove(currentTile); currentMosaic = new Mosaic(currentTile); listIterator.add(currentMosaic); next3 = currentMosaic; }
                                if let Some(ref suitable_stock) = current_tile {
                                    // Remove from unused stock panels - use Vec instead of LinkedList retain
                                    let mut unused_panels: Vec<_> = working_solution.get_unused_stock_panels().iter().cloned().collect();
                                    unused_panels.retain(|panel| !std::ptr::eq(panel as *const _, suitable_stock as *const _));
                                    working_solution.unused_stock_panels = unused_panels.into_iter().collect();
                                    
                                    // Java: currentMosaic = new Mosaic(currentTile);
                                    let current_mosaic = crate::features::engine::model::calculation_response::Mosaic::from_tile_dimensions(suitable_stock);
                                    
                                    // Java: listIterator.add(currentMosaic);
                                    working_solution.get_mosaics_mut().push(current_mosaic);
                                    
                                    // Java: next3 = currentMosaic; (continue loop with new mosaic)
                                    // The loop will continue and process this new mosaic
                                } else {
                                    // Java: next3 = null; (no suitable stock found, exit loop)
                                    break;
                                }
                            }
                        }
                        
                        // Java: if (tileWasPlaced == canFitTile) { solutionIterator.remove(); } else { next2.getNoFitPanels().add(tileDimensions); }
                        if tile_was_placed == can_fit_tile {
                            solutions_to_remove.push(solution_idx);
                        } else {
                            // Java: next2.getNoFitPanels().add(tileDimensions);
                            // Add tile to no-fit panels and keep the solution
                            working_solution.get_no_fit_panels_mut().push(tile_dimensions.clone());
                            // Store the modified solution
                            modified_solutions.push((solution_idx, working_solution));
                        }
                    }
                    
                    // Apply modifications to solutions
                    for (idx, modified_solution) in modified_solutions {
                        solutions[idx] = modified_solution;
                    }
                    
                    // Java: for (Solution solution2 : newSolutions) { solution2.setCreatorThreadGroup(this.group); solution2.setAuxInfo(this.auxInfo); }
                    for new_solution in &mut new_solutions {
                        new_solution.set_creator_thread_group(self.group.clone());
                        new_solution.set_aux_info(self.aux_info.clone());
                    }
                    
                    // Java: arrayList.addAll(newSolutions);
                    solutions.extend(new_solutions);
                    
                    // Remove solutions that were marked for removal (in reverse order to maintain indices)
                    for &solution_idx in solutions_to_remove.iter().rev() {
                        solutions.remove(solution_idx);
                    }
                    

                    // Java: removeDuplicated(arrayList);
                    let removed = self.remove_duplicated(&mut solutions);

                    
                    // Java: sort(arrayList, this.threadPrioritizedComparators);
                    // Using the same sorting logic as the existing method
                    self.sort_solutions(&mut solutions);
                    
                    // Java: arrayList4.addAll(arrayList.subList(Math.min(arrayList.size() - 1, this.accuracyFactor), arrayList.size() - 1));
                    // Java: arrayList.removeAll(arrayList4);
                    if solutions.len() > self.accuracy_factor as usize {
                        let before_prune = solutions.len();
                        solutions.truncate(self.accuracy_factor as usize);
                    
                    }
                }
                
                // Java: this.allSolutions.addAll(arrayList);
                self.all_solutions.extend(solutions);
                
                // Java: sort(this.allSolutions, this.finalSolutionPrioritizedComparators);
                let mut all_solutions = std::mem::take(&mut self.all_solutions);
                self.sort_solutions(&mut all_solutions);
                self.all_solutions = all_solutions;
                
                // Java: arrayList5.addAll(list.subList(Math.min(list.size() - 1, this.accuracyFactor), this.allSolutions.size() - 1));
                // Java: this.allSolutions.removeAll(arrayList5);
                if self.all_solutions.len() > self.accuracy_factor as usize {
                    self.all_solutions.truncate(self.accuracy_factor as usize);
                }
                
                // Note: Thread group rankings are incremented in the optimizer service after thread completion
                // This matches the Java pattern where the service handles task updates
                
                // Java lines 405-410: Iterator<Mosaic> it3 = this.allSolutions.get(0).getMosaics().iterator();
                // while (it3.hasNext()) { if (it3.next().getUsedArea() == 0) { it3.remove(); } }
                if !self.all_solutions.is_empty() {
                    for mosaic in self.all_solutions[0].get_mosaics_mut() {
                        // Remove mosaics with no used area - simplified for now
                    }
                }
            }
        }
        
        Ok(())
    }

    fn add_tile(&self, tile_dimensions: &TileDimensions, mosaic: &Mosaic, placement_options: &mut Vec<Mosaic>) {
        println!("    ADD_METHOD_START: tile={}x{}, mosaic.orientation={}, tile.orientation={}, considerGrain={}", 
                 tile_dimensions.width, tile_dimensions.height, 
                 0, // mosaic orientation simplified
                 tile_dimensions.orientation.to_numeric(), self.consider_grain_direction);
        
        if !self.consider_grain_direction || tile_dimensions.orientation == Orientation::Default {
            println!("    ADD_BRANCH_1: No grain direction constraint, trying both orientations");
            println!("    ADD_FIT_1: Trying original orientation {}x{}", tile_dimensions.width, tile_dimensions.height);
            self.fit_tile(tile_dimensions, mosaic, placement_options, self.cut_thickness);
            
            if tile_dimensions.is_square() {
                println!("    ADD_SQUARE: Tile is square, no need to rotate");
                return;
            }
            println!("    ADD_FIT_2: Trying rotated orientation {}x{}", tile_dimensions.height, tile_dimensions.width);
            let rotated_tile = tile_dimensions.rotate_90();
            self.fit_tile(&rotated_tile, mosaic, placement_options, self.cut_thickness);
        } else {
            println!("    ADD_BRANCH_2: Grain direction constraint active");
            let tile_to_use = if Orientation::Default != tile_dimensions.orientation { // Simplified grain logic
                println!("    ADD_ROTATE: Orientations differ, rotating tile");
                tile_dimensions.rotate_90()
            } else {
                println!("    ADD_NO_ROTATE: Orientations match, using original tile");
                tile_dimensions.clone()
            };
            println!("    ADD_FIT_GRAIN: Fitting with grain constraint");
            self.fit_tile(&tile_to_use, mosaic, placement_options, self.cut_thickness);
        }
    }

    fn fit_tile(&self, tile_dimensions: &TileDimensions, mosaic: &Mosaic, placement_options: &mut Vec<Mosaic>, cut_thickness: i32) {}

    fn find_candidates(&self, tile_width: i32, tile_height: i32, tile_node: &TileNode, candidates: &mut Vec<TileNode>) {
        // Java: if (tileNode == null || tileNode.isFinal() || tileNode.getWidth() < i || tileNode.getHeight() < i2)
        if tile_node.is_final || tile_node.get_width() < tile_width || tile_node.get_height() < tile_height {
            return;
        }
        
        // Java: if (tileNode.getChild1() == null && tileNode.getChild2() == null)
        if tile_node.child1.is_none() && tile_node.child2.is_none() {
            // Java: boolean tileWasPlaced = false; if (tileNode.getWidth() == i || tileNode.getWidth() >= this.minTrimDimension + i)
            let width_fits = if tile_node.get_width() == tile_width {
                true
            } else if tile_node.get_width() >= self.min_trim_dimension + tile_width {
                true
            } else {
                if tile_node.get_width() > tile_width {
                    // Java: this.task.setMinTrimDimensionInfluenced(true);
                    // For now skip this
                }
                false
            };
            
            // Java: if (tileNode.getHeight() == i2 || tileNode.getHeight() >= this.minTrimDimension + i2)
            let height_fits = if tile_node.get_height() == tile_height {
                true
            } else if tile_node.get_height() >= self.min_trim_dimension + tile_height {
                true
            } else {
                if tile_node.get_height() > tile_height {
                    // Java: this.task.setMinTrimDimensionInfluenced(true);
                    // For now skip this
                }
                false
            };
            
            // Java: if (canFitTile && tileWasPlaced) { list.add(tileNode); return; }
            if width_fits && height_fits {
                candidates.push(tile_node.clone());
                return;
            }
            return;
        }
        
        // Java: if (tileNode.getChild1() != null) { findCandidates(i, i2, tileNode.getChild1(), list); }
        if let Some(ref child1) = tile_node.child1 {
            self.find_candidates(tile_width, tile_height, child1, candidates);
        }
        
        // Java: if (tileNode.getChild2() != null) { findCandidates(i, i2, tileNode.getChild2(), list); }
        if let Some(ref child2) = tile_node.child2 {
            self.find_candidates(tile_width, tile_height, child2, candidates);
        }
    }

    fn copy_tile_node(&self, source: &TileNode, target: &TileNode) -> TileNode {
        // Java: TileNode tileNode3 = new TileNode(tileNode);
        let mut root_copy = TileNode::copy_node(source); // Use Java-style copy constructor
        // Java: copyChildren(tileNode, tileNode3, tileNode2);
        self.copy_children(source, &mut root_copy, target);
        root_copy
    }
    
    fn copy_children(&self, source: &TileNode, dest: &mut TileNode, target: &TileNode) {
        // Java: if (tileNode == tileNode3) { return; }
        if source.id == target.id {
            return;
        }
        
        // Java: if (tileNode.getChild1() != null) { tileNode2.setChild1(new TileNode(tileNode.getChild1())); copyChildren(...); }
        if let Some(ref source_child1) = source.child1 {
            let mut child1_copy = source_child1.as_ref().clone();
            self.copy_children(source_child1, &mut child1_copy, target);
            dest.set_child1(Some(Box::new(child1_copy)));
        }
        
        // Java: if (tileNode.getChild2() != null) { tileNode2.setChild2(new TileNode(tileNode.getChild2())); copyChildren(...); }
        if let Some(ref source_child2) = source.child2 {
            let mut child2_copy = source_child2.as_ref().clone();
            self.copy_children(source_child2, &mut child2_copy, target);
            dest.set_child2(Some(Box::new(child2_copy)));
        }
    }

    fn split_hv(&self, tile_node: &mut TileNode, tile_dimensions: &TileDimensions, cut_thickness: i32) -> Vec<Cut> {
        let mut cuts = Vec::new();
        
        // Java: if (tileNode.getWidth() > tileDimensions.getWidth())
        if tile_node.get_width() > tile_dimensions.width as i32 {
            // Java: arrayList.add(splitHorizontally(tileNode, tileDimensions.getWidth(), i));
            if let Some(cut) = self.split_horizontally(tile_node, tile_dimensions.width as i32, cut_thickness, None) {
                cuts.push(cut);
            }
            
            // Java: if (tileNode.getHeight() > tileDimensions.getHeight())
            if tile_node.get_height() > tile_dimensions.height as i32 {
                // Java: arrayList.add(splitVertically(tileNode.getChild1(), tileDimensions.getHeight(), i, tileDimensions.getId()));
                if let Some(ref mut child1) = tile_node.child1 {
                    if let Some(cut) = self.split_vertically(child1, tile_dimensions.height as i32, cut_thickness, Some(tile_dimensions.id)) {
                        cuts.push(cut);
                    }
                    // Java: tileNode.getChild1().getChild1().setFinal(true);
                    if let Some(ref mut child1_child1) = child1.child1 {
                        child1_child1.set_final_tile(true);
                        child1_child1.set_rotated(tile_dimensions.is_rotated);
                    }
                }
            } else {
                // Java: tileNode.getChild1().setFinal(true);
                if let Some(ref mut child1) = tile_node.child1 {
                    child1.set_final_tile(true);
                    child1.set_rotated(tile_dimensions.is_rotated);
                    child1.set_external_id(Some(tile_dimensions.id));
                }
            }
        } else {
            // Java: arrayList.add(splitVertically(tileNode, tileDimensions.getHeight(), i, tileDimensions.getId()));
            if let Some(cut) = self.split_vertically(tile_node, tile_dimensions.height as i32, cut_thickness, Some(tile_dimensions.id)) {
                cuts.push(cut);
            }
            // Java: tileNode.getChild1().setFinal(true);
            if let Some(ref mut child1) = tile_node.child1 {
                child1.set_final_tile(true);
                child1.set_rotated(tile_dimensions.is_rotated);
            }
        }
        
        cuts
    }

    fn split_vh(&self, tile_node: &mut TileNode, tile_dimensions: &TileDimensions, cut_thickness: i32) -> Vec<Cut> {
        let mut cuts = Vec::new();
        
        // Java: if (tileNode.getHeight() > tileDimensions.getHeight())
        if tile_node.get_height() > tile_dimensions.height as i32 {
            // Java: arrayList.add(splitVertically(tileNode, tileDimensions.getHeight(), i));
            if let Some(cut) = self.split_vertically(tile_node, tile_dimensions.height as i32, cut_thickness, None) {
                cuts.push(cut);
            }
            
            // Java: if (tileNode.getWidth() > tileDimensions.getWidth())
            if tile_node.get_width() > tile_dimensions.width as i32 {
                // Java: arrayList.add(splitHorizontally(tileNode.getChild1(), tileDimensions.getWidth(), i, tileDimensions.getId()));
                if let Some(ref mut child1) = tile_node.child1 {
                    if let Some(cut) = self.split_horizontally(child1, tile_dimensions.width as i32, cut_thickness, Some(tile_dimensions.id)) {
                        cuts.push(cut);
                    }
                    // Java: tileNode.getChild1().getChild1().setFinal(true);
                    if let Some(ref mut child1_child1) = child1.child1 {
                        child1_child1.set_final_tile(true);
                        child1_child1.set_rotated(tile_dimensions.is_rotated);
                    }
                }
            } else {
                // Java: tileNode.getChild1().setFinal(true);
                if let Some(ref mut child1) = tile_node.child1 {
                    child1.set_final_tile(true);
                    child1.set_rotated(tile_dimensions.is_rotated);
                    child1.set_external_id(Some(tile_dimensions.id));
                }
            }
        } else {
            // Java: arrayList.add(splitHorizontally(tileNode, tileDimensions.getWidth(), i, tileDimensions.getId()));
            if let Some(cut) = self.split_horizontally(tile_node, tile_dimensions.width as i32, cut_thickness, Some(tile_dimensions.id)) {
                cuts.push(cut);
            }
            // Java: tileNode.getChild1().setFinal(true);
            if let Some(ref mut child1) = tile_node.child1 {
                child1.set_final_tile(true);
                child1.set_rotated(tile_dimensions.is_rotated);
            }
        }
        
        cuts
    }

    fn split_horizontally(&self, tile_node: &mut TileNode, width: i32, cut_thickness: i32, external_id: Option<u32>) -> Option<Cut> {
        // Java: if (tileNode == null) return null;
        let original_width = tile_node.get_width();
        let original_height = tile_node.get_height();
        
        // Java: TileNode tileNode2 = new TileNode(tileNode.getX1(), tileNode.getX1() + i, tileNode.getY1(), tileNode.getY2());
        let mut child1 = TileNode::new(tile_node.x1, tile_node.x1 + width, tile_node.y1, tile_node.y2);
        if let Some(id) = external_id {
            child1.set_external_id(Some(id));
        }
        
        // Java: if (tileNode2.getArea() > 0) tileNode.setChild1(tileNode2);
        if child1.get_area() > 0 {
            let child1_id = child1.id;
            tile_node.set_child1(Some(Box::new(child1)));
            
            // Java: TileNode tileNode3 = new TileNode(tileNode.getX1() + i + i2, tileNode.getX2(), tileNode.getY1(), tileNode.getY2());
            let child2 = TileNode::new(tile_node.x1 + width + cut_thickness, tile_node.x2, tile_node.y1, tile_node.y2);
            
            // Java: if (tileNode3.getArea() > 0) tileNode.setChild2(tileNode3);
            if child2.get_area() > 0 {
                let child2_id = child2.id;
                tile_node.set_child2(Some(Box::new(child2)));
                
                // Java: return new Cut.Builder()...
                return Some(Cut {
                    x1: (tile_node.x1 + width) as f64,
                    y1: tile_node.y1 as f64,
                    x2: (tile_node.x1 + width) as f64,
                    y2: tile_node.y2 as f64,
                    cut_coord: width as f64,
                    is_horizontal: true,
                    original_tile_id: tile_node.id as i32,
                    original_width: original_width as f64,
                    original_height: original_height as f64,
                    child1_tile_id: child1_id as i32,
                    child2_tile_id: child2_id as i32,
                });
            }
        }
        
        None
    }

    fn split_vertically(&self, tile_node: &mut TileNode, height: i32, cut_thickness: i32, external_id: Option<u32>) -> Option<Cut> {
        // Java: if (tileNode == null) return null;
        let original_width = tile_node.get_width();
        let original_height = tile_node.get_height();
        
        // Java: TileNode tileNode2 = new TileNode(tileNode.getX1(), tileNode.getX2(), tileNode.getY1(), tileNode.getY1() + i);
        let mut child1 = TileNode::new(tile_node.x1, tile_node.x2, tile_node.y1, tile_node.y1 + height);
        if let Some(id) = external_id {
            child1.set_external_id(Some(id));
        }
        
        // Java: if (tileNode2.getArea() > 0) tileNode.setChild1(tileNode2);
        if child1.get_area() > 0 {
            let child1_id = child1.id;
            tile_node.set_child1(Some(Box::new(child1)));
            
            // Java: TileNode tileNode3 = new TileNode(tileNode.getX1(), tileNode.getX2(), tileNode.getY1() + i + i2, tileNode.getY2());
            let child2 = TileNode::new(tile_node.x1, tile_node.x2, tile_node.y1 + height + cut_thickness, tile_node.y2);
            
            // Java: if (tileNode3.getArea() > 0) tileNode.setChild2(tileNode3);
            if child2.get_area() > 0 {
                let child2_id = child2.id;
                tile_node.set_child2(Some(Box::new(child2)));
                
                // Java: return new Cut.Builder()...
                return Some(Cut {
                    x1: tile_node.x1 as f64,
                    y1: (tile_node.y1 + height) as f64,
                    x2: tile_node.x2 as f64,
                    y2: (tile_node.y1 + height) as f64,
                    cut_coord: height as f64,
                    is_horizontal: false,
                    original_tile_id: tile_node.id as i32,
                    original_width: original_width as f64,
                    original_height: original_height as f64,
                    child1_tile_id: child1_id as i32,
                    child2_tile_id: child2_id as i32,
                });
            }
        }
        
        None
    }
    
    /// Sort solutions using the same comparators as Java
    fn sort_solutions(&self, solutions: &mut Vec<Solution>) {
        
        // Java: использует threadPrioritizedComparators или finalSolutionPrioritizedComparators
        // Для примера используем optimization_priority = 0 (AREA приоритет)
        let priorities = PriorityListFactory::get_final_solution_prioritized_comparator_list(0);
        let comparator = SolutionComparator::new(priorities);
        
        solutions.sort_by(|a, b| {
            let result = comparator.compare(a, b);
            result
        });
        
    }
}