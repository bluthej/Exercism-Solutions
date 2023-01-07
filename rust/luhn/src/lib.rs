/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let spaceless = code.replace(' ', "");

    spaceless.len() > 1
        && spaceless.chars().all(|c| c.is_digit(10))
        && spaceless
            .chars()
            .rev()
            .enumerate()
            .map(|(i, c)| {
                if i % 2 == 1 {
                    match c.to_digit(10).unwrap() * 2 {
                        n if n < 10 => n,
                        n => n - 9,
                    }
                } else {
                    c.to_digit(10).unwrap()
                }
            })
            .sum::<u32>()
            % 10
            == 0
}
