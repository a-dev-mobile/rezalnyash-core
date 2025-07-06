// features/permutation_generator.rs
use crate::features::input::models::tile_dimensions::TileDimensions;

/// Отвечает за генерацию перестановок панелей
pub struct PermutationGenerator;

impl PermutationGenerator {
    /// Генерирует все возможные перестановки панелей
    pub fn generate_all_permutations(panels: Vec<TileDimensions>) -> Vec<Vec<TileDimensions>> {
        if panels.len() <= 1 {
            return vec![panels];
        }

        // Ограничиваем количество элементов для перестановок (чтобы не было взрыва памяти)
        let max_items_for_full_permutation = 8;

        if panels.len() <= max_items_for_full_permutation {
            println!("Генерация ВСЕХ перестановок для {} панелей", panels.len());
            Self::generate_full_permutations(panels)
        } else {
            println!(
                "Генерация ограниченных перестановок для {} панелей",
                panels.len()
            );
            Self::generate_limited_permutations(panels)
        }
    }

    /// Генерирует все возможные перестановки (факториал)
    fn generate_full_permutations(mut panels: Vec<TileDimensions>) -> Vec<Vec<TileDimensions>> {
        let mut result = Vec::new();
        Self::permute(&mut panels, 0, &mut result);
        println!("Сгенерировано {} полных перестановок", result.len());
        result
    }

    /// Рекурсивная функция для генерации перестановок
    fn permute(
        panels: &mut Vec<TileDimensions>,
        start: usize,
        result: &mut Vec<Vec<TileDimensions>>,
    ) {
        if start == panels.len() {
            result.push(panels.clone());
            return;
        }

        for i in start..panels.len() {
            panels.swap(start, i);
            Self::permute(panels, start + 1, result);
            panels.swap(start, i); // Возвращаем обратно
        }
    }

    /// Генерирует ограниченный набор перестановок для больших массивов
    fn generate_limited_permutations(panels: Vec<TileDimensions>) -> Vec<Vec<TileDimensions>> {
        let mut permutations = Vec::new();

        // 1. Исходный порядок
        permutations.push(panels.clone());

        // 2. Обратный порядок
        let mut reversed = panels.clone();
        reversed.reverse();
        permutations.push(reversed);

        // 3. Сортировка по площади (убывание)
        let mut by_area_desc = panels.clone();
        by_area_desc.sort_by(|a, b| {
            let area_a = a.width * a.height;
            let area_b = b.width * b.height;
            area_b.cmp(&area_a)
        });
        permutations.push(by_area_desc);

        // 4. Сортировка по площади (возрастание)
        let mut by_area_asc = panels.clone();
        by_area_asc.sort_by(|a, b| {
            let area_a = a.width * a.height;
            let area_b = b.width * b.height;
            area_a.cmp(&area_b)
        });
        permutations.push(by_area_asc);

        // 5. Сортировка по ширине (убывание)
        let mut by_width = panels.clone();
        by_width.sort_by(|a, b| b.width.cmp(&a.width));
        permutations.push(by_width);

        // 6. Сортировка по высоте (убывание)
        let mut by_height = panels.clone();
        by_height.sort_by(|a, b| b.height.cmp(&a.height));
        permutations.push(by_height);

        // 7. Сортировка по ширине (возрастание)
        let mut by_width_asc = panels.clone();
        by_width_asc.sort_by(|a, b| a.width.cmp(&b.width));
        permutations.push(by_width_asc);

        // 8. Сортировка по высоте (возрастание)
        let mut by_height_asc = panels.clone();
        by_height_asc.sort_by(|a, b| a.height.cmp(&b.height));
        permutations.push(by_height_asc);

        // 9-11. Случайные перестановки (несколько вариантов)
        for i in 0..3 {
            let mut random_perm = panels.clone();
            Self::shuffle(&mut random_perm, i);
            permutations.push(random_perm);
        }

        // Убираем дубликаты
        Self::remove_duplicates(&mut permutations);

        println!(
            "Сгенерировано {} ограниченных перестановок",
            permutations.len()
        );
        permutations
    }

    /// Простая функция перемешивания (псевдослучайная)
    fn shuffle(panels: &mut Vec<TileDimensions>, seed: usize) {
        for i in 0..panels.len() {
            let j = (i * 7 + 13 + seed * 31) % panels.len(); // Простая псевдослучайная формула
            panels.swap(i, j);
        }
    }

    /// Удаляет дубликаты перестановок
    fn remove_duplicates(permutations: &mut Vec<Vec<TileDimensions>>) {
        // Простое удаление дубликатов через сравнение строковых представлений
        let mut unique_permutations = Vec::new();
        let mut seen_signatures = std::collections::HashSet::new();

        for permutation in permutations.drain(..) {
            let signature = Self::get_permutation_signature(&permutation);
            if seen_signatures.insert(signature) {
                unique_permutations.push(permutation);
            }
        }

        *permutations = unique_permutations;
    }

    /// Создает подпись перестановки для сравнения
    fn get_permutation_signature(permutation: &[TileDimensions]) -> String {
        permutation
            .iter()
            .map(|p| {
                format!(
                    "{}x{}{}",
                    p.width,
                    p.height,
                    if p.is_rotated { "R" } else { "N" }
                )
            })
            .collect::<Vec<_>>()
            .join(",")
    }

    /// Выводит статистику перестановок
    pub fn print_permutation_stats(permutations: &[Vec<TileDimensions>]) {
        println!("=== Статистика перестановок ===");
        println!("Всего перестановок: {}", permutations.len());

        if !permutations.is_empty() {
            println!("Панелей в каждой перестановке: {}", permutations[0].len());
        }

        // Покажем первые несколько перестановок для примера
        for (i, permutation) in permutations.iter().take(3).enumerate() {
            println!(
                "Перестановка {}: {}",
                i + 1,
                Self::get_permutation_signature(permutation)
            );
        }

        if permutations.len() > 3 {
            println!("... и еще {} перестановок", permutations.len() - 3);
        }
    }
}
