pub fn build_proverb(list: &[&str]) -> String {
    let n = list.len();
    if n == 0 {
        return String::new();
    };

    let proverb = (0..(n - 1))
        .map(|i| format!("For want of a {} the {} was lost.\n", list[i], list[i + 1]))
        .collect::<Vec<String>>()
        .concat();

    [proverb, format!("And all for the want of a {}.", list[0])].concat()
}
