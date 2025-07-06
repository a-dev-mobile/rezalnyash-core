//! Ошибки для работы с масштабированными числами

use std::fmt;

/// Ошибки при работе с масштабированными числами
#[derive(Debug, Clone, PartialEq)]
pub enum ScaledError {
    /// Разные масштабы при операции
    ScaleMismatch { left: u8, right: u8 },
    /// Переполнение при вычислениях
    Overflow,
    /// Деление на ноль
    DivisionByZero,
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
            ScaledError::DivisionByZero => write!(f, "Division by zero"),
            ScaledError::InvalidPrecision(p) => write!(f, "Invalid precision: {}", p),
            ScaledError::ParseError(s) => write!(f, "Parse error: {}", s),
            ScaledError::NegativeToUnsigned => write!(f, "Cannot convert negative number to unsigned type"),
            ScaledError::ValueTooLarge => write!(f, "Value too large for target type"),
        }
    }
}

impl std::error::Error for ScaledError {}
