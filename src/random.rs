use rand::Rng;

pub enum RandomRange {
    NaturalUpTo(i32),
    Interval(i32, i32),
}

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
