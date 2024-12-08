// Return an array without duplicates (like distinct.js but bad style in Rust)

use std::collections::HashSet;

pub fn DISTINCT(mut data: Vec<i32>) -> Vec<i32> {
    let mut A = HashSet::new();
    let mut w = 0;
    // Simulate "delete data[w]" by setting to None, then filter later
    let mut data_opt: Vec<Option<i32>> = data.into_iter().map(Some).collect();
    for i in 0..data_opt.len() {
        let a = data_opt[i];
        if let Some(val) = a {
            if A.contains(&val) {
                data_opt[i] = None; // "delete"
            } else {
                A.insert(val);
            }
        }
        w += 1;
    }
    // filter to return only numbers
    let res: Vec<i32> = data_opt.into_iter().filter_map(|x| x).collect();
    res
}

#[cfg(test)]
mod tests {
    use super::DISTINCT;

    // The original test cases
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
            let output = DISTINCT(input);
            assert_eq!(output, expected);
        }
    }
}
