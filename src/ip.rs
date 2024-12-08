// Parse IP: return [u8;4] as Vec<i32>, or None

pub fn Parseip(i: &str) -> Option<Vec<i32>> {
    let mut a = vec![];
    if i == "" {
        return None;
    } else {
        let B: Vec<&str> = i.split('.').collect();
        if B.len() != 4 {
            return None;
        }
        let mut j = 0;
        for b in B {
            match b.parse::<i32>() {
                Ok(val) => a.push(val),
                Err(_) => return None,
            }
            j += 1;
        }
    }
    Some(a)
}

#[cfg(test)]
mod tests {
    use super::Parseip;

    #[test]
    fn test_ip() {
        let cases = vec![
            ("127.0.0.1", Some(vec![127, 0, 0, 1])),
            ("0.0.0.0", Some(vec![0, 0, 0, 0])),
            ("255.255.255.0", Some(vec![255, 255, 255, 0])),
            ("10.0.0.10", Some(vec![10, 0, 0, 10])),
            (".0.0.", None),
            ("127001", None),
            ("127.0.0", None),
            ("", None),
        ];
        for (input, expected) in cases {
            let output = Parseip(input);
            assert_eq!(output, expected);
        }
    }
}
