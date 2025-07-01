#[cfg(test)]
mod tests {
    use crate::models::client_info::ClientInfo;
    use crate::errors::client_info_errors::ClientInfoError;

    #[test]
    fn test_new_client_info() {
        let client = ClientInfo::new();
        assert_eq!(client.id, None);
        assert_eq!(client.email, None);
        assert_eq!(client.screen_width, None);
        assert!(!client.is_valid());
    }

    #[test]
    fn test_with_id() {
        let client = ClientInfo::with_id("test-id".to_string());
        assert_eq!(client.get_id(), Some("test-id"));
        assert!(client.is_valid());
    }

    #[test]
    fn test_builder_pattern() {
        let client = ClientInfo::new()
            .id("user123".to_string())
            .plan("premium".to_string())
            .location("New York".to_string())
            .device("iPhone".to_string());

        assert_eq!(client.get_id(), Some("user123"));
        assert_eq!(client.get_plan(), Some("premium"));
        assert_eq!(client.get_location(), Some("New York"));
        assert_eq!(client.get_device(), Some("iPhone"));
        assert!(client.is_valid());
    }

    #[test]
    fn test_set_email_valid() {
        let mut client = ClientInfo::new();
        let result = client.set_email("test@example.com".to_string());
        assert!(result.is_ok());
        assert_eq!(client.get_email(), Some("test@example.com"));
    }

    #[test]
    fn test_error_display_basic() {
        let error = ClientInfoError::InvalidIpAddress { ip: "invalid".to_string() };
        assert_eq!(error.to_string(), "Invalid IP address: invalid");
        
        let error = ClientInfoError::ValidationError { message: "Empty client ID".to_string() };
        assert_eq!(error.to_string(), "Client info validation error: Empty client ID");
        
        let error = ClientInfoError::InvalidEmail { email: "bad-email".to_string() };
        assert_eq!(error.to_string(), "Invalid email format: bad-email");
    }

    #[test]
    fn test_set_email_empty() {
        let mut client = ClientInfo::new();
        let result = client.set_email("".to_string());
        assert!(result.is_ok());
        assert_eq!(client.get_email(), None);
    }

    #[test]
    fn test_set_country_iso_code_valid() {
        let mut client = ClientInfo::new();
        let result = client.set_country_iso_code("us".to_string());
        assert!(result.is_ok());
        assert_eq!(client.get_country_iso_code(), Some("US"));
    }

    #[test]
    fn test_set_country_iso_code_invalid() {
        let mut client = ClientInfo::new();
        let result = client.set_country_iso_code("USA".to_string());
        assert!(result.is_err());
        assert_eq!(client.get_country_iso_code(), None);

        match result {
            Err(ClientInfoError::InvalidCountryCode { code }) => assert_eq!(code, "USA"),
            _ => panic!("Expected InvalidCountryCode error"),
        }
    }

    #[test]
    fn test_set_country_iso_code_empty() {
        let mut client = ClientInfo::new();
        let result = client.set_country_iso_code("".to_string());
        assert!(result.is_ok());
        assert_eq!(client.get_country_iso_code(), None);
    }

    #[test]
    fn test_set_screen_dimensions_valid() {
        let mut client = ClientInfo::new();
        let result = client.set_screen_dimensions(1920, 1080);
        assert!(result.is_ok());
        assert_eq!(client.get_screen_width(), Some(1920));
        assert_eq!(client.get_screen_height(), Some(1080));
        assert!(client.has_screen_info());
    }

    #[test]
    fn test_set_screen_dimensions_zero() {
        let mut client = ClientInfo::new();
        let result = client.set_screen_dimensions(0, 1080);
        assert!(result.is_err());
        assert_eq!(client.get_screen_width(), None);
        assert_eq!(client.get_screen_height(), None);
        assert!(!client.has_screen_info());
    }

    #[test]
    fn test_set_screen_dimensions_too_large() {
        let mut client = ClientInfo::new();
        let result = client.set_screen_dimensions(15000, 1080);
        assert!(result.is_err());
        assert_eq!(client.get_screen_width(), None);
        assert_eq!(client.get_screen_height(), None);
    }

    #[test]
    fn test_set_ip_valid() {
        let mut client = ClientInfo::new();
        let result = client.set_ip("192.168.1.1".to_string());
        assert!(result.is_ok());
        assert_eq!(client.get_ip(), Some("192.168.1.1"));
    }

    #[test]
    fn test_set_ip_invalid() {
        let mut client = ClientInfo::new();
        let result = client.set_ip("invalid.ip".to_string());
        assert!(result.is_err());
        assert_eq!(client.get_ip(), None);

        match result {
            Err(ClientInfoError::InvalidIpAddress { ip }) => assert_eq!(ip, "invalid.ip"),
            _ => panic!("Expected InvalidIpAddress error"),
        }
    }

    #[test]
    fn test_set_ip_empty() {
        let mut client = ClientInfo::new();
        let result = client.set_ip("".to_string());
        assert!(result.is_ok());
        assert_eq!(client.get_ip(), None);
    }

    #[test]
    fn test_is_valid_with_id() {
        let client = ClientInfo::new().id("test".to_string());
        assert!(client.is_valid());
    }

    #[test]
    fn test_is_valid_with_email() {
        let client = ClientInfo::new().email_unchecked("test@example.com".to_string());
        assert!(client.is_valid());
    }

    #[test]
    fn test_is_valid_with_device_id() {
        let client = ClientInfo::new().device_id("device123".to_string());
        assert!(client.is_valid());
    }

    #[test]
    fn test_is_valid_empty() {
        let client = ClientInfo::new();
        assert!(!client.is_valid());
    }

    #[test]
    fn test_summary() {
        let client = ClientInfo::new()
            .id("user123".to_string())
            .location("San Francisco".to_string())
            .device("MacBook Pro".to_string());

        let summary = client.summary();
        assert_eq!(summary, "ClientInfo(id: user123, location: San Francisco, device: MacBook Pro)");
    }

    #[test]
    fn test_summary_with_defaults() {
        let client = ClientInfo::new();
        let summary = client.summary();
        assert_eq!(summary, "ClientInfo(id: unknown, location: unknown, device: unknown)");
    }

    #[test]
    fn test_screen_resolution() {
        let mut client = ClientInfo::new();
        assert_eq!(client.screen_resolution(), None);

        client.set_screen_dimensions(1920, 1080).unwrap();
        assert_eq!(client.screen_resolution(), Some((1920, 1080)));
    }

    #[test]
    fn test_is_from_country() {
        let client = ClientInfo::new()
            .id("test".to_string());
        
        // No country set
        assert!(!client.is_from_country("US"));

        let mut client_with_country = client.clone();
        client_with_country.set_country_iso_code("us".to_string()).unwrap();
        
        assert!(client_with_country.is_from_country("US"));
        assert!(client_with_country.is_from_country("us"));
        assert!(!client_with_country.is_from_country("CA"));
    }

    #[test]
    fn test_all_getters() {
        let client = ClientInfo::new()
            .id("test-id".to_string())
            .plan("premium".to_string())
            .parent_id("parent-123".to_string())
            .ip_unchecked("192.168.1.1".to_string())
            .email_unchecked("test@example.com".to_string())
            .location("New York".to_string())
            .language("en-US".to_string())
            .time_zone("America/New_York".to_string())
            .os("macOS".to_string())
            .browser("Safari".to_string())
            .device("MacBook".to_string())
            .device_id("device-456".to_string())
            .version("1.0.0".to_string());

        assert_eq!(client.get_id(), Some("test-id"));
        assert_eq!(client.get_plan(), Some("premium"));
        assert_eq!(client.get_parent_id(), Some("parent-123"));
        assert_eq!(client.get_ip(), Some("192.168.1.1"));
        assert_eq!(client.get_email(), Some("test@example.com"));
        assert_eq!(client.get_location(), Some("New York"));
        assert_eq!(client.get_language(), Some("en-US"));
        assert_eq!(client.get_time_zone(), Some("America/New_York"));
        assert_eq!(client.get_os(), Some("macOS"));
        assert_eq!(client.get_browser(), Some("Safari"));
        assert_eq!(client.get_device(), Some("MacBook"));
        assert_eq!(client.get_device_id(), Some("device-456"));
        assert_eq!(client.get_version(), Some("1.0.0"));
    }

    #[test]
    fn test_clone_and_equality() {
        let client1 = ClientInfo::new()
            .id("test".to_string())
            .email_unchecked("test@example.com".to_string());

        let client2 = client1.clone();
        assert_eq!(client1, client2);

        let client3 = ClientInfo::new()
            .id("different".to_string())
            .email_unchecked("test@example.com".to_string());

        assert_ne!(client1, client3);
    }

    #[test]
    fn test_default() {
        let client1 = ClientInfo::default();
        let client2 = ClientInfo::new();
        assert_eq!(client1, client2);
    }

    #[test]
    fn test_error_display() {
        let email_error = ClientInfoError::InvalidEmail { email: "bad-email".to_string() };
        assert_eq!(format!("{}", email_error), "Invalid email format: bad-email");

        let country_error = ClientInfoError::InvalidCountryCode { code: "USA".to_string() };
        assert_eq!(format!("{}", country_error), "Invalid country ISO code: USA");

        let screen_error = ClientInfoError::InvalidScreenDimensions { message: "too big".to_string() };
        assert_eq!(format!("{}", screen_error), "Invalid screen dimensions: too big");

        let ip_error = ClientInfoError::InvalidIpAddress { ip: "bad.ip".to_string() };
        assert_eq!(format!("{}", ip_error), "Invalid IP address: bad.ip");

        let validation_error = ClientInfoError::ValidationError { message: "general error".to_string() };
        assert_eq!(format!("{}", validation_error), "Client info validation error: general error");
    }

    #[test]
    fn test_email_validation_edge_cases() {
        let mut client = ClientInfo::new();

        // Valid emails
        assert!(client.set_email("user@domain.com".to_string()).is_ok());
        assert!(client.set_email("test.email@sub.domain.org".to_string()).is_ok());
        assert!(client.set_email("a@b.c".to_string()).is_ok());

        // Invalid emails
        assert!(client.set_email("@domain.com".to_string()).is_err());
        assert!(client.set_email("user@".to_string()).is_err());
        assert!(client.set_email("user@domain".to_string()).is_err());
        assert!(client.set_email("userdomain.com".to_string()).is_err());
        assert!(client.set_email("user@@domain.com".to_string()).is_err());
    }

    #[test]
    fn test_ip_validation_edge_cases() {
        let mut client = ClientInfo::new();

        // Valid IPs
        assert!(client.set_ip("0.0.0.0".to_string()).is_ok());
        assert!(client.set_ip("255.255.255.255".to_string()).is_ok());
        assert!(client.set_ip("127.0.0.1".to_string()).is_ok());

        // Invalid IPs
        assert!(client.set_ip("256.1.1.1".to_string()).is_err());
        assert!(client.set_ip("1.1.1".to_string()).is_err());
        assert!(client.set_ip("1.1.1.1.1".to_string()).is_err());
        assert!(client.set_ip("a.b.c.d".to_string()).is_err());
        assert!(client.set_ip("192.168.1.-1".to_string()).is_err());
    }

    #[test]
    fn test_serde_serialization() {
        let client = ClientInfo::new()
            .id("test-123".to_string())
            .email_unchecked("test@example.com".to_string())
            .location("Test City".to_string());

        // Test serialization
        let json = serde_json::to_string(&client).expect("Failed to serialize");
        assert!(json.contains("test-123"));
        assert!(json.contains("test@example.com"));

        // Test deserialization
        let deserialized: ClientInfo = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(client, deserialized);
    }

    #[test]
    fn test_country_code_normalization() {
        let mut client = ClientInfo::new();
        
        // Lowercase should be converted to uppercase
        client.set_country_iso_code("gb".to_string()).unwrap();
        assert_eq!(client.get_country_iso_code(), Some("GB"));

        // Mixed case should be converted to uppercase
        client.set_country_iso_code("Ca".to_string()).unwrap();
        assert_eq!(client.get_country_iso_code(), Some("CA"));
    }

    #[test]
    fn test_screen_dimensions_boundary_values() {
        let mut client = ClientInfo::new();

        // Test minimum valid values
        assert!(client.set_screen_dimensions(1, 1).is_ok());
        assert_eq!(client.screen_resolution(), Some((1, 1)));

        // Test maximum valid values
        assert!(client.set_screen_dimensions(10000, 10000).is_ok());
        assert_eq!(client.screen_resolution(), Some((10000, 10000)));

        // Test just over the limit
        assert!(client.set_screen_dimensions(10001, 1080).is_err());
        assert!(client.set_screen_dimensions(1920, 10001).is_err());
    }
}
