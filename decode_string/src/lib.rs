pub fn decode_string_stack(s: String) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut k_stack = Vec::new();
    let mut str_stack = Vec::new();
    let mut decoded = String::new();

    let mut i = 0;
    while i < chars.len() {
        if chars[i].is_numeric() {
            let mut k = 0;
            while chars[i].is_numeric() {
                k = 10 * k + (chars[i] as u16 - '0' as u16);
                i += 1;
            }
            k_stack.push(k);
        } else if chars[i] == '[' {
            str_stack.push(decoded.clone());
            decoded = String::new();
            i += 1;
        } else if chars[i] == ']' {
            let k = k_stack.pop().unwrap();
            let mut substring = str_stack.pop().unwrap();
            for _ in 0..k as usize {
                substring.push_str(&decoded);
            }
            decoded = substring;
            i += 1;
        } else {
            decoded.push(chars[i]);
            i += 1;
        }
    }

    decoded
}

pub fn decode_string_recursive(s: String) -> String {
    let chars = s.chars().collect();
    let (result, _) = decode_substring(&chars, 0, 0);
    result
}

pub fn decode_substring(chars: &Vec<char>, k: u16, start: usize) -> (String, usize) {
    let mut substring = String::new();

    let mut i = start;
    while i < chars.len() {
        let mut n = 0;
        while chars[i].is_numeric() {
            n = 10 * n + (chars[i] as u16 - '0' as u16);
            i += 1;
        }
        
        if chars[i] == '[' {
            let (result, end) = decode_substring(&chars, n, i + 1);
            substring += &result;
            i = end;
        } else if chars[i] == ']' {
            break;
        } else {
            substring.push(chars[i]);
            i += 1;
        }
    }

    if k == 0 {
        (substring, i + 1)
    } else {
        let mut result = String::new();
        for _ in 0..k {
            result += &substring;
        }
        (result, i + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example_1() {
        let s = "3[a]2[bc]".to_owned();
        assert_eq!("aaabcbc".to_owned(), decode_string_stack(s));
    }

    #[test]
    fn test_example_1_recursive() {
        let s = "3[a]2[bc]".to_owned();
        assert_eq!("aaabcbc".to_owned(), decode_string_recursive(s));
    }

    #[test]
    fn test_example_2() {
        let s = "3[a2[c]]".to_owned();
        assert_eq!("accaccacc".to_owned(), decode_string_stack(s));
    }

    #[test]
    fn test_example_2_recursive() {
        let s = "3[a2[c]]".to_owned();
        assert_eq!("accaccacc".to_owned(), decode_string_recursive(s));
    }

    #[test]
    fn test_example_3() {
        let s = "2[abc]3[cd]ef".to_owned();
        assert_eq!("abcabccdcdcdef".to_owned(), decode_string_stack(s));
    }

    #[test]
    fn test_example_3_recursive() {
        let s = "2[abc]3[cd]ef".to_owned();
        assert_eq!("abcabccdcdcdef".to_owned(), decode_string_recursive(s));
    }

    #[test]
    fn test_example_4() {
        let s = "abc3[cd]xyz".to_owned();
        assert_eq!("abccdcdcdxyz".to_owned(), decode_string_stack(s));
    }

    #[test]
    fn test_example_4_recursive() {
        let s = "abc3[cd]xyz".to_owned();
        assert_eq!("abccdcdcdxyz".to_owned(), decode_string_recursive(s));
    }
}
