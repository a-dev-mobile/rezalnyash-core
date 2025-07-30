use crate::features::{cut::Cut, input::models::panel::Panel, placed_panel::PlacedPanel, rectangle::Rectangle};



/// Узел дерева разрезов
/// Основано на TileNode.java
#[derive(Debug, Clone)]
pub struct Node {
    pub id: i32,
    pub rectangle: Rectangle,
    pub child1: Option<Box<Node>>,  // Первый дочерний узел
    pub child2: Option<Box<Node>>,  // Второй дочерний узел  
    pub external_id: Option<u16>,   // ID размещенной детали
    pub panel_label: Option<String>, // Лейбл размещенной панели
    pub is_final: bool,             // Финальный узел (размещена панель)
    pub is_rotated: bool,
}

impl Node {
    pub fn new(id: i32, rect: Rectangle) -> Self {
        Self {
            id,
            rectangle: rect,
            child1: None,
            child2: None,
            external_id: None,
            panel_label: None,
            is_final: false,
            is_rotated: false,
        }
    }

    /// Поиск подходящих мест для размещения панели
    /// Основная логика из findCandidates() в CutListThread.java
    pub fn find_candidates(&self, panel_width: i32, panel_height: i32) -> Vec<&Node> {
        let mut candidates = Vec::new();
        self.find_candidates_recursive(panel_width, panel_height, &mut candidates, 0);
        candidates
    }

    fn find_candidates_recursive<'a>(&'a self, panel_width: i32, panel_height: i32, candidates: &mut Vec<&'a Node>, min_trim_dimension: i32) {
        // Проверки из Java кода
        if self.is_final || self.rectangle.width < panel_width || self.rectangle.height < panel_height {
            return;
        }

        // Если нода является листом (нет детей)
        if self.child1.is_none() && self.child2.is_none() {
            let width_fits = self.rectangle.width == panel_width || self.rectangle.width >= min_trim_dimension + panel_width;
            let height_fits = self.rectangle.height == panel_height || self.rectangle.height >= min_trim_dimension + panel_height;
            
            if width_fits && height_fits {
                candidates.push(self);
            }
            return;
        }

        // Рекурсивно проверяем детей
        if let Some(ref child1) = self.child1 {
            child1.find_candidates_recursive(panel_width, panel_height, candidates, min_trim_dimension);
        }
        if let Some(ref child2) = self.child2 {
            child2.find_candidates_recursive(panel_width, panel_height, candidates, min_trim_dimension);
        }
    }

    /// Размещение панели в ноде
    /// Основная логика из splitHV/splitVH в CutListThread.java
    pub fn place_panel(&mut self, panel: &Panel, cut_thickness: i32) -> Result<Vec<Cut>, String> {
        let panel_width = panel.width as i32;
        let panel_height = panel.height as i32;
        
        if self.rectangle.width < panel_width || self.rectangle.height < panel_height {
            return Err("Panel doesn't fit".to_string());
        }

        // Точное совпадение размеров
        if self.rectangle.width == panel_width && self.rectangle.height == panel_height {
            self.is_final = true;
            self.external_id = Some(panel.id);
            self.panel_label = Some(panel.label.clone());
            return Ok(Vec::new());
        }

        // Используем HV порядок (горизонтальный-вертикальный) как было изначально
        self.apply_hv_placement(panel, cut_thickness)
    }

    /// Принудительное размещение панели в HV порядке
    pub fn place_panel_hv(&mut self, panel: &Panel, cut_thickness: i32) -> Result<Vec<Cut>, String> {
        let panel_width = panel.width as i32;
        let panel_height = panel.height as i32;
        
        if self.rectangle.width < panel_width || self.rectangle.height < panel_height {
            return Err("Panel doesn't fit".to_string());
        }

        // Точное совпадение размеров
        if self.rectangle.width == panel_width && self.rectangle.height == panel_height {
            self.is_final = true;
            self.external_id = Some(panel.id);
            self.panel_label = Some(panel.label.clone());
            return Ok(Vec::new());
        }

        self.apply_hv_placement(panel, cut_thickness)
    }

    /// Принудительное размещение панели в VH порядке
    pub fn place_panel_vh(&mut self, panel: &Panel, cut_thickness: i32) -> Result<Vec<Cut>, String> {
        let panel_width = panel.width as i32;
        let panel_height = panel.height as i32;
        
        if self.rectangle.width < panel_width || self.rectangle.height < panel_height {
            return Err("Panel doesn't fit".to_string());
        }

        // Точное совпадение размеров
        if self.rectangle.width == panel_width && self.rectangle.height == panel_height {
            self.is_final = true;
            self.external_id = Some(panel.id);
            self.panel_label = Some(panel.label.clone());
            return Ok(Vec::new());
        }

        self.apply_vh_placement(panel, cut_thickness)
    }

    /// Пробует размещение в порядке Горизонтальный-Вертикальный (HV)
    fn try_place_hv(&self, panel: &Panel, cut_thickness: i32) -> Result<Vec<Cut>, String> {
        let panel_width = panel.width as i32;
        let panel_height = panel.height as i32;
        
        let mut cuts = Vec::new();
        
        if self.rectangle.width > panel_width {
            cuts.push(Cut::new(
                self.rectangle.x + panel_width,
                self.rectangle.y,
                self.rectangle.x + panel_width,
                self.rectangle.y + self.rectangle.height,
                false,
            ));
            
            if self.rectangle.height > panel_height {
                cuts.push(Cut::new(
                    self.rectangle.x,
                    self.rectangle.y + panel_height,
                    self.rectangle.x + panel_width,
                    self.rectangle.y + panel_height,
                    true,
                ));
            }
        } else if self.rectangle.height > panel_height {
            cuts.push(Cut::new(
                self.rectangle.x,
                self.rectangle.y + panel_height,
                self.rectangle.x + self.rectangle.width,
                self.rectangle.y + panel_height,
                true,
            ));
        }
        
        Ok(cuts)
    }

    /// Пробует размещение в порядке Вертикальный-Горизонтальный (VH)
    fn try_place_vh(&self, panel: &Panel, cut_thickness: i32) -> Result<Vec<Cut>, String> {
        let panel_width = panel.width as i32;
        let panel_height = panel.height as i32;
        
        let mut cuts = Vec::new();
        
        if self.rectangle.height > panel_height {
            cuts.push(Cut::new(
                self.rectangle.x,
                self.rectangle.y + panel_height,
                self.rectangle.x + self.rectangle.width,
                self.rectangle.y + panel_height,
                true,
            ));
            
            if self.rectangle.width > panel_width {
                cuts.push(Cut::new(
                    self.rectangle.x + panel_width,
                    self.rectangle.y,
                    self.rectangle.x + panel_width,
                    self.rectangle.y + panel_height,
                    false,
                ));
            }
        } else if self.rectangle.width > panel_width {
            cuts.push(Cut::new(
                self.rectangle.x + panel_width,
                self.rectangle.y,
                self.rectangle.x + panel_width,
                self.rectangle.y + self.rectangle.height,
                false,
            ));
        }
        
        Ok(cuts)
    }

    /// Применяет размещение в порядке HV
    fn apply_hv_placement(&mut self, panel: &Panel, cut_thickness: i32) -> Result<Vec<Cut>, String> {
        let panel_width = panel.width as i32;
        let panel_height = panel.height as i32;
        
        let mut cuts = Vec::new();

        if self.rectangle.width > panel_width {
            let cut = self.split_horizontal(panel_width, cut_thickness);
            cuts.push(cut);
            
            if self.rectangle.height > panel_height {
                if let Some(ref mut child1) = self.child1 {
                    let cut2 = child1.split_vertical(panel_height, cut_thickness);
                    cuts.push(cut2);
                    
                    if let Some(ref mut grandchild) = child1.child1 {
                        grandchild.is_final = true;
                        grandchild.external_id = Some(panel.id);
                        grandchild.panel_label = Some(panel.label.clone());
                    }
                }
            } else {
                if let Some(ref mut child1) = self.child1 {
                    child1.is_final = true;
                    child1.external_id = Some(panel.id);
                    child1.panel_label = Some(panel.label.clone());
                }
            }
        } else {
            let cut = self.split_vertical(panel_height, cut_thickness);
            cuts.push(cut);
            
            if let Some(ref mut child1) = self.child1 {
                child1.is_final = true;
                child1.external_id = Some(panel.id);
                child1.panel_label = Some(panel.label.clone());
            }
        }

        Ok(cuts)
    }

    /// Применяет размещение в порядке VH
    fn apply_vh_placement(&mut self, panel: &Panel, cut_thickness: i32) -> Result<Vec<Cut>, String> {
        let panel_width = panel.width as i32;
        let panel_height = panel.height as i32;
        
        let mut cuts = Vec::new();

        if self.rectangle.height > panel_height {
            let cut = self.split_vertical(panel_height, cut_thickness);
            cuts.push(cut);
            
            if self.rectangle.width > panel_width {
                if let Some(ref mut child1) = self.child1 {
                    let cut2 = child1.split_horizontal(panel_width, cut_thickness);
                    cuts.push(cut2);
                    
                    if let Some(ref mut grandchild) = child1.child1 {
                        grandchild.is_final = true;
                        grandchild.external_id = Some(panel.id);
                        grandchild.panel_label = Some(panel.label.clone());
                    }
                }
            } else {
                if let Some(ref mut child1) = self.child1 {
                    child1.is_final = true;
                    child1.external_id = Some(panel.id);
                    child1.panel_label = Some(panel.label.clone());
                }
            }
        } else {
            let cut = self.split_horizontal(panel_width, cut_thickness);
            cuts.push(cut);
            
            if let Some(ref mut child1) = self.child1 {
                child1.is_final = true;
                child1.external_id = Some(panel.id);
                child1.panel_label = Some(panel.label.clone());
            }
        }

        Ok(cuts)
    }

    /// Горизонтальный разрез
    /// Логика из splitHorizontally() в CutListThread.java
    pub fn split_horizontal(&mut self, cut_position: i32, cut_thickness: i32) -> Cut {
        let rect = &self.rectangle;
        
        // Левая часть (child1)
        let child1_rect = Rectangle::new(
            rect.x,
            rect.y,
            cut_position,
            rect.height
        );
        
        // Правая часть (child2)
        let child2_rect = Rectangle::new(
            rect.x + cut_position + cut_thickness,
            rect.y,
            rect.width - cut_position - cut_thickness,
            rect.height
        );

        if child1_rect.area() > 0 {
            self.child1 = Some(Box::new(Node::new(self.get_next_id(), child1_rect)));
        }
        
        if child2_rect.area() > 0 {
            self.child2 = Some(Box::new(Node::new(self.get_next_id(), child2_rect)));
        }

        Cut::new_vertical(
            rect.x + cut_position,
            rect.y,
            rect.height
        )
    }

    /// Вертикальный разрез
    /// Логика из splitVertically() в CutListThread.java
    pub fn split_vertical(&mut self, cut_position: i32, cut_thickness: i32) -> Cut {
        let rect = &self.rectangle;
        
        // Верхняя часть (child1)
        let child1_rect = Rectangle::new(
            rect.x,
            rect.y,
            rect.width,
            cut_position
        );
        
        // Нижняя часть (child2)
        let child2_rect = Rectangle::new(
            rect.x,
            rect.y + cut_position + cut_thickness,
            rect.width,
            rect.height - cut_position - cut_thickness
        );

        if child1_rect.area() > 0 {
            self.child1 = Some(Box::new(Node::new(self.get_next_id(), child1_rect)));
        }
        
        if child2_rect.area() > 0 {
            self.child2 = Some(Box::new(Node::new(self.get_next_id(), child2_rect)));
        }

        Cut::new_horizontal(
            rect.x,
            rect.y + cut_position,
            rect.width
        )
    }

    fn get_next_id(&self) -> i32 {
        // Простая генерация ID - можно улучшить
        (self.id + 1) * 10 + 1
    }

    /// Получить использованную площадь
    pub fn get_used_area(&self) -> i64 {
        if self.is_final {
            return self.rectangle.area();
        }

        let mut total_area = 0;
        if let Some(ref child1) = self.child1 {
            total_area += child1.get_used_area();
        }
        if let Some(ref child2) = self.child2 {
            total_area += child2.get_used_area();
        }
        total_area
    }

    /// Получить все финальные панели
    pub fn get_final_panels(&self) -> Vec<PlacedPanel> {
        let mut panels = Vec::new();
        self.collect_final_panels(&mut panels);
        panels
    }

    fn collect_final_panels(&self, panels: &mut Vec<PlacedPanel>) {
        if self.is_final {
            if let Some(panel_id) = self.external_id {
                panels.push(PlacedPanel {
                    panel_id: panel_id as i32,
                    x: self.rectangle.x,
                    y: self.rectangle.y,
                    width: self.rectangle.width,
                    height: self.rectangle.height,
                    label: self.panel_label.as_ref().unwrap_or(&format!("Panel_{}", panel_id)).clone(),
                    is_rotated: self.is_rotated,
                });
            }
            return;
        }

        if let Some(ref child1) = self.child1 {
            child1.collect_final_panels(panels);
        }
        if let Some(ref child2) = self.child2 {
            child2.collect_final_panels(panels);
        }
    }
}
