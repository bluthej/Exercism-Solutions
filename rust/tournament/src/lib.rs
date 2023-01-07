use std::collections::HashMap;

pub fn tally(match_results: &str) -> String {
    let mut results: HashMap<&str, (usize, usize, usize, usize, usize)> = HashMap::new();
    for line in match_results.lines() {
        let res: Vec<&str> = line.splitn(3, ";").collect();
        let result = res[2];

        if result == "draw" {
            for team in res[0..2].iter() {
                results
                    .entry(team)
                    .and_modify(|counts| {
                        counts.0 += 1;
                        counts.2 += 1;
                        counts.4 += 1;
                    })
                    .or_insert((1, 0, 1, 0, 1));
            }
            continue;
        };

        let winner = if result == "win" { res[0] } else { res[1] };
        let loser = if result == "loss" { res[0] } else { res[1] };
        results
            .entry(winner)
            .and_modify(|counts| {
                counts.0 += 1;
                counts.1 += 1;
                counts.4 += 3;
            })
            .or_insert((1, 1, 0, 0, 3));
        results
            .entry(loser)
            .and_modify(|counts| {
                counts.0 += 1;
                counts.3 += 1;
            })
            .or_insert((1, 0, 0, 1, 0));
    }

    let mut output = String::from(&format!(
        "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
        "Team", "MP", "W", "D", "L", "P"
    ));
    while !results.is_empty() {
        let mut first = None;
        let keys: Vec<&str> = results.keys().copied().collect();
        for team in keys {
            if let Some(first_team) = first {
                let points_first_team = results.get(first_team).unwrap().4;
                let points_team = results.get(team).unwrap().4;
                if points_first_team < points_team
                    || (points_first_team == points_team && team < first_team)
                {
                    first = Some(team);
                }
            } else {
                first = Some(team);
            }
        }
        let first = first.unwrap();
        let (mp, w, d, l, p) = results.remove(first).unwrap();
        output.push_str(&format!(
            "\n{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            first, mp, w, d, l, p
        ));
    }
    output
}
