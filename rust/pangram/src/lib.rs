/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let lc = sentence.to_lowercase();
    ('a'..='z').all(|c| lc.contains(c))
}
