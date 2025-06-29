//! Task structure definition for cut list optimization
//!
//! This module contains the main Task struct that manages the lifecycle of cutting calculations,
//! coordinates multiple threads, tracks progress, and aggregates solutions.

use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
    time::SystemTime,
};

use crate::{enums::status::Status, models::{calculation_request::CalculationRequest, solution::Solution}};

/// Task represents a complete cutting optimization job with thread management and progress tracking
#[derive(Debug, Clone)]
pub struct Task {
    // Идентификация
   pub id: String,
    
    // Входные данные  
   pub calculation_request: CalculationRequest,
   pub factor: i64,
    
    // Состояние
   pub status: Status,
   pub percentage_done: u8,
   pub start_time: Option<SystemTime>,

    
    // Результаты
   pub solutions: Vec<Solution>,
   pub best_solution: Option<Solution>,

    // Отладка
   pub error_message: Option<String>,
   pub iterations_completed: usize,
}



// Добавьте эти методы в impl Task (в models/task/structs.rs или где у вас определен Task):

impl Task {
    /// Check if task is currently running
    pub fn is_running(&self) -> bool {
        matches!(self.status, Status::Running)
    }

    /// Check if we have a solution where all tiles fit perfectly
    pub fn has_solution_all_fit(&self) -> bool {
        // Проверяем, есть ли решение где все панели размещены
        if let Some(ref best_solution) = self.best_solution {
            // Если есть лучшее решение, проверяем что все панели размещены
            // Это можно определить по отсутствию неразмещенных панелей
            // Или по 100% использованию материала
            // 
            // Пока что возвращаем false, так как структура Solution не показана
            return false;
        }
        
        // Проверяем среди всех решений
        for solution in &self.solutions {
            // TODO: Реализовать проверку что все панели размещены в решении
            // Например, можно проверить:
            // - solution.no_fit_panels.is_empty()
            // - solution.efficiency >= 1.0
            // - сумма размещенных панелей == сумма исходных панелей
        }
        
        false
    }

    /// Set task status to running
    pub fn set_running_status(&mut self) {
        self.status = Status::Running;
        self.start_time = Some(std::time::SystemTime::now());
    }

    /// Set task status to finished
    pub fn set_finished_status(&mut self) {
        self.status = Status::Finished;
    }

    /// Get number of running threads for this task
    pub fn get_nbr_running_threads(&self) -> usize {
        // TODO: Реализовать подсчет активных потоков
        // Это потребует отслеживания потоков в структуре Task
        0
    }

    /// Get number of queued threads for this task  
    pub fn get_nbr_queued_threads(&self) -> usize {
        // TODO: Реализовать подсчет очереди потоков
        0
    }

    /// Add a solution to the task
    pub fn add_solution(&mut self, solution: Solution) {
        self.solutions.push(solution);
        
        // Обновляем лучшее решение если нужно
        self.update_best_solution();
    }

    /// Update the best solution based on current solutions
    fn update_best_solution(&mut self) {
        if self.solutions.is_empty() {
            return;
        }

        // Находим лучшее решение по заданным критериям
        let mut best_idx = 0;
        let mut best_score = self.calculate_solution_score(&self.solutions[0]);

        for (idx, solution) in self.solutions.iter().enumerate().skip(1) {
            let score = self.calculate_solution_score(solution);
            if score > best_score {
                best_score = score;
                best_idx = idx;
            }
        }

        self.best_solution = Some(self.solutions[best_idx].clone());
    }

    /// Calculate solution score for comparison
    fn calculate_solution_score(&self, solution: &Solution) -> f64 {
        // TODO: Реализовать расчет оценки решения
        // Можно учитывать:
        // - Количество размещенных панелей
        // - Эффективность использования материала
        // - Количество резов
        // - Общую площадь отходов
        
        // Пока что простая заглушка
        0.0
    }
}

