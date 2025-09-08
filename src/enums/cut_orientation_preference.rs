use std::fmt;
use serde::{Deserialize, Serialize};

/// Enum представляющий предпочтения направления резов при оптимизации раскроя материалов.
/// 
/// Этот параметр контролирует, какие группы оптимизации будут использованы
/// в алгоритме раскроя, влияя на стратегии резки материала.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum CutOrientationPreference {
    /// Разрешены все направления резов (максимальная гибкость)
    /// Запускаются все группы оптимизации: AREA, AREA_HCUTS_1ST, AREA_VCUTS_1ST
    Both = 0,
    
    /// Предпочтение горизонтальным резам
    /// Запускаются группы: AREA и AREA_HCUTS_1ST
    Horizontal = 1,
    
    /// Предпочтение вертикальным резам  
    /// Запускаются группы: AREA и AREA_VCUTS_1ST
    Vertical = 2,
}

impl CutOrientationPreference {
    /// Возвращает числовое значение предпочтения (для совместимости с Java)
    pub fn value(&self) -> u8 {
        *self as u8
    }
    
    /// Создает enum из числового значения
    pub fn from_value(value: u8) -> Result<Self, String> {
        match value {
            0 => Ok(CutOrientationPreference::Both),
            1 => Ok(CutOrientationPreference::Horizontal),
            2 => Ok(CutOrientationPreference::Vertical),
            _ => Err(format!("Недопустимое значение cutOrientationPreference: {}. Допустимые значения: 0-2", value)),
        }
    }
    
}

impl Default for CutOrientationPreference {
    fn default() -> Self {
        CutOrientationPreference::Both
    }
}
