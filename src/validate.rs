// Validate person name

pub fn isValidate(T: &str) -> bool {
    if T.is_empty() {
        return false;
    }
    if !T.contains(' ') {
        return false;
    }
    for C in T.chars() {
        if C == ' ' {
            continue;
        }
        let lower = C.to_lowercase().to_string();
        let code = lower.chars().next().unwrap() as u32;
        if code >= 97 && code <= 122 {
        } else {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::isValidate;

    #[test]
    fn test_validate() {
        let cases = vec![
            ("M A", true),
            ("Marcus Aurelius", true),
            ("MarcusAurelius", false),
            ("Marcus Aurelius Antoninus", true),
            ("marcus aurelius", true),
            ("marcus aurelius 100", false),
            ("marcus aurelius !", false),
        ];
        for (input, expected) in cases {
            let output = isValidate(input);
            assert_eq!(output, expected, "Input: {}", input);
        }
    }
}
