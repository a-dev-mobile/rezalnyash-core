// models/panel_group.rs
use serde::Serialize;
use std::collections::HashMap;
use uuid::Uuid;

use super::panel_instance::PanelInstance;

/// Группа одинаковых панелей для оптимизации (без учета поворота)
#[derive(Serialize, Debug, Clone)]
pub struct PanelGroup {
    pub id: u8,                     // Уникальный ID размера
    pub group: String,              // Формат gropup=N[WxH]
    pub width: u32,                 // Ширина панелей в группе
    pub height: u32,                // Высота панелей в группе
    pub instances: Vec<PanelInstance>, // Все экземпляры в группе
    pub count: usize,               // Количество панелей в группе
}

impl PanelGroup {
    pub fn new(width: u32, height: u32, id: u8) -> Self {
        Self {
            id,
            group: String::new(), // Будет заполнен позже
            width,
            height,
            instances: Vec::new(),
            count: 0,
        }
    }

    /// Добавить экземпляр в группу
    pub fn add_instance(&mut self, instance: PanelInstance) {
        self.instances.push(instance);
        self.count = self.instances.len();
    }

    /// Проверить, подходит ли экземпляр для этой группы (по физическим размерам)
    pub fn matches(&self, instance: &PanelInstance) -> bool {
        instance.width == self.width && instance.height == self.height
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

    /// Получить размеры группы
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Установить group строку
    pub fn set_group_string(&mut self, group_number: u8) {
        self.group = format!("gropup={}[{}x{}]", group_number, self.width, self.height);
    }
}