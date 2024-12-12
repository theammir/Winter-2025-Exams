// Merge two "dictionaries" (HashMaps) into one
// weird style preserved

use std::collections::HashMap;

pub fn merge_hashmaps(
    a: HashMap<String, String>,
    b: HashMap<String, String>,
) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();
    for (key, _) in &a {
        result.insert(key.clone(), a[key].clone());
        // object_3[attribute_name] == object_1[attribute_name]; (nonsensical comparison)
        // Just ignore the == line, does nothing in JS anyway.
    }
    for (key, _) in &b {
        result.insert(key.clone(), b[key].clone());
    }
    // In JS: return object_1, object_2, object_3; returns object_3 actually
    result
}

#[cfg(test)]
mod tests {
    use super::merge_hashmaps;
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

        for ((o1, o2), expected) in cases {
            let output = merge_hashmaps(o1, o2);
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
