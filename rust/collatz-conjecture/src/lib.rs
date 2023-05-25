pub fn collatz(n: u64) -> Option<u64> {
    let mut n = n;
    for count in 0.. {
        match n {
            0 => return None,
            1 => return Some(count),
            m if m % 2 == 0 => n /= 2,
            _ => n = n.checked_mul(3)?.checked_add(1)?,
        }
    }
    unreachable!("The loop will always return `Some(value)` or `None` in case of overflow");
}
