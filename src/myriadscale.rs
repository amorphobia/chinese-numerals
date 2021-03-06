use crate::{characters::*, ChineseNumeralBase, ShortScaleInt, Sign, Signed};

/// Myriad scale integers (万进).
///
/// 「以万进者，万万曰亿，万亿曰兆。」
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Default)]
pub struct MyriadScaleInt {
    pub(super) sign: Sign,
    pub(super) data: u128,
}

impl MyriadScaleInt {
    /// Generates a new non-positive myriad scale integer from given absolute value.
    ///
    /// There is no way to generate Chinese numerals by `From` trait from negative primitive numbers less than [`i128::MIN`]. This associated function provides a way to generate them from the given absolute value less than or equal to [`u128::MAX`]. This crate also provides struct [`MyriadScaleBigInt`] for integers with absolute value larger than `u128::MAX`.
    pub fn new_non_pos(abs: u128) -> Self {
        if abs == 0 {
            Self::default()
        } else {
            Self {
                sign: Sign::Neg,
                data: abs,
            }
        }
    }
}

impl ChineseNumeralBase for MyriadScaleInt {
    fn to_chars(&self) -> Vec<NumChar> {
        let mut chars = Vec::new();
        let mut num = *self.data();
        let mut prev_rem = 1000;

        // u128 uses up to NUM_CHARS[21] = Jian (涧) for myriad scale numerals
        for exp in 12..=21 {
            let rem = (num % 1_0000) as u16;
            num /= 1_0000;

            if rem > 0 {
                if !chars.is_empty() && prev_rem < 1000 {
                    chars.push(NUM_CHARS[0]);
                }
                if exp > 12 {
                    chars.push(NUM_CHARS[exp]);
                }
                let short = ShortScaleInt::from(rem);
                let mut node = short.to_chars();
                chars.append(&mut node);
            }
            prev_rem = rem;
        }
        chars
    }

    fn to_chars_trimmed(&self) -> Vec<NumChar> {
        let mut chars = self.to_chars();
        let mut data = *self.data();
        while data >= 1_0000 {
            data /= 1_0000;
        }
        if data >= 10 && data <= 19 {
            let one = chars.pop();
            debug_assert_eq!(one, Some(NumChar::One));
        }
        chars
    }
}

#[cfg(feature = "bigint")]
use num_bigint::BigUint;
#[cfg(feature = "bigint")]
use num_integer::Integer;
#[cfg(feature = "bigint")]
use num_traits::{ToPrimitive, Zero};

/// Myriad scale big integers (万进).
///
/// Use it by turning on feature "bigint". It uses [`BigUint`](num_bigint::BigUint) to store the absolute value. Therefore, all integers that can be expressed in myriad scale are included.
#[cfg(feature = "bigint")]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Default)]
pub struct MyriadScaleBigInt {
    pub(super) sign: Sign,
    pub(super) data: BigUint,
}

#[cfg(feature = "bigint")]
impl MyriadScaleBigInt {
    pub(super) const MAX_ABS_ARR: &'static [u32] =
        &[4294967295, 2134966271, 2523967787, 239310294, 2938735877];

    /// The maximum integer can be expressed in myriad scale.
    pub fn max_value() -> Self {
        Self {
            sign: Sign::Pos,
            data: BigUint::from_slice(Self::MAX_ABS_ARR),
        }
    }

    /// The minimum integer can be expressed in myriad scale.
    pub fn min_value() -> Self {
        Self {
            sign: Sign::Neg,
            data: BigUint::from_slice(Self::MAX_ABS_ARR),
        }
    }
}

#[cfg(feature = "bigint")]
impl ChineseNumeralBase for MyriadScaleBigInt {
    fn to_chars(&self) -> Vec<NumChar> {
        let mut chars = Vec::new();
        let mut num = self.data().to_owned();
        let mut prev_rem = BigUint::new(vec![1000]);
        let lim = BigUint::new(vec![1000]);
        let div = BigUint::new(vec![1_0000]);

        for exp in 12..=23 {
            let (_, rem) = num.div_rem(&div);
            num /= &div;

            if rem > BigUint::zero() {
                if !chars.is_empty() && prev_rem < lim {
                    chars.push(NUM_CHARS[0]);
                }
                if exp > 12 {
                    chars.push(NUM_CHARS[exp]);
                }
                let rem = rem.to_u16().unwrap();
                let short = ShortScaleInt::from(rem);
                let mut node = short.to_chars();
                chars.append(&mut node);
            }
            prev_rem = rem;
        }
        chars
    }

    fn to_chars_trimmed(&self) -> Vec<NumChar> {
        let mut chars = self.to_chars();
        let mut data = self.data().to_owned();
        let div = BigUint::new(vec![1_0000]);
        let ten = BigUint::new(vec![10]);
        let nineteen = BigUint::new(vec![19]);
        while data >= div {
            data /= &div;
        }
        if data >= ten && data <= nineteen {
            let one = chars.pop();
            debug_assert_eq!(one, Some(NumChar::One));
        }
        chars
    }
}
