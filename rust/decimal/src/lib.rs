use std::ops::{Add, Mul, Sub};

use num_bigint::BigInt;

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, PartialEq)]
pub struct Decimal {
    value: BigInt,
    scale: u32,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let (integer_part, decimal_part) = input.split_once('.').unwrap_or((input, ""));
        Some(
            Self {
                value: [integer_part, decimal_part].join("").parse().ok()?,
                scale: decimal_part.len() as u32,
            }
            .sanitize(),
        )
    }

    fn sanitize(self) -> Self {
        let mut value = self.value;
        let mut scale = self.scale;
        while &value % 10 == BigInt::from(0) && scale > 0 {
            value /= 10;
            scale -= 1;
        }
        Self { value, scale }
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let left =
            self.value.clone() * BigInt::from(10).pow(other.scale.saturating_sub(self.scale));
        let right =
            other.value.clone() * BigInt::from(10).pow(self.scale.saturating_sub(other.scale));
        left.partial_cmp(&right)
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let left = self.value * BigInt::from(10).pow(other.scale.saturating_sub(self.scale));
        let right = other.value * BigInt::from(10).pow(self.scale.saturating_sub(other.scale));
        let value = left + right;
        let scale = u32::max(self.scale, other.scale);
        Self { value, scale }.sanitize()
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let left = self.value * BigInt::from(10).pow(other.scale.saturating_sub(self.scale));
        let right = other.value * BigInt::from(10).pow(self.scale.saturating_sub(other.scale));
        let value = left - right;
        let scale = u32::max(self.scale, other.scale);
        Self { value, scale }.sanitize()
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let value = self.value * other.value;
        let scale = self.scale + other.scale;
        Self { value, scale }.sanitize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_decimal_part() {
        let input = "10";
        let expected = Decimal {
            value: BigInt::from(10),
            scale: 0,
        };
        assert_eq!(Decimal::try_from(input), Some(expected));
    }

    #[test]
    fn one_decimal() {
        let input = "10.5";
        let expected = Decimal {
            value: BigInt::from(105),
            scale: 1,
        };
        assert_eq!(Decimal::try_from(input), Some(expected));
    }

    #[test]
    fn two_decimals() {
        let input = "10.52";
        let expected = Decimal {
            value: BigInt::from(1052),
            scale: 2,
        };
        assert_eq!(Decimal::try_from(input), Some(expected));
    }

    #[test]
    fn point_but_no_decimals() {
        let input = "10.";
        let expected = Decimal {
            value: BigInt::from(10),
            scale: 0,
        };
        assert_eq!(Decimal::try_from(input), Some(expected));
    }

    #[test]
    fn lots_of_zeros() {
        let input = "10000.0000";
        let expected = Decimal {
            value: BigInt::from(10000),
            scale: 0,
        };
        assert_eq!(Decimal::try_from(input), Some(expected));
    }

    #[test]
    fn just_decimals() {
        let input = "0.203";
        let expected = Decimal {
            value: BigInt::from(203),
            scale: 3,
        };
        assert_eq!(Decimal::try_from(input), Some(expected));
    }

    #[test]
    fn leading_zeros() {
        let input = "0020.203";
        let expected = Decimal {
            value: BigInt::from(20203),
            scale: 3,
        };
        assert_eq!(Decimal::try_from(input), Some(expected));
    }

    #[test]
    fn trailing_zeros() {
        let input = "20.20300";
        let expected = Decimal {
            value: BigInt::from(20203),
            scale: 3,
        };
        assert_eq!(Decimal::try_from(input), Some(expected));
    }

    #[test]
    fn point_but_no_integer_part() {
        let input = ".203";
        let expected = Decimal {
            value: BigInt::from(203),
            scale: 3,
        };
        assert_eq!(Decimal::try_from(input), Some(expected));
    }

    #[test]
    fn cmp_same_scale() {
        let left = Decimal::try_from("0.5").unwrap();
        let right = Decimal::try_from("0.7").unwrap();
        assert!(left < right);
    }

    #[test]
    fn cmp_different_scale() {
        let left = Decimal::try_from("5").unwrap();
        let right = Decimal::try_from("0.7").unwrap();
        assert!(left > right);
    }

    #[test]
    fn cmp_equal() {
        let left = Decimal::try_from("5.1").unwrap();
        let right = Decimal::try_from("5.1").unwrap();
        assert!(left == right);
    }

    #[test]
    fn cmp_several_digits() {
        let left = Decimal::try_from("5.1").unwrap();
        let right = Decimal::try_from("5.1").unwrap();
        assert!(left == right);
    }

    #[test]
    fn add_no_carry() {
        let left = Decimal::try_from("1.2").unwrap();
        let right = Decimal::try_from("2.3").unwrap();
        let expected = Decimal {
            value: BigInt::from(35),
            scale: 1,
        };
        assert_eq!(left + right, expected);
    }

    #[test]
    fn add_carry() {
        let left = Decimal::try_from("1.7").unwrap();
        let right = Decimal::try_from("2.3").unwrap();
        let expected = Decimal {
            value: BigInt::from(4),
            scale: 0,
        };
        assert_eq!(left + right, expected);
    }

    #[test]
    fn add_different_scale() {
        let left = Decimal::try_from("0.07").unwrap();
        let right = Decimal::try_from("10.3").unwrap();
        let expected = Decimal {
            value: BigInt::from(1037),
            scale: 2,
        };
        assert_eq!(left + right, expected);
    }

    #[test]
    fn sub_no_carry() {
        let left = Decimal::try_from("2.3").unwrap();
        let right = Decimal::try_from("1.2").unwrap();
        let expected = Decimal {
            value: BigInt::from(11),
            scale: 1,
        };
        assert_eq!(left - right, expected);
    }

    #[test]
    fn sub_carry() {
        let left = Decimal::try_from("2.2").unwrap();
        let right = Decimal::try_from("0.2").unwrap();
        let expected = Decimal {
            value: BigInt::from(2),
            scale: 0,
        };
        assert_eq!(left - right, expected);
    }

    #[test]
    fn sub_different_scale() {
        let left = Decimal::try_from("10.3").unwrap();
        let right = Decimal::try_from("0.07").unwrap();
        let expected = Decimal {
            value: BigInt::from(1023),
            scale: 2,
        };
        assert_eq!(left - right, expected);
    }

    #[test]
    fn sub_result_is_negative() {
        let left = Decimal::try_from("1.3").unwrap();
        let right = Decimal::try_from("3.4").unwrap();
        let expected = Decimal {
            value: BigInt::from(-21),
            scale: 1,
        };
        assert_eq!(left - right, expected);
    }

    #[test]
    fn mul_ints() {
        let left = Decimal::try_from("2").unwrap();
        let right = Decimal::try_from("3").unwrap();
        let expected = Decimal {
            value: BigInt::from(6),
            scale: 0,
        };
        assert_eq!(left * right, expected);
    }

    #[test]
    fn mul_decimals() {
        let left = Decimal::try_from("0.2").unwrap();
        let right = Decimal::try_from("0.3").unwrap();
        let expected = Decimal {
            value: BigInt::from(6),
            scale: 2,
        };
        assert_eq!(left * right, expected);
    }

    #[test]
    fn mul_different_scales() {
        let left = Decimal::try_from("10.2").unwrap();
        let right = Decimal::try_from("0.03").unwrap();
        let expected = Decimal {
            value: BigInt::from(306),
            scale: 3,
        };
        assert_eq!(left * right, expected);
    }
}
