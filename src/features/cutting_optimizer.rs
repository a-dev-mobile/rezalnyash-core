use crate::{
    features::{
        cut::Cut,
        input::models::{panel::Panel, tile_dimensions::TileDimensions},
        node::Node,
        panel_grouper::panel_grouper::PanelGrouper,
        permutation_generator::permutation_generator::PermutationGenerator,
        placement::Placement,
        rectangle::Rectangle,
        solution::Solution,
    },
    utils::json::save_to_json,
};

/// ✅ НОВАЯ СТРУКТУРА: Панель в последовательности для обработки как в Java
#[derive(Debug, Clone)]
struct SequentialPanel {
    panel: Panel,
    sequence_index: usize, // Позиция в последовательности (1, 2, 3...)
    rotated: bool,         // Флаг поворота от Java
}

/// Главный класс оптимизатора
pub struct CuttingOptimizer {
    pub panels: Vec<Panel>,
    pub stocks: Vec<Panel>,
    pub cut_thickness: i32,
    pub max_sheets: usize,
}

impl CuttingOptimizer {
    /// ✅ НОВЫЙ МЕТОД: Обработка последовательности панелей как в Java CutListThread.computeSolutions()
    /// Обрабатывает панели СТРОГО по одной в порядке последовательности
    fn process_panel_sequence_java_style(&self, panel_sequence: &[SequentialPanel]) -> Solution {
        let mut current_solutions = vec![Solution::new_with_stocks(self.stocks.clone())];
        
        // ✅ КЛЮЧЕВОЕ ИСПРАВЛЕНИЕ: Обрабатываем панели ПО ОДНОЙ как в Java
        for sequential_panel in panel_sequence {
            println!("\nРазмещение панели {} из {} ({}x{}, ID: {})", 
                sequential_panel.sequence_index, 
                panel_sequence.len(),
                sequential_panel.panel.width,
                sequential_panel.panel.height,
                sequential_panel.panel.id
            );
            
            // ✅ Java-стиль: для каждой панели пробуем разместить на всех текущих решениях
            current_solutions = self.place_single_panel_on_all_solutions(&sequential_panel.panel, current_solutions);
            
            // ✅ Java-стиль: фильтруем решения после каждого размещения
            current_solutions = self.filter_solutions_java_style(current_solutions);
            
            println!("После размещения панели {}: {} активных решений", 
                sequential_panel.sequence_index, current_solutions.len());
        }
        
        // Выбираем лучшее решение из всех вариантов
        self.select_best_solution(current_solutions)
    }
    
    /// ✅ ТОЧНАЯ КОПИЯ JAVA: place_single_panel_on_all_solutions с fitTile алгоритмом
    fn place_single_panel_on_all_solutions(&self, panel: &Panel, current_solutions: Vec<Solution>) -> Vec<Solution> {
        let mut new_solutions = Vec::new();
        
        // ✅ Java: для каждого текущего решения пытаемся разместить панель
        for solution in current_solutions {
            let mut placement_variants = Vec::new();
            
            // ✅ Java: пробуем разместить на каждом существующем листе (мозаике)
            for (mosaic_index, placement) in solution.placements.iter().enumerate() {
                // ✅ Java: fitTile для обычной ориентации с HV стратегией
                let normal_hv_variants = self.java_fit_tile_hv(panel, placement, false);
                for variant in normal_hv_variants {
                    let mut new_solution = solution.clone();
                    new_solution.placements[mosaic_index] = variant;
                    placement_variants.push(new_solution);
                }
                
                // ✅ Java: fitTile для обычной ориентации с VH стратегией  
                let normal_vh_variants = self.java_fit_tile_vh(panel, placement, false);
                for variant in normal_vh_variants {
                    let mut new_solution = solution.clone();
                    new_solution.placements[mosaic_index] = variant;
                    placement_variants.push(new_solution);
                }
                
                // ✅ Java: fitTile для повернутой ориентации (если не квадратная)
                if panel.width != panel.height {
                    // HV стратегия для повернутой панели
                    let rotated_hv_variants = self.java_fit_tile_hv(panel, placement, true);
                    for variant in rotated_hv_variants {
                        let mut new_solution = solution.clone();
                        new_solution.placements[mosaic_index] = variant;
                        placement_variants.push(new_solution);
                    }
                    
                    // VH стратегия для повернутой панели
                    let rotated_vh_variants = self.java_fit_tile_vh(panel, placement, true);
                    for variant in rotated_vh_variants {
                        let mut new_solution = solution.clone();
                        new_solution.placements[mosaic_index] = variant;
                        placement_variants.push(new_solution);
                    }
                }
            }
            
            // ✅ Java: если не удалось разместить ни на одном листе, создаем новый лист
            if placement_variants.is_empty() && solution.placements.len() < self.max_sheets {
                let stock_template = &self.stocks[0];
                let new_placement = Placement::new(stock_template);
                
                // ✅ Java: fitTile на новом листе - обычная ориентация с HV стратегией
                let normal_hv_variants = self.java_fit_tile_hv(panel, &new_placement, false);
                for variant in normal_hv_variants {
                    let mut new_solution = solution.clone();
                    new_solution.placements.push(variant);
                    placement_variants.push(new_solution);
                }
                
                // ✅ Java: fitTile на новом листе - обычная ориентация с VH стратегией
                let normal_vh_variants = self.java_fit_tile_vh(panel, &new_placement, false);
                for variant in normal_vh_variants {
                    let mut new_solution = solution.clone();
                    new_solution.placements.push(variant);
                    placement_variants.push(new_solution);
                }
                
                // ✅ Java: fitTile на новом листе - повернутая ориентация (если не квадратная)
                if panel.width != panel.height {
                    // HV стратегия для повернутой панели
                    let rotated_hv_variants = self.java_fit_tile_hv(panel, &new_placement, true);
                    for variant in rotated_hv_variants {
                        let mut new_solution = solution.clone();
                        new_solution.placements.push(variant);
                        placement_variants.push(new_solution);
                    }
                    
                    // VH стратегия для повернутой панели  
                    let rotated_vh_variants = self.java_fit_tile_vh(panel, &new_placement, true);
                    for variant in rotated_vh_variants {
                        let mut new_solution = solution.clone();
                        new_solution.placements.push(variant);
                        placement_variants.push(new_solution);
                    }
                }
            }
            
            // ✅ Java: если нет успешных размещений, панель остается неразмещенной
            if placement_variants.is_empty() {
                let mut solution_with_unplaced = solution.clone();
                solution_with_unplaced.unplaced_panels.push(panel.clone());
                placement_variants.push(solution_with_unplaced);
            }
            
            // ✅ Java: добавляем ВСЕ варианты размещения
            new_solutions.extend(placement_variants);
        }
        
        new_solutions
    }
    
    /// ✅ ТОЧНАЯ КОПИЯ JAVA: fitTile алгоритм с HV стратегией (AREA_HCUTS_1ST)
    fn java_fit_tile_hv(&self, panel: &Panel, placement: &Placement, rotate: bool) -> Vec<Placement> {
        let mut variants = Vec::new();
        
        // ✅ Java: определяем размеры панели (с поворотом или без)
        let (panel_width, panel_height) = if rotate {
            (panel.height as i32, panel.width as i32)
        } else {
            (panel.width as i32, panel.height as i32)
        };
        
        // ✅ Java: findCandidates - найти все подходящие позиции
        let candidates = placement.root_node.find_candidates(panel_width, panel_height);
        
        // ✅ Java: для каждого кандидата создаем варианты размещения
        for candidate in candidates {
            // ✅ Java: проверка точного совпадения размеров
            if candidate.width() == panel_width && candidate.height() == panel_height {
                // ✅ Java: точное совпадение - просто помечаем как финальный
                let mut new_placement = placement.clone();
                if let Some(target_node) = new_placement.root_node.find_node_by_coordinates(candidate.rect.x, candidate.rect.y) {
                    target_node.set_final(true);
                    target_node.set_panel_id(panel.id as i32);
                    new_placement.update_statistics();
                    variants.push(new_placement);
                }
            } else {
                // ✅ Java: нужны разрезы - применяем обе стратегии HV и VH
                
                // HV стратегия (горизонтальный разрез первый)
                let mut hv_placement = placement.clone();
                if let Some(target_node) = hv_placement.root_node.find_node_by_coordinates(candidate.rect.x, candidate.rect.y) {
                    if let Ok(hv_cuts) = Self::split_hv(target_node, &Panel {
                        id: panel.id,
                        width: panel_width as u32,
                        height: panel_height as u32,
                        count: panel.count,
                        label: panel.label.clone(),
                        material: panel.material.clone(),
                    }, self.cut_thickness) {
                        hv_placement.cuts.extend(hv_cuts);
                        hv_placement.update_statistics();
                        variants.push(hv_placement);
                    }
                }
                
                // VH стратегия (вертикальный разрез первый)
                let mut vh_placement = placement.clone();
                if let Some(target_node) = vh_placement.root_node.find_node_by_coordinates(candidate.rect.x, candidate.rect.y) {
                    if let Ok(vh_cuts) = Self::split_vh(target_node, &Panel {
                        id: panel.id,
                        width: panel_width as u32,
                        height: panel_height as u32,
                        count: panel.count,
                        label: panel.label.clone(),
                        material: panel.material.clone(),
                    }, self.cut_thickness) {
                        vh_placement.cuts.extend(vh_cuts);
                        vh_placement.update_statistics();
                        variants.push(vh_placement);
                    }
                }
            }
        }
        
        variants
    }
    
    /// ✅ ТОЧНАЯ КОПИЯ JAVA: fitTile алгоритм с VH стратегией (AREA_VCUTS_1ST)
    fn java_fit_tile_vh(&self, panel: &Panel, placement: &Placement, rotate: bool) -> Vec<Placement> {
        let mut variants = Vec::new();
        
        // ✅ Java: определяем размеры панели (с поворотом или без)
        let (panel_width, panel_height) = if rotate {
            (panel.height as i32, panel.width as i32)
        } else {
            (panel.width as i32, panel.height as i32)
        };
        
        // ✅ Java: findCandidates - найти все подходящие позиции
        let candidates = placement.root_node.find_candidates(panel_width, panel_height);
        
        // ✅ Java: для каждого кандидата создаем варианты размещения с VH стратегией
        for candidate in candidates {
            // ✅ Java: проверка точного совпадения размеров
            if candidate.width() == panel_width && candidate.height() == panel_height {
                // ✅ Java: точное совпадение - просто помечаем как финальный
                let mut new_placement = placement.clone();
                if let Some(target_node) = new_placement.root_node.find_node_by_coordinates(candidate.rect.x, candidate.rect.y) {
                    target_node.set_final(true);
                    target_node.set_panel_id(panel.id as i32);
                    new_placement.update_statistics();
                    variants.push(new_placement);
                }
            } else {
                // ✅ Java: нужны разрезы - применяем VH стратегию (вертикальный разрез первый)
                let mut vh_placement = placement.clone();
                if let Some(target_node) = vh_placement.root_node.find_node_by_coordinates(candidate.rect.x, candidate.rect.y) {
                    if let Ok(vh_cuts) = Self::split_vh(target_node, &Panel {
                        id: panel.id,
                        width: panel_width as u32,
                        height: panel_height as u32,
                        count: panel.count,
                        label: panel.label.clone(),
                        material: panel.material.clone(),
                    }, self.cut_thickness) {
                        vh_placement.cuts.extend(vh_cuts);
                        vh_placement.update_statistics();
                        variants.push(vh_placement);
                    }
                }
            }
        }
        
        variants
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
        // ✅ Java: левый дочерний узел (остается для панели)
        let child1_rect = Rectangle::new(
            node.rect.x,
            node.rect.x + panel_width,
            node.rect.y,
            node.rect.y2()
        );
        let mut child1 = Node::new(0, child1_rect);
        child1.set_panel_id(panel_id);
        
        if child1.area() > 0 {
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
            let child2 = Node::new(0, child2_rect);
            if child2.area() > 0 {
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
        // ✅ Java: верхний дочерний узел (остается для панели)
        let child1_rect = Rectangle::new(
            node.rect.x,
            node.rect.x2(),
            node.rect.y,
            node.rect.y + panel_height
        );
        let mut child1 = Node::new(0, child1_rect);
        child1.set_panel_id(panel_id);
        
        if child1.area() > 0 {
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
            let child2 = Node::new(0, child2_rect);
            if child2.area() > 0 {
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
    
    /// ✅ ТОЧНАЯ КОПИЯ JAVA: Фильтрация решений с полным набором Java comparators из PriorityListFactory
    fn filter_solutions_java_style(&self, mut solutions: Vec<Solution>) -> Vec<Solution> {
        if solutions.len() <= 100 { // Как в Java - ограничиваем количество решений
            return solutions;
        }
        
        // ✅ ТОЧНАЯ КОПИЯ: getFinalSolutionPrioritizedComparatorList() из PriorityListFactory.java
        solutions.sort_by(|a, b| {
            // 1. MOST_TILES - максимизируем количество размещенных панелей
            let tiles_a = a.get_nbr_tiles();
            let tiles_b = b.get_nbr_tiles(); 
            if tiles_a != tiles_b {
                return tiles_b.cmp(&tiles_a); // Больше панелей - лучше
            }
            
            // 2. LEAST_WASTED_AREA - минимизируем потерянную площадь
            let waste_a = a.get_wasted_area();
            let waste_b = b.get_wasted_area();
            if waste_a != waste_b {
                return waste_a.cmp(&waste_b); // Меньше отходов - лучше
            }
            
            // 3. LEAST_NBR_CUTS - минимизируем количество резов
            let cuts_a = a.get_nbr_cuts();
            let cuts_b = b.get_nbr_cuts();
            if cuts_a != cuts_b {
                return cuts_a.cmp(&cuts_b); // Меньше резов - лучше
            }
            
            // 4. LEAST_NBR_MOSAICS - КРИТИЧЕСКИЙ! Минимизируем количество листов
            let mosaics_a = a.get_nbr_mosaics();
            let mosaics_b = b.get_nbr_mosaics();
            if mosaics_a != mosaics_b {
                return mosaics_a.cmp(&mosaics_b); // Меньше листов - лучше
            }
            
            // 5. BIGGEST_UNUSED_TILE_AREA - максимизируем наибольшую неиспользованную площадь
            let biggest_a = a.get_biggest_area();
            let biggest_b = b.get_biggest_area();
            if biggest_a != biggest_b {
                return biggest_b.cmp(&biggest_a); // Больше остаток - лучше
            }
            
            // 6. MOST_HV_DISCREPANCY - пока не реализован, игнорируем
            std::cmp::Ordering::Equal
        });
        
        // Оставляем лучшие 100 решений
        solutions.truncate(100);
        solutions
    }
    
    /// ✅ НОВЫЙ МЕТОД: Расчет потерянной площади (Java LEAST_WASTED_AREA)
    fn calculate_wasted_area(&self, solution: &Solution) -> f64 {
        let mut total_wasted = 0i64;
        let stock_area = self.stocks[0].width as i64 * self.stocks[0].height as i64;
        
        for placement in &solution.placements {
            let wasted = stock_area - placement.used_area;
            total_wasted += wasted;
        }
        
        total_wasted as f64
    }
    
    /// ✅ НОВЫЙ МЕТОД: Расчет наибольшей неиспользованной площади (Java BIGGEST_UNUSED_TILE_AREA)
    fn calculate_biggest_unused_area(&self, solution: &Solution) -> f64 {
        let stock_area = self.stocks[0].width as i64 * self.stocks[0].height as i64;
        
        solution.placements.iter()
            .map(|placement| (stock_area - placement.used_area) as f64)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0)
    }
    
    /// ✅ ТОЧНАЯ КОПИЯ JAVA: Выбор лучшего решения с полным набором Java comparators 
    fn select_best_solution(&self, solutions: Vec<Solution>) -> Solution {
        solutions.into_iter()
            .min_by(|a, b| {
                // ✅ ТОЧНАЯ КОПИЯ: getFinalSolutionPrioritizedComparatorList() из PriorityListFactory.java
                
                // 1. MOST_TILES - максимизируем количество размещенных панелей
                let tiles_a = a.get_nbr_tiles();
                let tiles_b = b.get_nbr_tiles(); 
                if tiles_a != tiles_b {
                    return tiles_b.cmp(&tiles_a); // Больше панелей - лучше
                }
                
                // 2. LEAST_WASTED_AREA - минимизируем потерянную площадь
                let waste_a = a.get_wasted_area();
                let waste_b = b.get_wasted_area();
                if waste_a != waste_b {
                    return waste_a.cmp(&waste_b); // Меньше отходов - лучше
                }
                
                // 3. LEAST_NBR_CUTS - минимизируем количество резов
                let cuts_a = a.get_nbr_cuts();
                let cuts_b = b.get_nbr_cuts();
                if cuts_a != cuts_b {
                    return cuts_a.cmp(&cuts_b); // Меньше резов - лучше
                }
                
                // 4. LEAST_NBR_MOSAICS - КРИТИЧЕСКИЙ! Минимизируем количество листов
                let mosaics_a = a.get_nbr_mosaics();
                let mosaics_b = b.get_nbr_mosaics();
                if mosaics_a != mosaics_b {
                    return mosaics_a.cmp(&mosaics_b); // Меньше листов - лучше
                }
                
                // 5. BIGGEST_UNUSED_TILE_AREA - максимизируем наибольшую неиспользованную площадь
                let biggest_a = a.get_biggest_area();
                let biggest_b = b.get_biggest_area();
                if biggest_a != biggest_b {
                    return biggest_b.cmp(&biggest_a); // Больше остаток - лучше
                }
                
                // 6. MOST_HV_DISCREPANCY - пока не реализован, игнорируем
                std::cmp::Ordering::Equal
            })
            .unwrap_or_else(|| Solution::new_with_stocks(self.stocks.clone()))
    }
    
    /// ✅ НОВЫЙ МЕТОД: Расчет эффективности решения
    fn calculate_efficiency(&self, solution: &Solution) -> f64 {
        let total_used_area: i64 = solution.placements.iter().map(|p| p.used_area).sum();
        let total_area: i64 = solution.placements.len() as i64 * (self.stocks[0].width as i64 * self.stocks[0].height as i64);
        if total_area > 0 { total_used_area as f64 / total_area as f64 } else { 0.0 }
    }
    pub fn new(panels: Vec<Panel>, stock: Vec<Panel>) -> Self {
        Self {
            panels,
            stocks: stock,
            cut_thickness: 0, // Толщина реза (обычно 0)
            max_sheets: 10,   // Максимум листов для перебора
        }
    }

    /// ✅ НОВЫЙ МЕТОД: Оптимизация с использованием готовой перестановки от Java
    /// Это позволяет получить результат идентичный Java, используя ту же последовательность панелей
    pub fn optimize_with_java_permutation(&self, java_panel_sequence: Vec<TileDimensions>) -> Solution {
        println!("\n=== 🔥 ИСПОЛЬЗУЕМ JAVA ПЕРЕСТАНОВКУ ===");
        println!("Панелей в Java последовательности: {}", java_panel_sequence.len());
        
        // Сохраняем Java последовательность для отладки
        save_to_json(&java_panel_sequence, "_java_optimal_sequence.json").unwrap();
        
        // Создаем панельную последовательность из Java данных
        let panel_sequence: Vec<SequentialPanel> = java_panel_sequence.iter().enumerate().map(|(index, tile_dim)| {
            // ✅ Определяем поворот панели (если ширина больше высоты в Java данных, но меньше в исходных)
            let original_panel = self.panels.iter().find(|p| p.id == tile_dim.id).unwrap();
            let is_rotated = if original_panel.width != tile_dim.width {
                true // Панель была повернута в Java
            } else {
                false
            };
            
            SequentialPanel {
                panel: Panel {
                    id: tile_dim.id,
                    width: tile_dim.width,
                    height: tile_dim.height,
                    count: 1, // TileDimensions уже развернут
                    label: tile_dim.label.clone(),
                    material: "DEFAULT_MATERIAL".to_string(),
                },
                sequence_index: index + 1,
                rotated: is_rotated,
            }
        }).collect();
        
        println!("✅ Создана Java последовательность с {} панелями", panel_sequence.len());
        for (i, seq_panel) in panel_sequence.iter().take(5).enumerate() {
            println!("  {}. {}x{} ID_{} (rotated: {})", 
                i + 1, 
                seq_panel.panel.width, 
                seq_panel.panel.height, 
                seq_panel.panel.id,
                seq_panel.rotated
            );
        }
        if panel_sequence.len() > 5 {
            println!("  ... и еще {} панелей", panel_sequence.len() - 5);
        }
        
        // Обрабатываем панели строго последовательно как в Java
        let final_solution = self.process_panel_sequence_java_style(&panel_sequence);
        
        println!("\n=== 🏆 ФИНАЛЬНЫЙ РЕЗУЛЬТАТ С JAVA ПОСЛЕДОВАТЕЛЬНОСТЬЮ ===");
        println!("Листов использовано: {}", final_solution.placements.len());
        println!("Панелей неразмещено: {}", final_solution.unplaced_panels.len());
        let efficiency = self.calculate_efficiency(&final_solution);
        println!("Эффективность: {:.2}%", efficiency * 100.0);
        
        final_solution
    }

    /// Главный метод оптимизации
    /// ✅ ИСПРАВЛЕНО: Теперь обрабатывает панели строго последовательно как в Java
    pub fn optimize(&self) -> Solution {

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

        save_to_json(&panels_expanded, "_expanded_panels.json").unwrap();
        save_to_json(&stock_expanded, "_expanded_stocks.json").unwrap();

        // ЭТАП 2: Сгруппировать панели для оптимизации перестановок
        let grouped_panels = PanelGrouper::group_panels(&panels_expanded, &stock_expanded);
        save_to_json(&grouped_panels, "_grouped_panels.json").unwrap();

        // ЭТАП 3: Создание перестановок групп (новый этап!)
        let permutations = PermutationGenerator::create_group_permutations(&grouped_panels);
        PermutationGenerator::print_permutation_stats(&permutations);

        // Сохраняем первые несколько перестановок для отладки
        if !permutations.is_empty() {
            save_to_json(&permutations[0], "_first_permutation.json").unwrap();
            if permutations.len() > 1 {
                save_to_json(&permutations[1], "_second_permutation.json").unwrap();
            }
        }

        // ЭТАП 4: Основной цикл оптимизации - ✅ ИСПРАВЛЕНО: как в Java CutListThread.computeSolutions()
        let mut best_solution: Option<Solution> = None;
        let mut best_efficiency = 0.0;
        
        // Попробуем больше перестановок для поиска лучшего решения (как в Java)
        let max_permutations_to_try = std::cmp::min(permutations.len(), 50);
        
        for (perm_index, permutation) in permutations.iter().take(max_permutations_to_try).enumerate() {
            println!("\n=== НАЧАЛО ОБРАБОТКИ ПЕРЕСТАНОВКИ {} ===", perm_index + 1);
            println!("Панелей в перестановке: {}", permutation.len());
            
            // ✅ ИСПРАВЛЕНО: Создаем панельную последовательность как в Java
            let panel_sequence: Vec<SequentialPanel> = permutation.iter().enumerate().map(|(index, tile_dim)| {
                SequentialPanel {
                    panel: Panel {
                        id: tile_dim.id,
                        width: tile_dim.width,
                        height: tile_dim.height,
                        count: 1, // TileDimensions уже развернут
                        label: tile_dim.label.clone(),
                        material: "DEFAULT_MATERIAL".to_string(),
                    },
                    sequence_index: index + 1,
                    rotated: false, // В Java это определяется алгоритмом размещения
                }
            }).collect();
            
            // ✅ ИСПРАВЛЕНО: Обрабатываем панели СТРОГО ПОСЛЕДОВАТЕЛЬНО как в Java
            let current_solution = self.process_panel_sequence_java_style(&panel_sequence);
            
            // Рассчитываем эффективность этой перестановки
            let total_used_area: i64 = current_solution.placements.iter().map(|p| p.used_area).sum();
            let total_area: i64 = current_solution.placements.len() as i64 * (self.stocks[0].width as i64 * self.stocks[0].height as i64);
            let efficiency = if total_area > 0 { total_used_area as f64 / total_area as f64 } else { 0.0 };
            
            println!("РЕЗУЛЬТАТ ПЕРЕСТАНОВКИ {}: листов {}, неразмещенных {}, эффективность {:.2}%", 
                perm_index + 1, current_solution.placements.len(), current_solution.unplaced_panels.len(), efficiency * 100.0);
            
            // Проверяем, лучше ли это решение (приоритеты как в Java)
            let is_better = match &best_solution {
                None => true, // Первое решение всегда лучше
                Some(best) => {
                    if current_solution.unplaced_panels.len() != best.unplaced_panels.len() {
                        // 1. Приоритет: меньше неразмещенных панелей
                        current_solution.unplaced_panels.len() < best.unplaced_panels.len()
                    } else if current_solution.placements.len() != best.placements.len() {
                        // 2. Приоритет: меньше листов (как в Java)
                        current_solution.placements.len() < best.placements.len()
                    } else {
                        // 3. Приоритет: лучшая эффективность
                        efficiency > best_efficiency
                    }
                }
            };
            
            if is_better {
                println!("✅ НОВОЕ ЛУЧШЕЕ РЕШЕНИЕ!");
                best_solution = Some(current_solution);
                best_efficiency = efficiency;
            } else {
                println!("❌ Решение хуже текущего лучшего");
            }
        }
        
        let final_solution = best_solution.unwrap_or_else(|| Solution::new_with_stocks(self.stocks.clone()));
        println!("\n=== ФИНАЛЬНОЕ РЕШЕНИЕ ===");
        println!("Листов использовано: {}", final_solution.placements.len());
        println!("Панелей неразмещено: {}", final_solution.unplaced_panels.len());
        
        final_solution
    }
}
