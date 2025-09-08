# Документация по работе с ошибками в приложении

## Обзор

Приложение использует централизованную систему обработки ошибок на основе enum `AppError`, который объединяет все типы ошибок в единую иерархию. Эта система обеспечивает типобезопасную обработку ошибок и удобную категоризацию по функциональным областям.

## Основная структура

### AppError

Главный тип ошибки находится в `src/errors/app_error.rs` и содержит следующие категории:

```rust
use crate::errors::{AppError, Result};

pub enum AppError {
    Core(CoreError),        // Базовые ошибки приложения и внешних библиотек
    Task(TaskError),        // Ошибки управления задачами и их выполнения
    Computation(ComputationError), // Ошибки оптимизации и вычислений
    Service(ServiceError),  // Ошибки сервисов и управления ресурсами
    Stock(StockError),      // Ошибки работы со стоком и панелями
}
```

### Алиас Result

Для удобства определен тип `Result<T>`:

```rust
pub type Result<T> = std::result::Result<T, AppError>;
```

## Категории ошибок

### 1. CoreError (`src/errors/core_errors.rs`)

Базовые ошибки приложения и внешних библиотек:

```rust
// Ошибки конфигурации
CoreError::InvalidConfiguration { message }
CoreError::InvalidInput { details }

// Ошибки внешних библиотек (автоматические конверсии)
CoreError::Io(std::io::Error)
CoreError::Json(serde_json::Error)
CoreError::ParseFloat(std::num::ParseFloatError)

// Внутренние ошибки
CoreError::Internal { message }
```

#### Примеры использования:

```rust
// Прямое создание
return Err(AppError::Core(CoreError::InvalidConfiguration {
    message: "Missing required field: timeout".to_string()
}));

// Автоматическая конверсия из std::io::Error
let file = std::fs::File::open("config.json")?; // автоматически -> AppError::Core(CoreError::Io)

// Автоматическая конверсия из serde_json::Error
let config: Config = serde_json::from_str(content)?; // автоматически -> AppError::Core(CoreError::Json)
```

### 2. TaskError (`src/errors/task_errors.rs`)

Ошибки управления задачами, жизненного цикла и выполнения:

```rust
// Ошибки поиска и идентификации
TaskError::TaskNotFound { id }
TaskError::TaskInvalidId { task_id }
TaskError::TaskAlreadyExists { task_id }

// Ошибки выполнения
TaskError::TaskExecution(String)
TaskError::TaskCancelled
TaskError::TaskTimeout

// Ошибки состояния
TaskError::TaskInvalidState { current_state }
TaskError::TaskInvalidStatusTransition { from, to }
TaskError::TaskMissingClientInfo

// Ошибки потоков
TaskError::TaskThreadTerminated
TaskError::TaskThreadSync { message }
TaskError::TaskThreadError { details }

// Ошибки валидации
TaskError::TaskMaterialMismatch { tile_material, mosaic_material }

// Ошибки блокировок
TaskError::TaskLockError { operation }
```

#### Примеры использования:

```rust
// Проверка существования задачи
if !self.tasks.contains_key(&task_id) {
    return Err(TaskError::TaskNotFound { id: task_id }.into());
}

// Проверка состояния задачи
if task.status != TaskStatus::Pending {
    return Err(TaskError::TaskInvalidState { 
        current_state: task.status.to_string() 
    }.into());
}

// Проверка соответствия материалов
if tile.material != mosaic.material {
    return Err(TaskError::TaskMaterialMismatch {
        tile_material: tile.material.clone(),
        mosaic_material: mosaic.material.clone()
    }.into());
}
```

### 3. ComputationError (`src/errors/computation_errors.rs`)

Ошибки оптимизации, вычислений и алгоритмов:

```rust
ComputationError::OptimizationFailed { reason }
ComputationError::ComputationGeneral { message }
ComputationError::SolutionComputation { message }
ComputationError::SolutionComparison { message }
ComputationError::NodeCopy { message }
ComputationError::CandidateSearch { message }
```

#### Примеры использования:

```rust
// Ошибка оптимизации
if solutions.is_empty() {
    return Err(ComputationError::OptimizationFailed {
        reason: "No valid solutions found with given constraints".to_string()
    }.into());
}

// Ошибка при вычислении решения
match compute_solution(&params) {
    Ok(solution) => solution,
    Err(e) => return Err(ComputationError::SolutionComputation {
        message: format!("Failed to compute solution: {}", e)
    }.into())
}
```

### 4. ServiceError (`src/errors/service_errors.rs`)

Ошибки сервисов и управления ресурсами:

```rust
// Ошибки задач и клиентов
ServiceError::ServiceTaskAlreadyExists { task_id }
ServiceError::ServiceClientAlreadyHasTask { client_id, existing_task_id }
ServiceError::ServiceInvalidClientId { client_id }

// Ошибки состояния сервиса
ServiceError::ServiceShuttingDown
ServiceError::ServiceMaxTasksReached
ServiceError::ServiceNotInitialized

// Ошибки ресурсов
ServiceError::ServiceLockFailed { resource }
ServiceError::ServiceResourceUnavailable { resource }
ServiceError::ServicePermissionDenied { operation }

// Ошибки потоков
ServiceError::ServiceThreadSync { message }
ServiceError::ServiceThreadError { details }
ServiceError::ThreadPoolError { message }

// Ошибки инициализации и валидации
ServiceError::ServiceInitializationError { message }
ServiceError::ServiceValidationError { message }
ServiceError::ServiceLockError { message }
```

#### Примеры использования:

```rust
// Проверка лимита задач
if self.active_tasks.len() >= MAX_TASKS {
    return Err(ServiceError::ServiceMaxTasksReached.into());
}

// Проверка состояния сервиса
if self.is_shutting_down {
    return Err(ServiceError::ServiceShuttingDown.into());
}

// Ошибка доступа к ресурсу
match self.resource_pool.try_acquire() {
    Some(resource) => resource,
    None => return Err(ServiceError::ServiceResourceUnavailable {
        resource: "computation_thread".to_string()
    }.into())
}
```

### 5. StockError (`src/errors/stock_errors.rs`)

Ошибки работы со стоком и панелями:

```rust
StockError::StockNoStockTiles
StockError::StockNoTilesToFit
StockError::StockComputationLimitExceeded
StockError::StockPanelPickerNotInitialized
StockError::StockGenerationInterrupted { message }
StockError::StockNoMoreSolutions
StockError::StockPanelPickerThread { message }
```

#### Примеры использования:

```rust
// Проверка наличия плиток
if stock_tiles.is_empty() {
    return Err(StockError::StockNoStockTiles.into());
}

// Проверка лимитов вычислений
if iterations > MAX_ITERATIONS {
    return Err(StockError::StockComputationLimitExceeded.into());
}
```

## Методы анализа ошибок

`AppError` предоставляет методы для анализа характера ошибки:

### is_retryable()

Определяет, можно ли повторить операцию после ошибки:

```rust
match result {
    Err(error) if error.is_retryable() => {
        // Повторить операцию через некоторое время
        retry_operation().await?;
    }
    Err(error) => return Err(error),
    Ok(value) => value,
}
```

Ошибки, которые считаются повторяемыми:
- `CoreError::Io` - проблемы ввода-вывода
- `TaskError::TaskTimeout`, `TaskError::TaskExecution`, `TaskError::TaskThreadSync`
- `ServiceError::ServiceResourceUnavailable`, `ServiceError::ServiceLockFailed`, `ServiceError::ServiceMaxTasksReached`
- `StockError::StockGenerationInterrupted`, `StockError::StockPanelPickerThread`

### is_client_error()

Определяет ошибки клиента (эквивалент 4xx HTTP статусов):

```rust
match result {
    Err(error) if error.is_client_error() => {
        log::warn!("Client error: {}", error);
        return Err(error); // Не повторять
    }
    Err(error) => {
        log::error!("Server error: {}", error);
        // Возможно, стоит повторить
    }
    Ok(value) => value,
}
```

### is_server_error()

Определяет серверные ошибки (эквивалент 5xx HTTP статусов):

```rust
if error.is_server_error() {
    // Логгировать как критическую ошибку
    log::error!("Critical server error: {}", error);
    // Отправить уведомление администратору
    notify_admin(&error).await;
}
```

## Лучшие практики

### 1. Использование автоматических конверсий

```rust
// Хорошо - автоматическая конверсия
fn read_config() -> Result<Config> {
    let content = std::fs::read_to_string("config.json")?; // io::Error -> AppError
    let config: Config = serde_json::from_str(&content)?;  // serde_json::Error -> AppError
    Ok(config)
}

// Плохо - явное создание
fn read_config() -> Result<Config> {
    let content = match std::fs::read_to_string("config.json") {
        Ok(content) => content,
        Err(e) => return Err(AppError::Core(CoreError::Io(e))),
    };
    // ...
}
```

### 2. Контекстуальная информация

```rust
// Хорошо - предоставляет контекст
return Err(TaskError::TaskInvalidState {
    current_state: format!("{}[{}]", task.status, task.progress)
}.into());

// Плохо - мало информации
return Err(TaskError::TaskInvalidState {
    current_state: "invalid".to_string()
}.into());
```

### 3. Цепочка ошибок

```rust
// Хорошо - сохраняет первопричину
fn process_data(data: &str) -> Result<ProcessedData> {
    let parsed = parse_input(data)
        .map_err(|e| ComputationError::ComputationGeneral {
            message: format!("Failed to parse input data: {}", e)
        })?;
    
    compute_result(parsed)
        .map_err(|e| ComputationError::SolutionComputation {
            message: format!("Computation failed after parsing: {}", e)
        })
}
```

### 4. Обработка ошибок в разных слоях

```rust
// Слой сервиса
pub fn create_task(&mut self, request: CreateTaskRequest) -> Result<Task> {
    // Валидация входных данных
    if request.client_id.is_empty() {
        return Err(ServiceError::ServiceInvalidClientId {
            client_id: request.client_id
        }.into());
    }
    
    // Проверка бизнес-правил
    if self.client_has_active_task(&request.client_id) {
        return Err(ServiceError::ServiceClientAlreadyHasTask {
            client_id: request.client_id.clone(),
            existing_task_id: self.get_client_task_id(&request.client_id)
        }.into());
    }
    
    // Создание задачи
    let task = Task::new(request)?; // Может вернуть TaskError
    self.tasks.insert(task.id.clone(), task.clone());
    
    Ok(task)
}

// Слой контроллера
pub async fn handle_create_task(request: CreateTaskRequest) -> HttpResponse {
    match service.create_task(request).await {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(error) if error.is_client_error() => {
            HttpResponse::BadRequest().json(ErrorResponse {
                message: error.to_string()
            })
        }
        Err(error) => {
            log::error!("Server error: {}", error);
            HttpResponse::InternalServerError().json(ErrorResponse {
                message: "Internal server error".to_string()
            })
        }
    }
}
```

### 5. Логгирование ошибок

```rust
pub fn log_error(error: &AppError, context: &str) {
    match error {
        error if error.is_client_error() => {
            log::warn!("[{}] Client error: {}", context, error);
        }
        error if error.is_server_error() => {
            log::error!("[{}] Server error: {}", context, error);
            if let Some(source) = error.source() {
                log::error!("[{}] Caused by: {}", context, source);
            }
        }
        _ => {
            log::info!("[{}] Info: {}", context, error);
        }
    }
}
```

## Примеры типичных сценариев

### Создание новой задачи

```rust
pub fn create_optimization_task(&mut self, request: OptimizationRequest) -> Result<TaskId> {
    // 1. Валидация входных данных
    if request.tiles.is_empty() {
        return Err(CoreError::InvalidInput {
            details: "No tiles provided for optimization".to_string()
        }.into());
    }
    
    // 2. Проверка бизнес-правил
    if self.active_tasks.len() >= MAX_CONCURRENT_TASKS {
        return Err(ServiceError::ServiceMaxTasksReached.into());
    }
    
    // 3. Создание задачи
    let task_id = TaskId::new();
    let task = Task::new(task_id.clone(), request)?;
    
    // 4. Сохранение и запуск
    self.active_tasks.insert(task_id.clone(), task);
    self.start_optimization_thread(task_id.clone())?;
    
    Ok(task_id)
}
```

### Обработка результатов вычислений

```rust
pub fn process_optimization_result(&mut self, task_id: TaskId) -> Result<OptimizationResult> {
    // 1. Поиск задачи
    let task = self.active_tasks.get_mut(&task_id)
        .ok_or_else(|| TaskError::TaskNotFound { id: task_id.to_string() })?;
    
    // 2. Проверка состояния
    if task.status != TaskStatus::Completed {
        return Err(TaskError::TaskInvalidState {
            current_state: task.status.to_string()
        }.into());
    }
    
    // 3. Получение результата
    let result = task.result.take()
        .ok_or_else(|| ComputationError::SolutionComputation {
            message: "No result available for completed task".to_string()
        })?;
    
    // 4. Очистка
    self.active_tasks.remove(&task_id);
    
    Ok(result)
}
```

### Retry-логика

```rust
pub async fn execute_with_retry<F, T>(mut operation: F, max_attempts: u32) -> Result<T>
where
    F: FnMut() -> Result<T>,
{
    let mut attempts = 0;
    
    loop {
        attempts += 1;
        
        match operation() {
            Ok(result) => return Ok(result),
            Err(error) if error.is_retryable() && attempts < max_attempts => {
                let delay = Duration::from_millis(100 * attempts as u64);
                tokio::time::sleep(delay).await;
                continue;
            }
            Err(error) => return Err(error),
        }
    }
}
```

Эта система ошибок обеспечивает типобезопасную, расширяемую и удобную обработку всех видов ошибок в приложении.