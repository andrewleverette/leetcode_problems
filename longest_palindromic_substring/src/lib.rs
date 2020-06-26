use std::iter::FromIterator;

/// Returns the longest substring that is a palindrome
/// 
/// # Arguments
/// 
/// * `s` - String input 
/// 
/// # Approach
/// 
/// This approach checks every substring to see if it is a palindrome
/// and tracks the start and end of the longest
pub fn longest_palindrome_brute_force(s: String) -> String {
    let n = s.len();

    if n == 0 {
        return "".to_string();
    }

    let s: Vec<char> = s.chars().collect();

    let mut start = 0;
    let mut end = 0;

    for i in 0..n {
        for j in i..n {
            let mut m = i;
            let mut n = j;
            let mut is_palindrome = true;
            while m < n {
                if s[m] != s[n] {
                    is_palindrome = false;
                    break;
                }

                m += 1;
                n -= 1;
            }

            if is_palindrome && j - i > end - start {
                start = i;
                end = j + 1;
            }
        }
    }

    String::from_iter(&s[start..end])
}

/// Returns the longest substring that is a palindrome
/// 
/// # Arguments
/// 
/// * `s` - String input 
/// 
/// # Approach
/// 
/// This approach expands the search around the center of a substring
/// and keeps track of the start and end points of the longest substring
pub fn longest_palindrome_expansion(s: String) -> String {
    let n = s.len();

    if n == 0 {
        return "".to_string();
    }

    let s: Vec<char> = s.chars().collect();

    let mut start = 0;
    let mut end = 0;

    for i in 0..n {
        let mut left = i;
        let mut right = i;

        // Move right boundary if center is between two characters
        while right + 1 < n && s[right + 1] == s[left] {
            right += 1;
        }

        // Expand around center of substring
        while left > 0 && right < n - 1 && s[left - 1] == s[right + 1] {
            left -= 1;
            right += 1;
        }

        // Update start and end of longest substring
        if right - left > end - start {
            start = left;
            end = right;
        }
    }

    String::from_iter(&s[start..=end])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = "babad".to_string();
        assert_eq!(longest_palindrome_brute_force(s.clone()), "bab".to_string());
        assert_eq!(longest_palindrome_expansion(s), "bab".to_string());
    }

    #[test]
    fn test_example_2() {
        let s = "cbbd".to_string();
        assert_eq!(longest_palindrome_brute_force(s.clone()), "bb".to_string());
        assert_eq!(longest_palindrome_expansion(s), "bb".to_string());
    }
}
