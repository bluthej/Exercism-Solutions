#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    string_digits
        .chars()
        .map(|c| c.to_digit(10).map(u64::from).ok_or(Error::InvalidDigit(c)))
        .collect::<Result<Vec<u64>, _>>()?
        .windows(span)
        .map(|w| w.iter().product())
        .max()
        .ok_or(Error::SpanTooLong)
}
