use serde::{Deserialize, Serialize};

use crate::features::engine::cut_list_thread::CutListThread;
use crate::features::engine::model::calculation_request::CalculationRequest;
use crate::features::engine::model::calculation_response::{self, CalculationResponse};
use crate::features::engine::model::client_info::ClientInfo;
use crate::features::engine::model::solution::Solution;
use crate::features::engine::model::task::Task;
use crate::features::engine::model::tile_node::TileNode;
use crate::features::engine::model::{
    calculation_response::Mosaic, status::Status, stock_solution::StockSolution,
};
use crate::features::input::models::tile_dimensions::TileDimensions;
use std::collections::{HashMap, LinkedList};
use std::sync::atomic::{AtomicI32, Ordering};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationResponseBuilder {
    pub task: Task,
    pub calculation_request: CalculationRequest,
    pub solutions: HashMap<String, Vec<Solution>>,
    pub no_stock_material_panels: Vec<TileDimensions>,
}

impl CalculationResponseBuilder {
    // -= CalculationResponse build
    pub fn build(&self) -> CalculationResponse {
        let mut calculation_response = CalculationResponse::new();

        let panels = &self.calculation_request.panels;
        let stock_panels = &self.calculation_request.stock_panels;

        let mut solution = Solution::default();
        let mut solution_ids = Vec::new();
        let mut timestamp = 0;

        // Iterate through solutions map and aggregate data
        for (material, solutions) in &self.solutions {
            if let Some(first_solution) = solutions.first() {
                solution_ids.push(first_solution.id);
                solution.add_all_mosaics(first_solution.mosaics.clone());
                // Add all mosaics from first solution
                solution.add_all_no_fit_panels(first_solution.no_fit_panels.clone());

                // Track latest timestamp
                if first_solution.timestamp > timestamp {
                    timestamp = first_solution.timestamp;
                }
            }
        }

        // Handle materials with no solutions - add their tiles as no-fit
        if !solution.mosaics.is_empty() {
            for (material, solutions) in &self.solutions {
                if solutions.is_empty() {
                    if let Some(tile_dimensions_list) =
                        self.task.tile_dimensions_per_material.get(material)
                    {
                        for tile_dimensions in tile_dimensions_list {
                            self.add_no_fit_tile(&mut calculation_response, tile_dimensions);
                        }
                    }
                }
            }
        }

        // -= No Stock Material Panels =-
        if !self.no_stock_material_panels.is_empty() {
            solution.add_all_no_fit_panels(self.no_stock_material_panels.clone());
        }

        // Set basic response fields
        calculation_response.id = Some(format!(
            "{}",
            solution_ids.iter().fold(0, |acc, &x| acc ^ x)
        ));
        calculation_response.solution_elapsed_time = if timestamp > 0 {
            Some(timestamp - self.task.start_time)
        } else {
            None
        };
        calculation_response.request = self.calculation_request.clone();

        // -= Mosaics =-
        for mosaic in &solution.mosaics {
{
            let mut response_mosaic = Mosaic::new();

            if let Some(root_node) = mosaic.root_tile_node.first() {
                response_mosaic.request_stock_id = Some(root_node.external_id.unwrap_or(0) as i32);
                response_mosaic.used_area =
                    root_node.get_used_area() as f64 / (self.task.factor * self.task.factor) as f64;
                response_mosaic.used_area_ratio = if root_node.get_area() > 0 {
                    root_node.get_used_area() as f32 / root_node.get_area() as f32
                } else {
                    0.0
                };
                response_mosaic.nbr_final_panels = root_node.get_nbr_final_tiles();
                response_mosaic.nbr_wasted_panels = root_node.get_nbr_final_tiles(); // TODO: implement getNbrUnusedTiles
                response_mosaic.wasted_area =
                    mosaic.get_unused_area() as f64 / (self.task.factor * self.task.factor) as f64;
                response_mosaic.material = mosaic.material.clone();

                // Add children to tiles list
                // self.add_children_to_list(root_node, &mut response_mosaic.panels);
            }

            // Calculate cut length
            let cut_length: i64 = mosaic
                .cuts
                .iter()
                .map(|cut| {
                    if cut.is_horizontal {
                        ((cut.x2 - cut.x1) as f64 * self.task.factor as f64) as i64
                    } else {
                        ((cut.y2 - cut.y1) as f64 * self.task.factor as f64) as i64
                    }
                })
                .sum();
            response_mosaic.cut_length = cut_length as f64 / self.task.factor as f64;

            // TODO: Add edge band calculation
            // response_mosaic.edge_bands = EdgeBanding::calc_edge_bands(&final_tile_nodes, panels, self.task.factor);

            // Set panel labels
            for panel in panels {
                for tile in &mut response_mosaic.panels {
                    if tile.request_obj_id as u32 == panel.id {
                        tile.label = Some(panel.label.clone());
                    }
                }
            }

            // Set stock panel labels
            for stock_panel in stock_panels {
                if let Some(request_stock_id) = response_mosaic.request_stock_id {
                    if request_stock_id as u32 == stock_panel.id {
                        response_mosaic.stock_label = Some(stock_panel.label.clone());
                    }
                }
            }

            // Create final panels map
            let _final_panels_map: std::collections::HashMap<String, String> = std::collections::HashMap::new();
            // TODO: Implement final tile nodes collection and processing

            // Add cuts to response mosaic
            for cut in &mosaic.cuts {
                let response_cut = calculation_response::Cut {
                    x1: cut.x1 / self.task.factor as f64,
                    y1: cut.y1 / self.task.factor as f64,
                    x2: cut.x2 / self.task.factor as f64,
                    y2: cut.y2 / self.task.factor as f64,
                    cut_coord: cut.cut_coord / self.task.factor as f64,
                    is_horizontal: cut.is_horizontal,
                    original_tile_id: cut.original_tile_id,
                    original_width: cut.original_width / self.task.factor as f64,
                    original_height: cut.original_height / self.task.factor as f64,
                    child1_tile_id: cut.child1_tile_id,
                    child2_tile_id: cut.child2_tile_id,
                };
                response_mosaic.cuts.push(response_cut);
            }

            calculation_response.mosaics.push(response_mosaic);
        }


        }

        // Add no-fit panels from solution
        for no_fit_panel in &solution.no_fit_panels {
            self.add_no_fit_tile(&mut calculation_response, no_fit_panel);
        }

        // Calculate totals
        let mut total_used_area = 0.0;
        let mut total_wasted_area = 0.0;
        let mut total_cut_length = 0.0;
        let mut total_nbr_cuts = 0;

        for mosaic in &calculation_response.mosaics {
            total_used_area += mosaic.used_area;
            total_wasted_area += mosaic.wasted_area;
            total_nbr_cuts += mosaic.cuts.len() as i64;
            total_cut_length += mosaic.cut_length;
        }

        calculation_response.task_id = Some(self.task.id.clone());
        calculation_response.total_used_area = total_used_area;
        calculation_response.total_wasted_area = total_wasted_area;
        calculation_response.total_used_area_ratio = if (total_wasted_area + total_used_area) > 0.0
        {
            total_used_area / (total_wasted_area + total_used_area)
        } else {
            0.0
        };
        calculation_response.total_nbr_cuts = total_nbr_cuts;
        calculation_response.total_cut_length = total_cut_length;
        // calculation_response.elapsed_time = self.task.elapsed_time;


        calculation_response
    }

    fn add_no_fit_tile(
        &self,
        calculation_response: &mut CalculationResponse,
        tile_dimensions: &TileDimensions,
    ) {
        // Check if tile already exists and increment count
        for no_fit_tile in &mut calculation_response.no_fit_panels {
            if no_fit_tile.id == tile_dimensions.id {
                no_fit_tile.count += 1;
                return;
            }
        }

        // Create new no-fit tile
        let mut no_fit_tile = calculation_response::NoFitTile::new();
        no_fit_tile.id = tile_dimensions.id;
        no_fit_tile.width = (tile_dimensions.width / self.task.factor) as f64;
        no_fit_tile.height = (tile_dimensions.height / self.task.factor) as f64;
        no_fit_tile.count = 1;

        // Set label and material from calculation request panels
        for panel in &self.calculation_request.panels {
            if no_fit_tile.id == panel.id {
                no_fit_tile.label = Some(panel.label.clone());
                no_fit_tile.material = Some(panel.material.clone());
                break;
            }
        }

        calculation_response.no_fit_panels.push(no_fit_tile);
    }

    fn add_children_to_list(
        &self,
        tile_node: &TileNode,
        tiles: &mut Vec<calculation_response::Tile>,
    ) {
        let mut tile = calculation_response::Tile::new();

        tile.id = tile_node.id as i32;
        // tile.x = tile_node.x1 as f64 / self.task.factor;
        // tile.y = tile_node.y1 as f64 / self.task.factor;
        // tile.width = tile_node.get_width() as f64 / self.task.factor;
        // tile.height = tile_node.get_height() as f64 / self.task.factor;
        tile.is_final = tile_node.is_final;
        tile.is_rotated = tile_node.is_rotated;

        if let Some(external_id) = tile_node.external_id {
            tile.request_obj_id = Some(external_id as i32);
        }

        if tile_node.child1.is_some() || tile_node.child2.is_some() {
            tile.has_children = true;
            tiles.push(tile);

            if let Some(ref child1) = tile_node.child1 {
                self.add_children_to_list(child1, tiles);
            }
            if let Some(ref child2) = tile_node.child2 {
                self.add_children_to_list(child2, tiles);
            }
        } else {
            tile.has_children = false;
            tiles.push(tile);
        }
    }
}
