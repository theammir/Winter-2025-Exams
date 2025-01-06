//! Merge two `HashMap`s.
//!
//! # Specification
//! Given two `HashMap`s, return such that contains key-value pairs from both of them.

use std::collections::HashMap;

type JSObject = HashMap<String, String>;

/// # Example
/// ```rust
/// use std::collections::HashMap;
/// use winter_2025_exams::merge::*;
///
/// let mut dave = HashMap::new();
/// dave.insert("sunglasses".to_string(), "false".to_string());
///
/// let mut cooler_dave = HashMap::new();
/// cooler_dave.insert("salary".to_string(), "$200k".to_string());
/// cooler_dave.insert("sunglasses".to_string(), "true".to_string());
///
/// let mut your_moms_expected = HashMap::new();
/// your_moms_expected.insert("sunglasses".to_string(), "true".to_string());
/// your_moms_expected.insert("salary".to_string(), "$200k".to_string());
///
/// assert_eq!(merge_objects(&dave, &cooler_dave), your_moms_expected);
/// ```
pub fn merge_objects(a: &JSObject, b: &JSObject) -> JSObject {
    let mut result: JSObject = HashMap::new();
    for (key, value) in a {
        result.insert(key.to_string(), value.to_string());
    }
    for (key, value) in b {
        result.insert(key.to_string(), value.to_string());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::{merge_objects, JSObject};
    use std::collections::HashMap;

    #[test]
    fn test_merge() {
        let cases = [
            (
                (map(&[("a", "uno"), ("b", "due")]), map(&[("c", "tre")])),
                map(&[("a", "uno"), ("b", "due"), ("c", "tre")]),
            ),
            (
                (map(&[("a", "uno"), ("b", "due")]), map(&[("a", "uno")])),
                map(&[("a", "uno"), ("b", "due")]),
            ),
            (
                (map(&[("a", "uno"), ("b", "due")]), map(&[("a", "due")])),
                map(&[("a", "due"), ("b", "due")]),
            ),
            (
                (map(&[("a", "uno")]), map(&[("c", "tre")])),
                map(&[("a", "uno"), ("c", "tre")]),
            ),
            ((map(&[("a", "uno")]), map(&[])), map(&[("a", "uno")])),
            ((map(&[]), map(&[])), map(&[])),
        ];

        for ((a, b), expected) in cases {
            let output = merge_objects(&a, &b);
            assert_eq!(output, expected);
        }
    }

    fn map(pairs: &[(&str, &str)]) -> JSObject {
        let mut m = HashMap::new();
        for (k, v) in pairs {
            m.insert(k.to_string(), v.to_string());
        }
        m
    }
}
