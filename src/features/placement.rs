

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

    /// ✅ ИСПРАВЛЕНО: Попытка разместить панель точно как в Java CutListThread.add()
    /// Java логика: панели поворачиваются автоматически если не квадратные и нет ограничений на направление волокон
    pub fn try_place_panel(&mut self, panel: &Panel, cut_thickness: i32, consider_grain_direction: bool) -> bool {
        // ✅ Точная копия Java логики из CutListThread.add()
        
        // Пробуем сначала исходную ориентацию
        if self.try_place_panel_with_orientation(panel, cut_thickness) {
            return true;
        }
        
        // ✅ Java логика: если панель квадратная, дальше не пробуем (tileDimensions.isSquare())
        if panel.width == panel.height {
            return false;
        }
        
        // ✅ Java логика: если нужно учитывать направление волокон и есть ограничения, не поворачиваем
        if consider_grain_direction {
            // В реальной Java реализации здесь проверяется ориентация мозаики и панели
            // Для упрощения пока запрещаем поворот при consider_grain_direction=true
            return false;
        }
        
        // ✅ Java логика: пробуем повернутую ориентацию (tileDimensions.rotate90())
        let rotated_panel = Panel {
            id: panel.id,
            width: panel.height, // Поворот на 90°
            height: panel.width,
            count: panel.count,
            label: panel.label.clone(),
            material: panel.material.clone(),
        };
        
        self.try_place_panel_with_orientation(&rotated_panel, cut_thickness)
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

    /// ✅ ТОЧНАЯ КОПИЯ JAVA: fitTile метод - создает все возможные варианты размещения панели
    fn create_placement_variants(&self, panel: &Panel, cut_thickness: i32) -> Vec<Placement> {
        let mut variants = Vec::new();
        
        // ✅ Java: findCandidates() - найти все подходящие позиции
        let candidates = self.root_node.find_candidates(panel.width as i32, panel.height as i32);
        
        for candidate in candidates {
            // ✅ Java: проверка точного совпадения размеров
            if candidate.width() == panel.width as i32 && candidate.height() == panel.height as i32 {
                // ✅ Точное совпадение - просто пометить как финальный узел
                if let Ok(exact_variant) = self.try_create_exact_fit_variant(&candidate, panel) {
                    variants.push(exact_variant);
                }
            } else {
                // ✅ Java: нужны разрезы - применить обе стратегии HV и VH
                
                // HV стратегия (горизонтальный разрез первый)
                if let Ok(hv_variant) = self.try_create_hv_variant(&candidate, panel, cut_thickness) {
                    variants.push(hv_variant);
                }
                
                // VH стратегия (вертикальный разрез первый)  
                if let Ok(vh_variant) = self.try_create_vh_variant(&candidate, panel, cut_thickness) {
                    variants.push(vh_variant);
                }
            }
        }
        
        // ✅ Java: возвращаем все варианты (сортировка происходит на уровне решений)
        variants
    }
    
    /// ✅ НОВЫЙ МЕТОД: Создание варианта с точным совпадением размеров
    fn try_create_exact_fit_variant(&self, candidate: &Node, panel: &Panel) -> Result<Placement, String> {
        let mut test_root = self.root_node.clone();
        
        // Найти соответствующий узел в копии и пометить как финальный
        if let Some(target_node) = test_root.find_node_by_coordinates(candidate.rect.x, candidate.rect.y) {
            target_node.set_final(true);
            target_node.set_panel_id(panel.id as i32 as i32);
            
            let mut variant = self.clone();
            variant.root_node = test_root;
            variant.update_statistics();
            Ok(variant)
        } else {
            Err("Target node not found".to_string())
        }
    }

    /// ✅ ТОЧНАЯ КОПИЯ JAVA: HV стратегия разрезания (splitHV)
    fn try_create_hv_variant(&self, candidate: &Node, panel: &Panel, cut_thickness: i32) -> Result<Placement, String> {
        let mut test_root = self.root_node.clone();
        
        // Найти соответствующий узел в копии дерева
        if let Some(target_node) = test_root.find_node_by_coordinates(candidate.rect.x, candidate.rect.y) {
            let new_cuts = Self::split_hv(target_node, panel, cut_thickness)?;
            
            let mut variant = self.clone();
            variant.root_node = test_root;
            variant.cuts.extend(new_cuts);
            variant.update_statistics();
            Ok(variant)
        } else {
            Err("Target node not found for HV split".to_string())
        }
    }

    /// ✅ ТОЧНАЯ КОПИЯ JAVA: VH стратегия разрезания (splitVH)
    fn try_create_vh_variant(&self, candidate: &Node, panel: &Panel, cut_thickness: i32) -> Result<Placement, String> {
        let mut test_root = self.root_node.clone();
        
        // Найти соответствующий узел в копии дерева
        if let Some(target_node) = test_root.find_node_by_coordinates(candidate.rect.x, candidate.rect.y) {
            let new_cuts = Self::split_vh(target_node, panel, cut_thickness)?;
            
            let mut variant = self.clone();
            variant.root_node = test_root;
            variant.cuts.extend(new_cuts);
            variant.update_statistics();
            Ok(variant)
        } else {
            Err("Target node not found for VH split".to_string())
        }
    }
    
    /// ✅ ТОЧНАЯ КОПИЯ JAVA: splitHV - горизонтальный разрез первый
    fn split_hv(node: &mut Node, panel: &Panel, cut_thickness: i32) -> Result<Vec<Cut>, String> {
        let mut cuts = Vec::new();
        
        if node.width() > panel.width as i32 {
            // ✅ Java: сначала горизонтальный разрез
            let horizontal_cut = Self::split_horizontally(node, panel.width as i32, cut_thickness, panel.id as i32)?;
            cuts.push(horizontal_cut);
            
            // ✅ Java: затем вертикальный разрез левого дочернего узла
            if node.height() > panel.height as i32 {
                if let Some(child1) = &mut node.child1 {
                    let vertical_cut = Self::split_vertically(child1, panel.height as i32, cut_thickness, panel.id as i32)?;
                    cuts.push(vertical_cut);
                    
                    // ✅ Java: пометить левый-левый узел как финальный (для панели)
                    if let Some(child1_child1) = &mut child1.child1 {
                        child1_child1.set_final(true);
                        child1_child1.set_panel_id(panel.id as i32);
                    }
                }
            } else {
                // ✅ Java: только горизонтальный разрез нужен
                if let Some(child1) = &mut node.child1 {
                    child1.set_final(true);
                    child1.set_panel_id(panel.id as i32);
                }
            }
        } else {
            // ✅ Java: только вертикальный разрез нужен
            let vertical_cut = Self::split_vertically(node, panel.height as i32, cut_thickness, panel.id as i32)?;
            cuts.push(vertical_cut);
            
            if let Some(child1) = &mut node.child1 {
                child1.set_final(true);
                child1.set_panel_id(panel.id as i32);
            }
        }
        
        Ok(cuts)
    }
    
    /// ✅ ТОЧНАЯ КОПИЯ JAVA: splitVH - вертикальный разрез первый
    fn split_vh(node: &mut Node, panel: &Panel, cut_thickness: i32) -> Result<Vec<Cut>, String> {
        let mut cuts = Vec::new();
        
        if node.height() > panel.height as i32 {
            // ✅ Java: сначала вертикальный разрез
            let vertical_cut = Self::split_vertically(node, panel.height as i32, cut_thickness, panel.id as i32)?;
            cuts.push(vertical_cut);
            
            // ✅ Java: затем горизонтальный разрез левого дочернего узла
            if node.width() > panel.width as i32 {
                if let Some(child1) = &mut node.child1 {
                    let horizontal_cut = Self::split_horizontally(child1, panel.width as i32, cut_thickness, panel.id as i32)?;
                    cuts.push(horizontal_cut);
                    
                    // ✅ Java: пометить левый-левый узел как финальный (для панели)
                    if let Some(child1_child1) = &mut child1.child1 {
                        child1_child1.set_final(true);
                        child1_child1.set_panel_id(panel.id as i32);
                    }
                }
            } else {
                // ✅ Java: только вертикальный разрез нужен
                if let Some(child1) = &mut node.child1 {
                    child1.set_final(true);
                    child1.set_panel_id(panel.id as i32);
                }
            }
        } else {
            // ✅ Java: только горизонтальный разрез нужен
            let horizontal_cut = Self::split_horizontally(node, panel.width as i32, cut_thickness, panel.id as i32)?;
            cuts.push(horizontal_cut);
            
            if let Some(child1) = &mut node.child1 {
                child1.set_final(true);
                child1.set_panel_id(panel.id as i32);
            }
        }
        
        Ok(cuts)
    }
    
    /// ✅ ТОЧНАЯ КОПИЯ JAVA: splitHorizontally - создание горизонтального разреза
    fn split_horizontally(node: &mut Node, panel_width: i32, cut_thickness: i32, panel_id: i32) -> Result<Cut, String> {
        let original_width = node.width();
        let original_height = node.height();
        
        // ✅ Java: левый дочерний узел (остается для панели)
        let child1_rect = Rectangle::new(
            node.rect.x,
            node.rect.x + panel_width,
            node.rect.y,
            node.rect.y2()
        );
        let mut child1 = Node::new(0, child1_rect); // ID будет установлен позже
        child1.set_panel_id(panel_id);
        
        let child1_area = child1.area();
        if child1_area > 0 {
            node.child1 = Some(Box::new(child1));
        }
        
        // ✅ Java: правый дочерний узел (остаток)
        let child2_x1 = node.rect.x + panel_width + cut_thickness;
        if child2_x1 < node.rect.x2() {
            let child2_rect = Rectangle::new(
                child2_x1,
                node.rect.x2(),
                node.rect.y,
                node.rect.y2()
            );
            let child2 = Node::new(0, child2_rect); // ID будет установлен позже
            let child2_area = child2.area();
            if child2_area > 0 {
                node.child2 = Some(Box::new(child2));
            }
        }
        
        // ✅ Java: создание информации о разрезе
        Ok(Cut {
            x1: node.rect.x + panel_width,
            y1: node.rect.y,
            x2: node.rect.x + panel_width,
            y2: node.rect.y2(),
            is_horizontal: false, // Вертикальная линия разреза
        })
    }
    
    /// ✅ ТОЧНАЯ КОПИЯ JAVA: splitVertically - создание вертикального разреза
    fn split_vertically(node: &mut Node, panel_height: i32, cut_thickness: i32, panel_id: i32) -> Result<Cut, String> {
        let original_width = node.width();
        let original_height = node.height();
        
        // ✅ Java: верхний дочерний узел (остается для панели)
        let child1_rect = Rectangle::new(
            node.rect.x,
            node.rect.x2(),
            node.rect.y,
            node.rect.y + panel_height
        );
        let mut child1 = Node::new(0, child1_rect); // ID будет установлен позже
        child1.set_panel_id(panel_id);
        
        let child1_area = child1.area();
        if child1_area > 0 {
            node.child1 = Some(Box::new(child1));
        }
        
        // ✅ Java: нижний дочерний узел (остаток)
        let child2_y1 = node.rect.y + panel_height + cut_thickness;
        if child2_y1 < node.rect.y2() {
            let child2_rect = Rectangle::new(
                node.rect.x,
                node.rect.x2(),
                child2_y1,
                node.rect.y2()
            );
            let child2 = Node::new(0, child2_rect); // ID будет установлен позже
            let child2_area = child2.area();
            if child2_area > 0 {
                node.child2 = Some(Box::new(child2));
            }
        }
        
        // ✅ Java: создание информации о разрезе
        Ok(Cut {
            x1: node.rect.x,
            y1: node.rect.y + panel_height,
            x2: node.rect.x2(),
            y2: node.rect.y + panel_height,
            is_horizontal: true, // Горизонтальная линия разреза
        })
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
        if node.rect.x == target.rect.x && 
           node.rect.y == target.rect.y &&
           node.rect.width == target.rect.width &&
           node.rect.height == target.rect.height &&
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
        if node.rect.x == target.rect.x && 
           node.rect.y == target.rect.y &&
           node.rect.width == target.rect.width &&
           node.rect.height == target.rect.height &&
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
        if node.rect.x == target.rect.x && 
           node.rect.y == target.rect.y &&
           node.rect.width == target.rect.width &&
           node.rect.height == target.rect.height &&
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
    pub fn try_place_panels(&mut self, panels: &[Panel], cut_thickness: i32, consider_grain_direction: bool) -> usize {
        let mut placed_count = 0;

        for panel in panels {
            // TODO: Попробовать разместить панель и ее поворот
            // Взять логику из основного цикла computeSolutions()

            // Пробуем разместить панель как есть
            if self.try_place_panel(panel, cut_thickness, consider_grain_direction) {
                placed_count += 1;
                continue;
            }

            // Пробуем повернуть и разместить
            // let rotated = panel.rotate();
            // if self.try_place_panel(&rotated, cut_thickness, consider_grain_direction) {
            //     placed_count += 1;
            //     continue;
            // }

            // Панель не поместилась
            break;
        }

        self.update_statistics();
        placed_count
    }

    pub fn update_statistics(&mut self) {
        self.used_area = self.root_node.get_used_area();
        let total_area = self.root_node.rect.area();
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
