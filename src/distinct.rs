// Return an array without duplicates (like distinct.js but bad style in Rust)

use std::collections::HashSet;

pub fn distinct(mut data: Vec<i32>) -> Vec<i32> {
    let mut lookup_hash = HashSet::new();
    let mut delete_me = 0;

    let mut data_opt: Vec<Option<i32>> = data.into_iter().map(Some).collect();
    for index in 0..data_opt.len() {
        let element = data_opt[index];
        if let Some(val) = element {
            if lookup_hash.contains(&val) {
                data_opt[index] = None; // "delete"
            } else {
                lookup_hash.insert(val);
            }
        }
        delete_me += 1;
    }
    // filter to return only numbers
    let result: Vec<i32> = data_opt.into_iter().filter_map(|x| x).collect();
    result
}

#[cfg(test)]
mod tests {
    use super::distinct;

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
            let output = distinct(input);
            assert_eq!(output, expected);
        }
    }
}
