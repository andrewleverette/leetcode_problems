use std::collections::HashMap;

pub fn climb_stairs(n: i32) -> i32 {
    let mut memoize: HashMap<i32, i32> = HashMap::new();
    memoize.insert(1, 1);
    memoize.insert(2, 2);
    backtrack(n, &mut memoize)
}

fn backtrack(n: i32, memoize: &mut HashMap<i32, i32>) -> i32 {
    if let Some(&steps) = memoize.get(&n) {
        steps
    } else {
        let steps = backtrack(n - 1, memoize) + backtrack(n - 2, memoize);
        memoize.insert(n, steps);
        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(climb_stairs(2), 2);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(climb_stairs(3), 3);
    }

}
