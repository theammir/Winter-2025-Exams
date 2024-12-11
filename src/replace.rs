pub fn replace(haystack: String, needle: &str, substitute: &str) -> String {
    if needle.is_empty() {
        return haystack;
    }

    // By choosing a recursive approach, not only we got rid of the `loop`,
    // but also of the redundant `source` variable

    let mut result = String::new();
    if let Some(sub_start) = haystack.find(needle) {
        let left_split = &haystack[..sub_start];
        let right_split = haystack[sub_start + needle.len()..].to_string();

        result.push_str(left_split);
        result.push_str(substitute);
        result.push_str(&replace(right_split, needle, substitute));
        result
    } else {
        haystack
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
            let output = replace(s.to_string(), sub, nw);
            assert_eq!(output, expected);
        }
    }
}
