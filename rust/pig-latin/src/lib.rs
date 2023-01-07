pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            let mut i = 0;
            if !["a", "e", "i", "o", "u", "xr", "yt"]
                .iter()
                .any(|vowel| word.starts_with(vowel))
            {
                while !"aeiouy".contains(&word[i..=i]) {
                    i += 1;
                }
                if word[..3].contains("qu") || word.starts_with('y') {
                    i += 1;
                }
            };
            let (consonant, rest) = word.split_at(i);
            format!("{}{}ay", rest, consonant)
        })
        .collect::<Vec<String>>()
        .join(" ")
}
