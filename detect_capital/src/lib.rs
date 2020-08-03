/// Determines if the given word contains valid uses
/// of uppercase characters
/// 
/// # Arguments
/// 
/// * `word` - The word to check for valid uppercase character uses
pub fn detect_capital_use(word: String) -> bool {
    let mut all_lowercase = true;
    let mut all_uppercase = true;
    let mut first_uppercase = true;

    for (idx, c) in word.char_indices() {
        if c.is_ascii_uppercase() {
            all_lowercase = false;

            if idx > 0 {
                first_uppercase = false;
            }
        } else {
            all_uppercase = false;

            if idx == 0 {
                first_uppercase = false;
            }
        }
    }

    all_lowercase || all_uppercase || first_uppercase
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(detect_capital_use("USA".to_owned()), true);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(detect_capital_use("FlaG".to_owned()), false);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(detect_capital_use("Hello".to_owned()), true);
    }
}