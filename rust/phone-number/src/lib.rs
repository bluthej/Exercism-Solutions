use regex::Regex;

pub fn number(user_number: &str) -> Option<String> {
    let re =
        Regex::new(r"^\+?1?\s*\(?([2-9]\d{2})\)?(?:-|\s+|\.)?([2-9]\d{2})(?:-|\s+|\.)?(\d{4})\s*$")
            .ok()?;
    let cap = re.captures(user_number)?;
    Some(format!("{}{}{}", &cap[1], &cap[2], &cap[3]))
}
