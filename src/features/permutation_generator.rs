use std::collections::HashMap;

use crate::features::input::models::panel::Panel;



/// Генератор перестановок панелей
/// TODO: Взять из Arrangement.generatePermutations() и упростить
pub struct PermutationGenerator;

impl PermutationGenerator {
    /// Генерация всех перестановок панелей
    /// TODO: Реализовать по образцу Arrangement.generatePermutations()
    pub fn generate_permutations<T: Clone>(items: Vec<T>) -> Vec<Vec<T>> {
        if items.is_empty() {
            return vec![vec![]];
        }

        // TODO: Рекурсивная генерация перестановок
        // Взять алгоритм из Arrangement.java

        // Пока заглушка - возвращаем исходный порядок
        vec![items]
    }

    /// Группировка одинаковых панелей для оптимизации
    /// Реализовано по образцу generateGroups() из CutListOptimizerServiceImpl.java
    pub fn group_panels(panels: &[Panel]) -> Vec<Panel> {
        // Создаем карту для подсчета количества одинаковых панелей
        let mut panel_counts: HashMap<String, i32> = HashMap::new();

        // Подсчитываем количество каждого типа панели
        for panel in panels {
            let key = format!("{}x{}", panel.width, panel.height);
            *panel_counts.entry(key).or_insert(0) += 1;
        }

        // Проверяем, является ли это одномерной оптимизацией
        let is_one_dimensional = Self::is_one_dimensional_optimization(panels);

        // Определяем максимальный размер группы
        let max_group_size: usize = if is_one_dimensional {
            1 // Для одномерной оптимизации группы не разбиваем
        } else {
            std::cmp::max(panels.len() / 100, 1)
        };

        let mut result = Vec::new();
        let mut current_group_counts: HashMap<String, i32> = HashMap::new();
        let mut group_id = 0;

        for panel in panels {
            let panel_key = format!("{}x{}", panel.width, panel.height);
            let group_key = format!("{}{}", panel_key, group_id);

            // Увеличиваем счетчик для текущей группы
            let current_count = current_group_counts.entry(group_key.clone()).or_insert(0);
            *current_count += 1;

            // Создаем новую панель с group_id в качестве модификатора ID
            let mut grouped_panel = panel.clone();
            // grouped_panel.id = panel.id * 1000 + group_id;
            result.push(grouped_panel);

            // Проверяем, нужно ли создать новую группу
            let total_count = panel_counts.get(&panel_key).unwrap_or(&0);
            if *total_count > max_group_size as i32 && *current_count > total_count / 4 {
                group_id += 1;
            }
        }

        result
    }

    /// Проверка является ли оптимизация одномерной
    fn is_one_dimensional_optimization(panels: &[Panel]) -> bool {
        return false;
        // if panels.is_empty() {
        //     return false;
        // }

    //     let mut common_dimensions = vec![panels[0].width, panels[0].height];

    //     // Проверяем панели
    //     for panel in panels {
    //         // Удаляем размеры, которые не встречаются в текущей панели
    //         common_dimensions.retain(|&dim| dim == panel.width || dim == panel.height);

    //         if common_dimensions.is_empty() {
    //             return false;
    //         }
    //     }

    //     !common_dimensions.is_empty()
    }
}
