use std::fmt::{Display, Formatter, Result};

pub struct Roman {
    num: u32,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut num = self.num;
        let mut res = Vec::new();
        let mut level = 0;
        while num > 0 {
            let digit = num.rem_euclid(10);
            res.push(roman_format(digit, level));
            num = (num - digit) / 10;
            level += 1;
        }
        f.write_str(&res.into_iter().rev().collect::<String>())
    }
}

fn roman_format(digit: u32, level: u32) -> String {
    let mut it = "IVXLCDM".chars().skip(2 * level as usize).take(3);
    let (a, b, c) = (
        it.next().unwrap(),
        it.next().unwrap_or_default(),
        it.next().unwrap_or_default(),
    );
    match digit {
        1 => format!("{a}"),
        2 => format!("{a}{a}"),
        3 => format!("{a}{a}{a}"),
        4 => format!("{a}{b}"),
        5 => format!("{b}"),
        6 => format!("{b}{a}"),
        7 => format!("{b}{a}{a}"),
        8 => format!("{b}{a}{a}{a}"),
        9 => format!("{a}{c}"),
        _ => String::new(),
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Self { num }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", Roman::from(1234));
        println!("{}", Roman::from(245));
        println!("{}", Roman::from(89));
        println!("{}", Roman::from(7));
    }
}
