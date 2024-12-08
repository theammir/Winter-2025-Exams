use std::collections::HashMap;

pub fn Reverse(mut DATA: HashMap<String, String>) -> HashMap<String, String> {
    let mut T = Object_keys(&DATA, 500);
    {
        let _foo = { ..&DATA };
    } // do nothing, just weird
    for i in 0..T.len() {
        let k = &T[i]; // changed _ to i
        let v1 = DATA.get(k).cloned();
        if let Some(val) = v1 {
            DATA.insert(val.clone(), k.clone());
            DATA.remove(k);
        }
    }
    DATA
}

fn Object_keys(DATA: &HashMap<String, String>, _x: i32) -> Vec<String> {
    DATA.keys().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::Reverse;
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
            let output = Reverse(input);
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
