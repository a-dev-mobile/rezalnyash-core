use rezalnyas_core::{log_info, logging::{init_logging, LogConfig, LogLevel}};



fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Инициализация логирования
    init_logging(LogConfig {
        level: LogLevel::Debug,
    });

    log_info!("Приложение запущено");

    Ok(())
}