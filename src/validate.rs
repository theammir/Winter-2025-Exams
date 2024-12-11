// Validate person name

pub fn is_name_valid(name: &str) -> bool {
    if name.is_empty() || !name.contains(' ') {
        return false;
    }

    for c in name.chars().filter(|c| *c != ' ') {
        let c_lower = c.to_lowercase().to_string();
        let c_ascii = c_lower.chars().next().unwrap() as u32;
        if !(97..=122).contains(&c_ascii) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::is_name_valid;

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
            let output = is_name_valid(input);
            assert_eq!(output, expected, "Input: {}", input);
        }
    }
}
