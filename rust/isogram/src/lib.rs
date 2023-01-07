pub fn check(candidate: &str) -> bool {
    !candidate
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .enumerate()
        .any(|(i, c)| {
            candidate
                .to_lowercase()
                .chars()
                .filter(|d| d.is_alphabetic())
                .enumerate()
                .any(|(j, d)| c == d && i != j)
        })
}
