//! Calculation request model for cutting optimization
//! 
//! This module provides the main data structures for calculation requests,
//! including panels, edges, and validation functionality.

use serde::{Deserialize, Serialize};
use std::fmt;
use crate::models::configuration::Configuration;
use crate::models::client_info::ClientInfo;

/// Default material constant for panels
const DEFAULT_MATERIAL: &str = "DEFAULT_MATERIAL";

/// Represents a calculation request containing configuration, panels, and client information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalculationRequest {
    /// Configuration settings for the calculation
    configuration: Option<Configuration>,
    /// List of panels to be cut
    panels: Vec<Panel>,
    /// List of stock panels available for cutting
    stock_panels: Vec<Panel>,
    /// Client information for the request
    client_info: Option<ClientInfo>,
}

impl CalculationRequest {
    /// Creates a new empty CalculationRequest
    pub fn new() -> Self {
        Self {
            configuration: None,
            panels: Vec::new(),
            stock_panels: Vec::new(),
            client_info: None,
        }
    }

    /// Creates a new CalculationRequest with specified values
    pub fn with_values(
        configuration: Option<Configuration>,
        panels: Vec<Panel>,
        stock_panels: Vec<Panel>,
        client_info: Option<ClientInfo>,
    ) -> Self {
        Self {
            configuration,
            panels,
            stock_panels,
            client_info,
        }
    }

    // Getters
    pub fn configuration(&self) -> Option<&Configuration> {
        self.configuration.as_ref()
    }

    pub fn panels(&self) -> &[Panel] {
        &self.panels
    }

    pub fn stock_panels(&self) -> &[Panel] {
        &self.stock_panels
    }

    pub fn client_info(&self) -> Option<&ClientInfo> {
        self.client_info.as_ref()
    }

    // Setters
    pub fn set_configuration(&mut self, configuration: Option<Configuration>) {
        self.configuration = configuration;
    }

    pub fn set_panels(&mut self, panels: Vec<Panel>) {
        self.panels = panels;
    }

    pub fn set_stock_panels(&mut self, stock_panels: Vec<Panel>) {
        self.stock_panels = stock_panels;
    }

    pub fn set_client_info(&mut self, client_info: Option<ClientInfo>) {
        self.client_info = client_info;
    }

    /// Converts tiles to string representation, including only enabled panels with count > 0
    pub fn tiles_to_string(&self) -> String {
        let mut result = String::new();
        for panel in &self.panels {
            if panel.count() > 0 {
                result.push(' ');
                result.push_str(&panel.to_string());
            }
        }
        result
    }

    /// Converts base tiles (stock panels) to string representation
    pub fn base_tiles_to_string(&self) -> String {
        let mut result = String::new();
        for panel in &self.stock_panels {
            if panel.count() > 0 {
                result.push(' ');
                result.push_str(&panel.to_string());
            }
        }
        result
    }

    /// Validates the calculation request
    pub fn validate(&self) -> Result<(), Vec<CalculationRequestError>> {
        let mut errors = Vec::new();

        // Validate that we have at least one panel
        if self.panels.is_empty() {
            errors.push(CalculationRequestError::EmptyPanels);
        }

        // Validate each panel
        for (index, panel) in self.panels.iter().enumerate() {
            if let Err(panel_errors) = panel.validate() {
                for error in panel_errors {
                    errors.push(CalculationRequestError::InvalidPanel { index, error });
                }
            }
        }

        // Validate each stock panel
        for (index, panel) in self.stock_panels.iter().enumerate() {
            if let Err(panel_errors) = panel.validate() {
                for error in panel_errors {
                    errors.push(CalculationRequestError::InvalidStockPanel { index, error });
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Returns true if the calculation request is valid
    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }

    /// Builder pattern methods
    pub fn with_configuration(mut self, configuration: Configuration) -> Self {
        self.configuration = Some(configuration);
        self
    }

    pub fn with_panels(mut self, panels: Vec<Panel>) -> Self {
        self.panels = panels;
        self
    }

    pub fn with_stock_panels(mut self, stock_panels: Vec<Panel>) -> Self {
        self.stock_panels = stock_panels;
        self
    }

    pub fn with_client_info(mut self, client_info: ClientInfo) -> Self {
        self.client_info = Some(client_info);
        self
    }

    pub fn add_panel(mut self, panel: Panel) -> Self {
        self.panels.push(panel);
        self
    }

    pub fn add_stock_panel(mut self, panel: Panel) -> Self {
        self.stock_panels.push(panel);
        self
    }
}

impl Default for CalculationRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a panel with dimensions, count, and properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Panel {
    /// Unique identifier for the panel
    id: i32,
    /// Width of the panel as string for precision
    width: String,
    /// Height of the panel as string for precision
    height: String,
    /// Number of panels needed
    count: i32,
    /// Material type
    material: String,
    /// Whether the panel is enabled for cutting
    enabled: bool,
    /// Orientation value
    orientation: i32,
    /// Optional label for the panel
    label: Option<String>,
    /// Edge information for the panel
    edge: Option<Edge>,
}

impl Panel {
    /// Creates a new Panel with all parameters
    pub fn new(
        id: i32,
        width: String,
        height: String,
        count: i32,
        material: String,
        enabled: bool,
        orientation: i32,
        label: Option<String>,
        edge: Option<Edge>,
    ) -> Self {
        Self {
            id,
            width,
            height,
            count,
            material,
            enabled,
            orientation,
            label,
            edge,
        }
    }

    /// Creates a simple Panel with basic parameters
    pub fn simple(id: i32, width: String, height: String, count: i32) -> Self {
        Self {
            id,
            width,
            height,
            count,
            material: DEFAULT_MATERIAL.to_string(),
            enabled: true,
            orientation: 0,
            label: None,
            edge: None,
        }
    }

    // Getters
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn width(&self) -> &str {
        &self.width
    }

    pub fn height(&self) -> &str {
        &self.height
    }

    pub fn count(&self) -> i32 {
        self.count
    }

    pub fn material(&self) -> &str {
        &self.material
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn orientation(&self) -> i32 {
        self.orientation
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    pub fn edge(&self) -> Option<&Edge> {
        self.edge.as_ref()
    }

    // Setters
    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    pub fn set_width(&mut self, width: String) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: String) {
        self.height = height;
    }

    pub fn set_count(&mut self, count: i32) {
        self.count = count;
    }

    pub fn set_material(&mut self, material: Option<String>) {
        if let Some(mat) = material {
            self.material = mat;
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_orientation(&mut self, orientation: i32) {
        self.orientation = orientation;
    }

    pub fn set_label(&mut self, label: Option<String>) {
        self.label = label;
    }

    pub fn set_edge(&mut self, edge: Option<Edge>) {
        self.edge = edge;
    }

    /// Validates the panel and returns true if it's valid for cutting
    pub fn is_valid(&self) -> bool {
        if !self.enabled || self.count <= 0 {
            return false;
        }

        // Try to parse width and height as doubles
        match (self.width.parse::<f64>(), self.height.parse::<f64>()) {
            (Ok(w), Ok(h)) => w > 0.0 && h > 0.0,
            _ => false,
        }
    }

    /// Validates the panel and returns detailed validation results
    pub fn validate(&self) -> Result<(), Vec<PanelError>> {
        let mut errors = Vec::new();

        if self.count < 0 {
            errors.push(PanelError::NegativeCount(self.count));
        }

        if self.width.trim().is_empty() {
            errors.push(PanelError::EmptyDimension("width".to_string()));
        } else if let Err(_) = self.width.parse::<f64>() {
            errors.push(PanelError::InvalidDimension {
                field: "width".to_string(),
                value: self.width.clone(),
            });
        } else if let Ok(w) = self.width.parse::<f64>() {
            if w <= 0.0 {
                errors.push(PanelError::NonPositiveDimension {
                    field: "width".to_string(),
                    value: w,
                });
            }
        }

        if self.height.trim().is_empty() {
            errors.push(PanelError::EmptyDimension("height".to_string()));
        } else if let Err(_) = self.height.parse::<f64>() {
            errors.push(PanelError::InvalidDimension {
                field: "height".to_string(),
                value: self.height.clone(),
            });
        } else if let Ok(h) = self.height.parse::<f64>() {
            if h <= 0.0 {
                errors.push(PanelError::NonPositiveDimension {
                    field: "height".to_string(),
                    value: h,
                });
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Parses width as f64, returns None if invalid
    pub fn width_as_f64(&self) -> Option<f64> {
        self.width.parse().ok()
    }

    /// Parses height as f64, returns None if invalid
    pub fn height_as_f64(&self) -> Option<f64> {
        self.height.parse().ok()
    }

    /// Calculates area if dimensions are valid
    pub fn area(&self) -> Option<f64> {
        match (self.width_as_f64(), self.height_as_f64()) {
            (Some(w), Some(h)) => Some(w * h),
            _ => None,
        }
    }

    /// Builder pattern methods
    pub fn with_id(mut self, id: i32) -> Self {
        self.id = id;
        self
    }

    pub fn with_width(mut self, width: String) -> Self {
        self.width = width;
        self
    }

    pub fn with_height(mut self, height: String) -> Self {
        self.height = height;
        self
    }

    pub fn with_count(mut self, count: i32) -> Self {
        self.count = count;
        self
    }

    pub fn with_material(mut self, material: String) -> Self {
        self.material = material;
        self
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn with_orientation(mut self, orientation: i32) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    pub fn with_edge(mut self, edge: Edge) -> Self {
        self.edge = Some(edge);
        self
    }
}

impl fmt::Display for Panel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}x{}]*{}{}",
            self.width,
            self.height,
            self.count,
            if self.enabled { "" } else { "-disabled" }
        )
    }
}

/// Represents edge information for a panel
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    /// Top edge information
    top: Option<String>,
    /// Left edge information
    left: Option<String>,
    /// Bottom edge information
    bottom: Option<String>,
    /// Right edge information
    right: Option<String>,
}

impl Edge {
    /// Creates a new Edge with all sides
    pub fn new(
        top: Option<String>,
        left: Option<String>,
        bottom: Option<String>,
        right: Option<String>,
    ) -> Self {
        Self {
            top,
            left,
            bottom,
            right,
        }
    }

    /// Creates an empty Edge with no side information
    pub fn empty() -> Self {
        Self {
            top: None,
            left: None,
            bottom: None,
            right: None,
        }
    }

    // Getters
    pub fn top(&self) -> Option<&str> {
        self.top.as_deref()
    }

    pub fn left(&self) -> Option<&str> {
        self.left.as_deref()
    }

    pub fn bottom(&self) -> Option<&str> {
        self.bottom.as_deref()
    }

    pub fn right(&self) -> Option<&str> {
        self.right.as_deref()
    }

    // Setters
    pub fn set_top(&mut self, top: Option<String>) {
        self.top = top;
    }

    pub fn set_left(&mut self, left: Option<String>) {
        self.left = left;
    }

    pub fn set_bottom(&mut self, bottom: Option<String>) {
        self.bottom = bottom;
    }

    pub fn set_right(&mut self, right: Option<String>) {
        self.right = right;
    }

    /// Builder pattern methods
    pub fn with_top(mut self, top: String) -> Self {
        self.top = Some(top);
        self
    }

    pub fn with_left(mut self, left: String) -> Self {
        self.left = Some(left);
        self
    }

    pub fn with_bottom(mut self, bottom: String) -> Self {
        self.bottom = Some(bottom);
        self
    }

    pub fn with_right(mut self, right: String) -> Self {
        self.right = Some(right);
        self
    }

    /// Checks if any edge information is present
    pub fn has_any_edge(&self) -> bool {
        self.top.is_some() || self.left.is_some() || self.bottom.is_some() || self.right.is_some()
    }

    /// Checks if all edges are defined
    pub fn has_all_edges(&self) -> bool {
        self.top.is_some() && self.left.is_some() && self.bottom.is_some() && self.right.is_some()
    }
}

impl Default for Edge {
    fn default() -> Self {
        Self::empty()
    }
}

/// Errors that can occur when working with CalculationRequest
#[derive(Debug, Clone, PartialEq)]
pub enum CalculationRequestError {
    /// No panels provided in the request
    EmptyPanels,
    /// Invalid panel at specified index
    InvalidPanel { index: usize, error: PanelError },
    /// Invalid stock panel at specified index
    InvalidStockPanel { index: usize, error: PanelError },
}

/// Errors that can occur when working with Panel
#[derive(Debug, Clone, PartialEq)]
pub enum PanelError {
    /// Negative count value
    NegativeCount(i32),
    /// Empty dimension field
    EmptyDimension(String),
    /// Invalid dimension value that cannot be parsed
    InvalidDimension { field: String, value: String },
    /// Non-positive dimension value
    NonPositiveDimension { field: String, value: f64 },
}

impl fmt::Display for CalculationRequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalculationRequestError::EmptyPanels => {
                write!(f, "Calculation request must contain at least one panel")
            }
            CalculationRequestError::InvalidPanel { index, error } => {
                write!(f, "Invalid panel at index {}: {}", index, error)
            }
            CalculationRequestError::InvalidStockPanel { index, error } => {
                write!(f, "Invalid stock panel at index {}: {}", index, error)
            }
        }
    }
}

impl fmt::Display for PanelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PanelError::NegativeCount(count) => {
                write!(f, "Panel count cannot be negative: {}", count)
            }
            PanelError::EmptyDimension(field) => {
                write!(f, "Panel {} cannot be empty", field)
            }
            PanelError::InvalidDimension { field, value } => {
                write!(f, "Invalid {} dimension: '{}' cannot be parsed as number", field, value)
            }
            PanelError::NonPositiveDimension { field, value } => {
                write!(f, "Panel {} must be positive: {}", field, value)
            }
        }
    }
}

impl std::error::Error for CalculationRequestError {}
impl std::error::Error for PanelError {}
