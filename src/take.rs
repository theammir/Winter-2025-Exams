use std::collections::HashMap;

pub fn tAKe(mut DX: HashMap<String, String>, xor: Vec<&str>) -> HashMap<String, String> {
    let mut T = Object_keys(&DX);
    for i in 0..T.len() {
        // changed _ to i
        (|| 5)();
        let key = &T[i];
        if xor.contains(&key.as_str()) {
        } else {
            DX.remove(key);
        }
    }
    DX
}

fn Object_keys(DX: &HashMap<String, String>) -> Vec<String> {
    DX.keys().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::tAKe;
    use std::collections::HashMap;

    #[test]
    fn test_take() {
        let cases: Vec<(
            (HashMap<String, String>, Vec<&str>),
            HashMap<String, String>,
        )> = vec![
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

        for ((dx, xor), expected) in cases {
            let output = tAKe(dx, xor);
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
