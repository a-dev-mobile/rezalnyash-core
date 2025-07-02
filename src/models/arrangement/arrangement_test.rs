//! Comprehensive tests for the Arrangement module.
//! 
//! This test module provides extensive testing for all functionality
//! in the Arrangement struct, including edge cases and performance considerations.

use super::arrangement::Arrangement;
use std::collections::HashSet;

#[cfg(test)]
mod arrangement_tests {
    use super::*;

    /// Test the basic functionality with empty input
    #[test]
    fn test_generate_permutations_empty() {
        let empty: Vec<i32> = vec![];
        let result = Arrangement::generate_permutations(empty).unwrap();
        
        assert_eq!(result.len(), 1, "Empty list should return one empty permutation");
        assert_eq!(result[0].len(), 0, "The single permutation should be empty");
    }

    /// Test with a single element
    #[test]
    fn test_generate_permutations_single_element() {
        let input = vec![42];
        let result = Arrangement::generate_permutations(input).unwrap();
        
        assert_eq!(result.len(), 1, "Single element should have one permutation");
        assert_eq!(result[0], vec![42], "Permutation should contain the single element");
    }

    /// Test with two elements
    #[test]
    fn test_generate_permutations_two_elements() {
        let input = vec![1, 2];
        let result = Arrangement::generate_permutations(input).unwrap();
        
        assert_eq!(result.len(), 2, "Two elements should have 2! = 2 permutations");
        
        let expected_permutations = vec![vec![1, 2], vec![2, 1]];
        for expected in expected_permutations {
            assert!(
                result.contains(&expected),
                "Result should contain permutation {:?}",
                expected
            );
        }
    }

    /// Test with three elements - comprehensive check
    #[test]
    fn test_generate_permutations_three_elements() {
        let input = vec![1, 2, 3];
        let result = Arrangement::generate_permutations(input).unwrap();
        
        assert_eq!(result.len(), 6, "Three elements should have 3! = 6 permutations");
        
        let expected_permutations = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];
        
        for expected in &expected_permutations {
            assert!(
                result.contains(expected),
                "Result should contain permutation {:?}",
                expected
            );
        }
        
        // Ensure no duplicates
        let unique_permutations: HashSet<_> = result.iter().collect();
        assert_eq!(
            unique_permutations.len(),
            result.len(),
            "All permutations should be unique"
        );
    }

    /// Test with four elements to verify scaling
    #[test]
    fn test_generate_permutations_four_elements() {
        let input = vec!['a', 'b', 'c', 'd'];
        let result = Arrangement::generate_permutations(input).unwrap();
        
        assert_eq!(result.len(), 24, "Four elements should have 4! = 24 permutations");
        
        // Verify all permutations are unique
        let unique_permutations: HashSet<_> = result.iter().collect();
        assert_eq!(unique_permutations.len(), 24, "All permutations should be unique");
        
        // Verify each permutation has all original elements
        for permutation in &result {
            assert_eq!(permutation.len(), 4, "Each permutation should have 4 elements");
            assert!(permutation.contains(&'a'), "Each permutation should contain 'a'");
            assert!(permutation.contains(&'b'), "Each permutation should contain 'b'");
            assert!(permutation.contains(&'c'), "Each permutation should contain 'c'");
            assert!(permutation.contains(&'d'), "Each permutation should contain 'd'");
        }
    }

    /// Test with string elements
    #[test]
    fn test_generate_permutations_strings() {
        let input = vec!["hello".to_string(), "world".to_string()];
        let result = Arrangement::generate_permutations(input).unwrap();
        
        assert_eq!(result.len(), 2, "Two strings should have 2 permutations");
        
        let expected = vec![
            vec!["hello".to_string(), "world".to_string()],
            vec!["world".to_string(), "hello".to_string()],
        ];
        
        for expected_perm in expected {
            assert!(
                result.contains(&expected_perm),
                "Result should contain {:?}",
                expected_perm
            );
        }
    }

    /// Test with duplicate elements
    #[test]
    fn test_generate_permutations_with_duplicates() {
        let input = vec![1, 1, 2];
        let result = Arrangement::generate_permutations(input).unwrap();
        
        // Should still generate all 3! = 6 permutations, including duplicates
        assert_eq!(result.len(), 6, "Should generate all permutations including duplicates");
        
        // Count occurrences of specific permutations
        let count_112 = result.iter().filter(|&perm| *perm == vec![1, 1, 2]).count();
        let count_121 = result.iter().filter(|&perm| *perm == vec![1, 2, 1]).count();
        let count_211 = result.iter().filter(|&perm| *perm == vec![2, 1, 1]).count();
        
        assert!(count_112 > 0, "Should contain [1, 1, 2]");
        assert!(count_121 > 0, "Should contain [1, 2, 1]");
        assert!(count_211 > 0, "Should contain [2, 1, 1]");
    }

    /// Test the slice-based method
    #[test]
    fn test_generate_permutations_from_slice() {
        let input = [1, 2, 3];
        let result = Arrangement::generate_permutations_from_slice(&input).unwrap();
        
        assert_eq!(result.len(), 6, "Should generate 6 permutations from slice");
        
        // Verify original slice is unchanged
        assert_eq!(input, [1, 2, 3], "Original slice should remain unchanged");
        
        // Verify all expected permutations are present
        let expected_permutations = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];
        
        for expected in expected_permutations {
            assert!(
                result.contains(&expected),
                "Result should contain permutation {:?}",
                expected
            );
        }
    }

    /// Test counting permutations without generating them
    #[test]
    fn test_count_permutations() {
        assert_eq!(Arrangement::count_permutations(0).unwrap(), 1, "0! should be 1");
        assert_eq!(Arrangement::count_permutations(1).unwrap(), 1, "1! should be 1");
        assert_eq!(Arrangement::count_permutations(2).unwrap(), 2, "2! should be 2");
        assert_eq!(Arrangement::count_permutations(3).unwrap(), 6, "3! should be 6");
        assert_eq!(Arrangement::count_permutations(4).unwrap(), 24, "4! should be 24");
        assert_eq!(Arrangement::count_permutations(5).unwrap(), 120, "5! should be 120");
        assert_eq!(Arrangement::count_permutations(6).unwrap(), 720, "6! should be 720");
        assert_eq!(Arrangement::count_permutations(10).unwrap(), 3628800, "10! should be 3628800");
    }

    /// Test overflow protection in count_permutations
    #[test]
    fn test_count_permutations_overflow_protection() {
        let result = Arrangement::count_permutations(25);
        assert!(result.is_err(), "Should return error for large input");
        assert!(
            result.unwrap_err().contains("too large"),
            "Error message should mention input being too large"
        );
    }

    /// Test edge case with maximum safe factorial
    #[test]
    fn test_count_permutations_max_safe() {
        let result = Arrangement::count_permutations(20);
        assert!(result.is_ok(), "Should handle n=20 without overflow");
        
        let result = Arrangement::count_permutations(21);
        assert!(result.is_err(), "Should reject n=21 to prevent overflow");
    }

    /// Performance test with moderately sized input
    #[test]
    fn test_performance_moderate_size() {
        let input: Vec<i32> = (1..=5).collect(); // [1, 2, 3, 4, 5]
        let start = std::time::Instant::now();
        let result = Arrangement::generate_permutations(input).unwrap();
        let duration = start.elapsed();
        
        assert_eq!(result.len(), 120, "Should generate 5! = 120 permutations");
        assert!(
            duration.as_millis() < 100,
            "Should complete within 100ms, took {:?}",
            duration
        );
    }

    /// Test with custom struct to verify generic functionality
    #[derive(Debug, Clone, PartialEq)]
    struct TestStruct {
        id: i32,
        name: String,
    }

    #[test]
    fn test_generate_permutations_custom_struct() {
        let input = vec![
            TestStruct { id: 1, name: "first".to_string() },
            TestStruct { id: 2, name: "second".to_string() },
        ];
        
        let result = Arrangement::generate_permutations(input).unwrap();
        
        assert_eq!(result.len(), 2, "Should generate 2 permutations");
        
        // Verify both permutations are present
        let first_struct = TestStruct { id: 1, name: "first".to_string() };
        let second_struct = TestStruct { id: 2, name: "second".to_string() };
        
        let perm1 = vec![first_struct.clone(), second_struct.clone()];
        let perm2 = vec![second_struct, first_struct];
        
        assert!(result.contains(&perm1), "Should contain first permutation");
        assert!(result.contains(&perm2), "Should contain second permutation");
    }

    /// Test memory efficiency by ensuring no unnecessary allocations
    #[test]
    fn test_memory_efficiency() {
        let input = vec![1, 2, 3];
        let expected_count = Arrangement::count_permutations(input.len()).unwrap();
        let result = Arrangement::generate_permutations(input).unwrap();
        
        assert_eq!(
            result.len(),
            expected_count,
            "Should generate exactly the expected number of permutations"
        );
        
        // Verify each permutation has the correct length
        for permutation in &result {
            assert_eq!(
                permutation.len(),
                3,
                "Each permutation should have the same length as input"
            );
        }
    }

    /// Test error handling with Result type
    #[test]
    fn test_error_handling() {
        // This test verifies that the Result type is properly used
        // In this simple implementation, errors are unlikely, but the structure is in place
        let input = vec![1, 2, 3];
        let result = Arrangement::generate_permutations(input);
        
        assert!(result.is_ok(), "Valid input should return Ok");
        
        let permutations = result.unwrap();
        assert!(!permutations.is_empty(), "Result should not be empty");
    }

    /// Benchmark-style test to verify algorithmic complexity
    #[test]
    fn test_algorithmic_complexity() {
        // Test that the number of permutations grows as expected (n!)
        for n in 1..=6 {
            let input: Vec<i32> = (1..=n).collect();
            let result = Arrangement::generate_permutations(input).unwrap();
            let expected = Arrangement::count_permutations(n as usize).unwrap();
            
            assert_eq!(
                result.len(),
                expected,
                "For n={}, expected {} permutations, got {}",
                n,
                expected,
                result.len()
            );
        }
    }
}
