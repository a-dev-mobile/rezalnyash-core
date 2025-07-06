use rezalnyas_core::features::input::{panel::PanelInput, stock::StockInput};


fn main() {
    println!("=== Тест оптимизации раскроя ===");

    // Создаем детали
    let panels: Vec<PanelInput> = vec![
        PanelInput::new("100.0", "500.0", 2, "Деталь_1"),
        PanelInput::new("800.0", "600.0", 1, "Деталь_2"),
    ];

    // Заготовка
    let stocks: Vec<StockInput> = vec![StockInput::new("300.0", "200.0", "Заготовка_1")];

    let mut dimensions: Vec<&str> = vec![];

    for item in &panels {
        dimensions.push(item.get_dimensions());
    }
    for item in &stocks {
        dimensions.push(item.get_dimensions());
    }



    
    // Создаем оптимизатор
    let optimizer = CuttingOptimizer::new(panels, stocks);

    // Запускаем оптимизацию
    let solution = optimizer.optimize();

    // Выводим результат
    optimizer.print_solution(&solution);
}
