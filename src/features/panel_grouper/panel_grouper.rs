// features/panel_grouper.rs
use crate::features::input::models::{panel_group::PanelGroup, panel_instance::PanelInstance};

use std::collections::HashMap;

/// Отвечает за группировку панелей по размерам (с нормализацией)
pub struct PanelGrouper;

impl PanelGrouper {
    /// Группирует панели аналогично Java методу generateGroups
    pub fn group_panels(panels: &[PanelInstance]) -> Vec<PanelGroup> {
        // Подсчет количества каждого уникального размера
        let mut size_counts: HashMap<String, u32> = HashMap::new();
        for panel in panels {
            // Используем нормализованные размеры (больший размер как ширина)
            let (width, height) = Self::normalize_dimensions(panel.width, panel.height);
            let size_key = format!("{}x{}", width, height);
            *size_counts.entry(size_key).or_insert(0) += 1;
        }

        // Определяем максимальное количество панелей в группе
        let max_group_size = (panels.len() / 100).max(1) as u32;

        // Группируем панели с разбиением больших групп
        let mut groups: Vec<PanelGroup> = Vec::new();
        let mut group_counters: HashMap<String, u32> = HashMap::new();
        let mut current_group_id = 0u8;

        for panel in panels {
            let (width, height) = Self::normalize_dimensions(panel.width, panel.height);
            let size_key = format!("{}x{}", width, height);
            let total_count = size_counts[&size_key];
            
            // Получаем текущий счетчик для этого размера
            let current_count = group_counters.entry(size_key.clone()).or_insert(0);
            *current_count += 1;

            // Проверяем, нужно ли создать новую группу
            let should_split = total_count > max_group_size && 
                             *current_count > total_count / 4;

            // Создаем новую группу или добавляем в последнюю
            if groups.is_empty() || 
               groups.last().unwrap().width != width || 
               groups.last().unwrap().height != height || 
               should_split {
                
                current_group_id += 1;
                let mut new_group = PanelGroup::new(width, height, current_group_id);
                new_group.add_instance(panel.clone());
                groups.push(new_group);
                
                if should_split {
                    *current_count = 1; // Сброс счетчика для новой группы
                }
            } else {
                // Добавляем в последнюю группу того же размера
                if let Some(last_group) = groups.last_mut() {
                    last_group.add_instance(panel.clone());
                }
            }
        }

        // Сортируем группы по убыванию площади
        groups.sort_by(|a, b| {
            let area_a = a.width * a.height;
            let area_b = b.width * b.height;
            area_b.cmp(&area_a)
        });

        // Переназначаем ID после сортировки
        for (index, group) in groups.iter_mut().enumerate() {
            group.id = (index + 1) as u8;
        }

        groups
    }

    /// Нормализует размеры - больший размер всегда ширина
    fn normalize_dimensions(width: u32, height: u32) -> (u32, u32) {
        if width >= height {
            (width, height)
        } else {
            (height, width)
        }
    }

    /// Разворачивает группы в плоский список панелей
    pub fn flatten_groups(groups: &[PanelGroup]) -> Vec<PanelInstance> {
        let mut result = Vec::new();
        for group in groups {
            result.extend(group.get_all_instances().iter().cloned());
        }
        result
    }

    /// Выводит статистику группировки в требуемом формате
    pub fn print_grouping_stats(groups: &[PanelGroup]) {
        println!("=== Статистика группировки ===");
        println!("Всего групп: {}", groups.len());

        let total_instances: usize = groups.iter().map(|g| g.count).sum();
        println!("Всего экземпляров: {}", total_instances);

        let mut group_counter = 0u8;
        
        for group in groups {
            for _instance in group.get_all_instances() {
                println!(
                    "Группа {}: id={}, gropup={}[{}x{}], group={}",
                    group_counter,
                    group.id,
                    group_counter,
                    group.width,
                    group.height,
                    group_counter
                );
                group_counter += 1;
            }
        }
    }
}