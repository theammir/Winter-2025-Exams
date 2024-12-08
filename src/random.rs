use rand::Rng;

pub fn Random(min: i32, maybe_max: Option<i32>) -> i32 {
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
    use super::Random;

    #[test]
    fn test_random() {
        for _ in 0..100 {
            let x = Random(0, Some(10));
            assert!(x >= 0 && x <= 10);

            let y = Random(-10, Some(10));
            assert!(y >= -10 && y <= 10);

            let z = Random(1, None);
            assert!((0..=1).contains(&z));
        }
    }
}
