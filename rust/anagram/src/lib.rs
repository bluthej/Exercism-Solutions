use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();
    let low_word = word.to_lowercase();
    for &possible in possible_anagrams {
        let mut anagram = possible.to_owned().to_lowercase();
        if anagram == low_word || anagram.len() != word.len() {
            continue;
        }
        for c in low_word.chars() {
            anagram = anagram.replacen(c, "", 1);
        }
        if anagram.is_empty() {
            anagrams.insert(possible);
        }
    }
    anagrams
}
