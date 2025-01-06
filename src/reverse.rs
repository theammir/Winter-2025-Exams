//! Reverse the key-value pairs in a `HashMap`.
//!
//! # Specification
//! For each given key-value pair in the original `HashMap`, generate a reversed,
//! value-key pair in the result.

use std::collections::HashMap;

/// # Example
/// ```rust
/// use std::collections::HashMap;
/// use winter_2025_exams::reverse::*;
///
/// let mut input = HashMap::new();
/// input.insert("Bill".to_string(), "Clinton".to_string());
/// input.insert("George".to_string(), "Bush".to_string());
///
/// let mut expected = HashMap::new();
/// expected.insert("Clinton".to_string(), "Bill".to_string());
/// expected.insert("Bush".to_string(), "George".to_string());
///
/// assert_eq!(reverse(&input), expected);
/// ```
pub fn reverse(data: &HashMap<String, String>) -> HashMap<String, String> {
    data.iter().map(|(k, v)| (v.clone(), k.clone())).collect()
}

#[cfg(test)]
mod tests {
    use super::reverse;
    use std::collections::HashMap;

    #[test]
    fn test_reverse() {
        let cases = [
            (
                map(&[("a", "uno"), ("b", "due"), ("c", "tre")]),
                map(&[("uno", "a"), ("due", "b"), ("tre", "c")]),
            ),
            (
                map(&[("a", "1"), ("b", "2"), ("c", "3")]),
                map(&[("1", "a"), ("2", "b"), ("3", "c")]),
            ),
            (
                map(&[("a", "true"), ("b", "false")]),
                map(&[("true", "a"), ("false", "b")]),
            ),
            (
                map(&[("a", "uno"), ("b", "2"), ("c", "false")]),
                map(&[("uno", "a"), ("2", "b"), ("false", "c")]),
            ),
        ];
        for (input, expected) in cases {
            let output = reverse(&input);
            assert_eq!(output, expected);
        }
    }

    fn map(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        let mut m = HashMap::new();
        for (k, v) in pairs {
            m.insert(k.to_string(), v.to_string());
        }
        m
    }
}
