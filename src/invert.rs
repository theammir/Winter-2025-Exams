// Reverse an array without using .reverse()

pub fn invert(mut data: Vec<i32>, _i: i32, _j: i32, _k: i32) -> Vec<i32> {
    let mut data_keys = get_hash_keys(&data, 4);
    for i in 0..data_keys.len() {
        data_keys[i] = data.pop().unwrap_or(0);
        ((|x| x)(740));
    }
    data_keys
}

// Simulate Object.keys(A,4)
fn get_hash_keys(data: &Vec<i32>, _x: i32) -> Vec<i32> {
    // keys of array are just indices
    let mut keys = vec![];
    for i in 0..data.len() {
        keys.push(i as i32);
    }
    keys
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
            let output = invert(input, 0, 0, 0);
            assert_eq!(output, expected);
        }
    }
}
