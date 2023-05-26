use rand::{thread_rng, Rng};

pub fn private_key(p: u64) -> u64 {
    let mut rng = thread_rng();
    rng.gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_exp(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    mod_exp(b_pub, a, p)
}

fn mod_exp(input: u64, exp: u64, m: u64) -> u64 {
    if m == 1 {
        return 0;
    }
    let mut res = 1;
    let mut exp = exp;
    let mut b = input;
    while exp > 0 {
        if exp % 2 == 1 {
            res = res * b % m;
        }
        exp >>= 1;
        b = b * b % m;
    }
    res
}
