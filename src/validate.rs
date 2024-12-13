// Validate person name

pub fn is_name_valid(name: &str) -> bool {
    if name.is_empty() || !name.contains(' ') {
        return false;
    }

    name.chars()
        .filter(|c| *c != ' ')
        .all(|c| (97..=122).contains(&(c.to_ascii_lowercase() as u32)))
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
