pub mod app_error;
pub mod core_errors;
pub mod task_errors;
pub mod computation_errors;
pub mod service_errors;
pub mod stock_errors;

// Re-export the main types for convenience
pub use app_error::{AppError, Result};
pub use core_errors::CoreError;
pub use task_errors::TaskError;
pub use computation_errors::ComputationError;
pub use service_errors::ServiceError;
pub use stock_errors::StockError;


/* 

## Созданные файлы:

1. __`core_errors.rs`__ - Базовые ошибки приложения и внешних библиотек:

   - `InvalidConfiguration`
   - `InvalidInput`
   - `Io`, `Json`, `ParseFloat`
   - `Internal`

2. __`task_errors.rs`__ - Ошибки управления задачами, жизненного цикла и выполнения:

   - `TaskNotFound`, `TaskInvalidId`
   - `TaskExecution`, `TaskCancelled`, `TaskTimeout`
   - `TaskInvalidState`, `TaskInvalidStatusTransition`
   - `TaskMissingClientInfo`, `TaskThreadTerminated`
   - `TaskThreadSync`, `TaskThreadError`
   - `TaskMaterialMismatch`

3. __`computation_errors.rs`__ - Ошибки оптимизации, вычислений и алгоритмов:

   - `OptimizationFailed`
   - `ComputationGeneral`, `SolutionComputation`, `SolutionComparison`
   - `NodeCopy`, `CandidateSearch`

4. __`service_errors.rs`__ - Ошибки сервисов и управления ресурсами:

   - `ServiceTaskAlreadyExists`, `ServiceClientAlreadyHasTask`
   - `ServiceInvalidClientId`, `ServiceShuttingDown`
   - `ServiceMaxTasksReached`, `ServiceLockFailed`
   - `ServiceResourceUnavailable`, `ServicePermissionDenied`
   - `ServiceThreadSync`, `ServiceThreadError`

5. __`stock_errors.rs`__ - Ошибки работы со стоком и панелями:

   - `StockNoStockTiles`, `StockNoTilesToFit`
   - `StockComputationLimitExceeded`, `StockPanelPickerNotInitialized`
   - `StockGenerationInterrupted`, `StockNoMoreSolutions`
   - `StockPanelPickerThread`

6. __`app_error.rs`__ - Главный enum `AppError`, объединяющий все типы ошибок:

   - Содержит варианты для каждой категории ошибок
   - Реализует автоматические конверсии из специфичных типов ошибок
   - Сохраняет все методы: `is_retryable()`, `is_client_error()`, `is_server_error()`
   - Включает `Result<T>` type alias

7. __`mod.rs`__ - Обновлен для экспорта всех новых модулей

##


*/