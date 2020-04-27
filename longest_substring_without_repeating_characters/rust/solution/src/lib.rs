use std::cmp;

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
/// forward to the ending position of the current substring. This solution works
/// if you can make safe assumptions about the characters set. In this case I assume
/// the character set is standard ASCII.
pub fn length_of_longest_substring_sliding_window_optimized(s: String) -> i32 {
    let mut length = 0;

    let mut char_set = [0; 128];

    let mut start = 0;
    for (end, c) in s.char_indices() {
        start = cmp::max(char_set[c as usize], start);
        length = cmp::max(length, end - start + 1);
        char_set[c as usize] = end + 1;
    }

    length as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example_1() {
        assert_eq!(
            length_of_longest_substring_sliding_window_optimized(String::from("abcabcbb")),
            3
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            length_of_longest_substring_sliding_window_optimized(String::from("bbbb")),
            1
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            length_of_longest_substring_sliding_window_optimized(String::from("pwwkew")),
            3
        );
    }
}