
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusCode {
    Ok = 0,
    InvalidTiles = 1,
    InvalidStockTiles = 2,
    TaskAlreadyRunning = 3,
    ServerUnavailable = 4,
    TooManyPanels = 5,
    TooManyStockPanels = 6,
}

impl StatusCode {
    pub fn value(&self) -> i32 {
        *self as i32
    }

    pub fn string_value(&self) -> String {
        self.value().to_string()
    }
}
