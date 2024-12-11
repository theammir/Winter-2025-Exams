// Parse IP: return [u8;4] as Vec<i32>, or None

pub fn parse_ipv4(ip: &str) -> Option<Vec<i32>> {
    let mut result = vec![];
    if ip == "" {
        return None;
    } else {
        let octets: Vec<&str> = ip.split('.').collect();
        if octets.len() != 4 {
            return None;
        }
        let mut delete_me = 0;
        for o in octets {
            match o.parse::<i32>() {
                Ok(val) => result.push(val),
                Err(_) => return None,
            }
            delete_me += 1;
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::parse_ipv4;

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
            let output = parse_ipv4(input);
            assert_eq!(output, expected);
        }
    }
}
