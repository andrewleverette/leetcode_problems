pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
    let mut points = vec![p1, p2, p3, p4];
    points.sort_by(|a, b| {
        match a[0] == b[0] {
            true => a[1].cmp(&b[1]),
            false => a[0].cmp(&b[0]),
        }
    });

    square_distance(&points[0], &points[1]) != 0
        && square_distance(&points[0], &points[1]) == square_distance(&points[0], &points[2])
        && 2 * square_distance(&points[0], &points[1]) == square_distance(&points[0], &points[3]) 
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
