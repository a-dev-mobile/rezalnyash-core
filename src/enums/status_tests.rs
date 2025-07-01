//! Tests for Status enum
//! 
//! This module contains comprehensive unit tests for the Status enum,
//! including state transitions, validation, and utility methods.

#[cfg(test)]
mod tests {
    use super::super::status::*;

    #[test]
    fn test_status_creation() {
        let status = Status::Idle;
        assert_eq!(status, Status::Idle);
    }

    #[test]
    fn test_status_default() {
        let status = Status::default();
        assert_eq!(status, Status::Idle);
    }

    #[test]
    fn test_status_is_active() {
        assert!(!Status::Idle.is_active());
        assert!(Status::Queued.is_active());
        assert!(Status::Running.is_active());
        assert!(!Status::Finished.is_active());
        assert!(!Status::Stopped.is_active());
        assert!(!Status::Terminated.is_active());
        assert!(!Status::Error.is_active());
    }

    #[test]
    fn test_status_is_completed() {
        assert!(!Status::Idle.is_completed());
        assert!(!Status::Queued.is_completed());
        assert!(!Status::Running.is_completed());
        assert!(Status::Finished.is_completed());
        assert!(Status::Stopped.is_completed());
        assert!(Status::Terminated.is_completed());
        assert!(Status::Error.is_completed());
    }

    #[test]
    fn test_status_is_successful() {
        assert!(!Status::Idle.is_successful());
        assert!(!Status::Queued.is_successful());
        assert!(!Status::Running.is_successful());
        assert!(Status::Finished.is_successful());
        assert!(!Status::Stopped.is_successful());
        assert!(!Status::Terminated.is_successful());
        assert!(!Status::Error.is_successful());
    }

    #[test]
    fn test_status_is_failed() {
        assert!(!Status::Idle.is_failed());
        assert!(!Status::Queued.is_failed());
        assert!(!Status::Running.is_failed());
        assert!(!Status::Finished.is_failed());
        assert!(!Status::Stopped.is_failed());
        assert!(!Status::Terminated.is_failed());
        assert!(Status::Error.is_failed());
    }

    #[test]
    fn test_status_can_transition_to() {
        // From Idle
        assert!(Status::Idle.can_transition_to(Status::Queued));
        assert!(Status::Idle.can_transition_to(Status::Error));
        assert!(!Status::Idle.can_transition_to(Status::Running));
        assert!(!Status::Idle.can_transition_to(Status::Finished));

        // From Queued
        assert!(Status::Queued.can_transition_to(Status::Running));
        assert!(Status::Queued.can_transition_to(Status::Stopped));
        assert!(Status::Queued.can_transition_to(Status::Terminated));
        assert!(Status::Queued.can_transition_to(Status::Error));
        assert!(!Status::Queued.can_transition_to(Status::Idle));
        assert!(!Status::Queued.can_transition_to(Status::Finished));

        // From Running
        assert!(Status::Running.can_transition_to(Status::Finished));
        assert!(Status::Running.can_transition_to(Status::Stopped));
        assert!(Status::Running.can_transition_to(Status::Terminated));
        assert!(Status::Running.can_transition_to(Status::Error));
        assert!(!Status::Running.can_transition_to(Status::Idle));
        assert!(!Status::Running.can_transition_to(Status::Queued));

        // From Finished (terminal state)
        assert!(Status::Finished.can_transition_to(Status::Error));
        assert!(!Status::Finished.can_transition_to(Status::Idle));
        assert!(!Status::Finished.can_transition_to(Status::Queued));
        assert!(!Status::Finished.can_transition_to(Status::Running));

        // From Error
        assert!(Status::Error.can_transition_to(Status::Idle));
        assert!(Status::Error.can_transition_to(Status::Queued));
        assert!(!Status::Error.can_transition_to(Status::Running));
        assert!(!Status::Error.can_transition_to(Status::Finished));

        // Same status transitions
        assert!(Status::Idle.can_transition_to(Status::Idle));
        assert!(Status::Running.can_transition_to(Status::Running));
    }

    #[test]
    fn test_status_transition_to() {
        // Valid transitions
        let result = Status::Idle.transition_to(Status::Queued);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Status::Queued);

        let result = Status::Running.transition_to(Status::Finished);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Status::Finished);

        // Invalid transitions
        let result = Status::Idle.transition_to(Status::Running);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), StatusTransitionError::InvalidTransition { .. }));

        let result = Status::Finished.transition_to(Status::Running);
        assert!(result.is_err());
    }

    #[test]
    fn test_status_valid_next_statuses() {
        let idle_next = Status::Idle.valid_next_statuses();
        assert!(idle_next.contains(&Status::Queued));
        assert!(idle_next.contains(&Status::Error));
        assert!(!idle_next.contains(&Status::Idle)); // Same status excluded
        assert!(!idle_next.contains(&Status::Running));

        let running_next = Status::Running.valid_next_statuses();
        assert!(running_next.contains(&Status::Finished));
        assert!(running_next.contains(&Status::Stopped));
        assert!(running_next.contains(&Status::Terminated));
        assert!(running_next.contains(&Status::Error));
        assert!(!running_next.contains(&Status::Running)); // Same status excluded

        let finished_next = Status::Finished.valid_next_statuses();
        assert!(finished_next.contains(&Status::Error));
        assert_eq!(finished_next.len(), 1); // Only error transition allowed
    }

    #[test]
    fn test_status_description() {
        assert_eq!(Status::Idle.description(), "Task is idle and ready to be queued");
        assert_eq!(Status::Queued.description(), "Task is queued and waiting for processing");
        assert_eq!(Status::Running.description(), "Task is currently being processed");
        assert_eq!(Status::Finished.description(), "Task completed successfully");
        assert_eq!(Status::Stopped.description(), "Task was stopped by user request");
        assert_eq!(Status::Terminated.description(), "Task was forcefully terminated");
        assert_eq!(Status::Error.description(), "Task encountered an error during processing");
    }

    #[test]
    fn test_status_priority() {
        assert_eq!(Status::Error.priority(), 0);
        assert_eq!(Status::Running.priority(), 1);
        assert_eq!(Status::Queued.priority(), 2);
        assert_eq!(Status::Stopped.priority(), 3);
        assert_eq!(Status::Terminated.priority(), 4);
        assert_eq!(Status::Finished.priority(), 5);
        assert_eq!(Status::Idle.priority(), 6);
    }

    #[test]
    fn test_status_ordering() {
        let mut statuses = vec![
            Status::Idle,
            Status::Finished,
            Status::Error,
            Status::Running,
            Status::Queued,
        ];
        
        statuses.sort();
        
        // Should be sorted by priority (Error first, Idle last)
        assert_eq!(statuses[0], Status::Error);
        assert_eq!(statuses[1], Status::Running);
        assert_eq!(statuses[2], Status::Queued);
        assert_eq!(statuses[3], Status::Finished);
        assert_eq!(statuses[4], Status::Idle);
    }

    #[test]
    fn test_status_all() {
        let all_statuses = Status::all();
        assert_eq!(all_statuses.len(), 7);
        assert!(all_statuses.contains(&Status::Idle));
        assert!(all_statuses.contains(&Status::Queued));
        assert!(all_statuses.contains(&Status::Running));
        assert!(all_statuses.contains(&Status::Finished));
        assert!(all_statuses.contains(&Status::Stopped));
        assert!(all_statuses.contains(&Status::Terminated));
        assert!(all_statuses.contains(&Status::Error));
    }

    #[test]
    fn test_status_from_str() {
        assert_eq!(Status::from_str("idle").unwrap(), Status::Idle);
        assert_eq!(Status::from_str("QUEUED").unwrap(), Status::Queued);
        assert_eq!(Status::from_str("Running").unwrap(), Status::Running);
        assert_eq!(Status::from_str("finished").unwrap(), Status::Finished);
        assert_eq!(Status::from_str("stopped").unwrap(), Status::Stopped);
        assert_eq!(Status::from_str("terminated").unwrap(), Status::Terminated);
        assert_eq!(Status::from_str("error").unwrap(), Status::Error);

        // Invalid status
        let result = Status::from_str("invalid");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), StatusParseError::UnknownStatus(_)));
    }

    #[test]
    fn test_status_as_str() {
        assert_eq!(Status::Idle.as_str(), "idle");
        assert_eq!(Status::Queued.as_str(), "queued");
        assert_eq!(Status::Running.as_str(), "running");
        assert_eq!(Status::Finished.as_str(), "finished");
        assert_eq!(Status::Stopped.as_str(), "stopped");
        assert_eq!(Status::Terminated.as_str(), "terminated");
        assert_eq!(Status::Error.as_str(), "error");
    }

    #[test]
    fn test_status_display() {
        assert_eq!(format!("{}", Status::Idle), "idle");
        assert_eq!(format!("{}", Status::Queued), "queued");
        assert_eq!(format!("{}", Status::Running), "running");
        assert_eq!(format!("{}", Status::Finished), "finished");
        assert_eq!(format!("{}", Status::Stopped), "stopped");
        assert_eq!(format!("{}", Status::Terminated), "terminated");
        assert_eq!(format!("{}", Status::Error), "error");
    }

    #[test]
    fn test_status_serialization() {
        let status = Status::Running;
        
        // Test serialization to JSON
        let json = serde_json::to_string(&status).expect("Should serialize to JSON");
        assert_eq!(json, "\"Running\"");

        // Test deserialization from JSON
        let deserialized: Status = serde_json::from_str(&json)
            .expect("Should deserialize from JSON");
        assert_eq!(deserialized, Status::Running);
    }

    #[test]
    fn test_status_equality_and_hash() {
        use std::collections::HashMap;
        
        let status1 = Status::Running;
        let status2 = Status::Running;
        let status3 = Status::Finished;

        assert_eq!(status1, status2);
        assert_ne!(status1, status3);

        // Test that Status can be used as HashMap key
        let mut map = HashMap::new();
        map.insert(status1, "running task");
        map.insert(status3, "finished task");

        assert_eq!(map.get(&Status::Running), Some(&"running task"));
        assert_eq!(map.get(&Status::Finished), Some(&"finished task"));
        assert_eq!(map.get(&Status::Idle), None);
    }

    #[test]
    fn test_status_clone_and_copy() {
        let original = Status::Running;
        let cloned = original.clone();
        let copied = original;

        assert_eq!(original, cloned);
        assert_eq!(original, copied);
        assert_eq!(cloned, copied);
    }

    // Error Tests
    #[test]
    fn test_status_transition_error_display() {
        let error = StatusTransitionError::InvalidTransition {
            from: Status::Finished,
            to: Status::Running,
        };
        
        let error_string = format!("{}", error);
        assert!(error_string.contains("Invalid status transition"));
        assert!(error_string.contains("finished"));
        assert!(error_string.contains("running"));
    }

    #[test]
    fn test_status_parse_error_display() {
        let error = StatusParseError::UnknownStatus("invalid_status".to_string());
        
        let error_string = format!("{}", error);
        assert!(error_string.contains("Unknown status"));
        assert!(error_string.contains("invalid_status"));
    }

    // Integration Tests
    #[test]
    fn test_status_workflow() {
        // Simulate a typical task workflow
        let mut current_status = Status::Idle;
        
        // Start the task
        current_status = current_status.transition_to(Status::Queued).unwrap();
        assert_eq!(current_status, Status::Queued);
        assert!(current_status.is_active());
        
        // Begin processing
        current_status = current_status.transition_to(Status::Running).unwrap();
        assert_eq!(current_status, Status::Running);
        assert!(current_status.is_active());
        
        // Complete successfully
        current_status = current_status.transition_to(Status::Finished).unwrap();
        assert_eq!(current_status, Status::Finished);
        assert!(current_status.is_completed());
        assert!(current_status.is_successful());
    }

    #[test]
    fn test_status_error_workflow() {
        // Simulate an error workflow
        let mut current_status = Status::Running;
        
        // Encounter an error
        current_status = current_status.transition_to(Status::Error).unwrap();
        assert_eq!(current_status, Status::Error);
        assert!(current_status.is_completed());
        assert!(current_status.is_failed());
        
        // Retry from error
        current_status = current_status.transition_to(Status::Idle).unwrap();
        assert_eq!(current_status, Status::Idle);
        assert!(!current_status.is_active());
        assert!(!current_status.is_completed());
    }

    #[test]
    fn test_status_stop_workflow() {
        // Simulate a stop workflow
        let mut current_status = Status::Queued;
        
        // Stop the queued task
        current_status = current_status.transition_to(Status::Stopped).unwrap();
        assert_eq!(current_status, Status::Stopped);
        assert!(current_status.is_completed());
        assert!(!current_status.is_successful());
        assert!(!current_status.is_failed());
    }

    #[test]
    fn test_status_comprehensive_transitions() {
        // Test all valid transitions systematically
        let test_cases = vec![
            (Status::Idle, Status::Queued, true),
            (Status::Idle, Status::Error, true),
            (Status::Idle, Status::Running, false),
            
            (Status::Queued, Status::Running, true),
            (Status::Queued, Status::Stopped, true),
            (Status::Queued, Status::Terminated, true),
            (Status::Queued, Status::Error, true),
            (Status::Queued, Status::Idle, false),
            
            (Status::Running, Status::Finished, true),
            (Status::Running, Status::Stopped, true),
            (Status::Running, Status::Terminated, true),
            (Status::Running, Status::Error, true),
            (Status::Running, Status::Queued, false),
            
            (Status::Finished, Status::Error, true),
            (Status::Finished, Status::Idle, false),
            
            (Status::Error, Status::Idle, true),
            (Status::Error, Status::Queued, true),
            (Status::Error, Status::Running, false),
        ];

        for (from, to, should_succeed) in test_cases {
            let result = from.transition_to(to);
            if should_succeed {
                assert!(result.is_ok(), "Expected transition from {:?} to {:?} to succeed", from, to);
                assert_eq!(result.unwrap(), to);
            } else {
                assert!(result.is_err(), "Expected transition from {:?} to {:?} to fail", from, to);
            }
        }
    }

    #[test]
    fn test_status_round_trip_string_conversion() {
        for status in Status::all() {
            let as_string = status.as_str();
            let parsed = Status::from_str(as_string).unwrap();
            assert_eq!(status, parsed);
        }
    }
}
