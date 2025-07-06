use serde::Serialize;
use uuid::Uuid;

// 3. РАЗВЕРНУТАЯ МОДЕЛЬ - готова для алгоритма размещения (count всегда = 1)
#[derive(Serialize, Debug, Clone)]
pub struct TileDimensions {
    pub width: u32,
    pub height: u32,
    pub id: u16,
    pub instance_number: u16, // Номер экземпляра (1, 2, 3...)
    pub is_rotated: bool,    // Повернут на 90 градусов
}

impl TileDimensions {
    pub fn new(
        width: u32,
        height: u32,
        id: u16,
        instance_number: u16,
        is_rotated: bool,
    ) -> Self {
        Self {
            width,
            height,

            id,
            instance_number,
            is_rotated,
        }
    }
    /// Реализуем toString() ТОЧНО как в Java Это критично для правильной работы HashMap в алгоритме группировки
    pub fn to_string(&self) -> String {
        format!("id={}[{}x{}]", self.id, self.width, self.height)
    }



}
