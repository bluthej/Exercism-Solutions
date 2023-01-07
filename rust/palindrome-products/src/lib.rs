/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        let mut x = value;
        let mut rev = 0;
        while x != 0 {
            rev = rev * 10 + (x % 10);
            x = (x - (x % 10)) / 10;
        }
        if value == rev {
            Some(Palindrome(value))
        } else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let products = (min..=max)
        .map(|i| (i..=max).filter_map(move |j| Palindrome::new(i * j)))
        .flatten();
    let (mut min, mut max) = (u64::MAX, u64::MIN);
    for palindrome in products {
        let value = palindrome.into_inner();
        if value < min {
            min = value;
        }
        if value > max {
            max = value;
        }
    }
    if (min, max) == (u64::MAX, u64::MIN) {
        return None;
    }
    Some((Palindrome(min), Palindrome(max)))
}
