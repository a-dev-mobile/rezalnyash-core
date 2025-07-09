use serde::Serialize;
use uuid::Uuid;

use crate::constants::MaterialConstants;

// 3. РАЗВЕРНУТАЯ МОДЕЛЬ - готова для алгоритма размещения (count всегда = 1)
#[derive(Serialize, Debug, Clone)]
pub struct TileDimensions {
    pub id: u16,
    pub width: u32,
    pub height: u32,
    pub material: String,
    pub label: String,

    pub is_rotated: bool,
}

impl TileDimensions {
    pub fn new(
        id: u16,
        width: u32,
        height: u32,
        is_rotated: bool,
        label: &str,
        material: &str,
    ) -> Self {
        Self {
            id,
            width,
            height,

            label: label.to_string(),
            material: material.to_string(),
            is_rotated,
        }
    }
    //
    /// Реализуем toString() ТОЧНО как в Java Это критично для правильной работы HashMap в алгоритме группировки
    pub fn to_string(&self) -> String {
        format!("id={}[{}x{}]", self.id, self.width, self.height)
    }
}
