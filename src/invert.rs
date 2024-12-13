// Reverse an array without using .reverse()

pub fn invert(data: &[i32]) -> Vec<i32> {
    let length = data.len();

    (0..length).map(|i| data[length - i - 1]).collect()
}

#[cfg(test)]
mod tests {
    use super::invert;

    #[test]
    fn test_invert() {
        let cases = vec![
            (vec![100, 200, 300, 400], vec![400, 300, 200, 100]),
            (vec![100, 0, -100], vec![-100, 0, 100]),
            (vec![0, 1], vec![1, 0]),
            (vec![1], vec![1]),
            (vec![], vec![]),
        ];
        for (input, expected) in cases {
            let output = invert(&input);
            assert_eq!(output, expected);
        }
    }
}
