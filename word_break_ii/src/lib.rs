use std::collections::HashMap;

/// Finds sentences that can be created given a non-empty string and a
/// dictionary as a list of words.
/// 
/// # Arguments
/// 
/// * `s` - The original string
/// * `word_dict` - List of valid words 
pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
    let mut sentence_map = HashMap::new();
    sentence_builder(s, &word_dict, &mut sentence_map)
}

/// Recursively builds a list of sentences for each substring of 
/// `s` and finally returns the list of sentences for the original 
/// `s`
/// 
/// # Arguments
/// 
/// * `s` - A substring of the original string
/// * `word_dict` - A reference to the list of valid words
/// * `sentence_map` - A map of substrings to the current list of possible sentences
fn sentence_builder(
    s: String,
    word_dict: &[String],
    sentence_map: &mut HashMap<String, Vec<String>>,
) -> Vec<String> {
    // Base case return empty string if substring is empty
    if s.is_empty() {
        return vec![String::new()];
    }

    // Base case return current list of sentences if substring
    // has been seen before
    if let Some(words) = sentence_map.get(&s) {
        return words.to_vec();
    }

    let mut sentences = Vec::new();

    for word in word_dict {
        // Check if the beginning of the substring
        // matches a word in the word list, skip
        // if it doesn't
        match s.get(0..word.len()) {
            Some(slice) => {
                if slice != word {
                    continue;
                }
            }
            None => continue,
        }

        // Recurse on substring of any matches and update the current
        // sentence list
        let partials = sentence_builder(s[word.len()..].to_owned(), word_dict, sentence_map);
        for partial in partials {
            let space = if partial.is_empty() { "" } else { " " };
            sentences.push(format!("{}{}{}", word, space, partial));
        }
    }

    // Update current sentence list for substring
    sentence_map.insert(s, sentences.to_owned());

    sentences
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

        let expected: Vec<String> = vec!["cat sand dog".into(), "cats and dog".into()];

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
            "pine applepen apple".into(),
            "pineapple pen apple".into(),
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
