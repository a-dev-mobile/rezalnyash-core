//! src/optimizer.rs - Рефакторинг с явными методами

use crate::{
    algorithms::{BestFitAlgorithm, BottomLeftFillAlgorithm},
    parallel::{BatchProcessor, ParallelOptimizer},
    types::*,
    CuttingAlgorithm, OptimizationConfig, Result,
};
use std::time::Instant;

/// Стратегии оптимизации
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OptimizationStrategy {
    /// Быстрая последовательная обработка
    Sequential,
    /// Параллельная обработка
    Parallel,
    /// Пакетная обработка для больших объемов
    Batch,
    /// Автоматический выбор оптимальной стратегии
    Auto,
}

/// Основной оптимизатор раскроя
pub struct CutListOptimizerService {
    config: OptimizationConfig,
    algorithms: Vec<Box<dyn CuttingAlgorithm>>,
}

impl CutListOptimizerService {
    /// Создает новый оптимизатор с настройками по умолчанию
    pub fn new() -> Self {
        Self::with_config(OptimizationConfig::default())
    }

    /// Создает оптимизатор с пользовательской конфигурацией
    pub fn with_config(config: OptimizationConfig) -> Self {
        let algorithms: Vec<Box<dyn CuttingAlgorithm>> = vec![
            Box::new(BestFitAlgorithm::new()),
            Box::new(BottomLeftFillAlgorithm::new()),
        ];

        Self { config, algorithms }
    }

    /// Добавляет новый алгоритм оптимизации
    pub fn add_algorithm(&mut self, algorithm: Box<dyn CuttingAlgorithm>) {
        self.algorithms.push(algorithm);
    }

    // ===== ОСНОВНЫЕ МЕТОДЫ ОПТИМИЗАЦИИ =====

    /// Быстрая последовательная оптимизация
    /// Подходит для: малых объемов, быстрого прототипирования
    pub fn optimize_sequential(
        &self,
        material: &Material,
        requests: &[CuttingRequest],
    ) -> Result<CuttingResult> {
        let start_time = Instant::now();
        self.validate_input(material, requests)?;

        let mut sorted_requests = requests.to_vec();
        self.sort_requests(&mut sorted_requests);

        let algorithm = &self.algorithms[0]; // Используем первый алгоритм
        let result = algorithm.optimize(material, &sorted_requests)?;

        self.finalize_result(result, start_time)
    }

    /// Параллельная оптимизация
    /// Подходит для: средних объемов, когда производительность важна
    pub fn optimize_parallel(
        &self,
        material: &Material,
        requests: &[CuttingRequest],
    ) -> Result<CuttingResult> {
        let start_time = Instant::now();
        self.validate_input(material, requests)?;

        let mut sorted_requests = requests.to_vec();
        self.sort_requests(&mut sorted_requests);

        let parallel_optimizer = ParallelOptimizer::new(self.config.clone());
        let result = parallel_optimizer.optimize(material, &sorted_requests, &self.algorithms)?;

        self.finalize_result(result, start_time)
    }

    /// Пакетная оптимизация для больших объемов
    /// Подходит для: больших объемов, фоновой обработки
    pub fn optimize_batch(
        &self,
        material: &Material,
        requests: &[CuttingRequest],
    ) -> Result<CuttingResult> {
        self.optimize_batch_with_size(material, requests, 50)
    }

    /// Пакетная оптимизация с настраиваемым размером пакета
    pub fn optimize_batch_with_size(
        &self,
        material: &Material,
        requests: &[CuttingRequest],
        batch_size: usize,
    ) -> Result<CuttingResult> {
        let start_time = Instant::now();
        self.validate_input(material, requests)?;

        let batch_processor = BatchProcessor::new(batch_size);
        let algorithm = &self.algorithms[0];
        let result =
            batch_processor.process_batches_parallel(material, requests, algorithm.as_ref())?;

        self.finalize_result(result, start_time)
    }

    /// Оптимизация с выбором стратегии
    pub fn optimize_with_strategy(
        &self,
        material: &Material,
        requests: &[CuttingRequest],
        strategy: OptimizationStrategy,
    ) -> Result<CuttingResult> {
        match strategy {
            OptimizationStrategy::Sequential => self.optimize_sequential(material, requests),
            OptimizationStrategy::Parallel => self.optimize_parallel(material, requests),
            OptimizationStrategy::Batch => self.optimize_batch(material, requests),
            OptimizationStrategy::Auto => {
                let chosen_strategy = self.choose_optimal_strategy(requests);
                self.optimize_with_strategy(material, requests, chosen_strategy)
            }
        }
    }

    // ===== СПЕЦИАЛИЗИРОВАННЫЕ МЕТОДЫ =====

    /// Быстрая оценка без полной оптимизации
    pub fn estimate_quick(
        &self,
        material: &Material,
        requests: &[CuttingRequest],
    ) -> Result<OptimizationEstimate> {
        self.validate_input(material, requests)?;

        let total_area: f64 = requests.iter().map(|r| r.total_area()).sum();
        let material_area = material.area();

        let theoretical_sheets = (total_area / material_area).ceil() as usize;
        let estimated_efficiency =
            (total_area / (theoretical_sheets as f64 * material_area)).min(1.0);

        Ok(OptimizationEstimate {
            estimated_sheets: theoretical_sheets,
            estimated_efficiency,
            estimated_waste_area: (theoretical_sheets as f64 * material_area) - total_area,
            confidence: if requests.len() < 10 { 0.9 } else { 0.7 },
        })
    }

    /// Сравнение нескольких алгоритмов
    pub fn compare_algorithms(
        &self,
        material: &Material,
        requests: &[CuttingRequest],
    ) -> Result<Vec<AlgorithmComparison>> {
        let mut comparisons = Vec::new();

        for algorithm in &self.algorithms {
            let start = Instant::now();
            match algorithm.optimize(material, requests) {
                Ok(result) => {
                    comparisons.push(AlgorithmComparison {
                        algorithm_name: algorithm.name().to_string(),
                        execution_time_ms: start.elapsed().as_millis() as u64,
                        sheets_used: result.layouts.len(),
                        utilization: result.total_utilization,
                        waste_area: result.total_waste_area,
                        success: true,
                        error: None,
                    });
                }
                Err(e) => {
                    comparisons.push(AlgorithmComparison {
                        algorithm_name: algorithm.name().to_string(),
                        execution_time_ms: start.elapsed().as_millis() as u64,
                        sheets_used: 0,
                        utilization: 0.0,
                        waste_area: 0.0,
                        success: false,
                        error: Some(e.to_string()),
                    });
                }
            }
        }

        Ok(comparisons)
    }

    // ===== СЛУЖЕБНЫЕ МЕТОДЫ =====

    /// Выбирает оптимальную стратегию на основе данных
    fn choose_optimal_strategy(&self, requests: &[CuttingRequest]) -> OptimizationStrategy {
        let total_parts: usize = requests.iter().map(|r| r.quantity).sum();
        let unique_parts = requests.len();

        match (total_parts, unique_parts) {
            (parts, _) if parts > 100 => OptimizationStrategy::Batch,
            (parts, unique) if parts > 20 && unique > 5 => OptimizationStrategy::Parallel,
            _ => OptimizationStrategy::Sequential,
        }
    }

    fn validate_input(&self, material: &Material, requests: &[CuttingRequest]) -> Result<()> {
        if material.width <= 0.0 || material.height <= 0.0 {
            return Err(OptimizationError::InvalidMaterialSize(
                "Material dimensions must be positive".to_string(),
            ));
        }

        for (i, request) in requests.iter().enumerate() {
            if request.width <= 0.0 || request.height <= 0.0 {
                return Err(OptimizationError::InvalidPartSize(format!(
                    "Part {} has invalid dimensions",
                    i
                )));
            }

            if !request.fits_in_material(material) {
                return Err(OptimizationError::PartDoesNotFit(format!(
                    "Part {} does not fit in material",
                    i
                )));
            }
        }

        Ok(())
    }

    fn sort_requests(&self, requests: &mut [CuttingRequest]) {
        requests.sort_by(|a, b| {
            // Сначала по приоритету (убывание)
            let priority_cmp = b.priority.cmp(&a.priority);
            if priority_cmp != std::cmp::Ordering::Equal {
                return priority_cmp;
            }

            // Затем по площади (убывание)
            let area_cmp = b
                .area()
                .partial_cmp(&a.area())
                .unwrap_or(std::cmp::Ordering::Equal);
            if area_cmp != std::cmp::Ordering::Equal {
                return area_cmp;
            }

            // Затем по максимальной стороне (убывание)
            let max_side_a = a.width.max(a.height);
            let max_side_b = b.width.max(b.height);
            max_side_b
                .partial_cmp(&max_side_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    fn finalize_result(
        &self,
        mut result: CuttingResult,
        start_time: Instant,
    ) -> Result<CuttingResult> {
        result.execution_time_ms = start_time.elapsed().as_millis() as u64;
        result.recalculate_totals();
        Ok(result)
    }
}

impl Default for CutListOptimizerService {
    fn default() -> Self {
        Self::new()
    }
}

// ===== ТИПЫ ДЛЯ API =====

/// Быстрая оценка оптимизации
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OptimizationEstimate {
    pub estimated_sheets: usize,
    pub estimated_efficiency: f64,
    pub estimated_waste_area: f64,
    pub confidence: f64, // 0.0 - 1.0
}

/// Сравнение алгоритмов
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AlgorithmComparison {
    pub algorithm_name: String,
    pub execution_time_ms: u64,
    pub sheets_used: usize,
    pub utilization: f64,
    pub waste_area: f64,
    pub success: bool,
    pub error: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimizer_creation() {
        let optimizer = CutListOptimizerService::new();
        assert!(!optimizer.algorithms.is_empty());
    }

    #[test]
    fn test_sequential_optimization() {
        let optimizer = CutListOptimizerService::new();
        let material = Material::new(1000.0, 2000.0).unwrap();
        let requests = vec![
            CuttingRequest::new(300.0, 400.0, 2),
            CuttingRequest::new(200.0, 300.0, 1),
        ];

        let result = optimizer.optimize_sequential(&material, &requests);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(!result.layouts.is_empty());
        assert!(result.total_utilization > 0.0);
    }

    #[test]
    fn test_parallel_optimization() {
        let optimizer = CutListOptimizerService::new();
        let material = Material::new(1000.0, 2000.0).unwrap();
        let requests = vec![
            CuttingRequest::new(300.0, 400.0, 2),
            CuttingRequest::new(200.0, 300.0, 1),
        ];

        let result = optimizer.optimize_parallel(&material, &requests);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(!result.layouts.is_empty());
        assert!(result.total_utilization > 0.0);
    }

    #[test]
    fn test_strategy_selection() {
        let optimizer = CutListOptimizerService::new();

        // Маленькая задача → sequential
        let small_requests = vec![CuttingRequest::new(100.0, 200.0, 1)];
        let strategy = optimizer.choose_optimal_strategy(&small_requests);
        assert_eq!(strategy, OptimizationStrategy::Sequential);

        // Большая задача → batch
        let large_requests = (0..50)
            .map(|i| CuttingRequest::new(100.0 + i as f64, 200.0, 3))
            .collect::<Vec<_>>();
        let strategy = optimizer.choose_optimal_strategy(&large_requests);
        assert_eq!(strategy, OptimizationStrategy::Batch);
    }

    #[test]
    fn test_quick_estimate() {
        let optimizer = CutListOptimizerService::new();
        let material = Material::new(1000.0, 1000.0).unwrap();
        let requests = vec![CuttingRequest::new(300.0, 400.0, 8)];

        let estimate = optimizer.estimate_quick(&material, &requests).unwrap();
        assert!(estimate.estimated_sheets > 0);
        assert!(estimate.estimated_efficiency > 0.0);
        assert!(estimate.confidence > 0.0);
    }
}
