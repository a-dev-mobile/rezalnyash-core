// permutation_generator.rs - обновленная версия
use std::collections::HashMap;

use crate::features::input::models::{panel_group::PanelGroup, panel_instance::PanelInstance};

pub struct PermutationGenerator;

impl PermutationGenerator {
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

    /// Генерирует перестановки групп панелей
    pub fn generate_permutations(groups: Vec<PanelGroup>) -> Vec<Vec<PanelInstance>> {
        if groups.is_empty() {
            return vec![vec![]];
        }

        // Для начала создаем базовые стратегии размещения
        let mut permutations = Vec::new();

        // Стратегия 1: Сортировка по убыванию площади
        permutations.push(Self::create_permutation_by_area(&groups, true));

        // Стратегия 2: Сортировка по возрастанию площади
        permutations.push(Self::create_permutation_by_area(&groups, false));

        // Стратегия 3: Сортировка по убыванию ширины
        permutations.push(Self::create_permutation_by_width(&groups, true));

        // Стратегия 4: Сортировка по убыванию высоты
        permutations.push(Self::create_permutation_by_height(&groups, true));

        // Стратегия 5: Смешанная стратегия - крупные сначала, потом мелкие
        permutations.push(Self::create_mixed_permutation(&groups));

        // Убираем дубликаты
        // permutations.dedup();

        permutations
    }

    /// Создает перестановку на основе площади
    fn create_permutation_by_area(groups: &[PanelGroup], descending: bool) -> Vec<PanelInstance> {
        let mut sorted_groups = groups.to_vec();
        sorted_groups.sort_by(|a, b| {
            let area_a = a.width * a.height;
            let area_b = b.width * b.height;
            if descending {
                area_b.cmp(&area_a)
            } else {
                area_a.cmp(&area_b)
            }
        });

        Self::flatten_groups(&sorted_groups)
    }

    /// Создает перестановку на основе ширины
    fn create_permutation_by_width(groups: &[PanelGroup], descending: bool) -> Vec<PanelInstance> {
        let mut sorted_groups = groups.to_vec();
        sorted_groups.sort_by(|a, b| {
            if descending {
                a.width.cmp(&b.width).reverse()
            } else {
                a.width.cmp(&b.width)
            }
        });

        Self::flatten_groups(&sorted_groups)
    }

    /// Создает перестановку на основе высоты
    fn create_permutation_by_height(groups: &[PanelGroup], descending: bool) -> Vec<PanelInstance> {
        let mut sorted_groups = groups.to_vec();
        sorted_groups.sort_by(|a, b| {
            if descending {
                a.height.cmp(&b.height).reverse()
            } else {
                a.height.cmp(&b.height)
            }
        });

        Self::flatten_groups(&sorted_groups)
    }

    /// Создает смешанную перестановку
    fn create_mixed_permutation(groups: &[PanelGroup]) -> Vec<PanelInstance> {
        let mut result = Vec::new();
        let mut remaining_groups = groups.to_vec();

        // Сначала берем по одному экземпляру из каждой группы (крупные сначала)
        remaining_groups.sort_by(|a, b| {
            let area_a = a.width * a.height;
            let area_b = b.width * b.height;
            area_b.cmp(&area_a)
        });

        // Добавляем по одному представителю от каждой группы
        for group in &remaining_groups {
            if let Some(instance) = group.get_representative() {
                result.push(instance.clone());
            }
        }

        // Затем добавляем остальные экземпляры
        for group in &remaining_groups {
            for instance in group.get_all_instances().iter().skip(1) {
                result.push(instance.clone());
            }
        }

        result
    }

    /// Разворачивает группы в плоский список панелей
    fn flatten_groups(groups: &[PanelGroup]) -> Vec<PanelInstance> {
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
