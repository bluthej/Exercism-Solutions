pub fn get_diamond(c: char) -> Vec<String> {
    let n_c = ('A'..='Z').position(|k| k == c).unwrap();
    ('A'..=c)
        .chain(('A'..c).rev())
        .map(|l| {
            if l == 'A' {
                format!("{:n_c$}{}{:n_c$}", "", 'A', "")
            } else {
                let n = ('A'..='Z').position(|k| k == l).unwrap();
                let width = n_c + 1 - n;
                let spaces = 2 * (n - 1) + 1;
                format!("{:>width$}{:spaces$}{:<width$}", l, "", l)
            }
        })
        .collect()
}
