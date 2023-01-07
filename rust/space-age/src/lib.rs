// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const EARTH_YEAR_IN_SECONDS: f64 = 31557600.0;

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration((s as f64) / EARTH_YEAR_IN_SECONDS)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! impl_Planet {
    (for $($t:ident => $o:expr),+) => {
        $(
            pub struct $t;

            impl Planet for $t {
                fn years_during(d: &Duration) -> f64 {
                    let Duration(s) = d;
                    s / $o
                }
            }
        )*
    }
}

impl_Planet!(for Mercury => 0.2408467,
    Venus => 0.61519726,
    Earth => 1.,
    Mars => 1.8808158,
    Jupiter => 11.862615,
    Saturn => 29.447498,
    Uranus => 84.016846,
    Neptune => 164.79132);
