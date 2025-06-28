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
    },
    CutListOptimizerService, CuttingRequest, Material, OptimizationConfig, OptimizationStrategy,
};

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

    println!("\nПример завершен. Проверьте вывод логов выше.");

    let panels: Vec<Panel> = vec![
        Panel::new(1, 55.0, 45.0, 1),
        Panel::new(2, 35.0, 25.0, 1),
        Panel::new(3, 25.0, 15.0, 1),
        Panel::new(4, 15.0, 20.0, 1),
        Panel::new(5, 40.0, 30.0, 1),
    ];
    let stock_panels: Vec<Panel> = vec![Panel::new(1, 90.0, 120.0, 1)];

    let config = Configuration {
        cut_thickness: 0.0,             // Точная толщина реза
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












    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //

    let config = OptimizationConfig {
        max_threads: Some(1),
        cutting_gap: 2.0,
        min_waste_size: 50.0,
        timeout_seconds: Some(300),
    };

    let optimizer = CutListOptimizerService::with_config(config);
    // Простая задача для отладки
    let requests = vec![
        CuttingRequest::new(300.0, 400.0, 2),
        CuttingRequest::new(200.0, 300.0, 1),
        CuttingRequest::new(150.0, 250.0, 3),
    ];
    let material = Material::with_cost(1000.0, 2000.0, 0.08)?;

    println!("\n📊 Debug task:");
    println!(
        "   Material: {}x{} ({:.1} м²)",
        material.width,
        material.height,
        material.area() / 1_000_000.0
    );
    println!("   Requests: {}", requests.len());

    let mut total_parts = 0;
    let mut total_area = 0.0;
    for (i, req) in requests.iter().enumerate() {
        total_parts += req.quantity;
        total_area += req.total_area();
        println!(
            "     {}: {}x{} qty={} (area: {:.1} м²)",
            i,
            req.width,
            req.height,
            req.quantity,
            req.total_area() / 1_000_000.0
        );
    }

    println!("   Total parts: {}", total_parts);
    println!("   Total area needed: {:.1} м²", total_area / 1_000_000.0);
    println!(
        "   Theoretical efficiency: {:.1}%",
        (total_area / material.area()) * 100.0
    );

    // Последовательная оптимизация с подробным выводом
    println!("\n🔍 Starting sequential optimization...");
    let start = std::time::Instant::now();
    let result = optimizer.optimize_sequential(&material, &requests)?;
    let duration = start.elapsed();

    println!("\n📋 Results:");
    println!("   Time: {:.6}s", duration.as_secs_f64());
    println!("   Layouts: {}", result.layouts.len());
    println!("   Utilization: {:.1}%", result.total_utilization * 100.0);
    println!("   Unplaced: {}", result.unplaced_parts);
    println!("   Total cost: {:.2} руб", result.total_cost.unwrap_or(0.0));

    // Детальная информация о размещении
    for (layout_idx, layout) in result.layouts.iter().enumerate() {
        println!("\n📄 Layout {}:", layout_idx + 1);
        println!(
            "   Material: {}x{}",
            layout.material.width, layout.material.height
        );
        println!("   Parts placed: {}", layout.parts.len());
        println!("   Utilization: {:.1}%", layout.utilization * 100.0);
        println!("   Waste area: {:.1} м²", layout.waste_area / 1_000_000.0);

        for (part_idx, part) in layout.parts.iter().enumerate() {
            println!(
                "     Part {}: pos=({:.1}, {:.1}) size={:.1}x{:.1} rotated={} area={:.1}м²",
                part_idx + 1,
                part.rectangle.x,
                part.rectangle.y,
                part.rectangle.width,
                part.rectangle.height,
                part.rotated,
                part.rectangle.area() / 1_000_000.0
            );
        }
    }

    // Сравнение стратегий в однопоточном режиме
    println!("\n🔬 Comparing strategies in single-thread mode:");

    // Sequential
    let start = std::time::Instant::now();
    let seq_result =
        optimizer.optimize_with_strategy(&material, &requests, OptimizationStrategy::Sequential)?;
    let seq_time = start.elapsed();

    // Parallel (но с одним потоком)
    let start = std::time::Instant::now();
    let par_result =
        optimizer.optimize_with_strategy(&material, &requests, OptimizationStrategy::Parallel)?;
    let par_time = start.elapsed();

    // Batch
    let start = std::time::Instant::now();
    let batch_result =
        optimizer.optimize_with_strategy(&material, &requests, OptimizationStrategy::Batch)?;
    let batch_time = start.elapsed();

    println!(
        "   Sequential: {:.6}s, {:.1}% efficiency, {} layouts",
        seq_time.as_secs_f64(),
        seq_result.total_utilization * 100.0,
        seq_result.layouts.len()
    );
    println!(
        "   Parallel:   {:.6}s, {:.1}% efficiency, {} layouts",
        par_time.as_secs_f64(),
        par_result.total_utilization * 100.0,
        par_result.layouts.len()
    );
    println!(
        "   Batch:      {:.6}s, {:.1}% efficiency, {} layouts",
        batch_time.as_secs_f64(),
        batch_result.total_utilization * 100.0,
        batch_result.layouts.len()
    );

    // Быстрая оценка
    println!("\n⚡ Quick estimation:");
    let start = std::time::Instant::now();
    let estimate = optimizer.estimate_quick(&material, &requests)?;
    let estimate_time = start.elapsed();

    println!("   Time: {:.9}s (ultra-fast!)", estimate_time.as_secs_f64());
    println!("   Estimated sheets: {}", estimate.estimated_sheets);
    println!(
        "   Estimated efficiency: {:.1}%",
        estimate.estimated_efficiency * 100.0
    );
    println!("   Confidence: {:.1}%", estimate.confidence * 100.0);

    // Сравнение алгоритмов
    println!("\n⚔️ Algorithm comparison:");
    let start = std::time::Instant::now();
    let comparisons = optimizer.compare_algorithms(&material, &requests)?;
    let compare_time = start.elapsed();

    println!("   Comparison time: {:.6}s", compare_time.as_secs_f64());
    for (i, comp) in comparisons.iter().enumerate() {
        let trophy = match i {
            0 => "🥇",
            1 => "🥈",
            _ => "🥉",
        };

        if comp.success {
            println!(
                "   {} {}: {:.6}s, {:.1}% efficiency, {} sheets",
                trophy,
                comp.algorithm_name,
                comp.execution_time_ms as f64 / 1000.0,
                comp.utilization * 100.0,
                comp.sheets_used
            );
        } else {
            println!(
                "   ❌ {}: FAILED - {}",
                comp.algorithm_name,
                comp.error.as_ref().unwrap_or(&"Unknown error".to_string())
            );
        }
    }

    // Проверка переменной окружения
    println!("\n🔧 Environment check:");
    if let Ok(rayon_threads) = std::env::var("RAYON_NUM_THREADS") {
        println!("   RAYON_NUM_THREADS: {}", rayon_threads);
    } else {
        println!("   RAYON_NUM_THREADS: not set (using default)");
    }

    println!("   Actual parallelism: {}", rayon::current_num_threads());

    // Тест с увеличенной нагрузкой
    println!("\n🔥 Stress test with more parts:");
    let stress_requests: Vec<_> = (0..20)
        .map(|i| CuttingRequest::new(100.0 + i as f64 * 10.0, 150.0 + i as f64 * 8.0, 2 + (i % 3)))
        .collect();

    let stress_total: usize = stress_requests.iter().map(|r| r.quantity).sum();
    println!(
        "   Stress task: {} types, {} total parts",
        stress_requests.len(),
        stress_total
    );

    let start = std::time::Instant::now();
    let stress_result = optimizer.optimize_sequential(&material, &stress_requests)?;
    let stress_time = start.elapsed();

    println!(
        "   Result: {:.6}s, {:.1}% efficiency, {} unplaced",
        stress_time.as_secs_f64(),
        stress_result.total_utilization * 100.0,
        stress_result.unplaced_parts
    );

    println!("\n✅ Debug session complete!");

    Ok(())
}
