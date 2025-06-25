//! Базовый пример использования библиотеки rezalnyas_core

use rezalnyas_core::{Material, CuttingRequest, CuttingOptimizer, OptimizationConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Базовый пример оптимизации раскроя ===\n");

    // Создаем материал (лист ДСП 1220x2440 мм)
    let material = Material::with_cost(1220.0, 2440.0, 0.08)?; // 8 коп за мм²
    println!("Материал: {}x{} мм", material.width, material.height);
    println!("Площадь: {:.0} мм²", material.area());
    if let Some(cost) = material.total_cost() {
        println!("Стоимость листа: {:.2} руб\n", cost);
    }

    // Создаем запросы на детали
    let requests = vec![
        CuttingRequest::new(400.0, 600.0, 3),    // Столешницы
        CuttingRequest::new(300.0, 800.0, 4),    // Боковины
        CuttingRequest::new(280.0, 400.0, 2),    // Полки
        CuttingRequest::new(200.0, 300.0, 6),    // Дверцы
        CuttingRequest::new(150.0, 200.0, 4),    // Ящики
    ];

    println!("Детали для раскроя:");
    let mut total_area = 0.0;
    for (i, request) in requests.iter().enumerate() {
        let area = request.total_area();
        total_area += area;
        println!("  {}: {}x{} мм x {} шт = {:.0} мм²", 
                 i + 1, request.width, request.height, request.quantity, area);
    }
    println!("Общая площадь деталей: {:.0} мм²\n", total_area);

    // Создаем оптимизатор с настройками
    let config = OptimizationConfig {
        max_threads: Some(4),
        cutting_gap: 3.0,        // 3мм на пропил
        min_waste_size: 50.0,    // Остатки меньше 50мм не учитываем
        timeout_seconds: Some(30),
    };

    let optimizer = CuttingOptimizer::with_config(config);
    
    // Выполняем оптимизацию
    println!("Выполняем оптимизацию...");
    let start = std::time::Instant::now();
    let result = optimizer.optimize(&material, &requests)?;
    let duration = start.elapsed();

    // Выводим результаты
    println!("\n=== РЕЗУЛЬТАТЫ ОПТИМИЗАЦИИ ===");
    println!("Время выполнения: {:.2} сек", duration.as_secs_f64());
    println!("Количество листов: {}", result.layouts.len());
    println!("Общий коэффициент использования: {:.1}%", result.total_utilization * 100.0);
    println!("Общая площадь отходов: {:.0} мм²", result.total_waste_area);
    
    if result.unplaced_parts > 0 {
        println!("⚠️  Не размещено деталей: {}", result.unplaced_parts);
    }

    if let Some(total_cost) = result.total_cost {
        println!("Общая стоимость материалов: {:.2} руб", total_cost);
    }

    // Детальная информация по листам
    println!("\n=== ДЕТАЛИЗАЦИЯ ПО ЛИСТАМ ===");
    for (i, layout) in result.layouts.iter().enumerate() {
        println!("\nЛист {} ({:.0}x{:.0} мм):", 
                 i + 1, layout.material.width, layout.material.height);
        println!("  Использование: {:.1}%", layout.utilization * 100.0);
        println!("  Площадь отходов: {:.0} мм²", layout.waste_area);
        println!("  Количество деталей: {}", layout.parts.len());

        // Показываем первые 5 деталей
        for (j, part) in layout.parts.iter().take(5).enumerate() {
            println!("    Деталь {}: {:.0}x{:.0} в позиции ({:.0}, {:.0}){}",
                     j + 1,
                     part.rectangle.width,
                     part.rectangle.height,
                     part.rectangle.x,
                     part.rectangle.y,
                     if part.rotated { " [повернута]" } else { "" });
        }

        if layout.parts.len() > 5 {
            println!("    ... и еще {} деталей", layout.parts.len() - 5);
        }
    }

    // Расчет экономии
    let theoretical_sheets = (total_area / material.area()).ceil() as usize;
    let actual_sheets = result.layouts.len();
    
    println!("\n=== ЭКОНОМИЧЕСКИЙ АНАЛИЗ ===");
    println!("Теоретическое количество листов: {}", theoretical_sheets);
    println!("Фактическое количество листов: {}", actual_sheets);
    
    if actual_sheets <= theoretical_sheets {
        let saved_sheets = theoretical_sheets - actual_sheets;
        if let Some(cost_per_sheet) = material.total_cost() {
            let savings = saved_sheets as f64 * cost_per_sheet;
            println!("Экономия: {} листов = {:.2} руб", saved_sheets, savings);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_example() {
        let result = main();
        assert!(result.is_ok());
    }
}