use crate::features::{ input::models::panel::Panel, placement::Placement};

/// Полное решение задачи раскроя
#[derive(Debug, Clone)]
pub struct Solution {
    pub placements: Vec<Placement>,
    pub unplaced_panels: Vec<Panel>,
    pub total_efficiency: f64,
    pub total_cuts: usize,
    pub total_cut_length: i32,
    pub total_used_area: i64,
    pub total_waste_area: i64,
}

impl Solution {
    pub fn new() -> Self {
        Self {
            placements: Vec::new(),
            unplaced_panels: Vec::new(),
            total_efficiency: 0.0,
            total_cuts: 0,
            total_cut_length: 0,
            total_used_area: 0,
            total_waste_area: 0,
        }
    }

    pub fn calculate_totals(&mut self) {
        self.total_used_area = self.placements.iter().map(|p| p.used_area).sum();
        self.total_waste_area = self.placements.iter().map(|p| p.waste_area).sum();
        self.total_cuts = self.placements.iter().map(|p| p.cuts.len()).sum();
        self.total_cut_length = self
            .placements
            .iter()
            .flat_map(|p| &p.cuts)
            .map(|c| c.length())
            .sum();

        let total_area = self.total_used_area + self.total_waste_area;
        self.total_efficiency = if total_area > 0 {
            self.total_used_area as f64 / total_area as f64
        } else {
            0.0
        };
    }

    /// Оценка качества решения для сортировки
    /// TODO: Взять приоритеты из SolutionComparatorFactory.java
    pub fn score(&self) -> (i32, i64, i32) {
        // Приоритет 1: Максимум размещенных панелей (отрицательное для сортировки по убыванию)
        let placed_panels = -(self
            .placements
            .iter()
            .map(|p| p.placed_panels.len() as i32)
            .sum::<i32>());

        // Приоритет 2: Минимум отходов
        let waste_area = self.total_waste_area;

        // Приоритет 3: Минимум резов
        let cuts_count = self.total_cuts as i32;

        (placed_panels, waste_area, cuts_count)
    }
}
