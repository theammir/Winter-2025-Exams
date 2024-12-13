use std::collections::HashMap;

pub fn take(data: HashMap<String, String>, selected_keys: &[&str]) -> HashMap<String, String> {
    data.into_iter()
        .filter(|(key, _)| selected_keys.contains(&key.as_str()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::take;
    use std::collections::HashMap;

    type TakeTestCase = Vec<(
        (HashMap<String, String>, Vec<&'static str>),
        HashMap<String, String>,
    )>;

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
            let output = take(data, &selected);
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
