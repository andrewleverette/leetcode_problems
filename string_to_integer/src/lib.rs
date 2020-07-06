pub fn atoi(str: String) -> i32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_example_1() {
        let input = "42".to_string();
        assert_eq!(42, atoi(input));
    }

    #[test]
    fn text_example_2() {
        let input = "   -42".to_string();
        assert_eq!(-42, atoi(input));
    }

    #[test]
    fn text_example_3() {
        let input = "4193 with words".to_string();
        assert_eq!(4193, atoi(input));
    }

    #[test]
    fn text_example_4() {
        let input = "words and 987".to_string();
        assert_eq!(0, atoi(input));
    }

    #[test]
    fn text_example_5() {
        let input = "-91283472332".to_string();
        assert_eq!(-2147483648, atoi(String::new()));
    }
}
