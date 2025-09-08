use serde::{Deserialize, Serialize};

use crate::features::engine::cut_list_thread::CutListThread;
use crate::features::engine::model::calculation_request::CalculationRequest;
use crate::features::engine::model::calculation_response::CalculationResponse;
use crate::features::engine::model::calculation_response_builder::CalculationResponseBuilder;
use crate::features::engine::model::client_info::ClientInfo;
use crate::features::engine::model::solution::Solution;
use crate::features::engine::model::{calculation_response::Mosaic, status::Status, stock_solution::StockSolution};
use crate::features::input::models::tile_dimensions::TileDimensions;
use std::collections::{HashMap, LinkedList};
use std::sync::atomic::{AtomicI32, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

// Java: private static final AtomicInteger idAtomicInteger = new AtomicInteger(0);


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub status: Status,
    pub calculation_request: CalculationRequest,
    pub solution: CalculationResponse,
    pub solutions: HashMap<String, Vec<Solution>>,
    pub client_info: ClientInfo,
    pub stock_dimensions_per_material: HashMap<String, Vec<TileDimensions>>, 
    pub tile_dimensions_per_material: HashMap<String, Vec<TileDimensions>>, 
    pub no_material_tiles: Vec<TileDimensions>,

    pub thread_group_rankings: HashMap<String, HashMap<String, i32>>, // material -> group -> ranking
    pub finished_threads: HashMap<String, i32>, // material -> count
    pub has_solution_all_fit: bool,
    pub factor: u32,
    pub threads: Vec<CutListThread>, // List of threads for tracking finished ones (Java: List<CutListThread> threads)
    pub start_time: u64, // Start time for the task
}









impl Default for Task {
     fn default() -> Self {
        Self {
            id: String::new(),
            status: Status::Running, 
            thread_group_rankings: HashMap::new(),
            finished_threads: HashMap::new(),
            has_solution_all_fit: false,
            solutions: HashMap::new(),
            threads: Vec::new(),
            calculation_request: CalculationRequest::default(),
            solution: CalculationResponse::default(),
            stock_dimensions_per_material: HashMap::new(),
            tile_dimensions_per_material: HashMap::new(),
            client_info: ClientInfo::default(),
            factor: 1,
            no_material_tiles: Vec::new(),
            start_time:  SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64,
        }
    }
}

impl Task {


pub fn build_solution(&mut self)  {

let builder = CalculationResponseBuilder{
    task: self.clone(),
    calculation_request: self.calculation_request.clone(),
    solutions: self.solutions.clone(),
    no_stock_material_panels: self.no_material_tiles.clone(),
};



}

    /// Java: public void addMaterialToCompute(String str)
    pub fn add_material_to_compute(&mut self, material: &str) {
        self.solutions.insert(material.to_string(), Vec::new());
        self.thread_group_rankings.insert(material.to_string(), HashMap::new());
    }
    
    /// Java: public void incrementThreadGroupRankings(String str, String str2)
    pub fn increment_thread_group_rankings(&mut self, material: &str, group: &str) {
        if let Some(material_rankings) = self.thread_group_rankings.get_mut(material) {
            let current_ranking = material_rankings.get(group).copied().unwrap_or(0);
            material_rankings.insert(group.to_string(), current_ranking + 1);
        }
    }
    
    // /// Java: public synchronized int getNbrFinishedThreads(String str)
    // pub fn get_nbr_finished_threads(&self, material: &str) -> i32 {
    //     let mut count = 0;
    //     for thread in &self.threads {
    //         if matches!(thread.status, Status::Finished) 
    //             && thread.material.as_ref().map(|m| m == material).unwrap_or(false) {
    //             count += 1;
    //         }
    //     }
    //     count
    // }
    
    /// Add a finished thread to the tracking list (equivalent to Java thread completion)
    // pub fn add_finished_thread(&mut self, material: String, group: String) {
    //     let thread_info = CutListThreadInfo {
    //         status: Status::Finished,
    //         material: Some(material),
    //         group,
    //     };
    //     self.threads.push(thread_info);
    // }
    
    pub fn is_running(&self) -> bool {
        matches!(self.status, Status::Running)
    }
    
    pub fn has_solution_all_fit(&self) -> bool {
        self.has_solution_all_fit
    }
    
    pub fn get_solutions(&self, material: &str) -> Vec<Solution> {
        self.solutions.get(material).cloned().unwrap_or_default()
    }
    
    pub fn get_thread_group_rankings(&self, material: &str) -> HashMap<String, i32> {
        self.thread_group_rankings.get(material).cloned().unwrap_or_default()
    }
    
    // pub fn get_finished_threads(&self, material: &str) -> i32 {
    //     self.get_nbr_finished_threads(material)
    // }
    
    /// Java: task.getSolutions(material) returns existing solutions
    pub fn add_solutions(&mut self, material: &str, solutions: Vec<Solution>) {
        self.solutions.insert(material.to_string(), solutions);
    }
}