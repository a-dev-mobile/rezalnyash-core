//! Утилитный модуль для работы с масштабированными числами
//!
//! Позволяет работать с дробными числами как с целыми для высокой точности
//! в геометрических и финансовых вычислениях.

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

/// Ошибки при работе с масштабированными числами
#[derive(Debug, Clone, PartialEq)]
pub enum ScaledError {
    /// Разные масштабы при операции
    ScaleMismatch { left: u8, right: u8 },
    /// Переполнение при вычислениях
    Overflow,
    /// Недопустимая точность
    InvalidPrecision(u8),
    /// Ошибка парсинга
    ParseError(String),
    /// Преобразование отрицательного числа в беззнаковый тип
    NegativeToUnsigned,
    /// Значение слишком большое для целевого типа
    ValueTooLarge,
}

impl fmt::Display for ScaledError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScaledError::ScaleMismatch { left, right } => {
                write!(f, "Scale mismatch: {} vs {}", left, right)
            }
            ScaledError::Overflow => write!(f, "Arithmetic overflow"),
            ScaledError::InvalidPrecision(p) => write!(f, "Invalid precision: {}", p),
            ScaledError::ParseError(s) => write!(f, "Parse error: {}", s),
            ScaledError::NegativeToUnsigned => write!(f, "Cannot convert negative number to unsigned type"),
            ScaledError::ValueTooLarge => write!(f, "Value too large for target type"),
        }
    }
}

impl std::error::Error for ScaledError {}

/// Масштабированное число - дробное число, представленное как целое
///
/// # Примеры
///
/// ```rust
/// use scaled_math::ScaledNumber;
///
/// // Создание из дробного числа с точностью 2 знака
/// let price = ScaledNumber::from_f64(12.34, 2)?;
/// let tax = ScaledNumber::from_f64(1.23, 2)?;
///
/// // Арифметические операции
/// let total = price + tax;
/// assert_eq!(total.to_f64(), 13.57);
///
/// // Преобразование в целые типы
/// let amount = ScaledNumber::from_f64(42.0, 0)?;
/// assert_eq!(amount.to_u32()?, 42u32);
/// assert_eq!(amount.to_u64()?, 42u64);
///
/// // Сравнения точные
/// let a = ScaledNumber::from_f64(0.1, 3)?;
/// let b = ScaledNumber::from_f64(0.2, 3)?;
/// let c = ScaledNumber::from_f64(0.3, 3)?;
/// assert_eq!(a + b, c); // Работает правильно!
/// ```
#[derive(Debug, Clone, Copy, Hash)]
pub struct ScaledNumber {
    /// Целое значение (исходное число * 10^precision)
    value: i64,
    /// Количество знаков после запятой
    precision: u8,
    /// Кэшированный масштабный коэффициент
    scale: i64,
}
// ручную реализацию PartialEq и Eq
impl PartialEq for ScaledNumber {
    fn eq(&self, other: &Self) -> bool {
        // Приводим к одинаковой точности и сравниваем значения
        if let Ok((left, right)) = self.align_precision(other) {
            left.value == right.value
        } else {
            // Если не можем привести - сравниваем как f64
            (self.to_f64() - other.to_f64()).abs() < f64::EPSILON
        }
    }
}
impl Eq for ScaledNumber {}
impl ScaledNumber {
    /// Максимальная поддерживаемая точность
    pub const MAX_PRECISION: u8 = 9;

    /// Создает новое масштабированное число из целого значения и точности
    pub fn new(value: i64, precision: u8) -> Result<Self, ScaledError> {
        if precision > Self::MAX_PRECISION {
            return Err(ScaledError::InvalidPrecision(precision));
        }

        let scale = 10_i64.pow(precision as u32);
        Ok(Self {
            value,
            precision,
            scale,
        })
    }

    /// Создает из f32 с заданной точностью
    pub fn from_f32(value: f32, precision: u8) -> Result<Self, ScaledError> {
        if precision > Self::MAX_PRECISION {
            return Err(ScaledError::InvalidPrecision(precision));
        }

        let scale = 10_i64.pow(precision as u32);
        let scaled_value = (value as f64 * scale as f64).round() as i64;

        Ok(Self {
            value: scaled_value,
            precision,
            scale,
        })
    }

    /// Создает из f64 с заданной точностью
    pub fn from_f64(value: f64, precision: u8) -> Result<Self, ScaledError> {
        if precision > Self::MAX_PRECISION {
            return Err(ScaledError::InvalidPrecision(precision));
        }

        let scale = 10_i64.pow(precision as u32);
        let scaled_value = (value * scale as f64).round() as i64;

        Ok(Self {
            value: scaled_value,
            precision,
            scale,
        })
    }

    /// Создает из u32
    pub fn from_u32(value: u32, precision: u8) -> Result<Self, ScaledError> {
        if precision > Self::MAX_PRECISION {
            return Err(ScaledError::InvalidPrecision(precision));
        }

        let scale = 10_i64.pow(precision as u32);
        let scaled_value = (value as i64)
            .checked_mul(scale)
            .ok_or(ScaledError::Overflow)?;

        Ok(Self {
            value: scaled_value,
            precision,
            scale,
        })
    }

    /// Создает из u64
    pub fn from_u64(value: u64, precision: u8) -> Result<Self, ScaledError> {
        if precision > Self::MAX_PRECISION {
            return Err(ScaledError::InvalidPrecision(precision));
        }

        let scale = 10_i64.pow(precision as u32);

        // Проверяем, что значение помещается в i64
        if value > i64::MAX as u64 {
            return Err(ScaledError::ValueTooLarge);
        }

        let scaled_value = (value as i64)
            .checked_mul(scale)
            .ok_or(ScaledError::Overflow)?;

        Ok(Self {
            value: scaled_value,
            precision,
            scale,
        })
    }

    /// Создает из строки с автоматическим определением точности
    pub fn from_str_auto(s: &str) -> Result<Self, ScaledError> {
        let s = s.trim();

        // Определяем точность по количеству знаков после точки
        let precision = if let Some(dot_pos) = s.find('.') {
            let decimal_part = &s[dot_pos + 1..];
            decimal_part.len() as u8
        } else {
            0
        };

        let value: f64 = s
            .parse()
            .map_err(|_| ScaledError::ParseError(s.to_string()))?;

        Self::from_f64(value, precision)
    }

    /// Создает из строки с заданной точностью
    pub fn from_str(s: &str, precision: u8) -> Result<Self, ScaledError> {
        let value: f64 = s
            .trim()
            .parse()
            .map_err(|_| ScaledError::ParseError(s.to_string()))?;

        Self::from_f64(value, precision)
    }

    /// Преобразует в f32
    pub fn to_f32(&self) -> f32 {
        self.value as f32 / self.scale as f32
    }

    /// Преобразует в f64
    pub fn to_f64(&self) -> f64 {
        self.value as f64 / self.scale as f64
    }

    /// Преобразует в u32
    pub fn to_u32(&self) -> Result<u32, ScaledError> {
        if self.value < 0 {
            return Err(ScaledError::NegativeToUnsigned);
        }

        let integer_value = self.value / self.scale;

        if integer_value > u32::MAX as i64 {
            return Err(ScaledError::ValueTooLarge);
        }

        Ok(integer_value as u32)
    }

    /// Преобразует в u64
    pub fn to_u64(&self) -> Result<u64, ScaledError> {
        if self.value < 0 {
            return Err(ScaledError::NegativeToUnsigned);
        }

        let integer_value = self.value / self.scale;
        Ok(integer_value as u64)
    }

    /// Преобразует в u32 с округлением
    pub fn to_u32_rounded(&self) -> Result<u32, ScaledError> {
        if self.value < 0 {
            return Err(ScaledError::NegativeToUnsigned);
        }

        let rounded_value = (self.value + self.scale / 2) / self.scale;

        if rounded_value > u32::MAX as i64 {
            return Err(ScaledError::ValueTooLarge);
        }

        Ok(rounded_value as u32)
    }

    /// Преобразует в u64 с округлением
    pub fn to_u64_rounded(&self) -> Result<u64, ScaledError> {
        if self.value < 0 {
            return Err(ScaledError::NegativeToUnsigned);
        }

        let rounded_value = (self.value + self.scale / 2) / self.scale;
        Ok(rounded_value as u64)
    }

    /// Возвращает внутреннее целое значение
    pub fn raw_value(&self) -> i64 {
        self.value
    }

    /// Возвращает точность
    pub fn precision(&self) -> u8 {
        self.precision
    }

    /// Возвращает масштабный коэффициент
    pub fn scale(&self) -> i64 {
        self.scale
    }

    /// Приводит к другой точности
    pub fn with_precision(&self, new_precision: u8) -> Result<Self, ScaledError> {
        if new_precision > Self::MAX_PRECISION {
            return Err(ScaledError::InvalidPrecision(new_precision));
        }

        if new_precision == self.precision {
            return Ok(*self);
        }

        let new_scale = 10_i64.pow(new_precision as u32);
        let new_value = if new_precision > self.precision {
            // Увеличиваем точность
            let factor = 10_i64.pow((new_precision - self.precision) as u32);
            self.value
                .checked_mul(factor)
                .ok_or(ScaledError::Overflow)?
        } else {
            // Уменьшаем точность (округляем)
            let factor = 10_i64.pow((self.precision - new_precision) as u32);
            (self.value + factor / 2) / factor
        };

        Ok(Self {
            value: new_value,
            precision: new_precision,
            scale: new_scale,
        })
    }

    /// Возвращает абсолютное значение
    pub fn abs(&self) -> Self {
        Self {
            value: self.value.abs(),
            precision: self.precision,
            scale: self.scale,
        }
    }

    /// Проверяет, что число равно нулю
    pub fn is_zero(&self) -> bool {
        self.value == 0
    }

    /// Проверяет, что число положительное
    pub fn is_positive(&self) -> bool {
        self.value > 0
    }

    /// Проверяет, что число отрицательное
    pub fn is_negative(&self) -> bool {
        self.value < 0
    }

    /// Возвращает максимальное из двух чисел
    pub fn max(self, other: Self) -> Result<Self, ScaledError> {
        let (left, right) = self.align_precision(&other)?;
        Ok(if left.value >= right.value {
            left
        } else {
            right
        })
    }

    /// Возвращает минимальное из двух чисел
    pub fn min(self, other: Self) -> Result<Self, ScaledError> {
        let (left, right) = self.align_precision(&other)?;
        Ok(if left.value <= right.value {
            left
        } else {
            right
        })
    }

    /// Приводит два числа к одинаковой точности
    fn align_precision(&self, other: &Self) -> Result<(Self, Self), ScaledError> {
        let max_precision = self.precision.max(other.precision);
        let left = self.with_precision(max_precision)?;
        let right = other.with_precision(max_precision)?;
        Ok((left, right))
    }

    /// Безопасное сложение с обработкой ошибок
    pub fn checked_add(self, other: Self) -> Result<Self, ScaledError> {
        let (left, right) = self.align_precision(&other)?;
        let result_value = left
            .value
            .checked_add(right.value)
            .ok_or(ScaledError::Overflow)?;

        Ok(Self {
            value: result_value,
            precision: left.precision,
            scale: left.scale,
        })
    }

    /// Безопасное вычитание с обработкой ошибок
    pub fn checked_sub(self, other: Self) -> Result<Self, ScaledError> {
        let (left, right) = self.align_precision(&other)?;
        let result_value = left
            .value
            .checked_sub(right.value)
            .ok_or(ScaledError::Overflow)?;

        Ok(Self {
            value: result_value,
            precision: left.precision,
            scale: left.scale,
        })
    }

    /// Безопасное умножение с обработкой ошибок
    pub fn checked_mul(self, other: Self) -> Result<Self, ScaledError> {
        let (left, right) = self.align_precision(&other)?;

        // При умножении масштабы складываются, поэтому нужно разделить на один масштаб
        let result_value = left
            .value
            .checked_mul(right.value)
            .ok_or(ScaledError::Overflow)?
            .checked_div(left.scale)
            .ok_or(ScaledError::Overflow)?;

        Ok(Self {
            value: result_value,
            precision: left.precision,
            scale: left.scale,
        })
    }

    /// Безопасное деление с обработкой ошибок
    pub fn checked_div(self, other: Self) -> Result<Self, ScaledError> {
        if other.value == 0 {
            return Err(ScaledError::Overflow); // Division by zero
        }

        let (left, right) = self.align_precision(&other)?;

        // При делении нужно сначала увеличить масштаб числителя
        let result_value = left
            .value
            .checked_mul(left.scale)
            .ok_or(ScaledError::Overflow)?
            .checked_div(right.value)
            .ok_or(ScaledError::Overflow)?;

        Ok(Self {
            value: result_value,
            precision: left.precision,
            scale: left.scale,
        })
    }
}

// Арифметические операции
impl Add for ScaledNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self.checked_add(other)
            .expect("ScaledNumber addition overflow")
    }
}

impl Sub for ScaledNumber {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self.checked_sub(other)
            .expect("ScaledNumber subtraction overflow")
    }
}

impl Mul for ScaledNumber {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        self.checked_mul(other)
            .expect("ScaledNumber multiplication overflow")
    }
}

impl Div for ScaledNumber {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        self.checked_div(other)
            .expect("ScaledNumber division error")
    }
}

// Операции присваивания
impl AddAssign for ScaledNumber {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for ScaledNumber {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

// Сравнения
impl PartialOrd for ScaledNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScaledNumber {
    fn cmp(&self, other: &Self) -> Ordering {
        // Приводим к одинаковой точности для сравнения
        if let Ok((left, right)) = self.align_precision(other) {
            left.value.cmp(&right.value)
        } else {
            // Если не можем привести - сравниваем как f64
            self.to_f64()
                .partial_cmp(&other.to_f64())
                .unwrap_or(Ordering::Equal)
        }
    }
}

// Отображение
impl fmt::Display for ScaledNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.precision == 0 {
            write!(f, "{}", self.value)
        } else {
            let integer_part = self.value / self.scale;
            let fractional_part = (self.value % self.scale).abs();
            write!(
                f,
                "{}.{:0width$}",
                integer_part,
                fractional_part,
                width = self.precision as usize
            )
        }
    }
}

/// Утилиты для анализа точности в коллекциях чисел
pub struct PrecisionAnalyzer;

impl PrecisionAnalyzer {
    /// Находит максимальную точность среди строковых представлений чисел
    pub fn max_decimal_places(numbers: &[&str]) -> u8 {
        numbers
            .iter()
            .map(|s| Self::count_decimal_places(s))
            .max()
            .unwrap_or(0)
    }

    /// Находит максимальную точность среди ScaledNumber
    pub fn max_precision(numbers: &[ScaledNumber]) -> u8 {
        numbers.iter().map(|n| n.precision()).max().unwrap_or(0)
    }

    /// Подсчитывает количество знаков после запятой в строке
    pub fn count_decimal_places(s: &str) -> u8 {
        if let Some(dot_pos) = s.find('.') {
            (s.len() - dot_pos - 1) as u8
        } else {
            0
        }
    }

    /// Подсчитывает количество цифр до запятой в строке
    pub fn count_integer_places(s: &str) -> u8 {
        let s = s.trim_start_matches('-'); // Убираем знак минус
        if let Some(dot_pos) = s.find('.') {
            dot_pos as u8
        } else {
            s.len() as u8
        }
    }

    /// Проверяет, не превышает ли общее количество цифр лимит
    pub fn validate_total_digits(numbers: &[&str], max_digits: u8) -> Result<u8, ScaledError> {
        let max_decimal = Self::max_decimal_places(numbers);
        let max_integer = numbers
            .iter()
            .map(|s| Self::count_integer_places(s))
            .max()
            .unwrap_or(0);

        if max_decimal + max_integer > max_digits {
            // Урезаем точность
            let adjusted_precision = max_digits.saturating_sub(max_integer);
            Ok(adjusted_precision)
        } else {
            Ok(max_decimal)
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let num = ScaledNumber::from_f64(12.34, 2).unwrap();
        assert_eq!(num.to_f64(), 12.34);
        assert_eq!(num.raw_value(), 1234);
        assert_eq!(num.precision(), 2);
    }

    #[test]
    fn test_from_u32() {
        let num = ScaledNumber::from_u32(42, 0).unwrap();
        assert_eq!(num.to_f64(), 42.0);
        assert_eq!(num.raw_value(), 42);

        let num_with_precision = ScaledNumber::from_u32(42, 2).unwrap();
        assert_eq!(num_with_precision.to_f64(), 42.0);
        assert_eq!(num_with_precision.raw_value(), 4200);
    }

    #[test]
    fn test_from_u64() {
        let num = ScaledNumber::from_u64(1000000, 0).unwrap();
        assert_eq!(num.to_f64(), 1000000.0);
        assert_eq!(num.raw_value(), 1000000);

        let num_with_precision = ScaledNumber::from_u64(123, 3).unwrap();
        assert_eq!(num_with_precision.to_f64(), 123.0);
        assert_eq!(num_with_precision.raw_value(), 123000);
    }

    #[test]
    fn test_from_u64_overflow() {
        // Тест переполнения при создании из очень большого u64
        let result = ScaledNumber::from_u64(u64::MAX, 0);
        assert!(matches!(result, Err(ScaledError::ValueTooLarge)));

        // Тест переполнения при умножении на масштаб
        let result = ScaledNumber::from_u64(i64::MAX as u64, 1);
        assert!(matches!(result, Err(ScaledError::Overflow)));
    }

    #[test]
    fn test_to_u32() {
        let num = ScaledNumber::from_f64(42.0, 0).unwrap();
        assert_eq!(num.to_u32().unwrap(), 42u32);

        let num_fractional = ScaledNumber::from_f64(42.7, 1).unwrap();
        assert_eq!(num_fractional.to_u32().unwrap(), 42u32); // Усечение

        // Тест отрицательного числа
        let negative = ScaledNumber::from_f64(-42.0, 0).unwrap();
        assert!(matches!(negative.to_u32(), Err(ScaledError::NegativeToUnsigned)));

        // Тест переполнения
        let large = ScaledNumber::from_f64(u32::MAX as f64 + 1.0, 0).unwrap();
        assert!(matches!(large.to_u32(), Err(ScaledError::ValueTooLarge)));
    }

    #[test]
    fn test_to_u64() {
        let num = ScaledNumber::from_f64(1000000.0, 0).unwrap();
        assert_eq!(num.to_u64().unwrap(), 1000000u64);

        let num_fractional = ScaledNumber::from_f64(123.456, 3).unwrap();
        assert_eq!(num_fractional.to_u64().unwrap(), 123u64); // Усечение

        // Тест отрицательного числа
        let negative = ScaledNumber::from_f64(-123.0, 0).unwrap();
        assert!(matches!(negative.to_u64(), Err(ScaledError::NegativeToUnsigned)));
    }

    #[test]
    fn test_to_u32_rounded() {
        let num_down = ScaledNumber::from_f64(42.3, 1).unwrap();
        assert_eq!(num_down.to_u32_rounded().unwrap(), 42u32);

        let num_up = ScaledNumber::from_f64(42.7, 1).unwrap();
        assert_eq!(num_up.to_u32_rounded().unwrap(), 43u32);

        let num_half = ScaledNumber::from_f64(42.5, 1).unwrap();
        assert_eq!(num_half.to_u32_rounded().unwrap(), 43u32);

        // Тест отрицательного числа
        let negative = ScaledNumber::from_f64(-42.0, 0).unwrap();
        assert!(matches!(negative.to_u32_rounded(), Err(ScaledError::NegativeToUnsigned)));
    }

    #[test]
    fn test_to_u64_rounded() {
        let num_down = ScaledNumber::from_f64(123.3, 1).unwrap();
        assert_eq!(num_down.to_u64_rounded().unwrap(), 123u64);

        let num_up = ScaledNumber::from_f64(123.7, 1).unwrap();
        assert_eq!(num_up.to_u64_rounded().unwrap(), 124u64);

        let num_half = ScaledNumber::from_f64(123.5, 1).unwrap();
        assert_eq!(num_half.to_u64_rounded().unwrap(), 124u64);

        // Тест отрицательного числа
        let negative = ScaledNumber::from_f64(-123.0, 0).unwrap();
        assert!(matches!(negative.to_u64_rounded(), Err(ScaledError::NegativeToUnsigned)));
    }

    #[test]
    fn test_arithmetic() {
        let a = ScaledNumber::from_f64(1.5, 1).unwrap();
        let b = ScaledNumber::from_f64(2.3, 1).unwrap();

        let sum = (a + b);
        assert_eq!(sum.to_f64(), 3.8);

        let diff = (a - b);
        assert_eq!(diff.to_f64(), -0.8);
    }

    #[test]
    fn test_precision_handling() {
        let a = ScaledNumber::from_f64(1.5, 1).unwrap();
        let b = ScaledNumber::from_f64(2.35, 2).unwrap();

        let sum = (a + b);
        assert_eq!(sum.to_f64(), 3.85);
        assert_eq!(sum.precision(), 2);
    }

    #[test]
    fn test_float_precision_issue() {
        // Демонстрация решения проблемы точности float
        let a = ScaledNumber::from_f64(0.1, 3).unwrap();
        let b = ScaledNumber::from_f64(0.2, 3).unwrap();
        let c = ScaledNumber::from_f64(0.3, 3).unwrap();

        assert_eq!((a + b), c);

        // А с float не работает:
        assert_ne!(0.1f64 + 0.2f64, 0.3f64);
    }

    #[test]
    fn test_converter_u32_u64() {
        let converter = ScaledConverter::new(2).unwrap();
        
        // Тест u32
        let u32_values = [100u32, 200u32, 300u32];
        let scaled = converter.convert_u32_slice(&u32_values).unwrap();
        assert_eq!(scaled.len(), 3);
        assert_eq!(scaled[0].to_f64(), 100.0);
        assert_eq!(scaled[0].precision(), 2);

        let back_to_u32 = converter.to_u32_vec(&scaled).unwrap();
        assert_eq!(back_to_u32, u32_values);

        // Тест u64
        let u64_values = [1000000u64, 2000000u64, 3000000u64];
        let scaled = converter.convert_u64_slice(&u64_values).unwrap();
        assert_eq!(scaled.len(), 3);
        assert_eq!(scaled[0].to_f64(), 1000000.0);
        
        let back_to_u64 = converter.to_u64_vec(&scaled).unwrap();
        assert_eq!(back_to_u64, u64_values);
    }

    #[test]
    fn test_converter_rounded() {
        let converter = ScaledConverter::new(1).unwrap();
        
        let values = [
            ScaledNumber::from_f64(42.3, 1).unwrap(),
            ScaledNumber::from_f64(42.7, 1).unwrap(),
            ScaledNumber::from_f64(42.5, 1).unwrap(),
        ];

        let u32_rounded = converter.to_u32_vec_rounded(&values).unwrap();
        assert_eq!(u32_rounded, [42u32, 43u32, 43u32]);

        let u64_rounded = converter.to_u64_vec_rounded(&values).unwrap();
        assert_eq!(u64_rounded, [42u64, 43u64, 43u64]);
    }

    #[test]
    fn test_converter() {
        let values = ["12.34", "56.7", "89.123"];
        let converter = ScaledConverter::from_strings(&values).unwrap();

        let scaled = converter.convert_string_slice(&values).unwrap();
        assert_eq!(scaled.len(), 3);
        assert_eq!(scaled[0].to_f64(), 12.34);
        assert_eq!(scaled[2].precision(), 3);
    }

    #[test]
    fn test_from_f32() {
        let num = ScaledNumber::from_f32(12.34f32, 2).unwrap();
        assert_eq!(num.to_f32(), 12.34f32);
        assert_eq!(num.precision(), 2);
    }

    #[test]
    fn test_from_str_auto() {
        let num1 = ScaledNumber::from_str_auto("12.34").unwrap();
        assert_eq!(num1.to_f64(), 12.34);
        assert_eq!(num1.precision(), 2);

        let num2 = ScaledNumber::from_str_auto("123").unwrap();
        assert_eq!(num2.to_f64(), 123.0);
        assert_eq!(num2.precision(), 0);

        let num3 = ScaledNumber::from_str_auto("0.12345").unwrap();
        assert_eq!(num3.to_f64(), 0.12345);
        assert_eq!(num3.precision(), 5);
    }

    #[test]
    fn test_from_str() {
        let num = ScaledNumber::from_str("12.34", 3).unwrap();
        assert_eq!(num.to_f64(), 12.34);
        assert_eq!(num.precision(), 3);
    }

    #[test]
    fn test_invalid_precision() {
        let result = ScaledNumber::from_f64(12.34, 15);
        assert!(matches!(result, Err(ScaledError::InvalidPrecision(15))));
    }

    #[test]
    fn test_parse_error() {
        let result = ScaledNumber::from_str("not_a_number", 2);
        assert!(matches!(result, Err(ScaledError::ParseError(_))));
    }

    #[test]
    fn test_scale() {
        let num = ScaledNumber::from_f64(12.34, 2).unwrap();
        assert_eq!(num.scale(), 100);
    }

    #[test]
    fn test_with_precision() {
        let num = ScaledNumber::from_f64(12.345, 3).unwrap();

        // Увеличиваем точность
        let higher = num.with_precision(5).unwrap();
        assert_eq!(higher.precision(), 5);
        assert_eq!(higher.to_f64(), 12.345);

        // Уменьшаем точность (округление)
        let lower = num.with_precision(2).unwrap();
        assert_eq!(lower.precision(), 2);
        assert_eq!(lower.to_f64(), 12.35); // Округлилось

        // Та же точность
        let same = num.with_precision(3).unwrap();
        assert_eq!(same.precision(), 3);
        assert_eq!(same.to_f64(), 12.345);
    }

    #[test]
    fn test_abs() {
        let positive = ScaledNumber::from_f64(12.34, 2).unwrap();
        let negative = ScaledNumber::from_f64(-12.34, 2).unwrap();

        assert_eq!(positive.abs().to_f64(), 12.34);
        assert_eq!(negative.abs().to_f64(), 12.34);
    }

    #[test]
    fn test_is_zero() {
        let zero = ScaledNumber::from_f64(0.0, 2).unwrap();
        let non_zero = ScaledNumber::from_f64(0.01, 2).unwrap();

        assert!(zero.is_zero());
        assert!(!non_zero.is_zero());
    }

    #[test]
    fn test_is_positive() {
        let positive = ScaledNumber::from_f64(12.34, 2).unwrap();
        let negative = ScaledNumber::from_f64(-12.34, 2).unwrap();
        let zero = ScaledNumber::from_f64(0.0, 2).unwrap();

        assert!(positive.is_positive());
        assert!(!negative.is_positive());
        assert!(!zero.is_positive());
    }

    #[test]
    fn test_is_negative() {
        let positive = ScaledNumber::from_f64(12.34, 2).unwrap();
        let negative = ScaledNumber::from_f64(-12.34, 2).unwrap();
        let zero = ScaledNumber::from_f64(0.0, 2).unwrap();

        assert!(!positive.is_negative());
        assert!(negative.is_negative());
        assert!(!zero.is_negative());
    }

    #[test]
    fn test_max_min() {
        let a = ScaledNumber::from_f64(12.34, 2).unwrap();
        let b = ScaledNumber::from_f64(56.78, 2).unwrap();

        assert_eq!(a.max(b).unwrap().to_f64(), 56.78);
        assert_eq!(a.min(b).unwrap().to_f64(), 12.34);
    }

    #[test]
    fn test_max_min_different_precision() {
        let a = ScaledNumber::from_f64(12.3, 1).unwrap();
        let b = ScaledNumber::from_f64(12.35, 2).unwrap();

        assert_eq!(a.max(b).unwrap().to_f64(), 12.35);
        assert_eq!(a.min(b).unwrap().to_f64(), 12.30);
    }

    #[test]
    fn test_multiplication() {
        let a = ScaledNumber::from_f64(2.5, 1).unwrap();
        let b = ScaledNumber::from_f64(4.0, 1).unwrap();

        let result = a * b;
        assert_eq!(result.to_f64(), 10.0);
    }

    #[test]
    fn test_division() {
        let a = ScaledNumber::from_f64(10.0, 1).unwrap();
        let b = ScaledNumber::from_f64(2.5, 1).unwrap();

        let result = a / b;
        assert_eq!(result.to_f64(), 4.0);
    }

    #[test]
    #[should_panic(expected = "ScaledNumber division error")]
    fn test_division_by_zero() {
        let a = ScaledNumber::from_f64(10.0, 1).unwrap();
        let zero = ScaledNumber::from_f64(0.0, 1).unwrap();

        let _result = a / zero; // Должно паниковать
    }

    #[test]
    fn test_checked_division_by_zero() {
        let a = ScaledNumber::from_f64(10.0, 1).unwrap();
        let zero = ScaledNumber::from_f64(0.0, 1).unwrap();

        let result = a.checked_div(zero);
        assert!(matches!(result, Err(ScaledError::Overflow)));
    }

    #[test]
    fn test_add_assign() {
        let mut a = ScaledNumber::from_f64(1.5, 1).unwrap();
        let b = ScaledNumber::from_f64(2.3, 1).unwrap();

        a += b;
        assert_eq!(a.to_f64(), 3.8);
    }

    #[test]
    fn test_sub_assign() {
        let mut a = ScaledNumber::from_f64(5.0, 1).unwrap();
        let b = ScaledNumber::from_f64(2.3, 1).unwrap();

        a -= b;
        assert_eq!(a.to_f64(), 2.7);
    }

    #[test]
    fn test_ordering() {
        let a = ScaledNumber::from_f64(12.34, 2).unwrap();
        let b = ScaledNumber::from_f64(56.78, 2).unwrap();
        let c = ScaledNumber::from_f64(12.34, 2).unwrap();

        assert!(a < b);
        assert!(b > a);
        assert!(a <= c);
        assert!(a >= c);
        assert_eq!(a, c);
    }

    #[test]
    fn test_ordering_different_precision() {
        let a = ScaledNumber::from_f64(12.3, 1).unwrap();
        let b = ScaledNumber::from_f64(12.3, 2).unwrap();

        // Отладочная информация
        println!(
            "a: value={}, precision={}, scale={}",
            a.raw_value(),
            a.precision(),
            a.scale()
        );
        println!(
            "b: value={}, precision={}, scale={}",
            b.raw_value(),
            b.precision(),
            b.scale()
        );
        println!("a.to_f64() = {}", a.to_f64());
        println!("b.to_f64() = {}", b.to_f64());

        assert_eq!(a, b);
    }

    #[test]
    fn test_ordering_different_precision_explicit() {
        // Создаем числа через new() для более точного контроля
        let a = ScaledNumber::new(123, 1).unwrap(); // 12.3
        let b = ScaledNumber::new(1230, 2).unwrap(); // 12.30

        assert_eq!(a.to_f64(), 12.3);
        assert_eq!(b.to_f64(), 12.3);
        assert_eq!(a, b);
    }

    #[test]
    fn test_align_precision_manually() {
        let a = ScaledNumber::new(123, 1).unwrap(); // 12.3
        let b = ScaledNumber::new(1230, 2).unwrap(); // 12.30

        // Тестируем выравнивание точности через max/min (которые используют align_precision)
        let max_result = a.max(b).unwrap();
        let min_result = a.min(b).unwrap();

        assert_eq!(max_result.to_f64(), 12.3);
        assert_eq!(min_result.to_f64(), 12.3);
        assert_eq!(max_result, min_result);
    }

    #[test]
    fn test_display() {
        let num1 = ScaledNumber::from_f64(12.34, 2).unwrap();
        assert_eq!(format!("{}", num1), "12.34");

        let num2 = ScaledNumber::from_f64(123.0, 0).unwrap();
        assert_eq!(format!("{}", num2), "123");

        let num3 = ScaledNumber::from_f64(0.005, 3).unwrap();
        assert_eq!(format!("{}", num3), "0.005");

        let num4 = ScaledNumber::from_f64(-12.34, 2).unwrap();
        assert_eq!(format!("{}", num4), "-12.34");
    }

    #[test]
    fn test_error_display() {
        let err1 = ScaledError::ScaleMismatch { left: 2, right: 3 };
        assert_eq!(format!("{}", err1), "Scale mismatch: 2 vs 3");

        let err2 = ScaledError::Overflow;
        assert_eq!(format!("{}", err2), "Arithmetic overflow");

        let err3 = ScaledError::InvalidPrecision(15);
        assert_eq!(format!("{}", err3), "Invalid precision: 15");

        let err4 = ScaledError::ParseError("abc".to_string());
        assert_eq!(format!("{}", err4), "Parse error: abc");

        let err5 = ScaledError::NegativeToUnsigned;
        assert_eq!(format!("{}", err5), "Cannot convert negative number to unsigned type");

        let err6 = ScaledError::ValueTooLarge;
        assert_eq!(format!("{}", err6), "Value too large for target type");
    }

    #[test]
    fn test_precision_analyzer_max_decimal_places() {
        let numbers = ["12.34", "56.789", "1.2"];
        assert_eq!(PrecisionAnalyzer::max_decimal_places(&numbers), 3);

        let integers = ["12", "56", "78"];
        assert_eq!(PrecisionAnalyzer::max_decimal_places(&integers), 0);
    }

    #[test]
    fn test_precision_analyzer_max_precision() {
        let numbers = [
            ScaledNumber::from_f64(12.34, 2).unwrap(),
            ScaledNumber::from_f64(56.789, 3).unwrap(),
            ScaledNumber::from_f64(1.2, 1).unwrap(),
        ];
        assert_eq!(PrecisionAnalyzer::max_precision(&numbers), 3);
    }

    #[test]
    fn test_precision_analyzer_count_decimal_places() {
        assert_eq!(PrecisionAnalyzer::count_decimal_places("12.34"), 2);
        assert_eq!(PrecisionAnalyzer::count_decimal_places("123"), 0);
        assert_eq!(PrecisionAnalyzer::count_decimal_places("0.12345"), 5);
    }

    #[test]
    fn test_precision_analyzer_count_integer_places() {
        assert_eq!(PrecisionAnalyzer::count_integer_places("12.34"), 2);
        assert_eq!(PrecisionAnalyzer::count_integer_places("123"), 3);
        assert_eq!(PrecisionAnalyzer::count_integer_places("-123.45"), 3);
        assert_eq!(PrecisionAnalyzer::count_integer_places("0.12345"), 1);
    }

    #[test]
    fn test_precision_analyzer_validate_total_digits() {
        let numbers = ["123.45", "67.890"];

        // Общее количество цифр не превышает лимит
        let result = PrecisionAnalyzer::validate_total_digits(&numbers, 10);
        assert_eq!(result.unwrap(), 3);

        // Превышает лимит - должна урезаться точность
        let result = PrecisionAnalyzer::validate_total_digits(&numbers, 4);
        assert_eq!(result.unwrap(), 1); // 3 цифры до запятой + 1 после = 4 всего
    }

    #[test]
    fn test_scaled_converter_new() {
        let converter = ScaledConverter::new(2).unwrap();
        assert_eq!(converter.precision, 2);

        let invalid = ScaledConverter::new(15);
        assert!(matches!(invalid, Err(ScaledError::InvalidPrecision(15))));
    }

    #[test]
    fn test_scaled_converter_from_strings() {
        let strings = ["12.34", "56.789", "1.2"];
        let converter = ScaledConverter::from_strings(&strings).unwrap();
        assert_eq!(converter.precision, 3);
    }

    #[test]
    fn test_scaled_converter_convert_f32_slice() {
        let converter = ScaledConverter::new(2).unwrap();
        let values = [12.34f32, 56.78f32];

        let result = converter.convert_f32_slice(&values).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].to_f32(), 12.34f32);
        assert_eq!(result[1].to_f32(), 56.78f32);
    }

    #[test]
    fn test_scaled_converter_convert_f64_slice() {
        let converter = ScaledConverter::new(2).unwrap();
        let values = [12.34, 56.78];

        let result = converter.convert_f64_slice(&values).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].to_f64(), 12.34);
        assert_eq!(result[1].to_f64(), 56.78);
    }

    #[test]
    fn test_scaled_converter_convert_string_slice() {
        let converter = ScaledConverter::new(2).unwrap();
        let values = ["12.34", "56.78"];

        let result = converter.convert_string_slice(&values).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].to_f64(), 12.34);
        assert_eq!(result[1].to_f64(), 56.78);
    }

    #[test]
    fn test_scaled_converter_to_f32_vec() {
        let converter = ScaledConverter::new(2).unwrap();
        let scaled = [
            ScaledNumber::from_f64(12.34, 2).unwrap(),
            ScaledNumber::from_f64(56.78, 2).unwrap(),
        ];

        let result = converter.to_f32_vec(&scaled);
        assert_eq!(result, [12.34f32, 56.78f32]);
    }

    #[test]
    fn test_scaled_converter_to_f64_vec() {
        let converter = ScaledConverter::new(2).unwrap();
        let scaled = [
            ScaledNumber::from_f64(12.34, 2).unwrap(),
            ScaledNumber::from_f64(56.78, 2).unwrap(),
        ];

        let result = converter.to_f64_vec(&scaled);
        assert_eq!(result, [12.34, 56.78]);
    }

    #[test]
    fn test_overflow_protection() {
        // Тест на переполнение при создании с большой точностью
        let max_i64 = i64::MAX;
        let result = ScaledNumber::new(max_i64, ScaledNumber::MAX_PRECISION);
        assert!(result.is_ok());

        // Тест на переполнение при изменении точности
        let num = ScaledNumber::new(i64::MAX / 1000, 3).unwrap();
        let result = num.with_precision(9);
        assert!(matches!(result, Err(ScaledError::Overflow)));
    }

    #[test]
    fn test_edge_cases() {
        // Очень маленькие числа
        let tiny = ScaledNumber::from_f64(0.000000001, 9).unwrap();
        assert_eq!(tiny.raw_value(), 1);

        // Числа на границе точности
        let boundary = ScaledNumber::from_f64(999999999.999999999, 9).unwrap();
        assert!(boundary.to_f64() > 999999999.0);

        // Отрицательные числа
        let negative = ScaledNumber::from_f64(-123.456, 3).unwrap();
        assert_eq!(negative.raw_value(), -123456);
        assert_eq!(negative.to_f64(), -123.456);
    }

    #[test]
    fn test_u32_u64_edge_cases() {
        // Тест максимальных значений
        let max_u32 = ScaledNumber::from_u32(u32::MAX, 0).unwrap();
        assert_eq!(max_u32.to_u32().unwrap(), u32::MAX);

        // Тест нуля
        let zero = ScaledNumber::from_u32(0, 2).unwrap();
        assert_eq!(zero.to_u32().unwrap(), 0);
        assert_eq!(zero.to_u64().unwrap(), 0);

        // Тест очень больших чисел с малой точностью
        let large = ScaledNumber::from_u64(1_000_000_000_000u64, 0).unwrap();
        assert_eq!(large.to_u64().unwrap(), 1_000_000_000_000u64);

        // Тест дробных частей
        let fractional = ScaledNumber::from_f64(42.999, 3).unwrap();
        assert_eq!(fractional.to_u32().unwrap(), 42); // Усечение
        assert_eq!(fractional.to_u32_rounded().unwrap(), 43); // Округление
    }
}