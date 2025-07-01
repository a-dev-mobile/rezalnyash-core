//! Конвертер для массового преобразования данных

use super::errors::ScaledError;
use super::precision_analyzer::PrecisionAnalyzer;
use super::scaled_number::ScaledNumber;

/// Конвертер для массового преобразования данных
pub struct ScaledConverter {
    precision: u8,
}

impl ScaledConverter {
    /// Создает новый конвертер с заданной точностью
    pub fn new(precision: u8) -> Result<Self, ScaledError> {
        if precision > ScaledNumber::MAX_PRECISION {
            return Err(ScaledError::InvalidPrecision(precision));
        }
        Ok(Self { precision })
    }

    /// Возвращает точность конвертера
    pub fn precision(&self) -> u8 {
        self.precision
    }

    /// Создает конвертер с автоматической точностью на основе строковых данных
    pub fn from_strings(strings: &[&str]) -> Result<Self, ScaledError> {
        let precision = PrecisionAnalyzer::max_decimal_places(strings);
        Self::new(precision)
    }

    /// Преобразует массив f32 в ScaledNumber
    pub fn convert_f32_slice(&self, values: &[f32]) -> Result<Vec<ScaledNumber>, ScaledError> {
        values
            .iter()
            .map(|&v| ScaledNumber::from_f32(v, self.precision))
            .collect()
    }

    /// Преобразует массив f64 в ScaledNumber  
    pub fn convert_f64_slice(&self, values: &[f64]) -> Result<Vec<ScaledNumber>, ScaledError> {
        values
            .iter()
            .map(|&v| ScaledNumber::from_f64(v, self.precision))
            .collect()
    }

    /// Преобразует массив u32 в ScaledNumber
    pub fn convert_u32_slice(&self, values: &[u32]) -> Result<Vec<ScaledNumber>, ScaledError> {
        values
            .iter()
            .map(|&v| ScaledNumber::from_u32(v, self.precision))
            .collect()
    }

    /// Преобразует массив u64 в ScaledNumber
    pub fn convert_u64_slice(&self, values: &[u64]) -> Result<Vec<ScaledNumber>, ScaledError> {
        values
            .iter()
            .map(|&v| ScaledNumber::from_u64(v, self.precision))
            .collect()
    }

    /// Преобразует массив строк в ScaledNumber
    pub fn convert_string_slice(&self, values: &[&str]) -> Result<Vec<ScaledNumber>, ScaledError> {
        values
            .iter()
            .map(|s| ScaledNumber::from_str(s, self.precision))
            .collect()
    }

    /// Преобразует ScaledNumber обратно в f32
    pub fn to_f32_vec(&self, values: &[ScaledNumber]) -> Vec<f32> {
        values.iter().map(|v| v.to_f32()).collect()
    }

    /// Преобразует ScaledNumber обратно в f64
    pub fn to_f64_vec(&self, values: &[ScaledNumber]) -> Vec<f64> {
        values.iter().map(|v| v.to_f64()).collect()
    }

    /// Преобразует ScaledNumber обратно в u32 (с обработкой ошибок)
    pub fn to_u32_vec(&self, values: &[ScaledNumber]) -> Result<Vec<u32>, ScaledError> {
        values.iter().map(|v| v.to_u32()).collect()
    }

    /// Преобразует ScaledNumber обратно в u64 (с обработкой ошибок)
    pub fn to_u64_vec(&self, values: &[ScaledNumber]) -> Result<Vec<u64>, ScaledError> {
        values.iter().map(|v| v.to_u64()).collect()
    }

    /// Преобразует ScaledNumber обратно в u32 с округлением
    pub fn to_u32_vec_rounded(&self, values: &[ScaledNumber]) -> Result<Vec<u32>, ScaledError> {
        values.iter().map(|v| v.to_u32_rounded()).collect()
    }

    /// Преобразует ScaledNumber обратно в u64 с округлением
    pub fn to_u64_vec_rounded(&self, values: &[ScaledNumber]) -> Result<Vec<u64>, ScaledError> {
        values.iter().map(|v| v.to_u64_rounded()).collect()
    }
}
