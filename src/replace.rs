pub fn replace(haystack: String, needle: &str, substitute: &str) -> String {
    if needle.is_empty() {
        return haystack;
    }

    let mut source = haystack;
    let mut result = String::new();
    loop {
        if let Some(sub_start) = source.find(needle) {
            let left_split = source[..sub_start].to_string(); // Convert to owned string
            let right_split_start = sub_start + needle.len();

            source = source[right_split_start..].to_string();
            result.push_str(&left_split);
            result.push_str(substitute);
        } else {
            result.push_str(&source);
            return result;
        }
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
