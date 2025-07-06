use crate::{features::{input::models::{panel::Panel, stock::Stock}, permutation_generator::PermutationGenerator, placement::Placement, solution::Solution}, utils::json::save_to_json};


/// Главный класс оптимизатора
pub struct CuttingOptimizer {
    pub panels: Vec<Panel>,
    pub stock: Vec<Stock>,
    pub cut_thickness: i32,
    pub max_sheets: usize,
}

impl CuttingOptimizer {
    pub fn new(panels: Vec<Panel>, stock: Vec<Stock>) -> Self {
        Self {
            panels,
            stock,
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
            self.stock[0].width,
            self.stock[0].height
        );

        save_to_json(&self.panels, "_base_panels.json");
        // ЭТАП 1: Развернуть панели по количеству
        let expanded_panels = self.expand_panels();
        println!("Развернуто панелей: {}", expanded_panels.len());
        save_to_json(&expanded_panels, "_expanded_panels.json");
        // ЭТАП 2: Сгруппировать панели для оптимизации перестановок
        let grouped_panels = PermutationGenerator::group_panels(&expanded_panels);
        save_to_json(&grouped_panels, "_grouped_panels.json");
        // ЭТАП 3: Генерировать перестановки
        println!("Генерация перестановок...");
        let permutations = PermutationGenerator::generate_permutations(grouped_panels);
        println!("Сгенерировано перестановок: {}", permutations.len());

        let mut best_solution = Solution::new();

        // ЭТАП 4: Перебор перестановок и количества листов
        for (perm_idx, permutation) in permutations.iter().enumerate() {
            println!(
                "Обработка перестановки {}/{}",
                perm_idx + 1,
                permutations.len()
            );

            // Пробуем разные количества листов
            for sheet_count in 1..=self.max_sheets {
                let solution = self.try_solution(permutation, sheet_count);

                // Если все панели поместились - это хороший кандидат
                if solution.unplaced_panels.is_empty() {
                    if best_solution.placements.is_empty()
                        || solution.score() < best_solution.score()
                    {
                        best_solution = solution;
                        println!(
                            "Найдено лучшее решение: {} листов, эффективность {:.1}%",
                            sheet_count,
                            best_solution.total_efficiency * 100.0
                        );
                    }
                    break; // Переходим к следующей перестановке
                }

                // Если не все поместились, но это лучше предыдущего
                if solution.score() < best_solution.score() {
                    best_solution = solution;
                }
            }

            // Если нашли идеальное решение - можем остановиться
            if best_solution.unplaced_panels.is_empty() && best_solution.placements.len() == 1 {
                println!("Найдено оптимальное решение на 1 листе!");
                break;
            }
        }

        best_solution.calculate_totals();
        println!("=== Оптимизация завершена ===");
        best_solution
    }

    /// Развертывание панелей по количеству
    fn expand_panels(&self) -> Vec<Panel> {
        let mut expanded = Vec::new();
        for panel in &self.panels {
            for i in 0..panel.count {
                let mut new_panel = panel.clone();
                new_panel.count = 1;
                // Уникальный ID для каждой копии
                // new_panel.id = panel.id * 1000 + i;
                expanded.push(new_panel);
            }
        }
        expanded
    }

    /// Попытка решения с заданной перестановкой и количеством листов
    /// TODO: Основная логика размещения
    fn try_solution(&self, panels: &[Panel], sheet_count: usize) -> Solution {
        let mut solution = Solution::new();
        let mut remaining_panels = panels.to_vec();

        // // Создаем нужное количество листов
        // for sheet_id in 0..sheet_count {
        //     let mut placement = Placement::new(&self.stock);
        //     placement.stock_id = sheet_id as i32;

        //     // Пытаемся разместить оставшиеся панели на этом листе
        //     let placed_count = placement.try_place_panels(&remaining_panels, self.cut_thickness);

        //     // Убираем размещенные панели из списка
        //     remaining_panels.drain(0..placed_count);

        //     solution.placements.push(placement);

        //     // Если все панели размещены - отлично
        //     if remaining_panels.is_empty() {
        //         break;
        //     }
        // }

        // solution.unplaced_panels = remaining_panels;
        solution
    }

    /// Вывод результата
    pub fn print_solution(&self, solution: &Solution) {
        println!("\n=== Результат оптимизации ===");
        println!(
            "Общая использованная площадь: {:.2}",
            solution.total_used_area as f64 / 100.0
        );
        println!(
            "Общая потерянная площадь: {:.2}",
            solution.total_waste_area as f64 / 100.0
        );
        println!(
            "Коэффициент использования: {:.2}%",
            solution.total_efficiency * 100.0
        );
        println!("Количество резов: {}", solution.total_cuts);
        println!(
            "Общая длина резов: {:.2}",
            solution.total_cut_length as f64 / 10.0
        );

        println!("\n=== Мозаики (листы с раскроем) ===");
        for (i, placement) in solution.placements.iter().enumerate() {
            println!("Лист {}:", i + 1);
            println!(
                "  Использование: {:.2}% ({:.2}/{:.2})",
                placement.efficiency * 100.0,
                placement.used_area as f64 / 100.0,
                (placement.used_area + placement.waste_area) as f64 / 100.0
            );

            for panel in &placement.placed_panels {
                println!(
                    "    {:.1}x{:.1} [{}]",
                    panel.width as f64 / 10.0,
                    panel.height as f64 / 10.0,
                    panel.label
                );
            }
        }

        if solution.unplaced_panels.is_empty() {
            println!("\n=== Все детали размещены успешно! ===");
        } else {
            println!("\n=== Неразмещенные детали ===");
            for panel in &solution.unplaced_panels {
                // println!(
                //     "  {:.1}x{:.1} [{}]",
                //     panel.width as f64 / 10.0,
                //     panel.height as f64 / 10.0,
                //     panel.label
                // );
            }
        }
    }
}
