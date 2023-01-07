pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors: Vec<u64> = Vec::new();

    let mut current_value = n;
    while current_value != 1 {
        let div = (2..=current_value)
            .find(|i| current_value % i == 0)
            .unwrap();
        prime_factors.push(div);
        current_value /= div;
    }

    prime_factors
}
