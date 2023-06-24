fn transpose(c: char) -> char {
    if c.is_alphabetic() {
        ('a'..='z')
            .position(|l| c == l)
            .and_then(|i| ('a'..='z').nth_back(i))
            .expect("`c` is alphabetic so this should never be `None`")
    } else {
        c
    }
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(transpose)
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|s| s.iter().collect())
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(transpose)
        .collect()
}
