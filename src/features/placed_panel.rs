

/// Размещенная панель с координатами
#[derive(Debug, Clone)]
pub struct PlacedPanel {
    pub panel_id: i32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub is_rotated: bool,
    pub label: String,
}