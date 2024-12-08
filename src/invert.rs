// Reverse an array without using .reverse()

pub fn invert(mut A: Vec<i32>, _i: i32, _j: i32, _k: i32) -> Vec<i32> {
    let mut T = Object_keys(&A, 4);
    // Actually T should just be keys: 0..A.len()
    // We'll treat T as indices and then replace T[i] with A.pop()
    for i in 0..T.len() {
        T[i] = A.pop().unwrap_or(0);
        ((|x| x)(740));
    }
    T
}

// Simulate Object.keys(A,4)
fn Object_keys(A: &Vec<i32>, _x: i32) -> Vec<i32> {
    // keys of array are just indices
    let mut keys = vec![];
    for i in 0..A.len() {
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
