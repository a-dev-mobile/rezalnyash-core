//! Основная структура ScaledNumber и её методы

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

use super::errors::ScaledError;

/// Масштабированное число - дробное число, представленное как целое
///
/// # Примеры
///
/// ```rust
/// use rezalnyas_core::scaled_math::ScaledNumber;
///
/// // Создание из дробного числа с точностью 2 знака
/// let price = ScaledNumber::from_f64(12.34, 2).unwrap();
/// let tax = ScaledNumber::from_f64(1.23, 2).unwrap();
///
/// // Арифметические операции
/// let total = price + tax;
/// assert_eq!(total.to_f64(), 13.57);
///
/// // Преобразование в целые типы
/// let amount = ScaledNumber::from_f64(42.0, 0).unwrap();
/// assert_eq!(amount.to_u32().unwrap(), 42u32);
/// assert_eq!(amount.to_u64().unwrap(), 42u64);
///
/// // Сравнения точные
/// let a = ScaledNumber::from_f64(0.1, 3).unwrap();
/// let b = ScaledNumber::from_f64(0.2, 3).unwrap();
/// let c = ScaledNumber::from_f64(0.3, 3).unwrap();
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
    pub(crate) fn align_precision(&self, other: &Self) -> Result<(Self, Self), ScaledError> {
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
