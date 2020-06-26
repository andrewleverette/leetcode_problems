pub fn reverse_integer(x: i32) -> i32 {
    let mut value = i32::abs(x);
    let mut reverse: i32 = 0;

    while value != 0 {
        let pop = value % 10;
        value /= 10;

        reverse = match reverse.checked_mul(10) {
            Some(n) => n,
            None => return 0,
        };

        reverse = match reverse.checked_add(pop) {
            Some(n) => n,
            None => return 0,
        }
    }

    if x < 0 {
        reverse *= -1;
    }

    reverse
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(321, reverse_integer(123));
    }

    #[test]
    fn test_example_2() {
        assert_eq!(-321, reverse_integer(-123));
    }


    #[test]
    fn test_example_3() {
        assert_eq!(21, reverse_integer(120));
    }
}
