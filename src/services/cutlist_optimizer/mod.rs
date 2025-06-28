// use std::collections::{HashMap, HashSet};
// use std::sync::atomic::{AtomicU64, Ordering};

// use crate::enums::status::Status;
// use crate::enums::status_code::StatusCode;
// use crate::models::calculation_request::CalculationRequest;
// use crate::models::configuration::structs::Configuration;
// use crate::models::grouped_tile_dimensions::GroupedTileDimensions;
// use crate::models::stock_solution::StockSolution;
// use crate::models::task::Task;
// use crate::models::tile_dimensions::TileDimensions;
// use crate::services::arrangement::generate_permutations;


// static TASK_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

// const MAX_PERMUTATION_ITERATIONS: usize = 1000;
// const MAX_STOCK_ITERATIONS: usize = 1000;
// const MAX_ALLOWED_DIGITS: i32 = 6;
// const MAX_ACTIVE_THREADS_PER_TASK: i32 = 5;
// const MAX_PERMUTATIONS_WITH_SOLUTION: usize = 150;

// pub struct CutListOptimizerServiceImpl {
//     pub running_tasks: Task,

// }

// impl CutListOptimizerServiceImpl {



//     fn generate_task_id(&self) -> String {
//         let timestamp = chrono::Utc::now().format("%Y%m%d%H%M").to_string();
//         let counter = TASK_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
//         format!("{}{}", timestamp, counter)
//     }

//     fn remove_duplicated_permutations(&self, permutations: &mut Vec<Vec<TileDimensions>>) -> i32 {
//         let mut seen_hashes = Vec::new();
//         let mut removed_count = 0;
        
//         permutations.retain(|permutation| {
//             let mut hash = 0i32;
//             for tile in permutation {
//                 hash = hash.wrapping_mul(31).wrapping_add(tile.dimensions_based_hash_code());
//             }
            
//             if seen_hashes.contains(&hash) {
//                 removed_count += 1;
//                 false
//             } else {
//                 seen_hashes.push(hash);
//                 true
//             }
//         });
        
//         removed_count
//     }

//     fn is_one_dimensional_optimization(&self, tiles: &[TileDimensions], stock: &[TileDimensions]) -> bool {
//         let mut common_dimensions = vec![tiles[0].get_width(), tiles[0].get_height()];
        
//         // Проверяем tiles
//         for tile in tiles {
//             common_dimensions.retain(|&dim| {
//                 dim == tile.get_width() || dim == tile.get_height()
//             });
//             if common_dimensions.is_empty() {
//                 return false;
//             }
//         }
        
//         // Проверяем stock
//         for stock_tile in stock {
//             common_dimensions.retain(|&dim| {
//                 dim == stock_tile.get_width() || dim == stock_tile.get_height()
//             });
//             if common_dimensions.is_empty() {
//                 return false;
//             }
//         }
        
//         true
//     }

//     fn generate_groups(&self, tiles: &[TileDimensions], stock: &[TileDimensions], task: &Task) -> Vec<GroupedTileDimensions> {
//         // Подсчет количества каждого типа плитки
//         let mut tile_counts = HashMap::new();
//         for tile in tiles {
//             let key = tile.to_string();
//             *tile_counts.entry(key).or_insert(0) += 1;
//         }

//         // Логирование информации о плитках
//         let mut info_str = String::new();
//         for (tile_str, count) in &tile_counts {
//             info_str.push_str(&format!("{}*{} ", tile_str, count));
//         }
//         println!("Task[{}] TotalNbrTiles[{}] Tiles: {}", task.get_id(), tiles.len(), info_str);

//         let mut max_group_size = std::cmp::max(tiles.len() / 100, 1);
//         if self.is_one_dimensional_optimization(tiles, stock) {
//             println!("Task is one dimensional optimization");
//             max_group_size = 1;
//         }

//         let mut grouped_tiles = Vec::new();
//         let mut group_counts = HashMap::new();
//         let mut current_group = 0;

//         for tile in tiles {
//             let group_key = format!("{}{}", tile.to_string(), current_group);
//             let group_count = group_counts.entry(group_key.clone()).or_insert(0);
//             *group_count += 1;

//             grouped_tiles.push(GroupedTileDimensions::from_tile_dimensions(tile.clone(), current_group));

//             // Проверяем, нужно ли разделить группу
//             if let Some(&total_count) = tile_counts.get(&tile.to_string()) {
//                 if total_count > max_group_size && *group_count > total_count / 4 {
//                     println!("Task[{}] Splitting panel set [{}] with [{}] units into two groups", 
//                              task.get_id(), tile.dimensions_to_string(), total_count);
//                     current_group += 1;
//                 }
//             }
//         }

//         grouped_tiles
//     }

//     fn get_distinct_grouped_tile_dimensions<T>(&self, tiles: &[T]) -> HashMap<T, i32> 
//     where 
//         T: Clone + std::hash::Hash + Eq 
//     {
//         let mut counts = HashMap::new();
//         for tile in tiles {
//             *counts.entry(tile.clone()).or_insert(0) += 1;
//         }
//         counts
//     }



//     fn get_nbr_decimal_places(&self, value: &str) -> i32 {
//         if let Some(dot_pos) = value.find('.') {
//             (value.len() - dot_pos - 1) as i32
//         } else {
//             0
//         }
//     }

//     fn get_nbr_integer_places(&self, value: &str) -> i32 {
//         if let Some(dot_pos) = value.find('.') {
//             dot_pos as i32
//         } else {
//             value.len() as i32
//         }
//     }

//     fn get_max_nbr_decimal_places(&self, panels: &[CalculationRequest::Panel]) -> i32 {
//         panels.iter()
//             .filter(|panel| panel.is_valid())
//             .map(|panel| {
//                 std::cmp::max(
//                     self.get_nbr_decimal_places(&panel.width),
//                     self.get_nbr_decimal_places(&panel.height)
//                 )
//             })
//             .max()
//             .unwrap_or(0)
//     }

//     fn get_max_nbr_integer_places(&self, panels: &[CalculationRequest::Panel]) -> i32 {
//         panels.iter()
//             .filter(|panel| panel.is_valid())
//             .map(|panel| {
//                 std::cmp::max(
//                     self.get_nbr_integer_places(&panel.width),
//                     self.get_nbr_integer_places(&panel.height)
//                 )
//             })
//             .max()
//             .unwrap_or(0)
//     }

//     fn grouped_tile_dimensions_list_to_tile_dimensions_list(
//         &self, 
//         grouped_order: &[GroupedTileDimensions],
//         original_tiles: &[TileDimensions]
//     ) -> Vec<TileDimensions> {
//         let mut result = original_tiles.to_vec();
        
//         // Сортируем согласно порядку в grouped_order
//         result.sort_by(|a, b| {
//             let index_a = grouped_order.iter().position(|g| g.matches_tile(a)).unwrap_or(usize::MAX);
//             let index_b = grouped_order.iter().position(|g| g.matches_tile(b)).unwrap_or(usize::MAX);
//             index_a.cmp(&index_b)
//         });
        
//         result
//     }

//     fn compute_optimization(
//         &mut self,
//         tiles: Vec<TileDimensions>,
//         stock: Vec<TileDimensions>, 
//         configuration: &Configuration,
//         task: &mut Task
//     ) {
//         let performance_thresholds = configuration.performance_thresholds;

//         let solutions = task.get_solutions();
        
//         // Генерируем группы плиток
//         let grouped_tiles = self.generate_groups(&tiles, &stock, task);
//         let distinct_grouped = self.get_distinct_grouped_tile_dimensions(&grouped_tiles);

//         // Логирование групп
//         let mut group_info = String::new();
//         for (i, (group, count)) in distinct_grouped.iter().enumerate() {
//             group_info.push_str(&format!(" group[{}:{}*{}] ", i + 1, group, count));
//         }

//         println!("Task[{}] Calculating permutations...", task.get_id());

//         // Сортируем группы по площади (убывание)
//         let mut sorted_groups: Vec<_> = distinct_grouped.keys().cloned().collect();
//         sorted_groups.sort_by(|a, b| b.get_area().partial_cmp(&a.get_area()).unwrap_or(std::cmp::Ordering::Equal));

//         // Разделяем на основные и дополнительные группы
//         let (main_groups, additional_groups) = if sorted_groups.len() > 7 {
//             let main = sorted_groups[..7].to_vec();
//             let additional = sorted_groups[7..].to_vec();
//             (main, additional)
//         } else {
//             (sorted_groups, Vec::new())
//         };

//         // Генерируем перестановки
//         let mut permutations = generate_permutations(&main_groups);
        
//         // Добавляем дополнительные группы к каждой перестановке
//         for permutation in &mut permutations {
//             permutation.extend(additional_groups.clone());
//         }

//         println!("Task[{}] Sorting tiles according to permutations...", task.get_id());

//         // Преобразуем перестановки в списки плиток
//         let mut tile_permutations = Vec::new();
//         for permutation in &permutations {
//             let tile_list = self.grouped_tile_dimensions_list_to_tile_dimensions_list(
//                 permutation, &grouped_tiles
//             );
//             tile_permutations.push(tile_list);
//         }

//         println!("Removing duplicated permutations...");
//         let _removed_count = self.remove_duplicated_permutations(&mut tile_permutations);

//         task.set_running_status();

//         // Инициализируем выбор стоковых панелей
//         let use_single_stock = configuration.is_use_single_stock_unit();
//         let mut stock_picker = StockPanelPicker::new(
//             &tiles, 
//             &stock, 
//             task, 
//             if use_single_stock { Some(1) } else { None }
//         );
//         stock_picker.init();

//         // Рассчитываем фактор оптимизации
//         let mut optimization_factor = if configuration.get_optimization_factor() > 0.0 {
//             (100.0 * configuration.get_optimization_factor()) as i32
//         } else {
//             100
//         };

//         if tiles.len() > 100 {
//             let reduction_factor = 0.5 / (tiles.len() as f64 / 100.0);
//             optimization_factor = (optimization_factor as f64 * reduction_factor) as i32;
//             println!("Limiting solution pool elements to [{}]", optimization_factor);
//         }

//         // Обрабатываем каждую перестановку
//         for (perm_idx, tile_permutation) in tile_permutations.iter().enumerate() {
//             if !task.is_running() {
//                 println!("Task no longer has running status. Stopping at permutation [{}]", perm_idx);
//                 break;
//             }

//             if task.has_solution_all_fit() && perm_idx > MAX_PERMUTATIONS_WITH_SOLUTION {
//                 task.set_percentage_done(100);
//                 println!("Task has solution and reached max permutations");
//                 break;
//             }

//             // Обрабатываем различные стоковые решения
//             for stock_idx in 0..MAX_STOCK_ITERATIONS {
//                 let stock_solution = match stock_picker.get_stock_solution(stock_idx) {
//                     Some(solution) => solution,
//                     None => {
//                         println!("No more possible stock solutions: stockSolution[{}] permutationIdx[{}]", 
//                                 stock_idx, perm_idx);
//                         break;
//                     }
//                 };

//                 if !task.is_running() {
//                     println!("Task no longer has running status. Stopping stock loop for permutation [{}]", perm_idx);
//                     break;
//                 }

//                 // Проверяем, стоит ли продолжать с этим стоковым решением
//                 if task.has_solution_all_fit() {
//                     let current_solutions = task.get_solutions();
//                     if !current_solutions.is_empty() {
//                         let best_solution = &current_solutions[0];
//                         if best_solution.get_mosaics().len() == 1 && 
//                            best_solution.get_total_area() < stock_solution.get_total_area() {
//                             println!("Stopping stock loop - better solution exists");
//                             break;
//                         }
//                     }
//                 }

//                 println!("Starting permutationIdx[{}/{}] with stock solution [{}]", 
//                         perm_idx, tile_permutations.len(), stock_idx);

//                 // TODO: Реализовать создание и выполнение CutListThread
//                 // Здесь должна быть логика:
//                 // 1. Создание CutListThreadBuilder
//                 // 2. Настройка параметров (cut thickness, min trim dimension, etc.)
//                 // 3. Установка различных ориентаций резки (HORIZONTAL, VERTICAL, BOTH)
//                 // 4. Выполнение алгоритма раскроя
//                 // 5. Добавление найденных решений в task
                
//                 self.process_permutation_with_stock(
//                     tile_permutation,
//                     &stock_solution,
//                     configuration,
//                     task,
//                     perm_idx,
//                     stock_idx,
//                     optimization_factor
//                 );
//             }
//         }

//         if task.get_status() == Status::Running {
//             task.set_percentage_done(100);
//         }
//     }

//     fn process_permutation_with_stock(
//         &self,
//         tiles: &[TileDimensions],
//         stock_solution: &StockSolution,
//         configuration: &Configuration,
//         task: &mut Task,
//         perm_idx: usize,
//         stock_idx: usize,
//         optimization_factor: i32
//     ) {
//         // TODO: Реализовать основную логику раскроя
//         // Этот метод должен:
//         // 1. Получить компараторы для сортировки решений
//         // 2. Рассчитать толщину реза и минимальные размеры обрезки
//         // 3. Создать CutListThread с различными ориентациями резки
//         // 4. Выполнить алгоритм раскроя
//         // 5. Добавить найденные решения в список решений задачи
        
//         println!("TODO: Process permutation {} with stock {}", 
//                 perm_idx, stock_idx);
        
//         // Получаем компараторы
//         let _comparators = SolutionComparatorFactory::get_solution_comparator_list(
//             &PriorityListFactory::get_final_solution_prioritized_comparator_list(configuration)
//         );

//         // Рассчитываем толщину реза
//         let cut_thickness = match configuration.get_cut_thickness().parse::<f64>() {
//             Ok(thickness) => (thickness * task.get_factor()) as i32,
//             Err(_) => {
//                 println!("Error parsing cut thickness value: [{}]", configuration.get_cut_thickness());
//                 0
//             }
//         };

//         // Рассчитываем минимальный размер обрезки
//         let min_trim_dimension = match configuration.get_min_trim_dimension().parse::<f64>() {
//             Ok(trim) => (trim * task.get_factor()) as i32,
//             Err(_) => {
//                 println!("Error parsing minimum trim dimension value: [{}]", configuration.get_min_trim_dimension());
//                 0
//             }
//         };

//         // TODO: Здесь должна быть реализация различных стратегий резки:
//         // - AREA (любая ориентация)
//         // - AREA_HCUTS_1ST (горизонтальные резы первыми)  
//         // - AREA_VCUTS_1ST (вертикальные резы первыми)
        
//         // В зависимости от configuration.get_cut_orientation_preference():
//         // 0 - все ориентации
//         // 1 - только горизонтальные
//         // 2 - только вертикальные
//     }

//     pub fn compute(&mut self, request: CalculationRequest, task_id: String) {
//         let panels = request.get_panels();
//         let stock_panels = request.get_stock_panels();
//         let configuration = request.get_configuration();

//         // Рассчитываем масштабирующий фактор для точности вычислений
//         let max_decimal_places = std::cmp::max(
//             std::cmp::max(
//                 self.get_max_nbr_decimal_places(panels),
//                 self.get_max_nbr_decimal_places(stock_panels)
//             ),
//             std::cmp::max(
//                 self.get_nbr_decimal_places(&configuration.get_cut_thickness()),
//                 self.get_nbr_decimal_places(&configuration.get_min_trim_dimension())
//             )
//         );

//         let max_integer_places = std::cmp::max(
//             std::cmp::max(
//                 self.get_max_nbr_integer_places(panels),
//                 self.get_max_nbr_integer_places(stock_panels)
//             ),
//             std::cmp::max(
//                 self.get_nbr_integer_places(&configuration.get_cut_thickness()),
//                 self.get_nbr_integer_places(&configuration.get_min_trim_dimension())
//             )
//         );

//         let mut decimal_places = max_decimal_places;
//         if decimal_places + max_integer_places > MAX_ALLOWED_DIGITS {
//             println!("Maximum allowed digits exceeded: maxDecimalPlaces[{}] maxIntegerPlaces[{}] maxAllowedDigits[{}]", 
//                     decimal_places, max_integer_places, MAX_ALLOWED_DIGITS);
//             decimal_places = std::cmp::max(MAX_ALLOWED_DIGITS - max_integer_places, 0);
//         }

//         let factor = 10_f64.powi(decimal_places);

//         // Преобразуем панели в TileDimensions
//         let mut tiles = Vec::new();
//         for panel in panels.iter().filter(|p| p.is_valid()) {
//             let width = (panel.width.parse::<f64>().unwrap() * factor).round() as i32;
//             let height = (panel.height.parse::<f64>().unwrap() * factor).round() as i32;
            
//             for _ in 0..panel.count {
//                 tiles.push(TileDimensions::new(
//                     panel.id.clone(),
//                     width,
//                     height,
//                     panel.material.clone(),
//                     panel.orientation,
//                     panel.label.clone()
//                 ));
//             }
//         }

//         // Преобразуем стоковые панели в TileDimensions
//         let mut stock_tiles = Vec::new();
//         for panel in stock_panels.iter().filter(|p| p.is_valid()) {
//             let width = (panel.width.parse::<f64>().unwrap() * factor).round() as i32;
//             let height = (panel.height.parse::<f64>().unwrap() * factor).round() as i32;
            
//             for _ in 0..panel.count {
//                 stock_tiles.push(TileDimensions::new(
//                     panel.id.clone(),
//                     width,
//                     height,
//                     panel.material.clone(),
//                     panel.orientation,
//                     panel.label.clone()
//                 ));
//             }
//         }

//         // Создаем задачу
//         let mut task = Task::new(task_id.clone());
//         task.set_calculation_request(request);
//         task.set_factor(factor);
//         task.build_solution();

//         self.running_tasks.insert(task_id.clone(), task.clone());

//         // Запускаем оптимизацию
//         if let Some(task_ref) = self.running_tasks.get_mut(&task_id) {
//             self.compute_optimization(tiles, stock_tiles, &configuration, task_ref);
//             task_ref.check_if_finished();
//         }
//     }
// }

// impl CutListOptimizerService for CutListOptimizerServiceImpl {
//     fn set_allow_multiple_tasks_per_client(&mut self, allow: bool) {
//         self.allow_multiple_tasks_per_client = allow;
//     }

//     fn get_stats(&self) -> Stats {
//         // TODO: Реализовать сбор статистики
//         Stats::new()
//     }

//     fn init(&mut self, _thread_count: i32) {
//         // В однопоточной версии инициализация упрощена
//         println!("CutListOptimizerServiceImpl initialized in single-threaded mode");
//     }

//     fn get_tasks(&self, _client_id: &str, status: Status) -> Vec<String> {
//         self.running_tasks
//             .iter()
//             .filter(|(_, task)| task.get_status() == status)
//             .map(|(id, _)| id.clone())
//             .collect()
//     }

//     fn submit_task(&mut self, request: CalculationRequest) -> CalculationSubmissionResult {
//         // Генерируем ID задачи и запускаем вычисления
//         let task_id = self.generate_task_id();
//         self.compute(request, task_id.clone());

//         CalculationSubmissionResult::with_task_id(StatusCode::Ok.get_string_value(), task_id)
//     }

//     fn get_task_status(&mut self, task_id: &str) -> Option<TaskStatusResponse> {
//         if let Some(task) = self.running_tasks.get_mut(task_id) {
//             task.build_solution();
//             task.set_last_queried(chrono::Utc::now().timestamp_millis());

//             let mut response = TaskStatusResponse::new();
//             response.set_status(task.get_status().to_string());
//             response.set_init_percentage(task.get_max_thread_progress_percentage());
//             response.set_percentage_done(task.get_percentage_done());
//             response.set_solution(task.get_solution().cloned());

//             Some(response)
//         } else {
//             None
//         }
//     }

//     fn stop_task(&mut self, task_id: &str) -> Option<TaskStatusResponse> {
//         if let Some(task) = self.running_tasks.get_mut(task_id) {
//             let stop_result = task.stop();
//             if stop_result != 0 {
//                 println!("Unable to stop task. Current status is: {:?}", task.get_status());
//             }

//             let mut response = TaskStatusResponse::new();
//             response.set_status(task.get_status().to_string());
//             response.set_init_percentage(task.get_max_thread_progress_percentage());
//             response.set_percentage_done(task.get_percentage_done());
//             response.set_solution(task.get_solution().cloned());

//             Some(response)
//         } else {
//             None
//         }
//     }

//     fn terminate_task(&mut self, task_id: &str) -> i32 {
//         if let Some(task) = self.running_tasks.get_mut(task_id) {
//             let result = task.terminate();
//             if result != 0 {
//                 println!("Unable to terminate task. Current status is: {:?}", task.get_status());
//             }
//             result
//         } else {
//             -1
//         }
//     }
// }