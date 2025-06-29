use crate::{log_debug, log_info, models::{configuration::Configuration, task::Task, tile_dimensions::TileDimensions}};

/// Структура для управления обработкой перестановок (однопоточная версия)
#[derive(Debug)]
pub struct PermutationThreadSpawner {
    progress_tracker: Option<*mut ProgressTracker>,
    max_alive_spawner_threads: usize,
    interval_between_max_alive_check: u64,
    total_threads: usize,
    completed_threads: usize,
}

impl PermutationThreadSpawner {
    pub fn new() -> Self {
        Self {
            progress_tracker: None,
            max_alive_spawner_threads: 5,
            interval_between_max_alive_check: 1000,
            total_threads: 0,
            completed_threads: 0,
        }
    }

   pub fn set_progress_tracker(&mut self, tracker: &mut ProgressTracker) {
        self.progress_tracker = Some(tracker as *mut ProgressTracker);
    }

   pub fn set_max_alive_spawner_threads(&mut self, max_threads: usize) {
        self.max_alive_spawner_threads = max_threads;
    }

   pub fn set_interval_between_max_alive_check(&mut self, interval: u64) {
        self.interval_between_max_alive_check = interval;
    }

  pub  fn register_completed_permutation(&mut self) {
        self.total_threads += 1;
        self.completed_threads += 1;
    }

  pub  fn get_nbr_total_threads(&self) -> usize {
        self.total_threads
    }

  pub  fn get_nbr_unfinished_threads(&self) -> usize {
        self.total_threads - self.completed_threads
    }
}

/// Структура для отслеживания прогресса (однопоточная версия)
#[derive(Debug)]
pub struct ProgressTracker {
    total_permutations: usize,
    material: String,
    start_time: std::time::SystemTime,
}

impl ProgressTracker {
    const MAX_PERMUTATIONS_WITH_SOLUTION: usize = 150;

    pub fn new(total_permutations: usize, task: &mut Task, material: String) -> Self {
        Self {
            total_permutations,
            material,
            start_time: std::time::SystemTime::now(),
        }
    }

    pub fn refresh_task_status_info(&mut self, spawner: &PermutationThreadSpawner) {
        let elapsed_ms = self
            .start_time
            .elapsed()
            .unwrap_or(std::time::Duration::from_secs(0))
            .as_millis() as f32;

        let progress_percent = if spawner.get_nbr_total_threads() == 0 {
            0
        } else {
            let thread_progress = ((spawner.get_nbr_total_threads() - 1) as f32
                / self.total_permutations as f32)
                * 100.0;
            let time_progress = (elapsed_ms / 600000.0) * 100.0; // 10 минут = 100%

            std::cmp::min(
                std::cmp::max(time_progress as i32, thread_progress as i32),
                100,
            )
        };

        log_debug!(
            "Progress update: {}% complete, threads: {}, elapsed: {:.1}s",
            progress_percent,
            spawner.get_nbr_total_threads(),
            elapsed_ms / 1000.0
        );
    }
}

/// Обработка одной комбинации перестановки и набора листов
fn process_single_permutation_stock_combination(
    tile_arrangement: &[TileDimensions],
    stock_solution: &crate::models::stock_solution::StockSolution,
    permutation_idx: usize,
    stock_idx: usize,
    task: &mut Task,
    configuration: &Configuration,
    optimization_factor: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    log_debug!(
        "Processing combination: permutation[{}] + stock[{}] - {} tiles on {} stock panels",
        permutation_idx,
        stock_idx,
        tile_arrangement.len(),
        stock_solution.get_stock_tile_dimensions().len()
    );

    // TODO: Здесь будет реальный алгоритм размещения панелей
    // Пока что просто имитируем обработку

    // Простая имитация: "размещаем" панели и считаем эффективность
    let total_tile_area: u64 = tile_arrangement
        .iter()
        .map(|tile| tile.width * tile.height)
        .sum();

    let total_stock_area = stock_solution.get_total_area();

    let efficiency = if total_stock_area > 0 {
        (total_tile_area as f64 / total_stock_area as f64) * 100.0
    } else {
        0.0
    };

    log_debug!(
        "Combination efficiency: {:.2}% ({} tile area / {} stock area)",
        efficiency,
        total_tile_area,
        total_stock_area
    );

    // Имитируем нахождение хорошего решения
    if efficiency > 80.0 {
        log_info!(
            "Found good solution: {:.2}% efficiency for permutation[{}] + stock[{}]",
            efficiency,
            permutation_idx,
            stock_idx
        );
    }

    Ok(())
}
