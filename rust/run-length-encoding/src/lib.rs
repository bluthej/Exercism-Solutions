pub fn encode(source: &str) -> String {
    let mut iter = source.chars().peekable();
    let mut res = String::new();
    let mut n = 0;
    while let Some(c) = iter.next() {
        n += 1;
        if iter.peek() != Some(&c) {
            if n > 1 {
                res.push_str(&n.to_string());
            }
            res.push(c);
            n = 0;
        }
    }
    res
}

pub fn decode(source: &str) -> String {
    let mut iter = source.chars().peekable();
    let mut res = String::new();
    let mut num = String::new();
    while let Some(c) = iter.next() {
        if c.is_numeric() {
            num.push(c);
        } else {
            res.push_str(&c.to_string().repeat(num.parse::<usize>().unwrap_or(1)));
            num.clear();
        }
    }
    res
}
