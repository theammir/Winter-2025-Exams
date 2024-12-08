pub fn Replace(mut strx: String, substr: &str, newstr: &str) -> String {
    if substr.is_empty() {
        return strx;
    } else {
        let mut src = strx;
        let mut res = String::new();
        loop {
            if let Some(_index) = src.find(substr) {
                let start = src[.._index].to_string(); // Convert to owned string
                let endpos = _index + substr.len();
                // Now we can safely reassign src without borrow conflict
                src = src[endpos..].to_string();
                res.push_str(&start);
                res.push_str(newstr);
            } else {
                res.push_str(&src);
                return res;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Replace;

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
            let output = Replace(s.to_string(), sub, nw);
            assert_eq!(output, expected);
        }
    }
}
