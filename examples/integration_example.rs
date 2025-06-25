//! Пример интеграции библиотеки в веб-сервис или приложение

use rezalnyas_core::{
    CuttingOptimizer, CuttingRequest, CuttingResult, Material, OptimizationConfig,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Сервис для управления оптимизацией раскроя
pub struct CuttingService {
    optimizer: CuttingOptimizer,
    materials_catalog: HashMap<String, Material>,
}

/// Запрос от клиента на оптимизацию
#[derive(Debug, Serialize, Deserialize)]
pub struct OptimizationRequest {
    pub material_id: String,
    pub parts: Vec<PartRequest>,
    pub config: Option<ServiceConfig>,
}

/// Запрос на деталь от клиента
#[derive(Debug, Serialize, Deserialize)]
pub struct PartRequest {
    pub name: String,
    pub width: f64,
    pub height: f64,
    pub quantity: usize,
    pub can_rotate: bool,
    pub priority: i32,
}

/// Конфигурация сервиса
#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub cutting_gap: f64,
    pub min_waste_size: f64,
    pub max_execution_time: u64,
}

/// Ответ сервиса
#[derive(Debug, Serialize, Deserialize)]
pub struct OptimizationResponse {
    pub success: bool,
    pub message: String,
    pub result: Option<CuttingResult>,
    pub recommendations: Vec<String>,
}

impl CuttingService {
    /// Создает новый сервис с каталогом материалов
    pub fn new() -> Self {
        let mut materials_catalog = HashMap::new();

        // Добавляем стандартные материалы
        materials_catalog.insert(
            "dsp_1220x2440".to_string(),
            Material::with_cost(1220.0, 2440.0, 0.08).unwrap(),
        );
        materials_catalog.insert(
            "dsp_1830x2440".to_string(),
            Material::with_cost(1830.0, 2440.0, 0.08).unwrap(),
        );
        materials_catalog.insert(
            "plywood_1525x1525".to_string(),
            Material::with_cost(1525.0, 1525.0, 0.12).unwrap(),
        );

        Self {
            optimizer: CuttingOptimizer::new(),
            materials_catalog,
        }
    }

    /// Добавляет новый материал в каталог
    pub fn add_material(&mut self, id: String, material: Material) {
        self.materials_catalog.insert(id, material);
    }

    /// Основной метод для обработки запросов на оптимизацию
    pub fn process_optimization(&self, request: OptimizationRequest) -> OptimizationResponse {
        // Проверяем наличие материала
        let material = match self.materials_catalog.get(&request.material_id) {
            Some(m) => m,
            None => {
                return OptimizationResponse {
                    success: false,
                    message: format!("Material '{}' not found", request.material_id),
                    result: None,
                    recommendations: vec![
                        "Check available materials list".to_string(),
                        "Add custom material to catalog".to_string(),
                    ],
                }
            }
        };

        // Конвертируем запросы в внутренний формат
        let cutting_requests: Vec<CuttingRequest> = request
            .parts
            .into_iter()
            .map(|part| {
                let mut req = CuttingRequest::with_options(
                    part.width,
                    part.height,
                    part.quantity,
                    part.can_rotate,
                    part.priority,
                );
                req.id = Some(part.name);
                req
            })
            .collect();

        // Применяем конфигурацию если есть
        let optimizer = if let Some(config) = request.config {
            let opt_config = OptimizationConfig {
                max_threads: Some(4),
                cutting_gap: config.cutting_gap,
                min_waste_size: config.min_waste_size,
                timeout_seconds: Some(config.max_execution_time),
            };
            CuttingOptimizer::with_config(opt_config)
        } else {
            self.optimizer.clone()
        };

        // Выполняем оптимизацию
        match optimizer.optimize(material, &cutting_requests) {
            Ok(result) => {
                let recommendations = self.generate_recommendations(&result, material);

                OptimizationResponse {
                    success: true,
                    message: "Optimization completed successfully".to_string(),
                    result: Some(result),
                    recommendations,
                }
            }
            Err(e) => OptimizationResponse {
                success: false,
                message: format!("Optimization failed: {}", e),
                result: None,
                recommendations: vec![
                    "Check part dimensions".to_string(),
                    "Verify material size".to_string(),
                    "Consider using smaller parts".to_string(),
                ],
            },
        }
    }

    /// Получает список доступных материалов
    pub fn get_available_materials(&self) -> Vec<(String, &Material)> {
        self.materials_catalog
            .iter()
            .map(|(id, material)| (id.clone(), material))
            .collect()
    }

    /// Валидирует запрос перед обработкой
    pub fn validate_request(&self, request: &OptimizationRequest) -> Result<(), String> {
        // Проверка материала
        if !self.materials_catalog.contains_key(&request.material_id) {
            return Err(format!("Unknown material: {}", request.material_id));
        }

        // Проверка деталей
        if request.parts.is_empty() {
            return Err("No parts specified".to_string());
        }

        for part in &request.parts {
            if part.width <= 0.0 || part.height <= 0.0 {
                return Err(format!("Invalid dimensions for part '{}'", part.name));
            }
            if part.quantity == 0 {
                return Err(format!("Zero quantity for part '{}'", part.name));
            }
        }

        Ok(())
    }

    /// Генерирует рекомендации по результатам оптимизации
    fn generate_recommendations(&self, result: &CuttingResult, material: &Material) -> Vec<String> {
        let mut recommendations = Vec::new();

        // Анализ использования материала
        if result.total_utilization < 0.7 {
            recommendations.push(
                "Low material utilization. Consider optimizing part sizes or using smaller sheets"
                    .to_string(),
            );
        }

        if result.total_utilization > 0.95 {
            recommendations.push("Excellent material utilization achieved!".to_string());
        }

        // Анализ количества листов
        if result.layouts.len() > 5 {
            recommendations
                .push("Large number of sheets required. Consider batch production".to_string());
        }

        // Анализ неразмещенных деталей
        if result.unplaced_parts > 0 {
            recommendations.push(format!(
                "{} parts could not be placed. Consider using larger sheets or reducing part sizes",
                result.unplaced_parts
            ));
        }

        // Анализ стоимости
        if let Some(cost) = result.total_cost {
            if cost > 10000.0 {
                recommendations.push(
                    "High material cost. Consider alternative materials or optimization"
                        .to_string(),
                );
            }
        }

        if recommendations.is_empty() {
            recommendations.push("Optimization results look good!".to_string());
        }

        recommendations
    }

    /// Экспортирует результат в различные форматы
    pub fn export_result(&self, result: &CuttingResult, format: &str) -> Result<String, String> {
        match format.to_lowercase().as_str() {
            "json" => serde_json::to_string_pretty(result)
                .map_err(|e| format!("JSON export error: {}", e)),

            "csv" => {
                let mut csv_data = String::new();
                csv_data.push_str("Sheet,Part,Width,Height,X,Y,Rotated\n");

                for (sheet_idx, layout) in result.layouts.iter().enumerate() {
                    for (part_idx, part) in layout.parts.iter().enumerate() {
                        csv_data.push_str(&format!(
                            "{},{},{},{},{},{},{}\n",
                            sheet_idx + 1,
                            part_idx + 1,
                            part.rectangle.width,
                            part.rectangle.height,
                            part.rectangle.x,
                            part.rectangle.y,
                            part.rotated
                        ));
                    }
                }
                Ok(csv_data)
            }

            "summary" => {
                let mut summary = String::new();
                summary.push_str(&format!("CUTTING OPTIMIZATION SUMMARY\n"));
                summary.push_str(&format!("============================\n"));
                summary.push_str(&format!("Sheets required: {}\n", result.layouts.len()));
                summary.push_str(&format!(
                    "Material utilization: {:.1}%\n",
                    result.total_utilization * 100.0
                ));
                summary.push_str(&format!("Waste area: {:.0} mm²\n", result.total_waste_area));
                summary.push_str(&format!(
                    "Execution time: {} ms\n",
                    result.execution_time_ms
                ));

                if let Some(cost) = result.total_cost {
                    summary.push_str(&format!("Total cost: {:.2} RUB\n", cost));
                }

                Ok(summary)
            }

            _ => Err(format!("Unsupported export format: {}", format)),
        }
    }
}

/// Пример использования сервиса
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Пример интеграции CuttingService ===\n");

    // Создаем сервис
    let service = CuttingService::new();

    // Показываем доступные материалы
    println!("Доступные материалы:");
    for (id, material) in service.get_available_materials() {
        println!("  {}: {}x{} мм", id, material.width, material.height);
    }
    println!();

    // Создаем запрос на оптимизацию
    let request = OptimizationRequest {
        material_id: "dsp_1220x2440".to_string(),
        parts: vec![
            PartRequest {
                name: "Столешница".to_string(),
                width: 600.0,
                height: 900.0,
                quantity: 2,
                can_rotate: false,
                priority: 10,
            },
            PartRequest {
                name: "Боковина".to_string(),
                width: 400.0,
                height: 800.0,
                quantity: 4,
                can_rotate: true,
                priority: 5,
            },
            PartRequest {
                name: "Полка".to_string(),
                width: 350.0,
                height: 400.0,
                quantity: 3,
                can_rotate: true,
                priority: 3,
            },
        ],
        config: Some(ServiceConfig {
            cutting_gap: 3.0,
            min_waste_size: 100.0,
            max_execution_time: 30,
        }),
    };

    // Валидируем запрос
    match service.validate_request(&request) {
        Ok(_) => println!("✓ Запрос валиден"),
        Err(e) => {
            println!("✗ Ошибка валидации: {}", e);
            return Ok(());
        }
    }

    // Обрабатываем запрос
    println!("\nВыполняем оптимизацию...");
    let response = service.process_optimization(request);

    // Выводим результат
    if response.success {
        println!("✓ Оптимизация успешно завершена!");

        if let Some(result) = &response.result {
            println!("\nОсновные результаты:");
            println!("  Листов материала: {}", result.layouts.len());
            println!(
                "  Коэффициент использования: {:.1}%",
                result.total_utilization * 100.0
            );
            println!("  Время выполнения: {} мс", result.execution_time_ms);

            if result.unplaced_parts > 0 {
                println!("  ⚠️ Не размещено деталей: {}", result.unplaced_parts);
            }
        }

        println!("\nРекомендации:");
        for recommendation in &response.recommendations {
            println!("  • {}", recommendation);
        }

        // Демонстрация экспорта в разные форматы
        if let Some(result) = &response.result {
            println!("\n=== ЭКСПОРТ РЕЗУЛЬТАТОВ ===");

            // Экспорт в JSON
            match service.export_result(result, "json") {
                Ok(json_data) => {
                    println!("✓ JSON экспорт выполнен ({} символов)", json_data.len());
                    // В реальном приложении здесь можно сохранить в файл
                }
                Err(e) => println!("✗ Ошибка JSON экспорта: {}", e),
            }

            // Экспорт в CSV
            match service.export_result(result, "csv") {
                Ok(csv_data) => {
                    println!("✓ CSV экспорт выполнен");
                    println!("Первые строки CSV:");
                    for line in csv_data.lines().take(5) {
                        println!("  {}", line);
                    }
                    if csv_data.lines().count() > 5 {
                        println!("  ... ({} строк всего)", csv_data.lines().count());
                    }
                }
                Err(e) => println!("✗ Ошибка CSV экспорта: {}", e),
            }

            // Экспорт краткого отчета
            match service.export_result(result, "summary") {
                Ok(summary) => {
                    println!("\n✓ Краткий отчет:");
                    println!("{}", summary);
                }
                Err(e) => println!("✗ Ошибка создания отчета: {}", e),
            }
        }
    } else {
        println!("✗ Ошибка оптимизации: {}", response.message);
        println!("\nРекомендации по устранению:");
        for recommendation in &response.recommendations {
            println!("  • {}", recommendation);
        }
    }

    Ok(())
}

/// Пример веб-обработчика (псевдокод для веб-фреймворка)
#[allow(dead_code)]
mod web_handler_example {
    use super::*;

    // Псевдокод для Actix-web или аналогичного фреймворка
    pub async fn handle_optimization_request(
        service: &CuttingService,
        request_json: String,
    ) -> Result<String, String> {
        // Парсим JSON запрос
        let request: OptimizationRequest =
            serde_json::from_str(&request_json).map_err(|e| format!("Invalid JSON: {}", e))?;

        // Валидируем запрос
        service.validate_request(&request)?;

        // Обрабатываем
        let response = service.process_optimization(request);

        // Возвращаем JSON ответ
        serde_json::to_string(&response).map_err(|e| format!("Response serialization error: {}", e))
    }

    // Обработчик для получения списка материалов
    pub fn handle_materials_list(service: &CuttingService) -> Result<String, String> {
        let materials: Vec<_> = service
            .get_available_materials()
            .into_iter()
            .map(|(id, material)| {
                serde_json::json!({
                    "id": id,
                    "width": material.width,
                    "height": material.height,
                    "area": material.area(),
                    "cost_per_area": material.cost_per_area
                })
            })
            .collect();

        serde_json::to_string(&materials).map_err(|e| format!("Serialization error: {}", e))
    }
}

/// Пример интеграции с базой данных
#[allow(dead_code)]
mod database_integration {
    use super::*;
    use std::collections::HashMap;

    pub struct DatabaseService {
        // В реальном приложении здесь был бы connection pool
        cutting_service: CuttingService,
        optimization_history: HashMap<String, OptimizationResponse>,
    }

    impl DatabaseService {
        pub fn new() -> Self {
            Self {
                cutting_service: CuttingService::new(),
                optimization_history: HashMap::new(),
            }
        }

        /// Сохраняет результат оптимизации в "базу данных"
        pub fn save_optimization_result(
            &mut self,
            user_id: &str,
            project_name: &str,
            response: OptimizationResponse,
        ) -> String {
            let key = format!("{}_{}", user_id, project_name);
            self.optimization_history.insert(key.clone(), response);
            key
        }

        /// Загружает результат из "базы данных"
        pub fn load_optimization_result(&self, key: &str) -> Option<&OptimizationResponse> {
            self.optimization_history.get(key)
        }

        /// Получает историю оптимизаций пользователя
        pub fn get_user_history(&self, user_id: &str) -> Vec<(&String, &OptimizationResponse)> {
            self.optimization_history
                .iter()
                .filter(|(key, _)| key.starts_with(user_id))
                .collect()
        }

        /// Обрабатывает запрос с сохранением в историю
        pub fn process_and_save(
            &mut self,
            user_id: &str,
            project_name: &str,
            request: OptimizationRequest,
        ) -> String {
            let response = self.cutting_service.process_optimization(request);
            self.save_optimization_result(user_id, project_name, response)
        }
    }
}

/// Пример создания пользовательского алгоритма
#[allow(dead_code)]
mod custom_algorithm_example {
    use super::*;
    use rezalnyas_core::{types::*, CuttingAlgorithm};

    /// Пользовательский алгоритм "Спиральное размещение"
    pub struct SpiralPlacementAlgorithm {
        cutting_gap: f64,
    }

    impl SpiralPlacementAlgorithm {
        pub fn new() -> Self {
            Self { cutting_gap: 2.0 }
        }
    }

    impl CuttingAlgorithm for SpiralPlacementAlgorithm {
        fn optimize(
            &self,
            material: &Material,
            requests: &[CuttingRequest],
        ) -> rezalnyas_core::Result<CuttingResult> {
            let mut result = CuttingResult::new();
            let mut layout = CuttingLayout::new(material.clone());

            // Простая реализация спирального размещения
            let center_x = material.width / 2.0;
            let center_y = material.height / 2.0;
            let mut radius = 0.0;
            let mut angle = 0.0;

            // Развернутый список деталей
            let mut parts_to_place = Vec::new();
            for request in requests {
                for _ in 0..request.quantity {
                    parts_to_place.push(request.clone());
                }
            }

            for part_request in parts_to_place {
                let mut placed = false;
                let mut attempts = 0;

                while !placed && attempts < 360 {
                    let x = center_x + radius * angle.cos() - part_request.width / 2.0;
                    let y = center_y + radius * angle.sin() - part_request.height / 2.0;

                    let rect = Rectangle::new(x, y, part_request.width, part_request.height);

                    if layout.can_place_part(&rect) {
                        let placed_part =
                            PlacedPart::new(x, y, part_request.width, part_request.height, false);
                        layout.add_part(placed_part);
                        placed = true;
                    } else {
                        angle += 10.0_f64.to_radians();
                        if attempts % 36 == 0 {
                            radius += 50.0; // Увеличиваем радиус каждые 360 градусов
                        }
                        attempts += 1;
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
            "Spiral Placement Algorithm"
        }
    }

    /// Демонстрация использования пользовательского алгоритма
    pub fn demo_custom_algorithm() -> Result<(), Box<dyn std::error::Error>> {
        println!("\n=== Демонстрация пользовательского алгоритма ===");

        let mut service = CuttingService::new();

        // Создаем оптимизатор с нашим алгоритмом
        let mut optimizer = CuttingOptimizer::new();
        optimizer.add_algorithm(Box::new(SpiralPlacementAlgorithm::new()));

        let material = Material::new(1000.0, 1000.0)?;
        let requests = vec![
            CuttingRequest::new(100.0, 100.0, 5),
            CuttingRequest::new(150.0, 150.0, 3),
        ];

        let result = optimizer.optimize(&material, &requests)?;

        println!("Результат пользовательского алгоритма:");
        println!(
            "  Использование материала: {:.1}%",
            result.total_utilization * 100.0
        );
        println!(
            "  Размещено деталей: {}",
            result.layouts.iter().map(|l| l.parts.len()).sum::<usize>()
        );
        println!("  Не размещено: {}", result.unplaced_parts);

        Ok(())
    }
}

/// Пример многопоточной обработки больших задач
#[allow(dead_code)]
mod batch_processing_example {
    use super::*;
    use rezalnyas_core::parallel::{BatchProcessor, OptimizationWorker};
    use std::sync::Arc;

    pub fn demo_batch_processing() -> Result<(), Box<dyn std::error::Error>> {
        println!("\n=== Демонстрация пакетной обработки ===");

        // Создаем большое количество деталей для демонстрации
        let mut large_request_set = Vec::new();
        for i in 1..=20 {
            large_request_set.push(CuttingRequest::new(
                100.0 + i as f64 * 10.0,
                150.0 + i as f64 * 5.0,
                3,
            ));
        }

        let material = Material::new(2000.0, 3000.0)?;
        let batch_processor = BatchProcessor::new(15); // Пакеты по 15 деталей

        println!(
            "Обрабатываем {} типов деталей пакетами...",
            large_request_set.len()
        );

        // В реальном приложении здесь использовались бы настоящие алгоритмы
        let algorithm = Box::new(rezalnyas_core::algorithms::BestFitAlgorithm::new());

        // Демонстрация разбиения на пакеты
        let batches = batch_processor.split_into_batches(&large_request_set);
        println!("Разбито на {} пакетов", batches.len());

        for (i, batch) in batches.iter().enumerate() {
            let total_parts: usize = batch.iter().map(|r| r.quantity).sum();
            println!(
                "  Пакет {}: {} типов деталей, {} штук всего",
                i + 1,
                batch.len(),
                total_parts
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_cutting_service_creation() {
        let service = CuttingService::new();
        assert!(!service.get_available_materials().is_empty());
    }

    #[test]
    fn test_request_validation() {
        let service = CuttingService::new();

        let valid_request = OptimizationRequest {
            material_id: "dsp_1220x2440".to_string(),
            parts: vec![PartRequest {
                name: "Test".to_string(),
                width: 100.0,
                height: 200.0,
                quantity: 1,
                can_rotate: true,
                priority: 0,
            }],
            config: None,
        };

        assert!(service.validate_request(&valid_request).is_ok());

        let invalid_request = OptimizationRequest {
            material_id: "nonexistent".to_string(),
            parts: vec![],
            config: None,
        };

        assert!(service.validate_request(&invalid_request).is_err());
    }

    #[test]
    fn test_optimization_processing() {
        let service = CuttingService::new();

        let request = OptimizationRequest {
            material_id: "dsp_1220x2440".to_string(),
            parts: vec![PartRequest {
                name: "TestPart".to_string(),
                width: 300.0,
                height: 400.0,
                quantity: 2,
                can_rotate: true,
                priority: 1,
            }],
            config: None,
        };

        let response = service.process_optimization(request);
        assert!(response.success);
        assert!(response.result.is_some());
    }

    #[test]
    fn test_export_formats() {
        let service = CuttingService::new();
        let result = CuttingResult::new(); // Пустой результат для тестирования

        // Тестируем различные форматы экспорта
        assert!(service.export_result(&result, "json").is_ok());
        assert!(service.export_result(&result, "csv").is_ok());
        assert!(service.export_result(&result, "summary").is_ok());
        assert!(service.export_result(&result, "unknown").is_err());
    }

    #[test]
    fn test_database_integration() {
        let mut db_service = database_integration::DatabaseService::new();

        let response = OptimizationResponse {
            success: true,
            message: "Test".to_string(),
            result: None,
            recommendations: vec![],
        };

        let key = db_service.save_optimization_result("user1", "project1", response);
        assert!(db_service.load_optimization_result(&key).is_some());

        let history = db_service.get_user_history("user1");
        assert_eq!(history.len(), 1);
    }
}
