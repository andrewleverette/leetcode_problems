pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = "catsanddog".into();
        let word_dict = vec![
            "cat".into(),
            "cats".into(),
            "and".into(),
            "sand".into(),
            "dog".into(),
        ];

        let expected: Vec<String> = vec!["cats and dog".into(), "cat sand dog".into()];

        assert_eq!(word_break(s, word_dict), expected);
    }

    #[test]
    fn test_example_2() {
        let s = "pineapplepenapple".into();
        let word_dict = vec![
            "apple".into(),
            "pen".into(),
            "applepen".into(),
            "pine".into(),
            "pineapple".into(),
        ];

        let expected: Vec<String> = vec![
            "pine apple pen apple".into(),
            "pineapple pen apple".into(),
            "pine applepen apple".into(),
        ];

        assert_eq!(word_break(s, word_dict), expected);
    }

    #[test]
    fn test_example_3() {
        let s = "catsandog".into();
        let word_dict = vec![
            "cats".into(),
            "dog".into(),
            "sand".into(),
            "and".into(),
            "cat".into(),
        ];

        let expected: Vec<String> = vec![];

        assert_eq!(word_break(s, word_dict), expected);
    }
}
