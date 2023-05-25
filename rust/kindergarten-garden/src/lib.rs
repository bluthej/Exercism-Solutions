pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let char_to_str = |c: char| -> &'static str {
        match c {
            'G' => "grass",
            'C' => "clover",
            'R' => "radishes",
            'V' => "violets",
            _ => unimplemented!("Unknown initial"),
        }
    };

    let initial = student
        .chars()
        .next()
        .expect("Student name should have at least one character");
    let n = ('A'..='L')
        .position(|c| c == initial)
        .expect("Student initial should be in range A-L");

    diagram
        .lines()
        .flat_map(|line| line.chars().skip(2 * n).take(2).map(char_to_str))
        .collect()
}
