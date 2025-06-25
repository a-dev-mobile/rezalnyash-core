//! Основные типы данных для работы с раскроем материалов

use serde::{Deserialize, Serialize};
use std::fmt;

/// Ошибки, возникающие при оптимизации раскроя
#[derive(Debug, Clone, PartialEq)]
pub enum OptimizationError {
    /// Неверные размеры материала
    InvalidMaterialSize(String),
    /// Неверные размеры детали
    InvalidPartSize(String),
    /// Деталь не помещается в материал
    PartDoesNotFit(String),
    /// Превышено время выполнения
    TimeoutExceeded,
    /// Ошибка вычислений
    CalculationError(String),
}

impl fmt::Display for OptimizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptimizationError::InvalidMaterialSize(msg) => write!(f, "Invalid material size: {}", msg),
            OptimizationError::InvalidPartSize(msg) => write!(f, "Invalid part size: {}", msg),
            OptimizationError::PartDoesNotFit(msg) => write!(f, "Part does not fit: {}", msg),
            OptimizationError::TimeoutExceeded => write!(f, "Optimization timeout exceeded"),
            OptimizationError::CalculationError(msg) => write!(f, "Calculation error: {}", msg),
        }
    }
}

impl std::error::Error for OptimizationError {}

/// Прямоугольник с позицией и размерами
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rectangle {
    /// X координата левого верхнего угла
    pub x: f64,
    /// Y координата левого верхнего угла  
    pub y: f64,
    /// Ширина прямоугольника
    pub width: f64,
    /// Высота прямоугольника
    pub height: f64,
}

impl Rectangle {
    /// Создает новый прямоугольник
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self { x, y, width, height }
    }

    /// Возвращает площадь прямоугольника
    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    /// Проверяет, пересекается ли с другим прямоугольником
    pub fn intersects(&self, other: &Rectangle) -> bool {
        !(self.x + self.width <= other.x ||
          other.x + other.width <= self.x ||
          self.y + self.height <= other.y ||
          other.y + other.height <= self.y)
    }

    /// Проверяет, содержится ли точка в прямоугольнике
    pub fn contains_point(&self, x: f64, y: f64) -> bool {
        self.x <= x && x <= self.x + self.width &&
        self.y <= y && y <= self.y + self.height
    }
}

/// Материал для раскроя
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Material {
    /// Ширина материала
    pub width: f64,
    /// Высота материала
    pub height: f64,
    /// Стоимость за единицу площади (опционально)
    pub cost_per_area: Option<f64>,
    /// Идентификатор материала
    pub id: Option<String>,
}

impl Material {
    /// Создает новый материал с заданными размерами
    pub fn new(width: f64, height: f64) -> crate::Result<Self> {
        if width <= 0.0 || height <= 0.0 {
            return Err(OptimizationError::InvalidMaterialSize(
                format!("Width and height must be positive, got {}x{}", width, height)
            ));
        }

        Ok(Self {
            width,
            height,
            cost_per_area: None,
            id: None,
        })
    }

    /// Создает материал с указанием стоимости
    pub fn with_cost(width: f64, height: f64, cost_per_area: f64) -> crate::Result<Self> {
        let mut material = Self::new(width, height)?;
        material.cost_per_area = Some(cost_per_area);
        Ok(material)
    }

    /// Возвращает общую площадь материала
    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    /// Возвращает стоимость материала
    pub fn total_cost(&self) -> Option<f64> {
        self.cost_per_area.map(|cost| cost * self.area())
    }
}

/// Запрос на раскрой детали
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CuttingRequest {
    /// Ширина детали
    pub width: f64,
    /// Высота детали
    pub height: f64,
    /// Количество деталей
    pub quantity: usize,
    /// Возможность поворота детали на 90 градусов
    pub can_rotate: bool,
    /// Приоритет детали (больше = выше приоритет)
    pub priority: i32,
    /// Идентификатор детали
    pub id: Option<String>,
}

impl CuttingRequest {
    /// Создает новый запрос на раскрой
    pub fn new(width: f64, height: f64, quantity: usize) -> Self {
        Self {
            width,
            height,
            quantity,
            can_rotate: true,
            priority: 0,
            id: None,
        }
    }

    /// Создает запрос с дополнительными параметрами
    pub fn with_options(
        width: f64,
        height: f64,
        quantity: usize,
        can_rotate: bool,
        priority: i32,
    ) -> Self {
        Self {
            width,
            height,
            quantity,
            can_rotate,
            priority,
            id: None,
        }
    }

    /// Возвращает площадь одной детали
    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    /// Возвращает общую площадь всех деталей
    pub fn total_area(&self) -> f64 {
        self.area() * self.quantity as f64
    }

    /// Проверяет, помещается ли деталь в материал
    pub fn fits_in_material(&self, material: &Material) -> bool {
        (self.width <= material.width && self.height <= material.height) ||
        (self.can_rotate && self.height <= material.width && self.width <= material.height)
    }
}

/// Размещенная деталь в раскрое
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlacedPart {
    /// Прямоугольник, описывающий размещение
    pub rectangle: Rectangle,
    /// Была ли деталь повернута
    pub rotated: bool,
    /// Идентификатор исходного запроса
    pub request_id: Option<String>,
}

impl PlacedPart {
    /// Создает новую размещенную деталь
    pub fn new(x: f64, y: f64, width: f64, height: f64, rotated: bool) -> Self {
        Self {
            rectangle: Rectangle::new(x, y, width, height),
            rotated,
            request_id: None,
        }
    }
}

/// Раскладка на одном листе материала
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuttingLayout {
    /// Используемый материал
    pub material: Material,
    /// Размещенные детали
    pub parts: Vec<PlacedPart>,
    /// Коэффициент использования материала (0.0 - 1.0)
    pub utilization: f64,
    /// Общая площадь отходов
    pub waste_area: f64,
}

impl CuttingLayout {
    /// Создает новую раскладку
    pub fn new(material: Material) -> Self {
        Self {
            material,
            parts: Vec::new(),
            utilization: 0.0,
            waste_area: 0.0,
        }
    }

    /// Добавляет деталь в раскладку
    pub fn add_part(&mut self, part: PlacedPart) {
        self.parts.push(part);
        self.recalculate_metrics();
    }

    /// Пересчитывает метрики использования материала
    pub fn recalculate_metrics(&mut self) {
        let used_area: f64 = self.parts.iter().map(|p| p.rectangle.area()).sum();
        let total_area = self.material.area();
        
        self.utilization = if total_area > 0.0 { used_area / total_area } else { 0.0 };
        self.waste_area = total_area - used_area;
    }

    /// Проверяет, может ли деталь быть размещена без пересечений
    pub fn can_place_part(&self, rectangle: &Rectangle) -> bool {
        // Проверка границ материала
        if rectangle.x < 0.0 || rectangle.y < 0.0 ||
           rectangle.x + rectangle.width > self.material.width ||
           rectangle.y + rectangle.height > self.material.height {
            return false;
        }

        // Проверка пересечений с существующими деталями
        !self.parts.iter().any(|part| part.rectangle.intersects(rectangle))
    }
}

/// Результат оптимизации раскроя
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuttingResult {
    /// Все раскладки
    pub layouts: Vec<CuttingLayout>,
    /// Общий коэффициент использования материала
    pub total_utilization: f64,
    /// Общая площадь отходов
    pub total_waste_area: f64,
    /// Общая стоимость материалов
    pub total_cost: Option<f64>,
    /// Время выполнения оптимизации (миллисекунды)
    pub execution_time_ms: u64,
    /// Количество не размещенных деталей
    pub unplaced_parts: usize,
}

impl CuttingResult {
    /// Создает новый результат
    pub fn new() -> Self {
        Self {
            layouts: Vec::new(),
            total_utilization: 0.0,
            total_waste_area: 0.0,
            total_cost: None,
            execution_time_ms: 0,
            unplaced_parts: 0,
        }
    }

    /// Добавляет раскладку к результату
    pub fn add_layout(&mut self, layout: CuttingLayout) {
        self.layouts.push(layout);
        self.recalculate_totals();
    }

    /// Пересчитывает общие метрики
    pub fn recalculate_totals(&mut self) {
        if self.layouts.is_empty() {
            return;
        }

        let total_area: f64 = self.layouts.iter().map(|l| l.material.area()).sum();
        let total_used_area: f64 = self.layouts.iter()
            .map(|l| l.parts.iter().map(|p| p.rectangle.area()).sum::<f64>())
            .sum();

        self.total_utilization = if total_area > 0.0 { total_used_area / total_area } else { 0.0 };
        self.total_waste_area = total_area - total_used_area;

        // Расчет общей стоимости
        self.total_cost = self.layouts.iter()
            .filter_map(|l| l.material.total_cost())
            .reduce(|acc, cost| acc + cost);
    }
}