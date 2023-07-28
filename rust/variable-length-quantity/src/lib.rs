#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|d| {
            let n_bits = if d == &0 { 1 } else { 32 - d.leading_zeros() };
            let n_bytes = (n_bits as f32 / 7.).ceil() as usize;
            (0..n_bytes)
                .map(move |i| {
                    let n = (d >> (7 * i)) as u8 & 0b01111111;
                    let last_bit = if i == 0 { 0 } else { 0b10000000 };
                    n | last_bit
                })
                .rev()
        })
        .collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    if let Some(last_byte) = bytes.last() {
        if last_byte >> 7 != 0 {
            return Err(Error::IncompleteNumber);
        }
    }

    let mut res = Vec::new();
    let mut current_value = 0u32;
    for byte in bytes {
        if current_value.leading_zeros() < 7 {
            return Err(Error::Overflow);
        }
        current_value = (current_value << 7) + ((byte & 0b01111111) as u32);
        if (byte >> 7) == 0 {
            res.push(current_value);
            current_value = 0u32;
        }
    }
    Ok(res)
}
