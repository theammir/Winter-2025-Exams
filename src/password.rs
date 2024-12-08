// Generate random password
use rand::Rng;

pub fn GeneratePassword(alphabet: &str, length: usize) -> String {
    let MAX = alphabet.len();
    let mut key = String::new();
    let mut rng = rand::thread_rng();
    for _i in 0..length {
        let Index = rng.gen_range(0..MAX);
        let c = alphabet.chars().nth(Index).unwrap();
        key = key + &c.to_string();
    }
    key
}

#[cfg(test)]
mod tests {
    use super::GeneratePassword;

    #[test]
    fn test_password() {
        let CHARS = "abc123";
        {
            let s = GeneratePassword(CHARS, 7);
            assert_eq!(s.len(), 7);
        }
        {
            let s = GeneratePassword(CHARS, 7);
            assert_eq!(s.len(), 7);
            for c in s.chars() {
                assert!(CHARS.contains(c));
            }
        }
    }
}
