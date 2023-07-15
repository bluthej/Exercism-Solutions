use std::iter;

use rand::{distributions::Alphanumeric, Rng};

pub fn encode(key: &str, s: &str) -> Option<String> {
    let operation = |k: usize, c: usize| (k + c) % 26;
    code(key, s, operation)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    let operation = |k: usize, c: usize| if k <= c { c - k } else { c + 26 - k };
    code(key, s, operation)
}

fn code(key: &str, s: &str, shift: impl Fn(usize, usize) -> usize) -> Option<String> {
    if key.chars().any(|c| c.is_uppercase() || !c.is_alphabetic()) || key.is_empty() {
        return None;
    }
    key.chars()
        .chain(iter::repeat('a'))
        .zip(s.chars())
        .map(|(k, c)| {
            let key_index = ('a'..='z').position(|l| l == k).unwrap();
            let char_index = ('a'..='z').position(|l| l == c).unwrap();
            ('a'..='z').nth(shift(key_index, char_index))
        })
        .collect()
}

pub fn encode_random(s: &str) -> (String, String) {
    let key = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .filter(|c| c.is_ascii_alphabetic())
        .take(100)
        .map(char::from)
        .collect::<String>()
        .to_lowercase();
    (key.clone(), encode(&key, s).unwrap())
}
