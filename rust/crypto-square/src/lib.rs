pub fn encrypt(input: &str) -> String {
    let normalized = input
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .collect::<String>()
        .to_lowercase();
    let l = normalized.len();
    let r = (l as f32).sqrt().floor() as usize;
    let c = if r.pow(2) < l { r + 1 } else { r };
    let n = (r * c) - l;
    (0..c)
        .map(|i| {
            let mut s: String = normalized.chars().skip(i).step_by(c).collect();
            if i >= c - n {
                s.push(' ');
            }
            s
        })
        .collect::<Vec<String>>()
        .join(" ")
}
