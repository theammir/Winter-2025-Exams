// Merge two "dictionaries" (HashMaps) into one
// weird style preserved

use std::collections::HashMap;

type JSObject = HashMap<String, String>;

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
    use super::merge_objects;
    use std::collections::HashMap;

    #[test]
    fn test_merge() {
        let cases = vec![
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

    fn map(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        let mut m = HashMap::new();
        for (k, v) in pairs {
            m.insert(k.to_string(), v.to_string());
        }
        m
    }
}
