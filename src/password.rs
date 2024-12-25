//! Generate a random password.
//!
//! # Specification
//! Generate a random sequence of given length using characters from the specified alphabet.

use rand::Rng;

/// # Example
/// ```rust
/// use winter_2025_exams::password::*;
///
/// assert_eq!(generate_password("alphbet", 100).len(), 100);
/// ```
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

        let p1 = generate_password(chars, 7);
        assert_eq!(p1.len(), 7);

        let p2 = generate_password(chars, 7);
        assert_eq!(p2.len(), 7);
        for c in p2.chars() {
            assert!(chars.contains(c));
        }
    }
}
