//! Individual comparator functions for solution optimization
//!
//! This module provides specific comparison functions for different optimization
//! criteria. Each function compares two solutions based on a specific metric.

use crate::models::task::Solution;
use std::cmp::Ordering;

/// Compares solutions by the number of final tiles (most tiles first)
///
/// This comparator prioritizes solutions with more final tiles, which typically
/// indicates better utilization of the available material.
///
/// # Arguments
/// * `solution1` - First solution to compare
/// * `solution2` - Second solution to compare
///
/// # Returns
/// Ordering indicating which solution has more tiles
///
/// # Examples
/// ```
/// use rezalnyash_core::models::{task::Solution, solution_comparator::comparators::most_nbr_tiles_comparator};
/// use std::cmp::Ordering;
///
/// let solution1 = Solution::new("material1".to_string(), 0.8, 0.9);
/// let solution2 = Solution::new("material2".to_string(), 0.7, 0.8);
/// 
/// let result = most_nbr_tiles_comparator(&solution1, &solution2);
/// // Result depends on the number of tiles in each solution's response
/// ```
pub fn most_nbr_tiles_comparator(solution1: &Solution, solution2: &Solution) -> Ordering {
    let tiles1 = get_nbr_final_tiles(solution1);
    let tiles2 = get_nbr_final_tiles(solution2);
    
    // More tiles is better, so reverse the comparison
    tiles2.cmp(&tiles1)
}

/// Compares solutions by wasted area (least wasted area first)
///
/// This comparator prioritizes solutions with less wasted material area,
/// which indicates better material efficiency.
///
/// # Arguments
/// * `solution1` - First solution to compare
/// * `solution2` - Second solution to compare
///
/// # Returns
/// Ordering indicating which solution has less wasted area
pub fn least_wasted_area_comparator(solution1: &Solution, solution2: &Solution) -> Ordering {
    let unused_area1 = get_unused_area(solution1);
    let unused_area2 = get_unused_area(solution2);
    
    // Less wasted area is better
    unused_area1.partial_cmp(&unused_area2).unwrap_or(Ordering::Equal)
}

/// Compares solutions by the number of cuts (least cuts first)
///
/// This comparator prioritizes solutions with fewer cuts, which typically
/// reduces cutting time and complexity.
///
/// # Arguments
/// * `solution1` - First solution to compare
/// * `solution2` - Second solution to compare
///
/// # Returns
/// Ordering indicating which solution has fewer cuts
pub fn least_nbr_cuts_comparator(solution1: &Solution, solution2: &Solution) -> Ordering {
    let cuts1 = get_nbr_cuts(solution1);
    let cuts2 = get_nbr_cuts(solution2);
    
    // Fewer cuts is better
    cuts1.cmp(&cuts2)
}

/// Compares solutions by horizontal-vertical discrepancy (most discrepancy first)
///
/// This comparator prioritizes solutions with more distinct tile orientations,
/// which can indicate better space utilization.
///
/// # Arguments
/// * `solution1` - First solution to compare
/// * `solution2` - Second solution to compare
///
/// # Returns
/// Ordering indicating which solution has more HV discrepancy
pub fn most_hv_discrepancy_comparator(solution1: &Solution, solution2: &Solution) -> Ordering {
    let discrepancy1 = get_distinct_tile_set(solution1);
    let discrepancy2 = get_distinct_tile_set(solution2);
    
    // More discrepancy is better, so reverse the comparison
    discrepancy2.cmp(&discrepancy1)
}

/// Compares solutions by biggest unused tile area (biggest unused area first)
///
/// This comparator prioritizes solutions with larger unused areas, which
/// can be useful for future cutting operations.
///
/// # Arguments
/// * `solution1` - First solution to compare
/// * `solution2` - Second solution to compare
///
/// # Returns
/// Ordering indicating which solution has bigger unused tile area
pub fn biggest_unused_tile_area_comparator(solution1: &Solution, solution2: &Solution) -> Ordering {
    let biggest_area1 = get_biggest_area(solution1);
    let biggest_area2 = get_biggest_area(solution2);
    
    // Bigger unused area is better, so reverse the comparison
    biggest_area2.partial_cmp(&biggest_area1).unwrap_or(Ordering::Equal)
}

/// Compares solutions by center of mass distance to origin (smallest distance first)
///
/// This comparator prioritizes solutions where the center of mass of all tiles
/// is closer to the origin, which can indicate better balance.
///
/// # Arguments
/// * `solution1` - First solution to compare
/// * `solution2` - Second solution to compare
///
/// # Returns
/// Ordering indicating which solution has smaller center of mass distance
pub fn smallest_center_of_mass_dist_to_origin_comparator(
    solution1: &Solution,
    solution2: &Solution,
) -> Ordering {
    let distance1 = get_center_of_mass_distance_to_origin(solution1);
    let distance2 = get_center_of_mass_distance_to_origin(solution2);
    
    // Smaller distance is better
    distance1.partial_cmp(&distance2).unwrap_or(Ordering::Equal)
}

/// Compares solutions by the number of mosaics (least mosaics first)
///
/// This comparator prioritizes solutions with fewer mosaics, which typically
/// indicates simpler cutting patterns.
///
/// # Arguments
/// * `solution1` - First solution to compare
/// * `solution2` - Second solution to compare
///
/// # Returns
/// Ordering indicating which solution has fewer mosaics
pub fn least_nbr_mosaics_comparator(solution1: &Solution, solution2: &Solution) -> Ordering {
    let mosaics1 = get_nbr_mosaics(solution1);
    let mosaics2 = get_nbr_mosaics(solution2);
    
    // Fewer mosaics is better
    mosaics1.cmp(&mosaics2)
}

/// Compares solutions by the number of unused tiles (least unused tiles first)
///
/// This comparator prioritizes solutions with fewer unused tiles, which
/// indicates better material utilization.
///
/// # Arguments
/// * `solution1` - First solution to compare
/// * `solution2` - Second solution to compare
///
/// # Returns
/// Ordering indicating which solution has fewer unused tiles
pub fn least_nbr_unused_tiles_comparator(solution1: &Solution, solution2: &Solution) -> Ordering {
    let unused_tiles1 = get_nbr_unused_tiles(solution1);
    let unused_tiles2 = get_nbr_unused_tiles(solution2);
    
    // Fewer unused tiles is better
    unused_tiles1.cmp(&unused_tiles2)
}

/// Compares solutions by most unused panel area (most unused panel area first)
///
/// This comparator prioritizes solutions with more unused panel area, which
/// can be beneficial for certain optimization strategies.
///
/// # Arguments
/// * `solution1` - First solution to compare
/// * `solution2` - Second solution to compare
///
/// # Returns
/// Ordering indicating which solution has more unused panel area
pub fn most_unused_panel_area_comparator(solution1: &Solution, solution2: &Solution) -> Ordering {
    let unused_panel_area1 = get_most_unused_panel_area(solution1);
    let unused_panel_area2 = get_most_unused_panel_area(solution2);
    
    // More unused panel area is better, so reverse the comparison
    unused_panel_area2.partial_cmp(&unused_panel_area1).unwrap_or(Ordering::Equal)
}

// Helper functions to extract data from solutions

/// Gets the number of final tiles from a solution
fn get_nbr_final_tiles(solution: &Solution) -> i32 {
    solution
        .response
        .as_ref()
        .and_then(|response| response.panels.as_ref())
        .map(|panels| panels.len() as i32)
        .unwrap_or(0)
}

/// Gets the unused area from a solution
fn get_unused_area(solution: &Solution) -> f64 {
    solution
        .response
        .as_ref()
        .map(|response| response.total_wasted_area)
        .unwrap_or(0.0)
}

/// Gets the number of cuts from a solution
fn get_nbr_cuts(solution: &Solution) -> u64 {
    solution
        .response
        .as_ref()
        .map(|response| response.total_nbr_cuts)
        .unwrap_or(0)
}

/// Gets the distinct tile set count from a solution
fn get_distinct_tile_set(solution: &Solution) -> i32 {
    solution
        .response
        .as_ref()
        .and_then(|response| response.panels.as_ref())
        .map(|panels| {
            // Count unique tile dimensions
            let mut unique_dimensions = std::collections::HashSet::new();
            for panel in panels {
                unique_dimensions.insert((panel.width as i32, panel.height as i32));
            }
            unique_dimensions.len() as i32
        })
        .unwrap_or(0)
}

/// Gets the biggest unused area from a solution
fn get_biggest_area(solution: &Solution) -> f64 {
    solution
        .response
        .as_ref()
        .and_then(|response| response.panels.as_ref())
        .map(|panels| {
            // Find the largest panel area that's not fully utilized
            panels
                .iter()
                .map(|panel| panel.width * panel.height)
                .fold(0.0f64, |acc, area| acc.max(area))
        })
        .unwrap_or(0.0)
}

/// Gets the center of mass distance to origin from a solution
fn get_center_of_mass_distance_to_origin(solution: &Solution) -> f64 {
    solution
        .response
        .as_ref()
        .and_then(|response| response.panels.as_ref())
        .map(|panels| {
            if panels.is_empty() {
                return 0.0;
            }

            let mut total_area = 0.0f64;
            let mut weighted_x = 0.0f64;
            let mut weighted_y = 0.0f64;

            for panel in panels {
                let area = panel.width * panel.height;
                // Since FinalTile doesn't have x,y coordinates, we'll use width/height as approximation
                let center_x = panel.width / 2.0;
                let center_y = panel.height / 2.0;

                total_area += area;
                weighted_x += area * center_x;
                weighted_y += area * center_y;
            }

            if total_area == 0.0 {
                return 0.0;
            }

            let center_of_mass_x = weighted_x / total_area;
            let center_of_mass_y = weighted_y / total_area;

            // Calculate distance from origin
            (center_of_mass_x.powi(2) + center_of_mass_y.powi(2)).sqrt()
        })
        .unwrap_or(0.0)
}

/// Gets the number of mosaics from a solution
fn get_nbr_mosaics(solution: &Solution) -> usize {
    solution
        .response
        .as_ref()
        .map(|response| response.mosaics.len())
        .unwrap_or(0)
}

/// Gets the number of unused tiles from a solution
fn get_nbr_unused_tiles(solution: &Solution) -> i32 {
    solution
        .response
        .as_ref()
        .map(|response| response.no_fit_panels.len() as i32)
        .unwrap_or(0)
}

/// Gets the most unused panel area from a solution
fn get_most_unused_panel_area(solution: &Solution) -> f64 {
    solution
        .response
        .as_ref()
        .map(|response| {
            // Calculate the largest single unused area
            // This could be the largest no-fit panel or the largest unused portion
            response
                .no_fit_panels
                .iter()
                .map(|panel| panel.width * panel.height)
                .fold(0.0f64, |acc, area| acc.max(area))
        })
        .unwrap_or(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{calculation_response::CalculationResponse, task::Solution};

    fn create_test_solution_with_response(
        material: &str,
        score: f64,
        efficiency: f64,
        nbr_panels: usize,
        wasted_area: f64,
        nbr_cuts: u64,
    ) -> Solution {
        let mut response = CalculationResponse::new();
        response.total_wasted_area = wasted_area;
        response.total_nbr_cuts = nbr_cuts;

        // Create test panels
        let mut panels = Vec::new();
        for i in 0..nbr_panels {
            let panel = crate::models::calculation_response::FinalTile::with_params(
                i as i32 + 1,
                100.0 + i as f64 * 10.0,
                200.0 + i as f64 * 5.0,
            );
            panels.push(panel);
        }
        response.panels = if panels.is_empty() { None } else { Some(panels) };

        Solution::with_response(material.to_string(), score, efficiency, response)
    }

    #[test]
    fn test_most_nbr_tiles_comparator() {
        let solution1 = create_test_solution_with_response("mat1", 0.8, 0.9, 3, 100.0, 5);
        let solution2 = create_test_solution_with_response("mat2", 0.7, 0.8, 2, 150.0, 4);

        let result = most_nbr_tiles_comparator(&solution1, &solution2);
        assert_eq!(result, Ordering::Less); // solution1 has more tiles, so it should be "less" (better)
    }

    #[test]
    fn test_least_wasted_area_comparator() {
        let solution1 = create_test_solution_with_response("mat1", 0.8, 0.9, 3, 100.0, 5);
        let solution2 = create_test_solution_with_response("mat2", 0.7, 0.8, 2, 150.0, 4);

        let result = least_wasted_area_comparator(&solution1, &solution2);
        assert_eq!(result, Ordering::Less); // solution1 has less wasted area
    }

    #[test]
    fn test_least_nbr_cuts_comparator() {
        let solution1 = create_test_solution_with_response("mat1", 0.8, 0.9, 3, 100.0, 5);
        let solution2 = create_test_solution_with_response("mat2", 0.7, 0.8, 2, 150.0, 4);

        let result = least_nbr_cuts_comparator(&solution1, &solution2);
        assert_eq!(result, Ordering::Greater); // solution1 has more cuts
    }

    #[test]
    fn test_center_of_mass_distance_calculation() {
        let solution = create_test_solution_with_response("mat1", 0.8, 0.9, 2, 100.0, 5);
        let distance = get_center_of_mass_distance_to_origin(&solution);
        assert!(distance > 0.0); // Should have some distance from origin
    }

    #[test]
    fn test_helper_functions_with_empty_response() {
        let solution = Solution::new("material".to_string(), 0.8, 0.9);

        assert_eq!(get_nbr_final_tiles(&solution), 0);
        assert_eq!(get_unused_area(&solution), 0.0);
        assert_eq!(get_nbr_cuts(&solution), 0);
        assert_eq!(get_distinct_tile_set(&solution), 0);
        assert_eq!(get_biggest_area(&solution), 0.0);
        assert_eq!(get_center_of_mass_distance_to_origin(&solution), 0.0);
        assert_eq!(get_nbr_mosaics(&solution), 0);
        assert_eq!(get_nbr_unused_tiles(&solution), 0);
        assert_eq!(get_most_unused_panel_area(&solution), 0.0);
    }

    #[test]
    fn test_distinct_tile_set_calculation() {
        let solution = create_test_solution_with_response("mat1", 0.8, 0.9, 3, 100.0, 5);
        let distinct_count = get_distinct_tile_set(&solution);
        assert_eq!(distinct_count, 3); // Each panel has different dimensions
    }

    #[test]
    fn test_comparator_equality() {
        let solution1 = create_test_solution_with_response("mat1", 0.8, 0.9, 2, 100.0, 5);
        let solution2 = create_test_solution_with_response("mat2", 0.7, 0.8, 2, 100.0, 5);

        assert_eq!(most_nbr_tiles_comparator(&solution1, &solution2), Ordering::Equal);
        assert_eq!(least_wasted_area_comparator(&solution1, &solution2), Ordering::Equal);
        assert_eq!(least_nbr_cuts_comparator(&solution1, &solution2), Ordering::Equal);
    }
}
