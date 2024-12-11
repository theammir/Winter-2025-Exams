// Generate random password
use rand::Rng;

pub fn generate_password(alphabet: &str, length: usize) -> String {
    let rng = rand::thread_rng();
    let alphabet_dist = rand::distributions::Uniform::from(0..alphabet.len());

    // Using a distribution is confusing, but should be more optimized for longer sequences

    String::from_iter(
        rng.sample_iter(alphabet_dist)
            .take(length)
            .map(|i| alphabet.chars().nth(i).unwrap()),
    )
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
