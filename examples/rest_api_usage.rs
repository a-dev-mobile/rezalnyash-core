//! examples/rest_api_usage.rs
//! Чистый пример интеграции в REST API

use rezalnyas_core::{
    CuttingOptimizer, CuttingRequest, Material, OptimizationStrategy,
    OptimizationConfig, OptimizationEstimate, AlgorithmComparison
};
use serde::{Deserialize, Serialize};

// ===== API СТРУКТУРЫ =====

#[derive(Debug, Deserialize)]
pub struct OptimizeRequest {
    pub material: MaterialRequest,
    pub parts: Vec<PartRequest>,
    pub strategy: Option<String>, // "sequential", "parallel", "batch", "auto"
    pub config: Option<ConfigRequest>,
}

#[derive(Debug, Deserialize)]
pub struct MaterialRequest {
    pub width: f64,
    pub height: f64,
    pub cost_per_area: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct PartRequest {
    pub width: f64,
    pub height: f64,
    pub quantity: usize,
    pub can_rotate: Option<bool>,
    pub priority: Option<i32>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigRequest {
    pub cutting_gap: Option<f64>,
    pub min_waste_size: Option<f64>,
    pub timeout_seconds: Option<u64>,
    pub batch_size: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct OptimizeResponse {
    pub success: bool,
    pub strategy_used: String,
    pub execution_time_ms: u64,
    pub sheets_count: usize,
    pub utilization: f64,
    pub waste_area: f64,
    pub total_cost: Option<f64>,
    pub unplaced_parts: usize,
    pub layouts: Vec<LayoutResponse>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LayoutResponse {
    pub sheet_number: usize,
    pub utilization: f64,
    pub parts_count: usize,
    pub waste_area: f64,
    pub parts: Vec<PlacedPartResponse>,
}

#[derive(Debug, Serialize)]
pub struct PlacedPartResponse {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub rotated: bool,
    pub name: Option<String>,
}

// ===== ОСНОВНОЙ СЕРВИС =====

pub struct CuttingService {
    optimizer: CuttingOptimizer,
}

impl CuttingService {
    pub fn new() -> Self {
        Self {
            optimizer: CuttingOptimizer::new(),
        }
    }

    /// Главный endpoint для оптимизации
    pub async fn optimize(&self, request: OptimizeRequest) -> OptimizeResponse {
        // Конвертируем материал
        let material = match Material::with_cost(
            request.material.width,
            request.material.height,
            request.material.cost_per_area.unwrap_or(0.0)
        ) {
            Ok(m) => m,
            Err(e) => return self.error_response(format!("Invalid material: {}", e)),
        };

        // Конвертируем детали
        let parts: Vec<CuttingRequest> = request.parts.into_iter().map(|p| {
            let mut cutting_request = CuttingRequest::new(p.width, p.height, p.quantity);
            cutting_request.can_rotate = p.can_rotate.unwrap_or(true);
            cutting_request.priority = p.priority.unwrap_or(0);
            cutting_request.id = p.name;
            cutting_request
        }).collect();

        // Выбираем стратегию
        let strategy = match request.strategy.as_deref() {
            Some("sequential") => OptimizationStrategy::Sequential,
            Some("parallel") => OptimizationStrategy::Parallel,
            Some("batch") => OptimizationStrategy::Batch,
            Some("auto") | None => OptimizationStrategy::Auto,
            Some(unknown) => {
                return self.error_response(format!("Unknown strategy: {}", unknown));
            }
        };

        // Применяем конфигурацию
        let optimizer = if let Some(config) = request.config {
            let opt_config = OptimizationConfig {
                cutting_gap: config.cutting_gap.unwrap_or(2.0),
                min_waste_size: config.min_waste_size.unwrap_or(50.0),
                timeout_seconds: config.timeout_seconds,
                max_threads: Some(4),
            };
            CuttingOptimizer::with_config(opt_config)
        } else {
            self.optimizer.clone()
        };

        // Выполняем оптимизацию
        let start_time = std::time::Instant::now();
        match optimizer.optimize_with_strategy(&material, &parts, strategy) {
            Ok(result) => {
                let execution_time = start_time.elapsed().as_millis() as u64;
                
                OptimizeResponse {
                    success: true,
                    strategy_used: format!("{:?}", strategy),
                    execution_time_ms: execution_time,
                    sheets_count: result.layouts.len(),
                    utilization: result.total_utilization,
                    waste_area: result.total_waste_area,
                    total_cost: result.total_cost,
                    unplaced_parts: result.unplaced_parts,
                    layouts: self.convert_layouts(&result.layouts),
                    error: None,
                }
            }
            Err(e) => self.error_response(format!("Optimization failed: {}", e)),
        }
    }

    /// Быстрая оценка без полной оптимизации
    pub async fn estimate(&self, material: MaterialRequest, parts: Vec<PartRequest>) -> Result<OptimizationEstimate, String> {
        let material = Material::new(material.width, material.height)
            .map_err(|e| format!("Invalid material: {}", e))?;

        let cutting_requests: Vec<CuttingRequest> = parts.into_iter()
            .map(|p| CuttingRequest::new(p.width, p.height, p.quantity))
            .collect();

        self.optimizer.estimate_quick(&material, &cutting_requests)
            .map_err(|e| format!("Estimation failed: {}", e))
    }

    /// Сравнение алгоритмов
    pub async fn compare_algorithms(&self, material: MaterialRequest, parts: Vec<PartRequest>) -> Result<Vec<AlgorithmComparison>, String> {
        let material = Material::new(material.width, material.height)
            .map_err(|e| format!("Invalid material: {}", e))?;

        let cutting_requests: Vec<CuttingRequest> = parts.into_iter()
            .map(|p| CuttingRequest::new(p.width, p.height, p.quantity))
            .collect();

        self.optimizer.compare_algorithms(&material, &cutting_requests)
            .map_err(|e| format!("Comparison failed: {}", e))
    }

    /// Информация о возможностях сервиса
    pub async fn get_capabilities(&self) -> serde_json::Value {
        serde_json::json!({
            "strategies": ["sequential", "parallel", "batch", "auto"],
            "parallel_available": true,
            "max_recommended_threads": std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1),
            "algorithms": ["BestFit", "BottomLeftFill"],
            "version": env!("CARGO_PKG_VERSION")
        })
    }

    // Служебные методы
    fn error_response(&self, message: String) -> OptimizeResponse {
        OptimizeResponse {
            success: false,
            strategy_used: "none".to_string(),
            execution_time_ms: 0,
            sheets_count: 0,
            utilization: 0.0,
            waste_area: 0.0,
            total_cost: None,
            unplaced_parts: 0,
            layouts: vec![],
            error: Some(message),
        }
    }

    fn convert_layouts(&self, layouts: &[rezalnyas_core::CuttingLayout]) -> Vec<LayoutResponse> {
        layouts.iter().enumerate().map(|(i, layout)| {
            LayoutResponse {
                sheet_number: i + 1,
                utilization: layout.utilization,
                parts_count: layout.parts.len(),
                waste_area: layout.waste_area,
                parts: layout.parts.iter().map(|part| {
                    PlacedPartResponse {
                        x: part.rectangle.x,
                        y: part.rectangle.y,
                        width: part.rectangle.width,
                        height: part.rectangle.height,
                        rotated: part.rotated,
                        name: part.request_id.clone(),
                    }
                }).collect(),
            }
        }).collect()
    }
}

// ===== ПРИМЕРЫ ИСПОЛЬЗОВАНИЯ =====

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = CuttingService::new();

    // Демонстрация всех стратегий
    demo_all_strategies(&service).await?;
    
    // Демонстрация дополнительных возможностей
    demo_additional_features(&service).await?;

    Ok(())
}

async fn demo_all_strategies(service: &CuttingService) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Демонстрация всех стратегий оптимизации ===\n");

    let base_request = OptimizeRequest {
        material: MaterialRequest {
            width: 1220.0,
            height: 2440.0,
            cost_per_area: Some(0.08),
        },
        parts: vec![
            PartRequest {
                width: 300.0,
                height: 400.0,
                quantity: 5,
                can_rotate: Some(true),
                priority: Some(1),
                name: Some("Столешница".to_string()),
            },
            PartRequest {
                width: 200.0,
                height: 300.0,
                quantity: 8,
                can_rotate: Some(true),
                priority: Some(2),
                name: Some("Полка".to_string()),
            },
        ],
        strategy: None, // будем менять
        config: None,
    };

    let strategies = vec!["sequential", "parallel", "batch", "auto"];

    for strategy in strategies {
        let mut request = base_request.clone();
        request.strategy = Some(strategy.to_string());

        println!("Стратегия: {}", strategy);
        let result = service.optimize(request).await;
        
        if result.success {
            println!("  ✅ Успех: {} листов, {:.1}% эффективность, {} мс", 
                     result.sheets_count, result.utilization * 100.0, result.execution_time_ms);
        } else {
            println!("  ❌ Ошибка: {}", result.error.unwrap_or_default());
        }
        println!();
    }

    Ok(())
}

async fn demo_additional_features(service: &CuttingService) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Дополнительные возможности ===\n");

    // Быстрая оценка
    println!("1. Быстрая оценка:");
    let estimate = service.estimate(
        MaterialRequest {
            width: 1000.0,
            height: 2000.0,
            cost_per_area: None,
        },
        vec![
            PartRequest {
                width: 300.0,
                height: 400.0,
                quantity: 8,
                can_rotate: None,
                priority: None,
                name: None,
            },
        ],
    ).await?;

    println!("  Оценка: ~{} листов, ~{:.1}% эффективность (достоверность: {:.1}%)",
             estimate.estimated_sheets, estimate.estimated_efficiency * 100.0, estimate.confidence * 100.0);

    // Сравнение алгоритмов
    println!("\n2. Сравнение алгоритмов:");
    let comparisons = service.compare_algorithms(
        MaterialRequest {
            width: 1000.0,
            height: 1000.0,
            cost_per_area: None,
        },
        vec![
            PartRequest {
                width: 200.0,
                height: 300.0,
                quantity: 3,
                can_rotate: None,
                priority: None,
                name: None,
            },
        ],
    ).await?;

    for comparison in comparisons {
        println!("  {}: {} мс, {:.1}% эффективность", 
                 comparison.algorithm_name, comparison.execution_time_ms, comparison.utilization * 100.0);
    }

    // Возможности сервиса
    println!("\n3. Возможности сервиса:");
    let capabilities = service.get_capabilities().await;
    println!("  {}", serde_json::to_string_pretty(&capabilities)?);

    Ok(())
}

#[derive(Clone)]
struct OptimizeRequest {
    material: MaterialRequest,
    parts: Vec<PartRequest>,
    strategy: Option<String>,
    config: Option<ConfigRequest>,
}

#[derive(Clone)]
struct MaterialRequest {
    width: f64,
    height: f64,
    cost_per_area: Option<f64>,
}

#[derive(Clone)]
struct PartRequest {
    width: f64,
    height: f64,
    quantity: usize,
    can_rotate: Option<bool>,
    priority: Option<i32>,
    name: Option<String>,
}

#[derive(Clone)]
struct ConfigRequest {
    cutting_gap: Option<f64>,
    min_waste_size: Option<f64>,
    timeout_seconds: Option<u64>,
    batch_size: Option<usize>,
}