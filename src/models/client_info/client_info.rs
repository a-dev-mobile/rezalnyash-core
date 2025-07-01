use serde::{Deserialize, Serialize};
use crate::errors::client_info_errors::ClientInfoError;

/// Represents client information with validation and utility methods
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientInfo {
    /// Client's unique identifier
    pub id: Option<String>,
    /// Client's subscription plan
    pub plan: Option<String>,
    /// Parent client identifier for hierarchical relationships
    pub parent_id: Option<String>,
    /// Client's IP address
    pub ip: Option<String>,
    /// Client's email address
    pub email: Option<String>,
    /// ISO country code (e.g., "US", "GB")
    pub country_iso_code: Option<String>,
    /// Geographic location description
    pub location: Option<String>,
    /// Language preference (e.g., "en-US")
    pub language: Option<String>,
    /// Time zone identifier (e.g., "America/New_York")
    pub time_zone: Option<String>,
    /// Operating system information
    pub os: Option<String>,
    /// Screen width in pixels
    pub screen_width: Option<u32>,
    /// Screen height in pixels
    pub screen_height: Option<u32>,
    /// Browser information
    pub browser: Option<String>,
    /// Device type/model
    pub device: Option<String>,
    /// Unique device identifier
    pub device_id: Option<String>,
    /// Application/client version
    pub version: Option<String>,
}


impl ClientInfo {
    /// Creates a new ClientInfo instance with all fields set to None
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a ClientInfo with required fields
    pub fn with_id(id: String) -> Self {
        Self {
            id: Some(id),
            ..Default::default()
        }
    }

    /// Validates email format using a simple regex pattern
    pub fn set_email(&mut self, email: String) -> Result<(), ClientInfoError> {
        if email.is_empty() {
            self.email = None;
            return Ok(());
        }

        if self.is_valid_email(&email) {
            self.email = Some(email);
            Ok(())
        } else {
            Err(ClientInfoError::InvalidEmail { email })
        }
    }

    /// Validates and sets country ISO code (must be 2 characters)
    pub fn set_country_iso_code(&mut self, code: String) -> Result<(), ClientInfoError> {
        if code.is_empty() {
            self.country_iso_code = None;
            return Ok(());
        }

        if code.len() == 2 && code.chars().all(|c| c.is_ascii_alphabetic()) {
            self.country_iso_code = Some(code.to_uppercase());
            Ok(())
        } else {
            Err(ClientInfoError::InvalidCountryCode { code })
        }
    }

    /// Validates and sets screen dimensions
    pub fn set_screen_dimensions(&mut self, width: u32, height: u32) -> Result<(), ClientInfoError> {
        if width == 0 || height == 0 {
            return Err(ClientInfoError::InvalidScreenDimensions {
                message: "Screen dimensions must be greater than 0".to_string()
            });
        }

        if width > 10000 || height > 10000 {
            return Err(ClientInfoError::InvalidScreenDimensions {
                message: "Screen dimensions seem unreasonably large".to_string()
            });
        }

        self.screen_width = Some(width);
        self.screen_height = Some(height);
        Ok(())
    }

    /// Validates and sets IP address
    pub fn set_ip(&mut self, ip: String) -> Result<(), ClientInfoError> {
        if ip.is_empty() {
            self.ip = None;
            return Ok(());
        }

        if self.is_valid_ip(&ip) {
            self.ip = Some(ip);
            Ok(())
        } else {
            Err(ClientInfoError::InvalidIpAddress { ip })
        }
    }

    /// Checks if the client info has minimum required fields
    pub fn is_valid(&self) -> bool {
        self.id.is_some() || self.email.is_some() || self.device_id.is_some()
    }

    /// Returns a summary of the client for logging purposes
    pub fn summary(&self) -> String {
        let id = self.id.as_deref().unwrap_or("unknown");
        let location = self.location.as_deref().unwrap_or("unknown");
        let device = self.device.as_deref().unwrap_or("unknown");
        
        format!("ClientInfo(id: {}, location: {}, device: {})", id, location, device)
    }

    /// Checks if client has screen information
    pub fn has_screen_info(&self) -> bool {
        self.screen_width.is_some() && self.screen_height.is_some()
    }

    /// Gets screen resolution as a tuple if available
    pub fn screen_resolution(&self) -> Option<(u32, u32)> {
        match (self.screen_width, self.screen_height) {
            (Some(width), Some(height)) => Some((width, height)),
            _ => None,
        }
    }

    /// Checks if client is from a specific country
    pub fn is_from_country(&self, country_code: &str) -> bool {
        self.country_iso_code
            .as_ref()
            .map(|code| code.eq_ignore_ascii_case(country_code))
            .unwrap_or(false)
    }

    /// Builder pattern methods for fluent construction
    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn plan(mut self, plan: String) -> Self {
        self.plan = Some(plan);
        self
    }

    pub fn parent_id(mut self, parent_id: String) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    pub fn ip_unchecked(mut self, ip: String) -> Self {
        self.ip = Some(ip);
        self
    }

    pub fn email_unchecked(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    pub fn location(mut self, location: String) -> Self {
        self.location = Some(location);
        self
    }

    pub fn language(mut self, language: String) -> Self {
        self.language = Some(language);
        self
    }

    pub fn time_zone(mut self, time_zone: String) -> Self {
        self.time_zone = Some(time_zone);
        self
    }

    pub fn os(mut self, os: String) -> Self {
        self.os = Some(os);
        self
    }

    pub fn browser(mut self, browser: String) -> Self {
        self.browser = Some(browser);
        self
    }

    pub fn device(mut self, device: String) -> Self {
        self.device = Some(device);
        self
    }

    pub fn device_id(mut self, device_id: String) -> Self {
        self.device_id = Some(device_id);
        self
    }

    pub fn version(mut self, version: String) -> Self {
        self.version = Some(version);
        self
    }

    // Private helper methods
    fn is_valid_email(&self, email: &str) -> bool {
        // Simple email validation - contains @ and has characters before and after
        let parts: Vec<&str> = email.split('@').collect();
        parts.len() == 2 && !parts[0].is_empty() && !parts[1].is_empty() && parts[1].contains('.')
    }

    fn is_valid_ip(&self, ip: &str) -> bool {
        // Simple IPv4 validation
        let parts: Vec<&str> = ip.split('.').collect();
        if parts.len() != 4 {
            return false;
        }

        parts.iter().all(|part| {
            part.parse::<u8>().is_ok()
        })
    }
}

// Implement getters that return references to avoid unnecessary cloning
impl ClientInfo {
    pub fn get_id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn get_plan(&self) -> Option<&str> {
        self.plan.as_deref()
    }

    pub fn get_parent_id(&self) -> Option<&str> {
        self.parent_id.as_deref()
    }

    pub fn get_ip(&self) -> Option<&str> {
        self.ip.as_deref()
    }

    pub fn get_email(&self) -> Option<&str> {
        self.email.as_deref()
    }

    pub fn get_country_iso_code(&self) -> Option<&str> {
        self.country_iso_code.as_deref()
    }

    pub fn get_location(&self) -> Option<&str> {
        self.location.as_deref()
    }

    pub fn get_language(&self) -> Option<&str> {
        self.language.as_deref()
    }

    pub fn get_time_zone(&self) -> Option<&str> {
        self.time_zone.as_deref()
    }

    pub fn get_os(&self) -> Option<&str> {
        self.os.as_deref()
    }

    pub fn get_screen_width(&self) -> Option<u32> {
        self.screen_width
    }

    pub fn get_screen_height(&self) -> Option<u32> {
        self.screen_height
    }

    pub fn get_browser(&self) -> Option<&str> {
        self.browser.as_deref()
    }

    pub fn get_device(&self) -> Option<&str> {
        self.device.as_deref()
    }

    pub fn get_device_id(&self) -> Option<&str> {
        self.device_id.as_deref()
    }

    pub fn get_version(&self) -> Option<&str> {
        self.version.as_deref()
    }
}
