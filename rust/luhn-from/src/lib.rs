use std::fmt::Display;

pub struct Luhn {
    string: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let spaceless = self.string.replace(' ', "");

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

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
//impl<'a> From<&'a str> for Luhn {
//fn from(input: &'a str) -> Self {
//unimplemented!("From the given input '{input}' create a new Luhn struct.");
//}
//}

impl<T: Display> From<T> for Luhn {
    fn from(input: T) -> Self {
        let string = input.to_string();
        Self { string }
    }
}
