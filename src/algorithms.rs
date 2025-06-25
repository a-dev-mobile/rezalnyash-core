//! Алгоритмы оптимизации раскроя материалов

use crate::{types::*, CuttingAlgorithm, Result};

/// Алгоритм наилучшего размещения (Best Fit)
/// Размещает детали в позиции с наименьшими потерями материала
#[derive(Debug, Clone)]
pub struct BestFitAlgorithm {
    cutting_gap: f64,
}

impl BestFitAlgorithm {
    /// Создает новый экземпляр алгоритма
    pub fn new() -> Self {
        Self { cutting_gap: 2.0 }
    }

    /// Создает алгоритм с заданным зазором между деталями
    pub fn with_gap(cutting_gap: f64) -> Self {
        Self { cutting_gap }
    }

    /// Находит наилучшую позицию для размещения детали
    fn find_best_position(
        &self,
        layout: &CuttingLayout,
        width: f64,
        height: f64,
    ) -> Option<(f64, f64)> {
        let mut best_position = None;
        let mut best_waste = f64::INFINITY;

        // Пробуем разместить деталь в различных позициях
        for y in self.generate_y_positions(layout) {
            for x in self.generate_x_positions(layout) {
                let rect = Rectangle::new(x, y, width, height);
                
                if layout.can_place_part(&rect) {
                    let waste = self.calculate_local_waste(layout, &rect);
                    if waste < best_waste {
                        best_waste = waste;
                        best_position = Some((x, y));
                    }
                }
            }
        }

        best_position
    }

    /// Генерирует возможные Y позиции для размещения
    fn generate_y_positions(&self, layout: &CuttingLayout) -> Vec<f64> {
        let mut positions = vec![0.0];
        
        for part in &layout.parts {
            positions.push(part.rectangle.y + part.rectangle.height + self.cutting_gap);
        }

        positions.sort_by(|a, b| a.partial_cmp(b).unwrap());
        positions.dedup();
        positions
    }

    /// Генерирует возможные X позиции для размещения
    fn generate_x_positions(&self, layout: &CuttingLayout) -> Vec<f64> {
        let mut positions = vec![0.0];
        
        for part in &layout.parts {
            positions.push(part.rectangle.x + part.rectangle.width + self.cutting_gap);
        }

        positions.sort_by(|a, b| a.partial_cmp(b).unwrap());
        positions.dedup();
        positions
    }

    /// Вычисляет локальные потери материала при размещении детали
    fn calculate_local_waste(&self, _layout: &CuttingLayout, rect: &Rectangle) -> f64 {
        // Простая эвристика: чем ближе к углу, тем меньше потери
        rect.x + rect.y
    }
}

impl CuttingAlgorithm for BestFitAlgorithm {
    fn optimize(&self, material: &Material, requests: &[CuttingRequest]) -> Result<CuttingResult> {
        let mut result = CuttingResult::new();
        let mut layout = CuttingLayout::new(material.clone());
        
        // Развернутый список всех деталей для размещения
        let mut parts_to_place = Vec::new();
        for request in requests {
            for _ in 0..request.quantity {
                parts_to_place.push(request.clone());
            }
        }

        // Сортируем по убыванию площади
        parts_to_place.sort_by(|a, b| b.area().partial_cmp(&a.area()).unwrap());

        for part_request in parts_to_place {
            let mut placed = false;

            // Пробуем разместить без поворота
            if let Some((x, y)) = self.find_best_position(&layout, part_request.width, part_request.height) {
                let placed_part = PlacedPart::new(x, y, part_request.width, part_request.height, false);
                layout.add_part(placed_part);
                placed = true;
            }
            // Пробуем с поворотом, если разрешен
            else if part_request.can_rotate {
                if let Some((x, y)) = self.find_best_position(&layout, part_request.height, part_request.width) {
                    let placed_part = PlacedPart::new(x, y, part_request.height, part_request.width, true);
                    layout.add_part(placed_part);
                    placed = true;
                }
            }

            if !placed {
                result.unplaced_parts += 1;
            }
        }

        result.add_layout(layout);
        Ok(result)
    }

    fn name(&self) -> &'static str {
        "Best Fit Algorithm"
    }
}

/// Алгоритм заполнения снизу-слева (Bottom-Left Fill)
/// Размещает детали максимально близко к левому нижнему углу
#[derive(Debug, Clone)]
pub struct BottomLeftFillAlgorithm {
    cutting_gap: f64,
}

impl BottomLeftFillAlgorithm {
    /// Создает новый экземпляр алгоритма
    pub fn new() -> Self {
        Self { cutting_gap: 2.0 }
    }

    /// Создает алгоритм с заданным зазором
    pub fn with_gap(cutting_gap: f64) -> Self {
        Self { cutting_gap }
    }

    /// Находит позицию для размещения детали по алгоритму Bottom-Left
    fn find_bottom_left_position(
        &self,
        layout: &CuttingLayout,
        width: f64,
        height: f64,
    ) -> Option<(f64, f64)> {
        // Начинаем с левого нижнего угла и движемся вправо и вверх
        let step = 10.0; // Шаг для поиска позиций
        
        for y in (0..=(layout.material.height as i32)).step_by(step as usize) {
            for x in (0..=(layout.material.width as i32)).step_by(step as usize) {
                let rect = Rectangle::new(x as f64, y as f64, width, height);
                
                if layout.can_place_part(&rect) {
                    return Some((x as f64, y as f64));
                }
            }
        }

        None
    }

    /// Пытается разместить деталь как можно ниже и левее
    fn place_bottom_left(
        &self,
        layout: &CuttingLayout,
        width: f64,
        height: f64,
    ) -> Option<(f64, f64)> {
        let mut best_position = None;
        let mut lowest_y = f64::INFINITY;
        let mut leftmost_x = f64::INFINITY;

        // Генерируем сетку возможных позиций
        let x_positions = self.generate_x_grid(layout);
        let y_positions = self.generate_y_grid(layout);

        for &y in &y_positions {
            for &x in &x_positions {
                let rect = Rectangle::new(x, y, width, height);
                
                if layout.can_place_part(&rect) {
                    // Предпочитаем более низкие позиции, затем более левые
                    if y < lowest_y || (y == lowest_y && x < leftmost_x) {
                        lowest_y = y;
                        leftmost_x = x;
                        best_position = Some((x, y));
                    }
                }
            }
        }

        best_position
    }

    /// Генерирует сетку X координат
    fn generate_x_grid(&self, layout: &CuttingLayout) -> Vec<f64> {
        let mut positions = vec![0.0];
        
        for part in &layout.parts {
            positions.push(part.rectangle.x + part.rectangle.width + self.cutting_gap);
        }

        positions.sort_by(|a, b| a.partial_cmp(b).unwrap());
        positions.dedup();
        positions
    }

    /// Генерирует сетку Y координат
    fn generate_y_grid(&self, layout: &CuttingLayout) -> Vec<f64> {
        let mut positions = vec![0.0];
        
        for part in &layout.parts {
            positions.push(part.rectangle.y + part.rectangle.height + self.cutting_gap);
        }

        positions.sort_by(|a, b| a.partial_cmp(b).unwrap());
        positions.dedup();
        positions
    }
}

impl CuttingAlgorithm for BottomLeftFillAlgorithm {
    fn optimize(&self, material: &Material, requests: &[CuttingRequest]) -> Result<CuttingResult> {
        let mut result = CuttingResult::new();
        let mut layout = CuttingLayout::new(material.clone());
        
        // Развернутый список всех деталей
        let mut parts_to_place = Vec::new();
        for request in requests {
            for _ in 0..request.quantity {
                parts_to_place.push(request.clone());
            }
        }

        // Сортируем по убыванию высоты, затем по ширине
        parts_to_place.sort_by(|a, b| {
            let height_cmp = b.height.partial_cmp(&a.height).unwrap();
            if height_cmp != std::cmp::Ordering::Equal {
                height_cmp
            } else {
                b.width.partial_cmp(&a.width).unwrap()
            }
        });

        for part_request in parts_to_place {
            let mut placed = false;

            // Пробуем разместить без поворота
            if let Some((x, y)) = self.place_bottom_left(&layout, part_request.width, part_request.height) {
                let placed_part = PlacedPart::new(x, y, part_request.width, part_request.height, false);
                layout.add_part(placed_part);
                placed = true;
            }
            // Пробуем с поворотом
            else if part_request.can_rotate {
                if let Some((x, y)) = self.place_bottom_left(&layout, part_request.height, part_request.width) {
                    let placed_part = PlacedPart::new(x, y, part_request.height, part_request.width, true);
                    layout.add_part(placed_part);
                    placed = true;
                }
            }

            if !placed {
                result.unplaced_parts += 1;
            }
        }

        result.add_layout(layout);
        Ok(result)
    }

    fn name(&self) -> &'static str {
        "Bottom-Left Fill Algorithm"
    }
}

/// Алгоритм First Fit Decreasing (FFD)
/// Сортирует детали по убыванию и размещает в первое подходящее место
#[derive(Debug, Clone)]
pub struct FirstFitDecreasingAlgorithm {
    cutting_gap: f64,
}

impl FirstFitDecreasingAlgorithm {
    pub fn new() -> Self {
        Self { cutting_gap: 2.0 }
    }

    pub fn with_gap(cutting_gap: f64) -> Self {
        Self { cutting_gap }
    }
}

impl CuttingAlgorithm for FirstFitDecreasingAlgorithm {
    fn optimize(&self, material: &Material, requests: &[CuttingRequest]) -> Result<CuttingResult> {
        let mut result = CuttingResult::new();
        let mut layout = CuttingLayout::new(material.clone());
        
        let mut parts_to_place = Vec::new();
        for request in requests {
            for _ in 0..request.quantity {
                parts_to_place.push(request.clone());
            }
        }

        // Сортируем по убыванию площади
        parts_to_place.sort_by(|a, b| b.area().partial_cmp(&a.area()).unwrap());

        for part_request in parts_to_place {
            let mut placed = false;

            // Простой поиск первого подходящего места
            'outer: for y in (0..=(material.height as i32)).step_by(10) {
                for x in (0..=(material.width as i32)).step_by(10) {
                    let rect = Rectangle::new(x as f64, y as f64, part_request.width, part_request.height);
                    
                    if layout.can_place_part(&rect) {
                        let placed_part = PlacedPart::new(x as f64, y as f64, part_request.width, part_request.height, false);
                        layout.add_part(placed_part);
                        placed = true;
                        break 'outer;
                    }

                    // Пробуем с поворотом
                    if part_request.can_rotate {
                        let rotated_rect = Rectangle::new(x as f64, y as f64, part_request.height, part_request.width);
                        if layout.can_place_part(&rotated_rect) {
                            let placed_part = PlacedPart::new(x as f64, y as f64, part_request.height, part_request.width, true);
                            layout.add_part(placed_part);
                            placed = true;
                            break 'outer;
                        }
                    }
                }
            }

            if !placed {
                result.unplaced_parts += 1;
            }
        }

        result.add_layout(layout);
        Ok(result)
    }

    fn name(&self) -> &'static str {
        "First Fit Decreasing Algorithm"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_best_fit_algorithm() {
        let algorithm = BestFitAlgorithm::new();
        let material = Material::new(1000.0, 1000.0).unwrap();
        let requests = vec![
            CuttingRequest::new(300.0, 400.0, 1),
            CuttingRequest::new(200.0, 300.0, 1),
        ];

        let result = algorithm.optimize(&material, &requests);
        assert!(result.is_ok());
        
        let result = result.unwrap();
        assert!(!result.layouts.is_empty());
        assert_eq!(result.layouts[0].parts.len(), 2);
    }

    #[test]
    fn test_bottom_left_fill_algorithm() {
        let algorithm = BottomLeftFillAlgorithm::new();
        let material = Material::new(1000.0, 1000.0).unwrap();
        let requests = vec![
            CuttingRequest::new(500.0, 500.0, 1),
            CuttingRequest::new(400.0, 400.0, 1),
        ];

        let result = algorithm.optimize(&material, &requests);
        assert!(result.is_ok());
        
        let result = result.unwrap();
        assert!(!result.layouts.is_empty());
    }

    #[test]
    fn test_algorithm_names() {
        let best_fit = BestFitAlgorithm::new();
        let bottom_left = BottomLeftFillAlgorithm::new();
        let ffd = FirstFitDecreasingAlgorithm::new();

        assert_eq!(best_fit.name(), "Best Fit Algorithm");
        assert_eq!(bottom_left.name(), "Bottom-Left Fill Algorithm");
        assert_eq!(ffd.name(), "First Fit Decreasing Algorithm");
    }

    #[test]
    fn test_rectangle_intersection() {
        let rect1 = Rectangle::new(0.0, 0.0, 100.0, 100.0);
        let rect2 = Rectangle::new(50.0, 50.0, 100.0, 100.0);
        let rect3 = Rectangle::new(200.0, 200.0, 100.0, 100.0);

        assert!(rect1.intersects(&rect2));
        assert!(!rect1.intersects(&rect3));
    }
}