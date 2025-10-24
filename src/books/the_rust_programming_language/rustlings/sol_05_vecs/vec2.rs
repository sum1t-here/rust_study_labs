fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for element in input {
        // TODO: Multiply each element in the `input` slice by 2 and push it to
        // the `output` vector.
        output.push(*element * 2);
    }

    output
}

pub fn run() {
    // You can optionally experiment here.
    let inp = [1, 2, 3, 4];
    println!("{:?}", inp);

    println!("{:?}", vec_loop(&inp));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_loop(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }
}
