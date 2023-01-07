pub fn is_armstrong_number(num: u32) -> bool {
    let nums: Vec<_> = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let n = nums.len().try_into().unwrap();
    num == nums.iter().map(|d| d.pow(n)).sum()
}
