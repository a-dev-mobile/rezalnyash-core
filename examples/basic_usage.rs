use rezalnyas_core::{
    enums::{CutOrientationPreference, StatusCode}, log_debug, log_error, log_info, log_warn, logging::{init_logging, LogConfig, LogLevel}, models::{
        calculation_request::{CalculationRequest, Edge, Panel}, calculation_response::CalculationResponse, client_info::ClientInfo, configuration::Configuration, performance_thresholds::PerformanceThresholds, watch_dog::{CutListLogger, DefaultCutListLogger}
    }, services::{CutListOptimizerService, CutListOptimizerServiceImpl}
};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Инициализация логирования
    init_logging(LogConfig {
        level: LogLevel::Debug,
    });

    log_info!("=== Тест ===");

    // Инициализируем сервис точно как в Java
    let mut optimizer = CutListOptimizerServiceImpl::new();
    optimizer.init(8); // Фиксированно 8 потоков
    
    // Создаем консольный логгер
    let console_logger: Arc<dyn CutListLogger> = Arc::new(DefaultCutListLogger);
    optimizer.set_cut_list_logger(console_logger);
    
    // Настройки
    optimizer.set_allow_multiple_tasks_per_client(true); // Множественные задачи
    
    // Создаем запрос
    let request = create_request()?;
    
    log_info!("Отправляем задачу с настройками...");
    
    // Отправляем задачу
    let result = optimizer.submit_task(request);
    
    if result.status_code() == StatusCode::Ok.string_value() {
        if let Some(task_id) = result.task_id() {
            log_info!("Задача принята. ID: {}", task_id);
            
            // Ждем выполнения
            wait_for_completion(&optimizer, task_id)?;
        } else {
            log_error!("Ошибка отправки задачи: task_id отсутствует");
        }
    } else {
        log_error!("Ошибка отправки задачи: {}", result.status_code());
    }

    Ok(())
}

fn create_request() -> Result<CalculationRequest, Box<dyn std::error::Error>> {
    let mut request = CalculationRequest::new();
    
    // Создаем информацию о клиенте
    let client_info = ClientInfo::with_id("client".to_string())
        .device("Desktop".to_string()) // Эмулируем устройство
        .device_id("test-device-id".to_string());
    
    request.set_client_info(Some(client_info));
    
    // Более сложный набор деталей
    let mut panels = Vec::new();
    
    // Деталь 1: 150.5x100.25 (2 шт)
    let panel1 = Panel::new(
        1,
        "150.5".to_string(),
        "100.25".to_string(),
        2,
        "DEFAULT_MATERIAL".to_string(),
        true,
        0,
        Some("Деталь_1".to_string()),
        None,
    );
    panels.push(panel1);
    
    // Деталь 2: 80.75x60.5 (3 шт)
    let panel2 = Panel::new(
        2,
        "80.75".to_string(),
        "60.5".to_string(),
        3,
        "DEFAULT_MATERIAL".to_string(),
        true,
        0,
        Some("Деталь_2".to_string()),
        None,
    );
    panels.push(panel2);
    
    // Деталь 3: 120.0x45.75 (1 шт)
    let panel3 = Panel::new(
        3,
        "120.0".to_string(),
        "45.75".to_string(),
        1,
        "DEFAULT_MATERIAL".to_string(),
        true,
        0,
        Some("Деталь_3".to_string()),
        None,
    );
    panels.push(panel3);
    
    // Деталь 4: 95.25x75.5 (2 шт)
    let panel4 = Panel::new(
        4,
        "95.25".to_string(),
        "75.5".to_string(),
        2,
        "DEFAULT_MATERIAL".to_string(),
        true,
        0,
        Some("Деталь_4".to_string()),
        None,
    );
    panels.push(panel4);
    
    // Деталь 5: 65.5x85.25 (1 шт)
    let panel5 = Panel::new(
        5,
        "65.5".to_string(),
        "85.25".to_string(),
        1,
        "DEFAULT_MATERIAL".to_string(),
        true,
        0,
        Some("Деталь_5".to_string()),
        None,
    );
    panels.push(panel5);
    
    // Деталь 6: 110.75x55.0 (2 шт)
    let panel6 = Panel::new(
        6,
        "110.75".to_string(),
        "55.0".to_string(),
        2,
        "DEFAULT_MATERIAL".to_string(),
        true,
        0,
        Some("Деталь_6".to_string()),
        None,
    );
    panels.push(panel6);
    
    // Деталь 7: 40.25x90.5 (3 шт)
    let panel7 = Panel::new(
        7,
        "40.25".to_string(),
        "90.5".to_string(),
        3,
        "DEFAULT_MATERIAL".to_string(),
        true,
        0,
        Some("Деталь_7".to_string()),
        None,
    );
    panels.push(panel7);
    
    // Деталь 8: 130.0x35.75 (1 шт)
    let panel8 = Panel::new(
        8,
        "130.0".to_string(),
        "35.75".to_string(),
        1,
        "DEFAULT_MATERIAL".to_string(),
        true,
        0,
        Some("Деталь_8".to_string()),
        None,
    );
    panels.push(panel8);
    
    request.set_panels(panels);
    
    // Одна заготовка
    let mut stock_panels = Vec::new();
    let stock_panel = Panel::new(
        1,
        "400.0".to_string(),
        "300.0".to_string(),
        1,
        "DEFAULT_MATERIAL".to_string(),
        true,
        0,
        Some("Заготовка_1".to_string()),
        None,
    );
    stock_panels.push(stock_panel);
    
    request.set_stock_panels(stock_panels);
    
    // Настройки
    let mut config = Configuration::default();
    config.set_cut_thickness(Some("0".to_string())); // Точная толщина реза
    config.set_use_single_stock_unit(false); // Разрешаем использовать разные листы
    config.set_optimization_factor(2.0); // МАКСИМАЛЬНЫЙ фактор оптимизации = 2
    config.set_optimization_priority(0); // Приоритет
    config.set_cut_orientation_preference(CutOrientationPreference::Both); // Все направления резов
    config.set_consider_orientation(false); // Учитываем ориентацию волокон
    config.set_min_trim_dimension(Some("0".to_string())); // Разумный минимальный отход

    // Настройка производительности
    let max_threads = num_cpus::get() * 2;
    // let max_threads = num_cpus::get() * 2;
    let mut thresholds = PerformanceThresholds::new();
    thresholds.set_max_simultaneous_threads(max_threads as u32)?;
    thresholds.set_thread_check_interval(1000)?; // Реже проверяем, больше вычисляем
    thresholds.set_max_simultaneous_tasks(1)?;
    
    config.set_performance_thresholds(Some(thresholds));
    request.set_configuration(Some(config));
    
    log_info!("Создан запрос:");
    log_info!("- Деталей: {}", request.panels().len());
    log_info!("- Заготовка: {}x{}", 
        request.stock_panels().get(0).map(|p| p.width()).unwrap_or("?"),
        request.stock_panels().get(0).map(|p| p.height()).unwrap_or("?")
    );
    
    Ok(request)
}

fn wait_for_completion(
    optimizer: &CutListOptimizerServiceImpl,
    task_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    log_info!("Ожидание завершения задачи...");
    
    let max_attempts = 1800; // 5 минут
    let mut attempts = 0;
    let mut _last_progress = -1;
    let start_time = std::time::Instant::now();
    
    while attempts < max_attempts {
        thread::sleep(Duration::from_millis(100));
        attempts += 1;
        
        if let Some(status) = optimizer.get_task_status(task_id) {
            if attempts % 50 == 0 {
                let elapsed_seconds = start_time.elapsed().as_secs();
                log_info!("Статус: {}, прогресс: {}%, время: {}с",
                    status.get_status().unwrap_or("UNKNOWN"),
                    status.getpercentagedone(),
                    elapsed_seconds
                );
            }
            
            match status.get_status().as_deref() {
                Some("FINISHED") | Some("Finished") => {
                    let total_seconds = start_time.elapsed().as_secs();
                    log_info!("\n=== Задача выполнена за {} секунд! ===", total_seconds);
                    print_solution(&status.get_solution().unwrap());
                    generate_html_visualization(&status.get_solution().unwrap())?;
                    break;
                }
                Some(s) if s == "ERROR" || s == "Error" || s == "TERMINATED" || s == "Terminated" || s == "STOPPED" || s == "Stopped" => {
                    log_error!("Задача завершена с ошибкой: {}", s);
                    break;
                }
                _ => {
                    // Продолжаем ждать
                }
            }
        } else {
            log_error!("Не удается получить статус задачи");
            break;
        }
    }
    
    Ok(())
}

fn print_solution(solution: &CalculationResponse) {
    log_info!("\n=== Результат оптимизации ===");
    log_info!("Общая использованная площадь: {:.2}", solution.total_used_area);
    log_info!("Общая потерянная площадь: {:.2}", solution.total_wasted_area);
    log_info!("Коэффициент использования: {:.2}%", solution.total_used_area_ratio * 100.0);
    log_info!("Количество резов: {}", solution.total_nbr_cuts);
    log_info!("Общая длина резов: {:.2}", solution.total_cut_length);
    log_info!("Время выполнения: {} мс", solution.elapsed_time);
    
    log_info!("\n=== Мозаики (листы с раскроем) ===");
    for (i, mosaic) in solution.mosaics.iter().enumerate() {
        log_info!("Лист {}:", i + 1);
        log_info!("  Использование: {:.2}% ({:.2}/{:.2})",
            mosaic.used_area_ratio * 100.0,
            mosaic.used_area,
            mosaic.used_area + mosaic.wasted_area
        );
        
        if let Some(panels) = &mosaic.panels {
            for panel in panels {
                log_info!("    {:.1}x{:.1} x{} [{}]",
                    panel.width,
                    panel.height,
                    panel.count,
                    panel.label.as_deref().unwrap_or("")
                );
            }
        }
    }
    
    if !solution.no_fit_panels.is_empty() {
        log_info!("\n=== Неразмещенные детали ===");
        for no_fit in &solution.no_fit_panels {
            log_info!("  {:.1}x{:.1} x{} [{}]",
                no_fit.width,
                no_fit.height,
                no_fit.count,
                no_fit.label.as_deref().unwrap_or("")
            );
        }
    } else {
        log_info!("\n=== Все детали размещены успешно! ===");
    }
}

fn generate_html_visualization(solution: &CalculationResponse) -> Result<(), Box<dyn std::error::Error>> {
    if solution.mosaics.is_empty() {
        log_info!("Нет данных для визуализации");
        return Ok(());
    }
    
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n");
    html.push_str("<html>\n");
    html.push_str("<head>\n");
    html.push_str("    <meta charset='UTF-8'>\n");
    html.push_str("    <title>Результат раскроя</title>\n");
    html.push_str("    <style>\n");
    html.push_str("        body { font-family: Arial, sans-serif; margin: 20px; }\n");
    html.push_str("        .mosaic { border: 2px solid #000; margin: 20px 0; position: relative; display: inline-block; }\n");
    html.push_str("        .panel { position: absolute; border: 1px solid #333; text-align: center; display: flex; align-items: center; justify-content: center; font-size: 10px; font-weight: bold; }\n");
    html.push_str("        .info { margin: 10px 0; }\n");
    html.push_str("        .cuts { position: absolute; background: #ff0000; }\n");
    html.push_str("        .cut-h { height: 1px; }\n");
    html.push_str("        .cut-v { width: 1px; }\n");
    html.push_str("        h2 { color: #333; }\n");
    html.push_str("        .stats { background: #f5f5f5; padding: 10px; margin: 10px 0; border-radius: 5px; }\n");
    html.push_str("    </style>\n");
    html.push_str("</head>\n");
    html.push_str("<body>\n");
    html.push_str("    <h1>Результат оптимизации раскроя</h1>\n");
    
    // Общая статистика
    html.push_str("    <div class='stats'>\n");
    html.push_str("        <h3>Общая статистика:</h3>\n");
    html.push_str(&format!("        <p>Общая использованная площадь: {:.2}</p>\n", solution.total_used_area));
    html.push_str(&format!("        <p>Общая потерянная площадь: {:.2}</p>\n", solution.total_wasted_area));
    html.push_str(&format!("        <p>Коэффициент использования: {:.2}%</p>\n", solution.total_used_area_ratio * 100.0));
    html.push_str(&format!("        <p>Количество резов: {}</p>\n", solution.total_nbr_cuts));
    html.push_str(&format!("        <p>Время выполнения: {} мс</p>\n", solution.elapsed_time));
    html.push_str("    </div>\n");
    
    // Масштаб для визуализации (1 мм = 2 пикселя)
    let scale = 2.0;
    let colors = ["#FFB6C1", "#87CEEB", "#98FB98", "#F0E68C", "#DDA0DD", "#FFA07A", "#B0E0E6", "#FFEFD5"];
    
    for (i, mosaic) in solution.mosaics.iter().enumerate() {
        html.push_str(&format!("    <h2>Лист {}</h2>\n", i + 1));
        html.push_str("    <div class='info'>\n");
        html.push_str(&format!("        Использованная площадь: {:.2}, Потери: {:.2} ({:.1}% использования)\n",
            mosaic.used_area, mosaic.wasted_area, mosaic.used_area_ratio * 100.0));
        html.push_str("    </div>\n");
        
        if !mosaic.tiles.is_empty() {
            // Находим размеры листа
            let max_x = mosaic.tiles.iter().map(|t| t.x + t.width).fold(0.0, f64::max);
            let max_y = mosaic.tiles.iter().map(|t| t.y + t.height).fold(0.0, f64::max);
            
            html.push_str(&format!("    <div class='mosaic' style='width: {}px; height: {}px;'>\n",
                (max_x * scale) as i32, (max_y * scale) as i32));
            
            let mut color_index = 0;
            
            // Отображаем финальные панели
            for tile in &mosaic.tiles {
                if tile.is_final {
                    let color = colors[color_index % colors.len()];
                    color_index += 1;
                    
                    html.push_str(&format!("        <div class='panel' style='"));
                    html.push_str(&format!("left: {}px; ", (tile.x * scale) as i32));
                    html.push_str(&format!("top: {}px; ", (tile.y * scale) as i32));
                    html.push_str(&format!("width: {}px; ", (tile.width * scale) as i32));
                    html.push_str(&format!("height: {}px; ", (tile.height * scale) as i32));
                    html.push_str(&format!("background-color: {};'>\n", color));
                    html.push_str(&format!("            {:.0}x{:.0}", tile.width, tile.height));
                    
                    if let Some(label) = &tile.label {
                        html.push_str(&format!("<br>{}", label));
                    }
                    
                    html.push_str("\n        </div>\n");
                }
            }
            
            // Отображаем резы
            for cut in &mosaic.cuts {
                if cut.is_horizontal {
                    html.push_str(&format!("        <div class='cuts cut-h' style='"));
                    html.push_str(&format!("left: {}px; ", (cut.x1 * scale) as i32));
                    html.push_str(&format!("top: {}px; ", (cut.y1 * scale) as i32));
                    html.push_str(&format!("width: {}px;'></div>\n", ((cut.x2 - cut.x1) * scale) as i32));
                } else {
                    html.push_str(&format!("        <div class='cuts cut-v' style='"));
                    html.push_str(&format!("left: {}px; ", (cut.x1 * scale) as i32));
                    html.push_str(&format!("top: {}px; ", (cut.y1 * scale) as i32));
                    html.push_str(&format!("height: {}px;'></div>\n", ((cut.y2 - cut.y1) * scale) as i32));
                }
            }
            
            html.push_str("    </div>\n");
        }
        
        // Список панелей в этой мозаике
        if let Some(panels) = &mosaic.panels {
            html.push_str("    <div class='info'>\n");
            html.push_str("        <strong>Детали в листе:</strong><br>\n");
            for panel in panels {
                html.push_str(&format!("        • {:.0}x{:.0} (кол-во: {})", 
                    panel.width, panel.height, panel.count));
                if let Some(label) = &panel.label {
                    html.push_str(&format!(" [{}]", label));
                }
                html.push_str("<br>\n");
            }
            html.push_str("    </div>\n");
        }
    }
    
    // Неразмещенные панели
    if !solution.no_fit_panels.is_empty() {
        html.push_str("    <div class='stats'>\n");
        html.push_str("        <h3 style='color: #d00;'>Неразмещенные панели:</h3>\n");
        for no_fit in &solution.no_fit_panels {
            html.push_str(&format!("        • {:.0}x{:.0} (кол-во: {})", 
                no_fit.width, no_fit.height, no_fit.count));
            if let Some(label) = &no_fit.label {
                html.push_str(&format!(" [{}]", label));
            }
            html.push_str("<br>\n");
        }
        html.push_str("    </div>\n");
    }
    
    html.push_str("    <div class='info'>\n");
    html.push_str("        <small>Масштаб: 1 мм = 2 пикселя. Красные линии - резы.</small>\n");
    html.push_str("    </div>\n");
    html.push_str("</body>\n");
    html.push_str("</html>");
    
    // Записываем HTML в файл
    std::fs::write("cutting_result.html", html)?;
    
    log_info!("\n=== HTML визуализация создана ===");
    log_info!("Файл: cutting_result.html");
    log_info!("Откройте файл в браузере для просмотра схемы раскроя");
    
    Ok(())
}
