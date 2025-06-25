//! examples/simple_usage.rs
//! Простые примеры использования библиотеки

use rezalnyas_core::{Material, CuttingRequest, CuttingOptimizer, OptimizationStrategy};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Простые примеры использования rezalnyas_core ===\n");

    // Создаем материал и детали
    let material = Material::with_cost(1220.0, 2440.0, 0.08)?;
    let requests = vec![
        CuttingRequest::new(400.0, 600.0, 3),    // Столешницы
        CuttingRequest::new(300.0, 800.0, 4),    // Боковины
        CuttingRequest::new(300.0, 800.0, 4),    // Боковины
        CuttingRequest::new(280.0, 400.0, 2),    // Полки
    ];

    let optimizer = CuttingOptimizer::new();

    // Пример 1: Быстрая последовательная оптимизация
    println!("1. Быстрая последовательная оптимизация:");
    let start = std::time::Instant::now();
    let result = optimizer.optimize_sequential(&material, &requests)?;
    let duration = start.elapsed();
    
    println!("   Время: {:.2} сек", duration.as_secs_f64());
    println!("   Листов: {}", result.layouts.len());
    println!("   Эффективность: {:.1}%", result.total_utilization * 100.0);
    println!("   Стоимость: {:.2} руб", result.total_cost.unwrap_or(0.0));

    // Пример 2: Параллельная оптимизация
    println!("\n2. Параллельная оптимизация:");
    let start = std::time::Instant::now();
    let result = optimizer.optimize_parallel(&material, &requests)?;
    let duration = start.elapsed();
    
    println!("   Время: {:.2} сек", duration.as_secs_f64());
    println!("   Листов: {}", result.layouts.len());
    println!("   Эффективность: {:.1}%", result.total_utilization * 100.0);

    // Пример 3: Пакетная обработка
    println!("\n3. Пакетная обработка:");
    let start = std::time::Instant::now();
    let result = optimizer.optimize_batch(&material, &requests)?;
    let duration = start.elapsed();
    
    println!("   Время: {:.2} сек", duration.as_secs_f64());
    println!("   Листов: {}", result.layouts.len());
    println!("   Эффективность: {:.1}%", result.total_utilization * 100.0);

    // Пример 4: Автоматический выбор стратегии
    println!("\n4. Автоматический выбор стратегии:");
    let start = std::time::Instant::now();
    let result = optimizer.optimize_with_strategy(&material, &requests, OptimizationStrategy::Auto)?;
    let duration = start.elapsed();
    
    println!("   Время: {:.2} сек", duration.as_secs_f64());
    println!("   Листов: {}", result.layouts.len());
    println!("   Эффективность: {:.1}%", result.total_utilization * 100.0);

    // Пример 5: Быстрая оценка
    println!("\n5. Быстрая оценка (без полной оптимизации):");
    let estimate = optimizer.estimate_quick(&material, &requests)?;
    println!("   Примерно листов: {}", estimate.estimated_sheets);
    println!("   Примерная эффективность: {:.1}%", estimate.estimated_efficiency * 100.0);
    println!("   Достоверность оценки: {:.1}%", estimate.confidence * 100.0);

    // Пример 6: Сравнение алгоритмов
    println!("\n6. Сравнение алгоритмов:");
    let comparisons = optimizer.compare_algorithms(&material, &requests)?;
    for comparison in comparisons {
        println!("   {}: {} мс, {:.1}% эффективность, {} листов", 
                 comparison.algorithm_name, 
                 comparison.execution_time_ms,
                 comparison.utilization * 100.0,
                 comparison.sheets_used);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_strategies() {
        let material = Material::new(1000.0, 1000.0).unwrap();
        let requests = vec![CuttingRequest::new(200.0, 300.0, 2)];
        let optimizer = CuttingOptimizer::new();

        // Тестируем все стратегии
        assert!(optimizer.optimize_sequential(&material, &requests).is_ok());
        assert!(optimizer.optimize_parallel(&material, &requests).is_ok());
        assert!(optimizer.optimize_batch(&material, &requests).is_ok());
        assert!(optimizer.optimize_with_strategy(&material, &requests, OptimizationStrategy::Auto).is_ok());
    }

    #[test]
    fn test_estimate_and_comparison() {
        let material = Material::new(1000.0, 1000.0).unwrap();
        let requests = vec![CuttingRequest::new(200.0, 300.0, 2)];
        let optimizer = CuttingOptimizer::new();

        // Тестируем дополнительные функции
        assert!(optimizer.estimate_quick(&material, &requests).is_ok());
        assert!(optimizer.compare_algorithms(&material, &requests).is_ok());
    }
}