use std::fmt;

/// Stock errors - Stock solution and panel picker errors
#[derive(Debug)]
pub enum StockError {
    StockNoStockTiles,
    StockNoTilesToFit,
    StockComputationLimitExceeded,
    StockPanelPickerNotInitialized,
    StockGenerationInterrupted {
        message: String,
    },
    StockNoMoreSolutions,
    StockPanelPickerThread {
        message: String,
    },
}

impl fmt::Display for StockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StockNoStockTiles => write!(f, "No stock tiles provided"),
            Self::StockNoTilesToFit => write!(f, "No tiles to fit provided"),
            Self::StockComputationLimitExceeded => {
                write!(f, "Stock solution computation exceeded reasonable limits")
            }
            Self::StockPanelPickerNotInitialized => {
                write!(f, "Stock panel picker thread not initialized")
            }
            Self::StockGenerationInterrupted { message } => {
                write!(f, "Stock solution generation interrupted: {}", message)
            }
            Self::StockNoMoreSolutions => write!(f, "No more stock solutions available"),
            Self::StockPanelPickerThread { message } => {
                write!(f, "Stock panel picker thread error: {}", message)
            }
        }
    }
}

impl std::error::Error for StockError {}
