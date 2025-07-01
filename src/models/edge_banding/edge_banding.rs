//! Edge banding calculations for cutting optimization
//! 
//! This module provides functionality to calculate edge banding requirements
//! based on tile nodes and panel specifications. This is a direct conversion
//! from the Java EdgeBanding utility class.

use crate::models::{TileNode, calculation_request::Panel};
use std::collections::HashMap;

/// Errors that can occur during edge banding calculations
#[derive(Debug, Clone, PartialEq)]
pub enum EdgeBandingError {
    /// Invalid scale factor (must be positive)
    InvalidScaleFactor(f64),
    /// Division by zero in calculations
    DivisionByZero,
    /// Panel not found for given ID
    PanelNotFound(i32),
    /// Invalid panel dimensions
    InvalidPanelDimensions { panel_id: i32, width: String, height: String },
}

impl std::fmt::Display for EdgeBandingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EdgeBandingError::InvalidScaleFactor(factor) => {
                write!(f, "Invalid scale factor: {} (must be positive)", factor)
            }
            EdgeBandingError::DivisionByZero => {
                write!(f, "Division by zero in edge banding calculations")
            }
            EdgeBandingError::PanelNotFound(id) => {
                write!(f, "Panel with ID {} not found in tile nodes", id)
            }
            EdgeBandingError::InvalidPanelDimensions { panel_id, width, height } => {
                write!(f, "Invalid dimensions for panel {}: width='{}', height='{}'", panel_id, width, height)
            }
        }
    }
}

impl std::error::Error for EdgeBandingError {}

/// Edge banding calculator - direct conversion from Java EdgeBanding utility class
/// 
/// This struct provides static-like methods for calculating edge banding requirements.
/// The original Java class had only static methods, so this Rust implementation
/// follows the same pattern using associated functions.
pub struct EdgeBanding;

impl EdgeBanding {
    /// Calculates edge banding requirements for given tile nodes and panels
    /// 
    /// This is a direct conversion of the Java `calcEdgeBands` method.
    /// The algorithm processes each panel that has edge information and finds
    /// the corresponding tile node to calculate edge banding lengths.
    /// 
    /// # Arguments
    /// * `tile_nodes` - Slice of tile nodes representing the cut layout
    /// * `panels` - Slice of panels with edge specifications  
    /// * `scale_factor` - Scale factor for dimension conversion (must be positive)
    /// 
    /// # Returns
    /// * `Ok(HashMap<String, f64>)` - Map of edge type to total length required
    /// * `Err(EdgeBandingError)` - If calculation fails due to invalid inputs
    /// 
    /// # Algorithm
    /// 1. Validate scale factor (must be positive)
    /// 2. For each panel with edge information:
    ///    - Find matching tile node by external_id
    ///    - Calculate effective dimensions based on rotation
    ///    - Add edge lengths to the result map
    /// 
    /// # Examples
    /// ```
    /// use std::collections::HashMap;
    /// use rezalnyash_core::models::edge_banding::EdgeBanding;
    /// 
    /// let tile_nodes = vec![];
    /// let panels = vec![];
    /// let scale_factor = 1000.0;
    /// 
    /// let result = EdgeBanding::calc_edge_bands(&tile_nodes, &panels, scale_factor);
    /// assert!(result.is_ok());
    /// ```
    pub fn calc_edge_bands(
        tile_nodes: &[TileNode],
        panels: &[Panel],
        scale_factor: f64,
    ) -> Result<HashMap<String, f64>, EdgeBandingError> {
        // Validate scale factor - equivalent to Java's implicit validation
        if scale_factor <= 0.0 {
            return Err(EdgeBandingError::InvalidScaleFactor(scale_factor));
        }

        // Initialize result map - equivalent to Java's HashMap map = new HashMap()
        let mut edge_bands = HashMap::new();

        // Process each panel - equivalent to Java's for (CalculationRequest.Panel panel : list2)
        for panel in panels {
            // Check if panel has edge information - equivalent to Java's if (panel.getEdge() != null)
            if let Some(edge) = panel.edge() {
                // Find matching tile node - equivalent to Java's nested loop
                for tile_node in tile_nodes {
                    // Match by external ID - equivalent to Java's if (tileNode.getExternalId() == panel.getId())
                    if tile_node.external_id() == panel.id() {
                        // Calculate effective dimensions based on rotation
                        // This is equivalent to the Java rotation logic:
                        // if (tileNode.isRotated()) {
                        //     height = tileNode.getWidth();
                        //     height2 = tileNode.getHeight();
                        // } else {
                        //     int width = tileNode.getWidth();
                        //     height = tileNode.getHeight();
                        //     height2 = width;
                        // }
                        let (effective_width, effective_height) = if tile_node.is_rotated() {
                            (tile_node.height(), tile_node.width())
                        } else {
                            (tile_node.width(), tile_node.height())
                        };

                        // Process each edge type - direct conversion from Java
                        // String top = panel.getEdge().getTop();
                        // if (top != null) { ... }
                        if let Some(top_edge) = edge.top() {
                            Self::add_edge_length(
                                &mut edge_bands,
                                top_edge,
                                effective_width as f64,
                                scale_factor,
                            )?;
                        }

                        // String left = panel.getEdge().getLeft();
                        // if (left != null) { ... }
                        if let Some(left_edge) = edge.left() {
                            Self::add_edge_length(
                                &mut edge_bands,
                                left_edge,
                                effective_height as f64,
                                scale_factor,
                            )?;
                        }

                        // String bottom = panel.getEdge().getBottom();
                        // if (bottom != null) { ... }
                        if let Some(bottom_edge) = edge.bottom() {
                            Self::add_edge_length(
                                &mut edge_bands,
                                bottom_edge,
                                effective_width as f64,
                                scale_factor,
                            )?;
                        }

                        // String right = panel.getEdge().getRight();
                        // if (right != null) { ... }
                        if let Some(right_edge) = edge.right() {
                            Self::add_edge_length(
                                &mut edge_bands,
                                right_edge,
                                effective_height as f64,
                                scale_factor,
                            )?;
                        }

                        // Break after finding the matching tile node (optimization)
                        break;
                    }
                }
            }
        }

        Ok(edge_bands)
    }

    /// Adds edge length to the map - helper method that encapsulates the Java logic
    /// 
    /// This is equivalent to the Java pattern:
    /// ```java
    /// map.put(edgeType, Double.valueOf((map.get(edgeType) != null ? 
    ///     ((Double) map.get(edgeType)).doubleValue() : 0.0d) + (dimension / d)));
    /// ```
    /// 
    /// # Arguments
    /// * `edge_bands` - Mutable reference to the edge bands map
    /// * `edge_type` - The edge type identifier
    /// * `dimension` - The dimension length for this edge
    /// * `scale_factor` - Scale factor for conversion
    /// 
    /// # Returns
    /// * `Ok(())` - If processing succeeds
    /// * `Err(EdgeBandingError)` - If division by zero occurs
    fn add_edge_length(
        edge_bands: &mut HashMap<String, f64>,
        edge_type: &str,
        dimension: f64,
        scale_factor: f64,
    ) -> Result<(), EdgeBandingError> {
        // Prevent division by zero
        if scale_factor == 0.0 {
            return Err(EdgeBandingError::DivisionByZero);
        }

        // Calculate new length
        let length = dimension / scale_factor;
        
        // Get current value or 0.0 if not present - equivalent to Java's ternary operator
        let current_value = edge_bands.get(edge_type).copied().unwrap_or(0.0);
        
        // Add to existing value and update map
        edge_bands.insert(edge_type.to_string(), current_value + length);
        
        Ok(())
    }

    /// Alternative method with enhanced error checking and validation
    /// 
    /// This method provides additional validation beyond the original Java implementation
    /// to make the Rust version more robust.
    /// 
    /// # Arguments
    /// * `tile_nodes` - Slice of tile nodes representing the cut layout
    /// * `panels` - Slice of panels with edge specifications
    /// * `scale_factor` - Scale factor for dimension conversion
    /// 
    /// # Returns
    /// * `Ok(HashMap<String, f64>)` - Map of edge type to total length required
    /// * `Err(EdgeBandingError)` - Detailed error information
    pub fn calc_edge_bands_with_validation(
        tile_nodes: &[TileNode],
        panels: &[Panel],
        scale_factor: f64,
    ) -> Result<HashMap<String, f64>, EdgeBandingError> {
        // Enhanced input validation
        if scale_factor <= 0.0 {
            return Err(EdgeBandingError::InvalidScaleFactor(scale_factor));
        }

        // Validate that panels with edges have corresponding tile nodes
        for panel in panels {
            if panel.edge().is_some() {
                let found = tile_nodes.iter().any(|node| node.external_id() == panel.id());
                if !found {
                    return Err(EdgeBandingError::PanelNotFound(panel.id()));
                }
            }
        }

        // Perform the standard calculation
        Self::calc_edge_bands(tile_nodes, panels, scale_factor)
    }

    /// Calculates edge banding for a single panel and tile node pair
    /// 
    /// This is a utility method that wasn't in the original Java but provides
    /// useful functionality for processing individual panels.
    /// 
    /// # Arguments
    /// * `tile_node` - The tile node representing the cut piece
    /// * `panel` - The panel with edge specifications
    /// * `scale_factor` - Scale factor for dimension conversion
    /// 
    /// # Returns
    /// * `Ok(HashMap<String, f64>)` - Map of edge type to length required
    /// * `Err(EdgeBandingError)` - If calculation fails
    pub fn calc_single_panel_edge_bands(
        tile_node: &TileNode,
        panel: &Panel,
        scale_factor: f64,
    ) -> Result<HashMap<String, f64>, EdgeBandingError> {
        if scale_factor <= 0.0 {
            return Err(EdgeBandingError::InvalidScaleFactor(scale_factor));
        }

        let mut edge_bands = HashMap::new();

        if let Some(edge) = panel.edge() {
            // Only process if the tile node matches the panel
            if tile_node.external_id() == panel.id() {
                // Calculate effective dimensions based on rotation
                let (effective_width, effective_height) = if tile_node.is_rotated() {
                    (tile_node.height(), tile_node.width())
                } else {
                    (tile_node.width(), tile_node.height())
                };

                // Process each edge type
                if let Some(top_edge) = edge.top() {
                    Self::add_edge_length(
                        &mut edge_bands,
                        top_edge,
                        effective_width as f64,
                        scale_factor,
                    )?;
                }

                if let Some(left_edge) = edge.left() {
                    Self::add_edge_length(
                        &mut edge_bands,
                        left_edge,
                        effective_height as f64,
                        scale_factor,
                    )?;
                }

                if let Some(bottom_edge) = edge.bottom() {
                    Self::add_edge_length(
                        &mut edge_bands,
                        bottom_edge,
                        effective_width as f64,
                        scale_factor,
                    )?;
                }

                if let Some(right_edge) = edge.right() {
                    Self::add_edge_length(
                        &mut edge_bands,
                        right_edge,
                        effective_height as f64,
                        scale_factor,
                    )?;
                }
            }
        }

        Ok(edge_bands)
    }

    /// Gets the total edge banding length across all edge types
    /// 
    /// # Arguments
    /// * `edge_bands` - Map of edge type to length
    /// 
    /// # Returns
    /// Total length of all edge banding required
    pub fn total_edge_band_length(edge_bands: &HashMap<String, f64>) -> f64 {
        edge_bands.values().sum()
    }

    /// Gets unique edge types from the calculation result
    /// 
    /// # Arguments
    /// * `edge_bands` - Map of edge type to length
    /// 
    /// # Returns
    /// Vector of unique edge type names sorted alphabetically
    pub fn get_edge_types(edge_bands: &HashMap<String, f64>) -> Vec<String> {
        let mut types: Vec<String> = edge_bands.keys().cloned().collect();
        types.sort();
        types
    }

    /// Filters edge bands by minimum length threshold
    /// 
    /// # Arguments
    /// * `edge_bands` - Map of edge type to length
    /// * `min_length` - Minimum length threshold
    /// 
    /// # Returns
    /// Filtered map containing only edge types with length >= min_length
    pub fn filter_by_min_length(
        edge_bands: &HashMap<String, f64>,
        min_length: f64,
    ) -> HashMap<String, f64> {
        edge_bands
            .iter()
            .filter(|(_, &length)| length >= min_length)
            .map(|(k, &v)| (k.clone(), v))
            .collect()
    }

    /// Converts edge bands map to a sorted vector of (edge_type, length) pairs
    /// 
    /// # Arguments
    /// * `edge_bands` - Map of edge type to length
    /// 
    /// # Returns
    /// Vector of (edge_type, length) pairs sorted by edge type
    pub fn to_sorted_vec(edge_bands: &HashMap<String, f64>) -> Vec<(String, f64)> {
        let mut pairs: Vec<(String, f64)> = edge_bands
            .iter()
            .map(|(k, &v)| (k.clone(), v))
            .collect();
        pairs.sort_by(|a, b| a.0.cmp(&b.0));
        pairs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::calculation_request::{Panel, Edge};
    use crate::models::TileNode;

    #[test]
    fn test_empty_inputs() {
        let result = EdgeBanding::calc_edge_bands(&[], &[], 1000.0);
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_invalid_scale_factor() {
        let result = EdgeBanding::calc_edge_bands(&[], &[], 0.0);
        assert!(matches!(result, Err(EdgeBandingError::InvalidScaleFactor(0.0))));

        let result = EdgeBanding::calc_edge_bands(&[], &[], -1.0);
        assert!(matches!(result, Err(EdgeBandingError::InvalidScaleFactor(-1.0))));
    }

    #[test]
    fn test_division_by_zero() {
        // This test ensures our add_edge_length method properly handles division by zero
        let mut edge_bands = HashMap::new();
        let result = EdgeBanding::add_edge_length(&mut edge_bands, "test", 100.0, 0.0);
        assert!(matches!(result, Err(EdgeBandingError::DivisionByZero)));
    }

    #[test]
    fn test_basic_edge_calculation() -> Result<(), Box<dyn std::error::Error>> {
        // Create a simple tile node
        let mut tile_node = TileNode::new(0, 0, 1000, 500)?;
        tile_node.set_external_id(1);

        // Create a panel with edge information
        let edge = Edge::new(
            Some("top_edge".to_string()),
            Some("left_edge".to_string()),
            Some("bottom_edge".to_string()),
            Some("right_edge".to_string()),
        );
        let panel = Panel::simple(1, "1000".to_string(), "500".to_string(), 1)
            .with_edge(edge);

        let tile_nodes = vec![tile_node];
        let panels = vec![panel];
        let scale_factor = 1.0;

        let result = EdgeBanding::calc_edge_bands(&tile_nodes, &panels, scale_factor)?;

        // Check that all edges are calculated
        assert_eq!(result.len(), 4);
        assert_eq!(result.get("top_edge"), Some(&1000.0));
        assert_eq!(result.get("bottom_edge"), Some(&1000.0));
        assert_eq!(result.get("left_edge"), Some(&500.0));
        assert_eq!(result.get("right_edge"), Some(&500.0));

        Ok(())
    }

    #[test]
    fn test_rotated_tile_calculation() -> Result<(), Box<dyn std::error::Error>> {
        // Create a rotated tile node
        let mut tile_node = TileNode::new(0, 0, 1000, 500)?;
        tile_node.set_external_id(1);
        tile_node.set_rotated(true);

        // Create a panel with edge information
        let edge = Edge::new(
            Some("top_edge".to_string()),
            Some("left_edge".to_string()),
            Some("bottom_edge".to_string()),
            Some("right_edge".to_string()),
        );
        let panel = Panel::simple(1, "1000".to_string(), "500".to_string(), 1)
            .with_edge(edge);

        let tile_nodes = vec![tile_node];
        let panels = vec![panel];
        let scale_factor = 1.0;

        let result = EdgeBanding::calc_edge_bands(&tile_nodes, &panels, scale_factor)?;

        // For rotated tile, width and height are swapped
        assert_eq!(result.len(), 4);
        assert_eq!(result.get("top_edge"), Some(&500.0));    // height becomes width
        assert_eq!(result.get("bottom_edge"), Some(&500.0));  // height becomes width
        assert_eq!(result.get("left_edge"), Some(&1000.0));   // width becomes height
        assert_eq!(result.get("right_edge"), Some(&1000.0));  // width becomes height

        Ok(())
    }

    #[test]
    fn test_multiple_panels_same_edge_type() -> Result<(), Box<dyn std::error::Error>> {
        // Create two tile nodes
        let mut tile_node1 = TileNode::new(0, 0, 1000, 500)?;
        tile_node1.set_external_id(1);
        
        let mut tile_node2 = TileNode::new(0, 0, 800, 600)?;
        tile_node2.set_external_id(2);

        // Create panels with same edge type
        let edge1 = Edge::new(Some("common_edge".to_string()), None, None, None);
        let panel1 = Panel::simple(1, "1000".to_string(), "500".to_string(), 1)
            .with_edge(edge1);

        let edge2 = Edge::new(Some("common_edge".to_string()), None, None, None);
        let panel2 = Panel::simple(2, "800".to_string(), "600".to_string(), 1)
            .with_edge(edge2);

        let tile_nodes = vec![tile_node1, tile_node2];
        let panels = vec![panel1, panel2];
        let scale_factor = 1.0;

        let result = EdgeBanding::calc_edge_bands(&tile_nodes, &panels, scale_factor)?;

        // Should sum the lengths for the same edge type
        assert_eq!(result.len(), 1);
        assert_eq!(result.get("common_edge"), Some(&1800.0)); // 1000 + 800

        Ok(())
    }

    #[test]
    fn test_scale_factor_application() -> Result<(), Box<dyn std::error::Error>> {
        let mut tile_node = TileNode::new(0, 0, 1000, 500)?;
        tile_node.set_external_id(1);

        let edge = Edge::new(Some("test_edge".to_string()), None, None, None);
        let panel = Panel::simple(1, "1000".to_string(), "500".to_string(), 1)
            .with_edge(edge);

        let tile_nodes = vec![tile_node];
        let panels = vec![panel];
        let scale_factor = 1000.0;

        let result = EdgeBanding::calc_edge_bands(&tile_nodes, &panels, scale_factor)?;

        // Length should be divided by scale factor
        assert_eq!(result.get("test_edge"), Some(&1.0)); // 1000 / 1000

        Ok(())
    }

    #[test]
    fn test_panel_without_matching_tile_node() -> Result<(), Box<dyn std::error::Error>> {
        let mut tile_node = TileNode::new(0, 0, 1000, 500)?;
        tile_node.set_external_id(1);

        let edge = Edge::new(Some("test_edge".to_string()), None, None, None);
        let panel = Panel::simple(2, "1000".to_string(), "500".to_string(), 1) // Different ID
            .with_edge(edge);

        let tile_nodes = vec![tile_node];
        let panels = vec![panel];
        let scale_factor = 1.0;

        let result = EdgeBanding::calc_edge_bands(&tile_nodes, &panels, scale_factor)?;

        // Should be empty since no matching tile node
        assert!(result.is_empty());

        Ok(())
    }

    #[test]
    fn test_total_edge_band_length() {
        let mut edge_bands = HashMap::new();
        edge_bands.insert("type1".to_string(), 10.0);
        edge_bands.insert("type2".to_string(), 20.0);
        edge_bands.insert("type3".to_string(), 30.0);

        assert_eq!(EdgeBanding::total_edge_band_length(&edge_bands), 60.0);
    }

    #[test]
    fn test_get_edge_types() {
        let mut edge_bands = HashMap::new();
        edge_bands.insert("zebra".to_string(), 10.0);
        edge_bands.insert("alpha".to_string(), 20.0);
        edge_bands.insert("beta".to_string(), 30.0);

        let types = EdgeBanding::get_edge_types(&edge_bands);
        assert_eq!(types, vec!["alpha", "beta", "zebra"]); // Should be sorted
    }

    #[test]
    fn test_filter_by_min_length() {
        let mut edge_bands = HashMap::new();
        edge_bands.insert("type1".to_string(), 5.0);
        edge_bands.insert("type2".to_string(), 15.0);
        edge_bands.insert("type3".to_string(), 25.0);

        let filtered = EdgeBanding::filter_by_min_length(&edge_bands, 10.0);
        assert_eq!(filtered.len(), 2);
        assert!(filtered.contains_key("type2"));
        assert!(filtered.contains_key("type3"));
        assert!(!filtered.contains_key("type1"));
    }

    #[test]
    fn test_to_sorted_vec() {
        let mut edge_bands = HashMap::new();
        edge_bands.insert("zebra".to_string(), 30.0);
        edge_bands.insert("alpha".to_string(), 10.0);
        edge_bands.insert("beta".to_string(), 20.0);

        let sorted = EdgeBanding::to_sorted_vec(&edge_bands);
        assert_eq!(sorted, vec![
            ("alpha".to_string(), 10.0),
            ("beta".to_string(), 20.0),
            ("zebra".to_string(), 30.0),
        ]);
    }

    #[test]
    fn test_calc_edge_bands_with_validation() -> Result<(), Box<dyn std::error::Error>> {
        // Test with missing tile node
        let edge = Edge::new(Some("test_edge".to_string()), None, None, None);
        let panel = Panel::simple(1, "1000".to_string(), "500".to_string(), 1)
            .with_edge(edge);

        let tile_nodes = vec![];
        let panels = vec![panel];
        let scale_factor = 1.0;

        let result = EdgeBanding::calc_edge_bands_with_validation(&tile_nodes, &panels, scale_factor);
        assert!(matches!(result, Err(EdgeBandingError::PanelNotFound(1))));

        Ok(())
    }
}
