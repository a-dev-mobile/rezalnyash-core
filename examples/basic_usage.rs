// Главный скрипт оптимизации раскроя
// Упрощенная однопоточная версия Java-алгоритма

use std::collections::HashMap;

use rezalnyas_core::save_to_json::save_to_json::save_to_json;
use serde::Serialize;

// ============================================================================
// ЭТАП 1: БАЗОВЫЕ СТРУКТУРЫ ДАННЫХ
// ============================================================================

/// Базовый прямоугольник для всех геометрических операций
#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rectangle {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn area(&self) -> i64 {
        (self.width as i64) * (self.height as i64)
    }

    pub fn fits(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

/// Деталь для размещения
#[derive(Serialize, Debug, Clone)]
pub struct Panel {
    pub id: i32,
    pub width: i32,
    pub height: i32,
    pub count: i32,
    pub label: String,
    pub is_rotated: bool,
}

impl Panel {
    pub fn new(id: i32, width: i32, height: i32, count: i32, label: &str) -> Self {
        Self {
            id,
            width,
            height,
            count,
            label: label.to_string(),
            is_rotated: false,
        }
    }

    /// Поворот детали на 90 градусов
    pub fn rotate(&self) -> Panel {
        Panel {
            id: self.id,
            width: self.height,
            height: self.width,
            count: self.count,
            label: self.label.clone(),
            is_rotated: !self.is_rotated,
        }
    }

    pub fn fits_in(&self, rect: &Rectangle) -> bool {
        (self.width <= rect.width && self.height <= rect.height)
            || (self.height <= rect.width && self.width <= rect.height)
    }

    pub fn area(&self) -> i64 {
        (self.width as i64) * (self.height as i64)
    }
}

/// Заготовка (исходный лист материала)
#[derive(Debug, Clone)]
pub struct Stock {
    pub id: i32,
    pub width: i32,
    pub height: i32,
    pub label: String,
}

impl Stock {
    pub fn new(id: i32, width: i32, height: i32, label: &str) -> Self {
        Self {
            id,
            width,
            height,
            label: label.to_string(),
        }
    }

    pub fn area(&self) -> i64 {
        (self.width as i64) * (self.height as i64)
    }
}

/// Рез на листе
#[derive(Debug, Clone)]
pub struct Cut {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    pub is_horizontal: bool,
}

impl Cut {
    pub fn new_horizontal(x: i32, y: i32, length: i32) -> Self {
        Self {
            x1: x,
            y1: y,
            x2: x + length,
            y2: y,
            is_horizontal: true,
        }
    }

    pub fn new_vertical(x: i32, y: i32, length: i32) -> Self {
        Self {
            x1: x,
            y1: y,
            x2: x,
            y2: y + length,
            is_horizontal: false,
        }
    }

    pub fn length(&self) -> i32 {
        if self.is_horizontal {
            self.x2 - self.x1
        } else {
            self.y2 - self.y1
        }
    }
}

// ============================================================================
// ЭТАП 2: ДЕРЕВО РАЗРЕЗОВ (Tree Structure)
// ============================================================================

/// Узел дерева разрезов
/// TODO: Взять логику из TileNode.java
#[derive(Debug, Clone)]
pub struct Node {
    pub id: i32,
    pub rectangle: Rectangle,
    pub left_child: Option<Box<Node>>,
    pub right_child: Option<Box<Node>>,
    pub panel_id: Option<i32>, // ID размещенной детали
    pub is_used: bool,
    pub is_rotated: bool,
}

impl Node {
    pub fn new(id: i32, rect: Rectangle) -> Self {
        Self {
            id,
            rectangle: rect,
            left_child: None,
            right_child: None,
            panel_id: None,
            is_used: false,
            is_rotated: false,
        }
    }

    /// Поиск кандидатов для размещения панели
    /// TODO: Реализовать по образцу findCandidates() из CutListThread.java
    pub fn find_candidates(&self, panel_width: i32, panel_height: i32) -> Vec<&Node> {
        // TODO: Рекурсивный поиск подходящих узлов
        // 1. Проверить, подходит ли текущий узел
        // 2. Если есть дети - искать в них рекурсивно
        // 3. Вернуть список подходящих узлов
        vec![]
    }

    /// Разместить панель в узле
    /// TODO: Реализовать по образцу fitTile() из CutListThread.java
    pub fn place_panel(&mut self, panel: &Panel, cut_thickness: i32) -> Result<Vec<Cut>, String> {
        // TODO:
        // 1. Проверить, помещается ли панель
        // 2. Если точно совпадает - пометить как использованный
        // 3. Если больше - создать разрезы и дочерние узлы
        // 4. Вернуть список созданных резов
        Err("Not implemented".to_string())
    }

    /// Горизонтальный разрез
    /// TODO: Взять из splitHorizontally() в CutListThread.java
    pub fn split_horizontal(&mut self, cut_position: i32, cut_thickness: i32) -> Cut {
        // TODO: Создать два дочерних узла и вернуть рез
        todo!("Implement horizontal split from splitHorizontally() method")
    }

    /// Вертикальный разрез  
    /// TODO: Взять из splitVertically() в CutListThread.java
    pub fn split_vertical(&mut self, cut_position: i32, cut_thickness: i32) -> Cut {
        // TODO: Создать два дочерних узла и вернуть рез
        todo!("Implement vertical split from splitVertically() method")
    }

    /// Получить использованную площадь
    pub fn get_used_area(&self) -> i64 {
        // TODO: Рекурсивно собрать площадь всех использованных узлов
        0
    }

    /// Получить все финальные панели
    pub fn get_final_panels(&self) -> Vec<PlacedPanel> {
        // TODO: Рекурсивно собрать все размещенные панели
        vec![]
    }
}

/// Размещенная панель с координатами
#[derive(Debug, Clone)]
pub struct PlacedPanel {
    pub panel_id: i32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub is_rotated: bool,
    pub label: String,
}

// ============================================================================
// ЭТАП 3: РАЗМЕЩЕНИЕ НА ОДНОМ ЛИСТЕ
// ============================================================================

/// Результат размещения на одном листе
#[derive(Debug, Clone)]
pub struct Placement {
    pub stock_id: i32,
    pub root_node: Node,
    pub cuts: Vec<Cut>,
    pub placed_panels: Vec<PlacedPanel>,
    pub used_area: i64,
    pub waste_area: i64,
    pub efficiency: f64,
}

impl Placement {
    pub fn new(stock: &Stock) -> Self {
        let root_rect = Rectangle::new(0, 0, stock.width, stock.height);
        let root_node = Node::new(0, root_rect);

        Self {
            stock_id: stock.id,
            root_node,
            cuts: Vec::new(),
            placed_panels: Vec::new(),
            used_area: 0,
            waste_area: stock.area(),
            efficiency: 0.0,
        }
    }

    /// Попытка разместить панель
    /// TODO: Основная логика из computeSolutions() в CutListThread.java
    pub fn try_place_panel(&mut self, panel: &Panel, cut_thickness: i32) -> bool {
        // TODO:
        // 1. Найти подходящие места (find_candidates)
        // 2. Для каждого места попробовать разместить
        // 3. Выбрать лучшее размещение
        // 4. Обновить статистику
        false
    }

    /// Попытка разместить список панелей в определенном порядке
    /// TODO: Основной цикл из computeSolutions()
    pub fn try_place_panels(&mut self, panels: &[Panel], cut_thickness: i32) -> usize {
        let mut placed_count = 0;

        for panel in panels {
            // TODO: Попробовать разместить панель и ее поворот
            // Взять логику из основного цикла computeSolutions()

            // Пробуем разместить панель как есть
            if self.try_place_panel(panel, cut_thickness) {
                placed_count += 1;
                continue;
            }

            // Пробуем повернуть и разместить
            let rotated = panel.rotate();
            if self.try_place_panel(&rotated, cut_thickness) {
                placed_count += 1;
                continue;
            }

            // Панель не поместилась
            break;
        }

        self.update_statistics();
        placed_count
    }

    fn update_statistics(&mut self) {
        self.used_area = self.root_node.get_used_area();
        let total_area = self.root_node.rectangle.area();
        self.waste_area = total_area - self.used_area;
        self.efficiency = if total_area > 0 {
            self.used_area as f64 / total_area as f64
        } else {
            0.0
        };
    }
}

// ============================================================================
// ЭТАП 4: ПОЛНОЕ РЕШЕНИЕ
// ============================================================================

/// Полное решение задачи раскроя
#[derive(Debug, Clone)]
pub struct Solution {
    pub placements: Vec<Placement>,
    pub unplaced_panels: Vec<Panel>,
    pub total_efficiency: f64,
    pub total_cuts: usize,
    pub total_cut_length: i32,
    pub total_used_area: i64,
    pub total_waste_area: i64,
}

impl Solution {
    pub fn new() -> Self {
        Self {
            placements: Vec::new(),
            unplaced_panels: Vec::new(),
            total_efficiency: 0.0,
            total_cuts: 0,
            total_cut_length: 0,
            total_used_area: 0,
            total_waste_area: 0,
        }
    }

    pub fn calculate_totals(&mut self) {
        self.total_used_area = self.placements.iter().map(|p| p.used_area).sum();
        self.total_waste_area = self.placements.iter().map(|p| p.waste_area).sum();
        self.total_cuts = self.placements.iter().map(|p| p.cuts.len()).sum();
        self.total_cut_length = self
            .placements
            .iter()
            .flat_map(|p| &p.cuts)
            .map(|c| c.length())
            .sum();

        let total_area = self.total_used_area + self.total_waste_area;
        self.total_efficiency = if total_area > 0 {
            self.total_used_area as f64 / total_area as f64
        } else {
            0.0
        };
    }

    /// Оценка качества решения для сортировки
    /// TODO: Взять приоритеты из SolutionComparatorFactory.java
    pub fn score(&self) -> (i32, i64, i32) {
        // Приоритет 1: Максимум размещенных панелей (отрицательное для сортировки по убыванию)
        let placed_panels = -(self
            .placements
            .iter()
            .map(|p| p.placed_panels.len() as i32)
            .sum::<i32>());

        // Приоритет 2: Минимум отходов
        let waste_area = self.total_waste_area;

        // Приоритет 3: Минимум резов
        let cuts_count = self.total_cuts as i32;

        (placed_panels, waste_area, cuts_count)
    }
}

// ============================================================================
// ЭТАП 5: ГЕНЕРАЦИЯ ПЕРЕСТАНОВОК
// ============================================================================

/// Генератор перестановок панелей
/// TODO: Взять из Arrangement.generatePermutations() и упростить
pub struct PermutationGenerator;

impl PermutationGenerator {
    /// Генерация всех перестановок панелей
    /// TODO: Реализовать по образцу Arrangement.generatePermutations()
    pub fn generate_permutations<T: Clone>(items: Vec<T>) -> Vec<Vec<T>> {
        if items.is_empty() {
            return vec![vec![]];
        }

        // TODO: Рекурсивная генерация перестановок
        // Взять алгоритм из Arrangement.java

        // Пока заглушка - возвращаем исходный порядок
        vec![items]
    }

    /// Группировка одинаковых панелей для оптимизации
    /// Реализовано по образцу generateGroups() из CutListOptimizerServiceImpl.java
    pub fn group_panels(panels: &[Panel]) -> Vec<Panel> {
        // Создаем карту для подсчета количества одинаковых панелей
        let mut panel_counts: HashMap<String, i32> = HashMap::new();

        // Подсчитываем количество каждого типа панели
        for panel in panels {
            let key = format!("{}x{}", panel.width, panel.height);
            *panel_counts.entry(key).or_insert(0) += 1;
        }

        // Проверяем, является ли это одномерной оптимизацией
        let is_one_dimensional = Self::is_one_dimensional_optimization(panels);

        // Определяем максимальный размер группы
        let max_group_size: usize = if is_one_dimensional {
            1 // Для одномерной оптимизации группы не разбиваем
        } else {
            std::cmp::max(panels.len() / 100, 1)
        };

        let mut result = Vec::new();
        let mut current_group_counts: HashMap<String, i32> = HashMap::new();
        let mut group_id = 0;

        for panel in panels {
            let panel_key = format!("{}x{}", panel.width, panel.height);
            let group_key = format!("{}{}", panel_key, group_id);

            // Увеличиваем счетчик для текущей группы
            let current_count = current_group_counts.entry(group_key.clone()).or_insert(0);
            *current_count += 1;

            // Создаем новую панель с group_id в качестве модификатора ID
            let mut grouped_panel = panel.clone();
            grouped_panel.id = panel.id * 1000 + group_id;
            result.push(grouped_panel);

            // Проверяем, нужно ли создать новую группу
            let total_count = panel_counts.get(&panel_key).unwrap_or(&0);
            if *total_count > max_group_size as i32 && *current_count > total_count / 4 {
                group_id += 1;
            }
        }

        result
    }

    /// Проверка является ли оптимизация одномерной
    fn is_one_dimensional_optimization(panels: &[Panel]) -> bool {
        if panels.is_empty() {
            return false;
        }

        let mut common_dimensions = vec![panels[0].width, panels[0].height];

        // Проверяем панели
        for panel in panels {
            // Удаляем размеры, которые не встречаются в текущей панели
            common_dimensions.retain(|&dim| dim == panel.width || dim == panel.height);

            if common_dimensions.is_empty() {
                return false;
            }
        }

        !common_dimensions.is_empty()
    }
}

// ============================================================================
// ЭТАП 6: ГЛАВНЫЙ ОПТИМИЗАТОР
// ============================================================================

/// Главный класс оптимизатора
pub struct CuttingOptimizer {
    pub panels: Vec<Panel>,
    pub stock: Stock,
    pub cut_thickness: i32,
    pub max_sheets: usize,
}

impl CuttingOptimizer {
    pub fn new(panels: Vec<Panel>, stock: Stock) -> Self {
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
            self.stock.width,
            self.stock.height
        );

        // ЭТАП 1: Развернуть панели по количеству
        let expanded_panels = self.expand_panels();
        println!("Развернуто панелей: {}", expanded_panels.len());
        save_to_json(&expanded_panels, "expanded_panels.json");
        // ЭТАП 2: Сгруппировать панели для оптимизации перестановок
        let grouped_panels = PermutationGenerator::group_panels(&expanded_panels);
        save_to_json(&grouped_panels, "grouped_panels.json");
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
                new_panel.id = panel.id * 1000 + i;
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

        // Создаем нужное количество листов
        for sheet_id in 0..sheet_count {
            let mut placement = Placement::new(&self.stock);
            placement.stock_id = sheet_id as i32;

            // Пытаемся разместить оставшиеся панели на этом листе
            let placed_count = placement.try_place_panels(&remaining_panels, self.cut_thickness);

            // Убираем размещенные панели из списка
            remaining_panels.drain(0..placed_count);

            solution.placements.push(placement);

            // Если все панели размещены - отлично
            if remaining_panels.is_empty() {
                break;
            }
        }

        solution.unplaced_panels = remaining_panels;
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
                println!(
                    "  {:.1}x{:.1} [{}]",
                    panel.width as f64 / 10.0,
                    panel.height as f64 / 10.0,
                    panel.label
                );
            }
        }
    }
}

// ============================================================================
// ЭТАП 7: ПРИМЕР ИСПОЛЬЗОВАНИЯ
// ============================================================================

fn main() {
    println!("=== Тест оптимизации раскроя ===");

    // Создаем детали (размеры в мм * 10 для точности)
    let panels = vec![
        Panel::new(1, 1000, 500, 2, "Деталь_1"), // 100.0x50.0 мм, 2 шт
        Panel::new(2, 800, 600, 1, "Деталь_2"),  // 80.0x60.0 мм, 1 шт
    ];

    // Заготовка (размеры в мм * 10)
    let stock = Stock::new(1, 3000, 2000, "Заготовка_1"); // 300.0x200.0 мм

    // Создаем оптимизатор
    let optimizer = CuttingOptimizer::new(panels, stock);

    // Запускаем оптимизацию
    let solution = optimizer.optimize();

    // Выводим результат
    optimizer.print_solution(&solution);
}
