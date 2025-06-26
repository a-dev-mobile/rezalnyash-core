use crate::logging::enums::LogLevel;
use crate::logging::structs::LogConfig;
use std::io::{self, Write};
use std::sync::{Mutex, OnceLock};

/// Глобальная конфигурация логирования
static LOGGER_CONFIG: OnceLock<Mutex<LogConfig>> = OnceLock::new();

/// Простой логгер
pub struct AppLogger;

impl AppLogger {
    /// Записать сообщение в лог
    pub fn log(level: LogLevel, message: &str) {
        let config = LOGGER_CONFIG
            .get()
            .and_then(|mutex| mutex.lock().ok())
            .map(|guard| guard.clone())
            .unwrap();

        // Проверяем, нужно ли выводить сообщение
        if level > config.level {
            return;
        }

        // Выводим в stderr для ошибок, в stdout для остального
        match level {
            LogLevel::Error => {
                let _ = writeln!(io::stderr(), "[{}] {}", level, message);
            }
            _ => {
                let _ = writeln!(io::stdout(), "[{}] {}", level, message);
            }
        }
    }
}

/// Инициализация логирования
pub fn init_logging(config: LogConfig) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    LOGGER_CONFIG
        .set(Mutex::new(config))
        .map_err(|_| "Logger already initialized")?;
    Ok(())
}
