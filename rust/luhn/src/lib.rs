/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let spaceless = code.replace(' ', "");

    spaceless.len() > 1
        && spaceless.chars().all(|c| c.is_ascii_digit())
        && spaceless
            .chars()
            .rev()
            .map(|c| {
                c.to_digit(10)
                    .expect("Only pass here if all characters are ascii digits")
            })
            .enumerate()
            .map(|(i, d)| {
                let mut x = d;
                if i % 2 == 1 {
                    x *= 2;
                    if x >= 10 {
                        x -= 9;
                    }
                }
                x
            })
            .sum::<u32>()
            % 10
            == 0
}
