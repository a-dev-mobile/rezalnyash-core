use rezalnyas_core::{
    features::{
        cutting_optimizer::CuttingOptimizer,
        input::{
            converters::input_converter::InputConverter,
            models::{panel_input::PanelInput, stock_input::StockInput},
            traits::dimensions::{self, Dimensions},
        },
    },
    scaled_math::ScaledConverter,
};

fn main() {
    println!("=== Тест оптимизации раскроя ===");

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

    // Заготовка -
    let stocks_input: Vec<StockInput> = vec![StockInput::new(1, "400.0", "300.0", "Заготовка_1")];

    let input_converter = InputConverter::new(&panels_input, &stocks_input).unwrap();

    let precision = input_converter.precision();

    let (panels, stocks) = input_converter
        .convert_all(&panels_input, &stocks_input)
        .unwrap();

    println!("===");
    // Создаем оптимизатор
    let optimizer = CuttingOptimizer::new(panels, stocks);

    // Запускаем оптимизацию
    let solution = optimizer.optimize();

    // Выводим результат
    // optimizer.print_solution(&solution);
}
