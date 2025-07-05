//! Services Module
//!
//! This module contains service implementations for the cut list optimizer.
//! It provides the main service interface and implementation for optimizing
//! material cutting layouts.

pub mod cut_list_optimizer_service;
pub mod cut_list_optimizer_service_impl;

pub use cut_list_optimizer_service::CutListOptimizerService;
pub use cut_list_optimizer_service_impl::CutListOptimizerServiceImpl;

#[cfg(test)]
mod cut_list_optimizer_service_impl_test;