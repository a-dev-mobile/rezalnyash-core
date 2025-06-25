//! examples/simple_usage.rs
//! Простые примеры использования библиотеки с тяжелыми расчетами

use rezalnyas_core::{Material, CuttingRequest, CuttingOptimizer, OptimizationStrategy};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Тяжелые расчеты для демонстрации многопоточности ===\n");

    // Создаем МНОГО деталей для серьезной нагрузки
    println!("🔧 Создаем сложную задачу оптимизации...");
    let material = Material::with_cost(2500.0, 4000.0, 0.08)?;
    
    // Генерируем много разных деталей
    let mut requests = Vec::new();
    
    // Крупные детали (столешницы, фасады)
    for i in 1..=15 {
        requests.push(CuttingRequest::new(
            400.0 + i as f64 * 20.0,  // от 420 до 720
            600.0 + i as f64 * 15.0,  // от 615 до 825
            3 + (i % 4),              // от 3 до 6 штук
        ));
    }
    
    // Средние детали (боковины, полки)
    for i in 1..=25 {
        requests.push(CuttingRequest::new(
            200.0 + i as f64 * 10.0,  // от 210 до 460
            300.0 + i as f64 * 8.0,   // от 308 до 508
            2 + (i % 5),              // от 2 до 6 штук
        ));
    }
    
    // Мелкие детали (ящики, дверцы)
    for i in 1..=30 {
        requests.push(CuttingRequest::new(
            100.0 + i as f64 * 5.0,   // от 105 до 250
            150.0 + i as f64 * 4.0,   // от 154 до 270
            4 + (i % 6),              // от 4 до 9 штук
        ));
    }

    let total_parts: usize = requests.iter().map(|r| r.quantity).sum();
    let total_area: f64 = requests.iter().map(|r| r.total_area()).sum();
    
    println!("📊 Сложность задачи:");
    println!("   • {} типов деталей", requests.len());
    println!("   • {} деталей всего", total_parts);
    println!("   • {:.1} м² общая площадь деталей", total_area / 1_000_000.0);
    println!("   • Материал: {}x{} мм ({:.1} м²)", 
             material.width, material.height, material.area() / 1_000_000.0);
    println!("   • Примерно {} листов теоретически нужно\n", 
             (total_area / material.area()).ceil() as usize);

    let optimizer = CuttingOptimizer::new();

    // Пример 1: Быстрая последовательная оптимизация
    println!("🐌 1. ПОСЛЕДОВАТЕЛЬНАЯ оптимизация (1 поток):");
    let start = std::time::Instant::now();
    let result = optimizer.optimize_sequential(&material, &requests)?;
    let duration = start.elapsed();
    
    println!("   ⏱️  Время: {:.3} сек", duration.as_secs_f64());
    println!("   📄 Листов: {}", result.layouts.len());
    println!("   ⚡ Эффективность: {:.1}%", result.total_utilization * 100.0);
    println!("   💰 Стоимость: {:.2} руб", result.total_cost.unwrap_or(0.0));
    println!("   🗑️  Отходы: {:.1} м²", result.total_waste_area / 1_000_000.0);
    if result.unplaced_parts > 0 {
        println!("   ⚠️  Не размещено: {} деталей", result.unplaced_parts);
    }

    // Пример 2: Параллельная оптимизация (ЗДЕСЬ ДОЛЖНО БЫТЬ БЫСТРЕЕ!)
    println!("\n🚀 2. ПАРАЛЛЕЛЬНАЯ оптимизация (все ядра):");
    let start = std::time::Instant::now();
    let result = optimizer.optimize_parallel(&material, &requests)?;
    let duration = start.elapsed();
    
    println!("   ⏱️  Время: {:.3} сек", duration.as_secs_f64());
    println!("   📄 Листов: {}", result.layouts.len());
    println!("   ⚡ Эффективность: {:.1}%", result.total_utilization * 100.0);
    println!("   💰 Стоимость: {:.2} руб", result.total_cost.unwrap_or(0.0));
    println!("   🗑️  Отходы: {:.1} м²", result.total_waste_area / 1_000_000.0);
    if result.unplaced_parts > 0 {
        println!("   ⚠️  Не размещено: {} деталей", result.unplaced_parts);
    }

    // Пример 3: Пакетная обработка (для ОЧЕНЬ больших задач)
    println!("\n📦 3. ПАКЕТНАЯ обработка (большие объемы):");
    let start = std::time::Instant::now();
    let result = optimizer.optimize_batch(&material, &requests)?;
    let duration = start.elapsed();
    
    println!("   ⏱️  Время: {:.3} сек", duration.as_secs_f64());
    println!("   📄 Листов: {}", result.layouts.len());
    println!("   ⚡ Эффективность: {:.1}%", result.total_utilization * 100.0);
    println!("   💰 Стоимость: {:.2} руб", result.total_cost.unwrap_or(0.0));
    if result.unplaced_parts > 0 {
        println!("   ⚠️  Не размещено: {} деталей", result.unplaced_parts);
    }

    // Пример 4: Автоматический выбор стратегии
    println!("\n🤖 4. АВТО-выбор стратегии (библиотека решает сама):");
    let start = std::time::Instant::now();
    let result = optimizer.optimize_with_strategy(&material, &requests, OptimizationStrategy::Auto)?;
    let duration = start.elapsed();
    
    println!("   ⏱️  Время: {:.3} сек", duration.as_secs_f64());
    println!("   📄 Листов: {}", result.layouts.len());
    println!("   ⚡ Эффективность: {:.1}%", result.total_utilization * 100.0);
    println!("   💰 Стоимость: {:.2} руб", result.total_cost.unwrap_or(0.0));

    // ДОПОЛНИТЕЛЬНАЯ НАГРУЗКА: Сравниваем несколько раз для наглядности
    println!("\n🔥 БЕНЧМАРК: Сравнение производительности (5 прогонов каждой стратегии):");
    
    // Последовательная - 5 раз
    println!("\n🐌 Последовательная обработка:");
    let mut seq_times = Vec::new();
    for i in 1..=5 {
        print!("   Прогон {}/5... ", i);
        let start = std::time::Instant::now();
        let _result = optimizer.optimize_sequential(&material, &requests)?;
        let duration = start.elapsed();
        seq_times.push(duration.as_secs_f64());
        println!("{:.3}с", duration.as_secs_f64());
    }
    let avg_seq = seq_times.iter().sum::<f64>() / seq_times.len() as f64;
    println!("   📊 Среднее время: {:.3}с", avg_seq);
    
    // Параллельная - 5 раз  
    println!("\n🚀 Параллельная обработка:");
    let mut par_times = Vec::new();
    for i in 1..=5 {
        print!("   Прогон {}/5... ", i);
        let start = std::time::Instant::now();
        let _result = optimizer.optimize_parallel(&material, &requests)?;
        let duration = start.elapsed();
        par_times.push(duration.as_secs_f64());
        println!("{:.3}с", duration.as_secs_f64());
    }
    let avg_par = par_times.iter().sum::<f64>() / par_times.len() as f64;
    println!("   📊 Среднее время: {:.3}с", avg_par);
    
    // Итоговое сравнение
    println!("\n🏆 ИТОГОВОЕ СРАВНЕНИЕ:");
    println!("   🐌 Последовательная: {:.3}с (среднее)", avg_seq);
    println!("   🚀 Параллельная:     {:.3}с (среднее)", avg_par);
    
    if avg_par < avg_seq {
        let speedup = avg_seq / avg_par;
        println!("   ✅ Ускорение: {:.2}x быстрее!", speedup);
        if speedup > 1.5 {
            println!("   🔥 Отличный результат многопоточности!");
        }
    } else {
        println!("   ⚠️ Для этой задачи многопоточность не дала ускорения");
        println!("      (возможно, задача слишком простая или мало ядер)");
    }
    
    println!("   💻 Доступно ядер: {}", std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1));

    // Пример 5: Быстрая оценка (мгновенная!)
    println!("\n⚡ 5. МГНОВЕННАЯ оценка (без полной оптимизации):");
    let start = std::time::Instant::now();
    let estimate = optimizer.estimate_quick(&material, &requests)?;
    let duration = start.elapsed();
    
    println!("   ⏱️  Время: {:.6} сек (молниеносно!)", duration.as_secs_f64());
    println!("   📄 Примерно листов: {}", estimate.estimated_sheets);
    println!("   ⚡ Примерная эффективность: {:.1}%", estimate.estimated_efficiency * 100.0);
    println!("   🎯 Достоверность оценки: {:.1}%", estimate.confidence * 100.0);
    println!("   💡 Отлично для быстрой проверки перед полной оптимизацией!");

    // Пример 6: Сравнение алгоритмов
    println!("\n⚔️  6. БИТВА АЛГОРИТМОВ:");
    let start = std::time::Instant::now();
    let comparisons = optimizer.compare_algorithms(&material, &requests)?;
    let duration = start.elapsed();
    
    println!("   ⏱️  Время сравнения: {:.3} сек", duration.as_secs_f64());
    println!("   🥊 Результаты боя:");
    
    for (i, comparison) in comparisons.iter().enumerate() {
        let trophy = match i {
            0 => "🥇",
            1 => "🥈", 
            _ => "🥉"
        };
        println!("   {} {}: {:.3}с, {:.1}% эффективность, {} листов", 
                 trophy,
                 comparison.algorithm_name, 
                 comparison.execution_time_ms as f64 / 1000.0,
                 comparison.utilization * 100.0,
                 comparison.sheets_used);
    }

    // ФИНАЛЬНАЯ ДЕМОНСТРАЦИЯ: Экстремальная нагрузка
    println!("\n🔥🔥🔥 ЭКСТРЕМАЛЬНЫЙ ТЕСТ: Удваиваем сложность! 🔥🔥🔥");
    
    // Удваиваем количество каждой детали
    let extreme_requests: Vec<_> = requests.iter().map(|r| {
        CuttingRequest::new(r.width, r.height, r.quantity * 50)
    }).collect();
    
    let extreme_total: usize = extreme_requests.iter().map(|r| r.quantity).sum();
    println!("   📊 Экстремальная задача: {} деталей!", extreme_total);
    
    println!("\n   🐌 Последовательно:");
    let start = std::time::Instant::now();
    let seq_result = optimizer.optimize_sequential(&material, &extreme_requests)?;
    let seq_time = start.elapsed();
    println!("      ⏱️  {:.3}с, {} листов, {:.1}% эффективность", 
             seq_time.as_secs_f64(), seq_result.layouts.len(), seq_result.total_utilization * 100.0);
    
    println!("   🚀 Параллельно:");
    let start = std::time::Instant::now();
    let par_result = optimizer.optimize_parallel(&material, &extreme_requests)?;
    let par_time = start.elapsed();
    println!("      ⏱️  {:.3}с, {} листов, {:.1}% эффективность", 
             par_time.as_secs_f64(), par_result.layouts.len(), par_result.total_utilization * 100.0);
    
    if par_time < seq_time {
        let extreme_speedup = seq_time.as_secs_f64() / par_time.as_secs_f64();
        println!("\n   🏆 ФИНАЛЬНОЕ УСКОРЕНИЕ НА СЛОЖНОЙ ЗАДАЧЕ: {:.2}x!", extreme_speedup);
        
        if extreme_speedup > 2.0 {
            println!("   🚀🚀🚀 НЕВЕРОЯТНО! Многопоточность рулит! 🚀🚀🚀");
        } else if extreme_speedup > 1.3 {
            println!("   ✅✅ Отличный результат! Многопоточность работает! ✅✅");
        } else {
            println!("   ✅ Многопоточность дает ускорение!");
        }
    } else {
        println!("   🤔 На этой конфигурации последовательная оказалась не медленнее");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_strategies_heavy_load() {
        // Создаем тяжелую нагрузку для тестов
        let material = Material::new(2000.0, 3000.0).unwrap();
        let mut requests = Vec::new();
        
        // Много разных деталей
        for i in 1..=20 {
            requests.push(CuttingRequest::new(
                100.0 + i as f64 * 10.0,
                150.0 + i as f64 * 8.0,
                3 + (i % 4),
            ));
        }
        
        let optimizer = CuttingOptimizer::new();

        // Тестируем все стратегии на тяжелой нагрузке
        let seq_start = std::time::Instant::now();
        assert!(optimizer.optimize_sequential(&material, &requests).is_ok());
        let seq_time = seq_start.elapsed();
        
        let par_start = std::time::Instant::now();
        assert!(optimizer.optimize_parallel(&material, &requests).is_ok());
        let par_time = par_start.elapsed();
        
        let batch_start = std::time::Instant::now();
        assert!(optimizer.optimize_batch(&material, &requests).is_ok());
        let batch_time = batch_start.elapsed();
        
        let auto_start = std::time::Instant::now();
        assert!(optimizer.optimize_with_strategy(&material, &requests, OptimizationStrategy::Auto).is_ok());
        let auto_time = auto_start.elapsed();
        
        // Выводим времена для анализа
        println!("Времена выполнения в тестах:");
        println!("  Sequential: {:.3}s", seq_time.as_secs_f64());
        println!("  Parallel:   {:.3}s", par_time.as_secs_f64());
        println!("  Batch:      {:.3}s", batch_time.as_secs_f64());
        println!("  Auto:       {:.3}s", auto_time.as_secs_f64());
    }

    #[test]
    fn test_estimate_and_comparison_performance() {
        let material = Material::new(2000.0, 2000.0).unwrap();
        let mut requests = Vec::new();
        
        // Средняя нагрузка
        for i in 1..=15 {
            requests.push(CuttingRequest::new(
                200.0 + i as f64 * 15.0,
                300.0 + i as f64 * 10.0,
                2 + (i % 3),
            ));
        }
        
        let optimizer = CuttingOptimizer::new();

        // Тестируем быстроту дополнительных функций
        let estimate_start = std::time::Instant::now();
        assert!(optimizer.estimate_quick(&material, &requests).is_ok());
        let estimate_time = estimate_start.elapsed();
        
        let compare_start = std::time::Instant::now();
        assert!(optimizer.compare_algorithms(&material, &requests).is_ok());
        let compare_time = compare_start.elapsed();
        
        println!("Времена дополнительных функций:");
        println!("  Quick estimate: {:.6}s", estimate_time.as_secs_f64());
        println!("  Compare algos:  {:.3}s", compare_time.as_secs_f64());
        
        // Быстрая оценка должна быть очень быстрой
        assert!(estimate_time.as_millis() < 100, "Estimate should be under 100ms");
    }

    #[test]
    fn test_extreme_load() {
        // Экстремальная нагрузка - только если есть время
        if std::env::var("RUN_EXTREME_TESTS").is_ok() {
            let material = Material::new(3000.0, 4000.0).unwrap();
            let mut requests = Vec::new();
            
            // Очень много деталей
            for i in 1..=50 {
                requests.push(CuttingRequest::new(
                    50.0 + i as f64 * 8.0,
                    100.0 + i as f64 * 6.0,
                    2 + (i % 7),
                ));
            }
            
            let optimizer = CuttingOptimizer::new();
            
            println!("🔥 Экстремальный тест: {} типов деталей", requests.len());
            let total_parts: usize = requests.iter().map(|r| r.quantity).sum();
            println!("🔥 Всего деталей: {}", total_parts);
            
            // Только параллельная стратегия для экстремальной нагрузки
            let start = std::time::Instant::now();
            let result = optimizer.optimize_parallel(&material, &requests).unwrap();
            let duration = start.elapsed();
            
            println!("🔥 Время обработки: {:.3}s", duration.as_secs_f64());
            println!("🔥 Листов потребовалось: {}", result.layouts.len());
            println!("🔥 Эффективность: {:.1}%", result.total_utilization * 100.0);
            
            assert!(!result.layouts.is_empty());
        }
    }
}