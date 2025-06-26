//! Удобные макросы для логирования

use crate::logging::enums::LogLevel;
use crate::logging::init::AppLogger;

/// Макрос для логирования на уровне ERROR
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::logging::init::SimpleLogger::log(
            $crate::logging::enums::LogLevel::Error,
            &format!($($arg)*)
        );
    };
}

/// Макрос для логирования на уровне WARN
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        $crate::logging::init::SimpleLogger::log(
            $crate::logging::enums::LogLevel::Warn,
            &format!($($arg)*)
        );
    };
}

/// Макрос для логирования на уровне INFO
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::logging::init::SimpleLogger::log(
            $crate::logging::enums::LogLevel::Info,
            &format!($($arg)*)
        );
    };
}

/// Макрос для логирования на уровне DEBUG
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        $crate::logging::init::SimpleLogger::log(
            $crate::logging::enums::LogLevel::Debug,
            &format!($($arg)*)
        );
    };
}

/// Простые функции для логирования
pub fn error(message: &str) {
    AppLogger::log(LogLevel::Error, message);
}

pub fn warn(message: &str) {
    AppLogger::log(LogLevel::Warn, message);
}

pub fn info(message: &str) {
    AppLogger::log(LogLevel::Info, message);
}

pub fn debug(message: &str) {
    AppLogger::log(LogLevel::Debug, message);
}
