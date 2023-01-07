pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.as_bytes()
                .iter()
                .enumerate()
                .map(|(j, &square)| {
                    if square == b'*' {
                        return "*".to_owned();
                    }
                    match count_mines_if_not_mine(minefield, i, j) {
                        0 => " ".to_owned(),
                        n => n.to_string(),
                    }
                })
                .collect::<String>()
        })
        .collect()
}

fn count_mines_if_not_mine(minefield: &[&str], i: usize, j: usize) -> usize {
    ((i.max(1) - 1)..=((i + 1).min(minefield.len() - 1)))
        .map(|k| {
            ((j.max(1) - 1)..=((j + 1).min(minefield[0].len() - 1)))
                .filter(|&l| minefield[k].as_bytes()[l] == b'*')
                .count()
        })
        .sum()
}
