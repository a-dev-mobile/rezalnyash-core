pub mod watch_dog;
pub mod progress_tracker;
pub mod permutation_thread_spawner;

#[cfg(test)]
pub mod watch_dog_test;
#[cfg(test)]
pub mod progress_tracker_test;
#[cfg(test)]
pub mod permutation_thread_spawner_test;

pub use watch_dog::{
    WatchDog, 
    CutListLogger, 
    CutListOptimizerService, 
    ThreadPoolExecutor,
    DefaultCutListLogger,
    DefaultCutListOptimizerService,
    DefaultThreadPoolExecutor,
    WatchDogControl,
};

pub use progress_tracker::{
    ProgressTracker,
    PermutationThreadSpawner as ProgressTrackerTrait,
};

pub use permutation_thread_spawner::{
    PermutationThreadSpawner,
    ThreadState,
    ManagedThread,
    DEFAULT_MAX_ALIVE_SPAWNER_THREADS,
    DEFAULT_INTERVAL_BETWEEN_MAX_ALIVE_CHECK,
};

// Re-export TaskReport from the task module
pub use crate::models::task::TaskReport;
