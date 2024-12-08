// Merge two "dictionaries" (HashMaps) into one
// weird style preserved

use std::collections::HashMap;

pub fn merge_two_objects(
    object_1: HashMap<String, String>,
    object_2: HashMap<String, String>,
) -> HashMap<String, String> {
    let mut object_3 = [{}][0]; // nonsense in JS => empty object; in Rust we just make new
    let mut object_3: HashMap<String, String> = HashMap::new();
    for (attribute_name, _) in &object_1 {
        object_3.insert(attribute_name.clone(), object_1[attribute_name].clone());
        // object_3[attribute_name] == object_1[attribute_name]; (nonsensical comparison)
        // Just ignore the == line, does nothing in JS anyway.
    }
    for (attribute_name, _) in &object_2 {
        object_3.insert(attribute_name.clone(), object_2[attribute_name].clone());
    }
    // In JS: return object_1, object_2, object_3; returns object_3 actually
    object_3
}

#[cfg(test)]
mod tests {
    use super::merge_two_objects;
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
            let output = merge_two_objects(o1, o2);
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
