pub fn answer(command: &str) -> Option<i32> {
    let computation = command.strip_prefix("What is ")?.strip_suffix('?')?;
    let mut iter = computation.split_whitespace();
    let mut result = iter.next()?.parse::<i32>().ok()?;
    while let Some(step) = iter.next() {
        match step {
            "plus" => {
                let rhs = iter.next()?;
                result += rhs.parse::<i32>().ok()?;
            }
            "minus" => {
                let rhs = iter.next()?;
                result -= rhs.parse::<i32>().ok()?;
            }
            "multiplied" => {
                if iter.next() != Some("by") {
                    return None;
                }
                let rhs = iter.next()?;
                result *= rhs.parse::<i32>().ok()?;
            }
            "divided" => {
                if iter.next() != Some("by") {
                    return None;
                }
                let rhs = iter.next()?;
                result /= rhs.parse::<i32>().ok()?;
            }
            "raised" => {
                if iter.next() != Some("to") || iter.next() != Some("the") {
                    return None;
                }
                let exp = iter
                    .next()?
                    .trim_end_matches("th")
                    .trim_end_matches("nd")
                    .parse::<u32>()
                    .ok()?;
                if iter.next() != Some("power") {
                    return None;
                }
                result = result.pow(exp);
            }
            _ => return None,
        }
    }
    Some(result)
}
