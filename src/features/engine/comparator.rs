
use std::cmp::Ordering;

use crate::features::engine::model::solution::Solution;

#[derive(Debug, Clone)]
pub enum OptimizationPriority {
    MostTiles,
    LeastWastedArea,
    LeastNbrCuts,
    MostHvDiscrepancy,
    BiggestUnusedTileArea,
    SmallestCenterOfMassDistToOrigin,
    LeastNbrMosaics,
    LeastNbrUnusedTiles,
    MostUnusedPanelArea,
}

impl OptimizationPriority {
    pub fn to_string(&self) -> &'static str {
        match self {
            OptimizationPriority::MostTiles => "MOST_TILES",
            OptimizationPriority::LeastWastedArea => "LEAST_WASTED_AREA",
            OptimizationPriority::LeastNbrCuts => "LEAST_NBR_CUTS",
            OptimizationPriority::MostHvDiscrepancy => "MOST_HV_DISCREPANCY",
            OptimizationPriority::BiggestUnusedTileArea => "BIGGEST_UNUSED_TILE_AREA",

            OptimizationPriority::SmallestCenterOfMassDistToOrigin => {
                "SMALLEST_CENTER_OF_MASS_DIST_TO_ORIGIN"
            }
            OptimizationPriority::LeastNbrMosaics => "LEAST_NBR_MOSAICS",
            OptimizationPriority::LeastNbrUnusedTiles => "LEAST_NBR_UNUSED_TILES",
            OptimizationPriority::MostUnusedPanelArea => "MOST_UNUSED_PANEL_AREA",
        }
    }
}

pub struct PriorityListFactory;

impl PriorityListFactory {
    /// Java: getFinalSolutionPrioritizedComparatorList
    pub fn get_final_solution_prioritized_comparator_list(
        optimization_priority: i32,
    ) -> Vec<OptimizationPriority> {
        let mut priorities = Vec::new();

        if optimization_priority == 0 {
            priorities.push(OptimizationPriority::MostTiles);
            priorities.push(OptimizationPriority::LeastWastedArea);
            priorities.push(OptimizationPriority::LeastNbrCuts);
        } else {
            priorities.push(OptimizationPriority::MostTiles);
            priorities.push(OptimizationPriority::LeastNbrCuts);
            priorities.push(OptimizationPriority::LeastWastedArea);
        }

        priorities.push(OptimizationPriority::LeastNbrMosaics);
        priorities.push(OptimizationPriority::BiggestUnusedTileArea);
        priorities.push(OptimizationPriority::MostHvDiscrepancy);

        priorities
    }
}

pub struct SolutionComparator {
    priorities: Vec<OptimizationPriority>,
}

impl SolutionComparator {
    pub fn new(priorities: Vec<OptimizationPriority>) -> Self {
        Self { priorities }
    }

    pub fn compare(&self, a: &Solution, b: &Solution) -> Ordering {
        for priority in &self.priorities {
            let result = match priority {
                OptimizationPriority::MostTiles => {
                    // Java: solution2.getNbrFinalTiles() - solution.getNbrFinalTiles()
                    let tiles_a = a.get_nbr_final_tiles();
                    let tiles_b = b.get_nbr_final_tiles();
                    tiles_b.cmp(&tiles_a) // descending (more tiles is better)
                }
                OptimizationPriority::LeastWastedArea => {
                    // Java: solution.getUnusedArea() - solution2.getUnusedArea()
                    let waste_a = a.get_unused_area();
                    let waste_b = b.get_unused_area();
                    waste_a.cmp(&waste_b) // ascending (less waste is better)
                }
                OptimizationPriority::LeastNbrCuts => {
                    // Java: solution.getNbrCuts() - solution2.getNbrCuts()
                    let cuts_a = a.get_nbr_cuts();
                    let cuts_b = b.get_nbr_cuts();
                    cuts_a.cmp(&cuts_b) // ascending (fewer cuts is better)
                }
                OptimizationPriority::LeastNbrMosaics => {
                    let mosaics_a = a.get_nbr_mosaics();
                    let mosaics_b = b.get_nbr_mosaics();
                    mosaics_a.cmp(&mosaics_b) // ascending (fewer mosaics is better)
                }
                OptimizationPriority::BiggestUnusedTileArea => {
                    // Java: solution2.getBiggestArea() - solution.getBiggestArea()
                    let biggest_a = a.get_biggest_area();
                    let biggest_b = b.get_biggest_area();
                    biggest_b.cmp(&biggest_a) // descending (bigger area is better)
                }
                OptimizationPriority::MostHvDiscrepancy => {
                    // Java: solution.getDistictTileSet() - solution2.getDistictTileSet()
                    let distinct_a = a.get_distict_tile_set();
                    let distinct_b = b.get_distict_tile_set();
                    distinct_a.cmp(&distinct_b) // ascending (smaller distinct tile set first)
                }
                _ => {
                    // Placeholder for other comparators
                    a.id.cmp(&b.id)
                }
            };

            if result != Ordering::Equal {
                return result;
            }
        }

        // Final tiebreaker
        a.id.cmp(&b.id)
    }
}
