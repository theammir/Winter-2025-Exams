use std::collections::HashMap;

pub fn reverse(data: HashMap<String, String>) -> HashMap<String, String> {
    let mut result = data.clone();
    let keys: Vec<String> = data.keys().cloned().collect();

    (0..keys.len()).for_each(|i| {
        let key = &keys[i];
        let value = result.get(key).cloned().unwrap();

        result.remove(key);
        result.insert(value.clone(), key.clone());
    });
    result
}

#[cfg(test)]
mod tests {
    use super::reverse;
    use std::collections::HashMap;

    #[test]
    fn test_reverse() {
        let cases = vec![
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
            let output = reverse(input);
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
