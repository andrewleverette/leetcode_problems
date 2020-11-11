pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
    let d2 = square_distance(&p1, &p2);
    let d3 = square_distance(&p1, &p3);
    let d4 = square_distance(&p1, &p4);

    if d2 == 0 || d3 == 0 || d4 == 0 {
        false
    } else if d2 == d3
        && 2 * d2 == d4
        && 2 * square_distance(&p2, &p4) == square_distance(&p2, &p3)
        && square_distance(&p2, &p4) == square_distance(&p3, &p4)
    {
        true
    } else if d2 == d4 
        && 2 * d2 == d3 
        && 2 * square_distance(&p2, &p3) == square_distance(&p2, &p4)
        && square_distance(&p2, &p3) == square_distance(&p4, &p1)
    {
        true
    } else if d3 == d4 
        && 2 * d3 == d2 
        && 2 * square_distance(&p3, &p2) == square_distance(&p3, &p4)
        && square_distance(&p3, &p2) == square_distance(&p4, &p1)
    {
        true
    } else {
        false
    }
}

fn square_distance(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
    (a[0] - b[0]).pow(2) + (a[1] - b[1]).pow(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            true,
            valid_square(vec![0, 0], vec![1, 1], vec![1, 0], vec![0, 1])
        )
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            false,
            valid_square(vec![1, 1], vec![0, 1], vec![1, 2], vec![0, 0])
        )
    }
}
