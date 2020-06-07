use std::cmp;
use std::collections::{HashMap, HashSet};

/// Returns the length of the longest substring without repeating characters
///
/// # Arguments
///
/// * `s` - The input string
///
/// # Approach
///
/// This solution iterates through all possible substrings and tests if
/// they are unique. If the substring is unique, then update the result
/// with the maximum length.
pub fn length_of_longest_substring_naive(s: String) -> i32 {
    let n = s.len();
    let mut length = 0;

    for i in 0..n {
        for j in (i + 1)..=n {
            if all_unique(&s, i, j) {
                length = cmp::max(length, j - i);
            }
        }
    }

    length as i32
}

/// Checks if all the characters in a substring are unique
///
/// # Arguments
///
/// * `s` - The original string
/// * `start` - The starting index of the substring
/// * `end` - The index of the character past the end of the substring
fn all_unique(s: &str, start: usize, end: usize) -> bool {
    let mut set: HashSet<char> = HashSet::new();
    let chars: Vec<char> = s.chars().collect();

    for i in start..end {
        if !set.insert(chars[i]) {
            return false;
        }
    }

    true
}

/// Returns the length of the longest substring without repeating characters
///
/// # Arguments
///
/// * `s` - The input string
///
/// # Approach
///
/// This solution uses a sliding window approach where each character
/// in substring is stored in a `HashSet`. The algorithm slides an upper bound
/// to the right until a duplicate is reached or we reach the end of the string. If
/// a duplicate is found the character at the lower bound is removed from the set and
/// the lower bound is shifted to the right.
pub fn length_of_longest_substring_sliding_window(s: String) -> i32 {
    let n = s.len();
    let mut length = 0;
    let mut char_set: HashSet<char> = HashSet::new();
    let chars: Vec<char> = s.chars().collect();

    let mut i = 0;
    let mut j = 0;

    while i < n && j < n {
        if char_set.insert(chars[j]) {
            j += 1;
            length = cmp::max(length, j - i);
        } else {
            char_set.remove(&chars[i]);
            i += 1;
        }
    }

    length as i32
}

/// Returns the length of the longest substring without repeating characters.
///
/// # Arguments
///
/// * `s` - The input string
///
/// # Approach
///
/// This solution uses a sliding window approach which keeps track of the
/// starting and ending positions of the current substring. For each iteration,
/// the current character is checked against the characters seen so far.
/// If that character has already been seen, then slide the starting position
/// forward to the ending position of the current substring.
pub fn length_of_longest_substring_sliding_window_optimized(s: String) -> i32 {
    let mut length = 0;

    let mut char_set: HashMap<char, usize> = HashMap::new();

    let mut start = 0;
    for (end, c) in s.char_indices() {
        if let Some(&n) = char_set.get(&c) {
            start = cmp::max(start, n);
        }

        length = cmp::max(length, end - start + 1);
        char_set.insert(c, end + 1);
    }

    length as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example_1() {
        assert_eq!(length_of_longest_substring_naive(String::from("abcabcbb")), 3);
        assert_eq!(
            length_of_longest_substring_sliding_window(String::from("abcabcbb")),
            3
        );
        assert_eq!(
            length_of_longest_substring_sliding_window_optimized(String::from("abcabcbb")),
            3
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(length_of_longest_substring_naive(String::from("bbbb")), 1);
        assert_eq!(
            length_of_longest_substring_sliding_window(String::from("bbbb")),
            1
        );
        assert_eq!(
            length_of_longest_substring_sliding_window_optimized(String::from("bbbb")),
            1
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(length_of_longest_substring_naive(String::from("pwwkew")), 3);
        assert_eq!(
            length_of_longest_substring_sliding_window(String::from("pwwkew")),
            3
        );
        assert_eq!(
            length_of_longest_substring_sliding_window_optimized(String::from("pwwkew")),
            3
        );
    }
}
