// Reverse an array without using .reverse()

pub fn invert<T: Copy>(data: &[T]) -> Vec<T> {
    let length = data.len();

    (0..length).map(|i| data[length - i - 1]).collect()
}

#[cfg(test)]
mod tests {
    use super::invert;

    #[test]
    fn test_invert() {
        let cases_int = [
            (vec![100, 200, 300, 400], vec![400, 300, 200, 100]),
            (vec![100, 0, -100], vec![-100, 0, 100]),
            (vec![0, 1], vec![1, 0]),
            (vec![1], vec![1]),
            (vec![], vec![]),
        ];
        for (input, expected) in cases_int {
            let output = invert(&input);
            assert_eq!(output, expected);
        }

        let cases_str = [
            (vec!["a", "b", "C"], vec!["C", "b", "a"]),
            (vec!["~", "~"], vec!["~", "~"]),
            (vec![], vec![]),
        ];
        for (input, expected) in cases_str {
            let output = invert(&input);
            assert_eq!(output, expected);
        }
    }
}
