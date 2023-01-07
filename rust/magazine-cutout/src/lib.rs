// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut word_count = HashMap::new();

    for word in note {
        word_count
            .entry(word)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    for word in magazine {
        word_count.entry(word).and_modify(|count| *count -= 1);
    }

    word_count.iter().all(|(_, count)| *count < 1)
}
