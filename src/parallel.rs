//! Модуль для параллельной обработки оптимизации раскроя

use rayon::prelude::*;
use crossbeam::channel;
use crate::{types::*, CuttingAlgorithm, OptimizationConfig, Result};
use std::sync::Arc;

/// Параллельный оптимизатор раскроя
pub struct ParallelOptimizer {
    config: OptimizationConfig,
}

impl ParallelOptimizer {
    /// Создает новый параллельный оптимизатор
    pub fn new(config: OptimizationConfig) -> Self {
        Self { config }
    }

    /// Выполняет параллельную оптимизацию алгоритмов
    pub fn optimize(
        &self,
        material: &Material,
        requests: &[CuttingRequest],
        algorithms: &[Box<dyn CuttingAlgorithm>],
    ) -> Result<CuttingResult> {
        // Создаем локальный пул потоков если нужно
        let result = if let Some(max_threads) = self.config.max_threads {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(max_threads)
                .build()
                .map_err(|e| OptimizationError::CalculationError(format!("Thread pool error: {}", e)))?;

            pool.install(|| {
                algorithms
                    .par_iter()
                    .map(|algorithm| algorithm.optimize(material, requests))
                    .collect::<Vec<_>>()
            })
        } else {
            algorithms
                .par_iter()
                .map(|algorithm| algorithm.optimize(material, requests))
                .collect::<Vec<_>>()
        };

        // Выбираем лучший результат
        let mut best_result = None;
        let mut best_utilization = 0.0;

        for algorithm_result in result {
            if let Ok(cutting_result) = algorithm_result {
                if cutting_result.total_utilization > best_utilization {
                    best_utilization = cutting_result.total_utilization;
                    best_result = Some(cutting_result);
                }
            }
        }

        best_result.ok_or(OptimizationError::CalculationError(
            "No algorithm produced valid result".to_string(),
        ))
    }

    /// Параллельная обработка множественных материалов
    pub fn optimize_multiple_materials(
        &self,
        materials: &[Material],
        requests: &[CuttingRequest],
        algorithm: &dyn CuttingAlgorithm,
    ) -> Result<Vec<CuttingResult>> {
        let results = if let Some(max_threads) = self.config.max_threads {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(max_threads)
                .build()
                .map_err(|e| OptimizationError::CalculationError(format!("Thread pool error: {}", e)))?;

            pool.install(|| {
                materials
                    .par_iter()
                    .map(|material| algorithm.optimize(material, requests))
                    .collect::<Vec<_>>()
            })
        } else {
            materials
                .par_iter()
                .map(|material| algorithm.optimize(material, requests))
                .collect::<Vec<_>>()
        };

        let mut valid_results = Vec::new();
        for result in results {
            match result {
                Ok(r) => valid_results.push(r),
                Err(_) => continue,
            }
        }

        if valid_results.is_empty() {
            return Err(OptimizationError::CalculationError(
                "No valid results from multiple materials".to_string(),
            ));
        }

        Ok(valid_results)
    }
}

/// Структура для пакетной обработки большого количества деталей
pub struct BatchProcessor {
    batch_size: usize,
}

impl BatchProcessor {
    /// Создает новый обработчик пакетов
    pub fn new(batch_size: usize) -> Self {
        Self { batch_size }
    }

    /// Разделяет запросы на пакеты для параллельной обработки
    pub fn split_into_batches(&self, requests: &[CuttingRequest]) -> Vec<Vec<CuttingRequest>> {
        let mut batches = Vec::new();
        let mut current_batch = Vec::new();
        let mut current_count = 0;

        for request in requests {
            if current_count + request.quantity > self.batch_size && !current_batch.is_empty() {
                batches.push(current_batch);
                current_batch = Vec::new();
                current_count = 0;
            }

            current_batch.push(request.clone());
            current_count += request.quantity;
        }

        if !current_batch.is_empty() {
            batches.push(current_batch);
        }

        batches
    }

    /// Параллельная обработка пакетов
    pub fn process_batches_parallel(
        &self,
        material: &Material,
        requests: &[CuttingRequest],
        algorithm: &dyn CuttingAlgorithm,
    ) -> Result<CuttingResult> {
        let batches = self.split_into_batches(requests);
        
        let results: Vec<_> = batches
            .par_iter()
            .map(|batch| algorithm.optimize(material, batch))
            .collect();

        self.merge_batch_results(results)
    }

    /// Объединяет результаты обработки пакетов
    fn merge_batch_results(&self, results: Vec<Result<CuttingResult>>) -> Result<CuttingResult> {
        let mut merged_result = CuttingResult::new();
        let mut total_execution_time = 0;

        for result in results {
            match result {
                Ok(batch_result) => {
                    for layout in batch_result.layouts {
                        merged_result.add_layout(layout);
                    }
                    merged_result.unplaced_parts += batch_result.unplaced_parts;
                    total_execution_time += batch_result.execution_time_ms;
                }
                Err(e) => return Err(e),
            }
        }

        merged_result.execution_time_ms = total_execution_time;
        merged_result.recalculate_totals();

        Ok(merged_result)
    }
}

/// Воркер для асинхронной обработки задач оптимизации
pub struct OptimizationWorker {
    receiver: crossbeam::channel::Receiver<OptimizationTask>,
    sender: crossbeam::channel::Sender<OptimizationResult>,
}

/// Задача для оптимизации
#[derive(Debug, Clone)]
pub struct OptimizationTask {
    pub id: usize,
    pub material: Material,
    pub requests: Vec<CuttingRequest>,
    pub algorithm_name: String,
}

/// Результат выполнения задачи
#[derive(Debug)]
pub struct OptimizationResult {
    pub task_id: usize,
    pub result: Result<CuttingResult>,
}

impl OptimizationWorker {
    /// Создает новый воркер
    pub fn new() -> (Self, crossbeam::channel::Sender<OptimizationTask>, crossbeam::channel::Receiver<OptimizationResult>) {
        let (task_sender, task_receiver) = crossbeam::channel::unbounded();
        let (result_sender, result_receiver) = crossbeam::channel::unbounded();

        let worker = Self {
            receiver: task_receiver,
            sender: result_sender,
        };

        (worker, task_sender, result_receiver)
    }

    /// Запускает обработку задач
    pub fn run(&self, algorithms: Arc<Vec<Box<dyn CuttingAlgorithm>>>) {
        loop {
            match self.receiver.recv() {
                Ok(task) => {
                    let result = self.process_task(&task, &algorithms);
                    let optimization_result = OptimizationResult {
                        task_id: task.id,
                        result,
                    };

                    if self.sender.send(optimization_result).is_err() {
                        break; // Канал закрыт
                    }
                }
                Err(_) => break, // Канал закрыт
            }
        }
    }

    /// Обрабатывает одну задачу
    fn process_task(
        &self,
        task: &OptimizationTask,
        algorithms: &[Box<dyn CuttingAlgorithm>],
    ) -> Result<CuttingResult> {
        let algorithm = algorithms
            .iter()
            .find(|alg| alg.name() == task.algorithm_name)
            .ok_or_else(|| {
                OptimizationError::CalculationError(format!(
                    "Algorithm '{}' not found",
                    task.algorithm_name
                ))
            })?;

        algorithm.optimize(&task.material, &task.requests)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::BestFitAlgorithm;

    #[test]
    fn test_batch_processor() {
        let processor = BatchProcessor::new(5);
        let requests = vec![
            CuttingRequest::new(100.0, 200.0, 3),
            CuttingRequest::new(150.0, 250.0, 4),
            CuttingRequest::new(200.0, 300.0, 2),
        ];

        let batches = processor.split_into_batches(&requests);
        assert!(!batches.is_empty());
    }

    #[test]
    fn test_parallel_optimizer() {
        let config = OptimizationConfig::default();
        let optimizer = ParallelOptimizer::new(config);
        
        let material = Material::new(1000.0, 1000.0).unwrap();
        let requests = vec![CuttingRequest::new(100.0, 200.0, 1)];
        let algorithms: Vec<Box<dyn CuttingAlgorithm>> = vec![
            Box::new(BestFitAlgorithm::new()),
        ];

        let result = optimizer.optimize(&material, &requests, &algorithms);
        assert!(result.is_ok());
    }

    #[test]
    fn test_optimization_worker() {
        let (worker, task_sender, result_receiver) = OptimizationWorker::new();
        
        let task = OptimizationTask {
            id: 1,
            material: Material::new(1000.0, 1000.0).unwrap(),
            requests: vec![CuttingRequest::new(100.0, 200.0, 1)],
            algorithm_name: "Best Fit Algorithm".to_string(),
        };

        // Отправляем задачу
        task_sender.send(task).unwrap();
        drop(task_sender); // Закрываем канал для завершения работы

        let algorithms: Arc<Vec<Box<dyn CuttingAlgorithm>>> = Arc::new(vec![
            Box::new(BestFitAlgorithm::new()),
        ]);

        // Запускаем воркер в отдельном потоке
        let algorithms_clone = algorithms.clone();
        std::thread::spawn(move || {
            worker.run(algorithms_clone);
        });

        // Получаем результат
        let result = result_receiver.recv().unwrap();
        assert_eq!(result.task_id, 1);
        assert!(result.result.is_ok());
    }
}