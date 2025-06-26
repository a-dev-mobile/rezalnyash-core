//! Структуры для модуля логирования

use crate::logging::enums::LogLevel;

/// Простая конфигурация логирования
#[derive(Debug, Clone)]
pub struct LogConfig {
    /// Уровень логирования
    pub level: LogLevel,
}
