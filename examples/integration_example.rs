// //! Пример интеграции библиотеки в веб-сервис или приложение

// use rezalnyas_core::{Material, CuttingRequest, CuttingOptimizer, OptimizationConfig, CuttingResult};
// use serde::{Deserialize, Serialize};
// use std::collections::HashMap;


fn main() {}





// /// Сервис для управления оптимизацией раскроя
// pub struct CuttingService {
//     optimizer: CuttingOptimizer,
//     materials_catalog: HashMap<String, Material>,
// }

// /// Запрос от клиента на оптимизацию
// #[derive(Debug, Serialize, Deserialize)]
// pub struct OptimizationRequest {
//     pub material_id: String,
//     pub parts: Vec<PartRequest>,
//     pub config: Option<ServiceConfig>,
// }

// /// Запрос на деталь от клиента
// #[derive(Debug, Serialize, Deserialize)]
// pub struct PartRequest {
//     pub name: String,
//     pub width: f64,
//     pub height: f64,
//     pub quantity: usize,
//     pub can_rotate: bool,
//     pub priority: i32,
// }

// /// Конфигурация сервиса
// #[derive(Debug, Serialize, Deserialize)]
// pub struct ServiceConfig {
//     pub cutting_gap: f64,
//     pub min_waste_size: f64,
//     pub max_execution_time: u64,
// }

// /// Ответ сервиса
// #[derive(Debug, Serialize, Deserialize)]
// pub struct OptimizationResponse {
//     pub success: bool,
//     pub message: String,
//     pub result: Option<CuttingResult>,
//     pub recommendations: Vec<String>,
// }

// impl CuttingService {
//     /// Создает новый сервис с каталогом материалов
//     pub fn new() -> Self {
//         let mut materials_catalog = HashMap::new();
        
//         // Добавляем стандартные материалы
//         materials_catalog.insert(
//             "dsp_1220x2440".to_string(),
//             Material::with_cost(1220.0, 2440.0, 0.08).unwrap()
//         );
//         materials_catalog.insert(
//             "dsp_1830x2440".to_string(),
//             Material::with_cost(1830.0, 2440.0, 0.08).unwrap()
//         );
//         materials_catalog.insert(
//             "plywood_1525x1525".to_string(),
//             Material::with_cost(1525.0, 1525.0, 0.12).unwrap()
//         );

//         Self {
//             optimizer: CuttingOptimizer::new(),
//             materials_catalog,
//         }
//     }

//     /// Добавляет новый материал в каталог
//     pub fn add_material(&mut self, id: String, material: Material) {
//         self.materials_catalog.insert(id, material);
//     }

//     /// Основной метод для обработки запросов на оптимизацию
//     pub fn process_optimization(&self, request: OptimizationRequest) -> OptimizationResponse {
//         // Проверяем наличие материала
//         let material = match self.materials_catalog.get(&request.material_id) {
//             Some(m) => m,
//             None => return OptimizationResponse {
//                 success: false,
//                 message: format!("Material '{}' not found", request.material_id),
//                 result: None,
//                 recommendations: vec![
//                     "Check available materials list".to_string(),
//                     "Add custom material to catalog".to_string(),
//                 ],
//             },
//         };

//         // Конвертируем запросы в внутренний формат
//         let cutting_requests: Vec<CuttingRequest> = request.parts
//             .into_iter()
//             .map(|part| {
//                 let mut req = CuttingRequest::with_options(
//                     part.width,
//                     part.height,
//                     part.quantity,
//                     part.can_rotate,
//                     part.priority,
//                 );
//                 req.id = Some(part.name);
//                 req
//             })
//             .collect();

//         // Применяем конфигурацию если есть
//         let optimizer = if let Some(config) = request.config {
//             let opt_config = OptimizationConfig {
//                 max_threads: Some(4),
//                 cutting_gap: config.cutting_gap,
//                 min_waste_size: config.min_waste_size,
//                 timeout_seconds: Some(config.max_execution_time),
//             };
//             CuttingOptimizer::with_config(opt_config)
//         } else {
//             self.optimizer.clone()
//         };

//         // Выполняем оптимизацию
//         match optimizer.optimize(material, &cutting_requests) {
//             Ok(result) => {
//                 let recommendations = self.generate_recommendations(&result, material);
                
//                 OptimizationResponse {
//                     success: true,
//                     message: "Optimization completed successfully".to_string(),
//                     result: Some(result),
//                     recommendations,
//                 }
//             }
//             Err(e) => OptimizationResponse {
//                 success: false,
//                 message: format!("Optimization failed: {}", e),
//                 result: None,
//                 recommendations: vec![
//                     "Check part dimensions".to_string(),
//                     "Verify material size".to_string(),
//                     "Consider using smaller parts".to_string(),
//                 ],
//             },
//         }
//     }

//     /// Получает список доступных материалов
//     pub fn get_available_materials(&self) -> Vec<(String, &Material)> {
//         self.materials_catalog
//             .iter()
//             .map(|(id, material)| (id.clone(), material))
//             .collect()
//     }

//     /// Валидирует запрос перед обработкой
//     pub fn validate_request(&self, request: &OptimizationRequest) -> Result<(), String> {
//         // Проверка материала
//         if !self.materials_catalog.contains_key(&request.material_id) {
//             return Err(format!("Unknown material: {}", request.material_id));
//         }

//         // Проверка деталей
//         if request.parts.is_empty() {
//             return Err("No parts specified".to_string());
//         }

//         for part in &request.parts {
//             if part.width <= 0.0 || part.height <= 0.0 {
//                 return Err(format!("Invalid dimensions for part '{}'", part.name));
//             }
//             if part.quantity == 0 {
//                 return Err(format!("Zero quantity for part '{}'", part.name));
//             }
//         }

//         Ok(())
//     }

//     /// Генерирует рекомендации по результатам оптимизации
//     fn generate_recommendations(
//         &self,
//         result: &CuttingResult,
//         material: &Material,
//     ) -> Vec<String> {
//         let mut recommendations = Vec::new();

//         // Анализ использования материала
//         if result.total_utilization < 0.7 {
//             recommendations.push(
//                 "Low material utilization. Consider optimizing part sizes or using smaller sheets"
//                     .to_string(),
//             );
//         }

//         if result.total_utilization > 0.95 {
//             recommendations.push(
//                 "Excellent material utilization achieved!".to_string(),
//             );
//         }

//         // Анализ количества листов
//         if result.layouts.len() > 5 {
//             recommendations.push(
//                 "Large number of sheets required. Consider batch production".to_string(),
//             );
//         }

//         // Анализ неразмещенных деталей
//         if result.unplaced_parts > 0 {
//             recommendations.push(format!(
//                 "{} parts could not be placed. Consider using larger sheets or reducing part sizes",
//                 result.unplaced_parts
//             ));
//         }

//         // Анализ стоимости
//         if let Some(cost) = result.total_cost {
//             if cost > 10000.0 {
//                 recommendations.push(
//                     "High material cost. Consider alternative materials or optimization"
//                         .to_string(),
//                 );
//             }
//         }

//         if recommendations.is_empty() {
//             recommendations.push("Optimization results look good!".to_string());
//         }

//         recommendations
//     }

//     /// Экспортирует результат в различные форматы
//     pub fn export_result(&self, result: &CuttingResult, format: &str) -> Result<String, String> {
//         match format.to_lowercase().as_str() {
//             "json" => serde_json::to_string_pretty(result)
//                 .map_err(|e| format!("JSON export error: {}", e)),
            
//             "csv" => {
//                 let mut csv_data = String::new();
//                 csv_data.push_str("Sheet,Part,Width,Height,X,Y,Rotated\n");
                
//                 for (sheet_idx, layout) in result.layouts.iter().enumerate() {
//                     for (part_idx, part) in layout.parts.iter().enumerate() {
//                         csv_data.push_str(&format!(
//                             "{},{},{},{},{},{},{}\n",
//                             sheet_idx + 1,
//                             part_idx + 1,
//                             part.rectangle.width,
//                             part.rectangle.height,
//                             part.rectangle.x,
//                             part.rectangle.y,
//                             part.rotated
//                         ));
//                     }
//                 }
//                 Ok(csv_data)
//             }
            
//             "summary" => {
//                 let mut summary = String::new();
//                 summary.push_str(&format!("CUTTING OPTIMIZATION SUMMARY\n"));
//                 summary.push_str(&format!("============================\n"));
//                 summary.push_str(&format!("Sheets required: {}\n", result.layouts.len()));
//                 summary.push_str(&format!("Material utilization: {:.1}%\n", result.total_utilization * 100.0));
//                 summary.push_str(&format!("Waste area: {:.0} mm²\n", result.total_waste_area));
//                 summary.push_str(&format!("Execution time: {} ms\n", result.execution_time_ms));
                
//                 if let Some(cost) = result.total_cost {
//                     summary.push_str(&format!("Total cost: {:.2} RUB\n", cost));
//                 }
                
//                 Ok(summary)
//             }
            
//             _ => Err(format!("Unsupported export format: {}", format)),
//         }
//     }
// }

// /// Пример использования сервиса
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     println!("=== Пример интеграции CuttingService ===\n");

//     // Создаем сервис
//     let service = CuttingService::new();

//     // Показываем доступные материалы
//     println!("Доступные материалы:");
//     for (id, material) in service.get_available_materials() {
//         println!("  {}: {}x{} мм", id, material.width, material.height);
//     }
//     println!();

//     // Создаем запрос на оптимизацию
//     let request = OptimizationRequest {
//         material_id: "dsp_1220x2440".to_string(),
//         parts: vec![
//             PartRequest {
//                 name: "Столешница".to_string(),
//                 width: 600.0,
//                 height: 900.0,
//                 quantity: 2,
//                 can_rotate: false,
//                 priority: 10,
//             },
//             PartRequest {
//                 name: "Боковина".to_string(),
//                 width: 400.0,
//                 height: 800.0,
//                 quantity: 4,
//                 can_rotate: true,
//                 priority: 5,
//             },
//             PartRequest {
//                 name: "Полка".to_string(),
//                 width: 350.0,
//                 height: 400.0,
//                 quantity: 3,
//                 can_rotate: true,
//                 priority: 3,
//             },
//         ],
//         config: Some(ServiceConfig {
//             cutting_gap: 3.0,
//             min_waste_size: 100.0,
//             max_execution_time: 30,
//         }),
//     };

//     // Валидируем зап