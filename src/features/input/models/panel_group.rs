// models/panel_group.rs
use serde::Serialize;
use std::collections::HashMap;
use uuid::Uuid;

use super::panel_instance::PanelInstance;

/// Группа одинаковых панелей для оптимизации перестановок
#[derive(Serialize, Debug, Clone)]
pub struct PanelGroup {
    pub group_id: String,           // Уникальный идентификатор группы
    pub width: u32,                 // Ширина панелей в группе (реальная, не эффективная)
    pub height: u32,                // Высота панелей в группе (реальная, не эффективная)
    pub is_rotated: bool,           // Повернуты ли панели
    pub instances: Vec<PanelInstance>, // Все экземпляры в группе
    pub count: usize,               // Количество панелей в группе
}

impl PanelGroup {
    pub fn new(width: u32, height: u32, is_rotated: bool) -> Self {
        // Используем реальные размеры для group_id
        let group_id = if is_rotated {
            format!("{}x{}_R", width, height) // Реальные размеры панели
        } else {
            format!("{}x{}_N", width, height) // Реальные размеры панели
        };

        Self {
            group_id,
            width,
            height,
            is_rotated,
            instances: Vec::new(),
            count: 0,
        }
    }

    /// Добавить экземпляр в группу
    pub fn add_instance(&mut self, instance: PanelInstance) {
        self.instances.push(instance);
        self.count = self.instances.len();
    }

    /// Проверить, подходит ли экземпляр для этой группы
    pub fn matches(&self, instance: &PanelInstance) -> bool {
        // Сравниваем реальные размеры и ориентацию
        instance.width == self.width && 
        instance.height == self.height && 
        instance.is_rotated == self.is_rotated
    }

    /// Получить один экземпляр из группы (для представления)
    pub fn get_representative(&self) -> Option<&PanelInstance> {
        self.instances.first()
    }

    /// Получить все экземпляры
    pub fn get_all_instances(&self) -> &Vec<PanelInstance> {
        &self.instances
    }

    /// Получить N экземпляров из группы
    pub fn take_instances(&mut self, n: usize) -> Vec<PanelInstance> {
        let take_count = std::cmp::min(n, self.instances.len());
        let mut taken = Vec::new();
        
        for _ in 0..take_count {
            if let Some(instance) = self.instances.pop() {
                taken.push(instance);
            }
        }
        
        self.count = self.instances.len();
        taken
    }

    /// Проверить, пуста ли группа
    pub fn is_empty(&self) -> bool {
        self.instances.is_empty()
    }

    /// Получить эффективные размеры группы (с учетом поворота)
    pub fn effective_dimensions(&self) -> (u32, u32) {
        if self.is_rotated {
            (self.height, self.width)
        } else {
            (self.width, self.height)
        }
    }
}