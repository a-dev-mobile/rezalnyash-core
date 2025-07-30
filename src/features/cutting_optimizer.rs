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

        // ЭТАП 4: Основной цикл оптимизации - реализуем как в Java
        println!("\n=== ЭТАП 4: Основной цикл оптимизации ===");
        
        let mut best_solution = Solution::new_with_stocks(self.stocks.clone());
        let mut best_efficiency = 0.0;
        
        // Попробуем несколько перестановок для поиска лучшего решения
        let max_permutations_to_try = std::cmp::min(permutations.len(), 5); // Попробуем до 5 перестановок
        println!("Попробуем {} перестановок из {}", max_permutations_to_try, permutations.len());
        
        for (perm_index, permutation) in permutations.iter().take(max_permutations_to_try).enumerate() {
            println!("\n--- Пробуем перестановку {} ---", perm_index + 1);
            let mut current_solution = Solution::new_with_stocks(self.stocks.clone());
            
            let mut remaining_panels: Vec<_> = permutation.iter().map(|tile_dim| {
                Panel {
                    id: tile_dim.id,
                    width: tile_dim.width,
                    height: tile_dim.height,
                    count: 1, // TileDimensions уже развернут
                    label: tile_dim.label.clone(),
                    material: "DEFAULT_MATERIAL".to_string(),
                }
            }).collect();
            
            let mut stock_index = 0;
            let mut total_placed = 0;
            
            // Главный цикл: пробуем разместить все панели, используя новые листы по мере необходимости
            while !remaining_panels.is_empty() && stock_index < self.max_sheets {
                // Берем следующий лист (используем первый лист из stocks как шаблон)
                let stock_template = &self.stocks[0];
                let mut placement = crate::features::placement::Placement::new(stock_template);
                placement.stock_id = stock_index as i32;
                
                println!("\n--- Начинаем размещение на листе {} ---", stock_index + 1);
                
                let mut placed_on_this_sheet = 0;
                let mut i = 0;
                
                // Пробуем разместить панели на текущем листе
                while i < remaining_panels.len() {
                    let panel = &remaining_panels[i];
                    
                    if placement.try_place_panel(panel, self.cut_thickness) {
                        println!("✓ Размещена панель {}x{} [{}] на листе {}", 
                            panel.width, panel.height, panel.label, stock_index + 1);
                        placed_on_this_sheet += 1;
                        total_placed += 1;
                        remaining_panels.remove(i);
                        // Не увеличиваем i, так как элементы сдвинулись
                    } else {
                        i += 1; // Переходим к следующей панели
                    }
                }
                
                println!("На листе {} размещено {} панелей", stock_index + 1, placed_on_this_sheet);
                
                // Добавляем размещение только если что-то было размещено
                if placed_on_this_sheet > 0 {
                    current_solution.placements.push(placement);
                }
                
                stock_index += 1;
                
                // Если на листе ничего не разместилось, прекращаем попытки
                if placed_on_this_sheet == 0 {
                    println!("⚠️  На листе {} ничего не поместилось, прекращаем размещение", stock_index);
                    break;
                }
            }
            
            // Все оставшиеся панели помечаем как неразмещенные
            current_solution.unplaced_panels = remaining_panels;
            
            // Рассчитываем эффективность этой перестановки
            let total_used_area: i64 = current_solution.placements.iter().map(|p| p.used_area).sum();
            let total_area: i64 = current_solution.placements.len() as i64 * (self.stocks[0].width as i64 * self.stocks[0].height as i64);
            let efficiency = if total_area > 0 { total_used_area as f64 / total_area as f64 } else { 0.0 };
            
            println!("Перестановка {}: {} листов, эффективность {:.2}%, неразмещенных: {}", 
                perm_index + 1, current_solution.placements.len(), efficiency * 100.0, current_solution.unplaced_panels.len());
            
            // Проверяем, лучше ли это решение
            let is_better = current_solution.unplaced_panels.len() < best_solution.unplaced_panels.len() ||
                           (current_solution.unplaced_panels.len() == best_solution.unplaced_panels.len() && efficiency > best_efficiency);
            
            if is_better {
                println!("✓ Найдено лучшее решение!");
                best_solution = current_solution;
                best_efficiency = efficiency;
            }
        }
        
        println!("\n=== Итоговый результат оптимизации ===");
        println!("Использовано листов: {}", best_solution.placements.len());
        println!("Эффективность: {:.2}%", best_efficiency * 100.0);
        println!("Не размещено панелей: {}", best_solution.unplaced_panels.len());
        
        if !best_solution.unplaced_panels.is_empty() {
            println!("Неразмещенные панели:");
            for panel in &best_solution.unplaced_panels {
                println!("  • {}x{} [{}]", panel.width, panel.height, panel.label);
            }
        }

        best_solution
    }
}
