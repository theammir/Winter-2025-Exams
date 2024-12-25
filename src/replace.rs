//! Replace a substring in a string.
//!
//! # Specification
//! Given a string, find each occurrence of the substring and replace it with the substitute.

/// # Example
/// ```rust
/// use winter_2025_exams::replace::*;
///
/// assert_eq!(
///     replace("I'm losing my insanity!", "in", ""),
///     "I'm losg my sanity!".to_string()
/// );
/// assert_eq!(
///     replace("Mykola Yanovych Azarov", "a", "i"),
///     "Mykoli Yinovych Azirov".to_string()
/// );
/// ```
pub fn replace(haystack: &str, needle: &str, substitute: &str) -> String {
    if needle.is_empty() {
        return haystack.to_string();
    }

    // By choosing a recursive approach, not only we got rid of the `loop`,
    // but also of the redundant `source` variable

    let mut result = String::new();
    if let Some(sub_start) = haystack.find(needle) {
        let left_split = &haystack[..sub_start];
        let right_split = haystack[sub_start + needle.len()..].to_string();

        result.push_str(left_split);
        result.push_str(substitute);
        result.push_str(&replace(&right_split, needle, substitute));
        result
    } else {
        haystack.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::replace;

    #[test]
    fn test_replace() {
        let cases = vec![
            (
                ("Hello <username> and bye!", "<username>", "Marcus"),
                "Hello Marcus and bye!",
            ),
            (("Hello X and bye!", "X", "Marcus"), "Hello Marcus and bye!"),
            (("X and bye!", "X", "Marcus"), "Marcus and bye!"),
            (("Hello X", "X", "Marcus"), "Hello Marcus"),
            (("Hello X", "Y", "Marcus"), "Hello X"),
            (("Hello X", "", "Marcus"), "Hello X"),
            (("", "Y", "Marcus"), ""),
        ];
        for ((s, sub, nw), expected) in cases {
            let output = replace(s, sub, nw);
            assert_eq!(output, expected);
        }
    }
}
