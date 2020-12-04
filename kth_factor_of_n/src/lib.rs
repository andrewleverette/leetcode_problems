pub fn kth_factor_naive(n: i32, k: i32) -> i32 {
    let mut factor_counter = 0;
    let mut factor = 0;

    for i in 1..=n {
        if n % i == 0 {
            factor_counter += 1;
            factor = i;

            if factor_counter == k {
                break;
            }
        }
    }

    if factor_counter < k {
        -1
    } else {
        factor
    }
}

pub fn kth_factor_sqrt(n: i32, k: i32) -> i32 {
    let mut factor_counter = 0;
    let mut i = 1;

    while i * i <= n {
        if n % i == 0 {
            factor_counter += 1;

            if factor_counter == k {
                return i;
            }
        }

        i += 1;
    }

    i = i - 1;
    while i > 0 {
        if i * i != n && n % i == 0 {
            factor_counter += 1;

            if factor_counter == k {
                return n / i;
            }
        }

        i -= 1;
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(kth_factor_naive(12, 3), 3);
        assert_eq!(kth_factor_sqrt(12, 3), 3);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(kth_factor_naive(7, 2), 7);
        assert_eq!(kth_factor_sqrt(7, 2), 7);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(kth_factor_naive(4, 4), -1);
        assert_eq!(kth_factor_sqrt(4, 4), -1);
    }

    #[test]
    fn test_example_4() {
        assert_eq!(kth_factor_naive(1, 1), 1);
        assert_eq!(kth_factor_sqrt(1, 1), 1);
    }

    #[test]
    fn test_example_5() {
        assert_eq!(kth_factor_naive(1000, 3), 4);
        assert_eq!(kth_factor_sqrt(1000, 3), 4);
    }

    #[test]
    fn test_prime_number() {
        assert_eq!(kth_factor_naive(3, 2), 3);
        assert_eq!(kth_factor_sqrt(3, 2), 3);
    }

    #[test]
    fn test_larger_k_than_factors() {
        assert_eq!(kth_factor_naive(420, 25), -1);
        assert_eq!(kth_factor_sqrt(420, 25), -1);
    }

    #[test]
    fn test_larger_k_than_factors_alt() {
        assert_eq!(kth_factor_naive(6, 5), -1);
        assert_eq!(kth_factor_sqrt(6, 5), -1);
    }

    #[test]
    fn test_failing_input() {
        assert_eq!(kth_factor_sqrt(24, 6), 8);
    }
}
