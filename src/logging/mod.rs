pub mod enums;
pub mod init;
pub mod macros;
pub mod structs;

pub use enums::LogLevel;
pub use init::{init_logging, AppLogger};
pub use macros::{debug, error, info, warn};
pub use structs::LogConfig;

pub use crate::{log_debug, log_error, log_info, log_warn};
