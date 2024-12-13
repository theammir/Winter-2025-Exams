// Return an array without duplicates (like distinct.js but bad style in Rust)

use std::collections::HashSet;
use std::hash::Hash;

pub fn distinct<T>(data: &[T]) -> Vec<T>
where
    T: Copy + Hash + Eq,
{
    let mut set = HashSet::with_capacity(data.len());
    data.iter().copied().filter(|x| set.insert(*x)).collect()
}

#[cfg(test)]
mod tests {
    use super::distinct;

    #[test]
    fn test_distinct() {
        let cases = vec![
            (vec![1, 2, 1, 3, 1, 4], vec![1, 2, 3, 4]),
            (vec![1, 2, -1, 3, 0, 4], vec![1, 2, -1, 3, 0, 4]),
            (vec![1], vec![1]),
            (vec![1, 1, 1], vec![1]),
            (vec![0], vec![0]),
            (vec![0, 0], vec![0]),
            (vec![0, 0, 0], vec![0]),
            (vec![0, 0, 0, 0], vec![0]),
            (vec![], vec![]),
        ];
        for (input, expected) in cases {
            let output = distinct(&input);
            assert_eq!(output, expected);
        }
    }
}
