#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    };
    match (1..num).filter(|d| num % d == 0).sum::<u64>() {
        i if i < num => Some(Classification::Deficient),
        i if i > num => Some(Classification::Abundant),
        _ => Some(Classification::Perfect),
    }
}
