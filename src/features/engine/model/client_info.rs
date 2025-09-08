use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    pub id: Option<String>,
    pub plan: Option<String>,
    pub parent_id: Option<String>,
    pub ip: Option<String>,
    pub email: Option<String>,
    pub country_iso_code: Option<String>,
    pub location: Option<String>,
    pub language: Option<String>,
    pub time_zone: Option<String>,
    pub os: Option<String>,
    pub screen_width: Option<i32>,
    pub screen_height: Option<i32>,
    pub browser: Option<String>,
    pub device: Option<String>,
    pub device_id: Option<String>,
    pub version: Option<String>,
}

impl Default for ClientInfo {
    fn default() -> Self {
        Self {
            id: None,
            plan: None,
            parent_id: None,
            ip: None,
            email: None,
            country_iso_code: None,
            location: None,
            language: None,
            time_zone: None,
            os: None,
            screen_width: None,
            screen_height: None,
            browser: None,
            device: None,
            device_id: None,
            version: None,
        }
    }
}