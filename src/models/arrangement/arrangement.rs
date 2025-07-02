//! Arrangement module for generating permutations of collections.
//! 
//! This module provides functionality to generate all possible permutations
//! of a given collection, maintaining functional equivalence with the original
//! Java implementation while leveraging Rust's ownership system and error handling.

use std::fmt::Debug;

/// Arrangement utility struct for generating permutations.
/// 
/// This struct provides static methods for generating all possible permutations
/// of a given vector. The implementation uses Rust's ownership system to ensure
/// memory safety while maintaining the same algorithmic approach as the original Java code.
pub struct Arrangement;

impl Arrangement {
    /// Generates all permutations of the given vector.
    /// 
    /// This function creates all possible arrangements of the elements in the input vector.
    /// The algorithm works recursively by:
    /// 1. Removing the first element from the input
    /// 2. Generating permutations of the remaining elements
    /// 3. Inserting the removed element at all possible positions in each permutation
    /// 
    /// # Arguments
    /// 
    /// * `list` - A vector of elements to permute. The vector is consumed by this function.
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(Vec<Vec<T>>)` containing all permutations, or `Err(String)` if an error occurs.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crate::models::arrangement::Arrangement;
    /// 
    /// let input = vec![1, 2, 3];
    /// let result = Arrangement::generate_permutations(input).unwrap();
    /// assert_eq!(result.len(), 6); // 3! = 6 permutations
    /// ```
    /// 
    /// # Type Constraints
    /// 
    /// The type `T` must implement `Clone` to allow creating copies of elements
    /// when building permutations, and `Debug` for error reporting.
    pub fn generate_permutations<T>(mut list: Vec<T>) -> Result<Vec<Vec<T>>, String>
    where
        T: Clone + Debug,
    {
        // Base case: empty list returns a single empty permutation
        if list.is_empty() {
            return Ok(vec![vec![]]);
        }

        // Remove the first element (equivalent to list.remove(0) in Java)
        let removed_element = list.remove(0);
        
        // Generate permutations of the remaining elements
        let sub_permutations = Self::generate_permutations(list)
            .map_err(|e| format!("Failed to generate sub-permutations: {}", e))?;
        
        let mut result = Vec::new();
        
        // For each permutation of the remaining elements
        for permutation in sub_permutations {
            // Insert the removed element at each possible position
            for i in 0..=permutation.len() {
                let mut new_permutation = permutation.clone();
                new_permutation.insert(i, removed_element.clone());
                result.push(new_permutation);
            }
        }
        
        Ok(result)
    }

    /// Generates all permutations of the given slice without consuming it.
    /// 
    /// This is a convenience method that clones the input slice before generating
    /// permutations, allowing the caller to retain ownership of the original data.
    /// 
    /// # Arguments
    /// 
    /// * `slice` - A slice of elements to permute
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(Vec<Vec<T>>)` containing all permutations, or `Err(String)` if an error occurs.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crate::models::arrangement::Arrangement;
    /// 
    /// let input = [1, 2, 3];
    /// let result = Arrangement::generate_permutations_from_slice(&input).unwrap();
    /// assert_eq!(result.len(), 6);
    /// // Original slice is still available
    /// println!("{:?}", input);
    /// ```
    pub fn generate_permutations_from_slice<T>(slice: &[T]) -> Result<Vec<Vec<T>>, String>
    where
        T: Clone + Debug,
    {
        Self::generate_permutations(slice.to_vec())
    }

    /// Calculates the number of permutations for a given length without generating them.
    /// 
    /// This is a utility method that calculates n! (factorial) for the given length,
    /// which represents the total number of permutations possible.
    /// 
    /// # Arguments
    /// 
    /// * `n` - The length of the collection
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(usize)` with the factorial value, or `Err(String)` if overflow occurs.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crate::models::arrangement::Arrangement;
    /// 
    /// assert_eq!(Arrangement::count_permutations(3).unwrap(), 6);
    /// assert_eq!(Arrangement::count_permutations(4).unwrap(), 24);
    /// ```
    pub fn count_permutations(n: usize) -> Result<usize, String> {
        if n > 20 {
            return Err(format!("Input too large ({}), would cause overflow", n));
        }
        
        let mut result = 1usize;
        for i in 2..=n {
            result = result.checked_mul(i)
                .ok_or_else(|| format!("Factorial overflow at {}", i))?;
        }
        
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_list() {
        let empty: Vec<i32> = vec![];
        let result = Arrangement::generate_permutations(empty).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 0);
    }

    #[test]
    fn test_single_element() {
        let input = vec![42];
        let result = Arrangement::generate_permutations(input).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], vec![42]);
    }

    #[test]
    fn test_two_elements() {
        let input = vec![1, 2];
        let result = Arrangement::generate_permutations(input).unwrap();
        assert_eq!(result.len(), 2);
        assert!(result.contains(&vec![1, 2]));
        assert!(result.contains(&vec![2, 1]));
    }

    #[test]
    fn test_three_elements() {
        let input = vec![1, 2, 3];
        let result = Arrangement::generate_permutations(input).unwrap();
        assert_eq!(result.len(), 6);
        
        let expected = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];
        
        for perm in expected {
            assert!(result.contains(&perm), "Missing permutation: {:?}", perm);
        }
    }

    #[test]
    fn test_string_permutations() {
        let input = vec!["a".to_string(), "b".to_string()];
        let result = Arrangement::generate_permutations(input).unwrap();
        assert_eq!(result.len(), 2);
        assert!(result.contains(&vec!["a".to_string(), "b".to_string()]));
        assert!(result.contains(&vec!["b".to_string(), "a".to_string()]));
    }

    #[test]
    fn test_from_slice() {
        let input = [1, 2, 3];
        let result = Arrangement::generate_permutations_from_slice(&input).unwrap();
        assert_eq!(result.len(), 6);
        // Original slice should still be accessible
        assert_eq!(input, [1, 2, 3]);
    }

    #[test]
    fn test_count_permutations() {
        assert_eq!(Arrangement::count_permutations(0).unwrap(), 1);
        assert_eq!(Arrangement::count_permutations(1).unwrap(), 1);
        assert_eq!(Arrangement::count_permutations(2).unwrap(), 2);
        assert_eq!(Arrangement::count_permutations(3).unwrap(), 6);
        assert_eq!(Arrangement::count_permutations(4).unwrap(), 24);
        assert_eq!(Arrangement::count_permutations(5).unwrap(), 120);
    }

    #[test]
    fn test_count_permutations_overflow() {
        let result = Arrangement::count_permutations(25);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("too large"));
    }

    #[test]
    fn test_duplicate_elements() {
        let input = vec![1, 1, 2];
        let result = Arrangement::generate_permutations(input).unwrap();
        assert_eq!(result.len(), 6); // Still generates all permutations, including duplicates
        
        // Should contain duplicates like [1, 1, 2], [1, 2, 1], [1, 1, 2], etc.
        let count_112 = result.iter().filter(|&perm| *perm == vec![1, 1, 2]).count();
        assert!(count_112 > 0);
    }
}
