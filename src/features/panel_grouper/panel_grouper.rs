// features/panel_grouper.rs
use crate::features::input::models::{panel_group::PanelGroup, panel_instance::PanelInstance};

use std::collections::HashMap;

/// Отвечает за группировку панелей по размерам
pub struct PanelGrouper;

impl PanelGrouper {
    /// Группирует панели по уникальным размерам (независимо от ориентации)
    pub fn group_panels(panels: &[PanelInstance]) -> Vec<PanelGroup> {
        let mut size_groups: HashMap<String, Vec<PanelInstance>> = HashMap::new();

        // Группируем панели по нормализованным размерам
        for panel in panels {
            let normalized_key = Self::get_normalized_size_key(panel);
            size_groups
                .entry(normalized_key)
                .or_insert_with(Vec::new)
                .push(panel.clone());
        }

        // Создаем группы из сгруппированных панелей
        let mut result_groups = Vec::new();
        
        for (size_key, instances) in size_groups {
            // Разделяем на повернутые и обычные
            let normal_instances: Vec<_> = instances.iter().filter(|p| !p.is_rotated).cloned().collect();
            let rotated_instances: Vec<_> = instances.iter().filter(|p| p.is_rotated).cloned().collect();

            // Создаем группу для обычных панелей
            if !normal_instances.is_empty() {
                let first = &normal_instances[0];
                let mut group = PanelGroup::new(first.width, first.height, false);
                for instance in normal_instances {
                    group.add_instance(instance);
                }
                result_groups.push(group);
            }

            // Создаем группу для повернутых панелей
            if !rotated_instances.is_empty() {
                let first = &rotated_instances[0];
                let mut group = PanelGroup::new(first.width, first.height, true);
                for instance in rotated_instances {
                    group.add_instance(instance);
                }
                result_groups.push(group);
            }
        }

        // Сортируем группы по убыванию площади
        result_groups.sort_by(|a, b| {
            let area_a = a.width * a.height;
            let area_b = b.width * b.height;
            area_b.cmp(&area_a)
        });

        result_groups
    }

    /// Получить нормализованный ключ размера (больший размер всегда первый)
    fn get_normalized_size_key(panel: &PanelInstance) -> String {
        let (w, h) = panel.effective_dimensions();
        let (max_dim, min_dim) = if w >= h { (w, h) } else { (h, w) };
        format!("{}x{}", max_dim, min_dim)
    }

    /// Выводит статистику группировки в желаемом формате
    pub fn print_grouping_stats(groups: &[PanelGroup]) {
        println!("=== Статистика группировки ===");
        
        let mut panel_counter = 0;
        let mut unique_id_counter = 1;
        let mut last_size_key = String::new();
        
        for group in groups {
            let current_size_key = Self::get_normalized_size_key(
                group.get_representative().unwrap()
            );
            
            // Если размер изменился, увеличиваем счетчик уникальных размеров
            if current_size_key != last_size_key {
                if !last_size_key.is_empty() {
                    unique_id_counter += 1;
                }
                last_size_key = current_size_key;
            }
            
            // Выводим каждую панель в группе
            for (i, _) in group.instances.iter().enumerate() {
                println!(
                    "Группа {}: id={}, group={}[{}x{}], group={}",
                    panel_counter,
                    unique_id_counter,
                    panel_counter,
                    group.width,
                    group.height,
                    panel_counter
                );
                panel_counter += 1;
            }
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
}