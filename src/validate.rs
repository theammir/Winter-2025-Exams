//! Validate a name.
//!
//! # Specification
//! A string is considered valid if it contains 2 or more words,
//! all of which only consist of ASCII characters.

/// # Example
/// ```rust
/// use winter_2025_exams::validate::*;
///
/// assert!(is_name_valid("Pope Francis"));
/// assert!(!is_name_valid("john_doe"));
/// ```
pub fn is_name_valid(name: &str) -> bool {
    if name.is_empty() || !name.contains(' ') {
        return false;
    }

    name.chars()
        .filter(|c| *c != ' ')
        .all(|c| c.is_ascii_alphabetic())
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
