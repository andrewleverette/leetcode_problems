use std::collections::HashMap;

/// Calculates the distinct number of ways that 
/// n-number of steps can be climbed taking either 1 or
/// 2 steps at a time. This is calculated in using a
/// linear approach.
/// 
/// # Arguments:
/// 
/// * `n` - The number of steps
pub fn climb_stairs_linear(n: i32) -> i32 {
    if n < 3 {
        n
    } else {
        let mut a = 1;
        let mut b = 2;
        for _ in 2..n {
            b += a;
            a = b - a;
        }

        b
    }
}

/// Calculates the distinct number of ways that 
/// n-number of steps can be climbed taking either 1 or
/// 2 steps at a time. This is calculated using memoization
pub fn climb_stairs_memoize(n: i32) -> i32 {
    let mut cache: HashMap<i32, i32> = HashMap::new();
    cache.insert(1, 1);
    cache.insert(2, 2);
    backtrack(n, &mut cache)
}

/// Returns the distinct paths to `n`. If an `n` has been seen
/// before, the stored value is returned. Otherwise the number of
/// distinct paths are calculated as T(n) = T(n - 1) + T(n - 2).
/// 
/// # Arguments
/// 
/// * `n` - The current number of steps
/// * `cache` - A mapping of a `n` to the number of distinct paths
fn backtrack(n: i32, cache: &mut HashMap<i32, i32>) -> i32 {
    if let Some(&steps) = cache.get(&n) {
        steps
    } else {
        let steps = backtrack(n - 1, cache) + backtrack(n - 2, cache);
        cache.insert(n, steps);
        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(climb_stairs_memoize(2), 2);
        assert_eq!(climb_stairs_linear(2), 2)
    }

    #[test]
    fn test_example_2() {
        assert_eq!(climb_stairs_memoize(3), 3);
        assert_eq!(climb_stairs_linear(3), 3)
    }

    #[test]
    fn test_example_3() {
        assert_eq!(climb_stairs_memoize(4), 5);
        assert_eq!(climb_stairs_linear(4), 5)
    }
}
