use std::i32;

/// Attempts to convert a string into a 32-bit signed integer.
/// 
/// Returns 0 if there is no valid conversion. Returns the MIN or MAX of
/// i32 if the number is outside the bounds of a signed 32-bit number. Returns
/// the converted number otherwise.
pub fn string_to_integer(str: String) -> i32 {
    let mut n: i32 = 0;
    let mut sign = 1;
    let mut chars = str.chars().peekable();

    // Loop through string skipping whitespace characters until
    // the firs non-whitespace character. Return 0 if there 
    // are no non-whitespace characters.
    loop {
        match chars.peek().map(|c| !c.is_ascii_whitespace()) {
            Some(true) => break,
            Some(false) => {
                chars.next();
            },
            None => return 0,
        }
    }

    // Update sign of number
    match chars.peek() {
        Some('-') => {
            sign = -1;
            chars.next();
        }
        Some('+') => {
            chars.next();
        }
        _ => {}
    }

    // Iterate through the rest of string while the characters are digits
    // and attempt to convert them to a i32 number. Returns the MIN or MAX 
    // of i32 if the parsing fails.
    for digit in chars.take_while(|x| x.is_ascii_digit()) {
        if let Some(x) = n
            .checked_mul(10)
            .and_then(|n| n.checked_add(digit.to_digit(10).unwrap_or_default() as i32))
        {
            n = x
        } else {
            return if sign == -1 { i32::MIN } else { i32::MAX };
        }
    }

    sign * n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = "42".into();
        assert_eq!(string_to_integer(s), 42);
    }

    #[test]
    fn test_example_2() {
        let s = "   -42".into();
        assert_eq!(string_to_integer(s), -42);
    }

    #[test]
    fn test_example_3() {
        let s = "4193 with words".into();
        assert_eq!(string_to_integer(s), 4193);
    }

    #[test]
    fn test_example_4() {
        let s = "words and 987".into();
        assert_eq!(string_to_integer(s), 0);
    }

    #[test]
    fn test_example_5() {
        let s = "-91283472332".into();
        assert_eq!(string_to_integer(s), -2147483648);
    }

    #[test]
    fn test_example_6() {
        let s = " ".into();
        assert_eq!(string_to_integer(s), 0);
    }

    #[test]
    fn test_example_7() {
        let s = "+".into();
        assert_eq!(string_to_integer(s), 0);
    }
}
