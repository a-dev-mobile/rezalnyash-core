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
#[derive(Debug)]
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



