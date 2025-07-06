use serde::Serialize;
use uuid::Uuid;

// 2. РАБОЧАЯ МОДЕЛЬ - после конвертации размеров, но до развертывания
#[derive(Serialize, Debug, Clone)]
pub struct Panel {
    pub width: u32,
    pub height: u32,
    pub count: u8,
    pub label: String,
    pub original_id: u16,
}

impl Panel {
   pub fn new(width: u32, height: u32, count: u8, label: String, original_id: u16) -> Self {
        Self {
            width,
            height,
            count,
            label,
            original_id,
        }
    }
    /// Проверить, является ли панель квадратной
    pub fn is_square(&self) -> bool {
        self.width == self.height
    }
    

    
}
