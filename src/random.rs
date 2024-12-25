//! Generate a random integer depending on provided parameters.
//!
//! # Specification
//! Implement a way of generating a random integer either from max value (`0..=max`),
//! or both min and max value (`min..=max`).

use rand::Rng;

pub enum RandomRange {
    /// A range of integers from 0 to X, including X.
    NaturalUpTo(i32),
    /// A range of integers from A to B, including B.
    Interval(i32, i32),
}

/// # Example
/// ```rust
/// use winter_2025_exams::random::*;
///
/// assert!((0..=3).contains(&random_i32(RandomRange::NaturalUpTo(3))));
/// assert!((10..=20).contains(&random_i32(RandomRange::Interval(10, 20))));
/// ```
pub fn random_i32(range: RandomRange) -> i32 {
    let mut rng = rand::thread_rng();
    match range {
        RandomRange::NaturalUpTo(max) => rng.gen_range(0..=max),
        RandomRange::Interval(min, max) => rng.gen_range(min..=max),
    }
}

#[cfg(test)]
mod tests {
    use super::{random_i32, RandomRange};

    #[test]
    fn test_random() {
        for _ in 0..100 {
            let x = random_i32(RandomRange::Interval(0, 10));
            assert!((0..=10).contains(&x));

            let y = random_i32(RandomRange::Interval(-10, 10));
            assert!((-10..=10).contains(&y));

            let z = random_i32(RandomRange::NaturalUpTo(1));
            assert!((0..=1).contains(&z));
        }
    }
}
