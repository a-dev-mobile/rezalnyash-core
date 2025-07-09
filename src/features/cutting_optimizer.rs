use crate::{
    features::{
        input::models::{panel::Panel, tile_dimensions::TileDimensions},
        panel_grouper::panel_grouper::PanelGrouper,
        permutation_generator::permutation_generator::PermutationGenerator,
        placement::Placement,
        solution::Solution,
    },
    utils::json::save_to_json,
};

/// Главный класс оптимизатора
pub struct CuttingOptimizer {
    pub panels: Vec<Panel>,
    pub stocks: Vec<Panel>,
    pub cut_thickness: i32,
    pub max_sheets: usize,
}

impl CuttingOptimizer {
    pub fn new(panels: Vec<Panel>, stock: Vec<Panel>) -> Self {
        Self {
            panels,
            stocks: stock,
            cut_thickness: 0, // Толщина реза (обычно 0)
            max_sheets: 10,   // Максимум листов для перебора
        }
    }

    /// Главный метод оптимизации
    /// TODO: Главная логика из compute() в CutListOptimizerServiceImpl.java
    pub fn optimize(&self) -> Solution {
        println!("=== Начало оптимизации ===");
        println!(
            "Деталей: {}, Заготовка: {}x{}",
            self.panels.len(),
            self.stocks[0].width,
            self.stocks[0].height
        );

        // Сохраняем исходные данные для отладки
        save_to_json(&self.panels, "_base_panels.json").unwrap();

        // ЭТАП 1: Развернуть панели по количеству
        let panels_expanded = self
            .panels
            .iter()
            .flat_map(|panel| panel.expand())
            .collect::<Vec<TileDimensions>>();

        let stock_expanded = self
            .stocks
            .iter()
            .flat_map(|panel| panel.expand())
            .collect::<Vec<TileDimensions>>();

        println!("Развернуто панелей: {}", panels_expanded.len());
        println!("Развернуто заготовок: {}", stock_expanded.len());

        save_to_json(&panels_expanded, "_expanded_panels.json").unwrap();
        save_to_json(&stock_expanded, "_expanded_stocks.json").unwrap();

        // ЭТАП 2: Сгруппировать панели для оптимизации перестановок
        let grouped_panels = PanelGrouper::group_panels(&panels_expanded, &stock_expanded);

        println!("Создано групп: {}", grouped_panels.len());

        for grouped_panel in &grouped_panels {
            println!("{}", grouped_panel);
        }
        // PanelGrouper::print_grouping_stats(&grouped_panels);
        save_to_json(&grouped_panels, "_grouped_panels.json").unwrap();

        // ЭТАП 3: Создание перестановок групп (новый этап!)
        println!("\n=== ЭТАП 3: Создание перестановок ===");
        let permutations = PermutationGenerator::create_group_permutations(&grouped_panels);
        PermutationGenerator::print_permutation_stats(&permutations);

        // Сохраняем первые несколько перестановок для отладки
        if !permutations.is_empty() {
            save_to_json(&permutations[0], "_first_permutation.json").unwrap();

            if permutations.len() > 1 {
                save_to_json(&permutations[1], "_second_permutation.json").unwrap();
            }
        }

        // TODO: Следующие этапы (пока заглушка)
        println!("\n=== СЛЕДУЮЩИЕ ЭТАПЫ (TODO) ===");
        println!("- Инициализация генератора исходных листов (StockPanelPicker)");
        println!("- Настройка системы решений и компараторов");
        println!("- Основной цикл оптимизации по перестановкам");
        println!("- Алгоритм размещения панелей (CutListThread)");
        println!("- Сборка финального результата");

        Solution::new() // Возвращаем пустое решение для примера
    }
}
