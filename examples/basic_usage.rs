use rezalnyas_core::{
    features::{
        cutting_optimizer::CuttingOptimizer,
        html_generator::HtmlGenerator,
        input::{
            converters::input_converter::InputConverter,
            models::{panel_input::PanelInput, stock_input::StockInput},
            traits::dimensions::{self, Dimensions},
        },
    },
    scaled_math::ScaledConverter,
};
use std::time::Instant;

fn main() {
    println!("=== Тест оптимизации раскроя ===");
    let start_time = Instant::now();

    // Создаем детали - аналогично Java коду
    let panels_input: Vec<PanelInput> = vec![
        PanelInput::new(1, "150.5", "100.25", 2, "Деталь_1"),
        PanelInput::new(2, "80.75", "60.5", 3, "Деталь_2"),
        PanelInput::new(3, "120.0", "45.75", 1, "Деталь_3"),
        PanelInput::new(4, "95.25", "75.5", 2, "Деталь_4"),
        PanelInput::new(5, "65.5", "85.25", 1, "Деталь_5"),
        PanelInput::new(6, "110.75", "55.0", 2, "Деталь_6"),
        PanelInput::new(7, "40.25", "90.5", 3, "Деталь_7"),
        PanelInput::new(8, "130.0", "35.75", 1, "Деталь_8"),
    ];

    // Заготовка - аналогично Java коду
    let stocks_input: Vec<StockInput> =
        vec![StockInput::new(1, "400.0", "300.0", 1, "Заготовка_1")];

    println!("Создан запрос:");
    println!("- Деталей: {}", panels_input.len());
    println!("- Заготовка: {}x{}", stocks_input[0].width, stocks_input[0].height);

    let input_converter = InputConverter::new(&panels_input, &stocks_input).unwrap();
    let precision = input_converter.precision();

    let (panels, stocks) = input_converter
        .convert_all(&panels_input, &stocks_input)
        .unwrap();

    // Создаем оптимизатор
    let optimizer = CuttingOptimizer::new(panels, stocks);

    // Запускаем оптимизацию
    println!("Запуск оптимизации...");
    let mut solution = optimizer.optimize();
    
    // Вычисляем итоговые показатели
    solution.calculate_totals();
    
    let elapsed_time = start_time.elapsed();
    
    // Выводим результат аналогично Java версии
    print_solution(&solution, precision as u32, elapsed_time.as_millis());
    
    // Генерируем HTML файл
    match HtmlGenerator::generate_html(&solution, precision as u32, elapsed_time.as_millis(), "cutting_result.html") {
        Ok(_) => println!("HTML визуализация успешно создана!"),
        Err(e) => eprintln!("Ошибка создания HTML: {}", e),
    }
}

fn print_solution(solution: &rezalnyas_core::features::solution::Solution, precision: u32, elapsed_time_ms: u128) {
    let scale_factor = 10f64.powi(precision as i32);
    let total_used_area = solution.total_used_area as f64 / (scale_factor * scale_factor);
    let total_waste_area = solution.total_waste_area as f64 / (scale_factor * scale_factor);
    let total_cut_length = solution.total_cut_length as f64 / scale_factor;
    
    println!("\n=== Результат оптимизации ===");
    println!("Общая использованная площадь: {:.2}", total_used_area);
    println!("Общая потерянная площадь: {:.2}", total_waste_area);
    println!("Коэффициент использования: {:.2}%", solution.total_efficiency * 100.0);
    println!("Количество резов: {}", solution.total_cuts);
    println!("Общая длина резов: {:.2}", total_cut_length);
    println!("Время выполнения: {} мс", elapsed_time_ms);

    println!("\n=== Мозаики (листы с раскроем) ===");
    for (i, placement) in solution.placements.iter().enumerate() {
        let used_area = placement.used_area as f64 / (scale_factor * scale_factor);
        let waste_area = placement.waste_area as f64 / (scale_factor * scale_factor);
        let total_placement_area = used_area + waste_area;
        let efficiency = if total_placement_area > 0.0 {
            used_area / total_placement_area * 100.0
        } else {
            0.0
        };
        
        println!("Лист {}:", i + 1);
        println!("  Использование: {:.2}% ({:.2}/{:.2})", efficiency, used_area, total_placement_area);
        
        // Группируем панели по размеру и названию для отображения
        let mut panel_groups: std::collections::HashMap<String, (f64, f64, usize, String)> = std::collections::HashMap::new();
        
        for panel in &placement.placed_panels {
            let width_mm = panel.width as f64 / scale_factor;
            let height_mm = panel.height as f64 / scale_factor;
            let key = format!("{:.1}x{:.1}_{}", width_mm, height_mm, panel.label);
            
            let entry = panel_groups.entry(key).or_insert((width_mm, height_mm, 0, panel.label.clone()));
            entry.2 += 1;
        }

        for (_, (width, height, count, label)) in panel_groups {
            println!("    {:.1}x{:.1} x{} [{}]", width, height, count, label);
        }
    }

    if !solution.unplaced_panels.is_empty() {
        println!("\n=== Неразмещенные детали ===");
        for panel in &solution.unplaced_panels {
            let width_mm = panel.width as f64 / scale_factor;
            let height_mm = panel.height as f64 / scale_factor;
            println!("  {:.1}x{:.1} x{} [{}]", width_mm, height_mm, panel.count, panel.label);
        }
    } else {
        println!("\n=== Все детали размещены успешно! ===");
    }
}
