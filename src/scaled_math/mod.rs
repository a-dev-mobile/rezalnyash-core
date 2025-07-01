//! Утилитный модуль для работы с масштабированными числами
//!
//! Позволяет работать с дробными числами как с целыми для высокой точности
//! в геометрических и финансовых вычислениях.

mod errors;
mod scaled_number;
mod precision_analyzer;
mod converter;

#[cfg(test)]
mod tests;

// Публичные экспорты
pub use errors::ScaledError;
pub use scaled_number::ScaledNumber;
pub use precision_analyzer::PrecisionAnalyzer;
pub use converter::ScaledConverter;
