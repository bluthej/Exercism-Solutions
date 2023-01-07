#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }
    for &d in number.iter() {
        if d >= from_base {
            return Err(Error::InvalidDigit(d));
        }
    }
    let mut n: u32 = number
        .iter()
        .rev()
        .enumerate()
        .map(|(i, j)| from_base.pow(i as u32) * j)
        .sum();
    if n == 0 {
        return Ok([0].to_vec());
    }
    let mut res = Vec::new();
    let logn = (n as f32).log(to_base as f32) as u32;
    for i in (0..=logn).rev() {
        let b = n / to_base.pow(i);
        n -= b * to_base.pow(i);
        res.push(b);
    }
    Ok(res)
}
