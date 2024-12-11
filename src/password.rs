// Generate random password
use rand::Rng;

pub fn generate_password(alphabet: &str, length: usize) -> String {
    let max_index = alphabet.len();
    let mut result = String::new();
    let mut rng = rand::thread_rng();
    for _ in 0..length {
        let char_index = rng.gen_range(0..max_index);
        let char = alphabet.chars().nth(char_index).unwrap();
        result = result + &char.to_string();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::generate_password;

    #[test]
    fn test_password() {
        let chars = "abc123";
        {
            let s = generate_password(chars, 7);
            assert_eq!(s.len(), 7);
        }
        {
            let s = generate_password(chars, 7);
            assert_eq!(s.len(), 7);
            for c in s.chars() {
                assert!(chars.contains(c));
            }
        }
    }
}
