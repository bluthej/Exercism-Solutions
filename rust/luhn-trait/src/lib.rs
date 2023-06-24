use std::fmt::Display;

pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
//impl<'a> Luhn for &'a str {
//fn valid_luhn(&self) -> bool {
//unimplemented!("Determine if '{self}' is a valid credit card number.");
//}
//}

impl<T: Display> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let spaceless = self.to_string().replace(' ', "");

        spaceless.len() > 1
            && spaceless.chars().all(|c| c.is_ascii_digit())
            && spaceless
                .chars()
                .rev()
                .map(|c| {
                    c.to_digit(10)
                        .expect("Only pass here if all characters are ascii digits")
                })
                .enumerate()
                .map(|(i, d)| {
                    let mut x = d;
                    if i % 2 == 1 {
                        x *= 2;
                        if x >= 10 {
                            x -= 9;
                        }
                    }
                    x
                })
                .sum::<u32>()
                % 10
                == 0
    }
}
