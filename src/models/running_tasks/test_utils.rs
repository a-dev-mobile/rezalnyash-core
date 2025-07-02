//! Test utilities for RunningTasks
//!
//! This module provides shared test utilities to ensure proper synchronization
//! between different test modules that use the RunningTasks singleton.

use std::sync::Mutex;

/// Global mutex to ensure tests that use the RunningTasks singleton run sequentially
/// This prevents race conditions and state contamination between tests.
pub static GLOBAL_TEST_MUTEX: Mutex<()> = Mutex::new(());

/// Helper function to acquire the global test lock
/// This should be called at the beginning of any test that uses RunningTasks
pub fn acquire_test_lock() -> std::sync::MutexGuard<'static, ()> {
    GLOBAL_TEST_MUTEX.lock().unwrap()
}
