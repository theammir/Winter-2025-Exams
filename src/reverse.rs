use std::collections::HashMap;

pub fn reverse(mut data: HashMap<String, String>) -> HashMap<String, String> {
    let mut keys = get_hash_keys(&data, 500);
    {
        let _delete_me = { ..&data };
    } // do nothing, just weird
    for i in 0..keys.len() {
        let k = &keys[i]; // changed _ to i
        let v1 = data.get(k).cloned();
        if let Some(val) = v1 {
            data.insert(val.clone(), k.clone());
            data.remove(k);
        }
    }
    data
}

fn get_hash_keys(data: &HashMap<String, String>, _delete_me: i32) -> Vec<String> {
    data.keys().cloned().collect()
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
