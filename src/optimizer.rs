//! Основной оптимизатор раскроя материалов

use crate::{
    algorithms::{BestFitAlgorithm /* BottomLeftFillAlgorithm */},
    types::*,
    // parallel::ParallelOptimizer,
    CuttingAlgorithm,
    OptimizationConfig,
    Result,
};
use std::time::{Duration, Instant};

/// Основной оптимизатор раскроя
pub struct CuttingOptimizer {
    config: OptimizationConfig,
    algorithms: Vec<Box<dyn CuttingAlgorithm>>,
}

impl CuttingOptimizer {
    /// Создает новый оптимизатор с настройками по умолчанию
    pub fn new() -> Self {
        Self::with_config(OptimizationConfig::default())
    }

    /// Создает оптимизатор с пользовательской конфигурацией
    pub fn with_config(config: OptimizationConfig) -> Self {
        let algorithms: Vec<Box<dyn CuttingAlgorithm>> = vec![Box::new(BestFitAlgorithm::new())];

        Self { config, algorithms }
    }

    pub fn optimize(
        &self,
        material: &Material,
        requests: &[CuttingRequest],
    ) -> Result<CuttingResult> {
        Ok(CuttingResult::new())
    }
}

//     /// Добавляет новый алгоритм оптимизации
//     pub fn add_algorithm(&mut self, algorithm: Box<dyn CuttingAlgorithm>) {
//         self.algorithms.push(algorithm);
//     }

//     /// Выполняет оптимизацию раскроя
//     pub fn optimize(&self, material: &Material, requests: &[CuttingRequest]) -> Result<CuttingResult> {
//         let start_time = Instant::now();

//         // Валидация входных данных
//         self.validate_input(material, requests)?;

//         // Сортировка запросов по приоритету и размеру
//         let mut sorted_requests = requests.to_vec();
//         self.sort_requests(&mut sorted_requests);

//         // Выбор стратегии оптимизации
//         let result = if self.should_use_parallel(&sorted_requests) {
//             self.optimize_parallel(material, &sorted_requests)?
//         } else {
//             self.optimize_sequential(material, &sorted_requests)?
//         };

//         // Проверка таймаута
//         if let Some(timeout) = self.config.timeout_seconds {
//             if start_time.elapsed() > Duration::from_secs(timeout) {
//                 return Err(OptimizationError::TimeoutExceeded);
//             }
//         }

//         // Финализация результата
//         let mut final_result = result;
//         final_result.execution_time_ms = start_time.elapsed().as_millis() as u64;
//         final_result.recalculate_totals();

//         Ok(final_result)
//     }

//     /// Оптимизация одного материала с заданными алгоритмами
//     pub fn optimize_single_material(
//         &self,
//         material: &Material,
//         requests: &[CuttingRequest],
//     ) -> Result<CuttingLayout> {
//         let mut best_layout = CuttingLayout::new(material.clone());
//         let mut best_utilization = 0.0;

//         for algorithm in &self.algorithms {
//             match algorithm.optimize(material, requests) {
//                 Ok(result) => {
//                     if let Some(layout) = result.layouts.into_iter().next() {
//                         if layout.utilization > best_utilization {
//                             best_utilization = layout.utilization;
//                             best_layout = layout;
//                         }
//                     }
//                 }
//                 Err(_) => continue, // Пропускаем неудачные алгоритмы
//             }
//         }

//         Ok(best_layout)
//     }

//     /// Валидация входных данных
//     fn validate_input(&self, material: &Material, requests: &[CuttingRequest]) -> Result<()> {
//         if material.width <= 0.0 || material.height <= 0.0 {
//             return Err(OptimizationError::InvalidMaterialSize(
//                 "Material dimensions must be positive".to_string(),
//             ));
//         }

//         for (i, request) in requests.iter().enumerate() {
//             if request.width <= 0.0 || request.height <= 0.0 {
//                 return Err(OptimizationError::InvalidPartSize(
//                     format!("Part {} has invalid dimensions", i),
//                 ));
//             }

//             if !request.fits_in_material(material) {
//                 return Err(OptimizationError::PartDoesNotFit(
//                     format!("Part {} does not fit in material", i),
//                 ));
//             }
//         }

//         Ok(())
//     }

//     /// Сортировка запросов по оптимальному порядку размещения
//     fn sort_requests(&self, requests: &mut [CuttingRequest]) {
//         requests.sort_by(|a, b| {
//             // Сначала по приоритету (убывание)
//             let priority_cmp = b.priority.cmp(&a.priority);
//             if priority_cmp != std::cmp::Ordering::Equal {
//                 return priority_cmp;
//             }

//             // Затем по площади (убывание)
//             let area_cmp = b.area().partial_cmp(&a.area()).unwrap_or(std::cmp::Ordering::Equal);
//             if area_cmp != std::cmp::Ordering::Equal {
//                 return area_cmp;
//             }

//             // Затем по максимальной стороне (убывание)
//             let max_side_a = a.width.max(a.height);
//             let max_side_b = b.width.max(b.height);
//             max_side_b.partial_cmp(&max_side_a).unwrap_or(std::cmp::Ordering::Equal)
//         });
//     }

//     /// Определяет, стоит ли использовать параллельную обработку
//     fn should_use_parallel(&self, requests: &[CuttingRequest]) -> bool {
//         if !cfg!(feature = "parallel") {
//             return false;
//         }

//         let total_parts: usize = requests.iter().map(|r| r.quantity).sum();
//         total_parts > 20 || requests.len() > 10
//     }

//     /// Последовательная оптимизация
//     fn optimize_sequential(
//         &self,
//         material: &Material,
//         requests: &[CuttingRequest],
//     ) -> Result<CuttingResult> {
//         let mut result = CuttingResult::new();
//         let mut remaining_requests = requests.to_vec();

//         while !remaining_requests.is_empty() {
//             let layout = self.optimize_single_material(material, &remaining_requests)?;

//             // Подсчет размещенных деталей
//             let placed_count = layout.parts.len();
//             if placed_count == 0 {
//                 break; // Не удалось разместить ни одной детали
//             }

//             // Обновление оставшихся запросов
//             self.update_remaining_requests(&mut remaining_requests, &layout);

//             result.add_layout(layout);
//         }

//         // Подсчет не размещенных деталей
//         result.unplaced_parts = remaining_requests.iter().map(|r| r.quantity).sum();

//         Ok(result)
//     }

//     /// Параллельная оптимизация
//     fn optimize_parallel(
//         &self,
//         material: &Material,
//         requests: &[CuttingRequest],
//     ) -> Result<CuttingResult> {
//         let parallel_optimizer = ParallelOptimizer::new(self.config.clone());
//         parallel_optimizer.optimize(material, requests, &self.algorithms)
//     }

//     /// Обновляет список оставшихся запросов после размещения
//     fn update_remaining_requests(
//         &self,
//         remaining_requests: &mut Vec<CuttingRequest>,
//         layout: &CuttingLayout,
//     ) {
//         // Простая реализация - уменьшаем количество первого подходящего запроса
//         // В реальной реализации нужно отслеживать соответствие размещенных деталей запросам

//         for part in &layout.parts {
//             for request in remaining_requests.iter_mut() {
//                 if request.quantity > 0 {
//                     let matches = if request.can_rotate {
//                         (request.width == part.rectangle.width && request.height == part.rectangle.height) ||
//                         (request.height == part.rectangle.width && request.width == part.rectangle.height)
//                     } else {
//                         request.width == part.rectangle.width && request.height == part.rectangle.height
//                     };

//                     if matches {
//                         request.quantity -= 1;
//                         break;
//                     }
//                 }
//             }
//         }

//         // Удаляем запросы с нулевым количеством
//         remaining_requests.retain(|r| r.quantity > 0);
//     }
// }

// impl Default for CuttingOptimizer {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_optimizer_creation() {
//         let optimizer = CuttingOptimizer::new();
//         assert!(!optimizer.algorithms.is_empty());
//     }

//     #[test]
//     fn test_basic_optimization() {
//         let optimizer = CuttingOptimizer::new();
//         let material = Material::new(1000.0, 2000.0).unwrap();
//         let requests = vec![
//             CuttingRequest::new(300.0, 400.0, 2),
//             CuttingRequest::new(200.0, 300.0, 1),
//         ];

//         let result = optimizer.optimize(&material, &requests);
//         assert!(result.is_ok());

//         let result = result.unwrap();
//         assert!(!result.layouts.is_empty());
//         assert!(result.total_utilization > 0.0);
//     }

//     #[test]
//     fn test_invalid_material() {
//         let optimizer = CuttingOptimizer::new();
//         let material = Material::new(-100.0, 200.0);
//         assert!(material.is_err());
//     }

//     #[test]
//     fn test_part_too_large() {
//         let optimizer = CuttingOptimizer::new();
//         let material = Material::new(100.0, 200.0).unwrap();
//         let requests = vec![CuttingRequest::new(300.0, 400.0, 1)];

//         let result = optimizer.optimize(&material, &requests);
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_request_sorting() {
//         let optimizer = CuttingOptimizer::new();
//         let mut requests = vec![
//             CuttingRequest::new(100.0, 200.0, 1), // площадь 20000
//             CuttingRequest::new(300.0, 400.0, 1), // площадь 120000
//             CuttingRequest::new(150.0, 250.0, 1), // площадь 37500
//         ];

//         optimizer.sort_requests(&mut requests);

//         // После сортировки самая большая деталь должна быть первой
//         assert_eq!(requests[0].width, 300.0);
//         assert_eq!(requests[0].height, 400.0);
//     }
// }
