use std::fmt::{Display, Formatter, Result};

pub struct Roman {
    num: u32,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let output: String = (0..4)
            .rev()
            .filter_map(|level| {
                let q = self.num / 10_u32.pow(level);
                (q > 0).then_some(roman_format(q % 10, level))
            })
            .collect();
        f.write_str(&output)
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
