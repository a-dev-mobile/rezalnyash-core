

// ============================================================================
// ЭТАП 3: РАЗМЕЩЕНИЕ НА ОДНОМ ЛИСТЕ
// ============================================================================

use crate::features::{cut::Cut, input::models::{panel::Panel}, node::Node, placed_panel::PlacedPanel, rectangle::Rectangle};

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
    pub fn new(stock: &Panel) -> Self {
        let root_rect = Rectangle::new(0, 0, stock.width as i32, stock.height as i32);
        let total_area = root_rect.area();
        let root_node = Node::new(0, root_rect);

        Self {
            stock_id: stock.id as i32,
            root_node,
            cuts: Vec::new(),
            placed_panels: Vec::new(),
            used_area: 0,
            waste_area: total_area,
            efficiency: 0.0,
        }
    }

    /// Попытка разместить панель
    /// Основная логика из computeSolutions() в CutListThread.java
    pub fn try_place_panel(&mut self, panel: &Panel, cut_thickness: i32) -> bool {
        // Сначала пробуем разместить панель в исходной ориентации
        if self.try_place_panel_with_orientation(panel, cut_thickness) {
            return true;
        }
        
        // Если панель не квадратная, пробуем повернуть на 90 градусов
        if !panel.is_square() {
            let rotated_panel = panel.rotate();
            if self.try_place_panel_with_orientation(&rotated_panel, cut_thickness) {
                return true;
            }
        }
        
        false
    }

    /// Попытка разместить панель в определенной ориентации
    fn try_place_panel_with_orientation(&mut self, panel: &Panel, cut_thickness: i32) -> bool {
        let placement_variants = self.create_placement_variants(panel, cut_thickness);
        
        if let Some(best_variant) = placement_variants.into_iter().next() {
            // Выбираем первый (лучший) вариант и применяем его
            self.root_node = best_variant.root_node;
            self.cuts = best_variant.cuts;
            self.update_statistics();
            return true;
        }
        
        false
    }

    /// Создает все возможные варианты размещения панели (HV и VH) как в Java
    fn create_placement_variants(&self, panel: &Panel, cut_thickness: i32) -> Vec<Placement> {
        let mut variants = Vec::new();
        let candidates = self.root_node.find_candidates(panel.width as i32, panel.height as i32);
        
        for candidate_ref in candidates {
            // Пробуем HV размещение
            if let Ok(hv_variant) = self.try_create_hv_variant(candidate_ref, panel, cut_thickness) {
                variants.push(hv_variant);
            }
            
            // Пробуем VH размещение  
            if let Ok(vh_variant) = self.try_create_vh_variant(candidate_ref, panel, cut_thickness) {
                variants.push(vh_variant);
            }
        }
        
        // Сортируем варианты по качеству (меньше резов = лучше)
        variants.sort_by(|a, b| a.cuts.len().cmp(&b.cuts.len()));
        variants
    }

    /// Создает вариант размещения с HV порядком
    fn try_create_hv_variant(&self, candidate: &Node, panel: &Panel, cut_thickness: i32) -> Result<Placement, String> {
        let mut test_root = self.root_node.clone();
        let new_cuts = Self::place_in_copy_hv(&mut test_root, candidate, panel, cut_thickness)?;
        
        let mut variant = self.clone();
        variant.root_node = test_root;
        variant.cuts.extend(new_cuts);
        variant.update_statistics();
        Ok(variant)
    }

    /// Создает вариант размещения с VH порядком
    fn try_create_vh_variant(&self, candidate: &Node, panel: &Panel, cut_thickness: i32) -> Result<Placement, String> {
        let mut test_root = self.root_node.clone();
        let new_cuts = Self::place_in_copy_vh(&mut test_root, candidate, panel, cut_thickness)?;
        
        let mut variant = self.clone();
        variant.root_node = test_root;
        variant.cuts.extend(new_cuts);
        variant.update_statistics();
        Ok(variant)
    }

    fn place_in_copy(root: &mut Node, target: &Node, panel: &Panel, cut_thickness: i32) -> Result<Vec<Cut>, String> {
        // Простая реализация: найти узел с теми же координатами и попробовать разместить
        Self::find_and_place_recursive(root, target, panel, cut_thickness)
    }

    /// Размещение с HV порядком в копии
    fn place_in_copy_hv(root: &mut Node, target: &Node, panel: &Panel, cut_thickness: i32) -> Result<Vec<Cut>, String> {
        Self::find_and_place_recursive_hv(root, target, panel, cut_thickness)
    }

    /// Размещение с VH порядком в копии
    fn place_in_copy_vh(root: &mut Node, target: &Node, panel: &Panel, cut_thickness: i32) -> Result<Vec<Cut>, String> {
        Self::find_and_place_recursive_vh(root, target, panel, cut_thickness)
    }

    fn find_and_place_recursive(node: &mut Node, target: &Node, panel: &Panel, cut_thickness: i32) -> Result<Vec<Cut>, String> {
        if node.rectangle.x == target.rectangle.x && 
           node.rectangle.y == target.rectangle.y &&
           node.rectangle.width == target.rectangle.width &&
           node.rectangle.height == target.rectangle.height &&
           node.child1.is_none() && node.child2.is_none() {
            return node.place_panel(panel, cut_thickness);
        }

        if let Some(ref mut child1) = node.child1 {
            if let Ok(cuts) = Self::find_and_place_recursive(child1, target, panel, cut_thickness) {
                return Ok(cuts);
            }
        }

        if let Some(ref mut child2) = node.child2 {
            if let Ok(cuts) = Self::find_and_place_recursive(child2, target, panel, cut_thickness) {
                return Ok(cuts);
            }
        }

        Err("Target node not found".to_string())
    }

    fn find_and_place_recursive_hv(node: &mut Node, target: &Node, panel: &Panel, cut_thickness: i32) -> Result<Vec<Cut>, String> {
        if node.rectangle.x == target.rectangle.x && 
           node.rectangle.y == target.rectangle.y &&
           node.rectangle.width == target.rectangle.width &&
           node.rectangle.height == target.rectangle.height &&
           node.child1.is_none() && node.child2.is_none() {
            return node.place_panel_hv(panel, cut_thickness);
        }

        if let Some(ref mut child1) = node.child1 {
            if let Ok(cuts) = Self::find_and_place_recursive_hv(child1, target, panel, cut_thickness) {
                return Ok(cuts);
            }
        }

        if let Some(ref mut child2) = node.child2 {
            if let Ok(cuts) = Self::find_and_place_recursive_hv(child2, target, panel, cut_thickness) {
                return Ok(cuts);
            }
        }

        Err("Target node not found".to_string())
    }

    fn find_and_place_recursive_vh(node: &mut Node, target: &Node, panel: &Panel, cut_thickness: i32) -> Result<Vec<Cut>, String> {
        if node.rectangle.x == target.rectangle.x && 
           node.rectangle.y == target.rectangle.y &&
           node.rectangle.width == target.rectangle.width &&
           node.rectangle.height == target.rectangle.height &&
           node.child1.is_none() && node.child2.is_none() {
            return node.place_panel_vh(panel, cut_thickness);
        }

        if let Some(ref mut child1) = node.child1 {
            if let Ok(cuts) = Self::find_and_place_recursive_vh(child1, target, panel, cut_thickness) {
                return Ok(cuts);
            }
        }

        if let Some(ref mut child2) = node.child2 {
            if let Ok(cuts) = Self::find_and_place_recursive_vh(child2, target, panel, cut_thickness) {
                return Ok(cuts);
            }
        }

        Err("Target node not found".to_string())
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
        
        // Обновляем список размещенных панелей
        self.placed_panels = self.root_node.get_final_panels();
    }
}
