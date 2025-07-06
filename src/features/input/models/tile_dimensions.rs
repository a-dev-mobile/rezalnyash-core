use serde::Serialize;
use uuid::Uuid;

// 3. РАЗВЕРНУТАЯ МОДЕЛЬ - готова для алгоритма размещения (count всегда = 1)
#[derive(Serialize, Debug, Clone)]
pub struct TileDimensions {
    pub width: u32,
    pub height: u32,
    pub id: u16,
    pub instance_number: u8, // Номер экземпляра (1, 2, 3...)
    pub is_rotated: bool,    // Повернут на 90 градусов
}

impl TileDimensions {
    pub fn new(
        width: u32,
        height: u32,
        id: u16,
        instance_number: u8,
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
    /// Реализуем toString() ТОЧНО как в Java
    pub fn to_string(&self) -> String {
        format!("id={}[{}x{}]", self.id, self.width, self.height)
    }

    /// Реализуем dimensionsToString() как в Java
    pub fn dimensions_to_string(&self) -> String {
        format!("{}x{}", self.width, self.height)
    }
    /// Получить эффективные размеры с учетом поворота
    pub fn effective_dimensions(&self) -> (u32, u32) {
        if self.is_rotated {
            (self.height, self.width)
        } else {
            (self.width, self.height)
        }
    }

    /// Создать повернутую версию панели
    pub fn create_rotated(&self) -> Self {
        Self::new(
            self.height, // Меняем местами размеры
            self.width,
            self.id,
            self.instance_number,
            true,
        )
    }

    /// Получить уникальный идентификатор панели
    pub fn get_unique_key(&self) -> String {
        format!(
            "{}_{}_{}_{}",
            self.id,
            self.instance_number,
            if self.is_rotated { "R" } else { "N" },
            if self.is_rotated {
                format!("{}x{}", self.height, self.width)
            } else {
                format!("{}x{}", self.width, self.height)
            }
        )
    }
}
