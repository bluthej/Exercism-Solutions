/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut numbers: Vec<u32> = isbn.chars().filter_map(|c| c.to_digit(10)).collect();
    match isbn.chars().last() {
        Some('X') => numbers.push(10),
        _ => (),
    }
    numbers.len() == 10
        && numbers
            .iter()
            .enumerate()
            .map(|(i, &n)| n * (10 - i as u32))
            .sum::<u32>()
            % 11
            == 0
}
