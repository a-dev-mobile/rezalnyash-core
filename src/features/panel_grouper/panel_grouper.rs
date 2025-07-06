// features/panel_grouper.rs
use crate::features::input::models::{panel_group::PanelGroup, panel_instance::PanelInstance};

use std::collections::HashMap;

/// Отвечает за группировку панелей по размерам и ориентации
pub struct PanelGrouper;

impl PanelGrouper {
    /// Группирует панели по размерам и ориентации
    pub fn group_panels(panels: &[PanelInstance]) -> Vec<PanelGroup> {
        let mut groups: HashMap<String, PanelGroup> = HashMap::new();

        // Группируем панели по эффективным размерам и ориентации
        for panel in panels {
            let (width, height) = panel.effective_dimensions();
            let group_key = if panel.is_rotated {
                format!("{}x{}_R", width, height)
            } else {
                format!("{}x{}_N", width, height)
            };

            groups
                .entry(group_key.clone())
                .or_insert_with(|| PanelGroup::new(width, height, panel.is_rotated))
                .add_instance(panel.clone());
        }

        // Сортируем группы по убыванию площади (более крупные сначала)
        let mut sorted_groups: Vec<PanelGroup> = groups.into_values().collect();
        sorted_groups.sort_by(|a, b| {
            let area_a = a.width * a.height;
            let area_b = b.width * b.height;
            area_b.cmp(&area_a) // Убывание по площади
        });

        sorted_groups
    }

    /// Разворачивает группы в плоский список панелей
    pub fn flatten_groups(groups: &[PanelGroup]) -> Vec<PanelInstance> {
        let mut result = Vec::new();
        for group in groups {
            result.extend(group.get_all_instances().iter().cloned());
        }
        result
    }

    /// Выводит статистику группировки
    pub fn print_grouping_stats(groups: &[PanelGroup]) {
        println!("=== Статистика группировки ===");
        println!("Всего групп: {}", groups.len());

        let total_instances: usize = groups.iter().map(|g| g.count).sum();
        println!("Всего экземпляров: {}", total_instances);

        for (i, group) in groups.iter().enumerate() {
            println!(
                "Группа {}: {}x{} {} ({}шт)",
                i + 1,
                group.width,
                group.height,
                if group.is_rotated {
                    "повернуто"
                } else {
                    "обычно"
                },
                group.count
            );
        }
    }

}

