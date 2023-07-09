use std::ops::RangeInclusive;

pub fn rotate(input: &str, key: i8) -> String {
    let rotate_char = |c: char, mut rng: RangeInclusive<char>| -> char {
        let n = rng.clone().position(|l| l == c).unwrap();
        rng.nth((n as i8 + key).rem_euclid(26) as usize).unwrap()
    };
    input
        .chars()
        .map(|c| {
            if c.is_alphabetic() {
                if c.is_lowercase() {
                    rotate_char(c, 'a'..='z')
                } else {
                    rotate_char(c, 'A'..='Z')
                }
            } else {
                c
            }
        })
        .collect()
}
