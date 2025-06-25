

pub mod types;
pub mod optimizer;
pub mod algorithms;
pub mod parallel;

pub use types::{Material, CuttingRequest, CuttingResult, CuttingLayout, Rectangle, OptimizationError};
pub use optimizer::{CuttingOptimizer, OptimizationStrategy, OptimizationEstimate, AlgorithmComparison};
pub use algorithms::{BestFitAlgorithm, BottomLeftFillAlgorithm};

/// Результат выполнения операции оптимизации
pub type Result<T> = std::result::Result<T, OptimizationError>;

/// Основной интерфейс для оптимизации раскроя
pub trait CuttingAlgorithm: Send + Sync {
    /// Выполняет оптимизацию раскроя для заданного материала и запросов
    fn optimize(&self, material: &Material, requests: &[CuttingRequest]) -> Result<CuttingResult>;
    
    /// Возвращает название алгоритма
    fn name(&self) -> &'static str;
}

/// Конфигурация для оптимизации
#[derive(Debug, Clone)]
pub struct OptimizationConfig {
    /// Максимальное количество потоков для параллельной обработки
    pub max_threads: Option<usize>,
    /// Зазор между деталями (мм)
    pub cutting_gap: f64,
    /// Минимальный размер остатка для использования (мм)
    pub min_waste_size: f64,
    /// Максимальное время выполнения (секунды)
    pub timeout_seconds: Option<u64>,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            max_threads: Some(std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4)),
            cutting_gap: 2.0,
            min_waste_size: 50.0,
            timeout_seconds: Some(300), // 5 минут
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequential_optimization() {
        let material = Material::new(1000.0, 2000.0).unwrap();
        let requests = vec![
            CuttingRequest::new(300.0, 400.0, 2),
            CuttingRequest::new(200.0, 300.0, 1),
        ];

        let optimizer = CuttingOptimizer::new();
        let result = optimizer.optimize_sequential(&material, &requests);
        
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(!result.layouts.is_empty());
    }

    #[test]
    fn test_parallel_optimization() {
        let material = Material::new(1000.0, 2000.0).unwrap();
        let requests = vec![
            CuttingRequest::new(300.0, 400.0, 2),
            CuttingRequest::new(200.0, 300.0, 1),
        ];

        let optimizer = CuttingOptimizer::new();
        let result = optimizer.optimize_parallel(&material, &requests);
        
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(!result.layouts.is_empty());
    }

    #[test]
    fn test_strategy_auto_selection() {
        let material = Material::new(1000.0, 2000.0).unwrap();
        let requests = vec![CuttingRequest::new(300.0, 400.0, 1)];

        let optimizer = CuttingOptimizer::new();
        let result = optimizer.optimize_with_strategy(&material, &requests, OptimizationStrategy::Auto);
        
        assert!(result.is_ok());
    }
}