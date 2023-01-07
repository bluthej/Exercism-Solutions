pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let maxs = input
        .iter()
        .map(|row| row.iter().max())
        .collect::<Option<Vec<&u64>>>()
        .unwrap_or(Vec::new());
    let mins = (0..input[0].len())
        .map(|i| input.iter().map(|row| &row[i]).min())
        .collect::<Option<Vec<&u64>>>()
        .unwrap_or(Vec::new());
    input
        .iter()
        .enumerate()
        .map(|(i, row)| row.iter().enumerate().map(move |(j, x)| (i, j, x)))
        .flatten()
        .filter_map(|(i, j, x)| {
            if x == maxs[i] && x == mins[j] {
                Some((i, j))
            } else {
                None
            }
        })
        .collect()
}
