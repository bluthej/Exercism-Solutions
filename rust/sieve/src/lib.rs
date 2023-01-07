pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let ub = upper_bound as usize;
    let mut primes: Vec<_> = (2..=ub).map(Option::from).collect();
    (0..primes.len())
        .filter_map(|i| {
            let prime = primes[i].take()?;
            (prime..=ub)
                .step_by(prime)
                .skip(1)
                .for_each(|m| primes[m - 2] = None);
            Some(prime as u64)
        })
        .collect()
}
