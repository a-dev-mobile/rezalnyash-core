//! Утилиты для анализа точности в коллекциях чисел

use super::errors::ScaledError;
use super::scaled_number::ScaledNumber;

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
