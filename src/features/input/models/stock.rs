use serde::Serialize;

/// Заготовка (обработанная модель с числовыми размерами)
#[derive(Serialize, Debug, Clone)]
pub struct Stock {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Stock {
    pub fn new(width: u32, height: u32, label: &str) -> Self {
        Self {
            width,
            height,
            label: label.to_string(),
        }
    }
}