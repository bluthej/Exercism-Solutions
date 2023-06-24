// number of letters in the alphabet
const M: i32 = 26;

/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    mmi(a, M)?;

    let encryption = |x: i32| -> i32 { (a * x + b).rem_euclid(M) };

    Ok(plaintext
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| transpose(c, encryption))
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|s| s.iter().collect())
        .collect::<Vec<String>>()
        .join(" "))
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let inv_a = mmi(a, M)?;

    let decryption = |y: i32| -> i32 { (inv_a * (y - b)).rem_euclid(M) };

    Ok(ciphertext
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| transpose(c, decryption))
        .collect())
}

fn mmi(a: i32, m: i32) -> Result<i32, AffineCipherError> {
    let (mut r0, mut r1) = (a, m);
    let (mut s0, mut s1) = (1, 0);
    while r1 != 0 {
        let q = r0.div_euclid(r1);
        (r0, r1) = (r1, r0 - q * r1);
        (s0, s1) = (s1, s0 - q * s1);
    }
    if r0 != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }
    Ok(s0)
}

fn transpose(c: char, f: impl Fn(i32) -> i32) -> char {
    if c.is_alphabetic() {
        ('a'..='z')
            .position(|l| c == l)
            .map(|i| f(i as i32))
            .and_then(|i| ('a'..='z').nth(i as usize))
            .expect("`c` is alphabetic so this should never be `None`")
    } else {
        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nine_mod_twenty_six() {
        assert_eq!(mmi(9, M), Ok(3));
    }

    #[test]
    fn fifteen_mod_twenty_six() {
        assert_eq!(mmi(15, M), Ok(7));
    }

    #[test]
    fn nineteen_mod_twenty_six() {
        assert!(mmi(19, M).is_ok());
    }

    #[test]
    fn eighteem_mod_twenty_six() {
        assert!(mmi(18, M).is_err());
    }
}
