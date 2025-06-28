use rayon::vec;
use rezalnyas_core::{
    enums::{
        cut_orientation_preference::CutOrientationPreference,
        optimization_priority::OptimizationPriority,
    },
    log_debug, log_error, log_info, log_warn,
    logging::{init_logging, LogConfig, LogLevel},
    models::{
        calculation_request::CalculationRequest, configuration::structs::Configuration,
        panel::structs::Panel, performance_thresholds::PerformanceThresholds,
        tile_dimensions::TileDimensions,
    },
    scaled_math::{PrecisionAnalyzer, ScaledConverter, ScaledNumber},
    CutListOptimizerService, CuttingRequest, Material, OptimizationConfig, OptimizationStrategy,
};
const MAX_ALLOWED_DIGITS: u8 = 6;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🐛 DEBUG MODE: Single-threaded optimization");
    println!(
        "💻 Available cores: {}",
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
    );

    // Инициализация логирования
    init_logging(LogConfig {
        level: LogLevel::Debug,
    });

    // Примеры использования
    log_info!("Приложение запущено");
    log_debug!("Отладочная информация");
    log_warn!("Предупреждение");
    log_error!("Ошибка");

    let panels: Vec<Panel> = vec![
        Panel::new(1, "55.123".to_string(), "45.0".to_string(), 1),
        Panel::new(2, "35.12".to_string(), "25.0".to_string(), 1),
        Panel::new(3, "25.1".to_string(), "15.0".to_string(), 1),
        Panel::new(4, "15.000".to_string(), "20.0".to_string(), 1),
        Panel::new(5, "40.0".to_string(), "30.0".to_string(), 1),
    ];
    let stock_panels: Vec<Panel> = vec![Panel::new(1, "90.0255".to_string(), "120.01".to_string(), 1)];

    let config = Configuration {
        cut_thickness: 0.0,           // Точная толщина реза
        use_single_stock_unit: false, // Разрешаем использовать разные листы
        optimization_factor: 2,
        // in java = 0
        optimization_priority: vec![
            // Приоритеты оптимизации
            OptimizationPriority::MostTiles,
            OptimizationPriority::LeastWastedArea,
            OptimizationPriority::LeastNbrCuts,
            OptimizationPriority::LeastNbrMosaics,
            OptimizationPriority::BiggestUnusedTileArea,
            OptimizationPriority::MostHvDiscrepancy,
        ],
        cut_orientation_preference: CutOrientationPreference::Both, // Все направления резов

        consider_orientation: false, // Учитывать ориентацию волокон
        min_trim_dimension: 0.0, //  это минимальный полезный размер остатка в любом направлении, в тех же единицах, что и остальные размеры (мм, см, дюймы и т.д.).
        performance_thresholds: PerformanceThresholds {
            max_simultaneous_tasks: 1,   // Максимум потоков
            max_simultaneous_threads: 1, // Максимум потоков на задачу
            thread_check_interval: 100,
        },
    };
    let request = CalculationRequest {
        configuration: config,
        panels,
        stock_panels,
    };

    // submitTask в основном валидация входных данных и подготовка задачи
    // compute
    let panels = &request.panels;
    let stock_panels = &request.stock_panels;
    let configuration = &request.configuration;

    // Вычисление максимального количества знаков после запятой
    let max_decimal_places_panels = Panel::get_max_decimal_places(panels);
    let max_decimal_places_stock = Panel::get_max_decimal_places(stock_panels);

    // Получение точности для толщины реза и минимального размера обрезки
    let cut_thickness_precision =
        PrecisionAnalyzer::count_decimal_places(&configuration.cut_thickness.to_string());
    let min_trim_precision =
        PrecisionAnalyzer::count_decimal_places(&configuration.min_trim_dimension.to_string());

    // Определение максимальной точности

    let max_decimal_places = [
        max_decimal_places_panels,
        max_decimal_places_stock,
        cut_thickness_precision,
        min_trim_precision,
    ]
    .iter()
    .max()
    .copied()
    .unwrap_or(0);

    // Вычисление максимального количества цифр до запятой
    let max_integer_places_panels = Panel::get_max_integer_places(panels);
    let max_integer_places_stock = Panel::get_max_integer_places(stock_panels);

    let cut_thickness_integer =
        PrecisionAnalyzer::count_integer_places(&configuration.cut_thickness.to_string());
    let min_trim_integer =
        PrecisionAnalyzer::count_integer_places(&configuration.min_trim_dimension.to_string());

    let max_integer_places = [
        max_integer_places_panels,
        max_integer_places_stock,
        cut_thickness_integer,
        min_trim_integer,
    ]
    .iter()
    .max()
    .copied()
    .unwrap_or(0);

    // Проверка на превышение максимального количества цифр
    // iMax2 - final_precision
    let final_precision = if max_decimal_places + max_integer_places > MAX_ALLOWED_DIGITS {
        log_warn!(
            "Maximum allowed digits exceeded: maxDecimalPlaces[{}] maxIntegerPlaces[{}] maxAllowedDigits[{}]",
            max_decimal_places, max_integer_places, MAX_ALLOWED_DIGITS
        );
        MAX_ALLOWED_DIGITS.saturating_sub(max_integer_places).max(0)
    } else {
        max_decimal_places
    };

    // Создание конвертера с определенной точностью
    let converter = ScaledConverter::new(final_precision)?;
    // dPow - scale_factor
    let scale_factor = 10_i64.pow(final_precision as u32);
    // Создание списков для результатов
    let mut tiles = Vec::new();
    let mut stock_tiles = Vec::new();
    // Обработка панелей (tiles)
    for panel in panels {
        for _ in 0..panel.count {
            let width_scaled = ScaledNumber::from_str(&panel.width, final_precision)?;
            let height_scaled = ScaledNumber::from_str(&panel.height, final_precision)?;

            let tile = TileDimensions {
                id: panel.id,
                width: width_scaled.raw_value() as u32,
                height: height_scaled.raw_value() as u32,
                orientation: panel.orientation,
                is_rotated: false,
            };

            tiles.push(tile);
        }
    }

    // Обработка складских панелей (stock tiles)
    for stock_panel in stock_panels {
        for _ in 0..stock_panel.count {
            let width_scaled = ScaledNumber::from_str(&stock_panel.width, final_precision)?;
            let height_scaled = ScaledNumber::from_str(&stock_panel.height, final_precision)?;

            let stock_tile = TileDimensions {
                id: stock_panel.id,
                width: width_scaled.raw_value() as u32,
                height: height_scaled.raw_value() as u32,

                orientation: stock_panel.orientation,
                is_rotated: false,
            };

            stock_tiles.push(stock_tile);
        }
    }

    Ok(())
}
