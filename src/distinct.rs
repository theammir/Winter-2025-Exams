//! Remove duplicates from an array.
//!
//! # Specification
//! Given an array, return such that contains all the unique elements of the input.

use std::collections::HashSet;
use std::hash::Hash;

/// This implementation utilizes a `HashMap`, making it useless for unhashable `T`, such as `f32`.
///
/// # Example
/// ```rust
/// use winter_2025_exams::distinct::*;
///
/// assert_eq!(distinct(&[1, 2, 1, 2, 3]), vec![1, 2, 3]);
/// assert_eq!(distinct::<()>(&[]), vec![]);
/// ```
pub fn distinct<T>(data: &[T]) -> Vec<T>
where
    T: Copy + Hash + Eq,
{
    let mut set = HashSet::with_capacity(data.len());
    data.iter().copied().filter(|x| set.insert(*x)).collect()
}

#[cfg(test)]
mod tests {
    use super::distinct;

    #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[test]
    fn test_distinct() {
        let cases_int = [
            (vec![1, 2, 1, 3, 1, 4], vec![1, 2, 3, 4]),
            (vec![1, 2, -1, 3, 0, 4], vec![1, 2, -1, 3, 0, 4]),
            (vec![1], vec![1]),
            (vec![1, 1, 1], vec![1]),
            (vec![0], vec![0]),
            (vec![0, 0], vec![0]),
            (vec![0, 0, 0], vec![0]),
            (vec![0, 0, 0, 0], vec![0]),
            (vec![], vec![]),
        ];
        for (input, expected) in cases_int {
            let output = distinct(&input);
            assert_eq!(output, expected);
        }

        let cases_struct = [
            Point { x: 1, y: 2 },
            Point { x: 3, y: 4 },
            Point { x: 1, y: 2 },
            Point { x: 5, y: 6 },
            Point { x: 3, y: 4 },
        ];
        let expected = vec![
            Point { x: 1, y: 2 },
            Point { x: 3, y: 4 },
            Point { x: 5, y: 6 },
        ];
        assert_eq!(distinct(&cases_struct), expected);
    }
}
