pub mod watch_dog;

#[cfg(test)]
pub mod watch_dog_test;

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
// Re-export TaskReport from the task module
pub use crate::models::task::TaskReport;
