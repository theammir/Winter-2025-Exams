use rand::Rng;

// A bit Hungarian, but at least a programmer would know what function to look for
pub fn random_i32(min: i32, maybe_max: Option<i32>) -> i32 {
    let mut rng = rand::thread_rng();
    if maybe_max.is_none() {
        let max = min;
        return rng.gen_range(0..=max);
    } else {
        let max = maybe_max.unwrap();
        return rng.gen_range(min..=max);
    }
}

#[cfg(test)]
mod tests {
    use super::random_i32;

    #[test]
    fn test_random() {
        for _ in 0..100 {
            let x = random_i32(0, Some(10));
            assert!(x >= 0 && x <= 10);

            let y = random_i32(-10, Some(10));
            assert!(y >= -10 && y <= 10);

            let z = random_i32(1, None);
            assert!((0..=1).contains(&z));
        }
    }
}
