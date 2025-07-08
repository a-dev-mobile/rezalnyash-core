// features/permutation_generator.rs
use crate::features::input::models::{
    grouped_tile_dimensions::GroupedTileDimensions, 
    tile_dimensions::TileDimensions
};
use std::collections::HashMap;

/// Отвечает за генерацию перестановок панелей (точная копия Java логики)
pub struct PermutationGenerator;

impl PermutationGenerator {
    /// Главный метод создания перестановок групп (точная копия Java compute())
    pub fn create_group_permutations(
        grouped_panels: &[GroupedTileDimensions]
    ) -> Vec<Vec<TileDimensions>> {
        println!("=== Создание перестановок групп ===");
        
        // Шаг 1: Получаем уникальные группы с подсчетом (точная копия Java getDistinctGroupedTileDimensions)
        let distinct_groups = Self::get_distinct_grouped_tile_dimensions(grouped_panels);
        
        // Шаг 2: Создаем список уникальных групп (точная копия Java arrayList2)
        let mut unique_groups: Vec<GroupedTileDimensions> = distinct_groups.keys().cloned().collect();
        
        // Шаг 3: Сортируем группы по убыванию площади (точная копия Java Collections.sort)
        unique_groups.sort_by(|a, b| {
            let area_a = a.instance.width * a.instance.height;
            let area_b = b.instance.width * b.instance.height;
            area_b.cmp(&area_a) // убывание
        });
        
        println!("Уникальных групп найдено: {}", unique_groups.len());
        
        // Шаг 4: КЛЮЧЕВАЯ ОПТИМИЗАЦИЯ - ограничение до 7 групп (точная копия Java)
        let (groups_for_permutations, remaining_groups) = if unique_groups.len() > 7 {
            println!("Ограничиваем перестановки до первых 7 групп из {}", unique_groups.len());
            let first_7 = unique_groups[0..7].to_vec();
            let remaining = unique_groups[7..].to_vec();
            (first_7, remaining)
        } else {
            (unique_groups, vec![])
        };
        
        // Шаг 5: Генерируем все перестановки первых 7 групп (точная копия Java Arrangement.generatePermutations)
        println!("Генерация перестановок для {} групп...", groups_for_permutations.len());
        let mut group_permutations = Self::generate_permutations(groups_for_permutations);
        
        // Шаг 6: К каждой перестановке добавляем оставшиеся группы в исходном порядке (точная копия Java)
        for permutation in &mut group_permutations {
            permutation.extend(remaining_groups.clone());
        }
        
        println!("Сгенерировано {} перестановок групп", group_permutations.len());
        
        // Шаг 7: Преобразуем перестановки групп в перестановки отдельных панелей 
        // (точная копия Java groupedTileDimensionsList2TileDimensionsList)
        let tile_permutations = Self::convert_group_permutations_to_tile_permutations(
            &group_permutations, 
            grouped_panels
        );
        
        // Шаг 8: Удаляем дублирующие перестановки (точная копия Java removeDuplicatedPermutations)
        let mut final_permutations = tile_permutations;
        let removed_count = Self::remove_duplicated_permutations(&mut final_permutations);
        
        println!("Удалено дублирующих перестановок: {}", removed_count);
        println!("Итоговое количество перестановок: {}", final_permutations.len());
        
        final_permutations
    }
    
    /// Получение уникальных групп с подсчетом (точная копия Java getDistinctGroupedTileDimensions)
    fn get_distinct_grouped_tile_dimensions(
        grouped_panels: &[GroupedTileDimensions]
    ) -> HashMap<GroupedTileDimensions, i32> {
        let mut map = HashMap::new();
        
        for group in grouped_panels {
            let count = map.entry(group.clone()).or_insert(0);
            *count += 1;
        }
        
        map
    }
    
    /// Генерация всех перестановок (точная копия Java Arrangement.generatePermutations)
    fn generate_permutations<T: Clone>(mut list: Vec<T>) -> Vec<Vec<T>> {
        if list.is_empty() {
            return vec![vec![]];
        }
        
        let first = list.remove(0); // Точная копия Java list.remove(0)
        let mut result = Vec::new();
        
        // Рекурсивно генерируем перестановки для оставшихся элементов
        for sub_permutation in Self::generate_permutations(list.clone()) {
            // Для каждой перестановки вставляем первый элемент на каждую возможную позицию
            for i in 0..=sub_permutation.len() {
                let mut new_permutation = sub_permutation.clone();
                new_permutation.insert(i, first.clone());
                result.push(new_permutation);
            }
        }
        
        result
    }
    
    /// Преобразование перестановок групп в перестановки панелей 
    /// (точная копия Java groupedTileDimensionsList2TileDimensionsList)
    fn convert_group_permutations_to_tile_permutations(
        group_permutations: &[Vec<GroupedTileDimensions>],
        original_grouped_panels: &[GroupedTileDimensions]
    ) -> Vec<Vec<TileDimensions>> {
        let mut result = Vec::new();
        
        for group_permutation in group_permutations {
            let mut tile_permutation = Vec::new();
            
            // Для каждой группы в перестановке находим все соответствующие панели
            for group in group_permutation {
                for original_panel in original_grouped_panels {
                    if Self::groups_match(group, original_panel) {
                        tile_permutation.push(original_panel.instance.clone());
                    }
                }
            }
            
            result.push(tile_permutation);
        }
        
        result
    }
    
    /// Проверка соответствия групп
    fn groups_match(group1: &GroupedTileDimensions, group2: &GroupedTileDimensions) -> bool {
        group1.group == group2.group && 
        group1.instance.width == group2.instance.width &&
        group1.instance.height == group2.instance.height
    }
    
    /// Удаление дублирующих перестановок (точная копия Java removeDuplicatedPermutations)
    fn remove_duplicated_permutations(permutations: &mut Vec<Vec<TileDimensions>>) -> i32 {
        let mut unique_hashes = std::collections::HashSet::new();
        let mut removed_count = 0;
        
        permutations.retain(|permutation| {
            // Создаем хэш на основе размеров панелей (точная копия Java логики)
            let mut hash_code = 0i32;
            for tile in permutation {
                // Точная копия Java: (hashCode * 31) + tile.dimensionsBasedHashCode()
                hash_code = hash_code.wrapping_mul(31).wrapping_add(
                    Self::dimensions_based_hash_code(tile)
                );
            }
            
            if unique_hashes.insert(hash_code) {
                true // Оставляем уникальную перестановку
            } else {
                removed_count += 1;
                false // Удаляем дубликат
            }
        });
        
        removed_count
    }
    
    /// Хэш на основе размеров (точная копия Java dimensionsBasedHashCode)
    fn dimensions_based_hash_code(tile: &TileDimensions) -> i32 {
        // Точная копия Java: (this.width * 31) + this.height
        (tile.width as i32).wrapping_mul(31).wrapping_add(tile.height as i32)
    }
    
    /// Вывод статистики перестановок
    pub fn print_permutation_stats(permutations: &[Vec<TileDimensions>]) {
        println!("=== Статистика перестановок ===");
        println!("Всего перестановок: {}", permutations.len());
        
        if !permutations.is_empty() {
            println!("Панелей в каждой перестановке: {}", permutations[0].len());
            
            // Показываем первые несколько перестановок
            for (i, permutation) in permutations.iter().take(3).enumerate() {
                let signature: Vec<String> = permutation.iter()
                    .map(|tile| format!("{}x{}", tile.width, tile.height))
                    .collect();
                println!("Перестановка {}: {}", i + 1, signature.join(","));
            }
            
            if permutations.len() > 3 {
                println!("... и еще {} перестановок", permutations.len() - 3);
            }
        }
    }
}

// Добавляем реализацию PartialEq и Eq для GroupedTileDimensions для HashMap
impl PartialEq for GroupedTileDimensions {
    fn eq(&self, other: &Self) -> bool {
        self.group == other.group &&
        self.instance.width == other.instance.width &&
        self.instance.height == other.instance.height
    }
}

impl Eq for GroupedTileDimensions {}

impl std::hash::Hash for GroupedTileDimensions {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.group.hash(state);
        self.instance.width.hash(state);
        self.instance.height.hash(state);
    }
}