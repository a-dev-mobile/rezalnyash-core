pub mod client_info;

#[cfg(test)]
mod client_info_tests;

pub use client_info::ClientInfo;
pub use crate::errors::client_info_errors::ClientInfoError;
