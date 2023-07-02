use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .split(|c: char| !c.is_alphanumeric() && c != '\'')
        .filter(|word| !word.is_empty())
        .fold(HashMap::new(), |mut acc, word| {
            acc.entry(word.trim_matches('\'').to_lowercase())
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
            acc
        })
}
