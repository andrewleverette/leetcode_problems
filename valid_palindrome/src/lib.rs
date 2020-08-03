/// Determines if a given string is a palindrome.
/// Ignores whitespace and punctuation.
/// 
/// # Arguments
/// 
/// * `s` - The string to check
pub fn is_palindrome(s: String) -> bool {
    if s.is_empty() {
        return true;
    }
    
    // Filter out non-alphanumeric characters and convert to lowercase
    let ascii_characters: Vec<char> = s.chars().filter(|c| c.is_ascii_alphanumeric()).flat_map(|c| c.to_lowercase()).collect();
    
    let mut start = 0;
    let mut end = ascii_characters.len() - 1;
    
    // Check from outside in
    for _ in 0..ascii_characters.len() {
        if start > end {
            break;
        }

        if ascii_characters[start] != ascii_characters[end] {
            return false;
        }

        start += 1;
        end -= 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = "A man, a plan, a canal: Panama".to_owned();
        assert_eq!(is_palindrome(s), true);
    }

    #[test]
    fn test_example_2() {
        let s = "race a car".to_owned();
        assert_eq!(is_palindrome(s), false);
    }

    #[test]
    fn test_example_3() {
        let s = String::new();
        assert_eq!(is_palindrome(s), true);
    }
}
