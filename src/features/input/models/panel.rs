use serde::Serialize;


/// Деталь для размещения
#[derive(Serialize, Debug, Clone,)]
pub struct Panel {
    pub width: u32,
    pub height: u32,
    pub count: u8,
    pub label: String,
}
