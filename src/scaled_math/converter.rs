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

    /// Создает конвертер с автоматической точностью на основе ScaledNumber
    pub fn from_scaled_numbers(numbers: &[ScaledNumber]) -> Result<Self, ScaledError> {
        let precision = PrecisionAnalyzer::max_precision(numbers);
        Self::new(precision)
    }

    // Методы преобразования ИЗ других типов В ScaledNumber ===

    /// Преобразует f32 в ScaledNumber с точностью конвертера
    pub fn convert_f32(&self, value: f32) -> Result<ScaledNumber, ScaledError> {
        ScaledNumber::from_f32(value, self.precision)
    }

    /// Преобразует f64 в ScaledNumber с точностью конвертера
    pub fn convert_f64(&self, value: f64) -> Result<ScaledNumber, ScaledError> {
        ScaledNumber::from_f64(value, self.precision)
    }

    /// Преобразует u32 в ScaledNumber с точностью конвертера
    pub fn convert_u32(&self, value: u32) -> Result<ScaledNumber, ScaledError> {
        ScaledNumber::from_u32(value, self.precision)
    }

    /// Преобразует u64 в ScaledNumber с точностью конвертера
    pub fn convert_u64(&self, value: u64) -> Result<ScaledNumber, ScaledError> {
        ScaledNumber::from_u64(value, self.precision)
    }

    /// Преобразует i32 в ScaledNumber с точностью конвертера
    pub fn convert_i32(&self, value: i32) -> Result<ScaledNumber, ScaledError> {
        ScaledNumber::from_i32(value, self.precision)
    }

    /// Преобразует i64 в ScaledNumber с точностью конвертера
    pub fn convert_i64(&self, value: i64) -> Result<ScaledNumber, ScaledError> {
        ScaledNumber::from_i64(value, self.precision)
    }

    /// Преобразует строку в ScaledNumber с точностью конвертера
    pub fn convert_string(&self, value: &str) -> Result<ScaledNumber, ScaledError> {
        ScaledNumber::from_str(value, self.precision)
    }

    // Методы создания из raw значений ===

    /// Создает ScaledNumber из сырого значения i64 с точностью конвертера
    pub fn from_raw_i64(&self, raw_value: i64) -> Result<ScaledNumber, ScaledError> {
        ScaledNumber::from_raw(raw_value, self.precision)
    }

    /// Создает ScaledNumber из сырого значения u32 с точностью конвертера
    pub fn from_raw_u32(&self, raw_value: u32) -> Result<ScaledNumber, ScaledError> {
        ScaledNumber::from_raw_u32(raw_value, self.precision)
    }

    /// Создает ScaledNumber из сырого значения u64 с точностью конвертера
    pub fn from_raw_u64(&self, raw_value: u64) -> Result<ScaledNumber, ScaledError> {
        ScaledNumber::from_raw_u64(raw_value, self.precision)
    }

    /// Создает ScaledNumber из сырого значения i32 с точностью конвертера
    pub fn from_raw_i32(&self, raw_value: i32) -> Result<ScaledNumber, ScaledError> {
        ScaledNumber::from_raw_i32(raw_value, self.precision)
    }

    // Методы преобразования ИЗ ScaledNumber В другие типы ===
    // Все методы автоматически приводят к точности конвертера

    /// Преобразует ScaledNumber в f32 с приведением к точности конвертера
    pub fn to_f32(&self, value: ScaledNumber) -> f32 {
        let normalized = value.with_precision(self.precision)
            .unwrap_or(value); // Fallback если приведение невозможно
        normalized.to_f32()
    }

    /// Преобразует ScaledNumber в f64 с приведением к точности конвертера
    pub fn to_f64(&self, value: ScaledNumber) -> f64 {
        let normalized = value.with_precision(self.precision)
            .unwrap_or(value);
        normalized.to_f64()
    }

    /// Преобразует ScaledNumber в u32 с приведением к точности конвертера
    pub fn to_u32(&self, value: ScaledNumber) -> Result<u32, ScaledError> {
        let normalized = value.with_precision(self.precision)?;
        normalized.to_u32()
    }

    /// Преобразует ScaledNumber в u64 с приведением к точности конвертера
    pub fn to_u64(&self, value: ScaledNumber) -> Result<u64, ScaledError> {
        let normalized = value.with_precision(self.precision)?;
        normalized.to_u64()
    }

    /// Преобразует ScaledNumber в i32 с приведением к точности конвертера
    pub fn to_i32(&self, value: ScaledNumber) -> Result<i32, ScaledError> {
        let normalized = value.with_precision(self.precision)?;
        normalized.to_i32()
    }

    /// Преобразует ScaledNumber в i64 с приведением к точности конвертера
    pub fn to_i64(&self, value: ScaledNumber) -> Result<i64, ScaledError> {
        let normalized = value.with_precision(self.precision)?;
        normalized.to_i64()
    }

    /// Преобразует ScaledNumber в строку с точностью конвертера
    pub fn to_string(&self, value: ScaledNumber) -> String {
        let normalized = value.with_precision(self.precision)
            .unwrap_or(value);
        normalized.to_string()
    }

    // Утилитарные методы

    /// Нормализует ScaledNumber к точности конвертера
    pub fn normalize(&self, value: ScaledNumber) -> Result<ScaledNumber, ScaledError> {
        value.with_precision(self.precision)
    }

    /// Проверяет, соответствует ли точность ScaledNumber точности конвертера
    pub fn is_compatible(&self, value: &ScaledNumber) -> bool {
        value.precision() == self.precision
    }

    /// Приводит все переданные числа к единой точности конвертера
    pub fn align_precision(&self, values: &[ScaledNumber]) -> Result<Vec<ScaledNumber>, ScaledError> {
        values
            .iter()
            .map(|v| v.with_precision(self.precision))
            .collect()
    }
}
