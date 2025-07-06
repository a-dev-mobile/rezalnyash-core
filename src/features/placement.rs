

// ============================================================================
// ЭТАП 3: РАЗМЕЩЕНИЕ НА ОДНОМ ЛИСТЕ
// ============================================================================

use crate::features::{cut::Cut, input::models::{panel::Panel, stock::Stock}, node::Node, placed_panel::PlacedPanel, rectangle::Rectangle};

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
    // pub fn new(stock: &Stock) -> Self {
    //     let root_rect = Rectangle::new(0, 0, stock.width, stock.height);
    //     let root_node = Node::new(0, root_rect);

    //     Self {
    //         stock_id: stock.id,
    //         root_node,
    //         cuts: Vec::new(),
    //         placed_panels: Vec::new(),
    //         used_area: 0,
    //         waste_area: stock.area(),
    //         efficiency: 0.0,
    //     }
    // }

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
            // let rotated = panel.rotate();
            // if self.try_place_panel(&rotated, cut_thickness) {
            //     placed_count += 1;
            //     continue;
            // }

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
