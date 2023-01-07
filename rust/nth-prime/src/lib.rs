pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    };

    let mut count = 0;
    let mut current_prime = 2;

    while count < n {
        loop {
            current_prime += 1;
            let imax = (current_prime as f32).sqrt().ceil() as u32;
            if (2..=imax).all(|i| current_prime % i != 0) {
                count += 1;
                break;
            }
        }
    }
    current_prime
}
