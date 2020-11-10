pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    a.iter()
        .map(|row| {
            row.iter()
                .rev()
                .map(|n| n ^ 1)
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let a = vec![vec![1,1,0], vec![1,0,1], vec![0,0,0]];
        let expected = vec![vec![1,0,0], vec![0,1,0], vec![1,1,1]];

        assert_eq!(expected, flip_and_invert_image(a));
    }

    #[test]
    fn test_example_2() {
        let a = vec![vec![1,1,0,0],vec![1,0,0,1],vec![0,1,1,1],vec![1,0,1,0]];
        let expected = vec![vec![1,1,0,0],vec![0,1,1,0],vec![0,0,0,1],vec![1,0,1,0]];

        assert_eq!(expected, flip_and_invert_image(a));
    }
}