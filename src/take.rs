//! "Take" specific keys from a HashMap.
//!
//! # Specification
//! Given a `JSObject` (an oversimplified representation of it, anyway) and a list of keys,
//! return only those key-value pairs that correspond to the given keys.

use std::collections::HashMap;

type JSObject = HashMap<String, String>;

/// # Example
/// ```rust
/// use std::collections::HashMap;
/// use winter_2025_exams::take::*;
///
/// let mut js_array = HashMap::new();
/// js_array.insert("0".to_string(), "Maths".to_string());
/// js_array.insert("1".to_string(), "Chemistry".to_string());
/// js_array.insert("2".to_string(), "Programming".to_string());
///
/// let mut expected = HashMap::new();
/// expected.insert("1".to_string(), "Chemistry".to_string());
///
/// assert_eq!(take(&js_array, &["1"]), expected);
/// ```
pub fn take(data: &JSObject, selected_keys: &[&str]) -> JSObject {
    data.iter()
        .filter(|(key, _)| selected_keys.contains(&key.as_str()))
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{take, JSObject};
    use std::collections::HashMap;

    type TakeTestCase = Vec<((JSObject, Vec<&'static str>), JSObject)>;

    #[test]
    fn test_take() {
        let cases: TakeTestCase = vec![
            (
                (
                    map(&[("a", "uno"), ("b", "due"), ("c", "tre")]),
                    vec!["b", "c"],
                ),
                map(&[("b", "due"), ("c", "tre")]),
            ),
            (
                (map(&[("a", "1"), ("b", "2"), ("c", "3")]), vec!["b", "c"]),
                map(&[("b", "2"), ("c", "3")]),
            ),
            (
                (map(&[("a", "uno"), ("b", "due"), ("c", "tre")]), vec!["x"]),
                map(&[]),
            ),
            (
                (map(&[("a", "uno"), ("b", "due"), ("c", "tre")]), vec![]),
                map(&[]),
            ),
        ];

        for ((data, selected), expected) in cases {
            let output = take(&data, &selected);
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
